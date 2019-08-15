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

  pub fn build(mut self, item: Item, x: i32, y: i32, dir: CardinalDirection) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SetFilter { slot: Slot { typ: SlotType::Quickbar, slot: 0, }, item, })); // Configure quickbar slot
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarPickSlot { slot: 0, })); // Select quickbar into cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::BuildItem { x, y, dir, ghost: false, })); // Build item from cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::CleanCursorStack)); // Clear cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarSetSlot { slot: 0, source_slot: Slot { typ: SlotType::Nothing, slot: 0xffff, }})); // Clear quickbar slot
    self
  }

  pub fn add_fuel(mut self, item: Item, amount: usize, x: i32, y: i32) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SetFilter { slot: Slot { typ: SlotType::Quickbar, slot: 0, }, item, })); // Configure quickbar slot
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarPickSlot { slot: 0, })); // Select quickbar into cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityChanged { x, y, })); // Select entity
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::OpenGui)); // Open GUI
    for _ in 0..amount {
      self.items.push(ReplayItem::new(self.tick, PID, InputAction::CursorSplit { slot: Slot { typ: SlotType::ContainerOrMachineFuel, slot: 0, }, }));
    }
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::CloseGui)); // Close GUI
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityCleared)); // Clear selection
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::CleanCursorStack)); // Clear cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarSetSlot { slot: 0, source_slot: Slot { typ: SlotType::Nothing, slot: 0xffff, }})); // Clear quickbar slot
    self
  }

  #[allow(dead_code)]
  pub fn add_input(mut self, item: Item, amount: usize, x: i32, y: i32) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SetFilter { slot: Slot { typ: SlotType::Quickbar, slot: 0, }, item, })); // Configure quickbar slot
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarPickSlot { slot: 0, })); // Select quickbar into cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityChanged { x, y, })); // Select entity
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::OpenGui)); // Open GUI
    for _ in 0..amount {
      self.items.push(ReplayItem::new(self.tick, PID, InputAction::CursorSplit { slot: Slot { typ: SlotType::MachineInput, slot: 0, }, }));
    }
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::CloseGui)); // Close GUI
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityCleared)); // Clear selection
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::CleanCursorStack)); // Clear cursor
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::QuickBarSetSlot { slot: 0, source_slot: Slot { typ: SlotType::Nothing, slot: 0xffff, }})); // Clear quickbar slot
    self
  }

  pub fn take_contents(mut self, x: i32, y: i32) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityChanged { x, y, })); // Select entity
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::FastEntityTransfer { dir: TransferDirection::Out, })); // Take items
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityCleared)); // Clear selection
    self
  }

  pub fn mine_for(mut self, ticks: u32, x: i32, y: i32) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityChanged { x, y, })); // Select entity
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::BeginMining)); // begin mining
    self.tick += ticks;
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::StopMining)); // stop mining
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::SelectedEntityCleared)); // Clear selection
    self
  }

  #[allow(dead_code)]
  pub fn walk_for(mut self, ticks: u32, dir: Direction) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::StartWalking { dir, })); // begin walking
    self.tick += ticks;
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::StopWalking)); // stop walking
    self
  }

  #[allow(unused_imports)]
  pub fn craft(mut self, recipe: Recipe, amount: u32) -> Self {
    self.items.push(ReplayItem::new(self.tick, PID, InputAction::Craft { recipe, amount, })); // begin crafting
    self
  }

  pub fn wait_for(mut self, ticks: u32) -> Self {
    self.tick += ticks;
    self
  }

  pub fn into_replay_items(mut self) -> Vec<ReplayItem> {
    self.items.push(ReplayItem::new(self.tick + 1000, PID, InputAction::StopWalking)); // extend the replay a bit
    self.items
  }
}