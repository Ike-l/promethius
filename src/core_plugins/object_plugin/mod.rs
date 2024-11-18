mod object_registry;
mod models;

pub mod prelude {
    pub use super::*;
    
    pub use models::{
        material::*, 
        mesh::*,
        model::*,
    };
    
    pub use object_registry::{
        Object, ObjectId, ObjectRegistry
    };
}

use crate::prelude::*;

pub struct ObjectPlugin;

impl PluginTrait for ObjectPlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(1.499, object_registry::update_registry_instances);
        app.add_resource(object_registry::ObjectRegistry::default());

    }
    fn id(&self) -> PluginId {
        PluginId("prometheus_ObjectPlugin".to_string())
    }
}