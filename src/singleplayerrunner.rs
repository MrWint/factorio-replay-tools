use factorio_serialize::constants::*;
use factorio_serialize::replay::*;

const PID: u16 = 0;

pub struct SinglePlayerRunner {
  items: Vec<InputAction>,
  tick: u32,
}

impl SinglePlayerRunner {
  pub fn new<S: Into<String>>(player_name: S) -> Self {
    Self { items: vec![
      InputAction::new(0, 0xff, InputActionData::SingleplayerInit),
      InputAction::new(0, 0xff, InputActionData::PlayerJoinGame(PlayerJoinGameData { peer_id: 0, player_index: PID, force_id: ForceId::Player, username: player_name.into(), as_editor: false, admin: true, })),
      InputAction::new(0, PID, InputActionData::ToggleShowEntityInfo),
    ], tick: 0, }
  }

  pub fn build(mut self, item: Item, position: MapPosition, direction: Direction) -> Self {
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SetFilter(SetFilterParameters { target: ItemStackTargetSpecification::from_quick_bar(0, 0), filter: item, }))); // Configure quickbar slot
    self.items.push(InputAction::new(self.tick, PID, InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, }))); // Select quickbar into cursor
    self.items.push(InputAction::new(self.tick, PID, InputActionData::Build(BuildParameters { position, direction, created_by_moving: false, build_by_moving_start_position: None, flags: 0, }))); // Build item from cursor
    self.items.push(InputAction::new(self.tick, PID, InputActionData::ClearCursor)); // Clear cursor
    self.items.push(InputAction::new(self.tick, PID, InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { target_quick_bar_slot: 0, item_to_use: ItemStackTargetSpecification::from_nothing(), currently_selected_quick_bar_slot: 65535 }))); // Clear quickbar slot
    self
  }

  pub fn add_item(mut self, item: Item, amount: usize, pos: MapPosition) -> Self {
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SetFilter(SetFilterParameters { target: ItemStackTargetSpecification::from_quick_bar(0, 0), filter: item, }))); // Configure quickbar slot
    self.items.push(InputAction::new(self.tick, PID, InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, }))); // Select quickbar into cursor
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SelectedEntityChanged(pos))); // Select entity
    for _ in 0..amount {
      self.items.push(InputAction::new(self.tick, PID, InputActionData::DropItem(pos)));
    }
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SelectedEntityCleared)); // Clear selection
    self.items.push(InputAction::new(self.tick, PID, InputActionData::ClearCursor)); // Clear cursor
    self.items.push(InputAction::new(self.tick, PID, InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { target_quick_bar_slot: 0, item_to_use: ItemStackTargetSpecification::from_nothing(), currently_selected_quick_bar_slot: 65535 }))); // Clear quickbar slot
    self
  }

  pub fn add_fuel(mut self, item: Item, amount: usize, pos: MapPosition) -> Self {
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SetFilter(SetFilterParameters { target: ItemStackTargetSpecification::from_quick_bar(0, 0), filter: item, }))); // Configure quickbar slot
    self.items.push(InputAction::new(self.tick, PID, InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, }))); // Select quickbar into cursor
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SelectedEntityChanged(pos))); // Select entity
    self.items.push(InputAction::new(self.tick, PID, InputActionData::OpenGui)); // Open GUI
    for _ in 0..amount {
      self.items.push(InputAction::new(self.tick, PID, InputActionData::CursorSplit(ItemStackTargetSpecification::from_fuel(0))));
    }
    self.items.push(InputAction::new(self.tick, PID, InputActionData::CloseGui)); // Close GUI
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SelectedEntityCleared)); // Clear selection
    self.items.push(InputAction::new(self.tick, PID, InputActionData::ClearCursor)); // Clear cursor
    self.items.push(InputAction::new(self.tick, PID, InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { target_quick_bar_slot: 0, item_to_use: ItemStackTargetSpecification::from_nothing(), currently_selected_quick_bar_slot: 65535 }))); // Clear quickbar slot
    self
  }

  #[allow(dead_code)]
  pub fn add_input(mut self, item: Item, amount: usize, pos: MapPosition) -> Self {
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SetFilter(SetFilterParameters { target: ItemStackTargetSpecification::from_quick_bar(0, 0), filter: item, }))); // Configure quickbar slot
    self.items.push(InputAction::new(self.tick, PID, InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, }))); // Select quickbar into cursor
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SelectedEntityChanged(pos))); // Select entity
    self.items.push(InputAction::new(self.tick, PID, InputActionData::OpenGui)); // Open GUI
    for _ in 0..amount {
      self.items.push(InputAction::new(self.tick, PID, InputActionData::CursorSplit(ItemStackTargetSpecification::from_machine_input(0))));
    }
    self.items.push(InputAction::new(self.tick, PID, InputActionData::CloseGui)); // Close GUI
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SelectedEntityCleared)); // Clear selection
    self.items.push(InputAction::new(self.tick, PID, InputActionData::ClearCursor)); // Clear cursor
    self.items.push(InputAction::new(self.tick, PID, InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { target_quick_bar_slot: 0, item_to_use: ItemStackTargetSpecification::from_nothing(), currently_selected_quick_bar_slot: 65535 }))); // Clear quickbar slot
    self
  }

  pub fn take_contents(mut self, pos: MapPosition) -> Self {
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SelectedEntityChanged(pos))); // Select entity
    self.items.push(InputAction::new(self.tick, PID, InputActionData::FastEntityTransfer(TransferDirection::Out))); // Take items
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SelectedEntityCleared)); // Clear selection
    self
  }

  pub fn mine_for(mut self, ticks: u32, pos: MapPosition) -> Self {
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SelectedEntityChanged(pos))); // Select entity
    self.items.push(InputAction::new(self.tick, PID, InputActionData::BeginMining)); // begin mining
    self.tick += ticks;
    self.items.push(InputAction::new(self.tick, PID, InputActionData::StopMining)); // stop mining
    self.items.push(InputAction::new(self.tick, PID, InputActionData::SelectedEntityCleared)); // Clear selection
    self
  }

  #[allow(dead_code)]
  pub fn walk_for(mut self, ticks: u32, direction: Direction) -> Self {
    self.items.push(InputAction::new(self.tick, PID, InputActionData::StartWalking(direction))); // begin walking
    self.tick += ticks;
    self.items.push(InputAction::new(self.tick, PID, InputActionData::StopWalking)); // stop walking
    self
  }

  #[allow(dead_code)]
  pub fn craft(mut self, recipe: Recipe, count: u32) -> Self {
    self.items.push(InputAction::new(self.tick, PID, InputActionData::Craft(CraftData { recipe, count, }))); // begin crafting
    self
  }

  #[allow(dead_code)]
  pub fn start_research(mut self, technology: Technology) -> Self {
    self.items.push(InputAction::new(self.tick, PID, InputActionData::StartResearch(technology))); // begin crafting
    self
  }

  #[allow(dead_code)]
  pub fn wait_for(mut self, ticks: u32) -> Self {
    self.tick += ticks;
    self
  }

  pub fn into_replay_items(mut self) -> Vec<InputAction> {
    self.items.push(InputAction::new(self.tick + 1000, PID, InputActionData::StopWalking)); // extend the replay a bit
    self.items
  }
}