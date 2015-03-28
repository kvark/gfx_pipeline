#![feature(custom_attribute, plugin)]
#![plugin(gfx_macros)]

extern crate cgmath;
extern crate gfx;
extern crate gfx_phase;
extern crate gfx_scene;

pub mod forward;
pub mod view;

#[derive(Clone)]
pub struct Material<R: gfx::Resources> {
    pub color: gfx::ColorValue,
    pub texture: gfx::shade::TextureParam<R>,
    pub blend: Option<gfx::BlendPreset>,
}

impl<R: gfx::Resources> gfx_phase::Material for Material<R> {}
