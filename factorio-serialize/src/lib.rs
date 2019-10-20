mod error;
mod reader;
mod writer;

use std::io::{BufRead, Seek, Write};

pub use crate::error::{Error, Result};
pub use crate::reader::Reader;
pub use crate::writer::Writer;
pub use factorio_serialize_derive::{ReadWriteEnumU8, ReadWriteEnumU16};

pub trait ReadWrite: Sized {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self>;
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()>;
}

impl ReadWrite for u32 {
  #[inline] fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> { r.read_u32() }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> { w.write_u32(*self) }
}
