use small_derive_deref::{
    Deref, DerefMut
};

use super::{
    acceleration_structures::prelude::{
        Collider, ColliderComponent, QuadTree
    }, 
    label_plugin::prelude::LabelComponent, 
    RefWorld, ResMut
};

#[derive(Debug, Default, Deref, DerefMut)]
pub struct UIAccelerationStructure {
    qt: QuadTree
}

pub fn create_acceleration_structure(acc_struct: ResMut<UIAccelerationStructure>, world: RefWorld) {
    acc_struct.value.qt = QuadTree::auto(
        world.query::<(&ColliderComponent, &LabelComponent)>()
            .iter()
            .map(|(_, (collider, label))| {
                Collider::new(collider.clone(), label.clone())
    }).collect::<Vec<Collider>>());
}

