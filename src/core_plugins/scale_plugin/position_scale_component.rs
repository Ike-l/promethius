use cgmath::Vector3;

use super::{
    label_plugin::prelude::{
        LabelComponent, LabeledEntities
    }, 
    render_plugin::WindowDimensions, 
    MutWorld, Res
};

pub struct PositionScaleComponent {
    pub scale: Vector3<f32>,
    pub parent: Option<LabelComponent>,
}

impl PositionScaleComponent {
    pub fn new(scale: Vector3<f32>, parent: Option<LabelComponent>) -> Self {
        Self {
            scale,
            parent
        }
    }
}

pub fn update_position(_world: MutWorld, _labels: Res<LabeledEntities>, _screen: Res<WindowDimensions>) {
    todo!()
}


