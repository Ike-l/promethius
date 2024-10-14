use std::cmp::Ordering;

use small_derive_deref::{
    Deref, DerefMut
};

#[derive(Debug, Deref, DerefMut, Clone, PartialEq)]
pub struct Position {
    pub position: cgmath::Point3<f64>
}

impl Default for Position {
    fn default() -> Self {
        Self {
            position: cgmath::Point3 { x: 0.0, y: 0.0, z: 0.0 }
        }
    }
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            position: cgmath::Point3 { x, y, z }
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(
            self.x + other.x, 
            self.y + other.y,
            self.z + other.z
        )
    }
}

impl PartialOrd for Position {
    fn ge(&self, other: &Self) -> bool {
        self.x >= other.x && self.y >= other.y && self.z >= other.z
    }
    fn gt(&self, other: &Self) -> bool {
        self.x > other.x && self.y > other.y && self.z > other.z
    }
    fn le(&self, other: &Self) -> bool {
        self.x <= other.x && self.y <= other.y && self.z <= other.z
    }
    fn lt(&self, other: &Self) -> bool {
        self.x < other.x && self.y < other.y && self.z < other.z
    }
    
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let x_cmp = self.position.x.partial_cmp(&other.position.x);
        if let Some(Ordering::Equal) = x_cmp {
            let y_cmp = self.position.y.partial_cmp(&other.position.y);
            if let Some(Ordering::Equal) = y_cmp {
                return self.position.z.partial_cmp(&other.position.z);
            }
            return y_cmp;
        }
        x_cmp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_partial_eq_test() {
        let pos1 = Position::new(1.0, 2.0, 3.0);
        let pos2 = pos1.clone();
        let pos3 = Position::new(4.0, 5.0, 6.0);

        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn partial_cmp_test() {
        let pos1 = Position::new(1.0, 2.0, 3.0);
        let pos2 = pos1.clone();
        let pos3 = Position::new(2.0, 2.0, 3.0);
        let pos4 = Position::new(1.0, 3.0, 3.0);
        let pos5 = Position::new(1.0, 2.0, 4.0);

        assert_eq!(pos1.partial_cmp(&pos2), Some(Ordering::Equal));
        assert_eq!(pos1.partial_cmp(&pos3), Some(Ordering::Less));
        assert_eq!(pos3.partial_cmp(&pos1), Some(Ordering::Greater));

        assert_eq!(pos1.partial_cmp(&pos4), Some(Ordering::Less));
        assert_eq!(pos4.partial_cmp(&pos1), Some(Ordering::Greater));

        assert_eq!(pos1.partial_cmp(&pos5), Some(Ordering::Less));
        assert_eq!(pos5.partial_cmp(&pos1), Some(Ordering::Greater));
    }

    #[test]
    fn test_position_gt_lt_ge_le() {
        let pos1 = Position::new(1.0, 2.0, 3.0);
        let pos2 = pos1.clone();
        let pos3 = Position::new(2.0, 3.0, 4.0);
        let pos4 = Position::new(0.0, 1.0, 2.0);
        let pos5 = Position::new(0.0, 3.0, 1.0);

        assert!(pos1 >= pos2);
        assert!(pos1 <= pos2);
        
        assert!(pos3 > pos1);
        assert!(pos4 < pos1);

        assert!(!(pos1 > pos3));
        assert!(!(pos1 < pos4));

        assert!(!(pos5 > pos1));
        assert!(!(pos5 < pos1));
        assert!(!(pos5 == pos1));
    }
}