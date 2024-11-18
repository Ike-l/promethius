use crate::prelude::{
    *, 
    promethius_std::prelude::Position,
};

mod quad_tree;
mod collider;
mod aabb;

pub mod prelude {
    pub use super::{
        quad_tree::QuadTree, 
        AccelerationStructure,
        aabb::AABB,
        collider::{
            Collider, ColliderComponent
        }
    };    
}

pub struct AccelerationStructurePlugin;

impl PluginTrait for AccelerationStructurePlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(1.001, collider::update_colliders);
    }
    fn id(&self) -> PluginId {
        PluginId("prometheus_AccelerationStructurePlugin")
    }
}


pub trait AccelerationStructure {
    fn query(&self, position: &Position) -> Vec<collider::Collider>;
}

