pub mod constants;
mod error;
pub mod inputaction;
pub mod map;
mod reader;
mod writer;

use std::io::{BufRead, Seek, Write};

pub use crate::error::{Error, Result};
pub use crate::reader::Reader;
pub use crate::writer::Writer;
pub use factorio_serialize_derive::{ReadWriteStruct, ReadWriteTaggedUnion, ReadWriteEnumU8, ReadWriteEnumU16};

pub trait ReadWrite: Sized {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self>;
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()>;
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

impl<T: ReadWrite> ReadWrite for Vec<T> {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let len = r.read_opt_u32()?;
    let mut result = Vec::<T>::new();
    for _ in 0..len {
      result.push(T::read(r)?);
    }
    Ok(result)
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_opt_u32(self.len() as u32)?;
    for item in self {
      item.write(w)?;
    }
    Ok(())
  }
}