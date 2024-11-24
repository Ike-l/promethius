use std::collections::VecDeque;

use crate::utilities::promethius_std::prelude::Position;

use super::{
    aabb::AABB, 
    prelude::Collider, 
    AccelerationStructure
};

#[derive(Debug)]
enum Child {
    Leaf(Vec<Collider>),
    Branch(QuadTree),
}

#[derive(Debug)]
pub struct QuadTree {
    children: [Box<Child>; 4],
    space: AABB,
    depth: usize,
}

impl Default for QuadTree {
    fn default() -> Self {
        Self::auto(vec![])
    }
}

impl AccelerationStructure for QuadTree {
    fn query(&self, position: &Position) -> Vec<Collider> {
        if !self.space.contains(position) {
            return vec![];
        }

        let middle = self.space.middle();
        let (middle_x, middle_y) = (middle.x, middle.y);

        let index = if position.x > middle_x {
            if position.y > middle_y { 2 } else { 3 }
        } else {
            if position.y > middle_y { 1 } else { 0 }
        };

        match self.children[index].as_ref() {
            Child::Branch(b) => {
                b.query(position)
            },
            Child::Leaf(l) => {
                l.clone().into_iter().filter(
                    |collider| collider.collider.bbox.contains(position)
                ).collect::<Vec<Collider>>()
            }
        }
    }
}

