use enum_primitive_derive::Primitive;
use factorio_serialize_derive::ReplayReadWriteEnumU16;
use num_traits::{FromPrimitive, ToPrimitive};


// Version: 1.1.107
// Extraction method: util::export_prototypes
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU16)]
pub enum Equipment {
  BatteryEquipment = 1,
  BatteryMk2Equipment = 2,
  DischargeDefenseEquipment = 3,
  EnergyShieldEquipment = 4,
  EnergyShieldMk2Equipment = 5,
  ExoskeletonEquipment = 6,
  FusionReactorEquipment = 7,
  NightVisionEquipment = 8,
  PersonalLaserDefenseEquipment = 9,
  PersonalRoboportEquipment = 10,
  PersonalRoboportMk2Equipment = 11,
  SolarPanelEquipment = 12,
  BeltImmunityEquipment = 13,
}
impl Equipment {
  pub fn name(self) -> &'static str {
    match self {
      Equipment::BatteryEquipment => "battery-equipment",
      Equipment::BatteryMk2Equipment => "battery-mk2-equipment",
      Equipment::DischargeDefenseEquipment => "discharge-defense-equipment",
      Equipment::EnergyShieldEquipment => "energy-shield-equipment",
      Equipment::EnergyShieldMk2Equipment => "energy-shield-mk2-equipment",
      Equipment::ExoskeletonEquipment => "exoskeleton-equipment",
      Equipment::FusionReactorEquipment => "fusion-reactor-equipment",
      Equipment::NightVisionEquipment => "night-vision-equipment",
      Equipment::PersonalLaserDefenseEquipment => "personal-laser-defense-equipment",
      Equipment::PersonalRoboportEquipment => "personal-roboport-equipment",
      Equipment::PersonalRoboportMk2Equipment => "personal-roboport-mk2-equipment",
      Equipment::SolarPanelEquipment => "solar-panel-equipment",
      Equipment::BeltImmunityEquipment => "belt-immunity-equipment",
    }
  }
  pub fn from_name(name: &str) -> Equipment {
    match name {
      "battery-equipment" => Equipment::BatteryEquipment,
      "battery-mk2-equipment" => Equipment::BatteryMk2Equipment,
      "discharge-defense-equipment" => Equipment::DischargeDefenseEquipment,
      "energy-shield-equipment" => Equipment::EnergyShieldEquipment,
      "energy-shield-mk2-equipment" => Equipment::EnergyShieldMk2Equipment,
      "exoskeleton-equipment" => Equipment::ExoskeletonEquipment,
      "fusion-reactor-equipment" => Equipment::FusionReactorEquipment,
      "night-vision-equipment" => Equipment::NightVisionEquipment,
      "personal-laser-defense-equipment" => Equipment::PersonalLaserDefenseEquipment,
      "personal-roboport-equipment" => Equipment::PersonalRoboportEquipment,
      "personal-roboport-mk2-equipment" => Equipment::PersonalRoboportMk2Equipment,
      "solar-panel-equipment" => Equipment::SolarPanelEquipment,
      "belt-immunity-equipment" => Equipment::BeltImmunityEquipment,
      name => panic!("unknown Equipment \"{name}\""),
    }
  }
}