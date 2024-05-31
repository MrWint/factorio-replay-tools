// Implementation of Factorio's Vector class

use std::ops::{Add, Mul};

use factorio_serialize_derive::{MapReadWriteStruct, ReplayReadWriteStruct};

use crate::replay::Direction;

#[derive(Clone, Copy, Debug, MapReadWriteStruct, ReplayReadWriteStruct)]
pub struct Vector {
  pub x: f64,
  pub y: f64,
}
impl Vector {
  pub fn new(x: f64, y: f64) -> Self {
    Vector { x, y }
  }
  // from Vector::directionMultiplicators array
  pub fn direction_multiplicators(direction: Direction) -> Vector {
    match direction {
        Direction::North => Vector { x: 0.0, y: -1.0 },
        Direction::NorthEast => Vector { x: 0.70710678118, y: -0.70710678118 },
        Direction::East => Vector { x: 1.0, y: 0.0 },
        Direction::SouthEast => Vector { x: 0.70710678118, y: 0.70710678118 },
        Direction::South => Vector { x: 0.0, y: 1.0 },
        Direction::SouthWest => Vector { x: -0.70710678118, y: 0.70710678118 },
        Direction::West => Vector { x: -1.0, y: 0.0 },
        Direction::NorthWest => Vector { x: -0.70710678118, y: -0.70710678118 },
        _ => panic!("invalid direction {:?}", direction)
    }
  }
  pub fn is_orthogonal(&self) -> bool {
    self.x == 0.0 || self.y == 0.0
  }
}
impl Mul<f64> for Vector {
  type Output = Vector;
  fn mul(self, rhs: f64) -> Self::Output {
    Vector { x: self.x * rhs, y: self.y * rhs }
  }
}
impl Add<Vector> for Vector {
  type Output = Vector;
  fn add(self, rhs: Vector) -> Self::Output {
    Vector { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}