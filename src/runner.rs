use factorio_serialize::{constants::{Entity, Item, Recipe, Tile}, map::{Chunk, EntityCommon, EntityData, EntityWithHealth, MapData, ResourceEntity, SimpleEntity, Tree}, replay::{Direction, ForceId, InputAction, InputActionData, PlayerJoinGameData, ReplayData}, save::SaveFile, FixedPoint32_8, MapPosition, RandomGenerator, Result, TilePosition};

use crate::simulation::GameState;


const PID: u16 = 0;
const PLAYER_NAME: &str = "MrWint";

const DRY_TREE_FIXED_POSITION: MapPosition = MapPosition::new(FixedPoint32_8(-0x80), FixedPoint32_8(-0x100));
const MANUAL_IRON_ORE_FIXED_POSITION: TilePosition = TilePosition::new(0, -3);
const MANUAL_COPPER_ORE_FIXED_POSITION: TilePosition = TilePosition::new(0, -2);
const HUGE_ROCK_FIXED_POSITION: MapPosition = MapPosition::new(FixedPoint32_8(0x280), FixedPoint32_8(-0x1c0));

const HUGE_ROCK_RNG: RandomGenerator = RandomGenerator::new(80686, 3738370905, 872480768);  // entities RNG determines Huge Rock contents

pub struct Runner {
  pub entities: Vec<(Entity, MapPosition)>,
  pub input_actions: Vec<InputAction>,

  game_state: GameState,
}
impl Runner {
  pub fn new() -> Self {
    Self {
      entities: Vec::new(),
      input_actions: Vec::new(),

      game_state: GameState::new(HUGE_ROCK_RNG).with_instrumentation(),
    }
  }

  fn tick(&mut self) {
    self.game_state.tick();
  }
  fn n_tick(&mut self, n: u32) {
    for _ in 0..n { self.game_state.tick(); }
  }

  fn set_walking_direction(&mut self, direction: Direction) {
    self.game_state.set_walking_direction(direction);
    self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::StartWalking(direction))); // begin walking
  }
  fn stop_walking(&mut self) {
    self.game_state.set_walking_direction(Direction::None);
    self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::StopWalking)); // stop walking
  }
  pub fn walk_for(&mut self, direction: Direction, ticks: u32) {
    self.set_walking_direction(direction);
    for _ in 0..ticks {
      self.tick();
    }
    self.stop_walking();
  }
  pub fn make_water_tile(&mut self, position: TilePosition) {
    self.game_state.make_water_tile(position);
  }
  pub fn add_tree(&mut self, position: MapPosition) {
    self.entities.push((Entity::DryTree, position));
    self.game_state.add_tree(position);
  }  
  #[allow(dead_code)]
  pub fn mine_tree(&mut self) {
    self.entities.push((Entity::DryTree, DRY_TREE_FIXED_POSITION));
    self.game_state.add_tree(DRY_TREE_FIXED_POSITION);
    let ticks = self.game_state.mine_tree(DRY_TREE_FIXED_POSITION);
    self.n_tick(ticks);
  }
  #[allow(dead_code)]
  pub fn mine_rock(&mut self) {
    self.entities.push((Entity::RockHuge, HUGE_ROCK_FIXED_POSITION));
    self.game_state.add_rock(HUGE_ROCK_FIXED_POSITION);
    let ticks = self.game_state.mine_rock(HUGE_ROCK_FIXED_POSITION);
    self.n_tick(ticks);
  }
  #[allow(dead_code)]
  pub fn mine_iron_ore(&mut self, count: u32) {
    let ticks = self.game_state.mine_iron_ore(MANUAL_IRON_ORE_FIXED_POSITION, count);
    self.n_tick(ticks);
  }
  #[allow(dead_code)]
  pub fn mine_copper_ore(&mut self, count: u32) {
    let ticks = self.game_state.mine_copper_ore(MANUAL_COPPER_ORE_FIXED_POSITION, count);
    self.n_tick(ticks);
  }
  #[allow(dead_code)]
  pub fn craft(&mut self, recipe: Recipe, count: u32) {
    let _ticks = self.game_state.craft(recipe, count);
    // self.n_tick(ticks);
  }
  #[allow(dead_code)]
  pub fn build_stone_furnace(&mut self, position: TilePosition) {
    self.game_state.build_stone_furnace(position);
  }
  #[allow(dead_code)]
  pub fn add_fuel_to_stone_furnace(&mut self, item: Item, amount: u32, position: TilePosition) {
    self.game_state.add_fuel_to_stone_furnace(item, amount, position);
  }
  #[allow(dead_code)]
  pub fn add_input_to_stone_furnace(&mut self, item: Item, amount: u32, position: TilePosition) {
    self.game_state.add_input_to_stone_furnace(item, amount, position);
  }
  #[allow(dead_code)]
  pub fn build_iron_miner(&mut self, position: TilePosition, direction: Direction) {
    self.game_state.build_iron_miner(position, direction);
  }
  #[allow(dead_code)]
  pub fn add_fuel_to_iron_miner(&mut self, item: Item, amount: u32, position: TilePosition) {
    self.game_state.add_fuel_to_iron_miner(item, amount, position);
  }


  pub fn write_save_file(self, template_name: &str, out_name: &str) -> Result<()> {
    // load template map and script
    let template_save_file = SaveFile::load_save_file(template_name).unwrap();
    let mut map_data = MapData::parse_map_data(&template_save_file.level_init_dat)?;
    let script_init_dat = template_save_file.script_init_dat;

    // initialize input actions preamble to spawn player
    let mut input_actions = Vec::new();
    input_actions.push(InputAction::new(0, 255, InputActionData::SingleplayerInit));
    input_actions.push(InputAction::new(0, 255, InputActionData::GameCreatedFromScenario));
    input_actions.push(InputAction::new(0, 255, InputActionData::DisconnectAllPlayers));
    input_actions.push(InputAction::new(0, 255, InputActionData::PlayerJoinGame(PlayerJoinGameData { peer_id: 0, player_index: PID, force_id: ForceId::Player, username: PLAYER_NAME.to_owned(), as_editor: false, admin: true })));
    // copy over replay actions
    input_actions.extend(self.game_state.input_actions);
    // add dummy end action to extend runtime of the replay
    input_actions.push(InputAction::new(self.game_state.tick + 1000, crate::simulation::PID, InputActionData::StopWalking));

    // add entities to map template
    for (entity, map_position) in self.entities {
      add_entity_to_map(&mut map_data, entity, map_position);
    }
    for water_position in self.game_state.water_tiles {
      set_water_tile(&mut map_data, &water_position);
    }
    for (iron_ore_position, count) in self.game_state.iron_ores {
      add_resource_to_map(&mut map_data, Entity::IronOre, iron_ore_position, count);
    }
    for (copper_ore_position, count) in self.game_state.copper_ores {
      add_resource_to_map(&mut map_data, Entity::CopperOre, copper_ore_position, count);
    }
    for (iron_ore_position, miner) in self.game_state.iron_miners {
      add_resource_to_map(&mut map_data, Entity::IronOre, iron_ore_position, miner.num_resources_needed_on_map());
    }

    map_data.map.entities_random_generator.seed1 = HUGE_ROCK_RNG.seed1;  // entities RNG determines Huge Rock contents
    map_data.map.entities_random_generator.seed2 = HUGE_ROCK_RNG.seed2;  // entities RNG determines Huge Rock contents
    map_data.map.entities_random_generator.seed3 = HUGE_ROCK_RNG.seed3;  // entities RNG determines Huge Rock contents

    let level_init_dat = map_data.write_map_data()?;
    let replay_dat = ReplayData::from_input_actions(input_actions).write_replay_data()?;

    SaveFile::from_raw_dat(level_init_dat, replay_dat, script_init_dat).write_save_file_instrumented(out_name).unwrap();
    Ok(())
  }
}



