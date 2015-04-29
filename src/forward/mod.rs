use gfx;
use gfx_phase;

mod pipe;

pub use self::pipe::{order, Pipeline};

pub type Phase<R> = gfx_phase::CachedPhase<R,
    ::Material<R>,
    ::view::Info<f32>,
    Technique<R>,
>;

#[derive(Clone)]
#[shader_param]
pub struct Params<R: gfx::Resources> {
    #[name = "u_Transform"]
    pub mvp: [[f32; 4]; 4],
    #[name = "u_NormalRotation"]
    pub normal: [[f32; 3]; 3],
    #[name = "u_Color"]
    pub color: [f32; 4],
    #[name = "t_Diffuse"]
    pub texture: gfx::shade::TextureParam<R>,
    #[name = "u_AlphaTest"]
    pub alpha_test: f32,
}

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
    program: gfx::ProgramHandle<R>,
    program_textured: gfx::ProgramHandle<R>,
    state_add: gfx::DrawState,
    state_alpha: gfx::DrawState,
    state_opaque: gfx::DrawState,
    state_multiply: gfx::DrawState,
    pub default_texture: gfx::TextureHandle<R>,
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
