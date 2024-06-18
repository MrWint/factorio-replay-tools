use std::collections::HashMap;

use factorio_serialize::{constants::{Fluid, Item, Recipe}, BoundingBox, FixedPoint32_8};

use crate::prototypes::{self, Prototypes};

#[derive(Debug)]
pub enum ProductConfig {
  Item {
    id: Item,
    amount: u32,
  },
  Fluid {
    id: Fluid,
    amount: f64,
  },
}
impl ProductConfig {
  fn from_ingredient(ingredient: &prototypes::Ingredient) -> Self {
    match ingredient {
      prototypes::Ingredient::Item { name, amount } => ProductConfig::Item { id: Item::from_name(name), amount: *amount },
      prototypes::Ingredient::Fluid { name, amount } => ProductConfig::Fluid { id: Fluid::from_name(name), amount: *amount },
    }
  }
  fn from_product(product: &prototypes::Product) -> Self {
    match product {
      prototypes::Product::Item { name, amount } => ProductConfig::Item { id: Item::from_name(name), amount: *amount },
      prototypes::Product::Fluid { name, amount } => ProductConfig::Fluid { id: Fluid::from_name(name), amount: *amount },
    }
  }
}

#[derive(Debug)]
pub struct RecipeConfig {
  pub ingredients: Vec<ProductConfig>,
  pub results: Vec<ProductConfig>,
  pub energy_required: f64,
}
impl RecipeConfig {
  fn from_prototype(recipe: &prototypes::Recipe) -> Self {
    if let Some(normal) = &recipe.normal {
      let energy_required = normal.energy_required.unwrap_or(0.5);
      let ingredients = normal.ingredients.iter().map(|i| ProductConfig::from_ingredient(i)).collect();
      let results = if let Some(results) = &normal.results {
        results.iter().map(|i| ProductConfig::from_product(i)).collect()
      } else {
        vec![ProductConfig::Item { id: Item::from_name(normal.result.as_ref().unwrap()), amount: normal.result_count.unwrap_or(1) }]
      };
      RecipeConfig { ingredients, results, energy_required }
    } else {
      let energy_required = recipe.energy_required.unwrap_or(0.5);
      let ingredients = recipe.ingredients.as_ref().unwrap().iter().map(|i| ProductConfig::from_ingredient(i)).collect();
      let results = if let Some(results) = &recipe.results {
        results.iter().map(|i| ProductConfig::from_product(i)).collect()
      } else {
        vec![ProductConfig::Item { id: Item::from_name(recipe.result.as_ref().unwrap()), amount: recipe.result_count.unwrap_or(1) }]
      };
      RecipeConfig { ingredients, results, energy_required }
    }
  }
}

#[derive(Debug)]
pub struct ItemConfig {
  pub stack_size: u32,
}
impl ItemConfig {
  fn from_prototype(item: &prototypes::Item) -> Self {
    ItemConfig { stack_size: item.stack_size }
  }
}

#[derive(Debug)]
pub struct FuelConfig {
  pub fuel_value: f64,
}
impl FuelConfig {
  fn from_prototype(item: &prototypes::Item) -> Option<Self> {
    Some(FuelConfig { fuel_value: item.fuel_value.as_ref()?.parse() })
  }
}

#[derive(Debug)]
pub struct GameConfig {
  pub player_movement_speed: f64,
  pub player_bounding_box: BoundingBox,
  pub maximum_corner_sliding_distance: FixedPoint32_8,
  pub player_reach_distance: u32,
  pub player_reach_resource_distance: f64,
  pub player_mining_speed: f64,
  pub player_inventory_size: u32,

  pub dry_tree_bounding_box: BoundingBox,
  pub dry_tree_mining_time: f64,
  pub huge_rock_bounding_box: BoundingBox,
  pub huge_rock_mining_time: f64,
  pub iron_ore_bounding_box: BoundingBox,
  pub iron_ore_mining_time: f64,
  pub copper_ore_bounding_box: BoundingBox,
  pub copper_ore_mining_time: f64,

  pub burner_miner_energy_usage: f64,
  pub burner_miner_speed: f64,
  pub stone_furnace_energy_usage: f64,
  pub stone_furnace_speed: f64,

  pub fuels: HashMap<Item, FuelConfig>,
  pub items: HashMap<Item, ItemConfig>,
  pub recipes: HashMap<Recipe, RecipeConfig>,
}
impl GameConfig {
  fn from_prototypes(prototypes: Prototypes) -> Self {
    GameConfig {
      player_movement_speed: prototypes.character["character"].running_speed,
      player_bounding_box: prototypes.character["character"].collision_box.to_struct(),
      maximum_corner_sliding_distance: FixedPoint32_8::from_double(prototypes.character["character"].maximum_corner_sliding_distance),
      player_reach_distance: prototypes.character["character"].reach_distance,
      player_reach_resource_distance: prototypes.character["character"].reach_resource_distance,
      player_mining_speed: prototypes.character["character"].mining_speed,
      player_inventory_size: prototypes.character["character"].inventory_size,

      dry_tree_bounding_box: prototypes.tree["dry-tree"].collision_box.to_struct(),
      dry_tree_mining_time: prototypes.tree["dry-tree"].minable.mining_time,
      huge_rock_bounding_box: prototypes.simple_entity["rock-huge"].collision_box.to_struct(),
      huge_rock_mining_time: prototypes.simple_entity["rock-huge"].minable.as_ref().expect("huge rock not minable").mining_time,
      iron_ore_bounding_box: prototypes.resource["iron-ore"].collision_box.to_struct(),
      iron_ore_mining_time: prototypes.resource["iron-ore"].minable.mining_time,
      copper_ore_bounding_box: prototypes.resource["copper-ore"].collision_box.to_struct(),
      copper_ore_mining_time: prototypes.resource["copper-ore"].minable.mining_time,

      burner_miner_energy_usage: prototypes.mining_drill["burner-mining-drill"].energy_usage.parse(),
      burner_miner_speed: prototypes.mining_drill["burner-mining-drill"].mining_speed,
      stone_furnace_energy_usage: prototypes.furnace["stone-furnace"].energy_usage.parse(),
      stone_furnace_speed: prototypes.furnace["stone-furnace"].crafting_speed,

      fuels: prototypes.item.iter().filter_map(|(name, item)| Some((Item::from_name(name), FuelConfig::from_prototype(item)?))).collect(),
      items: prototypes.item.iter().map(|(name, item)| (Item::from_name(name), ItemConfig::from_prototype(item))).collect(),
      recipes: prototypes.recipe.iter().map(|(name, recipe)| (Recipe::from_name(name), RecipeConfig::from_prototype(recipe))).collect(),
    }
  }
}
lazy_static::lazy_static! {
  pub static ref GAME_CONFIG: GameConfig = {
    let c = GameConfig::from_prototypes(crate::prototypes::parse_prototype_data());
    // println!("{c:#?}");
    c
  };
}
