use std::collections::HashMap;
use hecs::Entity;

use super::{RefWorld, ResMut};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LabelComponent {
    id: String
}

#[derive(Debug, Default)]
pub struct LabeledEntities {
    labels: HashMap<LabelComponent, Entity>
}

pub fn update_labeled_entities(mut labeled_entities: ResMut<LabeledEntities>, world: RefWorld) {
    labeled_entities.labels = world.query::<&LabelComponent>().iter().fold(HashMap::new(), |mut acc, (e, l)| {
        acc.insert(l.clone(), e);
        acc
    });
}