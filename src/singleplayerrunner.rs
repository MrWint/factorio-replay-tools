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
      ReplayItem::new(0, 0xff, InputAction::SingleplayerInit),
      ReplayItem::new(0, 0xff, InputAction::PlayerJoinGame { player_id: PID, name: player_name.into(), }),
      ReplayItem::new(0, PID, InputAction::ToggleShowEntityInfo),
    ], tick: 0, }
  }

  pub fn build(mut self, item: Item, position: MapPosition, direction: Direction) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SetFilter { slot: Slot::from_quick_bar(0, 0), item, })); // Configure quickbar slot
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarPickSlot { slot: 0, })); // Select quickbar into cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::BuildItem(BuildItemParameters { position, direction, created_by_moving: false, allow_belt_power_replace: false, shift_build: false, skip_fog_of_war: false, }))); // Build item from cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::CleanCursorStack)); // Clear cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarSetSlot { slot: 0, source_slot: Slot::from_nothing() })); // Clear quickbar slot
    self
  }

  pub fn add_item(mut self, item: Item, amount: usize, pos: MapPosition) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SetFilter { slot: Slot::from_quick_bar(0, 0), item, })); // Configure quickbar slot
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarPickSlot { slot: 0, })); // Select quickbar into cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityChanged(pos))); // Select entity
    for _ in 0..amount {
      self.items.push(ReplayItem::new(self.tick, PID, InputAction::DropItem(pos)));
    }
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityCleared)); // Clear selection
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::CleanCursorStack)); // Clear cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarSetSlot { slot: 0, source_slot: Slot::from_nothing() })); // Clear quickbar slot
    self
  }

  pub fn add_fuel(mut self, item: Item, amount: usize, pos: MapPosition) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SetFilter { slot: Slot::from_quick_bar(0, 0), item, })); // Configure quickbar slot
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarPickSlot { slot: 0, })); // Select quickbar into cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityChanged(pos))); // Select entity
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::OpenGui)); // Open GUI
    for _ in 0..amount {
      self.items.push(ReplayItem::new(self.tick, PID, InputAction::CursorSplit(Slot::from_fuel(0))));
    }
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::CloseGui)); // Close GUI
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityCleared)); // Clear selection
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::CleanCursorStack)); // Clear cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarSetSlot { slot: 0, source_slot: Slot::from_nothing() })); // Clear quickbar slot
    self
  }

  #[allow(dead_code)]
  pub fn add_input(mut self, item: Item, amount: usize, pos: MapPosition) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SetFilter { slot: Slot::from_quick_bar(0, 0), item, })); // Configure quickbar slot
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarPickSlot { slot: 0, })); // Select quickbar into cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityChanged(pos))); // Select entity
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::OpenGui)); // Open GUI
    for _ in 0..amount {
      self.items.push(ReplayItem::new(self.tick, PID, InputAction::CursorSplit(Slot::from_machine_input(0))));
    }
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::CloseGui)); // Close GUI
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityCleared)); // Clear selection
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::CleanCursorStack)); // Clear cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarSetSlot { slot: 0, source_slot: Slot::from_nothing() })); // Clear quickbar slot
    self
  }

  pub fn take_contents(mut self, pos: MapPosition) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityChanged(pos))); // Select entity
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::FastEntityTransfer { dir: TransferDirection::Out, })); // Take items
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityCleared)); // Clear selection
    self
  }

  pub fn mine_for(mut self, ticks: u32, pos: MapPosition) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityChanged(pos))); // Select entity
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::BeginMining)); // begin mining
    self.tick += ticks;
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::StopMining)); // stop mining
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityCleared)); // Clear selection
    self
  }

  #[allow(dead_code)]
  pub fn walk_for(mut self, ticks: u32, direction: Direction) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::StartWalking(direction))); // begin walking
    self.tick += ticks;
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::StopWalking)); // stop walking
    self
  }

  #[allow(dead_code)]
  pub fn craft(mut self, recipe: Recipe, count: u32) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::Craft(CraftData { recipe, count, }))); // begin crafting
    self
  }

  #[allow(dead_code)]
  pub fn start_research(mut self, technology: Technology) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::StartResearch { technology, })); // begin crafting
    self
  }

  #[allow(dead_code)]
  pub fn wait_for(mut self, ticks: u32) -> Self {
    self.tick += ticks;
    self
  }

  pub fn into_replay_items(mut self) -> Vec<ReplayItem> {
    self.items.push(ReplayItem::new(self.tick + 1000, PID, InputAction::StopWalking)); // extend the replay a bit
    self.items
  }
}