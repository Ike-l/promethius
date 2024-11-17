mod button;

pub mod prelude {
    pub use super::button::{
        ToggleButtonComponent, TimedButtonComponent, Delay
    };
}

use crate::prelude::*;

pub struct UIPlugin;

impl PluginTrait for UIPlugin {
    fn build(&self, _app: &mut crate::app::App) {
        
    }

    fn id(&self) -> PluginId {
        PluginId("slingshot_UIPlugin".to_string())
    }
}