// pub fn gcd(mut m: i32, mut n: i32) -> i32 {
//   while m != 0 {
//     if m < n {
//       std::mem::swap(&mut m, &mut n);
//     }
//     m %= n;
//   }
//   n
// }



fn get_or_create_chunk(map_data: &mut MapData, position: MapPosition) -> &mut Chunk {
  if !map_data.map.surfaces[0].chunks.iter().any(|c| c.position == position.to_chunk_position()) {
    let mut new_chunk = Chunk {
      active_entities_serialisation_helper: 0,
      active_when_enemy_is_around: 0,
      entities_to_be_inserted_before_setup: Vec::new(),
      generated_status: 50,
      military_targets_len: 0,
      planned_update_counts_to_be_loaded: Vec::new(),
      pollution: 0.0,
      position: position.to_chunk_position(),
      tick_of_last_change_that_could_affect_charting: 0,
      tick_of_optional_activation: 0,
      tiles: [[(Tile::LabWhite, 0x10); 32]; 32],
    };
    for x in 0..32 {
      for y in 0..32 {
        new_chunk.tiles[x][y].0 = [Tile::LabDark1, Tile::LabDark2][(x + y) & 1];
      }
    }
    map_data.map.surfaces[0].chunks.push(new_chunk);
  }
  map_data.map.surfaces[0].chunks.iter_mut().find(|c| c.position == position.to_chunk_position()).unwrap()
}
fn add_entity_to_map(map_data: &mut MapData, entity: Entity, position: MapPosition) {
  let chunk = get_or_create_chunk(map_data, position);
  let entity_data = match entity {
    Entity::DryTree => EntityData::DryTree(Tree { entity: EntityWithHealth { entity: EntityCommon { position, usage_bit_mask: 0, targeter: None }, health: 0.0, damage_to_be_taken: 0.0, upgrade_target: None }, tree_data: 0, burn_progress: 0 }),
    Entity::RockHuge => EntityData::RockHuge(SimpleEntity { entity: EntityWithHealth { entity: EntityCommon { position, usage_bit_mask: 0, targeter: None }, health: 0.0, damage_to_be_taken: 0.0, upgrade_target: None }, variation: 0 }),
    _ => panic!("unsupported Entity {:?}", entity),
  };
  chunk.entities_to_be_inserted_before_setup.push((entity, entity_data));
}
fn add_resource_to_map(map_data: &mut MapData, entity: Entity, tile_position: TilePosition, resource_amount: u32) {
  let position = tile_position.center_map_position();
  let chunk = get_or_create_chunk(map_data, position);
  let entity_data = match entity {
    Entity::CopperOre => EntityData::CopperOre(ResourceEntity { entity: EntityCommon { position, usage_bit_mask: 0, targeter: None }, resource_amount, initial_amount: None, variation: 0 }),
    Entity::IronOre => EntityData::IronOre(ResourceEntity { entity: EntityCommon { position, usage_bit_mask: 0, targeter: None }, resource_amount, initial_amount: None, variation: 0 }),
    _ => panic!("unsupported resource {:?}", entity),
  };
  chunk.entities_to_be_inserted_before_setup.push((entity, entity_data));
}
fn set_water_tile(map_data: &mut MapData, position: &TilePosition) {
  let chunk = get_or_create_chunk(map_data, position.top_left_map_position());
  chunk.tiles[(position.x & 0x1f) as usize][(position.y & 0x1f) as usize] = (Tile::Water, 0x30);
}
