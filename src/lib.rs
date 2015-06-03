//! Standard rendering pipelines and materials for gfx_scene.

#![deny(missing_docs)]

extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_phase;
extern crate gfx_scene;

/// Flat pipeline.
pub mod flat;
/// Forward-rendering pipeline.
pub mod forward;
mod light;
mod view;

pub use self::light::PointSource as Light;
pub use self::view::Info as ViewInfo;
pub use gfx_scene::{Error, Report};

/// Alpha test type.
pub type AlphaThreshold = u8;

/// Transparency type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Transparency {
    /// Opaque material, no transparency.
	Opaque,
    /// Transparent in a sense of cutting out a shape from a texture.
	Cutout(AlphaThreshold),
    /// Real transparency with blending.
	Blend(gfx::BlendPreset),
}

/// A simple material type used in the pipelines.
#[derive(Clone)]
pub struct Material<R: gfx::Resources> {
    /// Color (both diffuse and specular).
    pub color: gfx::ColorValue,
    /// Diffuse color texture.
    pub texture: Option<gfx::shade::TextureParam<R>>,
    /// Transparency type.
    pub transparency: Transparency,
}

impl<R: gfx::Resources> gfx_phase::Material for Material<R> {}

/// A generic rendering pipeline.
pub trait Pipeline<S, R: gfx::Resources> {
    /// Render an abstract scene.
    fn render<A, T>(&mut self, &A, &A::Camera, &mut T)
              -> Result<A::Status, Error> where
        A: gfx_scene::AbstractScene<R, ViewInfo = view::Info<S>, Material = Material<R>>,
        T: gfx::Stream<R>;
}
