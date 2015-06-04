use cgmath;
use gfx_phase;
use gfx_scene;

/// Standard view information for an object.
#[derive(Clone, Copy)]
pub struct Info<S> {
    /// Vertex -> clip transform.
    pub mx_vertex: cgmath::Matrix4<S>,
    /// Vertex -> world transform.
    pub mx_world: cgmath::Matrix4<S>,
    /// Normal -> world transform.
    pub mx_normal: cgmath::Matrix3<S>,
}

impl<S: cgmath::BaseFloat> gfx_phase::ToDepth for Info<S> {
    type Depth = S;
    fn to_depth(&self) -> S {
        self.mx_vertex.w.z / self.mx_vertex.w.w
    }
}

impl<
    S: cgmath::BaseFloat + 'static,
    //R: cgmath::Rotation3<S>,
    T: cgmath::CompositeTransform3<S, cgmath::Quaternion<S>>,
> gfx_scene::ViewInfo<S, T> for Info<S> {
    fn new(mvp: cgmath::Matrix4<S>, view: T, model: T) -> Info<S> {
        let (_, rot, _) = view.decompose();
        Info {
            mx_vertex: mvp,
            mx_world: model.into(),
            mx_normal: rot.into(),
        }
    }
}
