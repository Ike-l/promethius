mod tick;
mod time;
mod accumulators;

pub mod prelude {
    pub use super::{
        tick::Tick, 
        time::{
            Time, fps_counter
        },
        accumulators::{
            Accumulator, Accumulators
        }
    };
}

use crate::prelude::*;

pub struct TimePlugin;

impl PluginTrait for TimePlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(1.001, time::update_time);
        app.add_system(1.001, tick::update_tick_count);
        app.add_resource(time::Time::default());
        app.add_resource(tick::Tick::default());
        app.add_resource(accumulators::Accumulators::default());

    }
    fn id(&self) -> PluginId {
        PluginId("prometheus_TimePlugin")
    }
}






