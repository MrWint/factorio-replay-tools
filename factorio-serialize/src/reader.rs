use byteorder::{LittleEndian, ReadBytesExt};
use crate::error::{Error, Result};
use std::io::{BufRead, Seek, SeekFrom};
use std::iter::FromIterator;
use std::string::FromUtf8Error;

pub struct Reader<R> {
  reader: R,
}
impl<R: BufRead + Seek> Reader<R> {
  pub fn position(&mut self) -> u64 {
    self.reader.seek(SeekFrom::Current(0)).unwrap()
  }
  pub fn io_error(&mut self, error: std::io::Error) -> Error {
    Error::from_io(error, self.position())
  }
  pub fn utf8_error_at(&mut self, error: FromUtf8Error, offset: u64) -> Error {
    Error::from_utf8(error, self.position() - offset)
  }
  pub fn error_at(&mut self, error: String, offset: u64) -> Error {
    Error::custom(error, self.position() - offset)
  }

  pub fn new(reader: R) -> Self {
    Self { reader }
  }
  pub fn is_at_eof(&mut self) -> Result<bool> {
    self.reader.fill_buf().map(|x| x.is_empty()).map_err(|e| self.io_error(e))
  }
  #[inline] pub fn read_i8(&mut self) -> Result<i8> { self.reader.read_i8().map_err(|e| self.io_error(e)) }
  #[inline] pub fn read_i16(&mut self) -> Result<i16> { self.reader.read_i16::<LittleEndian>().map_err(|e| self.io_error(e)) }
  #[inline] pub fn read_i32(&mut self) -> Result<i32> { self.reader.read_i32::<LittleEndian>().map_err(|e| self.io_error(e)) }
  #[inline] pub fn read_u8(&mut self) -> Result<u8> { self.reader.read_u8().map_err(|e| self.io_error(e)) }
  #[inline] pub fn read_u16(&mut self) -> Result<u16> { self.reader.read_u16::<LittleEndian>().map_err(|e| self.io_error(e)) }
  #[inline] pub fn read_u32(&mut self) -> Result<u32> { self.reader.read_u32::<LittleEndian>().map_err(|e| self.io_error(e)) }
  #[inline] pub fn read_u64(&mut self) -> Result<u64> { self.reader.read_u64::<LittleEndian>().map_err(|e| self.io_error(e)) }
  #[inline] pub fn read_f32(&mut self) -> Result<f32> { self.reader.read_f32::<LittleEndian>().map_err(|e| self.io_error(e)) }
  #[inline] pub fn read_f64(&mut self) -> Result<f64> { self.reader.read_f64::<LittleEndian>().map_err(|e| self.io_error(e)) }
  pub fn read_bool(&mut self) -> Result<bool> {
    let value = self.read_u8()?;
    if value > 1 { // bools other than 0 or 1 are UB in C++, something went wrong
      Err(self.error_at(format!("value {:#x} is not a valid boolean", value), 1))
    } else {
      Ok(value == 1) // https://wiki.factorio.com/Data_types#bool
    }
  }
  pub fn read_opt_u16(&mut self) -> Result<u16> {
    let tmp = self.read_u8()?;
    if tmp != 0xff {
      Ok(u16::from(tmp))
    } else {
      self.read_u16()
    }
  }
  pub fn read_opt_u32(&mut self) -> Result<u32> {
    let tmp = self.read_u8()?;
    if tmp != 0xff {
      Ok(u32::from(tmp))
    } else {
      self.read_u32()
    }
  }
  pub fn read_size_optimized_u32(&mut self) -> Result<u32> {
    let mut tmp = self.read_u8()?;
    let mut result = 0;
    let mut count = 0;
    while tmp >= 0x80 {
      tmp <<= 1;
      result = (result << 8) | self.read_u8()? as u32;
      count += 1;
    }
    Ok(result | ((tmp as u32) << (count * 7)))
  }
  pub fn read_compacted_sorted_indices(&mut self) -> Result<Vec<u32>> {
    let len = self.read_size_optimized_u32()? as usize;
    let mut result = Vec::with_capacity(len);

    while result.len() < len {
      let mut first_val = self.read_size_optimized_u32()?;
      let sequence_len = if first_val & 1 == 0 { 1 } else { std::cmp::min(self.read_size_optimized_u32()? + 1, 100) };
      first_val = (first_val >> 1) + result.last().map(|&x| x+1).unwrap_or(0);
      for o in 0..sequence_len {
        result.push(first_val + o);
      }
    }

    Ok(result)
  }
  pub fn read_opt_u64(&mut self) -> Result<u64> {
    let tmp = self.read_u8()?;
    if tmp != 0xff {
      Ok(u64::from(tmp))
    } else {
      self.read_u64()
    }
  }
  pub fn read_string(&mut self) -> Result<String> {
    let len = self.read_opt_u32()? as usize;
    let mut bytes = vec![0; len];
    self.reader.read_exact(&mut bytes).map_err(|e| self.io_error(e))?;
    String::from_utf8(bytes).map_err(|e| self.utf8_error_at(e, len as u64 + 1))
  }
  pub fn read_immutable_string(&mut self) -> Result<Option<String>> {
    let is_null = self.read_bool()?;
    if is_null {
      Ok(None)
    } else {
      Ok(Some(self.read_string()?))
    }
  }

  pub fn read_array<C: FromIterator<u32>>(&mut self, len: u32) -> Result<C> {
    (0..len).map(|_| self.read_u32()).collect()
  }

  pub fn into_inner(self) -> R { self.reader }


  pub fn read_to_end(&mut self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    self.reader.read_to_end(&mut bytes).map_err(|e| self.io_error(e))?;
    Ok(bytes)
  }

  pub fn read_u8_assert(&mut self, expected_value: u8) -> Result<u8> {
    let value = self.read_u8()?;
    if value != expected_value {
      Err(self.error_at(format!("value {:#x} does not match expected value {:#?}", value, expected_value), 1))
    } else {
      Ok(value)
    }
  }
  pub fn read_u16_assert(&mut self, expected_value: u16) -> Result<u16> {
    let value = self.read_u16()?;
    if value != expected_value {
      Err(self.error_at(format!("value {:#x} does not match expected value {:#?}", value, expected_value), 2))
    } else {
      Ok(value)
    }
  }
  pub fn read_u32_assert(&mut self, expected_value: u32) -> Result<u32> {
    let value = self.read_u32()?;
    if value != expected_value {
      Err(self.error_at(format!("value {:#x} does not match expected value {:#?}", value, expected_value), 4))
    } else {
      Ok(value)
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_u32() {
    let bytes = [0x11, 0x22, 0x33, 0x44];
    assert_eq!(Reader::new(std::io::Cursor::new(&bytes)).read_u32().unwrap(), 0x44332211);
  }
}
