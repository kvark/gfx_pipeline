extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_phase;
extern crate gfx_scene;

pub mod forward;
mod view;

pub use self::view::Info as ViewInfo;
pub use gfx_scene::{Error, Report};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Transparency {
	Opaque,
	Cutout(u8),
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
    type Success;
    fn render<A, T>(&mut self, &A, &A::Camera, &mut T)
              -> Result<Self::Success, Error> where
        A: gfx_scene::AbstractScene<R, ViewInfo = view::Info<S>, Material = Material<R>>,
        T: gfx::Stream<R>;
}
