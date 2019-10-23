use crate::action::*;
use crate::constants::*;
use crate::replay::*;

const PID: u16 = 0;

pub struct SinglePlayerRunner {
  items: Vec<ReplayItem>,
  tick: u32,
}

impl SinglePlayerRunner {
  pub fn new<S: Into<String>>(player_name: S) -> Self {
    Self { items: vec![
      ReplayItem::new(0, 0xff, InputActionData::SingleplayerInit),
      ReplayItem::new(0, 0xff, InputActionData::PlayerJoinGame(PlayerJoinGameData { peer_id: PID, player_index: 0, force_id: ForceId::Player, username: player_name.into(), as_editor: false, admin: true, })),
      ReplayItem::new(0, PID, InputActionData::ToggleShowEntityInfo),
    ], tick: 0, }
  }

  pub fn build(mut self, item: Item, position: MapPosition, direction: Direction) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SetFilter(SetFilterParameters { target: Slot::from_quick_bar(0, 0), filter: item, }))); // Configure quickbar slot
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, }))); // Select quickbar into cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::BuildItem(BuildItemParameters { position, direction, created_by_moving: false, allow_belt_power_replace: false, shift_build: false, skip_fog_of_war: false, }))); // Build item from cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::CleanCursorStack)); // Clear cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { location: 0, item_to_use: Slot::from_nothing() }))); // Clear quickbar slot
    self
  }

  pub fn add_item(mut self, item: Item, amount: usize, pos: MapPosition) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SetFilter(SetFilterParameters { target: Slot::from_quick_bar(0, 0), filter: item, }))); // Configure quickbar slot
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, }))); // Select quickbar into cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SelectedEntityChanged(pos))); // Select entity
    for _ in 0..amount {
      self.items.push(ReplayItem::new(self.tick, PID, InputActionData::DropItem(pos)));
    }
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SelectedEntityCleared)); // Clear selection
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::CleanCursorStack)); // Clear cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { location: 0, item_to_use: Slot::from_nothing() }))); // Clear quickbar slot
    self
  }

  pub fn add_fuel(mut self, item: Item, amount: usize, pos: MapPosition) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SetFilter(SetFilterParameters { target: Slot::from_quick_bar(0, 0), filter: item, }))); // Configure quickbar slot
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, }))); // Select quickbar into cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SelectedEntityChanged(pos))); // Select entity
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::OpenGui)); // Open GUI
    for _ in 0..amount {
      self.items.push(ReplayItem::new(self.tick, PID, InputActionData::CursorSplit(Slot::from_fuel(0))));
    }
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::CloseGui)); // Close GUI
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SelectedEntityCleared)); // Clear selection
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::CleanCursorStack)); // Clear cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { location: 0, item_to_use: Slot::from_nothing() }))); // Clear quickbar slot
    self
  }

  #[allow(dead_code)]
  pub fn add_input(mut self, item: Item, amount: usize, pos: MapPosition) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SetFilter(SetFilterParameters { target: Slot::from_quick_bar(0, 0), filter: item, }))); // Configure quickbar slot
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, }))); // Select quickbar into cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SelectedEntityChanged(pos))); // Select entity
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::OpenGui)); // Open GUI
    for _ in 0..amount {
      self.items.push(ReplayItem::new(self.tick, PID, InputActionData::CursorSplit(Slot::from_machine_input(0))));
    }
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::CloseGui)); // Close GUI
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SelectedEntityCleared)); // Clear selection
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::CleanCursorStack)); // Clear cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { location: 0, item_to_use: Slot::from_nothing() }))); // Clear quickbar slot
    self
  }

  pub fn take_contents(mut self, pos: MapPosition) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SelectedEntityChanged(pos))); // Select entity
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::FastEntityTransfer(TransferDirection::Out))); // Take items
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SelectedEntityCleared)); // Clear selection
    self
  }

  pub fn mine_for(mut self, ticks: u32, pos: MapPosition) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SelectedEntityChanged(pos))); // Select entity
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::BeginMining)); // begin mining
    self.tick += ticks;
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::StopMining)); // stop mining
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::SelectedEntityCleared)); // Clear selection
    self
  }

  #[allow(dead_code)]
  pub fn walk_for(mut self, ticks: u32, direction: Direction) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::StartWalking(direction))); // begin walking
    self.tick += ticks;
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::StopWalking)); // stop walking
    self
  }

  #[allow(dead_code)]
  pub fn craft(mut self, recipe: Recipe, count: u32) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::Craft(CraftData { recipe, count, }))); // begin crafting
    self
  }

  #[allow(dead_code)]
  pub fn start_research(mut self, technology: Technology) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputActionData::StartResearch(technology))); // begin crafting
    self
  }

  #[allow(dead_code)]
  pub fn wait_for(mut self, ticks: u32) -> Self {
    self.tick += ticks;
    self
  }

  pub fn into_replay_items(mut self) -> Vec<ReplayItem> {
    self.items.push(ReplayItem::new(self.tick + 1000, PID, InputActionData::StopWalking)); // extend the replay a bit
    self.items
  }
}