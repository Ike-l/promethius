use std::{
    collections::HashMap, 
    time::{
        Duration, Instant
    }
};

use small_derive_deref::{
    Deref, DerefMut
};

#[derive(Debug, Deref, DerefMut, Default)]
pub struct Accumulators {
    pub accumulators: HashMap<String, Accumulator>
}

impl Accumulators {
    pub fn insert_one(&mut self, label: &str) -> Option<Accumulator> {
        self.insert(label.to_string(), Accumulator::default())
    }
}

#[derive(Debug)]
pub struct Accumulator {
    pub time: Instant
}

impl Default for Accumulator {
    fn default() -> Self {
        Self {
            time: Instant::now()
        }
    }
}

impl Accumulator {
    pub fn time_since(&self) -> Duration {
        Instant::now().duration_since(self.time)
    }

    pub fn update(&mut self) {
        self.time = Instant::now();
    }
}