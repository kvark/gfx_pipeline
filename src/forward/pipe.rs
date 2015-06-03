use gfx;
use gfx_phase;
use gfx_scene;
use std::cmp::Ordering;


/// Custom ordering function to draw opaque objects first in a
/// front-to-back order, and transparent objects on top in a
/// back-to-front order.
pub fn order<S: PartialOrd, R: gfx::Resources>(
             a: &gfx_phase::Object<S, super::Kernel, super::param::Struct<R>>,
             b: &gfx_phase::Object<S, super::Kernel, super::param::Struct<R>>)
             -> Ordering {
    use ::Transparency::*;
    match (a.kernel.transparency, b.kernel.transparency) {
        (Blend(_), Blend(_)) => b.cmp_depth(a),
        (Blend(_), _)        => Ordering::Greater,
        (_, Blend(_))        => Ordering::Less,
        (_, _)               => a.cmp_depth(b),
    }
}

/// The foreard rendering pipeline.
pub struct Pipeline<R: gfx::Resources> {
    /// The only rendering phase.
    pub phase: super::Phase<R>,
    /// Background color. Set to none if you don't want the screen to be cleared.
    pub background: Option<gfx::ColorValue>,
}

impl<R: gfx::Resources> Pipeline<R> {
    /// Create a new pipeline.
    pub fn new<F: gfx::Factory<R>>(factory: &mut F)
               -> Result<Pipeline<R>, super::Error> {
        super::Technique::new(factory).map(|tech| Pipeline {
            phase: gfx_phase::Phase::new("Main", tech)
                                    .with_sort(order)
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
        // prepare
        self.phase.technique.update(stream);
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
