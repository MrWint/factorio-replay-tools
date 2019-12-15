use crate as factorio_serialize;
use factorio_serialize::{ReadWrite,ReadWriteStruct, Reader, Result, Writer};
use crate::constants::*;
use std::io::{BufRead, Seek, Write};

type FixedPoint32 = i32;
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MapPosition { // in 1/256th tiles
  pub x: FixedPoint32,
  pub y: FixedPoint32,
}
impl MapPosition {
  pub fn new(x: FixedPoint32, y: FixedPoint32) -> Self {
    Self { x, y }
  }
}
impl ReadWrite for MapPosition {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let x = i32::read(r)?;
    let y = i32::read(r)?;
    Ok( Self { x, y })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    self.x.write(w)?;
    self.y.write(w)
  }
  fn map_read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let dx = i16::read(r)?;
    let (x, y) = if dx == 0x7fff {
      let x = i32::read(r)?;
      let y = i32::read(r)?;
      (x, y)
    } else {
      let dy = i16::read(r)?;
      (r.last_loaded_position.x + i32::from(dx), r.last_loaded_position.y + i32::from(dy))
    };
    r.last_loaded_position = MapPosition::new(x, y);
    Ok( Self { x, y })
  }
  fn map_write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    if (i64::from(self.x) - i64::from(w.last_saved_position.x)).abs() >= 0x7ffe || (i64::from(self.y) - i64::from(w.last_saved_position.y)).abs() >= 0x7ffe {
      0x7fffi16.write(w)?;
      self.x.write(w)?;
      self.y.write(w)?;
    } else {
      ((self.x - w.last_saved_position.x) as i16).write(w)?;
      ((self.y - w.last_saved_position.y) as i16).write(w)?;
    }
    w.last_saved_position = self.clone();
    Ok(())
  }
}

#[derive(Clone, Copy, Debug, PartialEq, ReadWriteStruct)]
pub struct BoundingBox {
  pub left_top: MapPosition,
  pub right_bottom: MapPosition,
  pub orientation: f32, // always in [0,1)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct RidingState {
  pub direction: RidingDirection,
  pub acceleration_state: RidingAccelerationState,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct WaitCondition {
  typ: WaitConditionType,
  compare_type: WaitConditionComparisonType,
  ticks: u32,
  circuit_condition: CircuitCondition,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct CircuitCondition {
  pub comparator: Comparison,
  pub first_signal: SignalId,
  pub second_signal: SignalId,
  pub second_constant: i32,
  pub second_signal_is_constant: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SignalId {
  Item { item: Item },
  Fluid { fluid: Fluid, },
  VirtualSignal { virtual_signal: VirtualSignal, },
}
impl ReadWrite for SignalId {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    match SignalIdType::read(r)? {
      SignalIdType::Item => Ok(SignalId::Item { item: Item::read(r)?, }),
      SignalIdType::Fluid => Ok(SignalId::Fluid { fluid: Fluid::read(r)?, }),
      SignalIdType::VirtualSignal => Ok(SignalId::VirtualSignal { virtual_signal: VirtualSignal::read(r)?, }),
    }
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    match self {
      SignalId::Item { item, } => {
        SignalIdType::Item.write(w)?;
        item.write(w)
      },
      SignalId::Fluid { fluid, } => {
        SignalIdType::Fluid.write(w)?;
        fluid.write(w)
      },
      SignalId::VirtualSignal { virtual_signal, } => {
        SignalIdType::VirtualSignal.write(w)?;
        virtual_signal.write(w)
      },
    }
  }
}
