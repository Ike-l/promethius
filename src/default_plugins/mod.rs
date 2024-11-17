mod spawn_cameras;

use crate::plugins::PluginTrait;

pub fn get_default_plugins() -> Vec<Box<dyn PluginTrait>> {
    vec![
        Box::new(spawn_cameras::DefaultCameraPlugin),
    ]
}