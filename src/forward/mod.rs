//! This is a classy "Forward" rendering pipeline.
//! It supports all materials except for `Inverse` blending.
//! It renders everything with a single pass, ordering opaque objects
//! first front-to-back, and then blended ones back-to-front on top.
//! TODO: apply actual lights in the shader. Currently hard-coded.

use std::marker::PhantomData;
use gfx;
use gfx_phase;

mod pipe;

pub use self::pipe::{order, Pipeline};

/// A short typedef for the phase.
pub type Phase<R> = gfx_phase::CachedPhase<R,
    ::Material<R>,
    ::view::Info<f32>,
    Technique<R>,
>;

mod param {
    #![allow(missing_docs)]
    use gfx::shade::TextureParam;
    use gfx::handle::Buffer;

    #[derive(Debug, Clone, Copy)]
    pub struct Light {
        pub position: [f32; 4],
        pub color: [f32; 4],
        pub attenuation: [f32; 4],
    }

    gfx_parameters!( Struct {
        u_Transform@ mvp: [[f32; 4]; 4],
        u_WorldTransform@ world: [[f32; 4]; 4],
        u_NormalRotation@ normal: [[f32; 3]; 3],
        u_Color@ color: [f32; 4],
        u_Ambient@ ambient: [f32; 4],
        t_Diffuse@ texture: TextureParam<R>,
        u_AlphaTest@ alpha_test: f32,
        u_LightMask@ light_mask: [i32; 4], //TODO: u32
        b_Lights@ lights: Buffer<R, Light>,
    });
}

/// Typedef for the ordering function.
pub type OrderFun<R> = gfx_phase::OrderFun<f32, Kernel, param::Struct<R>>;

const MAX_LIGHTS: usize = 256; // must be in sync with the shaders
const PHONG_VS    : &'static [u8] = include_bytes!("../../gpu/phong.glslv");
const PHONG_FS    : &'static [u8] = include_bytes!("../../gpu/phong.glslf");
const PHONG_TEX_VS: &'static [u8] = include_bytes!("../../gpu/phong_tex.glslv");
const PHONG_TEX_FS: &'static [u8] = include_bytes!("../../gpu/phong_tex.glslf");

/// Pipeline creation error.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Failed to create a texture.
    Texture(gfx::tex::TextureError),
    /// Failed to link a program.
    Program(gfx::ProgramError),
}

impl From<gfx::tex::TextureError> for Error {
    fn from(e: gfx::tex::TextureError) -> Error {
        Error::Texture(e)
    }
}

impl From<gfx::ProgramError> for Error {
    fn from(e: gfx::ProgramError) -> Error {
        Error::Program(e)
    }
}


/// The core technique of the pipeline.
pub struct Technique<R: gfx::Resources> {
    program: gfx::handle::Program<R>,
    program_textured: gfx::handle::Program<R>,
    state_add: gfx::DrawState,
    state_alpha: gfx::DrawState,
    state_opaque: gfx::DrawState,
    state_multiply: gfx::DrawState,
    light_buf: gfx::handle::Buffer<R, param::Light>,
    /// The default texture used for materials that don't have it.
    pub default_texture: gfx::handle::Texture<R>,
    /// The light color of non-lit areas.
    pub ambient_color: gfx::ColorValue,
    /// Active lights.
    pub lights: Vec<super::Light<f32>>,
}

impl<R: gfx::Resources> Technique<R> {
    /// Create a new technique.
    pub fn new<F: gfx::Factory<R>>(factory: &mut F)
               -> Result<Technique<R>, Error> {
        use gfx::traits::FactoryExt;
        let texture = try!(factory.create_texture_rgba8_static(1, 1, &[0xFFFFFFFF]));
        let prog0 = try!(factory.link_program(PHONG_VS, PHONG_FS));
        let prog1 = try!(factory.link_program(PHONG_TEX_VS, PHONG_TEX_FS));
        let state = gfx::DrawState::new().depth(
            gfx::state::Comparison::LessEqual,
            true
        );
        Ok(Technique {
            program: prog0,
            program_textured: prog1,
            state_add: state.clone().blend(gfx::BlendPreset::Add),
            state_alpha: state.clone().blend(gfx::BlendPreset::Alpha),
            state_multiply: state.clone().blend(gfx::BlendPreset::Multiply),
            state_opaque: state,
            light_buf: factory.create_buffer_dynamic(MAX_LIGHTS, gfx::BufferRole::Uniform),
            default_texture: texture,
            ambient_color: [0.1, 0.1, 0.1, 0.0],
            lights: Vec::new(),
        })
    }

