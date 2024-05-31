use factorio_serialize_derive::{MapReadWriteStruct, ReplayReadWriteStruct};


#[derive(Clone, Copy, Debug, MapReadWriteStruct, ReplayReadWriteStruct)]
pub struct VectorOrientation {
  pub x: i16,
  pub y: i16,
}
impl VectorOrientation {
  pub fn north() -> Self {
    VectorOrientation { x: 0, y: -0x7fff }
  }
}