use std::{io::{BufRead, Seek}, ops::{Add, Sub}};

use crate::{map::{MapDeserialiser, MapReadWrite, MapSerialiser}, replay::ReplayReadWrite, ChunkPosition, FixedPoint32_8, Result, TilePosition, Vector};

#[derive(Clone, Copy, Debug, Default)]
pub struct MapPosition {
  pub x: FixedPoint32_8,
  pub y: FixedPoint32_8,

  deserialized_relative: bool,
}

impl MapPosition {
  pub fn new(x: FixedPoint32_8, y: FixedPoint32_8) -> Self {
    Self { x, y, deserialized_relative: false }
  }
  pub fn to_tile_position(&self) -> TilePosition {
    TilePosition { x: self.x.0 >> 8, y: self.y.0 >> 8 }
  }
  pub fn to_chunk_position(&self) -> ChunkPosition {
    ChunkPosition { x: self.x.0 >> 13, y: self.y.0 >> 13 }
  }
}
impl Add<Vector> for MapPosition {
  type Output = MapPosition;
  fn add(self, rhs: Vector) -> Self::Output {
    MapPosition::new(self.x + FixedPoint32_8::from_double(rhs.x), self.y + FixedPoint32_8::from_double(rhs.y))      
  }
}
impl Add<MapPosition> for MapPosition {
  type Output = MapPosition;
  fn add(self, rhs: MapPosition) -> Self::Output {
    MapPosition::new(self.x + rhs.x, self.y + rhs.y)      
  }
}
impl Sub<MapPosition> for MapPosition {
  type Output = MapPosition;
  fn sub(self, rhs: MapPosition) -> Self::Output {
    MapPosition::new(self.x - rhs.x, self.y - rhs.y)      
  }
}


// parsing logic
impl MapReadWrite for MapPosition {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    let dx = input.stream.read_i16()?;

    let map_position = if dx == 0x7fff {
      let x = FixedPoint32_8(input.stream.read_i32()?);
      let y = FixedPoint32_8(input.stream.read_i32()?);
      MapPosition { x, y, deserialized_relative: false }
    } else {
      let dy = input.stream.read_i16()?;
      MapPosition { x: input.last_loaded_position.x + FixedPoint32_8(dx as i32), y: input.last_loaded_position.y + FixedPoint32_8(dy as i32), deserialized_relative: true }
    };
    input.last_loaded_position = map_position.clone();
    Ok(map_position)
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    if self.deserialized_relative {
      let dx = self.x - input.last_saved_position.x;
      let dy = self.y - input.last_saved_position.y;
      assert!(dx.0.abs() < 0x7ffe && dy.0.abs() < 0x7ffe);  // based on MapPosition::saveInternal
      input.stream.write_i16(dx.0 as i16)?;
      input.stream.write_i16(dy.0 as i16)?;
    } else {
      input.stream.write_i16(0x7fff)?;
      input.stream.write_i32(self.x.0)?;
      input.stream.write_i32(self.y.0)?;
    }
    input.last_saved_position = self.clone();
    Ok(())
  }
}
impl ReplayReadWrite for MapPosition {
  fn replay_read<R: BufRead + Seek>(input: &mut crate::replay::ReplayDeserialiser<R>) -> Result<Self> {
    let x = FixedPoint32_8(input.stream.read_i32()?);
    let y = FixedPoint32_8(input.stream.read_i32()?);
    Ok(MapPosition { x, y, deserialized_relative: false })
  }
  fn replay_write(&self, input: &mut crate::replay::ReplaySerialiser) -> Result<()> {
    input.stream.write_i32(self.x.0)?;
    input.stream.write_i32(self.y.0)?;
    Ok(())
  }
}
  