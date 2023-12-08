use std::io::BufRead;
use std::io::Seek;

use factorio_serialize_derive::MapReadWriteStruct;
use factorio_serialize_derive::ReplayReadWriteStruct;

use crate::Result;
use crate::map::MapDeserialiser;
use crate::map::MapReadWrite;
use crate::map::MapSerialiser;
use crate::replay::ReplayReadWrite;


#[derive(Clone, Debug, MapReadWriteStruct, PartialEq)]
pub struct ChunkPosition {
  pub x: i32,
  pub y: i32,
}

#[derive(Clone, Copy, Debug, ReplayReadWriteStruct)]
pub struct TilePosition {
  x: i32,
  y: i32,
}
impl TilePosition {
  pub fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }
  pub fn center_map_position(&self) -> MapPosition {
    MapPosition::new(self.x * 0x100 + 0x80, self.y * 0x100 + 0x80)
  }
  pub fn top_left_map_position(&self) -> MapPosition {
    MapPosition::new(self.x * 0x100, self.y * 0x100)
  }
}

type FixedPoint32_8 = i32;  // *1/256, .5 rounded away from 0

#[derive(Clone, Copy, Debug, Default)]
pub struct MapPosition {
  pub x: FixedPoint32_8,
  pub y: FixedPoint32_8,

  deserialized_relative: bool,
}
impl MapReadWrite for MapPosition {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    let dx = input.stream.read_i16()?;

    let map_position = if dx == 0x7fff {
      let x = input.stream.read_i32()?;
      let y = input.stream.read_i32()?;
      MapPosition { x, y, deserialized_relative: false }
    } else {
      let dy = input.stream.read_i16()?;
      MapPosition { x: input.last_loaded_position.x + dx as i32, y: input.last_loaded_position.y + dy as i32, deserialized_relative: true }
    };
    input.last_loaded_position = map_position.clone();
    Ok(map_position)
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    if self.deserialized_relative {
      let dx = self.x - input.last_saved_position.x;
      let dy = self.y - input.last_saved_position.y;
      assert!(dx.abs() < 0x7ffe && dy.abs() < 0x7ffe);  // based on MapPosition::saveInternal
      input.stream.write_i16(dx as i16)?;
      input.stream.write_i16(dy as i16)?;
    } else {
      input.stream.write_i16(0x7fff)?;
      input.stream.write_i32(self.x)?;
      input.stream.write_i32(self.y)?;
    }
    input.last_saved_position = self.clone();
    Ok(())
  }
}
impl ReplayReadWrite for MapPosition {
  fn replay_read<R: BufRead + Seek>(input: &mut crate::replay::ReplayDeserialiser<R>) -> Result<Self> {
    let x = input.stream.read_i32()?;
    let y = input.stream.read_i32()?;
    Ok(MapPosition { x, y, deserialized_relative: false })
  }
  fn replay_write(&self, input: &mut crate::replay::ReplaySerialiser) -> Result<()> {
    input.stream.write_i32(self.x)?;
    input.stream.write_i32(self.y)?;
    Ok(())
  }
}

impl MapPosition {
  pub fn new(x: FixedPoint32_8, y: FixedPoint32_8) -> Self {
    Self { x, y, deserialized_relative: false }
  }
  pub fn to_chunk_position(&self) -> ChunkPosition {
    ChunkPosition { x: self.x / 0x2000, y: self.y / 0x2000 }
  }
}
