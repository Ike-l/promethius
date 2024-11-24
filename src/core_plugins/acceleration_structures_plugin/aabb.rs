use super::promethius_std::prelude::Position;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AABB {
    pub min: Position,
    pub max: Position,
}

impl AABB {
    pub fn new(min: Position, max: Position) -> Self {
        Self {
            min,
            max
        }
    }
    
    pub fn middle(&self) -> Position {
        Position::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
            (self.min.z + self.max.z) / 2.0,
        )
    }

    pub fn split_2d(&self) -> [AABB; 4] {
        let middle = self.middle();
        [
            AABB {
                min: self.min.clone(), 
                max: middle.clone(),
            },
            AABB {
                min: Position::new(
                    self.min.x, 
                    middle.y,
                    0.0
                ), 
                max: Position::new(
                    middle.x, 
                    self.max.y,
                    0.0
                ),
            },
            AABB {
                min: middle.clone(), 
                max: self.max.clone(),
            },
            AABB {
                min: Position::new(
                    middle.x, 
                    self.min.y,
                    0.0
                ),
                max: Position::new(
                    self.max.x, 
                    middle.y,
                    0.0
                ),
            },
        ]
    }

    pub fn contains(&self, position: &Position) -> bool {
        position >= &self.min && position <= &self.max
    }

    pub fn expand(&mut self, other: &AABB) {
        if self.min.x > other.min.x { self.min.x = other.min.x };
        if self.min.y > other.min.y { self.min.y = other.min.y };
        if self.max.x < other.max.x { self.max.x = other.max.x };
        if self.max.y < other.max.y { self.max.y = other.max.y };
    }

    pub fn expand_pos(&mut self, other: Position) {
        if self.min.x > other.x {
            self.min.x = other.x
        } else if self.max.x < other.x {
            self.max.x = other.x
        };
        if self.min.y > other.y {
            self.min.y = other.y 
        } else if self.max.y < other.y {
            self.max.y = other.y 
        };
    }

    pub fn add(&self, position: &Position) -> AABB {
        AABB::new(
            self.min.add(position),
            self.max.add(position),
        )
    }
}