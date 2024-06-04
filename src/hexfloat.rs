use std::fmt::Display;

pub struct HexFloat(pub f64);

impl Display for HexFloat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut n = self.0;
    if n.is_nan() { return write!(f, "NaN"); }
    if n == 0.0 { return write!(f, "0x0p+0"); }

    if n < 0.0 { write!(f, "-")?; }
    n = n.abs();

    if n.is_infinite() { return write!(f, "inf"); }

    let mut p = 0;
    while n < 1.0 { n *= 16.0; p -= 4; }
    while n >= 16.0 { n /= 16.0; p += 4; }

    write!(f, "0x{:x}", n.trunc() as u8)?;
    n = n.fract();
    if n != 0.0 { write!(f, ".")?; }
    while n != 0.0 {
      n *= 16.0;
      write!(f, "{:x}", n.trunc() as u8)?;
      n = n.fract();
    }
    write!(f, "p{:+}", p)
  }
}


pub struct BinaryFloat(pub f64);

impl Display for BinaryFloat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut n = self.0;
    if n.is_nan() { return write!(f, "NaN"); }
    if n == 0.0 { return write!(f, "0x0p+0"); }

    if n < 0.0 { write!(f, "-")?; }
    n = n.abs();

    if n.is_infinite() { return write!(f, "inf"); }

    let mut p = 0;
    while n < 1.0 { n *= 2.0; p -= 1; }
    while n >= 2.0 { n /= 2.0; p += 1; }

    write!(f, "0x{:x}", n.trunc() as u8)?;
    n = n.fract();
    if n != 0.0 { write!(f, ".")?; }
    while n != 0.0 {
      n *= 16.0;
      write!(f, "{:x}", n.trunc() as u8)?;
      n = n.fract();
    }
    write!(f, "p{:+}", p)
  }
}