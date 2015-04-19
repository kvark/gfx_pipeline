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
    pub visible: bool,
    pub color: gfx::ColorValue,
    pub texture: gfx::shade::TextureParam<R>,
    pub blend: Option<gfx::BlendPreset>,
}

impl<R: gfx::Resources> gfx_phase::Material for Material<R> {}

pub trait Pipeline<S, R: gfx::Resources> {
    fn render<
        X: gfx_scene::OrderedScene<R, ViewInfo = view::Info<S>>,
        C: gfx::CommandBuffer<R>,
        O: gfx::Output<R>,
    >(  &mut self, scene: &X, renderer: &mut gfx::Renderer<R, C>,
        camera: &X::Camera, output: &O) -> Result<(), gfx_scene::Error>
    where
        X::Entity: gfx_phase::Entity<R, Material<R>>;
}
