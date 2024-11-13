use cgmath::InnerSpace;

use super::super::TransformComposer;

#[derive(Debug)]
pub struct PerspView {
    pub position: cgmath::Point3<f32>,
    pub yaw: cgmath::Rad<f32>,
    pub pitch: cgmath::Rad<f32>,
}

impl PerspView {
    pub fn new(position: cgmath::Point3<f32>, yaw: f32, pitch: f32) -> Self {
        Self {
            position,
            yaw: cgmath::Rad(yaw),
            pitch: cgmath::Rad(pitch),
        }
    }
}

impl TransformComposer for PerspView {
    fn compose_transform(&self) -> cgmath::Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        cgmath::Matrix4::look_to_rh(
            self.position,
            cgmath::Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            cgmath::Vector3::unit_y(),
        )
    }
}