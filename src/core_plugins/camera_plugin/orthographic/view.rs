use cgmath::EuclideanSpace;

use super::super::TransformComposer;

#[derive(Debug)]
pub struct OrthoView {
    pub position: cgmath::Point3<f32>,
}

impl TransformComposer for OrthoView {
    fn compose_transform(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::from_translation(-self.position.to_vec())
    }
}

impl OrthoView {
    pub fn new<T: Into<cgmath::Point3<f32>>>(position: T) -> Self {
        Self {
            position: position.into(),
        }
    }
}