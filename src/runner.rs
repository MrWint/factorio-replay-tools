use factorio_serialize::{constants::{Entity, Tile}, map::{Chunk, EntityCommon, EntityData, EntityWithHealth, MapData, ResourceEntity, SimpleEntity, Tree}, replay::{Direction, ForceId, InputAction, InputActionData, PlayerJoinGameData, ReplayData}, save::SaveFile, MapPosition, Result, TilePosition};

use crate::simulation::GameState;


const PID: u16 = 0;
const PLAYER_NAME: &str = "MrWint";

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

      game_state: GameState::new().with_instrumentation(),
    }
  }

  // pub fn build_miner_for(&mut self, resource: Item, position: TilePosition, direction: Direction) {
  //   let resource_entity = match resource {
  //     Item::IronOre => Entity::IronOre,
  //     _ => panic!("Unknown resource entity for item {:?}", resource),
  //   };
  //   self.entities.push((resource_entity, position.center_map_position()));
  //   self.build(Item::BurnerMiningDrill, position.top_left_map_position(), direction)
  // }

  // pub fn build(&mut self, item: Item, position: MapPosition, direction: Direction) {
  //   self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::SetFilter(SetFilterParameters { target: ItemStackTargetSpecification::from_quick_bar(0, 0), filter: item, }))); // Configure quickbar slot
  //   self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, }))); // Select quickbar into cursor
  //   self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::Build(BuildParameters { position, direction, created_by_moving: false, build_by_moving_start_position: None, flags: 0, }))); // Build item from cursor
  //   self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::ClearCursor)); // Clear cursor
  //   self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { target_quick_bar_slot: 0, item_to_use: ItemStackTargetSpecification::from_nothing(), currently_selected_quick_bar_slot: 65535 }))); // Clear quickbar slot
  // }

  // pub fn add_item(&mut self, item: Item, amount: usize, pos: MapPosition) {
  //   self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::SetFilter(SetFilterParameters { target: ItemStackTargetSpecification::from_quick_bar(0, 0), filter: item, }))); // Configure quickbar slot
  //   self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, }))); // Select quickbar into cursor
  //   self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::SelectedEntityChanged(pos))); // Select entity
  //   for _ in 0..amount {
  //     self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::DropItem(pos)));
  //   }
  //   self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::SelectedEntityCleared)); // Clear selection
  //   self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::ClearCursor)); // Clear cursor
  //   self.input_actions.push(InputAction::new(self.game_state.tick, PID, InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { target_quick_bar_slot: 0, item_to_use: ItemStackTargetSpecification::from_nothing(), currently_selected_quick_bar_slot: 65535 }))); // Clear quickbar slot
  // }

  fn tick(&mut self) {
    self.game_state.tick();
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
    self.game_state.add_tree(position);
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
    for tree_position in self.game_state.dry_trees {
      add_entity_to_map(&mut map_data, Entity::DryTree, tree_position);
    }

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
      tiles: [[(Tile::Nothing, 0x10); 32]; 32],
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
    Entity::IronOre => EntityData::IronOre(ResourceEntity { entity: EntityCommon { position, usage_bit_mask: 0, targeter: None }, resource_amount: 999999, initial_amount: None, variation: 0 }),
    Entity::DryTree => EntityData::DryTree(Tree { entity: EntityWithHealth { entity: EntityCommon { position, usage_bit_mask: 0, targeter: None }, health: 0.0, damage_to_be_taken: 0.0, upgrade_target: Entity::Nothing }, tree_data: 0, burn_progress: 0 }),
    Entity::RockHuge => EntityData::RockHuge(SimpleEntity { entity: EntityWithHealth { entity: EntityCommon { position, usage_bit_mask: 0, targeter: None }, health: 0.0, damage_to_be_taken: 0.0, upgrade_target: Entity::Nothing }, variation: 0 }),
    _ => panic!("unsupported Entity {:?}", entity),
  };
  chunk.entities_to_be_inserted_before_setup.push((entity, entity_data));
}
fn set_water_tile(map_data: &mut MapData, position: &TilePosition) {
  let chunk = get_or_create_chunk(map_data, position.top_left_map_position());
  chunk.tiles[(position.x & 0x1f) as usize][(position.y & 0x1f) as usize] = (Tile::Water, 0x30);
}
