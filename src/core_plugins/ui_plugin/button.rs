use std::time::Duration;

use super::time_plugin::prelude::Tick;

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

// click once = on, after delay = off
#[derive(Debug, Default)]
pub struct TimedButtonComponent {
    pub on: bool,
    pub delay: Delay,
}



// input
// pub struct ClickComponent;
// with ClickComponent dispatch on <Entity + Event> as UIWindowEvent || UIDeviceEvent
