use byteorder::{LittleEndian, WriteBytesExt};
use crate::error::{Error, Result};
use std::io::{Write, Seek, SeekFrom};


pub struct Writer<W> {
  writer: W,
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
    Self { writer }
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
  pub fn write_size_optimized_u32(&mut self, value: u32) -> Result<()> {
    if value < 0x80 {
      self.write_u8(value as u8)
    } else if value < 0x4000 {
      self.write_u8((value >> 8) as u8 | 0x80)?;
      self.write_u8(value as u8)
    } else if value < 0x200000 {
      self.write_u8((value >> 16) as u8 | 0xc0)?;
      self.write_u8((value >> 8) as u8)?;
      self.write_u8(value as u8)
    } else if value < 0x10000000 {
      self.write_u8((value >> 24) as u8 | 0xe0)?;
      self.write_u8((value >> 16) as u8)?;
      self.write_u8((value >> 8) as u8)?;
      self.write_u8(value as u8)
    } else {
      self.write_u8(0xf0)?;
      self.write_u8((value >> 24) as u8)?;
      self.write_u8((value >> 16) as u8)?;
      self.write_u8((value >> 8) as u8)?;
      self.write_u8(value as u8)
    }
  }
  pub fn write_compacted_sorted_indices(&mut self, value: &[u32]) -> Result<()> {
    self.write_size_optimized_u32(value.len() as u32)?;
    let mut cur_index = 0;
    while cur_index < value.len() {
      let first_val = if cur_index == 0 { value[cur_index] } else {value[cur_index] - value[cur_index-1] - 1};
      let mut sequence_len = 1;
      while cur_index + 1 < value.len() && value[cur_index + 1] == value[cur_index] + 1 && sequence_len < 100 {
        sequence_len += 1;
        cur_index += 1;
      }
      if sequence_len > 1 {
        self.write_size_optimized_u32(first_val * 2 + 1)?;
        self.write_size_optimized_u32(sequence_len - 1)?;
      } else {
        self.write_size_optimized_u32(first_val * 2)?;
      }
      cur_index += 1;
    }

    Ok(())
  }
  pub fn write_opt_u64(&mut self, value: u64) -> Result<()> {
    if value < 0xff {
      self.write_u8(value as u8)
    } else {
      self.write_u8(0xff)?;
      self.write_u64(value)
    }
  }
  pub fn write_string(&mut self, string: &str) -> Result<()> {
    let bytes: &[u8] = string.as_bytes();
    self.write_opt_u32(bytes.len() as u32)?;
    self.writer.write_all(&bytes).map_err(|e| self.io_error(e))
  }
  pub fn write_immutable_string(&mut self, string: Option<&str>) -> Result<()> {
    match string {
      None => self.write_bool(true),
      Some(string) => {
        self.write_bool(false)?;
        self.write_string(string)
      }
    }
  }
  pub fn write_bytes(&mut self, slice: &[u8]) -> Result<()> {
    self.writer.write_all(slice).map_err(|e| self.io_error(e))
  }
  pub fn write_array<'a, I: IntoIterator<Item=&'a u32>>(&mut self, array: I) -> Result<()> {
    array.into_iter().map(|v| self.write_u32(*v)).collect()
  }

  pub fn into_inner(self) -> W { self.writer }
}
