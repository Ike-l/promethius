mod label;

use crate::prelude::*;

pub mod prelude {
    pub use super::label::{
        LabelComponent, LabeledEntities
    };
}

pub struct LabelPlugin;

impl PluginTrait for LabelPlugin {
    fn build(&self, app: &mut App) {
        app.add_resource(label::LabeledEntities::default());
        
        app.add_system(1.001, label::update_labeled_entities);
    }

    fn id(&self) -> PluginId {
        PluginId("prometheus_LabelPlugin")
    }
}

