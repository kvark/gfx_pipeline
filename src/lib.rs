#![feature(custom_attribute, plugin)]
#![plugin(gfx_macros)]

extern crate cgmath;
extern crate gfx;
extern crate gfx_phase;
extern crate gfx_scene;

pub mod forward;
mod view;

pub use self::view::Info as ViewInfo;
pub use gfx_scene::{Error, FailCount};


#[derive(Clone)]
pub struct Material<R: gfx::Resources> {
    pub visible: bool,
    pub color: gfx::ColorValue,
    pub texture: Option<gfx::shade::TextureParam<R>>,
    pub blend: Option<gfx::BlendPreset>,
}

impl<R: gfx::Resources> gfx_phase::Material for Material<R> {}

pub type CullEntity<'a, R> = gfx_scene::CullEntity<'a, R, Material<R>, view::Info<f32>>;

pub trait Pipeline<S, R: gfx::Resources> {
    fn render<A, T>(&mut self, &A, &A::Camera, &mut T)
              -> Result<FailCount, Error> where
        A: gfx_scene::AbstractScene<R, ViewInfo = view::Info<S>, Material = Material<R>>,
        T: gfx::Stream<R>;
}
