use gfx;
use gfx_phase;
use gfx_phase::Object;
use gfx_scene;
use std::cmp::Ordering;


pub fn order<S: PartialOrd, R: gfx::Resources>(
             a: &Object<S, super::Kernel, super::Params<R>>,
             b: &Object<S, super::Kernel, super::Params<R>>)
             -> Ordering {
    match (a.kernel, b.kernel) {
        (None, None)        => Object::front_to_back(a, b),
        (None, Some(_))     => Ordering::Less,
        (Some(_), None)     => Ordering::Greater,
        (Some(_), Some(_))  => Object::back_to_front(a, b),
    }
}

pub struct Pipeline<D: gfx::Device> {
    pub phase: super::Phase<D::Resources>,
    pub renderer: gfx::Renderer<D::Resources, D::CommandBuffer>,
}

impl<
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    D: gfx::Device<Resources = R, CommandBuffer = C> + gfx::Factory<R>
> Pipeline<D>{
    pub fn new(device: &mut D, tex_default: gfx::shade::TextureParam<R>)
               -> Result<Pipeline<D>, gfx::ProgramError> {
        use gfx::traits::DeviceExt;
        let renderer = device.create_renderer();
        super::Technique::new(device, tex_default).map(|tech| Pipeline {
            phase: gfx_phase::Phase::new_cached("Main", tech),
            renderer: renderer,
        })
    }
}

impl<D: gfx::Device> ::Pipeline<f32, D> for Pipeline<D> {
    fn render<
        C: gfx_scene::OrderedScene<D::Resources, ViewInfo = ::view::Info<f32>>,
    >(  &mut self, scene: &C, camera: &C::Camera,
        frame: &gfx::Frame<D::Resources>)
        -> Result<gfx::SubmitInfo<D>, gfx_scene::Error>
    where
        C::Entity: gfx_phase::Entity<D::Resources, ::Material<D::Resources>>,
    {
        match scene.draw_ordered(&mut self.phase, order, camera, frame, &mut self.renderer)  {
            Ok(_) => Ok(self.renderer.as_buffer()),
            Err(e) => Err(e),
        }
    }
}
