#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum WorldId {
    Hecs(hecs::Entity),
    Other(usize),
}

impl WorldId {
    pub fn unwrap_hecs(self) -> hecs::Entity {
        match self {
            WorldId::Hecs(h) => h,
            WorldId::Other(_) => panic!("Expected hecs::Entity, got: {:?}", self)
        }
    }

    pub fn unwrap_other(self) -> usize {
        match self {
            WorldId::Other(u) => u,
            WorldId::Hecs(_) => panic!("Expected usize, got: {:?}", self)
        }
    }
}
