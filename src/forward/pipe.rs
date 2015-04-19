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

pub struct Pipeline<R: gfx::Resources> {
    pub phase: super::Phase<R>,
    pub background: Option<gfx::ColorValue>,
}

impl<R: gfx::Resources> Pipeline<R> {
    pub fn new<F: gfx::Factory<R>>(factory: &mut F,
               tex_default: gfx::shade::TextureParam<R>)
               -> Result<Pipeline<R>, gfx::ProgramError> {
        super::Technique::new(factory, tex_default).map(|tech| Pipeline {
            phase: gfx_phase::Phase::new_cached("Main", tech),
            background: Some([0.0; 4]),
        })
    }
}

impl<R: gfx::Resources> ::Pipeline<f32, R> for Pipeline<R> {
    fn render<
        X: gfx_scene::OrderedScene<R, ViewInfo = ::view::Info<f32>>,
        C: gfx::CommandBuffer<R>,
        O: gfx::Output<R>,
    >(  &mut self, scene: &X, renderer: &mut gfx::Renderer<R, C>,
        camera: &X::Camera, output: &O) -> Result<(), gfx_scene::Error>
    where
        X::Entity: gfx_phase::Entity<R, ::Material<R>>,
    {
        renderer.reset();
        // clear
        match self.background {
            Some(color) => {
                let cdata = gfx::ClearData {
                    color: color,
                    depth: 1.0,
                    stencil: 0,
                };
                renderer.clear(cdata, gfx::COLOR | gfx::DEPTH, output);
            },
            None => (),
        }
        // draw
        scene.draw_ordered(&mut self.phase, order, camera, output, renderer)
    }
}
