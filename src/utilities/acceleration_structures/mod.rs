mod quad_tree;

pub mod prelude {
    pub use super::{
        quad_tree::*, AccelerationStructure
    };
}

pub trait AccelerationStructure {
    fn query(&self, position: &super::promethius_std::prelude::Position) -> Vec<quad_tree::Collider>;
}