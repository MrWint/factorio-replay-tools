use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::action::*;
use crate::constants::*;
use num_traits::{FromPrimitive, ToPrimitive};
use std::io::{Cursor, Error, ErrorKind, Read, Result, Write};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ReplayItem {
  tick: u32,
  player_id: u16,
  action: InputAction,
}
impl ReplayItem {
  pub fn new(tick: u32, player_id: u16, action: InputAction) -> Self {
    Self { tick, player_id, action }
  }
}

pub fn parse_replay(replay_bytes: Vec<u8>) -> Vec<ReplayItem> {
  let mut read = ReplayReader::new(replay_bytes);
  let mut result = vec![];
  while !read.is_at_eof() {
    if let Some(replay_item) = parse_replay_item(&mut read).unwrap() {
      result.push(replay_item);
    }
  }
  result
}

fn parse_replay_item(read: &mut ReplayReader) -> Result<Option<ReplayItem>> {
  let action_type_pos = read.position();
  let action_type = InputActionType::from_u8(read.read_u8()?).unwrap();
  let tick = read.read_u32::<LittleEndian>()?;
  let player_id = read.read_opt_u16()?;
  if let Some(action) = InputAction::read(action_type, action_type_pos, read)? {
    assert!(action_type == action.action_type());
    Ok(Some(ReplayItem { tick, player_id, action }))
  } else {
    Ok(None)
  }
}

pub fn write_replay(replay_items: Vec<ReplayItem>) -> Vec<u8> {
  let mut write = ReplayWriter::new();
  for replay_item in replay_items {
    write_replay_item(&mut write, replay_item).unwrap();
  }
  write.into_inner()
}

fn write_replay_item(write: &mut ReplayWriter, replay_item: ReplayItem) -> Result<()> {
  write.write_u8(replay_item.action.action_type().to_u8().unwrap())?;
  write.write_u32::<LittleEndian>(replay_item.tick)?;
  write.write_opt_u16(replay_item.player_id)?;
  replay_item.action.write(write)
}


pub struct ReplayReader {
  reader: Cursor<Vec<u8>>,
}
impl ReplayReader {
  pub fn new(bytes: Vec<u8>) -> Self {
    Self { reader: Cursor::new(bytes) }
  }
  pub fn is_at_eof(&self) -> bool {
    let len = self.reader.get_ref().len();
    self.reader.position() >= len as u64
  }
  pub fn read_u8_assert(&mut self, expected_value: u8) -> Result<u8> {
    let value = self.reader.read_u8()?;
    if value != expected_value {
      panic!("value {:#x} at position {} does not match expected value {:#?}", value, self.position() - 1, expected_value)
    } else {
      Ok(value)
    }
  }
  pub fn read_u16_assert(&mut self, expected_value: u16) -> Result<u16> {
    let value = self.reader.read_u16::<LittleEndian>()?;
    if value != expected_value {
      Err(Error::new(ErrorKind::NotFound, format!("value {:#x} at position {} does not match expected value {:#?}", value, self.position() - 2, expected_value)))
    } else {
      Ok(value)
    }
  }
  pub fn read_u32_assert(&mut self, expected_value: u32) -> Result<u32> {
    let value = self.reader.read_u32::<LittleEndian>()?;
    if value != expected_value {
      Err(Error::new(ErrorKind::NotFound, format!("value {:#x} at position {} does not match expected value {:#?}", value, self.position() - 4, expected_value)))
    } else {
      Ok(value)
    }
  }
  pub fn read_opt_u16(&mut self) -> Result<u16> {
    let tmp = self.reader.read_u8()?;
    if tmp != 0xff {
      Ok(u16::from(tmp))
    } else {
      self.reader.read_u16::<LittleEndian>()
    }
  }
  pub fn read_opt_u32(&mut self) -> Result<u32> {
    let tmp = self.reader.read_u8()?;
    if tmp != 0xff {
      Ok(u32::from(tmp))
    } else {
      self.reader.read_u32::<LittleEndian>()
    }
  }
  pub fn read_string(&mut self) -> Result<String> {
    let len = self.read_opt_u32()? as usize;
    let mut bytes = vec![];
    bytes.resize_with(len, Default::default);
    self.read_exact(&mut bytes)?;
    Ok(String::from_utf8(bytes).unwrap())
  }
  pub fn read_past_add_blueprint_record_data(&mut self) -> Result<()> { // AddBlueprintRecordData
    self.read_u32::<LittleEndian>()?;
    for _ in 0..20 { self.read_u8()?; } // SHA1Digest
    self.read_u16::<LittleEndian>()?; // fixed point number
    let is_book = self.read_u8()?;
    let signal_id_count = self.read_opt_u32()?;
    for _ in 0..signal_id_count { self.read_past_signal_id()?; }
    self.read_string()?; // name
    self.read_u32::<LittleEndian>()?;
    if is_book != 0 {
      let blueprint_count = self.read_opt_u32()?;
      for _ in 0..blueprint_count { self.read_past_single_record_data_in_book()?; }
    }
    Ok(())
  }
  pub fn read_past_single_record_data_in_book(&mut self) -> Result<()> { // SingleRecordDataInBook
    self.read_u32::<LittleEndian>()?;
    self.read_u16::<LittleEndian>()?; // fixed point number
    for _ in 0..20 { self.read_u8()?; } // SHA1Digest
    let signal_id_count = self.read_opt_u32()?;
    for _ in 0..signal_id_count { self.read_past_signal_id()?; }
    self.read_string()?; // name
    Ok(())
  }
  pub fn read_past_update_blueprint_data(&mut self) -> Result<()> { // UpdateBlueprintData
    self.read_u32::<LittleEndian>()?;
    for _ in 0..20 { self.read_u8()?; } // SHA1Digest
    self.read_string()?; // name
    Ok(())
  }
  fn read_past_signal_id(&mut self) -> Result<()> { // SignalID
    self.read_u8()?; // some sort of type
    self.read_u16::<LittleEndian>()?; // fixed point number
    Ok(())
  }
}
impl Deref for ReplayReader {
  type Target = Cursor<Vec<u8>>;
  fn deref(&self) -> &Self::Target {
    &self.reader
  }
}
impl DerefMut for ReplayReader {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.reader
  }
}

pub struct ReplayWriter {
  writer: Cursor<Vec<u8>>,
}
impl ReplayWriter {
  fn new() -> Self {
    Self { writer: Cursor::new(vec![]) }
  }
  pub fn write_opt_u16(&mut self, value: u16) -> Result<()> {
    if value < 0xff {
      self.write_u8(value as u8)
    } else {
      self.write_u8(0xff)?;
      self.write_u16::<LittleEndian>(value)
    }
  }
  pub fn write_opt_u32(&mut self, value: u32) -> Result<()> {
    if value < 0xff {
      self.write_u8(value as u8)
    } else {
      self.write_u8(0xff)?;
      self.write_u32::<LittleEndian>(value)
    }
  }
  pub fn write_string(&mut self, string: &str) -> Result<()> {
    let bytes: &[u8] = string.as_bytes();
    assert!(bytes.len() <= std::u32::MAX as usize, "String {} too long", string);
    self.write_opt_u32(bytes.len() as u32)?;
    self.write_all(&bytes)
  }
  fn into_inner(self) -> Vec<u8> { self.writer.into_inner() }
}
impl Deref for ReplayWriter {
  type Target = Cursor<Vec<u8>>;
  fn deref(&self) -> &Self::Target {
    &self.writer
  }
}
impl DerefMut for ReplayWriter {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.writer
  }
}

