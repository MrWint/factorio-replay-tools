use factorio_serialize::{save::SaveFile, map::{MapData, Chunk, EntityData, ResourceEntity, EntityCommon}, Result, replay::{ReplayData, InputAction, InputActionData, PlayerJoinGameData, ForceId, Direction, SetFilterParameters, ItemStackTargetSpecification, QuickBarPickSlotParameters, BuildParameters, QuickBarSetSlotParameters}, MapPosition, TilePosition, constants::{Entity, Item, Tile}};


const PID: u16 = 0;

pub struct Runner {
  map_data: MapData,
  pub input_actions: Vec<InputAction>,
  script_init_dat: Vec<u8>,

  tick: u32,
}
impl Runner {
  pub fn add_entity_to_map(&mut self, entity: Entity, position: MapPosition) {
    if !self.map_data.map.surfaces[0].chunks.iter().any(|c| c.position == position.to_chunk_position()) {
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
      self.map_data.map.surfaces[0].chunks.push(new_chunk);
    }
    let chunk = self.map_data.map.surfaces[0].chunks.iter_mut().find(|c| c.position == position.to_chunk_position()).unwrap();
    let entity_data = match entity {
      Entity::IronOre => EntityData::IronOre(ResourceEntity { entity: EntityCommon { position, usage_bit_mask: 0, targeter: None }, resource_amount: 999999, initial_amount: None, variation: 0 }),
      _ => panic!("unsupported Entity {:?}", entity),
    };
    chunk.entities_to_be_inserted_before_setup.push((entity, entity_data));
  }
  
  pub fn from_template_map(template_name: &str) -> Result<Self> {
    let mut save_file = SaveFile::load_save_file(template_name).unwrap();

    let map_data = MapData::parse_map_data(&save_file.level_init_dat)?;
    let mut input_actions = Vec::new();
    let script_init_dat = save_file.script_init_dat;

    input_actions.push(InputAction::new(0, 255, InputActionData::SingleplayerInit));
    input_actions.push(InputAction::new(0, 255, InputActionData::GameCreatedFromScenario));
    input_actions.push(InputAction::new(0, 255, InputActionData::DisconnectAllPlayers));
    input_actions.push(InputAction::new(0, 255, InputActionData::PlayerJoinGame(PlayerJoinGameData { peer_id: 0, player_index: 0, force_id: ForceId::Player, username: "MrWint".to_owned(), as_editor: false, admin: true })));
  
    Ok(Self { map_data, input_actions, script_init_dat, tick: 0 })
  }

  pub fn build_miner_for(&mut self, resource: Item, position: TilePosition, direction: Direction) {
    let resource_entity = match resource {
      Item::IronOre => Entity::IronOre,
      _ => panic!("Unknown resource entity for item {:?}", resource),
    };
    self.add_entity_to_map(resource_entity, position.center_map_position());
    self.build(Item::BurnerMiningDrill, position.top_left_map_position(), direction)
  }

  pub fn build(&mut self, item: Item, position: MapPosition, direction: Direction) {
    self.input_actions.push(InputAction::new(self.tick, PID, InputActionData::SetFilter(SetFilterParameters { target: ItemStackTargetSpecification::from_quick_bar(0, 0), filter: item, }))); // Configure quickbar slot
    self.input_actions.push(InputAction::new(self.tick, PID, InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, }))); // Select quickbar into cursor
    self.input_actions.push(InputAction::new(self.tick, PID, InputActionData::Build(BuildParameters { position, direction, created_by_moving: false, build_by_moving_start_position: None, flags: 0, }))); // Build item from cursor
    self.input_actions.push(InputAction::new(self.tick, PID, InputActionData::ClearCursor)); // Clear cursor
    self.input_actions.push(InputAction::new(self.tick, PID, InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { target_quick_bar_slot: 0, item_to_use: ItemStackTargetSpecification::from_nothing(), currently_selected_quick_bar_slot: 65535 }))); // Clear quickbar slot
  }

  pub fn add_item(&mut self, item: Item, amount: usize, pos: MapPosition) {
    self.input_actions.push(InputAction::new(self.tick, PID, InputActionData::SetFilter(SetFilterParameters { target: ItemStackTargetSpecification::from_quick_bar(0, 0), filter: item, }))); // Configure quickbar slot
    self.input_actions.push(InputAction::new(self.tick, PID, InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, }))); // Select quickbar into cursor
    self.input_actions.push(InputAction::new(self.tick, PID, InputActionData::SelectedEntityChanged(pos))); // Select entity
    for _ in 0..amount {
      self.input_actions.push(InputAction::new(self.tick, PID, InputActionData::DropItem(pos)));
    }
    self.input_actions.push(InputAction::new(self.tick, PID, InputActionData::SelectedEntityCleared)); // Clear selection
    self.input_actions.push(InputAction::new(self.tick, PID, InputActionData::ClearCursor)); // Clear cursor
    self.input_actions.push(InputAction::new(self.tick, PID, InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { target_quick_bar_slot: 0, item_to_use: ItemStackTargetSpecification::from_nothing(), currently_selected_quick_bar_slot: 65535 }))); // Clear quickbar slot
  }

  pub fn write_save_file(mut self, name: &str) -> Result<()> {
    self.input_actions.push(InputAction::new(self.tick + 1000, PID, InputActionData::StopWalking));

    let level_init_dat = self.map_data.write_map_data()?;
    let replay_dat = ReplayData::from_input_actions(self.input_actions).write_replay_data()?;

    SaveFile::from_raw_dat(level_init_dat, replay_dat, self.script_init_dat).write_save_file_instrumented(name).unwrap();
    Ok(())
  }
}