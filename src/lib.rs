extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_phase;
extern crate gfx_scene;

pub mod flat;
pub mod forward;
mod view;

pub use self::view::Info as ViewInfo;
pub use gfx_scene::{Error, Report};

pub type AlphaThreshold = u8;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Transparency {
	Opaque,
	Cutout(AlphaThreshold),
	Blend(gfx::BlendPreset),
}

#[derive(Clone)]
pub struct Material<R: gfx::Resources> {
    pub color: gfx::ColorValue,
    pub texture: Option<gfx::shade::TextureParam<R>>,
    pub transparency: Transparency,
}

impl<R: gfx::Resources> gfx_phase::Material for Material<R> {}

pub trait Pipeline<S, R: gfx::Resources> {
    fn render<A, T>(&mut self, &A, &A::Camera, &mut T)
              -> Result<A::Status, Error> where
        A: gfx_scene::AbstractScene<R, ViewInfo = view::Info<S>, Material = Material<R>>,
        T: gfx::Stream<R>;
}
