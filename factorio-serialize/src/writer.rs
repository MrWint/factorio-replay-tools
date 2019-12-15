use byteorder::{LittleEndian, WriteBytesExt};
use crate::ReadWrite;
use crate::error::{Error, Result};
use crate::structs::MapPosition;
use std::io::{Write, Seek, SeekFrom};


pub struct Writer<W> {
  writer: W,
  pub last_saved_position: MapPosition,
}
impl<W: Write + Seek> Writer<W> {
  pub fn position(&mut self) -> u64 {
    self.writer.seek(SeekFrom::Current(0)).unwrap()
  }
  pub fn io_error(&mut self, error: std::io::Error) -> Error {
    Error::from_io(error, self.position())
  }
  pub fn error_at(&mut self, error: String, offset: u64) -> Error {
    Error::custom(error, self.position() - offset)
  }

  pub fn new(writer: W) -> Self {
    Self { writer, last_saved_position: MapPosition::new(0, 0) }
  }

  #[inline] pub fn write_i8(&mut self, value: i8) -> Result<()> { self.writer.write_i8(value).map_err(|e| self.io_error(e)) }
  #[inline] pub fn write_i16(&mut self, value: i16) -> Result<()> { self.writer.write_i16::<LittleEndian>(value).map_err(|e| self.io_error(e)) }
  #[inline] pub fn write_i32(&mut self, value: i32) -> Result<()> { self.writer.write_i32::<LittleEndian>(value).map_err(|e| self.io_error(e)) }
  #[inline] pub fn write_u8(&mut self, value: u8) -> Result<()> { self.writer.write_u8(value).map_err(|e| self.io_error(e)) }
  #[inline] pub fn write_u16(&mut self, value: u16) -> Result<()> { self.writer.write_u16::<LittleEndian>(value).map_err(|e| self.io_error(e)) }
  #[inline] pub fn write_u32(&mut self, value: u32) -> Result<()> { self.writer.write_u32::<LittleEndian>(value).map_err(|e| self.io_error(e)) }
  #[inline] pub fn write_u64(&mut self, value: u64) -> Result<()> { self.writer.write_u64::<LittleEndian>(value).map_err(|e| self.io_error(e)) }
  #[inline] pub fn write_f32(&mut self, value: f32) -> Result<()> { self.writer.write_f32::<LittleEndian>(value).map_err(|e| self.io_error(e)) }
  #[inline] pub fn write_f64(&mut self, value: f64) -> Result<()> { self.writer.write_f64::<LittleEndian>(value).map_err(|e| self.io_error(e)) }

  pub fn write_bool(&mut self, value: bool) -> Result<()> {
    self.write_u8(if value { 1 } else { 0 })
  }
  pub fn write_opt_u16(&mut self, value: u16) -> Result<()> {
    if value < 0xff {
      self.write_u8(value as u8)
    } else {
      self.write_u8(0xff)?;
      self.write_u16(value)
    }
  }
  pub fn write_opt_u32(&mut self, value: u32) -> Result<()> {
    if value < 0xff {
      self.write_u8(value as u8)
    } else {
      self.write_u8(0xff)?;
      self.write_u32(value)
    }
  }
  pub fn write_string(&mut self, string: &str) -> Result<()> {
    let bytes: &[u8] = string.as_bytes();
    self.write_opt_u32(bytes.len() as u32)?;
    self.writer.write_all(&bytes).map_err(|e| self.io_error(e))
  }
  pub fn write_array<'a, T: 'a + ReadWrite, I: IntoIterator<Item=&'a T>>(&mut self, array: I) -> Result<()> {
    array.into_iter().map(|v| v.write(self)).collect()
  }
  pub fn map_write_array<'a, T: 'a + ReadWrite, I: IntoIterator<Item=&'a T>>(&mut self, array: I) -> Result<()> {
    array.into_iter().map(|v| v.map_write(self)).collect()
  }

  pub fn into_inner(self) -> W { self.writer }
}