impl QuadTree {
    pub fn new(buffer: Vec<Collider>, space: AABB, depth: usize) -> Self {
        let middle = space.middle();
        let (middle_x, middle_y) = (middle.x, middle.y);
        // lb, lt, rb, rt
        let mut buffers = [vec![], vec![], vec![], vec![]];

        buffer.into_iter().for_each(|collider| {
            let bbox = &collider.collider.bbox;
            let mut pass = 0;
            pass += if bbox.min.x > middle_x { 1 } else { 2 };
            pass += if bbox.min.y > middle_y { 4 } else { 8 };
            pass += if bbox.max.x > middle_x { 16 } else { 32 };
            pass += if bbox.max.y > middle_y { 64 } else { 128 };

            match pass {
                90 => {
                    buffers[0].push(collider.clone());
                    buffers[1].push(collider.clone());
                    buffers[2].push(collider.clone());
                    buffers[3].push(collider.clone());
                },
                86 => {
                    buffers[1].push(collider.clone());
                    buffers[2].push(collider.clone());
                },
                89 => {
                    buffers[2].push(collider.clone());
                    buffers[3].push(collider.clone());
                },
                106 => {
                    buffers[0].push(collider.clone());
                    buffers[1].push(collider.clone());
                },
                154 => {
                    buffers[0].push(collider.clone());
                    buffers[3].push(collider.clone());
                },
                85 => {
                    buffers[2].push(collider.clone());
                },
                102 => {
                    buffers[1].push(collider.clone());
                },
                153 => {
                    buffers[3].push(collider.clone());
                },
                170 => {
                    buffers[0].push(collider.clone());
                },
                166 | 165 | 101 | 150 | 149 | 169 | 105 => {
                    panic!("min > max: pass: {:?}, bbox: {:?}", pass, bbox)
                },
                _ => panic!("computing pass")
            }
        });

            let children = if depth == 0 {
                buffers
                    .into_iter()
                    .map(|buffer| Box::new(Child::Leaf(buffer)))
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Exactly 4 elements")
            } else {
                let mut next_space = VecDeque::from(space.split_2d());

                buffers
                    .into_iter()
                    .map(|buffer| {
                        Box::new(Child::Branch(
                            QuadTree::new(buffer, next_space.pop_front().unwrap(), depth - 1)
                        ))
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Expected exactly 4 elements")
            };

            Self {
                children,
                space,
                depth
            }

    }

    pub fn auto(buffer: Vec<Collider>) -> Self {
        let space = buffer.iter().fold(AABB { min: Position::default(), max: Position::default() }, 
            |acc, cur| {
                let min_x = if cur.collider.bbox.min.x < acc.min.x { cur.collider.bbox.min.x } else { acc.min.x };
                let min_y = if cur.collider.bbox.min.y < acc.min.y { cur.collider.bbox.min.y } else { acc.min.y };

                let max_x = if cur.collider.bbox.max.x > acc.max.x { cur.collider.bbox.max.x } else { acc.max.x };
                let max_y = if cur.collider.bbox.max.y > acc.max.y { cur.collider.bbox.max.y } else { acc.max.y };

                let min = Position::new(min_x, min_y, 0.0);
                let max = Position::new(max_x, max_y, 0.0);
                
                AABB {
                    min,
                    max
                }
            });
        
        let depth = buffer.len().checked_ilog(4).unwrap_or(0);

        QuadTree::new(buffer, space, depth as usize)

    }

    pub fn print(&self) {
        println!("Printing QuadTree");
        println!("Space: {:?}\nDepth: {:?}", self.space, self.depth);
        self.children.iter().for_each(|c| {
            match c.as_ref() {
                Child::Branch(b) => {
                    b.print();
                },
                Child::Leaf(l) => {
                    println!("{:?}", l)
                }
            }
        });
    }

    pub fn space(&self) -> &AABB {
        &self.space
    }
}


#[cfg(test)]
mod tests {
    use crate::prelude::{
        acceleration_structures_plugin::prelude::ColliderComponent, 
        label_plugin::prelude::LabelComponent
    };

    use super::*;

    fn generate_quadrants() -> Vec<Collider> {
        vec![
            Collider::new(
                ColliderComponent::new(AABB {
                    min: Position::new(3.0, 3.0, 0.0),
                    max: Position::new(6.0, 6.0, 0.0),
                }),
                LabelComponent::new("0"),
            ),
            Collider::new(
                ColliderComponent::new(AABB {
                    min: Position::new(3.0, -6.0, 0.0),
                    max: Position::new(6.0, -3.0, 0.0),
                }),
                LabelComponent::new("1"),
            ),
            Collider::new(
                ColliderComponent::new(AABB {
                    min: Position::new(-6.0, -6.0, 0.0),
                    max: Position::new(-3.0, -3.0, 0.0),
                }),
                LabelComponent::new("2"),
            ),
            Collider::new(
                ColliderComponent::new(AABB {
                    min: Position::new(-6.0, 3.0, 0.0),
                    max: Position::new(-3.0, 6.0, 0.0),
                }),
                LabelComponent::new("3"),
            ),
        ]
    }

    fn generate_overlapped() -> Vec<Collider> {
        vec![
            Collider::new(
                ColliderComponent::new(AABB {
                    min: Position::new(3.0, 3.0, 0.0),
                    max: Position::new(6.0, 6.0, 0.0),
                }),
                LabelComponent::new("0"),
            ),
            Collider::new(
                ColliderComponent::new(AABB {
                    min: Position::new(3.0, 3.0, 0.0),
                    max: Position::new(6.0, 6.0, 0.0),
                }),
                LabelComponent::new("1"),
            ),
            Collider::new(
                ColliderComponent::new(AABB {
                    min: Position::new(3.0, 3.0, 0.0),
                    max: Position::new(6.0, 6.0, 0.0),
                }),
                LabelComponent::new("2"),
            ),
            Collider::new(
                ColliderComponent::new(AABB {
                    min: Position::new(3.0, 3.0, 0.0),
                    max: Position::new(6.0, 6.0, 0.0),
                }),
                LabelComponent::new("3"),
            ),
        ]
    }

    fn ten_space() -> AABB {
        AABB {
            min: Position::new(-10.0, -10.0, 0.0),
            max: Position::new(10.0, 10.0, 0.0),
        }
    }
    
    #[test]
    fn create_qt() {
        let _ = QuadTree::auto(vec![]);
    }

    #[test]
    fn fill_quadrants() {
        let _ = QuadTree::new(generate_quadrants(), ten_space(), 1);
    }
    
    #[test]
    fn query_random_single() {
        let qt = QuadTree::new(generate_quadrants(), ten_space(), 4);
        
        let c = qt.query(&Position::new(4.0,4.0, 0.0));
        assert_eq!(c.len(), 1);
        assert_eq!(c.first().unwrap().entity_label, LabelComponent::new("0"));

        let c = qt.query(&Position::new(-4.0,-4.0, 0.0));
        assert_eq!(c.len(), 1);
        assert_eq!(c.first().unwrap().entity_label, LabelComponent::new("2"));

        let c = qt.query(&Position::new(4.0,-4.0, 0.0));
        assert_eq!(c.len(), 1);
        assert_eq!(c.first().unwrap().entity_label, LabelComponent::new("1"));

        let c = qt.query(&Position::new(-4.0,4.0, 0.0));
        assert_eq!(c.len(), 1);
        assert_eq!(c.first().unwrap().entity_label, LabelComponent::new("3"));
    }

    #[test]
    fn query_random_single_auto() {
        let qt = QuadTree::auto(generate_quadrants());
        
        let c = qt.query(&Position::new(4.0, 4.0, 0.0));
        assert_eq!(c.len(), 1);
        assert_eq!(c.first().unwrap().entity_label, LabelComponent::new("0"));

        let c = qt.query(&Position::new(-4.0, -4.0, 0.0));
        assert_eq!(c.len(), 1);
        assert_eq!(c.first().unwrap().entity_label, LabelComponent::new("2"));

        let c = qt.query(&Position::new(4.0, -4.0, 0.0));
        assert_eq!(c.len(), 1);
        assert_eq!(c.first().unwrap().entity_label, LabelComponent::new("1"));

        let c = qt.query(&Position::new(-4.0, 4.0, 0.0));
        assert_eq!(c.len(), 1);
        assert_eq!(c.first().unwrap().entity_label, LabelComponent::new("3"));
    }

    #[test]
    fn fill_many() {
        let qt = QuadTree::new(generate_overlapped(), ten_space(), 8);

        let c = qt.query(&Position::new(4.0, 4.0, 0.0));
        assert_eq!(c, generate_overlapped());

        let c = qt.query(&Position::new(-4.0, 4.0, 0.0));
        assert_eq!(c.len(), 0)
    }

    #[test]
    fn fill_many_auto() {
        let qt = QuadTree::auto(generate_overlapped());

        let c = qt.query(&Position::new(4.0, 4.0, 0.0));
        assert_eq!(c, generate_overlapped());

        let c = qt.query(&Position::new(-4.0, 4.0, 0.0));
        assert_eq!(c.len(), 0)
    }

    #[test]
    fn bbox_contains() {
        let space = AABB::new(Position::new(-10.0, -10.0, 0.0), Position::new(10.0, 10.0, 0.0));
        let position = Position::new(0.0, 0.0, 0.0);

        assert!(space.contains(&position));

        let position = Position::new(-10.1, 0.0, 0.0);
        assert!(!space.contains(&position));
        let position = Position::new(-10.1, 10.1, 0.0);
        assert!(!space.contains(&position));
        let position = Position::new(0.0, 10.1, 0.0);
        assert!(!space.contains(&position)); //
        let position = Position::new(10.1, 10.1, 0.0);
        assert!(!space.contains(&position));
        let position = Position::new(10.1, 0.0, 0.0);
        assert!(!space.contains(&position));
        let position = Position::new(10.1, -10.1, 0.0);
        assert!(!space.contains(&position));
        let position = Position::new(0.0, -10.1, 0.0);
        assert!(!space.contains(&position));
        let position = Position::new(-10.1, -10.1, 0.0);
        assert!(!space.contains(&position));
    }

    #[test]
    fn auto_correct_space() {
        let c_1 = Collider::new(
            ColliderComponent::new(AABB::new(
                Position::new(-2.0, -2.0, 0.0), 
                Position::new(2.0, 2.0, 0.0)
            )), 
            LabelComponent::new("0")
        );

        let c_2 = Collider::new(
            ColliderComponent::new(AABB::new(
                Position::new(6.0, 6.0, 0.0), 
                Position::new(8.0, 8.0, 0.0)
            )), 
            LabelComponent::new("0")
        );
        let buffer = vec![
            c_1.clone()
        ];
        let qt = QuadTree::auto(buffer);

        assert_eq!(qt.space, c_1.collider.bbox);

        let buffer = vec![
            c_1.clone(),
            c_2.clone(),
        ];

        let qt = QuadTree::auto(buffer);

        assert_eq!(qt.space, AABB::new(Position::new(-2.0, -2.0, 0.0), Position::new(8.0, 8.0, 0.0)));
    }

    #[test]
    fn aabb_expand() {
        let mut aabb = AABB::default();

        let aabb_max = AABB::new(Position::default(), Position::new(3.0, 1.0, 0.0));
        aabb.expand(&aabb_max);
        assert_eq!(aabb.middle(), Position::new(1.5, 0.5, 0.0));

        let aabb_min = AABB::new(Position::new(-3.0, -1.0, 0.0), Position::default());
        aabb.expand(&aabb_min);
        assert_eq!(aabb.middle(), Position::default());

        let mut aabb = AABB::default();

        let aabb_t = AABB::new(Position::default(), Position::new(1.0, 0.0, 0.0));
        aabb.expand(&aabb_t);
        assert_eq!(aabb.middle(), Position::new(0.5, 0.0, 0.0));
    }
}
