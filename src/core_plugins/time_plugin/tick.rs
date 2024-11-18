use small_derive_deref::{
    Deref, DerefMut
};

use super::ResMut;

#[derive(Debug, Default, Deref, DerefMut)]
pub struct Tick(pub u64);

pub fn update_tick_count(mut ticks: ResMut<Tick>) {
    **ticks += 1;
} 