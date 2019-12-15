pub mod constants;
mod error;
pub mod inputaction;
pub mod map;
mod reader;
pub mod structs;
mod writer;

use std::collections::HashMap;
use std::hash::Hash;
use std::io::{BufRead, Seek, Write};

pub use crate::error::{Error, Result};
pub use crate::reader::Reader;
pub use crate::writer::Writer;
pub use factorio_serialize_derive::{ReadWriteStruct, ReadWriteTaggedUnion, ReadWriteEnumU8, ReadWriteEnumU16};

pub trait ReadWrite: Sized {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self>;
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()>;
  fn map_read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> { Self::read(r) }
  fn map_write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> { self.write(w) }
}

impl ReadWrite for i8 {
  #[inline] fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> { r.read_i8() }
  #[inline] fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> { w.write_i8(*self) }
}
impl ReadWrite for i16 {
  #[inline] fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> { r.read_i16() }
  #[inline] fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> { w.write_i16(*self) }
}
impl ReadWrite for i32 {
  #[inline] fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> { r.read_i32() }
  #[inline] fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> { w.write_i32(*self) }
}

impl ReadWrite for u8 {
  #[inline] fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> { r.read_u8() }
  #[inline] fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> { w.write_u8(*self) }
}
impl ReadWrite for u16 {
  #[inline] fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> { r.read_u16() }
  #[inline] fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> { w.write_u16(*self) }
}
impl ReadWrite for u32 {
  #[inline] fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> { r.read_u32() }
  #[inline] fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> { w.write_u32(*self) }
}
impl ReadWrite for u64 {
  #[inline] fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> { r.read_u64() }
  #[inline] fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> { w.write_u64(*self) }
}
impl ReadWrite for f32 {
  #[inline] fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> { r.read_f32() }
  #[inline] fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> { w.write_f32(*self) }
}
impl ReadWrite for f64 {
  #[inline] fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> { r.read_f64() }
  #[inline] fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> { w.write_f64(*self) }
}

impl ReadWrite for bool {
  #[inline] fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> { r.read_bool() }
  #[inline] fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> { w.write_bool(*self) }
}

impl ReadWrite for String {
  #[inline] fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> { r.read_string() }
  #[inline] fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> { w.write_string(self) }
}

impl<T: ReadWrite> ReadWrite for Option<T> {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    if r.read_bool()? { Ok(Some(T::read(r)?)) } else { Ok(None) }
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_bool(self.is_some())?;
    if let Some(value) = self { value.write(w)?; }
    Ok(())
  }
  fn map_read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    if r.read_bool()? { Ok(Some(T::map_read(r)?)) } else { Ok(None) }
  }
  fn map_write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_bool(self.is_some())?;
    if let Some(value) = self { value.map_write(w)?; }
    Ok(())
  }
}

impl<T: ReadWrite> ReadWrite for Vec<T> {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let len = r.read_opt_u32()?;
    r.read_array(len)
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_opt_u32(self.len() as u32)?;
    w.write_array(self)
  }
  fn map_read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let len = r.read_opt_u32()?;
    r.map_read_array(len)
  }
  fn map_write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_opt_u32(self.len() as u32)?;
    w.map_write_array(self)
  }
}

impl<K: ReadWrite, V: ReadWrite> ReadWrite for (K, V) {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let key = K::read(r)?;
    let value = V::read(r)?;
    Ok((key, value))
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    self.0.write(w)?;
    self.1.write(w)
  }
  fn map_read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let key = K::map_read(r)?;
    let value = V::map_read(r)?;
    Ok((key, value))
  }
  fn map_write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    self.0.map_write(w)?;
    self.1.map_write(w)
  }
}

impl<K: ReadWrite + Eq + Hash, V: ReadWrite> ReadWrite for HashMap<K, V> {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let len = r.read_opt_u32()?;
    let mut result = HashMap::<K, V>::new();
    for _ in 0..len {
      let key = K::read(r)?;
      let value = V::read(r)?;
      result.insert(key, value);
    }
    Ok(result)
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_opt_u32(self.len() as u32)?;
    for (key, value) in self {
      key.write(w)?;
      value.write(w)?;
    }
    Ok(())
  }
  fn map_read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let len = r.read_opt_u32()?;
    let mut result = HashMap::<K, V>::new();
    for _ in 0..len {
      let key = K::map_read(r)?;
      let value = V::map_read(r)?;
      result.insert(key, value);
    }
    Ok(result)
  }
  fn map_write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_opt_u32(self.len() as u32)?;
    for (key, value) in self {
      key.map_write(w)?;
      value.map_write(w)?;
    }
    Ok(())
  }
}