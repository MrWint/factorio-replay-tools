use crate::constants::*;
use num_traits::{FromPrimitive, ToPrimitive};
use std::io::{BufRead, Seek, Write};
use factorio_serialize::{Error, ReadWrite, ReadWriteStruct, Reader, Result, Writer};

#[derive(Debug, ReadWriteStruct)]
pub struct EquipmentData {
  pos: EquipmentPosition,
  typ: EquipmentDataType,
}

#[derive(Debug, ReadWriteStruct)]
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
type FixedPoint32 = i32;
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct MapPosition { // in 1/256th tiles
  pub x: FixedPoint32,
  pub y: FixedPoint32,
}
impl MapPosition {
  pub fn new(x: FixedPoint32, y: FixedPoint32) -> Self {
    Self { x, y }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, ReadWriteStruct)]
pub struct BoundingBox {
  pub left_top: MapPosition,
  pub right_bottom: MapPosition,
  pub orientation: f32, // always in [0,1)
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct EquipmentPosition {
  pub x: i32,
  pub y: i32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct TilePosition {
  pub x: i32,
  pub y: i32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct CrcData {
  pub crc: u32,
  pub tick_of_crc: u32,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct GuiChangedData {
  pub gui_element_index: u32,
  pub button: MouseButton,
  pub is_alt: bool,
  pub is_control: bool,
  pub is_shift: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct SetupBlueprintData {
  pub include_modules: bool,
  pub include_entities: bool,
  pub include_tiles: bool,
  pub include_station_names: bool,
  pub include_trains: bool,
  pub excluded_entities: Vec<u32>,
  pub excluded_tiles: Vec<u32>,
  pub excluded_items: Vec<Item>,
  pub icons: Vec<SignalId>,
}


#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct BuildItemParameters {
  pub position: MapPosition,
  pub direction: Direction,
  #[negated_bool] pub created_by_moving: bool,
  #[negated_bool] pub allow_belt_power_replace: bool,
  pub shift_build: bool,
  pub skip_fog_of_war: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct RidingState {
  pub direction: RidingDirection,
  pub acceleration_state: RidingAccelerationState,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct CraftData {
  pub recipe: Recipe,
  pub count: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct ShootingState {
  pub state: ShootingStateState,
  pub target: MapPosition,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct SmartPipetteData {
  pub entity: Entity,
  pub tile: Tile,
  pub pick_ghost_cursor: bool,
}

#[derive(Debug)]
pub enum InputAction {
  ActivateCopy,
  ActivateCut,
  ActivatePaste,
  AddTrainStation { name: String, pos: MapPosition, temporary: bool, },
  AlternativeCopy { area: SelectAreaData },
  AltSelectBlueprintEntities { area: SelectAreaData },
  BeginMining,
  BeginMiningTerrain(MapPosition),
  BuildItem(BuildItemParameters),
  CancelCraft { crafting_index: u16, count: u16, },
  CancelDropBlueprintRecord,
  CancelNewBlueprint,
  ChangeActiveItemGroupForCrafting { item_group: ItemGroup, },
  ChangeActiveItemGroupForFilters { item_group: ItemGroup, },
  ChangeRidingState(RidingState),
  ChangeShootingState(ShootingState),
  ChangeTrainStopStation { new_name: String, },
  CheckCRC(CrcData),
  CheckCRCHeuristic(CrcData),
  CleanCursorStack,
  CloseBlueprintRecord,
  CloseGui,
  ConnectRollingStock,
  Copy { area: SelectAreaData },
  CopyEntitySettings,
  Craft(CraftData),
  CursorSplit(Slot),
  CursorTransfer(Slot),
  CycleBlueprintBookBackwards,
  CycleBlueprintBookForwards,
  CycleClipboardBackwards,
  CycleClipboardForwards,
  Deconstruct { area: SelectAreaData },
  DeleteBlueprintLibrary,
  DestroyOpenedItem,
  DisconnectRollingStock,
  DropItem(MapPosition),
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
  InventorySplit(Slot),
  InventoryTransfer(Slot),
  LaunchRocket,
  MarketOffer { slot_index: u32, count: u32, },
  MoveOnZoom { x: f64, y: f64, },
  MultiplayerInit,
  Nothing,
  OpenAchievementsGui,
  OpenBlueprintLibraryGui,
  OpenBonusGui,
  OpenCharacterGui,
  OpenEquipment(EquipmentData),
  OpenGui,
  OpenItem(Slot),
  OpenKillsGui,
  OpenLogisticGui,
  OpenModItem(Slot),
  OpenProductionGui,
  OpenTechnologyGui,
  OpenTrainsGui,
  OpenTutorialsGui,
  PasteEntitySettings,
  PlaceEquipment(EquipmentData),
  ResetAssemblingMachine,
  SelectBlueprintEntities { area: SelectAreaData },
  SelectedEntityChanged(MapPosition),
  SelectedEntityCleared,
  SelectNextValidGun,
  SetCircuitCondition { circuit_index: u8, comparison: Comparison, first_signal_id: SignalId, second_signal_id: SignalIdOrConstant, },
  SetCircuitModeOfOperation { mode_of_operation: u8, enabled: bool, },
  SetFilter { slot: Slot, item: Item, },
  SetInventoryBar(Slot),
  SetLogisticFilterItem { item: Item, filter_index: u16, count: u32, },
  SetLogisticFilterSignal { signal: SignalId, filter_index: u16, count: u32, },
  SetSignal { signal_id: SignalId, signal_index: u16, },
  SetupAssemblingMachine(Recipe),
  SingleplayerInit,
  SmartPipette(SmartPipetteData),
  StackSplit(Slot),
  StackTransfer(Slot),
  StartRepair { pos: MapPosition, },
  StartResearch { technology: Technology },
  StartWalking(Direction),
  StopBuildingByMoving,
  StopMining,
  StopMovementInTheNextTick,
  StopRepair,
  StopWalking,
  SwitchToRenameStopGui,
  TakeEquipment(EquipmentData),
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
  WireDragging(MapPosition),
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
      InputActionType::BeginMiningTerrain => Ok(InputAction::BeginMiningTerrain(MapPosition::read(r)?)),
      InputActionType::BuildItem => Ok(InputAction::BuildItem(BuildItemParameters::read(r)?)),
      InputActionType::CancelCraft => {
        let crafting_index = r.read_u16()?;
        let count = r.read_u16()?;
        Ok(InputAction::CancelCraft { crafting_index, count, })
      },
      InputActionType::CancelDropBlueprintRecord => Ok(InputAction::CancelDropBlueprintRecord),
      InputActionType::CancelNewBlueprint => Ok(InputAction::CancelNewBlueprint),
      InputActionType::ChangeActiveItemGroupForCrafting => Ok(InputAction::ChangeActiveItemGroupForCrafting { item_group: ItemGroup::read(r)?, }),
      InputActionType::ChangeActiveItemGroupForFilters => Ok(InputAction::ChangeActiveItemGroupForFilters { item_group: ItemGroup::read(r)?, }),
      InputActionType::ChangeRidingState => Ok(InputAction::ChangeRidingState(RidingState::read(r)?)),
      InputActionType::ChangeShootingState => Ok(InputAction::ChangeShootingState(ShootingState::read(r)?)),
      InputActionType::ChangeTrainStopStation => Ok(InputAction::ChangeTrainStopStation { new_name: r.read_string()?, }),
      InputActionType::CheckCRC => Ok(InputAction::CheckCRC(CrcData::read(r)?)),
      InputActionType::CheckCRCHeuristic => Ok(InputAction::CheckCRCHeuristic(CrcData::read(r)?)),
      InputActionType::CleanCursorStack => Ok(InputAction::CleanCursorStack),
      InputActionType::CloseBlueprintRecord => Ok(InputAction::CloseBlueprintRecord),
      InputActionType::CloseGui => Ok(InputAction::CloseGui),
      InputActionType::ConnectRollingStock => Ok(InputAction::ConnectRollingStock),
      InputActionType::Copy => Ok(InputAction::Copy { area: SelectAreaData::read(r)? }),
      InputActionType::CopyEntitySettings => Ok(InputAction::CopyEntitySettings),
      InputActionType::Craft => Ok(InputAction::Craft(CraftData::read(r)?)),
      InputActionType::CursorSplit => Ok(InputAction::CursorSplit(Slot::read(r)?)),
      InputActionType::CursorTransfer => Ok(InputAction::CursorTransfer(Slot::read(r)?)),
      InputActionType::CycleBlueprintBookBackwards => Ok(InputAction::CycleBlueprintBookBackwards),
      InputActionType::CycleBlueprintBookForwards => Ok(InputAction::CycleBlueprintBookForwards),
      InputActionType::CycleClipboardBackwards => Ok(InputAction::CycleClipboardBackwards),
      InputActionType::CycleClipboardForwards => Ok(InputAction::CycleClipboardForwards),
      InputActionType::Deconstruct => Ok(InputAction::Deconstruct { area: SelectAreaData::read(r)? }),
      InputActionType::DeleteBlueprintLibrary => Ok(InputAction::DeleteBlueprintLibrary),
      InputActionType::DestroyOpenedItem => Ok(InputAction::DestroyOpenedItem),
      InputActionType::DisconnectRollingStock => Ok(InputAction::DisconnectRollingStock),
      InputActionType::DropItem => Ok(InputAction::DropItem(MapPosition::read(r)?)),
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
      InputActionType::InventorySplit => Ok(InputAction::InventorySplit(Slot::read(r)?)),
      InputActionType::InventoryTransfer => Ok(InputAction::InventoryTransfer(Slot::read(r)?)),
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
      InputActionType::OpenEquipment => Ok(InputAction::OpenEquipment(EquipmentData::read(r)?)),
      InputActionType::OpenGui => Ok(InputAction::OpenGui),
      InputActionType::OpenItem => Ok(InputAction::OpenItem(Slot::read(r)?)),
      InputActionType::OpenKillsGui => Ok(InputAction::OpenKillsGui),
      InputActionType::OpenLogisticGui => Ok(InputAction::OpenLogisticGui),
      InputActionType::OpenModItem => Ok(InputAction::OpenModItem(Slot::read(r)?)),
      InputActionType::OpenProductionGui => Ok(InputAction::OpenProductionGui),
      InputActionType::OpenTechnologyGui => Ok(InputAction::OpenTechnologyGui),
      InputActionType::OpenTrainsGui => Ok(InputAction::OpenTrainsGui),
      InputActionType::OpenTutorialsGui => Ok(InputAction::OpenTutorialsGui),
      InputActionType::PasteEntitySettings => Ok(InputAction::PasteEntitySettings),
      InputActionType::PlaceEquipment => Ok(InputAction::PlaceEquipment(EquipmentData::read(r)?)),
      InputActionType::ResetAssemblingMachine => Ok(InputAction::ResetAssemblingMachine),
      InputActionType::SelectBlueprintEntities => Ok(InputAction::SelectBlueprintEntities { area: SelectAreaData::read(r)? }),
      InputActionType::SelectedEntityChanged => Ok(InputAction::SelectedEntityChanged(MapPosition::read(r)?)),
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
      InputActionType::SetInventoryBar => Ok(InputAction::SetInventoryBar(Slot::read(r)?)),
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
      InputActionType::SetupAssemblingMachine => Ok(InputAction::SetupAssemblingMachine(Recipe::read(r)?)),
      InputActionType::SingleplayerInit => Ok(InputAction::SingleplayerInit),
      InputActionType::SmartPipette => Ok(InputAction::SmartPipette(SmartPipetteData::read(r)?)),
      InputActionType::StackSplit => Ok(InputAction::StackSplit(Slot::read(r)?)),
      InputActionType::StackTransfer => Ok(InputAction::StackTransfer(Slot::read(r)?)),
      InputActionType::StartRepair => Ok(InputAction::StartRepair { pos: MapPosition::read(r)?, }),
      InputActionType::StartResearch => Ok(InputAction::StartResearch { technology: Technology::read(r)?, }),
      InputActionType::StartWalking => Ok(InputAction::StartWalking(Direction::read(r)?)),
      InputActionType::StopBuildingByMoving => Ok(InputAction::StopBuildingByMoving),
      InputActionType::StopMining => Ok(InputAction::StopMining),
      InputActionType::StopMovementInTheNextTick => Ok(InputAction::StopMovementInTheNextTick),
      InputActionType::StopRepair => Ok(InputAction::StopRepair),
      InputActionType::StopWalking => Ok(InputAction::StopWalking),
      InputActionType::SwitchToRenameStopGui => Ok(InputAction::SwitchToRenameStopGui),
      InputActionType::TakeEquipment => Ok(InputAction::TakeEquipment(EquipmentData::read(r)?)),
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
      InputActionType::WireDragging => Ok(InputAction::WireDragging(MapPosition::read(r)?)),
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
      InputAction::BeginMiningTerrain(pos) => pos.write(w),
      InputAction::BuildItem(params) => params.write(w),
      InputAction::CancelCraft { crafting_index, count, } => {
        w.write_u16(*crafting_index)?;
        w.write_u16(*count)
      },
      InputAction::CancelDropBlueprintRecord => Ok(()),
      InputAction::CancelNewBlueprint => Ok(()),
      InputAction::ChangeActiveItemGroupForCrafting { item_group, } => item_group.write(w),
      InputAction::ChangeActiveItemGroupForFilters { item_group, } => item_group.write(w),
      InputAction::ChangeRidingState(riding_state) => riding_state.write(w),
      InputAction::ChangeShootingState(state) => state.write(w),
      InputAction::ChangeTrainStopStation { new_name, } => w.write_string(new_name),
      InputAction::CheckCRC(crc) => crc.write(w),
      InputAction::CheckCRCHeuristic(crc) => crc.write(w),
      InputAction::CleanCursorStack => Ok(()),
      InputAction::CloseBlueprintRecord => Ok(()),
      InputAction::CloseGui => Ok(()),
      InputAction::ConnectRollingStock => Ok(()),
      InputAction::Copy { area } => area.write(w),
      InputAction::CopyEntitySettings => Ok(()),
      InputAction::Craft(data) => data.write(w),
      InputAction::CursorSplit(slot) => slot.write(w),
      InputAction::CursorTransfer(slot) => slot.write(w),
      InputAction::CycleBlueprintBookBackwards => Ok(()),
      InputAction::CycleBlueprintBookForwards => Ok(()),
      InputAction::CycleClipboardBackwards => Ok(()),
      InputAction::CycleClipboardForwards => Ok(()),
      InputAction::Deconstruct { area } => area.write(w),
      InputAction::DeleteBlueprintLibrary => Ok(()),
      InputAction::DestroyOpenedItem => Ok(()),
      InputAction::DisconnectRollingStock => Ok(()),
      InputAction::DropItem(pos) => pos.write(w),
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
      InputAction::InventorySplit(slot) => slot.write(w),
      InputAction::InventoryTransfer(slot) => slot.write(w),
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
      InputAction::OpenEquipment(equipment) => equipment.write(w),
      InputAction::OpenGui => Ok(()),
      InputAction::OpenItem(slot) => slot.write(w),
      InputAction::OpenKillsGui => Ok(()),
      InputAction::OpenLogisticGui => Ok(()),
      InputAction::OpenModItem(slot) => slot.write(w),
      InputAction::OpenProductionGui => Ok(()),
      InputAction::OpenTechnologyGui => Ok(()),
      InputAction::OpenTrainsGui => Ok(()),
      InputAction::OpenTutorialsGui => Ok(()),
      InputAction::PasteEntitySettings => Ok(()),
      InputAction::PlaceEquipment(equipment) => equipment.write(w),
      InputAction::ResetAssemblingMachine => Ok(()),
      InputAction::SelectBlueprintEntities { area } => area.write(w),
      InputAction::SelectedEntityChanged(pos) => pos.write(w),
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
      InputAction::SetInventoryBar(slot) => slot.write(w),
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
      InputAction::SetupAssemblingMachine(recipe) => recipe.write(w),
      InputAction::SingleplayerInit => Ok(()),
      InputAction::SmartPipette(data) => data.write(w),
      InputAction::StackSplit(slot) => slot.write(w),
      InputAction::StackTransfer(slot) => slot.write(w),
      InputAction::StartRepair { pos, } => pos.write(w),
      InputAction::StartResearch { technology, } => technology.write(w),
      InputAction::StartWalking(direction) => direction.write(w),
      InputAction::StopBuildingByMoving => Ok(()),
      InputAction::StopMining => Ok(()),
      InputAction::StopMovementInTheNextTick => Ok(()),
      InputAction::StopRepair => Ok(()),
      InputAction::StopWalking => Ok(()),
      InputAction::SwitchToRenameStopGui => Ok(()),
      InputAction::TakeEquipment(equipment) => equipment.write(w),
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
      InputAction::WireDragging(pos) => pos.write(w),
      InputAction::WriteToConsole { value, } => w.write_string(value),



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
