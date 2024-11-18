use std::time::{
    Duration, Instant
};
use small_derive_deref::{
    Deref, DerefMut
};

pub mod prelude {
    pub use super::{
        Tick, 
        Time,
        fps_counter,
    };
}

use crate::prelude::*;

pub struct TimePlugin;

impl PluginTrait for TimePlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(1.001, update_time);
        app.add_system(1.001, update_tick_count);
        app.add_resource(Time::default());
        app.add_resource(Tick(0));

    }
    fn id(&self) -> PluginId {
        PluginId("prometheus_TimePlugin".to_string())
    }
}

#[derive(Debug)]
pub struct Time {
    pub now: Instant,
    pub dt: Duration,
    pub init: Instant,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            now: Instant::now(),
            dt: Duration::new(0, 0),
            init: Instant::now(),
        }
    }
}

impl Time {
    pub fn fps(&self) -> f64 {
        1. / self.dt.as_secs_f64()
    }
}

#[derive(Debug, Default, Deref, DerefMut)]
pub struct Tick(pub u64);

fn update_time(mut time: ResMut<Time>) {
    let dt = Instant::now().duration_since(time.now);
    time.dt = dt;
    time.now += dt;
}

fn update_tick_count(mut ticks: ResMut<Tick>) {
    **ticks += 1;
} 

pub fn fps_counter(time: Res<Time>) {
    println!("FPS: {:?}", time.fps());
}