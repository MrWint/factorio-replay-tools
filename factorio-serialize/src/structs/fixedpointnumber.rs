use std::{io::{BufRead, Seek}, ops::{Add, Neg, Sub}};

use crate::{map::{MapDeserialiser, MapReadWrite, MapSerialiser}, replay::ReplayReadWrite, Result};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FixedPoint32_8(pub i32);
impl FixedPoint32_8 {
  pub fn from_double(value: f64) -> Self {
    let value = value * 256.0;
    assert!(!value.is_infinite());
    FixedPoint32_8(value.max(-2147483648.0).min(2147483647.0) as i32)
  }
}

impl Add<FixedPoint32_8> for FixedPoint32_8 {
  type Output = FixedPoint32_8;
  fn add(self, rhs: FixedPoint32_8) -> Self::Output {
    FixedPoint32_8(self.0 + rhs.0)
  }
}
impl Sub<FixedPoint32_8> for FixedPoint32_8 {
  type Output = FixedPoint32_8;
  fn sub(self, rhs: FixedPoint32_8) -> Self::Output {
    FixedPoint32_8(self.0 - rhs.0)
  }
}
impl Neg for FixedPoint32_8 {
  type Output = FixedPoint32_8;
  fn neg(self) -> Self::Output {
    FixedPoint32_8(-self.0)
  }
}



// parsing logic
impl MapReadWrite for FixedPoint32_8 {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    Ok(FixedPoint32_8(input.stream.read_i32()?))
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    input.stream.write_i32(self.0)
  }
}
impl ReplayReadWrite for FixedPoint32_8 {
  fn replay_read<R: BufRead + Seek>(input: &mut crate::replay::ReplayDeserialiser<R>) -> Result<Self> {
    Ok(FixedPoint32_8(input.stream.read_i32()?))
  }
  fn replay_write(&self, input: &mut crate::replay::ReplaySerialiser) -> Result<()> {
    input.stream.write_i32(self.0)
  }
}