use factorio_serialize::FixedPoint32_8;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;



#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
  x: f64,
  y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoundingBox {
  top_left: Point,
  bottom_right: Point,
}
impl BoundingBox {
  pub fn to_struct(&self) -> factorio_serialize::BoundingBox {
    factorio_serialize::BoundingBox {
      left_top: factorio_serialize::MapPosition::new(FixedPoint32_8::from_double(self.top_left.x), FixedPoint32_8::from_double(self.top_left.y)),
      right_bottom: factorio_serialize::MapPosition::new(FixedPoint32_8::from_double(self.bottom_right.x), FixedPoint32_8::from_double(self.bottom_right.y)),
      orientation: factorio_serialize::VectorOrientation::north()
    }
  }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Minable {
  mining_time: f64,
  result: Option<String>,
  count: Option<u32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct FluidBox {
  base_area: f64,
  height: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
  name: String,
  amount: u32,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
  stack_size: u32,
  fuel_category: Option<String>,
  fuel_value: Option<String>,
  fuel_acceleration_multiplier: Option<f64>,
  fuel_top_speed_multiplier: Option<f64>,
  place_result: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Fluid {
  default_temperature: i32,
  max_temperature: Option<i32>,
  heat_capacity: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
  minable: Option<Minable>,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  inventory_size: u32,
  circuit_wire_max_distance: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
  pub collision_box: BoundingBox,
  pub selection_box: BoundingBox,
  pub inventory_size: u32,  // 80
  pub build_distance: f64,  // 10
  pub drop_item_distance: f64,  // 10
  pub reach_distance: f64,  // 10,
  pub item_pickup_distance: f64,  // 1,
  pub loot_pickup_distance: f64,  // 2,
  pub enter_vehicle_distance: f64,  // 3,
  pub reach_resource_distance: f64,  // 2.7,
  pub running_speed: f64,  // 0.15,
  pub maximum_corner_sliding_distance: f64,  // 0.7,
  pub mining_speed: f64,  // 0.5,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct EnergySource {
  r#type: String,
  fuel_category: Option<String>,
  usage_priority: Option<String>,
  effectivity: Option<f64>,
  fuel_inventory_size: Option<u32>,
  drain: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Furnace {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  result_inventory_size: u32,
  energy_usage: String,
  crafting_speed: f64,
  source_inventory_size: u32,
  energy_source: EnergySource,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TransportBelt {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  speed: f64,
  circuit_wire_max_distance: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Boiler {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  target_temperature: f64,
  energy_consumption: String,
  energy_source: EnergySource,
  burning_cooldown: u32,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct ElectricPole {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  maximum_wire_distance: f64,
  supply_area_distance: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Generator {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  effectivity: f64,
  fluid_usage_per_tick: f64,
  maximum_temperature: i32,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct OffshorePump {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  pumping_speed: f64,
  circuit_wire_max_distance: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Inserter {
  stack: Option<bool>,
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  energy_per_movement: String,
  energy_per_rotation: String,
  energy_source: EnergySource,
  extension_speed: f64,
  rotation_speed: f64,
  pickup_position: Point,
  insert_position: Point,
  circuit_wire_max_distance: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Pipe {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct PipeToGround {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct AssemblingMachine {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  crafting_categories: Vec<String>,
  crafting_speed: f64,
  energy_source: EnergySource,
  energy_usage: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Lab {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  energy_source: EnergySource,
  energy_usage: String,
  researching_speed: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct RocketSilo {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  energy_source: EnergySource,
  energy_usage: String,
  active_energy_usage: String,
  rocket_parts_required: u32,
  crafting_speed: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct StorageTank {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  fluid_box: FluidBox,
  circuit_wire_max_distance: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Pump {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  fluid_box: FluidBox,
  energy_source: EnergySource,
  energy_usage: String,
  pumping_speed: f64,
  circuit_wire_max_distance: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct MiningDrill {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  energy_source: EnergySource,
  energy_usage: String,
  mining_speed: f64,
  resource_searching_radius: f64,
  circuit_wire_max_distance: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  infinite: Option<bool>,
  minimum: Option<u32>,
  normal: Option<u32>,
  infinite_depletion_amount: Option<u32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Tree {
  pub minable: Minable,
  pub collision_box: BoundingBox,
  pub selection_box: BoundingBox,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
  enabled: Option<bool>,
  energy_required: Option<f64>,
  ingredients: Option<Vec<Ingredient>>,
  result: Option<String>,
  result_count: Option<u32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TechnologyUnit {
  prerequisites: Option<Vec<String>>,
  count: Option<u64>,
  count_formula: Option<String>,
  time: f64,
  ingredients: Vec<Ingredient>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Technology {
  prerequisites: Option<Vec<String>>,
  unit: TechnologyUnit,
  upgrade: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Prototypes {
  pub item: HashMap<String, Item>,
  pub fluid: HashMap<String, Fluid>,
  pub container: HashMap<String, Container>,
  pub character: HashMap<String, Character>,
  pub furnace: HashMap<String, Furnace>,
  #[serde(rename = "transport-belt")] pub transport_belt: HashMap<String, TransportBelt>,
  pub boiler: HashMap<String, Boiler>,
  #[serde(rename = "electric-pole")] pub electric_pole: HashMap<String, ElectricPole>,
  pub generator: HashMap<String, Generator>,
  #[serde(rename = "offshore-pump")] pub offshore_pump: HashMap<String, OffshorePump>,
  pub inserter: HashMap<String, Inserter>,
  pub pipe: HashMap<String, Pipe>,
  #[serde(rename = "pipe-to-ground")] pub pipe_to_ground: HashMap<String, PipeToGround>,
  #[serde(rename = "assembling-machine")] pub assembling_machine: HashMap<String, AssemblingMachine>,
  pub lab: HashMap<String, Lab>,
  #[serde(rename = "rocket-silo")] pub rocket_silo: HashMap<String, RocketSilo>,
  #[serde(rename = "storage-tank")] pub storage_tank: HashMap<String, StorageTank>,
  pub pump: HashMap<String, Pump>,
  #[serde(rename = "mining-drill")] pub mining_drill: HashMap<String, MiningDrill>,
  pub resource: HashMap<String, Resource>,
  pub tree: HashMap<String, Tree>,
  pub recipe: HashMap<String, Recipe>,
  pub technology: HashMap<String, Technology>,
}



pub fn parse_prototype_data() -> Prototypes {
  if std::fs::metadata("data/data.min.json").is_ok_and(|md| md.is_file()) {
    parse_prototype_data_from("data/data.min.json")
  } else {
    create_minimized_prototypes()
  }
}

pub fn create_minimized_prototypes() -> Prototypes {
  let prototypes = parse_prototype_data_from("data/data.json");

  let out_buf = serde_json::to_vec_pretty(&prototypes).expect("couldn't write JSON data");
  std::fs::write("data/data.min.json", out_buf).expect("couldn't create data.min.json file");

  prototypes
}

pub fn parse_prototype_data_from(file_name: &str) -> Prototypes {
  let buf = std::fs::read(file_name).expect("couldn't read prototype file");
  serde_json::from_slice(&buf).expect("couldn't parse JSON data in prototype file")
}
