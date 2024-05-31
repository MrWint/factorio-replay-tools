use factorio_serialize_derive::{MapReadWriteStruct, ReplayReadWriteStruct};

use crate::{replay::Direction, FixedPoint32_8, MapPosition, TilePosition};

use super::VectorOrientation;

#[derive(Clone, Debug, MapReadWriteStruct, ReplayReadWriteStruct)]
pub struct BoundingBox {
  pub left_top: MapPosition,
  pub right_bottom: MapPosition,
  pub orientation: VectorOrientation,
}
impl BoundingBox {
  pub fn offset(&self, offset: MapPosition) -> BoundingBox {
    BoundingBox { left_top: self.left_top + offset, right_bottom: self.right_bottom + offset, orientation: self.orientation }
  }

  // from BoundingBox::isZero
  pub fn is_zero(&self) -> bool {
    self.left_top.x == self.right_bottom.x && self.left_top.y == self.right_bottom.y
  }

  // from BoundingBox::getAABB
  pub fn get_aabb(&self) -> BoundingBox {
    assert!(self.orientation.x == 0, "rotated bounding boxes are not supported");
    self.clone()
  }

  // from BoundingBox::getRotatedVertices
  pub fn get_rotated_vertices(&self) -> (MapPosition, MapPosition, MapPosition, MapPosition) {
    assert!(self.orientation.x == 0, "rotated bounding boxes are not supported");
    let left_top = self.left_top;
    let right_top = MapPosition::new(self.right_bottom.x, self.left_top.y);
    let left_bottom = MapPosition::new(self.left_top.x, self.right_bottom.y);
    let right_bottom = self.right_bottom;
    (left_top, right_top, left_bottom, right_bottom)
  }

  // from BoundingBox::BoundingBox
  pub fn with_direction(&self, direction: Direction) -> BoundingBox {
    match direction {
      Direction::East => BoundingBox { left_top: MapPosition::new(-self.right_bottom.y, self.left_top.x), right_bottom: MapPosition::new(-self.left_top.y, self.right_bottom.x), orientation: VectorOrientation::north() },
      Direction::South => BoundingBox { left_top: MapPosition::new(-self.right_bottom.x, -self.right_bottom.y), right_bottom: MapPosition::new(-self.left_top.x, -self.left_top.y), orientation: VectorOrientation::north() },
      Direction::West => BoundingBox { left_top: MapPosition::new(self.left_top.y, -self.right_bottom.x), right_bottom: MapPosition::new(self.right_bottom.y, -self.left_top.x), orientation: VectorOrientation::north() },
      _ => BoundingBox { orientation: VectorOrientation::north(), ..*self },
    }
  }

  // from BoundingBox::extend
  pub fn extend(&mut self, other: &BoundingBox) {
    let this_aabb = self.get_aabb();
    let other_aabb = other.get_aabb();
    self.left_top.x = std::cmp::min(this_aabb.left_top.x, other_aabb.left_top.x);
    self.left_top.y = std::cmp::min(this_aabb.left_top.y, other_aabb.left_top.y);
    self.right_bottom.x = std::cmp::max(this_aabb.right_bottom.x, other_aabb.right_bottom.x);
    self.right_bottom.y = std::cmp::max(this_aabb.right_bottom.y, other_aabb.right_bottom.y);
  }

  // from BoundingBox::tileBox
  pub fn tile_box(tile_position: TilePosition, offset: f64) -> BoundingBox {
    let offset = FixedPoint32_8::from_double(offset);
    BoundingBox {
      left_top: MapPosition::new(FixedPoint32_8(tile_position.x * 0x100) - offset, FixedPoint32_8(tile_position.y * 0x100) - offset),
      right_bottom: MapPosition::new(FixedPoint32_8((tile_position.x + 1) * 0x100) + offset, FixedPoint32_8((tile_position.y + 1) * 0x100) + offset),
      orientation: VectorOrientation::north(),
    }
  }

  // from BoundingBox::collide
  pub fn collide_bounding_box(&self, other: &BoundingBox) -> bool {
    assert!(self.orientation.x == 0 && other.orientation.x == 0, "rotated bounding boxes are not supported");
    self.left_top.x <= other.right_bottom.x &&
    self.left_top.y <= other.right_bottom.y &&
    other.left_top.x <= self.right_bottom.x &&
    other.left_top.y <= self.right_bottom.y
  }

  // from BoundingBox::collide
  pub fn collide_point(&self, other: &MapPosition) -> bool {
    if self.is_zero() {
      return false;
    }
    assert!(self.orientation.x == 0, "rotated bounding boxes are not supported");
    self.left_top.x <= other.x &&
    self.left_top.y <= other.y &&
    other.x <= self.right_bottom.x &&
    other.y <= self.right_bottom.y
  }

  // from BoundingBox::distanceFromPointSquared
  pub fn distance_from_point_squared(&self, point: &MapPosition) -> f64 {
    assert!(self.orientation.x == 0, "rotated bounding boxes are not supported");
    let double_x_dist = ((point.x.0 * 2 - self.left_top.x.0 - self.right_bottom.x.0).abs() - (self.right_bottom.x.0 - self.left_top.x.0)).max(0) as i64;
    let double_y_dist = ((point.y.0 * 2 - self.left_top.y.0 - self.right_bottom.y.0).abs() - (self.right_bottom.y.0 - self.left_top.y.0)).max(0) as i64;
    (double_x_dist * double_x_dist + double_y_dist * double_y_dist) as f64 * (1.0 / 262144.0)
  }

  // from BoundingBox::distanceFromPoint
  pub fn distance_from_point(&self, point: &MapPosition) -> f64 {
    assert!(self.orientation.x == 0, "rotated bounding boxes are not supported");
    let double_x_dist = ((point.x.0 * 2 - self.left_top.x.0 - self.right_bottom.x.0).abs() - (self.right_bottom.x.0 - self.left_top.x.0)).max(0) as i64;
    let double_y_dist = ((point.y.0 * 2 - self.left_top.y.0 - self.right_bottom.y.0).abs() - (self.right_bottom.y.0 - self.left_top.y.0)).max(0) as i64;
    if double_x_dist == 0 {
      double_y_dist as f64 * (1.0 / 512.0)
    } else if double_y_dist == 0 {
      double_x_dist as f64 * (1.0 / 512.0)
    } else {
      ((double_x_dist * double_x_dist + double_y_dist * double_y_dist) as f64).sqrt() * (1.0 / 512.0)
    }
  }

  // from BoundingBox::distance (loosely)
  pub fn distance_from_bounding_box(&self, other: &BoundingBox) -> f64 {
    let double_x_dist = ((other.left_top.x.0 + other.right_bottom.x.0 - self.left_top.x.0 - self.right_bottom.x.0).abs() - (self.right_bottom.x.0 - self.left_top.x.0) - (other.right_bottom.x.0 - other.left_top.x.0)).max(0) as i64;
    let double_y_dist = ((other.left_top.y.0 + other.right_bottom.y.0 - self.left_top.y.0 - self.right_bottom.y.0).abs() - (self.right_bottom.y.0 - self.left_top.y.0) - (other.right_bottom.y.0 - other.left_top.y.0)).max(0) as i64;
    if double_x_dist == 0 {
      double_y_dist as f64 * (1.0 / 512.0)
    } else if double_y_dist == 0 {
      double_x_dist as f64 * (1.0 / 512.0)
    } else {
      ((double_x_dist * double_x_dist + double_y_dist * double_y_dist) as f64).sqrt() * (1.0 / 512.0)
    }
  }
}