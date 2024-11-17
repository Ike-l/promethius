use std::time::Duration;

use super::{entity::prelude::InstanceRenderComponent, time_plugin::Tick, MutWorld};

// click once = on, click after = off
#[derive(Debug, Default)]
pub struct ToggleButtonComponent {
    pub on: bool,
}

#[derive(Debug)]
pub enum Delay {
    Time(Duration),
    Tick(Tick),
}

impl Default for Delay {
    fn default() -> Self {
        Self::Tick(Tick(1))
    }
}

// click once = on, after x ticks = off
#[derive(Debug, Default)]
pub struct TimedButtonComponent {
    pub on: bool,
    pub delay: Delay,
}

pub fn toggle_button(world: MutWorld) {
    for (_, (button, render)) in &mut world.query::<(&mut ToggleButtonComponent, &mut InstanceRenderComponent)>() {

    }
}