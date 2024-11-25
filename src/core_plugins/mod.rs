pub mod object_plugin;
pub mod render_plugin;
pub mod camera_plugin;
pub mod time_plugin;
pub mod ui_plugin;
pub mod label_plugin;
pub mod acceleration_structures_plugin;
pub mod scale_plugin;

use crate::plugins::PluginTrait;

pub fn get_core_plugins() -> Vec<Box<dyn PluginTrait>> {
    vec![
        Box::new(object_plugin::ObjectPlugin),
        Box::new(render_plugin::RenderPlugin),
        Box::new(camera_plugin::CameraPlugin),
        Box::new(time_plugin::TimePlugin),
        Box::new(ui_plugin::UIPlugin),
        Box::new(label_plugin::LabelPlugin),
        Box::new(acceleration_structures_plugin::AccelerationStructurePlugin),
        Box::new(scale_plugin::ScalePlugin),
    ]
}

