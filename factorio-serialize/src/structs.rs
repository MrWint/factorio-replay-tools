use factorio_serialize_derive::MapReadWriteStruct;
use factorio_serialize_derive::ReplayReadWriteStruct;

mod boundingbox;
pub use boundingbox::BoundingBox;
mod fixedpointnumber;
pub use fixedpointnumber::FixedPoint32_8;
mod mapposition;
pub use mapposition::MapPosition;
mod randomgenerator;
pub use randomgenerator::RandomGenerator;
mod vector;
pub use vector::Vector;
mod vectororientation;
pub use vectororientation::VectorOrientation;

#[derive(Clone, Debug, MapReadWriteStruct, PartialEq)]
pub struct ChunkPosition {
  pub x: i32,
  pub y: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ReplayReadWriteStruct)]
pub struct TilePosition {
  pub x: i32,
  pub y: i32,
}
impl TilePosition {
  pub const fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }
  pub const fn center_map_position(&self) -> MapPosition {
    MapPosition::new( FixedPoint32_8(self.x * 0x100 + 0x80), FixedPoint32_8(self.y * 0x100 + 0x80))
  }
  pub fn top_left_map_position(&self) -> MapPosition {
    MapPosition::new(FixedPoint32_8(self.x * 0x100), FixedPoint32_8(self.y * 0x100))
  }
  pub fn to_chunk_position(&self) -> ChunkPosition {
    ChunkPosition { x: self.x >> 5, y: self.y >> 5 }
  }
}
