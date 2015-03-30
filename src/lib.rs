#![feature(custom_attribute, plugin)]
#![plugin(gfx_macros)]

extern crate cgmath;
extern crate gfx;
extern crate gfx_phase;
extern crate gfx_scene;

pub mod forward;
mod view;

pub use self::view::Info as ViewInfo;
pub use gfx_scene::Error;


#[derive(Clone)]
pub struct Material<R: gfx::Resources> {
    pub color: gfx::ColorValue,
    pub texture: gfx::shade::TextureParam<R>,
    pub blend: Option<gfx::BlendPreset>,
}

impl<R: gfx::Resources> gfx_phase::Material for Material<R> {}

pub trait Pipeline<S, D: gfx::Device> {
    fn render<C: gfx_scene::OrderedScene<D::Resources, ViewInfo = view::Info<S>>>(
              &mut self, scene: &C, camera: &C::Camera, frame: &gfx::Frame<D::Resources>)
              -> Result<gfx::SubmitInfo<D>, gfx_scene::Error> where
        C::Entity: gfx_phase::Entity<D::Resources, Material<D::Resources>>;
}
