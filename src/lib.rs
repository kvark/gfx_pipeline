#![feature(custom_attribute, plugin)]
#![plugin(gfx_macros)]

extern crate cgmath;
extern crate gfx;
extern crate gfx_phase;
extern crate gfx_scene;

pub mod forward;

#[derive(Copy)]
pub struct ViewInfo<S> {
    pub mx_vertex: cgmath::Matrix4<S>,
    pub mx_normal: cgmath::Matrix3<S>,
}

impl<S: cgmath::BaseFloat> gfx_phase::ToDepth for ViewInfo<S> {
    type Depth = S;
    fn to_depth(&self) -> S {
        self.mx_vertex.w.z / self.mx_vertex.w.w
    }
}

impl<
    S: cgmath::BaseFloat + 'static,
    T: cgmath::CompositeTransform3<S, cgmath::Quaternion<S>>, //TODO
> gfx_scene::ViewInfo<S, T> for ViewInfo<S> {
    fn new(mvp: cgmath::Matrix4<S>, view: T, _model: T) -> ViewInfo<S> {
        use cgmath::ToMatrix3;
        let (_, rot, _) = view.decompose();
        ViewInfo {
            mx_vertex: mvp,
            mx_normal: rot.to_matrix3(),
        }
    }
}
