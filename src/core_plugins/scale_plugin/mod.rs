mod position_scale_component;
mod size_scale_component;

use crate::prelude::*;

pub mod prelude {
    #[allow(unused_braces)]
    pub use super::{
        position_scale_component::PositionScaleComponent,
        size_scale_component::SizeScaleComponent,
    };
}


pub struct ScalePlugin;

const UPDATE_PHASE: f64 = 1.001;

impl PluginTrait for ScalePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(UPDATE_PHASE, size_scale_component::update_size);
        app.add_system(UPDATE_PHASE + 0.001, position_scale_component::update_position);
    }

    fn id(&self) -> PluginId {
        PluginId("prometheus_ScalePlugin")    
    }
}

/*
 Position stay relative to parent, Position is {x%;y%} of parent width/height
 Size stay relative to parent, Width/Height is {x%;y%} of parent width/height

 make a tree of all, 
 -- BFS updating the size then position in order, 
 -- updating Colliders: 
 -- -- If using colliders for children: recalculating after each, or applying a transformation (harder?). 
 -- -- If using the model mat then can recalculate last/before the position is updated?

*/



