use gfx;
use cgmath;

/// A point light source.
#[derive(Clone, Debug, PartialEq)]
pub struct PointSource<S: cgmath::BaseNum> {
    /// Boolean flag to enable/disable the light.
    pub active: bool,
    /// Kind of the light.
    pub kind: Kind,
    /// Color.
    pub color: gfx::ColorValue,
    /// Attenuation type.
    pub attenuation: Attenuation<S>,
    /// Homogeneous world position.
    pub position: cgmath::Vector4<S>,
    /// View transformation.
    pub view: cgmath::Matrix4<S>,
}

/// Attenuation type of the light.
#[derive(Clone, Debug, PartialEq)]
pub enum Attenuation<S> {
    /// No attenuation with distance.
    Constant {
        /// Constant intensity.
        intensity: S,
    },
    /// Quadratic attenuation.
    Quadratic {
        /// Constant term.
        k0: S,
        /// Linear term.
        k1: S,
        /// Quadratic term.
        k2: S,
    },
    /// Spherical attenuation, limited at a distance.
    Spherical {
        /// Base intensity.
        intensity: S,
        /// Distance limit.
        distance: S,
    },
}

/// Kind of the light source.
#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    /// Omni-directional light.
    Omni,
    /// Hemi-spherical light.
    Hemi,
    /// Directed (cone shape) light.
    Directed,
}