    /// Update the light buffer before drawing.
    pub fn update<S: gfx::Stream<R>>(&self, stream: &mut S) {
        use cgmath::FixedArray;
        for (i, lit) in self.lights.iter().enumerate() {
            use super::light::Attenuation::*;
            let par = param::Light {
                position: *lit.position.as_fixed(),
                color: lit.color,
                attenuation: match lit.attenuation {
                    Constant { intensity } => [1.0 / intensity, 0.0, 0.0, 0.0],
                    Quadratic { k0, k1, k2 } => [k0, k1, k2, 0.0],
                    Spherical { intensity, distance } => [
                        1.0 / intensity,
                        2.0 / distance,
                        1.0 / (distance * distance),
                        0.0],
                },
            };
            stream.access().0.update_buffer(self.light_buf.raw(), &[par], i+1).unwrap()
        }
    }
}

/// Kernel of the technique, defining what program needs to be used.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Kernel {
    textured: bool,
    transparency: ::Transparency,
}

impl<R: gfx::Resources> gfx_phase::Technique<R, ::Material<R>, ::view::Info<f32>> for Technique<R> {
    type Kernel = Kernel;
    type Params = param::Struct<R>;

    fn test(&self, mesh: &gfx::Mesh<R>, mat: &::Material<R>) -> Option<Kernel> {
        if mat.transparency == ::Transparency::Blend(gfx::BlendPreset::Invert) {
            return None
        }
        Some(Kernel {
            textured: mat.texture.is_some() &&
                mesh.attributes.iter().find(|a| a.name == "a_Tex0").is_some(),
            transparency: mat.transparency,
        })
    }

    fn compile<'a>(&'a self, kernel: Kernel, _space: &::view::Info<f32>)
                   -> gfx_phase::TechResult<'a, R, param::Struct<R>> {
        use ::Transparency::*;
        (   if kernel.textured {
                &self.program_textured
            } else {
                &self.program
            },
            param::Struct {
                mvp: [[0.0; 4]; 4],
                world: [[0.0; 4]; 4],
                normal: [[0.0; 3]; 3],
                color: [0.0; 4],
                ambient: [0.0; 4],
                texture: (self.default_texture.clone(), None),
                alpha_test: if let Cutout(v) = kernel.transparency {
                    v as f32 / 255 as f32
                }else { 0.0 },
                light_mask: [0; 4],
                lights: self.light_buf.clone(),
                _r: PhantomData,
            },
            None,
            match kernel.transparency {
                Blend(gfx::BlendPreset::Add)      => &self.state_add,
                Blend(gfx::BlendPreset::Alpha)    => &self.state_alpha,
                Blend(gfx::BlendPreset::Multiply) => &self.state_multiply,
                _ => &self.state_opaque,
            }
        )
    }

    fn fix_params(&self, mat: &::Material<R>, space: &::view::Info<f32>,
                  params: &mut param::Struct<R>) {
        use cgmath::FixedArray;

        params.mvp = *space.mx_vertex.as_fixed();
        params.world = *space.mx_world.as_fixed();
        params.normal = *space.mx_normal.as_fixed();
        params.color = mat.color;
        params.ambient = self.ambient_color;

        params.light_mask = self.lights.iter().enumerate().fold(
            (0, 0, [0; 4]), |(mut bit, element, mut mask), (i, lit)| {
                //TODO: frustum intersect with entity
                if lit.active {
                    mask[element] |= ((i + 1) as i32) << bit;
                    bit += 8;
                    (bit & 0x1F, element + (bit >> 5), mask)
                } else { (bit, element, mask) }
            }
        ).2;

        if let Some(ref tex) = mat.texture {
            params.texture = tex.clone();
        }
    }
}
