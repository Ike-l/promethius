use crate::prelude::object_plugin::prelude::ObjectId;

use super::{
    raw_render_component::RawRenderComponent, FloatPrecision
};

use cgmath::{
    Deg, InnerSpace, Matrix4, Quaternion, Rotation3, SquareMatrix, Vector3, Vector4
};

#[derive(Debug)]
pub struct InstanceRenderComponent {
    pub visible: bool,
    pub object_id: ObjectId,

    pub local_translation: Vector3<FloatPrecision>,
    pub global_translation: Vector3<FloatPrecision>,

    pub local_rotation: Quaternion<FloatPrecision>,
    pub global_rotation: Quaternion<FloatPrecision>,

    pub local_scale: Matrix4<FloatPrecision>,
    pub global_scale: Matrix4<FloatPrecision>,

    pub tint: Vector4<FloatPrecision>,
    pub highlight: Vector4<FloatPrecision>,
}

impl Default for InstanceRenderComponent {
    fn default() -> Self {
        Self {
            visible: true,
            object_id: ObjectId("Used Default(), use 'new' and pass the object id".to_string()),

            local_translation: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            global_translation: Vector3 { x: 0.0, y: 0.0, z: 0.0 },

            local_rotation: Quaternion::new(0.0, 0.0, 0.0, 1.0),
            global_rotation: Quaternion::new(0.0, 0.0, 0.0, 1.0),

            local_scale: Matrix4::identity(),
            global_scale: Matrix4::identity(),

            tint: Vector4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 },
            highlight: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
        }
    }
}

impl InstanceRenderComponent {
    pub fn new(object_id: ObjectId) -> Self {
        Self {
            object_id,
            ..Default::default()
        }
    }

    pub fn to_raw(&self) -> RawRenderComponent {
        let model = self.model_matrix();

        RawRenderComponent::new(model.into(), self.tint.into(), self.highlight.into())
    }

    pub fn model_matrix(&self) -> Matrix4<FloatPrecision> {
        Matrix4::from_translation(self.global_translation) *
        Matrix4::from(self.global_rotation) *
        self.global_scale *
        Matrix4::from_translation(self.local_translation) *
        Matrix4::from(self.local_rotation) *
        self.local_scale
    }

    pub fn local_stretch(&mut self, other: &Vector3<FloatPrecision>) {
        self.local_scale = Matrix4::from_nonuniform_scale(other.x, other.y, other.z) * self.local_scale;
    }

    pub fn global_stretch(&mut self, other: &Vector3<FloatPrecision>) {
        self.global_scale = Matrix4::from_nonuniform_scale(other.x, other.y, other.z) * self.global_scale;
    }

    pub fn local_rotate(&mut self, angle: Deg<FloatPrecision>, axis: Vector3<FloatPrecision>) {
        let rotation_quat = Quaternion::from_axis_angle(axis.normalize(), angle);
        self.local_rotation = rotation_quat * self.local_rotation;
    }

    pub fn global_rotate(&mut self, angle: Deg<FloatPrecision>, axis: Vector3<FloatPrecision>) {
        let rotation_quat = Quaternion::from_axis_angle(axis.normalize(), angle);
        self.global_rotation = rotation_quat * self.global_rotation;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {
        todo!()
    }
}

