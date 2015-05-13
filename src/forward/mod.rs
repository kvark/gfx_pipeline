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

pub type Phase<R> = gfx_phase::CachedPhase<R,
    ::Material<R>,
    ::view::Info<f32>,
    Technique<R>,
>;

pub type OrderFun<R> = gfx_phase::OrderFun<f32, Kernel, Params<R>>;

gfx_parameters!( Params/Link {
    u_Transform@ mvp: [[f32; 4]; 4],
    u_NormalRotation@ normal: [[f32; 3]; 3],
    u_Color@ color: [f32; 4],
    t_Diffuse@ texture: gfx::shade::TextureParam<R>,
    u_AlphaTest@ alpha_test: f32,
});

const PHONG_VS    : &'static [u8] = include_bytes!("../../gpu/phong.glslv");
const PHONG_FS    : &'static [u8] = include_bytes!("../../gpu/phong.glslf");
const PHONG_TEX_VS: &'static [u8] = include_bytes!("../../gpu/phong_tex.glslv");
const PHONG_TEX_FS: &'static [u8] = include_bytes!("../../gpu/phong_tex.glslf");

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Texture(gfx::tex::TextureError),
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


pub struct Technique<R: gfx::Resources> {
    program: gfx::handle::Program<R>,
    program_textured: gfx::handle::Program<R>,
    state_add: gfx::DrawState,
    state_alpha: gfx::DrawState,
    state_opaque: gfx::DrawState,
    state_multiply: gfx::DrawState,
    pub default_texture: gfx::handle::Texture<R>,
}

impl<R: gfx::Resources> Technique<R> {
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
            default_texture: texture,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Kernel {
    textured: bool,
    transparency: ::Transparency,
}

impl<R: gfx::Resources> gfx_phase::Technique<R, ::Material<R>, ::view::Info<f32>> for Technique<R> {
    type Kernel = Kernel;
    type Params = Params<R>;

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
                   -> gfx_phase::TechResult<'a, R, Params<R>> {
        use ::Transparency::*;
        (   if kernel.textured {
                &self.program_textured
            } else {
                &self.program
            },
            Params {
                mvp: [[0.0; 4]; 4],
                normal: [[0.0; 3]; 3],
                color: [0.0; 4],
                texture: (self.default_texture.clone(), None),
                alpha_test: if let Cutout(v) = kernel.transparency {
                    v as f32 / 255 as f32
                }else { 0.0 },
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

    fn fix_params(&self, mat: &::Material<R>, space: &::view::Info<f32>, params: &mut Params<R>) {
        use cgmath::FixedArray;
        params.mvp = *space.mx_vertex.as_fixed();
        params.normal = *space.mx_normal.as_fixed();
        params.color = mat.color;
        if let Some(ref tex) = mat.texture {
            params.texture = tex.clone();
        }
    }
}
