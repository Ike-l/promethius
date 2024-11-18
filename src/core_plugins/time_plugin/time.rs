use std::time::{
    Duration, Instant
};

use super::{
    Res, ResMut
};

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

pub fn update_time(mut time: ResMut<Time>) {
    let dt = Instant::now().duration_since(time.now);
    time.dt = dt;
    time.now += dt;
}

pub fn fps_counter(time: Res<Time>) {
    println!("FPS: {:?}", time.fps());
}