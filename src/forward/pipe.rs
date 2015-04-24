use gfx;
use gfx_phase;
use gfx_scene;
use std::cmp::Ordering;


pub fn order<S: PartialOrd, R: gfx::Resources>(
             a: &gfx_phase::Object<S, super::Kernel, super::Params<R>>,
             b: &gfx_phase::Object<S, super::Kernel, super::Params<R>>)
             -> Ordering {
    match (a.kernel, b.kernel) {
        (None, None)        => a.cmp_depth(b),
        (None, Some(_))     => Ordering::Less,
        (Some(_), None)     => Ordering::Greater,
        (Some(_), Some(_))  => b.cmp_depth(a),
    }
}

pub struct Pipeline<R: gfx::Resources, E> {
    pub phase: super::Phase<R, E>,
    pub background: Option<gfx::ColorValue>,
}

impl<R: gfx::Resources, E> Pipeline<R, E> {
    pub fn new<F: gfx::Factory<R>>(factory: &mut F,
               tex_default: gfx::shade::TextureParam<R>)
               -> Result<Pipeline<R, E>, gfx::ProgramError> {
        super::Technique::new(factory, tex_default).map(|tech| Pipeline {
            phase: gfx_phase::Phase::new("Main", tech)
                                    .with_sort(order)
                                    .with_cache(),
            background: Some([0.0; 4]),
        })
    }
}

impl<R: gfx::Resources, E: gfx_phase::Entity<R, ::Material<R>>> ::Pipeline<f32, R, E> for Pipeline<R, E> {
    fn render<
        A: gfx_scene::AbstractScene<R, ViewInfo = ::view::Info<f32>, Entity = E>,
        C: gfx::CommandBuffer<R>,
        O: gfx::Output<R>,
    >(  &mut self, scene: &A, renderer: &mut gfx::Renderer<R, C>,
        camera: &A::Camera, output: &O) -> Result<(), gfx_scene::Error>
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
        scene.draw(&mut self.phase, camera, output, renderer)
    }
}
