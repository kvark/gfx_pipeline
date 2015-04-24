use gfx;
use gfx_phase;

mod pipe;

pub use self::pipe::{order, Pipeline};

pub type Phase<R, E> = gfx_phase::CachedPhase<R,
    ::Material<R>,
    ::view::Info<f32>,
    Technique<R>,
    E,
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
}

const PHONG_VS: &'static [u8] = include_bytes!("../../gpu/phong.glslv");
const PHONG_FS: &'static [u8] = include_bytes!("../../gpu/phong.glslf");

pub struct Technique<R: gfx::Resources> {
    program: gfx::ProgramHandle<R>,
    state_add: gfx::DrawState,
    state_alpha: gfx::DrawState,
    state_opaque: gfx::DrawState,
    state_multiply: gfx::DrawState,
    default_texture_param: gfx::shade::TextureParam<R>,
}

impl<R: gfx::Resources> Technique<R> {
    pub fn from_program(program: gfx::ProgramHandle<R>, tex_param: gfx::shade::TextureParam<R>)
                        -> Technique<R> {
        let state = gfx::DrawState::new().depth(
            gfx::state::Comparison::LessEqual,
            true
        );
        Technique {
            program: program,
            state_add: state.clone().blend(gfx::BlendPreset::Add),
            state_alpha: state.clone().blend(gfx::BlendPreset::Alpha),
            state_multiply: state.clone().blend(gfx::BlendPreset::Multiply),
            state_opaque: state,
            default_texture_param: tex_param,
        }
    }

    pub fn new<F: gfx::Factory<R>>(factory: &mut F, tex_param: gfx::shade::TextureParam<R>)
               -> Result<Technique<R>, gfx::ProgramError> {
        use gfx::traits::FactoryExt;
        factory.link_program(PHONG_VS, PHONG_FS)
               .map(|p| Technique::from_program(p, tex_param))
    }
}

pub type Kernel = Option<gfx::BlendPreset>;

impl<R: gfx::Resources> gfx_phase::Technique<R, ::Material<R>, ::view::Info<f32>> for Technique<R> {
    type Kernel = Kernel;
    type Params = Params<R>;

    fn test(&self, _mesh: &gfx::Mesh<R>, mat: &::Material<R>) -> Option<Kernel> {
        if !mat.visible {
            return None
        }
        match mat.blend {
            Some(gfx::BlendPreset::Invert) => None,
            other => Some(other)
        }
    }

    fn compile<'a>(&'a self, kernel: Kernel, _space: ::view::Info<f32>)
                   -> gfx_phase::TechResult<'a, R, Params<R>> {
        (   &self.program,
            Params {
                mvp: [[0.0; 4]; 4],
                normal: [[0.0; 3]; 3],
                color: [0.0; 4],
                texture: self.default_texture_param.clone(),
            },
            None,
            match kernel {
                Some(gfx::BlendPreset::Add) => &self.state_add,
                Some(gfx::BlendPreset::Alpha) => &self.state_alpha,
                Some(gfx::BlendPreset::Multiply) => &self.state_multiply,
                _ => &self.state_opaque,
            }
        )
    }

    fn fix_params(&self, mat: &::Material<R>, space: &::view::Info<f32>, params: &mut Params<R>) {
        use cgmath::FixedArray;
        params.mvp = *space.mx_vertex.as_fixed();
        params.normal = *space.mx_normal.as_fixed();
        params.color = mat.color;
        params.texture = mat.texture.clone();
    }
}
