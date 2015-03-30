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
