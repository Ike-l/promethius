use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct BiMap<K, V> {
    forward: HashMap<K, V>,
    backward: HashMap<V, K>,
}

impl<K, V> Default for BiMap<K, V> {
    fn default() -> Self {
        Self {
            forward: HashMap::new(),
            backward: HashMap::new(),
        }
    }
}


impl<K, V> BiMap<K, V> where 
    K: Hash + Eq + Clone,
    V: Hash + Eq + Clone,
{
    pub fn insert(&mut self, key: K, value: V) {
        self.forward.insert(key.clone(), value.clone());
        self.backward.insert(value, key);
    }

    pub fn get_key(&self, value: &V) -> Option<&K> {
        self.backward.get(value)
    }

    pub fn get_value(&self, key: &K) -> Option<&V> {
        self.forward.get(key)
    }
}
