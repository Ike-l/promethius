mod render_component;
mod controller_component;
mod camera;
mod orthographic;
mod perspective;

pub mod prelude {
    use super::*;

    pub use render_component::*;

    pub use controller_component::*;

    pub use camera::Camera;

    pub use orthographic::{
        OrthoController, OrthoProjection, OrthoUniform, OrthoView
    };

    pub use perspective::{
        PerspController, PerspProjection, PerspUniform, PerspView
    };
}

use crate::prelude::*;

pub struct CameraPlugin;

impl PluginTrait for CameraPlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(1.498, camera::update_camera_bind_group);
        app.add_system(1.002, camera::input);
        app.add_system(1.003, camera::update_camera);

    }
    fn id(&self) -> PluginId {
        PluginId("prometheus_CameraPlugin")
    }
}


pub trait TransformComposer {
    #[rustfmt::skip]
    const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.5,
        0.0, 0.0, 0.0, 1.0,
    );
    fn compose_transform(&self) -> cgmath::Matrix4<f32>;
}
