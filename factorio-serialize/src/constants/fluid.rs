use enum_primitive_derive::Primitive;
use factorio_serialize_derive::ReplayReadWriteEnumU16;
use num_traits::{FromPrimitive, ToPrimitive};


// Version: 1.1.107
// Extraction method: util::export_prototypes
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU16)]
pub enum Fluid {
  FluidUnknown = 1,
  Water = 2,
  CrudeOil = 3,
  Steam = 4,
  HeavyOil = 5,
  LightOil = 6,
  PetroleumGas = 7,
  SulfuricAcid = 8,
  Lubricant = 9,
}
impl Fluid {
  pub fn name(self) -> &'static str {
    match self {
      Fluid::FluidUnknown => "fluid-unknown",
      Fluid::Water => "water",
      Fluid::CrudeOil => "crude-oil",
      Fluid::Steam => "steam",
      Fluid::HeavyOil => "heavy-oil",
      Fluid::LightOil => "light-oil",
      Fluid::PetroleumGas => "petroleum-gas",
      Fluid::SulfuricAcid => "sulfuric-acid",
      Fluid::Lubricant => "lubricant",
    }
  }
  pub fn from_name(name: &str) -> Fluid {
    match name {
      "fluid-unknown" => Fluid::FluidUnknown,
      "water" => Fluid::Water,
      "crude-oil" => Fluid::CrudeOil,
      "steam" => Fluid::Steam,
      "heavy-oil" => Fluid::HeavyOil,
      "light-oil" => Fluid::LightOil,
      "petroleum-gas" => Fluid::PetroleumGas,
      "sulfuric-acid" => Fluid::SulfuricAcid,
      "lubricant" => Fluid::Lubricant,
      name => panic!("unknown Fluid \"{name}\""),
    }
  }
}
