use crate::constants::*;
use num_traits::{FromPrimitive, ToPrimitive};
use std::io::{BufRead, Seek, Write};
use factorio_serialize::{Error, ReadWrite, Reader, Result, Writer};

#[derive(Debug)]
pub struct EquipmentData {
  pos: TilePosition,
  typ: EquipmentDataType,
}
impl ReadWrite for EquipmentData {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let pos = TilePosition::read(r)?;
    let typ = EquipmentDataType::read(r)?;
    Ok(Self { pos, typ, })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    self.pos.write(w)?;
    self.typ.write(w)
  }
}

#[derive(Debug)]
pub struct Slot { // ItemStackTargetSpecification
  pub inventory_index: u8, // from: defines.inventory
  pub slot_index: u16, // context-dependent
  pub source: SlotSource,
  pub target: SlotTarget,
}
impl Slot {
  #[allow(dead_code)] pub fn from_quick_bar(qbar: u16, bar_slot: u16) -> Self {
    Slot { inventory_index: 0x00, slot_index: qbar * 10 + bar_slot, source: SlotSource::PlayerQuickBar, target: SlotTarget::Default, }
  }
  #[allow(dead_code)] pub fn from_nothing() -> Self {
    Slot { inventory_index: 0xff, slot_index: 0xffff, source: SlotSource::Empty, target: SlotTarget::Default, }
  }
  #[allow(dead_code)] pub fn from_cursor() -> Self {
    Slot { inventory_index: 0xff, slot_index: 0xffff, source: SlotSource::PlayerCursor, target: SlotTarget::Default, }
  }
  #[allow(dead_code)] pub fn from_player_inventory(slot_index: u16) -> Self {
    Slot { inventory_index: 1, slot_index, source: SlotSource::PlayerInventory, target: SlotTarget::Default, }
  }
  #[allow(dead_code)] pub fn from_container(slot_index: u16) -> Self {
    Slot { inventory_index: 1, slot_index, source: SlotSource::EntityInventory, target: SlotTarget::Default, }
  }
  #[allow(dead_code)] pub fn from_fuel(slot_index: u16) -> Self {
    Slot { inventory_index: 1, slot_index, source: SlotSource::EntityInventory, target: SlotTarget::Default, }
  }
  #[allow(dead_code)] pub fn from_machine_input(slot_index: u16) -> Self {
    Slot { inventory_index: 2, slot_index, source: SlotSource::EntityInventory, target: SlotTarget::Default, }
  }
  #[allow(dead_code)] pub fn from_machine_output(slot_index: u16) -> Self {
    Slot { inventory_index: 3, slot_index, source: SlotSource::EntityInventory, target: SlotTarget::Default, }
  }
}
impl ReadWrite for Slot {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let inventory_index = r.read_u8()?;
    let slot_index = r.read_u16()?;
    let source = SlotSource::read(r)?;
    let target = SlotTarget::read(r)?;
    Ok(Slot { inventory_index, slot_index, source, target, })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_u8(self.inventory_index)?;
    w.write_u16(self.slot_index)?;
    self.source.write(w)?;
    self.target.write(w)
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MapPosition { // in 1/256th tiles
  pub x: i32,
  pub y: i32,
}
impl MapPosition {
  pub fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }
}
impl ReadWrite for MapPosition {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let x = r.read_i32()?;
    let y = r.read_i32()?;
    Ok(MapPosition { x, y, })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_i32(self.x)?;
    w.write_i32(self.y)
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
  pub left_top: MapPosition,
  pub right_bottom: MapPosition,
  pub orientation: f32, // always in [0,1)
}
impl ReadWrite for BoundingBox {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let left_top = MapPosition::read(r)?;
    let right_bottom = MapPosition::read(r)?;
    let orientation = r.read_f32()?;
    Ok(BoundingBox { left_top, right_bottom, orientation, })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    self.left_top.write(w)?;
    self.right_bottom.write(w)?;
    w.write_f32(self.orientation)
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelectAreaData {
  pub bounding_box: BoundingBox,
  pub item: Item,
  pub skip_fog_of_war: bool,
}
impl ReadWrite for SelectAreaData {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let bounding_box = BoundingBox::read(r)?;
    let item = Item::read(r)?;
    let skip_fog_of_war = r.read_bool()?;
    Ok(SelectAreaData { bounding_box, item, skip_fog_of_war, })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    self.bounding_box.write(w)?;
    self.item.write(w)?;
    w.write_bool(self.skip_fog_of_war)
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TilePosition {
  pub x: i32,
  pub y: i32,
}
impl ReadWrite for TilePosition {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let x = r.read_i32()?;
    let y = r.read_i32()?;
    Ok(TilePosition { x, y, })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_i32(self.x)?;
    w.write_i32(self.y)
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CrcData {
  pub crc: u32,
  pub tick_of_crc: u32,
}
impl ReadWrite for CrcData {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let crc = r.read_u32()?;
    let tick_of_crc = r.read_u32()?;
    Ok(Self { crc, tick_of_crc, })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_u32(self.crc)?;
    w.write_u32(self.tick_of_crc)
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SignalId {
  Item { item: Item },
  Fluid { fluid: Fluid, },
  VirtualSignal { virtual_signal: VirtualSignal, },
}
impl ReadWrite for SignalId {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    match SignalIdType::read(r)? {
      SignalIdType::Item => Ok(SignalId::Item { item: Item::read(r)?, }),
      SignalIdType::Fluid => Ok(SignalId::Fluid { fluid: Fluid::read(r)?, }),
      SignalIdType::VirtualSignal => Ok(SignalId::VirtualSignal { virtual_signal: VirtualSignal::read(r)?, }),
    }
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    match self {
      SignalId::Item { item, } => {
        SignalIdType::Item.write(w)?;
        item.write(w)
      },
      SignalId::Fluid { fluid, } => {
        SignalIdType::Fluid.write(w)?;
        fluid.write(w)
      },
      SignalId::VirtualSignal { virtual_signal, } => {
        SignalIdType::VirtualSignal.write(w)?;
        virtual_signal.write(w)
      },
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SignalIdOrConstant {
  SignalId { signal_id: SignalId },
  Constant { value: i32, },
}
impl ReadWrite for SignalIdOrConstant {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let signal_id = SignalId::read(r)?;
    let value = r.read_i32()?;
    if r.read_bool()? {
      Ok(SignalIdOrConstant::Constant { value, })
    } else {
      Ok(SignalIdOrConstant::SignalId { signal_id, })
    }
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    match self {
      SignalIdOrConstant::Constant { value, } => {
        SignalId::Item { item: Item::WoodenChest }.write(w)?;
        w.write_i32(*value)?;
        w.write_bool(true)
      },
      SignalIdOrConstant::SignalId { signal_id, } => {
        signal_id.write(w)?;
        w.write_i32(0)?;
        w.write_bool(true)
      },
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GuiChangedData {
  pub gui_element_index: u32,
  pub button: MouseButton,
  pub is_alt: bool,
  pub is_control: bool,
  pub is_shift: bool,
}
impl ReadWrite for GuiChangedData {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let gui_element_index = r.read_u32()?;
    let button = MouseButton::read(r)?;
    let is_alt = r.read_bool()?;
    let is_control = r.read_bool()?;
    let is_shift = r.read_bool()?;
    Ok(GuiChangedData { gui_element_index, button, is_alt, is_control, is_shift, })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_u32(self.gui_element_index)?;
    self.button.write(w)?;
    w.write_bool(self.is_alt)?;
    w.write_bool(self.is_control)?;
    w.write_bool(self.is_shift)
  }
}

// #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
// pub struct SetupBlueprintData {
//   pub include_modules: bool,
//   pub include_entities: bool,
//   pub include_tiles: bool,
//   pub include_station_names: bool,
//   pub include_trains: bool,
//   pub excluded_entities: Vec<u32>,
//   pub excluded_tiles: Vec<u32>,
//   pub excluded_items: Vec<Item>,
//   pub icons: Vec<SignalId>,
// }
// impl ReadWrite for SetupBlueprintData {
//   fn read(r: &mut ReplayReader) -> Result<Self> {
//     let include_modules = r.read_bool()?;
//     let include_entities = r.read_bool()?;
//     let include_tiles = r.read_bool()?;
//     let include_station_names = r.read_bool()?;
//     let include_trains = r.read_bool()?;

//     let count = r.read_opt_u32()?;

//     Ok(SetupBlueprintData { include_modules, include_entities, include_tiles, include_station_names, include_trains, })
//   }
//   fn write(&self, w: &mut ReplayWriter) -> Result<()> {
//     w.write_bool(self.include_modules)?;
//     w.write_bool(self.include_entities)?;
//     w.write_bool(self.include_tiles)?;
//     w.write_bool(self.include_station_names)?;
//     w.write_bool(self.include_trains)
//   }
// }

#[derive(Debug)]
pub enum InputAction {
  ActivateCopy,
  ActivateCut,
  ActivatePaste,
  AddTrainStation { name: String, pos: MapPosition, temporary: bool, },
  AlternativeCopy { area: SelectAreaData },
  AltSelectBlueprintEntities { area: SelectAreaData },
  BeginMining,
  BeginMiningTerrain { pos: MapPosition },
  BuildItem { pos: MapPosition, dir: Direction, created_by_moving: bool, size: u8, ghost_mode: bool, skip_fog_of_war: bool, },
  CancelCraft { crafting_index: u16, count: u16, },
  CancelDropBlueprintRecord,
  CancelNewBlueprint,
  ChangeActiveItemGroupForCrafting { item_group: ItemGroup, },
  ChangeActiveItemGroupForFilters { item_group: ItemGroup, },
  ChangeRidingState { direction: RidingDirection, acceleration_state: RidingAccelerationState, },
  ChangeShootingState { state: ShootingState, pos: MapPosition, },
  ChangeTrainStopStation { new_name: String, },
  CheckCRC { crc: CrcData, },
  CheckCRCHeuristic { crc: CrcData, },
  CleanCursorStack,
  CloseBlueprintRecord,
  CloseGui,
  ConnectRollingStock,
  Copy { area: SelectAreaData },
  CopyEntitySettings,
  Craft { recipe: Recipe, amount: u32, },
  CursorSplit { slot: Slot, },
  CursorTransfer { slot: Slot, },
  CycleBlueprintBookBackwards,
  CycleBlueprintBookForwards,
  CycleClipboardBackwards,
  CycleClipboardForwards,
  Deconstruct { area: SelectAreaData },
  DeleteBlueprintLibrary,
  DestroyOpenedItem,
  DisconnectRollingStock,
  DropItem { pos: MapPosition },
  GameCreatedFromScenario,
  GuiCheckedStateChanged { gui_changed_data: GuiChangedData },
  GuiClick { gui_changed_data: GuiChangedData, },
  GuiConfirmed { gui_changed_data: GuiChangedData, },
  GuiLocationChanged { gui_changed_data: GuiChangedData, x: i32, y: i32 },
  GuiSelectedTabChanged { gui_changed_data: GuiChangedData, value: i32 },
  GuiSelectionStateChanged { gui_changed_data: GuiChangedData, value: i32 },
  GuiSwitchStateChanged { gui_changed_data: GuiChangedData, value: SwitchState },
  GuiTextChanged { gui_changed_data: GuiChangedData, value: String },
  GuiValueChanged { gui_changed_data: GuiChangedData, value: f64 },
  InventorySplit { slot: Slot, },
  InventoryTransfer { slot: Slot, },
  LaunchRocket,
  MarketOffer { slot_index: u32, count: u32, },
  MoveOnZoom { x: f64, y: f64, },
  MultiplayerInit,
  Nothing,
  OpenAchievementsGui,
  OpenBlueprintLibraryGui,
  OpenBonusGui,
  OpenCharacterGui,
  OpenEquipment { equipment: EquipmentData, },
  OpenGui,
  OpenItem { slot: Slot, },
  OpenKillsGui,
  OpenLogisticGui,
  OpenModItem { slot: Slot, },
  OpenProductionGui,
  OpenTechnologyGui,
  OpenTrainsGui,
  OpenTutorialsGui,
  PasteEntitySettings,
  PlaceEquipment { equipment: EquipmentData, },
  ResetAssemblingMachine,
  SelectBlueprintEntities { area: SelectAreaData },
  SelectedEntityChanged { pos: MapPosition, },
  SelectedEntityCleared,
  SelectNextValidGun,
  SetCircuitCondition { circuit_index: u8, comparison: Comparison, first_signal_id: SignalId, second_signal_id: SignalIdOrConstant, },
  SetCircuitModeOfOperation { mode_of_operation: u8, enabled: bool, },
  SetFilter { slot: Slot, item: Item, },
  SetInventoryBar { slot: Slot, },
  SetLogisticFilterItem { item: Item, filter_index: u16, count: u32, },
  SetLogisticFilterSignal { signal: SignalId, filter_index: u16, count: u32, },
  SetSignal { signal_id: SignalId, signal_index: u16, },
  SetupAssemblingMachine { recipe: Recipe, },
  SingleplayerInit,
  SmartPipette { entity: Entity, tile: Tile, pick_ghost_cursor: bool },
  StackSplit { slot: Slot, },
  StackTransfer { slot: Slot, },
  StartRepair { pos: MapPosition, },
  StartResearch { technology: Technology },
  StartWalking { dir: Direction },
  StopBuildingByMoving,
  StopMining,
  StopMovementInTheNextTick,
  StopRepair,
  StopWalking,
  SwitchToRenameStopGui,
  TakeEquipment { equipment: EquipmentData, },
  ToggleDeconstructionItemEntityFilterMode,
  ToggleDeconstructionItemTileFilterMode,
  ToggleDriving,
  ToggleEnableVehicleLogisticsWhileMoving,
  ToggleEquipmentMovementBonus,
  ToggleMapEditor,
  TogglePersonalRoboport,
  ToggleShowEntityInfo,
  Undo,
  Upgrade { area: SelectAreaData },
  UpgradeOpenedBlueprint,
  UseArtilleryRemote { pos: MapPosition, },
  UseItem { pos: MapPosition, },
  WireDragging { pos: MapPosition, },
  WriteToConsole { value: String, },



  FastEntityTransfer { dir: TransferDirection, },
  FastEntitySplit { dir: TransferDirection, },
  PlayerJoinGame { player_id: u16, name: String },
  QuickBarPickSlot { slot: u16, },
  QuickBarSetSlot { slot: u16,  source_slot: Slot, },
  SelectedEntityChangedRelative { x: i16, y: i16, },
  SelectedEntityChangedVeryClose { x: i8, y: i8, },
  SelectedEntityChangedVeryClosePrecise { x: i8, y: i8, },
}
impl InputAction {
  pub fn read<R: BufRead + Seek>(action_type: InputActionType, action_type_pos: u64, r: &mut Reader<R>) -> Result<InputAction> {
    match action_type {
      InputActionType::ActivateCopy => Ok(InputAction::ActivateCopy),
      InputActionType::ActivateCut => Ok(InputAction::ActivateCut),
      InputActionType::ActivatePaste => Ok(InputAction::ActivatePaste),
      InputActionType::AddTrainStation => {
        let name = r.read_string()?;
        let pos = MapPosition::read(r)?;
        let temporary = r.read_bool()?;
        Ok(InputAction::AddTrainStation { name, pos, temporary, })
      },
      InputActionType::AltSelectBlueprintEntities => Ok(InputAction::AltSelectBlueprintEntities { area: SelectAreaData::read(r)? }),
      InputActionType::AlternativeCopy => Ok(InputAction::AlternativeCopy { area: SelectAreaData::read(r)? }),
      InputActionType::BeginMining => Ok(InputAction::BeginMining),
      InputActionType::BeginMiningTerrain => Ok(InputAction::BeginMiningTerrain { pos: MapPosition::read(r)? }),
      InputActionType::BuildItem => {
        let pos = MapPosition::read(r)?;
        let dir = Direction::read(r)?;
        let created_by_moving = r.read_bool()?;
        let size = r.read_u8()?;
        let ghost_mode = r.read_bool()?;
        let skip_fog_of_war = r.read_bool()?;
        Ok(InputAction::BuildItem { pos, dir, created_by_moving, size, ghost_mode, skip_fog_of_war, })
      },
      InputActionType::CancelCraft => {
        let crafting_index = r.read_u16()?;
        let count = r.read_u16()?;
        Ok(InputAction::CancelCraft { crafting_index, count, })
      },
      InputActionType::CancelDropBlueprintRecord => Ok(InputAction::CancelDropBlueprintRecord),
      InputActionType::CancelNewBlueprint => Ok(InputAction::CancelNewBlueprint),
      InputActionType::ChangeActiveItemGroupForCrafting => Ok(InputAction::ChangeActiveItemGroupForCrafting { item_group: ItemGroup::read(r)?, }),
      InputActionType::ChangeActiveItemGroupForFilters => Ok(InputAction::ChangeActiveItemGroupForFilters { item_group: ItemGroup::read(r)?, }),
      InputActionType::ChangeRidingState => {
        let direction = RidingDirection::read(r)?;
        let acceleration_state = RidingAccelerationState::read(r)?;
        Ok(InputAction::ChangeRidingState { direction, acceleration_state, })
      },
      InputActionType::ChangeShootingState => {
        let state = ShootingState::read(r)?;
        let pos = MapPosition::read(r)?;
        Ok(InputAction::ChangeShootingState { state, pos, })
      },
      InputActionType::ChangeTrainStopStation => Ok(InputAction::ChangeTrainStopStation { new_name: r.read_string()?, }),
      InputActionType::CheckCRC => Ok(InputAction::CheckCRC { crc: CrcData::read(r)?, }),
      InputActionType::CheckCRCHeuristic => Ok(InputAction::CheckCRCHeuristic { crc: CrcData::read(r)?, }),
      InputActionType::CleanCursorStack => Ok(InputAction::CleanCursorStack),
      InputActionType::CloseBlueprintRecord => Ok(InputAction::CloseBlueprintRecord),
      InputActionType::CloseGui => Ok(InputAction::CloseGui),
      InputActionType::ConnectRollingStock => Ok(InputAction::ConnectRollingStock),
      InputActionType::Copy => Ok(InputAction::Copy { area: SelectAreaData::read(r)? }),
      InputActionType::CopyEntitySettings => Ok(InputAction::CopyEntitySettings),
      InputActionType::Craft => {
        let recipe = Recipe::read(r)?;
        let amount = r.read_u32()?;
        Ok(InputAction::Craft { recipe, amount, })
      },
      InputActionType::CursorSplit => Ok(InputAction::CursorSplit { slot: Slot::read(r)? }),
      InputActionType::CursorTransfer => Ok(InputAction::CursorTransfer { slot: Slot::read(r)? }),
      InputActionType::CycleBlueprintBookBackwards => Ok(InputAction::CycleBlueprintBookBackwards),
      InputActionType::CycleBlueprintBookForwards => Ok(InputAction::CycleBlueprintBookForwards),
      InputActionType::CycleClipboardBackwards => Ok(InputAction::CycleClipboardBackwards),
      InputActionType::CycleClipboardForwards => Ok(InputAction::CycleClipboardForwards),
      InputActionType::Deconstruct => Ok(InputAction::Deconstruct { area: SelectAreaData::read(r)? }),
      InputActionType::DeleteBlueprintLibrary => Ok(InputAction::DeleteBlueprintLibrary),
      InputActionType::DestroyOpenedItem => Ok(InputAction::DestroyOpenedItem),
      InputActionType::DisconnectRollingStock => Ok(InputAction::DisconnectRollingStock),
      InputActionType::DropItem => Ok(InputAction::DropItem { pos: MapPosition::read(r)? }),
      InputActionType::GameCreatedFromScenario => Ok(InputAction::GameCreatedFromScenario),
      InputActionType::GuiCheckedStateChanged => Ok(InputAction::GuiCheckedStateChanged { gui_changed_data: GuiChangedData::read(r)?, }),
      InputActionType::GuiClick => Ok(InputAction::GuiClick { gui_changed_data: GuiChangedData::read(r)?, }),
      InputActionType::GuiConfirmed => Ok(InputAction::GuiConfirmed { gui_changed_data: GuiChangedData::read(r)?, }),
      InputActionType::GuiLocationChanged => {
        let gui_changed_data = GuiChangedData::read(r)?;
        let x = r.read_i32()?;
        let y = r.read_i32()?;
        Ok(InputAction::GuiLocationChanged { gui_changed_data, x, y })
      },
      InputActionType::GuiSelectedTabChanged => {
        let gui_changed_data = GuiChangedData::read(r)?;
        let value = r.read_i32()?;
        Ok(InputAction::GuiSelectedTabChanged { gui_changed_data, value })
      },
      InputActionType::GuiSelectionStateChanged => {
        let gui_changed_data = GuiChangedData::read(r)?;
        let value = r.read_i32()?;
        Ok(InputAction::GuiSelectionStateChanged { gui_changed_data, value })
      },
      InputActionType::GuiSwitchStateChanged => {
        let gui_changed_data = GuiChangedData::read(r)?;
        let value = SwitchState::read(r)?;
        Ok(InputAction::GuiSwitchStateChanged { gui_changed_data, value })
      },
      InputActionType::GuiTextChanged => {
        let gui_changed_data = GuiChangedData::read(r)?;
        let value = r.read_string()?;
        Ok(InputAction::GuiTextChanged { gui_changed_data, value })
      },
      InputActionType::GuiValueChanged => {
        let gui_changed_data = GuiChangedData::read(r)?;
        let value = r.read_f64()?;
        Ok(InputAction::GuiValueChanged { gui_changed_data, value })
      },
      InputActionType::InventorySplit => Ok(InputAction::InventorySplit { slot: Slot::read(r)? }),
      InputActionType::InventoryTransfer => Ok(InputAction::InventoryTransfer { slot: Slot::read(r)? }),
      InputActionType::LaunchRocket => Ok(InputAction::LaunchRocket),
      InputActionType::MarketOffer => {
        let slot_index = r.read_u32()?;
        let count = r.read_u32()?;
        Ok(InputAction::MarketOffer { slot_index, count, })
      },
      InputActionType::MoveOnZoom => {
        let x = r.read_f64()?;
        let y = r.read_f64()?;
        Ok(InputAction::MoveOnZoom { x, y, })
      },
      InputActionType::MultiplayerInit => Ok(InputAction::MultiplayerInit),
      InputActionType::Nothing => Ok(InputAction::Nothing),
      InputActionType::OpenAchievementsGui => Ok(InputAction::OpenAchievementsGui),
      InputActionType::OpenBlueprintLibraryGui => Ok(InputAction::OpenBlueprintLibraryGui),
      InputActionType::OpenBonusGui => Ok(InputAction::OpenBonusGui),
      InputActionType::OpenCharacterGui => Ok(InputAction::OpenCharacterGui),
      InputActionType::OpenEquipment => Ok(InputAction::OpenEquipment { equipment: EquipmentData::read(r)?, }),
      InputActionType::OpenGui => Ok(InputAction::OpenGui),
      InputActionType::OpenItem => Ok(InputAction::OpenItem { slot: Slot::read(r)?, }),
      InputActionType::OpenKillsGui => Ok(InputAction::OpenKillsGui),
      InputActionType::OpenLogisticGui => Ok(InputAction::OpenLogisticGui),
      InputActionType::OpenModItem => Ok(InputAction::OpenModItem { slot: Slot::read(r)?, }),
      InputActionType::OpenProductionGui => Ok(InputAction::OpenProductionGui),
      InputActionType::OpenTechnologyGui => Ok(InputAction::OpenTechnologyGui),
      InputActionType::OpenTrainsGui => Ok(InputAction::OpenTrainsGui),
      InputActionType::OpenTutorialsGui => Ok(InputAction::OpenTutorialsGui),
      InputActionType::PasteEntitySettings => Ok(InputAction::PasteEntitySettings),
      InputActionType::PlaceEquipment => Ok(InputAction::PlaceEquipment { equipment: EquipmentData::read(r)?, }),
      InputActionType::ResetAssemblingMachine => Ok(InputAction::ResetAssemblingMachine),
      InputActionType::SelectBlueprintEntities => Ok(InputAction::SelectBlueprintEntities { area: SelectAreaData::read(r)? }),
      InputActionType::SelectedEntityChanged => Ok(InputAction::SelectedEntityChanged { pos: MapPosition::read(r)?, }),
      InputActionType::SelectedEntityCleared => Ok(InputAction::SelectedEntityCleared),
      InputActionType::SelectNextValidGun => Ok(InputAction::SelectNextValidGun),
      InputActionType::SetCircuitCondition => {
        let circuit_index = r.read_u8()?;
        let comparison = Comparison::read(r)?;
        let first_signal_id = SignalId::read(r)?;
        let second_signal_id = SignalIdOrConstant::read(r)?;
        Ok(InputAction::SetCircuitCondition { circuit_index, comparison, first_signal_id, second_signal_id, })
      },
      InputActionType::SetCircuitModeOfOperation => {
        let mode_of_operation = r.read_u8()?;
        let enabled = r.read_bool()?;
        Ok(InputAction::SetCircuitModeOfOperation { mode_of_operation, enabled, })
      },
      InputActionType::SetFilter => {
        let slot = Slot::read(r)?;
        let item = Item::read(r)?;
        Ok(InputAction::SetFilter { slot, item, })
      },
      InputActionType::SetInventoryBar => Ok(InputAction::SetInventoryBar { slot: Slot::read(r)? }),
      InputActionType::SetLogisticFilterItem => {
        let item = Item::read(r)?;
        let filter_index = r.read_u16()?;
        let count = r.read_u32()?;
        Ok(InputAction::SetLogisticFilterItem { item, filter_index, count, })
      },
      InputActionType::SetLogisticFilterSignal => {
        let signal = SignalId::read(r)?;
        let filter_index = r.read_u16()?;
        let count = r.read_u32()?;
        Ok(InputAction::SetLogisticFilterSignal { signal, filter_index, count, })
      },
      InputActionType::SetSignal => {
        let signal_id = SignalId::read(r)?;
        let signal_index = r.read_u16()?;
        Ok(InputAction::SetSignal { signal_id, signal_index, })
      },
      InputActionType::SetupAssemblingMachine => Ok(InputAction::SetupAssemblingMachine { recipe: Recipe::read(r)? }),
      InputActionType::SingleplayerInit => Ok(InputAction::SingleplayerInit),
      InputActionType::SmartPipette => {
        let entity = Entity::read(r)?;
        let tile = Tile::read(r)?;
        let pick_ghost_cursor = r.read_bool()?;
        Ok(InputAction::SmartPipette { entity, tile, pick_ghost_cursor })
      },
      InputActionType::StackSplit => Ok(InputAction::StackSplit { slot: Slot::read(r)? }),
      InputActionType::StackTransfer => Ok(InputAction::StackTransfer { slot: Slot::read(r)? }),
      InputActionType::StartRepair => Ok(InputAction::StartRepair { pos: MapPosition::read(r)?, }),
      InputActionType::StartResearch => Ok(InputAction::StartResearch { technology: Technology::read(r)?, }),
      InputActionType::StartWalking => Ok(InputAction::StartWalking { dir: Direction::read(r)? }),
      InputActionType::StopBuildingByMoving => Ok(InputAction::StopBuildingByMoving),
      InputActionType::StopMining => Ok(InputAction::StopMining),
      InputActionType::StopMovementInTheNextTick => Ok(InputAction::StopMovementInTheNextTick),
      InputActionType::StopRepair => Ok(InputAction::StopRepair),
      InputActionType::StopWalking => Ok(InputAction::StopWalking),
      InputActionType::SwitchToRenameStopGui => Ok(InputAction::SwitchToRenameStopGui),
      InputActionType::TakeEquipment => Ok(InputAction::TakeEquipment { equipment: EquipmentData::read(r)?, }),
      InputActionType::ToggleDeconstructionItemEntityFilterMode => Ok(InputAction::ToggleDeconstructionItemEntityFilterMode),
      InputActionType::ToggleDeconstructionItemTileFilterMode => Ok(InputAction::ToggleDeconstructionItemTileFilterMode),
      InputActionType::ToggleDriving => Ok(InputAction::ToggleDriving),
      InputActionType::ToggleEnableVehicleLogisticsWhileMoving => Ok(InputAction::ToggleEnableVehicleLogisticsWhileMoving),
      InputActionType::ToggleEquipmentMovementBonus => Ok(InputAction::ToggleEquipmentMovementBonus),
      InputActionType::ToggleMapEditor => Ok(InputAction::ToggleMapEditor),
      InputActionType::TogglePersonalRoboport => Ok(InputAction::TogglePersonalRoboport),
      InputActionType::ToggleShowEntityInfo => Ok(InputAction::ToggleShowEntityInfo),
      InputActionType::Undo => Ok(InputAction::Undo),
      InputActionType::UseItem => Ok(InputAction::UseItem { pos: MapPosition::read(r)?, }),
      InputActionType::UseArtilleryRemote => Ok(InputAction::UseArtilleryRemote { pos: MapPosition::read(r)?, }),
      InputActionType::Upgrade => Ok(InputAction::Upgrade { area: SelectAreaData::read(r)? }),
      InputActionType::UpgradeOpenedBlueprint => Ok(InputAction::UpgradeOpenedBlueprint),
      InputActionType::WireDragging => Ok(InputAction::WireDragging { pos: MapPosition::read(r)?, }),
      InputActionType::WriteToConsole => Ok(InputAction::WriteToConsole { value: r.read_string()?, }),




      InputActionType::QuickBarSetSelectedPage => {
        let qbar = r.read_u8()?; // top or bottom quickbar
        let set = r.read_u8()?; // 0-9
        println!("ignoring QuickBarSetSelectedPage setting bar {} to set {}!", qbar, set);
        Ok(InputAction::Nothing)
      }
      InputActionType::ChangePickingState => {
        let val1 = r.read_u8()?;
        println!("ignoring ChangePickingState {}!", val1);
        Ok(InputAction::Nothing)
      },
      InputActionType::QuickBarSetSlot => {
        let slot = r.read_u16()?; // fixed point number
        let source_slot = Slot::read(r)?;
        Ok(InputAction::QuickBarSetSlot { slot,  source_slot, })
      },
      InputActionType::DisplayResolutionChanged => {
        let x = r.read_u32()?; // ChunkPosition
        let y = r.read_u32()?;
        println!("ignoring DisplayResolutionChanged ({}, {})!", x, y);
        Ok(InputAction::Nothing)
      },
      InputActionType::DisplayScaleChanged => {
        let scale = r.read_f64()?; // scale
        println!("ignoring DisplayScaleChanged ({})!", scale);
        Ok(InputAction::Nothing)
      },
      InputActionType::FastEntitySplit => Ok(InputAction::FastEntitySplit { dir: TransferDirection::from_u8(r.read_u8()?).unwrap() }),
      InputActionType::FastEntityTransfer => Ok(InputAction::FastEntityTransfer { dir: TransferDirection::from_u8(r.read_u8()?).unwrap() }),
      InputActionType::PlayerJoinGame => {
        let player_id = r.read_opt_u16()?;
        r.read_u16_assert(0)?;
        r.read_u8_assert(1)?; // AllowedCommands
        let name = r.read_string()?;
        r.read_u8_assert(0)?;
        r.read_u8_assert(1)?;
        Ok(InputAction::PlayerJoinGame { player_id, name })
      },
      InputActionType::QuickBarPickSlot => {
        let slot = r.read_u16()?;
        r.read_u8_assert(0)?; // unknown boolean
        r.read_u8_assert(0)?; // unknown boolean
        Ok(InputAction::QuickBarPickSlot { slot, })
      },
      InputActionType::SelectedEntityChangedRelative => {
        let y = r.read_i16()?;
        let x = r.read_i16()?;
        Ok(InputAction::SelectedEntityChangedRelative { x, y, })
      },
      InputActionType::SelectedEntityChangedVeryClose => {
        let xy = r.read_u8()?;
        let x = (xy >> 4) as i8 - 8;
        let y = (xy & 0x0f) as i8 - 8;
        Ok(InputAction::SelectedEntityChangedVeryClose { x, y, })
      },
      InputActionType::SelectedEntityChangedVeryClosePrecise => {
        let y = r.read_i8()?;
        let x = r.read_i8()?;
        Ok(InputAction::SelectedEntityChangedVeryClosePrecise { x, y, })
      },
      InputActionType::UpdateBlueprintShelf => {
        r.read_u16_assert(0)?; // player id?
        r.read_u32_assert(1)?;
        r.read_u32()?; // checksum
        let unknown_count = r.read_opt_u32()?;
        for _ in 0..unknown_count { r.read_u32()?; } // dump unknown values
        let add_blueprint_record_data_count = r.read_opt_u32()?;
        for _ in 0..add_blueprint_record_data_count { r.read_past_add_blueprint_record_data()?; }
        let update_blueprint_data_count = r.read_opt_u32()?;
        for _ in 0..update_blueprint_data_count { r.read_past_update_blueprint_data()?; }
        println!("ignoring {} added and {} updated blueprints!", add_blueprint_record_data_count, update_blueprint_data_count);
        Ok(InputAction::Nothing)
      },
      _ => Err(Error::custom(format!("Unsupported action type {:?}", action_type), action_type_pos)),
    }
  }
  pub fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    match self {
      InputAction::ActivateCopy => Ok(()),
      InputAction::ActivateCut => Ok(()),
      InputAction::ActivatePaste => Ok(()),
      InputAction::AddTrainStation { name, pos, temporary, } => {
        w.write_string(name)?;
        pos.write(w)?;
        w.write_bool(*temporary)
      },
      InputAction::AlternativeCopy { area } => area.write(w),
      InputAction::AltSelectBlueprintEntities { area } => area.write(w),
      InputAction::BeginMining => Ok(()),
      InputAction::BeginMiningTerrain { pos, } => pos.write(w),
      InputAction::CancelCraft { crafting_index, count, } => {
        w.write_u16(*crafting_index)?;
        w.write_u16(*count)
      },
      InputAction::CancelDropBlueprintRecord => Ok(()),
      InputAction::CancelNewBlueprint => Ok(()),
      InputAction::ChangeActiveItemGroupForCrafting { item_group, } => item_group.write(w),
      InputAction::ChangeActiveItemGroupForFilters { item_group, } => item_group.write(w),
      InputAction::ChangeRidingState { acceleration_state, direction, } => {
        direction.write(w)?;
        acceleration_state.write(w)
      },
      InputAction::ChangeShootingState { state, pos, } => {
        state.write(w)?;
        pos.write(w)
      },
      InputAction::ChangeTrainStopStation { new_name, } => w.write_string(new_name),
      InputAction::CheckCRC { crc, } => crc.write(w),
      InputAction::CheckCRCHeuristic { crc, } => crc.write(w),
      InputAction::CleanCursorStack => Ok(()),
      InputAction::CloseBlueprintRecord => Ok(()),
      InputAction::CloseGui => Ok(()),
      InputAction::ConnectRollingStock => Ok(()),
      InputAction::Copy { area } => area.write(w),
      InputAction::CopyEntitySettings => Ok(()),
      InputAction::Craft { recipe, amount, } => {
        recipe.write(w)?;
        w.write_u32(*amount)
      },
      InputAction::CursorSplit { slot, } => slot.write(w),
      InputAction::CursorTransfer { slot, } => slot.write(w),
      InputAction::CycleBlueprintBookBackwards => Ok(()),
      InputAction::CycleBlueprintBookForwards => Ok(()),
      InputAction::CycleClipboardBackwards => Ok(()),
      InputAction::CycleClipboardForwards => Ok(()),
      InputAction::Deconstruct { area } => area.write(w),
      InputAction::DeleteBlueprintLibrary => Ok(()),
      InputAction::DestroyOpenedItem => Ok(()),
      InputAction::DisconnectRollingStock => Ok(()),
      InputAction::DropItem { pos, } => pos.write(w),
      InputAction::GameCreatedFromScenario => Ok(()),
      InputAction::GuiCheckedStateChanged { gui_changed_data } => gui_changed_data.write(w),
      InputAction::GuiClick { gui_changed_data, } => gui_changed_data.write(w),
      InputAction::GuiConfirmed { gui_changed_data, } => gui_changed_data.write(w),
      InputAction::GuiLocationChanged { gui_changed_data, x, y } => {
        gui_changed_data.write(w)?;
        w.write_i32(*x)?;
        w.write_i32(*y)
      },
      InputAction::GuiSelectedTabChanged { gui_changed_data, value } => {
        gui_changed_data.write(w)?;
        w.write_i32(*value)
      },
      InputAction::GuiSelectionStateChanged { gui_changed_data, value } => {
        gui_changed_data.write(w)?;
        w.write_i32(*value)
      },
      InputAction::GuiSwitchStateChanged { gui_changed_data, value } => {
        gui_changed_data.write(w)?;
        value.write(w)
      },
      InputAction::GuiTextChanged { gui_changed_data, value } => {
        gui_changed_data.write(w)?;
        w.write_string(value)
      },
      InputAction::GuiValueChanged { gui_changed_data, value } => {
        gui_changed_data.write(w)?;
        w.write_f64(*value)
      },
      InputAction::InventorySplit { slot, } => slot.write(w),
      InputAction::InventoryTransfer { slot, } => slot.write(w),
      InputAction::LaunchRocket => Ok(()),
      InputAction::MarketOffer { slot_index, count, } => {
        w.write_u32(*slot_index)?;
        w.write_u32(*count)
      },
      InputAction::MoveOnZoom { x, y, } => {
        w.write_f64(*x)?;
        w.write_f64(*y)
      },
      InputAction::MultiplayerInit => Ok(()),
      InputAction::Nothing => Ok(()),
      InputAction::OpenAchievementsGui => Ok(()),
      InputAction::OpenBlueprintLibraryGui => Ok(()),
      InputAction::OpenBonusGui => Ok(()),
      InputAction::OpenCharacterGui => Ok(()),
      InputAction::OpenEquipment { equipment, } => equipment.write(w),
      InputAction::OpenGui => Ok(()),
      InputAction::OpenItem { slot, } => slot.write(w),
      InputAction::OpenKillsGui => Ok(()),
      InputAction::OpenLogisticGui => Ok(()),
      InputAction::OpenModItem { slot, } => slot.write(w),
      InputAction::OpenProductionGui => Ok(()),
      InputAction::OpenTechnologyGui => Ok(()),
      InputAction::OpenTrainsGui => Ok(()),
      InputAction::OpenTutorialsGui => Ok(()),
      InputAction::PasteEntitySettings => Ok(()),
      InputAction::PlaceEquipment { equipment, } => equipment.write(w),
      InputAction::ResetAssemblingMachine => Ok(()),
      InputAction::SelectBlueprintEntities { area } => area.write(w),
      InputAction::SelectedEntityChanged { pos, } => pos.write(w),
      InputAction::SelectedEntityCleared => Ok(()),
      InputAction::SelectNextValidGun => Ok(()),
      InputAction::SetCircuitCondition { circuit_index, comparison, first_signal_id, second_signal_id, } => {
        w.write_u8(*circuit_index)?;
        comparison.write(w)?;
        first_signal_id.write(w)?;
        second_signal_id.write(w)
      },
      InputAction::SetCircuitModeOfOperation { mode_of_operation, enabled, } => {
        w.write_u8(*mode_of_operation)?;
        w.write_bool(*enabled)
      },
      InputAction::SetFilter { slot, item, } => {
        slot.write(w)?;
        item.write(w)
      },
      InputAction::SetInventoryBar { slot, } => slot.write(w),
      InputAction::SetLogisticFilterItem { item, filter_index, count, } => {
        item.write(w)?;
        w.write_u16(*filter_index)?;
        w.write_u32(*count)
      },
      InputAction::SetLogisticFilterSignal { signal, filter_index, count, } => {
        signal.write(w)?;
        w.write_u16(*filter_index)?;
        w.write_u32(*count)
      },
      InputAction::SetSignal { signal_id, signal_index, } => {
        signal_id.write(w)?;
        w.write_u16(*signal_index)
      },
      InputAction::SetupAssemblingMachine { recipe, } => recipe.write(w),
      InputAction::SingleplayerInit => Ok(()),
      InputAction::SmartPipette { entity, tile, pick_ghost_cursor, } => {
        entity.write(w)?;
        tile.write(w)?;
        w.write_bool(*pick_ghost_cursor)
      },
      InputAction::StackSplit { slot, } => slot.write(w),
      InputAction::StackTransfer { slot, } => slot.write(w),
      InputAction::StartRepair { pos, } => pos.write(w),
      InputAction::StartResearch { technology, } => technology.write(w),
      InputAction::StartWalking { dir, } => dir.write(w),
      InputAction::StopBuildingByMoving => Ok(()),
      InputAction::StopMining => Ok(()),
      InputAction::StopMovementInTheNextTick => Ok(()),
      InputAction::StopRepair => Ok(()),
      InputAction::StopWalking => Ok(()),
      InputAction::SwitchToRenameStopGui => Ok(()),
      InputAction::TakeEquipment { equipment, } => equipment.write(w),
      InputAction::ToggleDeconstructionItemEntityFilterMode => Ok(()),
      InputAction::ToggleDeconstructionItemTileFilterMode => Ok(()),
      InputAction::ToggleDriving => Ok(()),
      InputAction::ToggleEnableVehicleLogisticsWhileMoving => Ok(()),
      InputAction::ToggleEquipmentMovementBonus => Ok(()),
      InputAction::ToggleMapEditor => Ok(()),
      InputAction::TogglePersonalRoboport => Ok(()),
      InputAction::ToggleShowEntityInfo => Ok(()),
      InputAction::Undo => Ok(()),
      InputAction::UseItem { pos, } => pos.write(w),
      InputAction::UseArtilleryRemote { pos, } => pos.write(w),
      InputAction::Upgrade { area } => area.write(w),
      InputAction::UpgradeOpenedBlueprint => Ok(()),
      InputAction::WireDragging { pos, } => pos.write(w),
      InputAction::WriteToConsole { value, } => w.write_string(value),



      InputAction::BuildItem { pos, dir, created_by_moving, size, ghost_mode, skip_fog_of_war, } => {
        pos.write(w)?;
        dir.write(w)?;
        w.write_bool(*created_by_moving)?;
        w.write_u8(*size)?;
        w.write_bool(*ghost_mode)?;
        w.write_bool(*skip_fog_of_war)
      },
      InputAction::FastEntityTransfer { dir, } => w.write_u8(dir.to_u8().unwrap()),
      InputAction::FastEntitySplit { dir, } => w.write_u8(dir.to_u8().unwrap()),
      InputAction::PlayerJoinGame { player_id, name, } => {
        w.write_opt_u16(*player_id)?;
        w.write_u16(0)?;
        w.write_u8(1)?; // AllowedCommands
        w.write_string(name)?;
        w.write_u8(0)?;
        w.write_u8(1)
      },
      &InputAction::QuickBarPickSlot { slot, } => {
        w.write_u16(slot)?;
        w.write_u8(0)?;
        w.write_u8(0)
      },
      InputAction::QuickBarSetSlot { slot,  source_slot, } => {
        w.write_u16(*slot)?;
       source_slot.write(w)
      },
      &InputAction::SelectedEntityChangedRelative { x, y, } => {
        w.write_i16(y)?;
        w.write_i16(x)
      },
      &InputAction::SelectedEntityChangedVeryClose { x, y, } => {
        assert!(x >= -8 && x < 8 && y >= -8 && y < 8);
        let xy = (((x + 8) as u8) << 4) | ((y + 8) as u8);
        w.write_u8(xy)
      },
      &InputAction::SelectedEntityChangedVeryClosePrecise { x, y, } => {
        w.write_i8(y)?;
        w.write_i8(x)
      },
    }
  }
  pub fn action_type(&self) -> InputActionType {
    match self {
      InputAction::ActivateCopy => InputActionType::ActivateCopy,
      InputAction::ActivateCut => InputActionType::ActivateCut,
      InputAction::ActivatePaste => InputActionType::ActivatePaste,
      InputAction::AddTrainStation { .. } => InputActionType::AddTrainStation,
      InputAction::AlternativeCopy { .. } => InputActionType::AlternativeCopy,
      InputAction::AltSelectBlueprintEntities { .. } => InputActionType::AltSelectBlueprintEntities,
      InputAction::BeginMining => InputActionType::BeginMining,
      InputAction::BeginMiningTerrain { .. } => InputActionType::BeginMiningTerrain,
      InputAction::BuildItem { .. } => InputActionType::BuildItem,
      InputAction::CancelCraft { .. } => InputActionType::CancelCraft,
      InputAction::CancelDropBlueprintRecord => InputActionType::CancelDropBlueprintRecord,
      InputAction::CancelNewBlueprint => InputActionType::CancelNewBlueprint,
      InputAction::ChangeActiveItemGroupForCrafting { .. } => InputActionType::ChangeActiveItemGroupForCrafting,
      InputAction::ChangeActiveItemGroupForFilters { .. } => InputActionType::ChangeActiveItemGroupForFilters,
      InputAction::ChangeRidingState { .. } => InputActionType::ChangeRidingState,
      InputAction::ChangeShootingState { .. } => InputActionType::ChangeShootingState,
      InputAction::ChangeTrainStopStation { .. } => InputActionType::ChangeTrainStopStation,
      InputAction::CheckCRC { .. } => InputActionType::CheckCRC,
      InputAction::CheckCRCHeuristic { .. } => InputActionType::CheckCRCHeuristic,
      InputAction::CleanCursorStack => InputActionType::CleanCursorStack,
      InputAction::CloseBlueprintRecord => InputActionType::CloseBlueprintRecord,
      InputAction::CloseGui => InputActionType::CloseGui,
      InputAction::ConnectRollingStock => InputActionType::ConnectRollingStock,
      InputAction::Copy { .. } => InputActionType::Copy,
      InputAction::CopyEntitySettings => InputActionType::CopyEntitySettings,
      InputAction::Craft { .. } => InputActionType::Craft,
      InputAction::CursorSplit { .. } => InputActionType::CursorSplit,
      InputAction::CursorTransfer { .. } => InputActionType::CursorTransfer,
      InputAction::CycleBlueprintBookBackwards => InputActionType::CycleBlueprintBookBackwards,
      InputAction::CycleBlueprintBookForwards => InputActionType::CycleBlueprintBookForwards,
      InputAction::CycleClipboardBackwards => InputActionType::CycleClipboardBackwards,
      InputAction::CycleClipboardForwards => InputActionType::CycleClipboardForwards,
      InputAction::Deconstruct { .. } => InputActionType::Deconstruct,
      InputAction::DeleteBlueprintLibrary => InputActionType::DeleteBlueprintLibrary,
      InputAction::DestroyOpenedItem => InputActionType::DestroyOpenedItem,
      InputAction::DisconnectRollingStock => InputActionType::DisconnectRollingStock,
      InputAction::DropItem { .. } => InputActionType::DropItem,
      InputAction::GameCreatedFromScenario => InputActionType::GameCreatedFromScenario,
      InputAction::GuiCheckedStateChanged { .. } => InputActionType::GuiCheckedStateChanged,
      InputAction::GuiClick { .. } => InputActionType::GuiClick,
      InputAction::GuiConfirmed { .. } => InputActionType::GuiConfirmed,
      InputAction::GuiLocationChanged { .. } => InputActionType::GuiLocationChanged,
      InputAction::GuiSelectedTabChanged { .. } => InputActionType::GuiSelectedTabChanged,
      InputAction::GuiSelectionStateChanged { .. } => InputActionType::GuiSelectionStateChanged,
      InputAction::GuiSwitchStateChanged { .. } => InputActionType::GuiSwitchStateChanged,
      InputAction::GuiTextChanged { .. } => InputActionType::GuiTextChanged,
      InputAction::GuiValueChanged { .. } => InputActionType::GuiValueChanged,
      InputAction::InventorySplit { .. } => InputActionType::InventorySplit,
      InputAction::InventoryTransfer { .. } => InputActionType::InventoryTransfer,
      InputAction::LaunchRocket => InputActionType::LaunchRocket,
      InputAction::MarketOffer { .. } => InputActionType::MarketOffer,
      InputAction::MoveOnZoom { .. } => InputActionType::MoveOnZoom,
      InputAction::MultiplayerInit => InputActionType::MultiplayerInit,
      InputAction::Nothing => InputActionType::Nothing,
      InputAction::OpenAchievementsGui => InputActionType::OpenAchievementsGui,
      InputAction::OpenBlueprintLibraryGui => InputActionType::OpenBlueprintLibraryGui,
      InputAction::OpenBonusGui => InputActionType::OpenBonusGui,
      InputAction::OpenCharacterGui => InputActionType::OpenCharacterGui,
      InputAction::OpenEquipment { .. } => InputActionType::OpenEquipment,
      InputAction::OpenGui => InputActionType::OpenGui,
      InputAction::OpenItem { .. } => InputActionType::OpenItem,
      InputAction::OpenKillsGui => InputActionType::OpenKillsGui,
      InputAction::OpenLogisticGui => InputActionType::OpenLogisticGui,
      InputAction::OpenModItem { .. } => InputActionType::OpenModItem,
      InputAction::OpenProductionGui => InputActionType::OpenProductionGui,
      InputAction::OpenTechnologyGui => InputActionType::OpenTechnologyGui,
      InputAction::OpenTrainsGui => InputActionType::OpenTrainsGui,
      InputAction::OpenTutorialsGui => InputActionType::OpenTutorialsGui,
      InputAction::PasteEntitySettings => InputActionType::PasteEntitySettings,
      InputAction::PlaceEquipment { .. } => InputActionType::PlaceEquipment,
      InputAction::ResetAssemblingMachine => InputActionType::ResetAssemblingMachine,
      InputAction::SelectBlueprintEntities { .. } => InputActionType::SelectBlueprintEntities,
      InputAction::SelectedEntityChanged { .. } => InputActionType::SelectedEntityChanged,
      InputAction::SelectedEntityCleared => InputActionType::SelectedEntityCleared,
      InputAction::SelectNextValidGun => InputActionType::SelectNextValidGun,
      InputAction::SetCircuitCondition { .. } => InputActionType::SetCircuitCondition,
      InputAction::SetCircuitModeOfOperation { .. } => InputActionType::SetCircuitModeOfOperation,
      InputAction::SetFilter { .. } => InputActionType::SetFilter,
      InputAction::SetInventoryBar { .. } => InputActionType::SetInventoryBar,
      InputAction::SetLogisticFilterItem { .. } => InputActionType::SetLogisticFilterItem,
      InputAction::SetLogisticFilterSignal { .. } => InputActionType::SetLogisticFilterSignal,
      InputAction::SetSignal { .. } => InputActionType::SetSignal,
      InputAction::SetupAssemblingMachine { .. } => InputActionType::SetupAssemblingMachine,
      InputAction::SingleplayerInit => InputActionType::SingleplayerInit,
      InputAction::SmartPipette { .. } => InputActionType::SmartPipette,
      InputAction::StackSplit { .. } => InputActionType::StackSplit,
      InputAction::StackTransfer { .. } => InputActionType::StackTransfer,
      InputAction::StartRepair { .. } => InputActionType::StartRepair,
      InputAction::StartResearch { .. } => InputActionType::StartResearch,
      InputAction::StartWalking { .. } => InputActionType::StartWalking,
      InputAction::StopBuildingByMoving => InputActionType::StopBuildingByMoving,
      InputAction::StopMining => InputActionType::StopMining,
      InputAction::StopMovementInTheNextTick => InputActionType::StopMovementInTheNextTick,
      InputAction::StopRepair => InputActionType::StopRepair,
      InputAction::StopWalking => InputActionType::StopWalking,
      InputAction::SwitchToRenameStopGui => InputActionType::SwitchToRenameStopGui,
      InputAction::TakeEquipment { .. } => InputActionType::TakeEquipment,
      InputAction::ToggleDeconstructionItemEntityFilterMode => InputActionType::ToggleDeconstructionItemEntityFilterMode,
      InputAction::ToggleDeconstructionItemTileFilterMode => InputActionType::ToggleDeconstructionItemTileFilterMode,
      InputAction::ToggleDriving => InputActionType::ToggleDriving,
      InputAction::ToggleEnableVehicleLogisticsWhileMoving => InputActionType::ToggleEnableVehicleLogisticsWhileMoving,
      InputAction::ToggleEquipmentMovementBonus => InputActionType::ToggleEquipmentMovementBonus,
      InputAction::ToggleMapEditor => InputActionType::ToggleMapEditor,
      InputAction::TogglePersonalRoboport => InputActionType::TogglePersonalRoboport,
      InputAction::ToggleShowEntityInfo => InputActionType::ToggleShowEntityInfo,
      InputAction::Undo => InputActionType::Undo,
      InputAction::Upgrade { .. } => InputActionType::Upgrade,
      InputAction::UpgradeOpenedBlueprint => InputActionType::UpgradeOpenedBlueprint,
      InputAction::UseArtilleryRemote { .. } => InputActionType::UseArtilleryRemote,
      InputAction::UseItem { .. } => InputActionType::UseItem,
      InputAction::WireDragging { .. } => InputActionType::WireDragging,
      InputAction::WriteToConsole { .. } => InputActionType::WriteToConsole,



      InputAction::FastEntitySplit { .. } => InputActionType::FastEntitySplit,
      InputAction::FastEntityTransfer { .. } => InputActionType::FastEntityTransfer,
      InputAction::PlayerJoinGame { .. } => InputActionType::PlayerJoinGame,
      InputAction::QuickBarPickSlot { .. } => InputActionType::QuickBarPickSlot,
      InputAction::QuickBarSetSlot { .. } => InputActionType::QuickBarSetSlot,
      InputAction::SelectedEntityChangedRelative { .. } => InputActionType::SelectedEntityChangedRelative,
      InputAction::SelectedEntityChangedVeryClose { .. } => InputActionType::SelectedEntityChangedVeryClose,
      InputAction::SelectedEntityChangedVeryClosePrecise { .. } => InputActionType::SelectedEntityChangedVeryClosePrecise,
    }
  }
}
