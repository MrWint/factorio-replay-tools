use enum_primitive_derive::Primitive;
use factorio_serialize_derive::ReplayReadWriteEnumU8;
use num_traits::{FromPrimitive, ToPrimitive};


// Version: 1.1.107
// Extraction method: util::export_prototypes
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum ItemGroup {
  Logistics = 1,
  Production = 2,
  IntermediateProducts = 3,
  Combat = 4,
  Fluids = 5,
  Signals = 6,
  Enemies = 7,
  Environment = 8,
  Effects = 9,
  Other = 10,
}
impl ItemGroup {
  pub fn name(self) -> &'static str {
    match self {
      ItemGroup::Logistics => "logistics",
      ItemGroup::Production => "production",
      ItemGroup::IntermediateProducts => "intermediate-products",
      ItemGroup::Combat => "combat",
      ItemGroup::Fluids => "fluids",
      ItemGroup::Signals => "signals",
      ItemGroup::Enemies => "enemies",
      ItemGroup::Environment => "environment",
      ItemGroup::Effects => "effects",
      ItemGroup::Other => "other",
    }
  }
  pub fn from_name(name: &str) -> ItemGroup {
    match name {
      "logistics" => ItemGroup::Logistics,
      "production" => ItemGroup::Production,
      "intermediate-products" => ItemGroup::IntermediateProducts,
      "combat" => ItemGroup::Combat,
      "fluids" => ItemGroup::Fluids,
      "signals" => ItemGroup::Signals,
      "enemies" => ItemGroup::Enemies,
      "environment" => ItemGroup::Environment,
      "effects" => ItemGroup::Effects,
      "other" => ItemGroup::Other,
      name => panic!("unknown ItemGroup \"{name}\""),
    }
  }
}
