use super::super::TransformComposer;

#[derive(Debug)]
pub struct PerspProjection {
    pub aspect: f32,
    pub fovy: cgmath::Rad<f32>,
    pub znear: f32,
    pub zfar: f32,
}

impl PerspProjection {
    pub fn new<T: Into<cgmath::Rad<f32>>>(width: u32, height: u32, fovy: T, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
}

impl TransformComposer for PerspProjection {
    fn compose_transform(&self) -> cgmath::Matrix4<f32> {
        Self::OPENGL_TO_WGPU_MATRIX * cgmath::perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}