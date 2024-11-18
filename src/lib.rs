mod core_plugins;
mod default_plugins;
mod utilities;

pub mod scheduler;
pub mod app;
pub mod plugins;


pub mod prelude {  
    pub use crate::core_plugins::{
        get_core_plugins,
        object_plugin,
        camera_plugin,
        time_plugin,
        render_plugin,  
        ui_plugin,
        label_plugin,
    };

    #[allow(unused_braces)]
    pub use crate::default_plugins::{
        get_default_plugins,
    };

    pub use crate::utilities::{
        resources, 
        texture, 
        entity,
        acceleration_structures,
        promethius_std,
    };
    
    pub use crate::app::{
        App, AppBuilder, WindowEventBus, DeviceEventBus
    };

    pub use crate::scheduler::{
        Res, ResMut, RefWorld, MutWorld, EventWriter, EventReader, CommandBuffer,
    };

    pub use crate::plugins::{
        PluginId, PluginTrait, PluginCollisionHandler
    };

    pub use wgpu::PrimitiveTopology::{
        LineList, TriangleList,
    };
}
