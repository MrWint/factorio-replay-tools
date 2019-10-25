use crate::constants::*;
use std::io::{BufRead, Seek, Write};
use factorio_serialize::{ReadWrite, ReadWriteStruct, ReadWriteTaggedUnion, Reader, Result, Writer};

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

#[derive(Clone, Copy, Debug, PartialEq, ReadWriteStruct)]
pub struct SelectAreaData {
  pub bounding_box: BoundingBox,
  pub item: Item,
  pub skip_fog_of_war: bool,
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

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct CancelCraftOrder {
  pub crafting_index: u16,
  pub count: u16,
}

#[derive(Debug, ReadWriteStruct)]
pub struct SetFilterParameters {
  pub target: Slot,
  pub filter: Item,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct CircuitCondition {
  pub comparator: Comparison,
  pub first_signal: SignalId,
  pub second_signal: SignalIdOrConstant,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct CircuitConditionParameters {
  pub circuit_index: i8,
  pub condition: CircuitCondition,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct SignalData {
  pub signal_id: SignalId,
  pub signal_index: u16,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct LogisticFilterItemData {
  item: Item,
  filter_index: u16,
  count: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct LogisticFilterSignalData {
  signal: SignalId,
  filter_index: u16,
  count: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct BehaviorModeOfOperationParameters {
  mode_of_operation: u8,
  enabled: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct MarketOfferData {
  slot_index: u32,
  count: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct AddTrainStationData {
  name: String,
  rail_position: MapPosition,
  temporary: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct GuiGenericChangedData<T: ReadWrite> {
  gui_changed_data: GuiChangedData,
  value: T,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct GuiLocationChangedData {
  gui_changed_data: GuiChangedData,
  x: i32,
  y: i32,
}

#[derive(Debug, PartialEq, ReadWriteStruct)]
pub struct Vector {
  x: f64,
  y: f64,
}

#[derive(Debug, ReadWriteStruct)]
pub struct QuickBarSetSlotParameters {
  pub location: u16,
  pub item_to_use: Slot,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct QuickBarPickSlotParameters {
  pub location: u16,
  pub pick_ghost_cursor: bool,
  pub cursor_split: bool,
}

type FixedPoint16 = i16;
#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct SelectedEntityChangedRelativeData {
  y: FixedPoint16,
  x: FixedPoint16,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct SelectedEntityChangedVeryClosePreciseData {
  y: u8, // in 1/16 of a tile, starting from tileY(curpos) - 8
  x: u8, // in 1/16 of a tile, starting from tileX(curpos) - 8
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct SelectedEntityChangedVeryCloseData {
  xy: u8, // 2 4-bit numbers format 0xXXXXYYYY in full tiles, starting from tile(curpos) - 8
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct PlayerJoinGameData {
  #[space_optimized] pub peer_id: u16, // consecutive player ids
  pub player_index: u16,
  pub force_id: ForceId,
  pub username: String,
  pub as_editor: bool,
  pub admin: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct QuickBarSetSelectedPageParameters {
  main_window_row: u8, // top or bottom
  new_selected_page: u8, // 0-9
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct PixelPosition {
  x: i32,
  y: i32,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct UpdateBlueprintShelfData {
  shelf_player_index: u16,
  next_record_id: u32,
  timestamp: u32,
  records_to_remove: Vec<u32>,
  records_to_add: Vec<AddBlueprintRecordData>,
  records_to_update: Vec<UpdateBlueprintData>,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct AddBlueprintRecordData {
  id: u32,
  hash: Sha1Digest,
  item: Item,
  previews_in_book: Option<Vec<SingleRecordDataInBook>>,
  blueprint_icons: Vec<SignalId>,
  label: String,
  add_in_book: u32,
}
impl ReadWrite for AddBlueprintRecordData {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let id = u32::read(r)?;
    let hash = Sha1Digest::read(r)?;
    let item = Item::read(r)?;
    let is_book = bool::read(r)?;
    let blueprint_icons = <Vec<SignalId>>::read(r)?;
    let label = String::read(r)?;
    let add_in_book = u32::read(r)?;
    let previews_in_book = if is_book {
      Some(<Vec<SingleRecordDataInBook>>::read(r)?)
    } else { None };
    Ok(AddBlueprintRecordData { id, hash, item, previews_in_book, blueprint_icons, label, add_in_book })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    self.id.write(w)?;
    self.hash.write(w)?;
    self.item.write(w)?;
    self.previews_in_book.is_some().write(w)?;
    self.blueprint_icons.write(w)?;
    self.label.write(w)?;
    self.add_in_book.write(w)?;
    if let Some(previews_in_book) = &self.previews_in_book {
      previews_in_book.write(w)
    } else { Ok(()) }
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Sha1Digest([u8; 20]);
impl ReadWrite for Sha1Digest {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let mut sha1 = [0; 20];
    for i in 0..20 { sha1[i] = r.read_u8()? }
    Ok(Sha1Digest(sha1))
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    for i in 0..20 { w.write_u8(self.0[i])? }
    Ok(())
  }
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct SingleRecordDataInBook {
  id: u32,
  item: Item,
  hash: Sha1Digest,
  blueprint_icons: Vec<SignalId>,
  label: String,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct UpdateBlueprintData {
  id: u32,
  new_hash: Sha1Digest,
  new_label: String,
}


#[derive(Debug, ReadWriteTaggedUnion)]
#[tag_type(InputActionType)]
#[allow(dead_code)]
pub enum InputActionData {
  Nothing,
  StopWalking, // Equivalent to StartWalking(None)
  BeginMining,
  StopMining,
  ToggleDriving,
  OpenGui,
  CloseGui,
  OpenCharacterGui,
  ConnectRollingStock,
  DisconnectRollingStock,
  SelectedEntityCleared,
  CleanCursorStack,
  ResetAssemblingMachine,
  OpenTechnologyGui,
  LaunchRocket,
  OpenBlueprintLibraryGui,
  OpenProductionGui,
  OpenKillsGui,
  StopRepair,
  CancelNewBlueprint,
  CloseBlueprintRecord,
  CopyEntitySettings,
  PasteEntitySettings,
  DestroyOpenedItem,
  UpgradeOpenedBlueprint,
  ToggleShowEntityInfo,
  SingleplayerInit,
  MultiplayerInit,
  SwitchToRenameStopGui,
  OpenBonusGui,
  OpenTrainsGui,
  OpenAchievementsGui,
  OpenTutorialsGui,
  CycleBlueprintBookForwards,
  CycleBlueprintBookBackwards,
  CycleClipboardForwards,
  CycleClipboardBackwards,
  StopMovementInTheNextTick,
  ToggleEnableVehicleLogisticsWhileMoving,
  ToggleDeconstructionItemEntityFilterMode,
  ToggleDeconstructionItemTileFilterMode,
  OpenLogisticGui,
  CancelDropBlueprintRecord,
  SelectNextValidGun,
  ToggleMapEditor,
  DeleteBlueprintLibrary,
  GameCreatedFromScenario,
  ActivateCopy,
  ActivateCut,
  ActivatePaste,
  Undo,
  TogglePersonalRoboport,
  ToggleEquipmentMovementBonus,
  StopBuildingByMoving,
  DropItem(MapPosition),
  BuildItem(BuildItemParameters),
  StartWalking(Direction),
  BeginMiningTerrain(MapPosition),
  ChangeRidingState(RidingState),
  OpenItem(Slot),
  OpenModItem(Slot),
  OpenEquipment(EquipmentData),
  CursorTransfer(Slot),
  CursorSplit(Slot),
  StackTransfer(Slot),
  InventoryTransfer(Slot),
  CheckCRCHeuristic(CrcData),
  Craft(CraftData),
  WireDragging(MapPosition),
  ChangeShootingState(ShootingState),
  SetupAssemblingMachine(Recipe),
  SelectedEntityChanged(MapPosition),
  SmartPipette(SmartPipetteData),
  StackSplit(Slot),
  InventorySplit(Slot),
  CancelCraft(CancelCraftOrder),
  SetFilter(SetFilterParameters),
  CheckCRC(CrcData),
  SetCircuitCondition(CircuitConditionParameters),
  SetSignal(SignalData),
  StartResearch(Technology),
  SetLogisticFilterItem(LogisticFilterItemData),
  SetLogisticFilterSignal(LogisticFilterSignalData),
  SetCircuitModeOfOperation(BehaviorModeOfOperationParameters),
  GuiClick(GuiChangedData),
  GuiConfirmed(GuiChangedData),
  WriteToConsole(String),
  MarketOffer(MarketOfferData),
  AddTrainStation(AddTrainStationData),
  ChangeTrainStopStation(String),
  ChangeActiveItemGroupForCrafting(ItemGroup),
  GuiTextChanged(GuiGenericChangedData<String>),
  GuiCheckedStateChanged(GuiChangedData),
  GuiSelectionStateChanged(GuiGenericChangedData<i32>),
  GuiSelectedTabChanged(GuiGenericChangedData<i32>),
  GuiValueChanged(GuiGenericChangedData<f64>),
  GuiSwitchStateChanged(GuiGenericChangedData<SwitchState>),
  GuiLocationChanged(GuiLocationChangedData),
  PlaceEquipment(EquipmentData),
  TakeEquipment(EquipmentData),
  UseItem(MapPosition),
  UseArtilleryRemote(MapPosition),
  SetInventoryBar(Slot),
  ChangeActiveItemGroupForFilters(ItemGroup),
  MoveOnZoom(Vector),
  StartRepair(MapPosition),
  Deconstruct(SelectAreaData),
  Upgrade(SelectAreaData),
  Copy(SelectAreaData),
  AlternativeCopy(SelectAreaData),
  SelectBlueprintEntities(SelectAreaData),
  AltSelectBlueprintEntities(SelectAreaData),
  // SetupBlueprint,
  // SetupSingleBlueprintRecord,
  // SetSingleBlueprintRecordIcon,
  // OpenBlueprintRecord,
  // CloseBlueprintBook,
  // ChangeSingleBlueprintRecordLabel,
  // GrabBlueprintRecord,
  // DropBlueprintRecord,
  // DeleteBlueprintRecord,
  // CreateBlueprintLike,
  // CreateBlueprintLikeStackTransfer,
  UpdateBlueprintShelf(UpdateBlueprintShelfData),
  // TransferBlueprint,
  // TransferBlueprintImmediately,
  // ChangeBlueprintBookRecordLabel,
  // RemoveCables,
  // ExportBlueprint,
  // ImportBlueprint,
  PlayerJoinGame(PlayerJoinGameData),
  // CancelDeconstruct,
  // CancelUpgrade,
  // ChangeArithmeticCombinatorParameters,
  // ChangeDeciderCombinatorParameters,
  // ChangeProgrammableSpeakerParameters,
  // ChangeProgrammableSpeakerAlertParameters,
  // ChangeProgrammableSpeakerCircuitParameters,
  // BuildTerrain,
  // ChangeTrainWaitCondition,
  // ChangeTrainWaitConditionData,
  // CustomInput,
  // ChangeItemLabel,
  // BuildRail,
  // CancelResearch,
  // SelectArea,
  // AltSelectArea,
  // ServerCommand,
  // ClearSelectedBlueprint,
  // ClearSelectedDeconstructionItem,
  // ClearSelectedUpgradeItem,
  // SetLogisticTrashFilterItem,
  // SetInfinityContainerFilterItem,
  // SetInfinityPipeFilter,
  // ModSettingsChanged,
  // SetEntityEnergyProperty,
  // EditCustomTag,
  // EditPermissionGroup,
  // ImportBlueprintString,
  // ImportPermissionsString,
  // ReloadScript,
  // ReloadScriptDataTooLarge,
  // GuiElemChanged,
  // BlueprintTransferQueueUpdate,
  // DragTrainSchedule,
  // DragTrainWaitCondition,
  // SelectItem,
  // SelectEntitySlot,
  // SelectTileSlot,
  // SelectMapperSlot,
  DisplayResolutionChanged(PixelPosition),
  QuickBarSetSlot(QuickBarSetSlotParameters),
  QuickBarPickSlot(QuickBarPickSlotParameters),
  QuickBarSetSelectedPage(QuickBarSetSelectedPageParameters),
  // PlayerLeaveGame,
  // MapEditorAction,
  // PutSpecialItemInMap,
  // ChangeMultiplayerConfig,
  // AdminAction,
  // LuaShortcut,
  // TranslateString,
  ChangePickingState(u8),
  SelectedEntityChangedVeryClose(SelectedEntityChangedVeryCloseData),
  SelectedEntityChangedVeryClosePrecise(SelectedEntityChangedVeryClosePreciseData),
  SelectedEntityChangedRelative(SelectedEntityChangedRelativeData),
  SelectedEntityChangedBasedOnUnitNumber(u32),
  // SetAutosortInventory,
  // SetAutoLaunchRocket,
  // SwitchConstantCombinatorState,
  // SwitchPowerSwitchState,
  // SwitchInserterFilterModeState,
  // SwitchConnectToLogisticNetwork,
  // SetBehaviorMode,
  FastEntityTransfer(TransferDirection),
  // RotateEntity,
  FastEntitySplit(TransferDirection),
  // SetTrainStopped,
  // ChangeControllerSpeed,
  // SetAllowCommands,
  // SetResearchFinishedStopsGame,
  // SetInserterMaxStackSize,
  // OpenTrainGui,
  // SetEntityColor,
  // SetDeconstructionItemTreesAndRocksOnly,
  // SetDeconstructionItemTileSelectionMode,
  // DropToBlueprintBook,
  // DeleteCustomTag,
  // DeletePermissionGroup,
  // AddPermissionGroup,
  // SetInfinityContainerRemoveUnfilteredItems,
  // SetCarWeaponsControl,
  // SetRequestFromBuffers,
  // ChangeActiveQuickBar,
  // OpenPermissionsGui,
  DisplayScaleChanged(f64),
  // SetSplitterPriority,
  // GrabInternalBlueprintFromText,
  // SetHeatInterfaceTemperature,
  // SetHeatInterfaceMode,
  // OpenTrainStationGui,
  // RemoveTrainStation,
  // GoToTrainStation,
  // RenderModeChanged,
}

#[derive(Debug)]
pub struct InputAction {
  update_tick: u32,
  player_index: u16,
  action: InputActionData,
}
impl InputAction {
  pub fn new(update_tick: u32, player_index: u16, action: InputActionData) -> Self {
    Self { update_tick, player_index, action }
  }
}
impl ReadWrite for InputAction {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let action_type_pos = r.position();
    let action_type = InputActionType::read(r)?;
    let update_tick = r.read_u32()?;
    let player_index = r.read_opt_u16()?;
    let action = InputActionData::read(action_type, action_type_pos, r)?;
    assert!(action_type == action.to_tag(), "Action type {:?} does not match {:?}", action_type, action.to_tag());
    Ok(InputAction { update_tick, player_index, action })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    self.action.to_tag().write(w)?;
    w.write_u32(self.update_tick)?;
    w.write_opt_u16(self.player_index)?;
    self.action.write(w)
  }
}
