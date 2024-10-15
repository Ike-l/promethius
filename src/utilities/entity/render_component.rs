use crate::prelude::object_plugin::prelude::ObjectId;

use super::{
    raw_render_component::RawRenderComponent, FloatPrecision
};

#[derive(Debug)]
pub struct RenderComponent {
    pub visible: bool,
    pub object_id: ObjectId,
    pub position: cgmath::Vector3<FloatPrecision>,
    pub rotation: cgmath::Matrix2<FloatPrecision>,
    /*
    scale?
    quaternion?

    scale and rotation and translation functions
     */
}

impl RenderComponent {
    pub fn new<T, Y>(visible: bool, object_id: ObjectId, position: T, rotation: Y) -> Self 
    where 
        T: Into<cgmath::Vector3<FloatPrecision>>,
        Y: Into<cgmath::Matrix2<FloatPrecision>>,
    {
        Self {
            visible,
            object_id,
            position: position.into(),
            rotation: rotation.into(),
        }
    }

    pub fn to_raw(&self) -> RawRenderComponent {
        let model = cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation);
        RawRenderComponent::new(model.into())
    }
}