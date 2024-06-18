use factorio_serialize::FixedPoint32_8;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
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
  pub mining_time: f64,
  pub result: Option<String>,
  pub count: Option<u32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct FluidBox {
  base_area: f64,
  height: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Product {
  #[serde(rename = "item")] Item {
    name: String,
    amount: u32,
  },
  #[serde(rename = "fluid")] Fluid {
    name: String,
    amount: f64,
  },
}
impl<'de> Deserialize<'de> for Product {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Product, D::Error> {
    let value = Value::deserialize(deserializer)?;
    if let Some(array) = value.as_array() {
      Ok(Product::Item {
        name: serde_json::from_value(array[0].clone()).unwrap(),
        amount: serde_json::from_value(array[1].clone()).unwrap(),
      })
    } else if let Some(object) = value.as_object() {
      let typ: String = object.get("type").map(|v| serde_json::from_value(v.clone()).unwrap()).unwrap_or_default();
      match typ.as_str() {
        "fluid" => {
          Ok(Product::Fluid {
            name: serde_json::from_value(object["name"].clone()).unwrap(),
            amount: serde_json::from_value(object["amount"].clone()).unwrap(),
          })
        },
        _ => {
          Ok(Product::Item {
            name: serde_json::from_value(object["name"].clone()).unwrap(),
            amount: serde_json::from_value(object["amount"].clone()).unwrap(),
          })
        }
      }
    } else {
      panic!("unknown value {}", value)
    }
  }
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Ingredient {
  #[serde(rename = "item")] Item {
    name: String,
    amount: u32,
  },
  #[serde(rename = "fluid")] Fluid {
    name: String,
    amount: f64,
  },
}
impl<'de> Deserialize<'de> for Ingredient {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Ingredient, D::Error> {
    let value = Value::deserialize(deserializer)?;
    if let Some(array) = value.as_array() {
      Ok(Ingredient::Item {
        name: serde_json::from_value(array[0].clone()).unwrap(),
        amount: serde_json::from_value(array[1].clone()).unwrap(),
      })
    } else if let Some(object) = value.as_object() {
      let typ: String = serde_json::from_value(object["type"].clone()).unwrap();
      match typ.as_str() {
        "fluid" => {
          Ok(Ingredient::Fluid {
            name: serde_json::from_value(object["name"].clone()).unwrap(),
            amount: serde_json::from_value(object["amount"].clone()).unwrap(),
          })
        },
        _ => {
          Ok(Ingredient::Item {
            name: serde_json::from_value(object["name"].clone()).unwrap(),
            amount: serde_json::from_value(object["amount"].clone()).unwrap(),
          })
        }
      }
    } else {
      panic!("unknown value {}", value)
    }
  }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
  pub stack_size: u32,
  pub fuel_category: Option<String>,
  pub fuel_value: Option<Energy>,
  pub fuel_acceleration_multiplier: Option<f64>,
  pub fuel_top_speed_multiplier: Option<f64>,
  pub place_result: Option<String>,
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
  pub build_distance: u32,  // 10
  pub drop_item_distance: u32,  // 10
  pub reach_distance: u32,  // 10,
  pub item_pickup_distance: f64,  // 1,
  pub loot_pickup_distance: f64,  // 2,
  pub enter_vehicle_distance: f64,  // 3,
  pub reach_resource_distance: f64,  // 2.7,
  pub running_speed: f64,  // 0.15,
  pub maximum_corner_sliding_distance: f64,  // 0.7,
  pub mining_speed: f64,  // 0.5,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Energy(String);
impl Energy {
  // from EnergyHelpers::fromString
  pub fn parse(&self) -> f64 {
    let value = self.0.as_bytes();
    let len = value.len();
    let unit = b"JW".into_iter().position(|&c| c == value[len - 1].to_ascii_uppercase()).expect("unknown unit") as f64 * 59.0 + 1.0;
    (if value[len - 2].is_ascii_digit() {
      std::str::from_utf8(&value[..len-1]).unwrap().parse::<f64>().unwrap()
    } else {
      std::str::from_utf8(&value[..len-2]).unwrap().parse::<f64>().unwrap() * 1000f64.powi(b"KMGTPEZY".into_iter().position(|&c| c == value[len - 2].to_ascii_uppercase()).expect("unknown modifier") as i32 + 1)
    }) / unit
  }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum EnergySource {
  #[serde(rename = "electric")] Electric {
    usage_priority: String,
    drain: Option<String>,
  },
  #[serde(rename = "burner")] Burner {
    fuel_category: String,
    effectivity: f64,
    fuel_inventory_size: u32,
  },
  #[serde(rename = "heat")] Heat {
    max_temperature: f64,
    specific_heat: Energy,
    max_transfer: String,
  },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Furnace {
  pub minable: Minable,
  pub collision_box: BoundingBox,
  pub selection_box: BoundingBox,
  pub result_inventory_size: u32,
  pub energy_usage: Energy,
  pub crafting_speed: f64,
  pub source_inventory_size: u32,
  pub energy_source: EnergySource,
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
  energy_consumption: Energy,
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
  energy_per_movement: Energy,
  energy_per_rotation: Energy,
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
  energy_usage: Energy,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Lab {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  energy_source: EnergySource,
  energy_usage: Energy,
  researching_speed: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct RocketSilo {
  minable: Minable,
  collision_box: BoundingBox,
  selection_box: BoundingBox,
  energy_source: EnergySource,
  energy_usage: Energy,
  active_energy_usage: Energy,
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
  energy_usage: Energy,
  pumping_speed: f64,
  circuit_wire_max_distance: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct MiningDrill {
  pub minable: Minable,
  pub collision_box: BoundingBox,
  pub selection_box: BoundingBox,
  pub energy_source: EnergySource,
  pub energy_usage: Energy,
  pub mining_speed: f64,
  pub resource_searching_radius: f64,
  pub circuit_wire_max_distance: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
  pub minable: Minable,
  pub collision_box: BoundingBox,
  pub selection_box: BoundingBox,
  pub infinite: Option<bool>,
  pub minimum: Option<u32>,
  pub normal: Option<u32>,
  pub infinite_depletion_amount: Option<u32>,
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
pub struct SimpleEntity {
  pub minable: Option<Minable>,
  pub collision_box: BoundingBox,
  pub selection_box: BoundingBox,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeData {
  pub energy_required: Option<f64>,
  pub ingredients: Vec<Ingredient>,
  pub result: Option<String>,
  pub result_count: Option<u32>,
  pub results: Option<Vec<Product>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
  pub category: Option<String>,
  pub enabled: Option<bool>,
  pub energy_required: Option<f64>,
  pub ingredients: Option<Vec<Ingredient>>,
  pub normal: Option<RecipeData>,
  pub result: Option<String>,
  pub result_count: Option<u32>,
  pub results: Option<Vec<Product>>,
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
  #[serde(rename = "simple-entity")] pub simple_entity: HashMap<String, SimpleEntity>,
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
