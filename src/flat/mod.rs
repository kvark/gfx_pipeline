//! This is a simple "Flat" rendering pipeline.
//! It doesn't support blended objects and uses front-to-back ordering.
//! The pipeline is meant for simple applications and fall-back paths.

use std::marker::PhantomData;
use gfx;
use gfx_phase;
use gfx_scene;

/// A short typedef for the phase.
pub type Phase<R> = gfx_phase::CachedPhase<R,
    ::Material<R>,
    ::view::Info<f32>,
    Technique<R>,
>;

mod param {
    #![allow(missing_docs)]
    use gfx::shade::TextureParam;

    gfx_parameters!( Struct {
        u_Transform@ mvp: [[f32; 4]; 4],
        u_Color@ color: [f32; 4],
        t_Diffuse@ texture: TextureParam<R>,
        u_AlphaTest@ alpha_test: f32,
    });
}

const FLAT_VS    : &'static [u8] = include_bytes!("../../gpu/flat.glslv");
const FLAT_FS    : &'static [u8] = include_bytes!("../../gpu/flat.glslf");
const FLAT_TEX_VS: &'static [u8] = include_bytes!("../../gpu/flat_tex.glslv");
const FLAT_TEX_FS: &'static [u8] = include_bytes!("../../gpu/flat_tex.glslf");

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
    state: gfx::DrawState,
    /// The default texture used for materials that don't have it.
    pub default_texture: gfx::handle::Texture<R>,
}

impl<R: gfx::Resources> Technique<R> {
    /// Create a new technique.
    pub fn new<F: gfx::Factory<R>>(factory: &mut F)
               -> Result<Technique<R>, Error> {
        use gfx::traits::FactoryExt;
        Ok(Technique {
            program: try!(factory.link_program(FLAT_VS, FLAT_FS)),
            program_textured: try!(factory.link_program(FLAT_TEX_VS, FLAT_TEX_FS)),
            state: gfx::DrawState::new().depth(gfx::state::Comparison::LessEqual, true),
            default_texture: try!(factory.create_texture_rgba8_static(1, 1, &[0xFFFFFFFF])),
        })
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kernel {
    Flat,
    Textured,
    AlphaCut(::AlphaThreshold),
}

impl<R: gfx::Resources> gfx_phase::Technique<R, ::Material<R>, ::view::Info<f32>> for Technique<R> {
    type Kernel = Kernel;
    type Params = param::Struct<R>;

    fn test(&self, mesh: &gfx::Mesh<R>, mat: &::Material<R>) -> Option<Kernel> {
        let textured = mat.texture.is_some() &&
            mesh.attributes.iter().find(|a| a.name == "a_Tex0").is_some();
        match mat.transparency {
            ::Transparency::Opaque if textured => Some(Kernel::Textured),
            ::Transparency::Opaque => Some(Kernel::Flat),
            ::Transparency::Cutout(v) if textured => Some(Kernel::AlphaCut(v)),
            _ => None
        }
    }

    fn compile<'a>(&'a self, kernel: Kernel)
                   -> gfx_phase::TechResult<'a, R, param::Struct<R>> {
        (   if kernel != Kernel::Flat {
                &self.program_textured
            } else {
                &self.program
            },
            param::Struct {
                mvp: [[0.0; 4]; 4],
                color: [0.0; 4],
                texture: (self.default_texture.clone(), None),
                alpha_test: if let Kernel::AlphaCut(v) = kernel {
                    v as f32 / 255 as f32
                }else { 0.0 },
                _r: PhantomData,
            },
            &self.state,
            None,
        )
    }

    fn fix_params(&self, mat: &::Material<R>, space: &::view::Info<f32>,
                  params: &mut param::Struct<R>) {
        use cgmath::FixedArray;
        params.mvp = *space.mx_vertex.as_fixed();
        params.color = mat.color;
        if let Some(ref tex) = mat.texture {
            params.texture = tex.clone();
        }
    }
}

/// The flat pipeline.
pub struct Pipeline<R: gfx::Resources> {
    /// The only rendering phase.
    pub phase: Phase<R>,
    /// Background color. Set to none if you don't want the screen to be cleared.
    pub background: Option<gfx::ColorValue>,
}

impl<R: gfx::Resources> Pipeline<R> {
    /// Create a new pipeline.
    pub fn new<F: gfx::Factory<R>>(factory: &mut F)
               -> Result<Pipeline<R>, Error> {
        Technique::new(factory).map(|tech| Pipeline {
            phase: gfx_phase::Phase::new("Main", tech)
                                    .with_sort(gfx_phase::sort::front_to_back)
                                    .with_cache(),
            background: Some([0.0; 4]),
        })
    }
}

impl<R: gfx::Resources> ::Pipeline<f32, R> for Pipeline<R> {
    fn render<A, T>(&mut self, scene: &A, camera: &A::Camera, stream: &mut T)
              -> Result<A::Status, gfx_scene::Error> where
        A: gfx_scene::AbstractScene<R, ViewInfo = ::view::Info<f32>, Material = ::Material<R>>,
        T: gfx::Stream<R>,
    {
        // clear
        if let Some(color) = self.background {
            stream.clear(gfx::ClearData {
                color: color,
                depth: 1.0,
                stencil: 0,
            });
        }
        // draw
        scene.draw(&mut self.phase, camera, stream)
    }
}
