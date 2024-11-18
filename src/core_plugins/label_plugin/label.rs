use hecs::Entity;

use crate::utilities::bimap::BiMap;

use super::{
    RefWorld, ResMut
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LabelComponent {
    pub id: String
}

impl LabelComponent {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string()
        }
    }
}

#[derive(Debug, Default)]
pub struct LabeledEntities {
    pub labels: BiMap<LabelComponent, Entity>
}

pub fn update_labeled_entities(mut labeled_entities: ResMut<LabeledEntities>, world: RefWorld) {
    labeled_entities.labels = world.query::<&LabelComponent>().iter().fold(BiMap::default(), |mut acc, (e, l)| {
        acc.insert(l.clone(), e);
        acc
    });
}