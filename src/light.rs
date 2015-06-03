use gfx;
use cgmath;

/// A point light source.
#[derive(Clone)]
pub struct PointSource<S> {
    /// Kind of the light.
    pub kind: Kind,
    /// Color.
    pub color: gfx::ColorValue,
    /// Attenuation type.
    pub attenuation: Attenuation<S>,
    /// World position.
    pub position: cgmath::Vector3<S>,
    /// View transformation.
    pub view: cgmath::Matrix4<S>,
}

/// Attenuation type of the light.
#[derive(Clone)]
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
#[derive(Clone)]
pub enum Kind {
    /// Omni-directional light.
    Omni,
    /// Hemi-spherical light.
    Hemi,
    /// Directed (cone shape) light.
    Directed,
}
