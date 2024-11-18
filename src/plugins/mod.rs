use std::any::Any;

use super::app::App;

pub trait PluginTrait {
    fn build(&self, app: &mut App);
    fn id(&self) -> PluginId;
}

pub trait PluginCollisionHandler {
    // Which type is colliding, what phase is it in, how many orders of magnitude is suggested
    // i.e if 1 level (or phase / 10) is not enough, user can suggest phase / 100 etc. 
    fn handle_collision<T: Any>(&mut self, phase: f64, levels: u8);
}


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct PluginId(pub &'static str);
