use std::fmt::Debug;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Seek;

use enum_primitive_derive::Primitive;
use factorio_serialize_derive::ReplayReadWriteEnumU32;
use factorio_serialize_derive::ReplayReadWriteEnumU8;
use factorio_serialize_derive::ReplayReadWriteStruct;
use factorio_serialize_derive::ReplayReadWriteTaggedUnion;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;

use crate::structs::BoundingBox;
use crate::structs::Vector;
use crate::MapPosition;
use crate::Reader;
use crate::Result;
use crate::TilePosition;
use crate::Writer;
use crate::constants::Achievement;
use crate::constants::Decorative;
use crate::constants::Entity;
use crate::constants::Equipment;
use crate::constants::Fluid;
use crate::constants::Item;
use crate::constants::ItemGroup;
use crate::constants::Recipe;
use crate::constants::Technology;
use crate::constants::Tile;
use crate::constants::VirtualSignal;



pub struct ReplayData {
  pub actions: Vec<InputAction>,
}
impl ReplayData {
  pub fn from_input_actions(actions: Vec<InputAction>) -> Self {
    Self { actions }
  }
  pub fn parse_replay_data(replay_data: &[u8]) -> Result<ReplayData> {
    let mut replay_deserialiser = ReplayDeserialiser::new(Cursor::new(replay_data))?;

    let mut actions = vec![];
    while !replay_deserialiser.stream.is_at_eof().unwrap() {
      actions.push(InputAction::replay_read(&mut replay_deserialiser).unwrap());
      println!("parsed {:?} end pos {}", actions.last().unwrap(), replay_deserialiser.stream.position())
    }

    Ok(ReplayData { actions })
  }

  pub fn write_replay_data(&self) -> Result<Vec<u8>> {
    let mut replay_serialiser = ReplaySerialiser::new()?;

    for input_action in &self.actions {
      input_action.replay_write(&mut replay_serialiser).unwrap();
    }
  
    Ok(replay_serialiser.stream.into_inner().into_inner())
  }
}
impl std::fmt::Debug for ReplayData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "actions: {:?}", self.actions)?;
    Ok(())
  }
}


pub struct ReplayDeserialiser<R: BufRead + Seek> {
  pub stream: Reader<R>,
}
impl<R: BufRead + Seek> ReplayDeserialiser<R> {
  pub fn new(replay_data: R) -> Result<ReplayDeserialiser<R>> {
    let stream = Reader::new(replay_data);

    Ok(ReplayDeserialiser {
      stream,
    })
  }
}
pub struct ReplaySerialiser {
  pub stream: Writer<Cursor<Vec<u8>>>,
}
impl ReplaySerialiser{
  fn new() -> Result<ReplaySerialiser> {
    let stream = Writer::new(Cursor::new(Vec::new()));

    Ok(ReplaySerialiser {
      stream,
    })
  }
}
pub fn replay_read_vec_opt_u16<R: BufRead + Seek, T: ReplayReadWrite>(input: &mut ReplayDeserialiser<R>) -> Result<Vec<T>> {
  let len = input.stream.read_opt_u16()?;
  (0..len).map(|_| T::replay_read(input)).collect()
}
pub fn replay_write_vec_opt_u16<T: ReplayReadWrite>(v: &[T], input: &mut ReplaySerialiser) -> Result<()> {
  input.stream.write_opt_u16(v.len() as u16)?;
  v.into_iter().map(|v| v.replay_write(input)).collect()
}
pub fn replay_read_vec_u8<R: BufRead + Seek, T: ReplayReadWrite>(input: &mut ReplayDeserialiser<R>) -> Result<Vec<T>> {
  let len = input.stream.read_u8()?;
  (0..len).map(|_| T::replay_read(input)).collect()
}
pub fn replay_write_vec_u8<T: ReplayReadWrite>(v: &[T], input: &mut ReplaySerialiser) -> Result<()> {
  input.stream.write_u8(v.len() as u8)?;
  v.into_iter().map(|v| v.replay_write(input)).collect()
}
pub fn replay_read_vec_u32<R: BufRead + Seek, T: ReplayReadWrite>(input: &mut ReplayDeserialiser<R>) -> Result<Vec<T>> {
  let len = input.stream.read_u32()?;
  (0..len).map(|_| T::replay_read(input)).collect()
}
pub fn replay_write_vec_u32<T: ReplayReadWrite>(v: &[T], input: &mut ReplaySerialiser) -> Result<()> {
  input.stream.write_u32(v.len() as u32)?;
  v.into_iter().map(|v| v.replay_write(input)).collect()
}

pub trait ReplayReadWrite: Sized {
  fn replay_read<R: BufRead + Seek>(input: &mut ReplayDeserialiser<R>) -> Result<Self>;
  fn replay_write(&self, input: &mut ReplaySerialiser) -> Result<()>;
}
impl ReplayReadWrite for bool {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> { r.stream.read_bool() }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> { w.stream.write_bool(*self) }
}
impl ReplayReadWrite for i16 {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> { r.stream.read_i16() }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> { w.stream.write_i16(*self) }
}
impl ReplayReadWrite for i32 {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> { r.stream.read_i32() }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> { w.stream.write_i32(*self) }
}
impl ReplayReadWrite for u8 {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> { r.stream.read_u8() }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> { w.stream.write_u8(*self) }
}
impl ReplayReadWrite for u16 {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> { r.stream.read_u16() }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> { w.stream.write_u16(*self) }
}
impl ReplayReadWrite for u32 {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> { r.stream.read_u32() }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> { w.stream.write_u32(*self) }
}
impl ReplayReadWrite for u64 {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> { r.stream.read_u64() }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> { w.stream.write_u64(*self) }
}
impl ReplayReadWrite for f32 {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> { r.stream.read_f32() }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> { w.stream.write_f32(*self) }
}
impl ReplayReadWrite for f64 {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> { r.stream.read_f64() }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> { w.stream.write_f64(*self) }
}
impl ReplayReadWrite for String {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> { r.stream.read_string() }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> { w.stream.write_string(self) }
}
impl<T: ReplayReadWrite> ReplayReadWrite for Vec<T> {
  fn replay_read<R: BufRead + Seek>(input: &mut ReplayDeserialiser<R>) -> Result<Self> {
    let len = input.stream.read_opt_u32()?;
    (0..len).map(|_| T::replay_read(input)).collect()
  }
  fn replay_write(&self, input: &mut ReplaySerialiser) -> Result<()> {
    input.stream.write_opt_u32(self.len() as u32)?;
    self.into_iter().map(|v| v.replay_write(input)).collect()
  }
}
impl<T: ReplayReadWrite + Debug, const N: usize> ReplayReadWrite for [T; N] {
  fn replay_read<R: BufRead + Seek>(input: &mut ReplayDeserialiser<R>) -> Result<Self> {
    Ok((0..N).map(|_| T::replay_read(input)).collect::<Result<Vec<_>>>()?.try_into().unwrap())
  }
  fn replay_write(&self, input: &mut ReplaySerialiser) -> Result<()> {
    self.into_iter().map(|v| v.replay_write(input)).collect()
  }
}
impl<T: ReplayReadWrite> ReplayReadWrite for Option<T> {
  fn replay_read<R: BufRead + Seek>(input: &mut ReplayDeserialiser<R>) -> Result<Self> {
    if input.stream.read_bool()? {
      Ok(Some(T::replay_read(input)?))
    } else {
      Ok(None)
    }
  }
  fn replay_write(&self, input: &mut ReplaySerialiser) -> Result<()> {
    match self {
      Some(value) => {
        input.stream.write_bool(true)?;
        value.replay_write(input)
      },
      None => input.stream.write_bool(false)
    }
  }
}
impl <A: ReplayReadWrite, B: ReplayReadWrite> ReplayReadWrite for (A, B) {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> {
    let a = A::replay_read(r)?;
    let b = B::replay_read(r)?;

    Ok((a, b))
  }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> {
    self.0.replay_write(w)?;
    self.1.replay_write(w)?;

    Ok(())
  }
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
impl ReplayReadWrite for InputAction {
  fn replay_read<R: BufRead + Seek>(input: &mut ReplayDeserialiser<R>) -> Result<Self> {
    let action_type_pos = input.stream.position();
    let action_type = InputActionType::replay_read(input)?;
    let update_tick = input.stream.read_u32()?;
    let player_index = input.stream.read_opt_u16()?;
    let action = InputActionData::replay_read(action_type, action_type_pos, input)?;
    assert!(action_type == action.to_tag(), "Action type {:?} does not match {:?}", action_type, action.to_tag());
    Ok(InputAction { update_tick, player_index , action })
  }
  fn replay_write(&self, input: &mut ReplaySerialiser) -> Result<()> {
    self.action.to_tag().replay_write(input)?;
    input.stream.write_u32(self.update_tick)?;
    input.stream.write_opt_u16(self.player_index)?;
    self.action.replay_write(input)?;

    Ok(())
  }
}


#[derive(Debug, ReplayReadWriteTaggedUnion)]
#[tag_type(InputActionType)]
pub enum InputActionData {
  Nothing,  // 0,
  StopWalking,  // 1,
  BeginMining,  // 2,
  StopMining,  // 3,
  ToggleDriving,  // 4,
  OpenGui,  // 5,
  CloseGui,  // 6,
  OpenCharacterGui,  // 7,
  OpenCurrentVehicleGui,  // 8,
  ConnectRollingStock,  // 9,
  DisconnectRollingStock,  // 10,
  SelectedEntityCleared,  // 11,
  ClearCursor,  // 12,
  ResetAssemblingMachine,  // 13,
  OpenTechnologyGui,  // 14,
  LaunchRocket,  // 15,
  OpenProductionGui,  // 16,
  StopRepair,  // 17,
  CancelNewBlueprint,  // 18,
  CloseBlueprintRecord,  // 19,
  CopyEntitySettings,  // 20,
  PasteEntitySettings,  // 21,
  DestroyOpenedItem,  // 22,
  CopyOpenedItem,  // 23,
  ToggleShowEntityInfo,  // 24,
  SingleplayerInit,  // 25,
  MultiplayerInit,  // 26,
  DisconnectAllPlayers,  // 27,
  SwitchToRenameStopGui,  // 28,
  OpenBonusGui,  // 29,
  OpenTrainsGui,  // 30,
  OpenAchievementsGui,  // 31,
  CycleBlueprintBookForwards,  // 32,
  CycleBlueprintBookBackwards,  // 33,
  CycleClipboardForwards,  // 34,
  CycleClipboardBackwards,  // 35,
  StopMovementInTheNextTick,  // 36,
  ToggleEnableVehicleLogisticsWhileMoving,  // 37,
  ToggleDeconstructionItemEntityFilterMode,  // 38,
  ToggleDeconstructionItemTileFilterMode,  // 39,
  OpenLogisticGui,  // 40,
  SelectNextValidGun,  // 41,
  ToggleMapEditor,  // 42,
  DeleteBlueprintLibrary,  // 43,
  GameCreatedFromScenario,  // 44,
  ActivateCopy,  // 45,
  ActivateCut,  // 46,
  ActivatePaste,  // 47,
  Undo,  // 48,
  TogglePersonalRoboport,  // 49,
  ToggleEquipmentMovementBonus,  // 50,
  TogglePersonalLogisticRequests,  // 51,
  ToggleEntityLogisticRequests,  // 52,
  StopBuildingByMoving,  // 53,
  FlushOpenedEntityFluid,  // 54,
  OpenTipsAndTricksGui(u32),  // 56
  OpenBlueprintLibraryGui(u16),  // 57
  ChangeBlueprintLibraryTab(u16),  // 58
  DropItem(MapPosition),  // 59
  Build(BuildParameters),  // 60
  StartWalking(Direction),  // 61
  BeginMiningTerrain(MapPosition),  // 62
  ChangeRidingState(RidingState),  // 63
  OpenItem(ItemStackTargetSpecification),  // 64
  OpenParentOfOpenedItem(u16),  // 65
  ResetItem(ItemStackTargetSpecification),  // 66
  DestroyItem(ItemStackTargetSpecification),  // 67
  OpenModItem(ItemStackTargetSpecification),  // 68
  OpenEquipment(EquipmentData),  // 69
  CursorTransfer(ItemStackTargetSpecification),  // 70
  CursorSplit(ItemStackTargetSpecification),  // 71
  StackTransfer(ItemStackTargetSpecification),  // 72
  InventoryTransfer(ItemStackTargetSpecification),  // 73
  CheckCRCHeuristic(CrcData),  // 74
  Craft(CraftData),  // 75
  WireDragging(MapPosition),  // 76
  ChangeShootingState(ShootingState),  // 77
  SetupAssemblingMachine(Recipe),  // 78
  SelectedEntityChanged(MapPosition),  // 79
  SmartPipette(SmartPipetteData),  // 80
  StackSplit(ItemStackTargetSpecification),  // 81
  InventorySplit(ItemStackTargetSpecification),  // 82
  CancelCraft(CancelCraftOrder),  // 83
  SetFilter(SetFilterParameters),  // 84
  CheckCRC(CrcData),  // 85
  SetCircuitCondition(CircuitConditionParameters),  // 86
  SetSignal(SignalData),  // 87
  StartResearch(Technology),  // 88
  SetLogisticFilterItem(LogisticFilterItemData),  // 89
  SetLogisticFilterSignal(LogisticFilterSignalData),  // 90
  SetCircuitModeOfOperation(BehaviorModeOfOperationParameters),  // 91
  GuiClick(GuiChangedData),  // 92
  GuiConfirmed(GuiChangedData),  // 93
  WriteToConsole(String),  // 94
  MarketOffer(MarketOfferData),  // 95
  AddTrainStation(AddTrainStationData),  // 96
  ChangeTrainStopStation(String),  // 97
  ChangeActiveItemGroupForCrafting(ItemGroup),  // 98
  ChangeActiveItemGroupForFilters(ItemGroup),  // 99
  ChangeActiveCharacterTab(u8),  // 100
  GuiTextChanged(GuiGenericChangedData<String>),  // 101
  GuiCheckedStateChanged(GuiChangedData),  // 102
  GuiSelectionStateChanged(GuiGenericChangedData<i32>),  // 103
  GuiSelectedTabChanged(GuiGenericChangedData<i32>),  // 104
  GuiValueChanged(GuiGenericChangedData<f64>),  // 105
  GuiSwitchStateChanged(GuiGenericChangedData<u8>),  // 106
  GuiLocationChanged(GuiGenericChangedData<PixelPosition>),  // 107
  PlaceEquipment(EquipmentData),  // 108
  TakeEquipment(EquipmentData),  // 109
  UseItem(MapPosition),  // 110
  SendSpidertron(SendSpidertronParameters),  // 111
  UseArtilleryRemote(MapPosition),  // 112
  SetInventoryBar(ItemStackTargetSpecification),  // 113
  MoveOnZoom(Vector),  // 114
  StartRepair(MapPosition),  // 115
  Deconstruct(SelectAreaData),  // 116
  Upgrade((SelectAreaData, bool)),  // 117
  Copy(SelectAreaData),  // 118
  AlternativeCopy(SelectAreaData),  // 119
  SelectBlueprintEntities(SelectAreaData),  // 120
  AltSelectBlueprintEntities(SelectAreaData),  // 121
  SetupBlueprint(SetupBlueprintData),  // 122
  SetupSingleBlueprintRecord(SetupBlueprintData),  // 123
  CopyOpenedBlueprint(SetupBlueprintData),  // 124
  ReassignBlueprint(SetupBlueprintData),  // 125
  OpenBlueprintRecord(BlueprintRecordId),  // 126
  GrabBlueprintRecord(BlueprintRecordId),  // 127
  DropBlueprintRecord(BlueprintRecordLocation),  // 128
  DeleteBlueprintRecord(BlueprintRecordId),  // 129
  UpgradeOpenedBlueprintByRecord(UpgradeOpenedBlueprintByRecordParameters),  // 130
  UpgradeOpenedBlueprintByItem(UpgradeOpenedBlueprintByItemParameters),  // 131
  SpawnItem(Item),  // 132
  SpawnItemStackTransfer(Item),  // 133
  UpdateBlueprintShelf(UpdateBlueprintShelfData),  // 134
  TransferBlueprint(TransferBlueprintData),  // 135
  TransferBlueprintImmediately(TransferBlueprintData),  // 136
  EditBlueprintToolPreview(EditBlueprintToolPreviewData),  // 137
  RemoveCables(MapPosition),  // 138
  ExportBlueprint(BlueprintRecordLocation),  // 139
  ImportBlueprint(BlueprintRecordId),  // 140
  ImportBlueprintsFiltered(ImportBlueprintsFilteredParameters),  // 141
  PlayerJoinGame(PlayerJoinGameData),  // 142
  PlayerAdminChange(bool),  // 143
  CancelDeconstruct(SelectAreaData),  // 144
  CancelUpgrade(SelectAreaData),  // 145
  ChangeArithmeticCombinatorParameters(ArithmeticCombinatorParameters),  // 146
  ChangeDeciderCombinatorParameters(DeciderCombinatorParameters),  // 147
  ChangeProgrammableSpeakerParameters(ProgrammableSpeakerParameters),  // 148
  ChangeProgrammableSpeakerAlertParameters(ProgrammableSpeakerAlertParameters),  // 149
  ChangeProgrammableSpeakerCircuitParameters(ProgrammableSpeakerCircuitParameters),  // 150
  SetVehicleAutomaticTargetingParameters(VehicleAutomaticTargetingParameters),  // 151
  BuildTerrain(BuildTerrainParameters),  // 152
  ChangeTrainWaitCondition(TrainWaitCondition),  // 153
  ChangeTrainWaitConditionData(TrainWaitConditionData),  // 154
  CustomInput(CustomInputData),  // 155
  ChangeItemLabel(String),  // 156
  ChangeItemDescription(String),  // 157
  ChangeEntityLabel(String),  // 158
  BuildRail(BuildRailData),  // 159
  CancelResearch(TechnologyWithCount),  // 160
  SelectArea(SelectAreaData),  // 161
  AltSelectArea(SelectAreaData),  // 162
  ReverseSelectArea(SelectAreaData),  // 163
  AltReverseSelectArea(SelectAreaData),  // 164
  ServerCommand(ServerCommandData),  // 165
  SetControllerLogisticTrashFilterItem(LogisticFilterItemData), // 166
  SetEntityLogisticTrashFilterItem(LogisticFilterItemData),  // 167
  SetInfinityContainerFilterItem(InfinityContainerFilterItemData),  // 168
  SetInfinityPipeFilter(InfinityPipeFilterData),  // 169
  ModSettingsChanged(ModSettingsChangedData),  // 170
  SetEntityEnergyProperty(EntityEnergyPropertyChangedData),  // 171
  EditCustomTag(CustomChartTagData),  // 172
  EditPermissionGroup(EditPermissionGroupParameters),  // 173
  ImportBlueprintString(ImportBlueprintStringData),  // 174
  ImportPermissionsString(String),  // 175
  ReloadScript(String),  // 176
  ReloadScriptDataTooLarge(ScriptDataTooLarge),  // 177
  GuiElemChanged(GuiGenericChangedData<ChooseElemId>),  // 178
  BlueprintTransferQueueUpdate(BlueprintTransferQueueUpdateData),  // 179
  DragTrainSchedule(DragListBoxData),  // 180
  DragTrainWaitCondition(DragWaitConditionListBoxData),  // 181
  SelectItem(SelectSlotParameters<Item>),  // 182
  SelectEntitySlot(SelectSlotParameters<Entity>),  // 183
  SelectTileSlot(SelectSlotParameters<Tile>),  // 184
  SelectMapperSlot(SelectMapperSlotParameters),  // 185
  DisplayResolutionChanged(PixelPosition),  // 186
  QuickBarSetSlot(QuickBarSetSlotParameters),  // 187
  QuickBarPickSlot(QuickBarPickSlotParameters),  // 188
  QuickBarSetSelectedPage(QuickBarSetSelectedPageParameters),  // 189
  PlayerLeaveGame(DisconnectReason),  // 190
  // MapEditorAction, // has lots of sub-operations
  PutSpecialItemInMap(ItemStackTargetSpecification),  // 192
  PutSpecialRecordInMap(BlueprintRecordId),  // 193
  ChangeMultiplayerConfig(MultiplayerConfigSettings),  // 194
  AdminAction(AdminActionData),  // 195
  LuaShortcut(LuaShortcutData),  // 196
  TranslateString(Vec<TranslationResultDataEntry>),  // 197
  FlushOpenedEntitySpecificFluid(Fluid),  // 198
  ChangePickingState(bool),  // 199
  SelectedEntityChangedVeryClose(SelectedEntityChangedVeryCloseData),  // 200
  SelectedEntityChangedVeryClosePrecise(SelectedEntityChangedVeryClosePreciseData),  // 201
  SelectedEntityChangedRelative(SelectedEntityChangedRelativeData),  // 202
  SelectedEntityChangedBasedOnUnitNumber(u32),  // 203
  SetAutosortInventory(bool),  // 204
  SetFlatControllerGui(bool),  // 205
  SetRecipeNotifications(bool),  // 206
  SetAutoLaunchRocket(bool),  // 207
  SwitchConstantCombinatorState(bool),  // 208
  SwitchPowerSwitchState(bool),  // 209
  SwitchInserterFilterModeState(bool),  // 210
  SwitchConnectToLogisticNetwork(bool),  // 211
  SetBehaviorMode(u8),  // 212
  FastEntityTransfer(TransferDirection),  // 213
  RotateEntity(bool),  // 214
  FastEntitySplit(TransferDirection),  // 215
  SetTrainStopped(bool),  // 216
  ChangeControllerSpeed(f64),  // 217
  SetAllowCommands(AllowedCommands),  // 218
  SetResearchFinishedStopsGame(bool),  // 219
  SetInserterMaxStackSize(u8),  // 220
  OpenTrainGui(u32),  // 221
  SetEntityColor(ByteColor),  // 222
  SetDeconstructionItemTreesAndRocksOnly(bool),  // 223
  SetDeconstructionItemTileSelectionMode(TileSelectionMode),  // 224
  DeleteCustomTag(u32),  // 225
  DeletePermissionGroup(u32),  // 226
  AddPermissionGroup(u32),  // 227
  SetInfinityContainerRemoveUnfilteredItems(bool),  // 228
  SetCarWeaponsControl(bool),  // 229
  SetRequestFromBuffers(bool),  // 230
  ChangeActiveQuickBar(u8),  // 231
  OpenPermissionsGui(bool),  // 232
  DisplayScaleChanged(f64),  // 233
  SetSplitterPriority(SetSplitterPriorityData),  // 234
  GrabInternalBlueprintFromText(u32),  // 235
  SetHeatInterfaceTemperature(f64),  // 236
  SetHeatInterfaceMode(u8),  // 237
  OpenTrainStationGui(u32),  // 238
  RemoveTrainStation(u32),  // 239
  GoToTrainStation(u32),  // 240
  RenderModeChanged(GameRenderMode),  // 241
  PlayerInputMethodChanged(PlayerInputMethod),  // 242
  SetPlayerColor(ByteColor),  // 243
  PlayerClickedGpsTag(PingCoordinates),  // 244
  SetTrainsLimit(u32),  // 245
  ClearRecipeNotification(Recipe),  // 246
  SetLinkedContainerLinkID(u32),  // 247
  GuiHover(u32),  // 248
  GuiLeave(u32),  // 249
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct PingCoordinates {
  position: MapPosition,
  surface_name: String,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum PlayerInputMethod {
  KeyboardAndMouse = 0,
  GameController = 1,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ByteColor {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct TranslationResultDataEntry {
  localised_string: LocalisedString,
  result: String,
  translated: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct LocalisedString {
  key: String,
  mode: LocalisedStringMode,
  #[vec_u8] parameters: Vec<LocalisedString>,
}

// Source: disassembly LocalisedString::Mode
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum LocalisedStringMode {
  Empty = 0,
  Translation = 1,
  Literal = 2,
  LiteralTranslation = 3,
  FallbackGroup = 4,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct LuaShortcutData {
  player: u16,
  prototype_name: String,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct AdminActionData {
  player_index: u16,
  username: String,
  new_group_id: u32,
  action: AdminActionDataType,
}

// Source: disassembly AdminActionData::Type"
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum AdminActionDataType {
  None = 0,
  Add = 1,
  Kick = 2,
  Ban = 3,
  Unban = 4,
  Promote = 5,
  Demote = 6,
  Purge = 7,
  Mute = 8,
  Unmute = 9,
  Whitelist = 10,
  UnWhitelist = 11,
  ChangePermissionGroup = 12,
  Delete = 13,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct MultiplayerConfigSettings {
  name: String,
  description: String,
  password: String,
  allow_commands: AllowedCommands,
  visibility: ServerGameVisibility,
  max_players: u16,
  autosave_interval: u32,
  afk_auto_kick_interval: u32,
  max_upload_in_kilobytes_per_second: u32,
  max_upload_slots: u32,
  autosave_only_on_server: bool,
  non_blocking_saving: bool,
  ignore_player_limit_for_returning_players: bool,
  only_admins_can_pause_the_game: bool,
  require_user_verification: bool,
  enable_whitelist: bool,
  tags: Option<Vec<String>>,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ServerGameVisibility {
  public_game: bool,
  steam_game: bool,
  lan_game: bool,
}

// Source: disassembly AllowedCommands::Enum
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum AllowedCommands {
  True = 1,
  False = 2,
  AdminsOnly = 3,
}

// Source: disassembly DisconnectReason::Enum
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum DisconnectReason {
  Quit = 0,
  Dropped = 1,
  Reconnect = 2,
  WrongInput = 3,
  DesyncLimitReached = 4,
  CantKeepUp = 5,
  AFK = 6,
  Kicked = 7,
  KickedAndDeleted = 8,
  Banned = 9,
  BannedByAuthServer = 10,
  SwitchingServers = 11,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct QuickBarSetSelectedPageParameters {
  main_window_row: u8, // top or bottom
  new_selected_page: u8, // 0-9
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct SelectMapperSlotParameters {
  id: UpgradeId,
  index: u16,
  is_to: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct DragWaitConditionListBoxData {
  from: u32,
  to: u32,
  schedule_index: u32,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct DragListBoxData {
  from: u32,
  to: u32,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ChooseElemId {
  item: Item,
  entity: Entity,
  tile: Tile,
  fluid: Fluid,
  recipe: Recipe,
  signal: SignalId,
  decorative: Decorative,
  item_group: ItemGroup,
  achievement: Achievement,
  equipment: Equipment,
  technology: Technology,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ScriptDataTooLarge {
  size: u32,
  max_size: u32,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ImportBlueprintStringData {
  string_data: String,
  import_as_clipboard: bool,
  hide_imported_text: bool,
  not_from_chat: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct EditPermissionGroupParameters {
  #[space_optimized] group_id: u32,
  player_index: u16,
  action_index: u8,
  new_group_name: String,
  typ: EditPermissionGroupType,
}

// Source: disassembly EditPermissionGroupType
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum EditPermissionGroupType {
  Nothing = 0,
  AddPermission = 1,
  RemovePermission = 2,
  EnableAllPermissions = 3,
  DisableAllPermissions = 4,
  AddPlayer = 5,
  RemovePlayer = 6,
  EditGroupName = 7,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct CustomChartTagData {
  tag_number: u32,
  name: String,
  icon: SignalId,
  position: MapPosition,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct EntityEnergyPropertyChangedData {
  typ: EnergyPropertyType,
  value: f64,
}

// Source: disassembly EnergyPropertyType
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU32)]
pub enum EnergyPropertyType {
  BufferSize = 0,
  PowerProduction = 1,
  PowerUsage = 2,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ModSettingsChangedData {
  #[vec_u32] settings: Vec<ModSetting>,
}

#[derive(Debug)]
pub enum ModSetting {
  BoolSetting(String, bool),
  DoubleSetting(String, f64),
  IntSetting(String, u64),
  StringSetting(String, String),
  ColorSetting(String, Color),
}
impl ReplayReadWrite for ModSetting {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> {
    let typ = u8::replay_read(r)?;
    match typ {
      1 => {
        let name = String::replay_read(r)?;
        let value = bool::replay_read(r)?;
        Ok(ModSetting::BoolSetting(name, value))
      },
      2 => {
        let name = String::replay_read(r)?;
        let value = f64::replay_read(r)?;
        Ok(ModSetting::DoubleSetting(name, value))
      },
      3 => {
        let name = String::replay_read(r)?;
        let value = u64::replay_read(r)?;
        Ok(ModSetting::IntSetting(name, value))
      },
      4 => {
        let name = String::replay_read(r)?;
        let value = String::replay_read(r)?;
        Ok(ModSetting::StringSetting(name, value))
      },
      5 => {
        let name = String::replay_read(r)?;
        let value = Color::replay_read(r)?;
        Ok(ModSetting::ColorSetting(name, value))
      },
      _ => Err(r.stream.error_at(format!("Unknown ModSetting type {}", typ), 1))
    }
  }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> {
    match self {
      ModSetting::BoolSetting(name, value) => {
        1u8.replay_write(w)?;
        name.replay_write(w)?;
        value.replay_write(w)
      },
      ModSetting::DoubleSetting(name, value) => {
        2u8.replay_write(w)?;
        name.replay_write(w)?;
        value.replay_write(w)
      },
      ModSetting::IntSetting(name, value) => {
        3u8.replay_write(w)?;
        name.replay_write(w)?;
        value.replay_write(w)
      },
      ModSetting::StringSetting(name, value) => {
        4u8.replay_write(w)?;
        name.replay_write(w)?;
        value.replay_write(w)
      },
      ModSetting::ColorSetting(name, value) => {
        5u8.replay_write(w)?;
        name.replay_write(w)?;
        value.replay_write(w)
      },
    }
  }
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct InfinityPipeFilterData {
  fluid: Fluid,
  mode: InfinityFilterMode,
  percentage: f64,
  temperature: f64,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct InfinityContainerFilterItemData {
  item: Item,
  mode: InfinityFilterMode,
  filter_index: u16,
  count: u32,
}

// Source: disassembly InfinityFilter::Mode
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum InfinityFilterMode {
  KeepAtLeast = 0,
  KeepAtMost = 1,
  KeepExactly = 2,
  Add = 3,
  Remove = 4,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ServerCommandData {
  command: String,
  id: u32,
  connection_id: u64,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct TrainWaitConditionData {
  schedule_index: u32,
  condition_index: u32,
  condition: WaitCondition,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct WaitCondition {
  typ: WaitConditionType,
  compare_type: WaitConditionComparisonType,
  ticks: u32,
  circuit_condition: CircuitCondition,
}

// Source: disassembly WaitCondition::ComparisonType
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum WaitConditionComparisonType {
  And = 0,
  Or = 1,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct VehicleAutomaticTargetingParameters {
  auto_target_without_gunner: bool,
  auto_target_with_gunner: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ProgrammableSpeakerCircuitParameters {
  signal_value_is_pitch: bool,
  selected_instrument_id: u32,
  selected_note_id: u32,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ProgrammableSpeakerAlertParameters {
  show_alert: bool,
  show_on_map: bool,
  icon_signal_id: SignalId,
  alert_message: String,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ProgrammableSpeakerParameters {
  playback_volume: f64,
  playback_globally: bool,
  allow_polyphony: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ArithmeticCombinatorParameters {
  first_signal_id: SignalId,
  second_signal_id: SignalId,
  output_signal_id: SignalId,
  second_constant: i32,
  operation: ArithmeticCombinatorParametersOperation,
  second_signal_is_constant: bool,
  first_constant: i32,
  first_signal_is_constant: bool,
}

// Source: disassembly ArithmeticCombinatorParameters::Operation
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum ArithmeticCombinatorParametersOperation {
  Multiply = 0,
  Divide = 1,
  Add = 2,
  Subtract = 3,
  Modulo = 4,
  Power = 5,
  LeftShift = 6,
  RightShift = 7,
  AND = 8,
  OR = 9,
  XOR = 10,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ImportBlueprintsFilteredParameters {
  filter: Item,
  personal_shelf: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct EditBlueprintToolPreviewData {
  label: String,
  description: String,
  icons: Vec<SignalId>,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct UpgradeOpenedBlueprintByItemParameters {
  upgrade_record_id: BlueprintRecordId,
  upgrade: bool,
  setup_data: Option<SetupBlueprintData>,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct SendSpidertronParameters {
  position: MapPosition,
  flags: u8,  // append, follow_command
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct GuiGenericChangedData<T: ReplayReadWrite> {
  gui_changed_data: GuiChangedData,
  value: T,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct MarketOfferData {
  slot_index: u32,
  count: u32,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct GuiChangedData {
  pub gui_element_index: u32,
  pub button: u16,  // MouseButton bit field,
  pub is_alt: bool,
  pub is_control: bool,
  pub is_shift: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct LogisticFilterSignalData {
  signal: SignalId,
  filter_index: u16,
  count: u32,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct SignalData {
  pub signal_id: SignalId,
  pub signal_index: u16,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct AddTrainStationData {
  name: String,
  rail_position: MapPosition,
  temporary: bool,
  for_vehicle: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct BlueprintRecordLocation {
  shelf_index: Option<u16>,
  #[conditional_or_default(shelf_index.is_none())] parent_book_id: BlueprintRecordId,
  slot_index: u16,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct LogisticFilterItemData {
  item: Item,
  filter_index: u16,
  count: u32,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct TrainWaitCondition {
  action: TrainWaitConditionAction,
  add_type: WaitConditionType,
  schedule_index: u32,
  condition_index: u32,
}

// Source: disassembly ActionData::TrainWaitCondition::Action
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum TrainWaitConditionAction {
  Add = 0,
  Remove = 1,
  Toggle = 2,
}

// Source: disassembly WaitCondition::ConditionType
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum WaitConditionType {
  Time = 0,
  Full = 1,
  Empty = 2,
  ItemCount = 3,
  Circuit = 4,
  Inactivity = 5,
  RobotsInactive = 6,
  FluidCount = 7,
  PassengerPresent = 8,
  PassengerNotPresent = 9,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct BuildRailData {
  mode: RailBuildingMode,
  path: RailPathSpecification,
  alternative_build: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct RailPathSpecification {
  starting_point: RailPlanFinderLocation,
  buffer: ExtendedBitBuffer,
}

#[derive(Debug)]
pub struct ExtendedBitBuffer {
  bits: u32,
  data: Vec<u32>,
}
impl ReplayReadWrite for ExtendedBitBuffer {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> {
    let bits = r.stream.read_opt_u32()?;
    Ok(ExtendedBitBuffer { bits, data: r.stream.read_array((bits + 31) / 32)?, })
  }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> {
    w.stream.write_opt_u32(self.bits)?;
    w.stream.write_array(&self.data)
  }
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct RailPlanFinderLocation {
  position: TilePosition,
  direction: Direction,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct RailPathFinderLocation {
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum RailBuildingMode {
  Manual = 0,
  Ghost = 1,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct RidingState {
  pub direction: RidingDirection,
  pub acceleration_state: RidingAccelerationState,
}

// Source: defines.riding.direction
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum RidingDirection {
  Left = 0,
  Straight = 1,
  Right = 2,
}

// Source: defines.riding.acceleration
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum RidingAccelerationState {
  Nothing = 0,
  Accelerating = 1,
  Braking = 2,
  Reversing = 3,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct BuildTerrainParameters {
  pub position: MapPosition,
  pub direction: Direction,
  pub created_by_moving: bool,
  pub size: u8,
  pub ghost_mode: bool,
  pub skip_fog_of_war: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct TransferBlueprintData {
  record_id: BlueprintRecordId,
  raw_blueprint_data: Vec<u8>,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct BlueprintTransferQueueUpdateData {
  #[vec_u32] records: Vec<BlueprintTransferQueueUpdateDataRecord>,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct BlueprintTransferQueueUpdateDataRecord {
  id: u32,
  size: u32,
}


#[derive(Debug, ReplayReadWriteStruct)]
pub struct DeciderCombinatorParameters {
  first_signal_id: SignalId,
  second_signal_id: SignalId,
  output_signal_id: SignalId,
  second_constant: i32,
  comparator: Comparison,
  copy_count_from_input: bool,
  second_signal_is_constant: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct BehaviorModeOfOperationParameters {
  mode_of_operation: u8,
  enabled: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct CircuitConditionParameters {
  pub circuit_index: u8,
  pub condition: CircuitCondition,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct CircuitCondition {
  pub comparator: Comparison,
  pub first_signal: SignalId,
  pub second_signal: SignalId,
  pub second_constant: i32,
  pub second_signal_is_constant: bool,
}

// Source: disassembly Comparison::Enum
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum Comparison {
  GreaterThan = 0,
  LessThan = 1,
  Equals = 2,
  GreaterOrEqual = 3,
  LessOrEqual = 4,
  NotEqual = 5,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct TechnologyWithCount {
  technology: Technology,
  count: u32,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct UpgradeOpenedBlueprintByRecordParameters {
  upgrade_record_id: BlueprintRecordId,
  upgrade: bool,
  setup_data: Option<SetupBlueprintData>,
}

#[derive(Debug, Default, ReplayReadWriteStruct)]
pub struct BlueprintRecordId {
  player_index: u16,
  id: u32,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct SetupBlueprintData {
  label_data: ItemLabelData,
  description: String,
  snap_to_grid: Option<TilePosition>,  // + positionRelativeToTheGrid
  blueprint_shift: TilePosition,
  #[conditional_or_default(snap_to_grid.is_some())] position_relative_to_the_grid: Option<TilePosition>,
  include_modules: bool,
  include_fuel: bool,
  include_entities: bool,
  include_tiles: bool,
  include_station_names: bool,
  include_trains: bool,
  excluded_items: Vec<Item>,
  preview_icons: Vec<SignalId>,
  #[compacted_sorted] excluded_entities: Vec<u32>,
  #[assert_eq(0)] excluded_tiles: u8,  // loadCompactedSortedIndices
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ItemLabelData {
  label: String,
  label_color: Color,
  allow_manual_label_change: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct Color {
  r: f32,
  g: f32,
  b: f32,
  a: f32,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct SelectSlotParameters<T: ReplayReadWrite> {
  id: T,
  index: u16,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum GameRenderMode {
  Nothing = 0,
  Game = 1,
  Chart = 2,
  ChartZoomedIn = 3,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct SetSplitterPriorityData {
  input_priority: SplitterPriority,
  output_priority: SplitterPriority,
}
impl ReplayReadWrite for SetSplitterPriorityData {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> {
    let value = u8::replay_read(r)?;
    Ok(SetSplitterPriorityData { input_priority: SplitterPriority::from_u8(value / 3).unwrap(), output_priority: SplitterPriority::from_u8(value % 3).unwrap(), })
  }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> {
    let value = self.input_priority.to_u8().unwrap() * 3 + self.output_priority.to_u8().unwrap();
    value.replay_write(w)
  }
}

// Source: disassembly enum Splitter::Priority
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum SplitterPriority {
  Left = 0,
  None = 1,
  Right = 2,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct SelectAreaData {
  pub bounding_box: BoundingBox,
  pub skip_fog_of_war: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ShootingState {
  pub state: ShootingStateState,
  pub target: MapPosition,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum ShootingStateState {
  NotShooting = 0,
  ShootingEnemies = 1,
  ShootingSelected = 2,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct CancelCraftOrder {
  pub crafting_index: u16,
  pub count: u32,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct QuickBarSetSlotParameters {
  pub target_quick_bar_slot: u16,
  pub item_to_use: ItemStackTargetSpecification,
  pub currently_selected_quick_bar_slot: u16,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct QuickBarPickSlotParameters {
  pub location: u16,
  pub pick_ghost_cursor: bool,
  pub cursor_split: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct BuildParameters {
  pub position: MapPosition,
  pub direction: Direction,
  pub created_by_moving: bool,
  #[conditional_or_default(created_by_moving)] pub build_by_moving_start_position: Option<MapPosition>,
  pub flags: u8,  // 0:allow_belt_power_replace, 1:shiftBuild, 2:skipFogOfWar, 3+4:flip, 5:smartBeltBuilding
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct SmartPipetteData {
  pub entity_id: Entity,
  pub tile_id: Tile,
  pub equipment_id: Equipment,
  pub pick_ghost_cursor: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum TransferDirection {
  Out = 0,
  In = 1,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct SelectedEntityChangedVeryClosePreciseData {
  y: u8, // in 1/16 of a tile, starting from tileY(curpos) - 8
  x: u8, // in 1/16 of a tile, starting from tileX(curpos) - 8
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct CraftData {
  pub recipe: Recipe,
  pub count: u32,
}

type FixedPoint16 = i16;
#[derive(Debug, ReplayReadWriteStruct)]
pub struct SelectedEntityChangedRelativeData {
  y: FixedPoint16,
  x: FixedPoint16,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct EquipmentData {
  pos: EquipmentPosition,
  typ: EquipmentDataType,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
enum EquipmentDataType {
  CursorTransfer = 0,
  StandardTransfer = 1,
  TransferAllOfType = 2,
  TransferHalfOfType = 3,
  Place = 4,
  Open = 5,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct EquipmentPosition {
  pub x: i32,
  pub y: i32,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct SetFilterParameters {
  pub target: ItemStackTargetSpecification,
  pub filter: Item,  // Item
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct ItemStackTargetSpecification {
  inventory_index: u8,
  slot_index: u16,
  source: SlotSource,
  target: SlotTarget,
  #[conditional_or_default(target == SlotTarget::BlueprintLibrary)] local_shelf_target: bool,
}
impl ItemStackTargetSpecification {
  #[allow(dead_code)] pub fn from_quick_bar(qbar: u16, bar_slot: u16) -> Self {
    Self { inventory_index: 0x00, slot_index: qbar * 10 + bar_slot, source: SlotSource::PlayerQuickBar, target: SlotTarget::Default, local_shelf_target: false, }
  }
  #[allow(dead_code)] pub fn from_nothing() -> Self {
    Self { inventory_index: 0xff, slot_index: 0xffff, source: SlotSource::Empty, target: SlotTarget::Default, local_shelf_target: false, }
  }
  #[allow(dead_code)] pub fn from_cursor() -> Self {
    Self { inventory_index: 0xff, slot_index: 0xffff, source: SlotSource::PlayerCursor, target: SlotTarget::Default, local_shelf_target: false, }
  }
  #[allow(dead_code)] pub fn from_player_inventory(slot_index: u16) -> Self {
    Self { inventory_index: 1, slot_index, source: SlotSource::PlayerInventory, target: SlotTarget::Default, local_shelf_target: false, }
  }
  #[allow(dead_code)] pub fn from_container(slot_index: u16) -> Self {
    Self { inventory_index: 1, slot_index, source: SlotSource::EntityInventory, target: SlotTarget::Default, local_shelf_target: false, }
  }
  #[allow(dead_code)] pub fn from_fuel(slot_index: u16) -> Self {
    Self { inventory_index: 1, slot_index, source: SlotSource::EntityInventory, target: SlotTarget::Default, local_shelf_target: false, }
  }
  #[allow(dead_code)] pub fn from_machine_input(slot_index: u16) -> Self {
    Self { inventory_index: 2, slot_index, source: SlotSource::EntityInventory, target: SlotTarget::Default, local_shelf_target: false, }
  }
  #[allow(dead_code)] pub fn from_machine_output(slot_index: u16) -> Self {
    Self { inventory_index: 3, slot_index, source: SlotSource::EntityInventory, target: SlotTarget::Default, local_shelf_target: false, }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum SlotSource {
  Empty = 0,
  PlayerInventory = 1,
  PlayerExternalInventory = 2,
  PlayerCursor = 3,
  EntityInventory = 4,
  VehicleInventory = 5,
  OpenedItemInventory = 6,
  OpenedEquipmentInventory = 7,
  OpenedOtherPlayerInventory = 8,
  OpenedOtherPlayerCursor = 9,
  OpenedEntityCursor = 10,
  PlayerQuickBar = 11,
  OpenedEditorCharacterInventory = 12,
  OpenedEditorCharacterCursor = 13,
  VehicleInventoryInTools = 14,
  PlayerToolBar = 15,
  HeldByInserter = 16,
  OpenedScriptInventory = 17,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum SlotTarget {
  Default = 0,
  EquipmentGrid = 1,
  BlueprintLibrary = 2,
  TrashSlots = 3,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct SelectedEntityChangedVeryCloseData {
  xy: u8, // 2 4-bit numbers format 0xXXXXYYYY in full tiles ({xy}*16 + 8), starting from tile(curpos) - 8
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct CrcData {
  pub crc: u32,
  pub tick_of_crc: u32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum Direction {
  North= 0,
  NorthEast= 1,
  East= 2,
  SouthEast= 3,
  South= 4,
  SouthWest= 5,
  West= 6,
  NorthWest= 7,
  None= 8,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct CustomInputData {
  custom_input_id: u16,
  cursor_position: MapPosition,
  cursor_display_location: PixelPosition,
  selected_prototype_data: Option<SelectedPrototypeData>,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct PixelPosition {
  x: i32,
  y: i32,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct SelectedPrototypeData {
  base_type: String,
  derived_type: String,
  name: String,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct UpdateBlueprintShelfData {
  shelf_player_index: u16,
  next_record_id: u32,
  timestamp: u32,
  records_to_remove: Vec<u32>,
  records_to_invalidate_blueprint_contents: Vec<u32>,
  records_to_update: Vec<AddBlueprintRecordData>,
  records_to_reorder: Vec<(u32, u16)>,
  book_active_indexes_to_update: Vec<(u32, u16)>,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct AddBlueprintRecordData {
  item_id: u16,
  id: u32,
  label: String,
  typ: BlueprintRecordType,
  position: RecordPosition,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct RecordPosition {
  book_id: u32,
  index: u16,
}

#[derive(Debug)]
enum BlueprintRecordType {
  SingleBlueprint(SingleBlueprintSpecialData),
  BlueprintBook(BlueprintBookSpecialData),
  Deconstruction(DeconstructionSpecialData),
  Upgrade(UpgradeSpecialData),
}
impl ReplayReadWrite for BlueprintRecordType {
  fn replay_read<R: BufRead + Seek>(input: &mut ReplayDeserialiser<R>) -> Result<Self> {
    let typ = u8::replay_read(input)?;
    match typ {
      0 => Ok(BlueprintRecordType::SingleBlueprint(SingleBlueprintSpecialData::replay_read(input)?)),
      1 => Ok(BlueprintRecordType::BlueprintBook(BlueprintBookSpecialData::replay_read(input)?)),
      2 => Ok(BlueprintRecordType::Deconstruction(DeconstructionSpecialData::replay_read(input)?)),
      3 => Ok(BlueprintRecordType::Upgrade(UpgradeSpecialData::replay_read(input)?)),
      x => Err(input.stream.error_at(format!("unknown BlueprintRecordType type {}", x), 1)),
    }
  }
  fn replay_write(&self, input: &mut ReplaySerialiser) -> Result<()> {
    match self {
      BlueprintRecordType::SingleBlueprint(special) => { 0_u8.replay_write(input)?; special.replay_write(input) }
      BlueprintRecordType::BlueprintBook(special) => { 1_u8.replay_write(input)?; special.replay_write(input) }
      BlueprintRecordType::Deconstruction(special) => { 2_u8.replay_write(input)?; special.replay_write(input) }
      BlueprintRecordType::Upgrade(special) => { 3_u8.replay_write(input)?; special.replay_write(input) }
    }
  }
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct SingleBlueprintSpecialData {
  description: String,
  preview_icons: PreviewIconsPersistent,
  snap_to_grid: Option<(TilePosition, Option<TilePosition>)>,  // + positionRelativeToTheGrid
  blueprint_empty: bool,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct PreviewIconsPersistent {
  data_backup: Vec<String>,
  icons: Vec<SignalId>,
}

#[derive(Debug)]
pub enum SignalId {
  Item { item: Item },
  Fluid { fluid: Fluid, },
  VirtualSignal { virtual_signal: VirtualSignal, },
}
impl ReplayReadWrite for SignalId {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> {
    match u8::replay_read(r)? {
      0 => Ok(SignalId::Item { item: Item::replay_read(r)?, }),
      1 => Ok(SignalId::Fluid { fluid: Fluid::replay_read(r)?, }),
      2 => Ok(SignalId::VirtualSignal { virtual_signal: VirtualSignal::replay_read(r)?, }),
      x => Err(r.stream.error_at(format!("unknown SignalId type {}", x), 1)),
    }
  }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> {
    match self {
      SignalId::Item { item, } => { 0_u8.replay_write(w)?; item.replay_write(w) },
      SignalId::Fluid { fluid, } => { 1_u8.replay_write(w)?; fluid.replay_write(w) },
      SignalId::VirtualSignal { virtual_signal, } => { 2_u8.replay_write(w)?; virtual_signal.replay_write(w) },
    }
  }
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct BlueprintBookSpecialData {
  description: String,
  preview_icons: PreviewIconsPersistent,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct DeconstructionSpecialData {
  description: String,
  preview_icons: PreviewIconsPersistent,
  entity_filter_mode: EntityFilterMode,
  entity_filters_backup: Vec<IdBackupWithLocation>,
  #[vec_opt_u16] entity_filters: Vec<u16>,
  trees_and_rocks_only: bool,
  tile_filter_mode: TileFilterMode,
  tile_selection_mode: TileSelectionMode,
  tile_filters_backup: Vec<IdBackupWithLocation>,
  #[vec_opt_u16] tile_filters: Vec<u8>,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum EntityFilterMode {
  Whitelist = 0,
  Blacklist = 1,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum TileFilterMode {
  Whitelist = 0,
  Blacklist = 1,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum TileSelectionMode {
  Normal = 0,
  Always = 1,
  Never = 2,
  Only = 3,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct IdBackupWithLocation {
  index: u16,
  backup: String,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct UpgradeSpecialData {
  description: String,
  preview_icons: PreviewIconsPersistent,
  upgrade_mappers_backup: Vec<UpgradeIdBackupWithLocation>,
  mappers: Vec<(UpgradeId, UpgradeId)>, 
}

#[derive(Debug)]
pub enum UpgradeId {
  Entity { entity: u16 },
  Item { item: u16, },
}
impl ReplayReadWrite for UpgradeId {
  fn replay_read<R: BufRead + Seek>(r: &mut ReplayDeserialiser<R>) -> Result<Self> {
    match u8::replay_read(r)? {
      0 => Ok(UpgradeId::Entity { entity: u16::replay_read(r)?, }),
      1 => Ok(UpgradeId::Item { item: u16::replay_read(r)?, }),
      x => Err(r.stream.error_at(format!("unknown UpgradeId type {}", x), 1)),
    }
  }
  fn replay_write(&self, w: &mut ReplaySerialiser) -> Result<()> {
    match self {
      UpgradeId::Entity { entity, } => { 0_u8.replay_write(w)?; entity.replay_write(w) },
      UpgradeId::Item { item, } => { 1_u8.replay_write(w)?; item.replay_write(w) },
    }
  }
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct UpgradeIdBackupWithLocation {
  backup: String,
  mapper_index: u16,
}

#[derive(Debug, ReplayReadWriteStruct)]
pub struct PlayerJoinGameData {
  #[space_optimized] pub peer_id: u16, // consecutive player ids
  pub player_index: u16,
  pub force_id: ForceId,
  pub username: String,
  pub as_editor: bool,
  pub admin: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum ForceId {
  Player = 1,
  Enemy = 2,
  Neutral = 3,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU8)]
pub enum InputActionType {
  Nothing = 0,
  StopWalking = 1,
  BeginMining = 2,
  StopMining = 3,
  ToggleDriving = 4,
  OpenGui = 5,
  CloseGui = 6,
  OpenCharacterGui = 7,
  OpenCurrentVehicleGui = 8,
  ConnectRollingStock = 9,
  DisconnectRollingStock = 10,
  SelectedEntityCleared = 11,
  ClearCursor = 12,
  ResetAssemblingMachine = 13,
  OpenTechnologyGui = 14,
  LaunchRocket = 15,
  OpenProductionGui = 16,
  StopRepair = 17,
  CancelNewBlueprint = 18,
  CloseBlueprintRecord = 19,
  CopyEntitySettings = 20,
  PasteEntitySettings = 21,
  DestroyOpenedItem = 22,
  CopyOpenedItem = 23,
  ToggleShowEntityInfo = 24,
  SingleplayerInit = 25,
  MultiplayerInit = 26,
  DisconnectAllPlayers = 27,
  SwitchToRenameStopGui = 28,
  OpenBonusGui = 29,
  OpenTrainsGui = 30,
  OpenAchievementsGui = 31,
  CycleBlueprintBookForwards = 32,
  CycleBlueprintBookBackwards = 33,
  CycleClipboardForwards = 34,
  CycleClipboardBackwards = 35,
  StopMovementInTheNextTick = 36,
  ToggleEnableVehicleLogisticsWhileMoving = 37,
  ToggleDeconstructionItemEntityFilterMode = 38,
  ToggleDeconstructionItemTileFilterMode = 39,
  OpenLogisticGui = 40,
  SelectNextValidGun = 41,
  ToggleMapEditor = 42,
  DeleteBlueprintLibrary = 43,
  GameCreatedFromScenario = 44,
  ActivateCopy = 45,
  ActivateCut = 46,
  ActivatePaste = 47,
  Undo = 48,
  TogglePersonalRoboport = 49,
  ToggleEquipmentMovementBonus = 50,
  TogglePersonalLogisticRequests = 51,
  ToggleEntityLogisticRequests = 52,
  StopBuildingByMoving = 53,
  FlushOpenedEntityFluid = 54,
  ForceFullCRC = 55,
  OpenTipsAndTricksGui = 56,
  OpenBlueprintLibraryGui = 57,
  ChangeBlueprintLibraryTab = 58,
  DropItem = 59,
  Build = 60,
  StartWalking = 61,
  BeginMiningTerrain = 62,
  ChangeRidingState = 63,
  OpenItem = 64,
  OpenParentOfOpenedItem = 65,
  ResetItem = 66,
  DestroyItem = 67,
  OpenModItem = 68,
  OpenEquipment = 69,
  CursorTransfer = 70,
  CursorSplit = 71,
  StackTransfer = 72,
  InventoryTransfer = 73,
  CheckCRCHeuristic = 74,
  Craft = 75,
  WireDragging = 76,
  ChangeShootingState = 77,
  SetupAssemblingMachine = 78,
  SelectedEntityChanged = 79,
  SmartPipette = 80,
  StackSplit = 81,
  InventorySplit = 82,
  CancelCraft = 83,
  SetFilter = 84,
  CheckCRC = 85,
  SetCircuitCondition = 86,
  SetSignal = 87,
  StartResearch = 88,
  SetLogisticFilterItem = 89,
  SetLogisticFilterSignal = 90,
  SetCircuitModeOfOperation = 91,
  GuiClick = 92,
  GuiConfirmed = 93,
  WriteToConsole = 94,
  MarketOffer = 95,
  AddTrainStation = 96,
  ChangeTrainStopStation = 97,
  ChangeActiveItemGroupForCrafting = 98,
  ChangeActiveItemGroupForFilters = 99,
  ChangeActiveCharacterTab = 100,
  GuiTextChanged = 101,
  GuiCheckedStateChanged = 102,
  GuiSelectionStateChanged = 103,
  GuiSelectedTabChanged = 104,
  GuiValueChanged = 105,
  GuiSwitchStateChanged = 106,
  GuiLocationChanged = 107,
  PlaceEquipment = 108,
  TakeEquipment = 109,
  UseItem = 110,
  SendSpidertron = 111,
  UseArtilleryRemote = 112,
  SetInventoryBar = 113,
  MoveOnZoom = 114,
  StartRepair = 115,
  Deconstruct = 116,
  Upgrade = 117,
  Copy = 118,
  AlternativeCopy = 119,
  SelectBlueprintEntities = 120,
  AltSelectBlueprintEntities = 121,
  SetupBlueprint = 122,
  SetupSingleBlueprintRecord = 123,
  CopyOpenedBlueprint = 124,
  ReassignBlueprint = 125,
  OpenBlueprintRecord = 126,
  GrabBlueprintRecord = 127,
  DropBlueprintRecord = 128,
  DeleteBlueprintRecord = 129,
  UpgradeOpenedBlueprintByRecord = 130,
  UpgradeOpenedBlueprintByItem = 131,
  SpawnItem = 132,
  SpawnItemStackTransfer = 133,
  UpdateBlueprintShelf = 134,
  TransferBlueprint = 135,
  TransferBlueprintImmediately = 136,
  EditBlueprintToolPreview = 137,
  RemoveCables = 138,
  ExportBlueprint = 139,
  ImportBlueprint = 140,
  ImportBlueprintsFiltered = 141,
  PlayerJoinGame = 142,
  PlayerAdminChange = 143,
  CancelDeconstruct = 144,
  CancelUpgrade = 145,
  ChangeArithmeticCombinatorParameters = 146,
  ChangeDeciderCombinatorParameters = 147,
  ChangeProgrammableSpeakerParameters = 148,
  ChangeProgrammableSpeakerAlertParameters = 149,
  ChangeProgrammableSpeakerCircuitParameters = 150,
  SetVehicleAutomaticTargetingParameters = 151,
  BuildTerrain = 152,
  ChangeTrainWaitCondition = 153,
  ChangeTrainWaitConditionData = 154,
  CustomInput = 155,
  ChangeItemLabel = 156,
  ChangeItemDescription = 157,
  ChangeEntityLabel = 158,
  BuildRail = 159,
  CancelResearch = 160,
  SelectArea = 161,
  AltSelectArea = 162,
  ReverseSelectArea = 163,
  AltReverseSelectArea = 164,
  ServerCommand = 165,
  SetControllerLogisticTrashFilterItem = 166,
  SetEntityLogisticTrashFilterItem = 167,
  SetInfinityContainerFilterItem = 168,
  SetInfinityPipeFilter = 169,
  ModSettingsChanged = 170,
  SetEntityEnergyProperty = 171,
  EditCustomTag = 172,
  EditPermissionGroup = 173,
  ImportBlueprintString = 174,
  ImportPermissionsString = 175,
  ReloadScript = 176,
  ReloadScriptDataTooLarge = 177,
  GuiElemChanged = 178,
  BlueprintTransferQueueUpdate = 179,
  DragTrainSchedule = 180,
  DragTrainWaitCondition = 181,
  SelectItem = 182,
  SelectEntitySlot = 183,
  SelectTileSlot = 184,
  SelectMapperSlot = 185,
  DisplayResolutionChanged = 186,
  QuickBarSetSlot = 187,
  QuickBarPickSlot = 188,
  QuickBarSetSelectedPage = 189,
  PlayerLeaveGame = 190,
  MapEditorAction = 191,
  PutSpecialItemInMap = 192,
  PutSpecialRecordInMap = 193,
  ChangeMultiplayerConfig = 194,
  AdminAction = 195,
  LuaShortcut = 196,
  TranslateString = 197,
  FlushOpenedEntitySpecificFluid = 198,
  ChangePickingState = 199,
  SelectedEntityChangedVeryClose = 200,
  SelectedEntityChangedVeryClosePrecise = 201,
  SelectedEntityChangedRelative = 202,
  SelectedEntityChangedBasedOnUnitNumber = 203,
  SetAutosortInventory = 204,
  SetFlatControllerGui = 205,
  SetRecipeNotifications = 206,
  SetAutoLaunchRocket = 207,
  SwitchConstantCombinatorState = 208,
  SwitchPowerSwitchState = 209,
  SwitchInserterFilterModeState = 210,
  SwitchConnectToLogisticNetwork = 211,
  SetBehaviorMode = 212,
  FastEntityTransfer = 213,
  RotateEntity = 214,
  FastEntitySplit = 215,
  SetTrainStopped = 216,
  ChangeControllerSpeed = 217,
  SetAllowCommands = 218,
  SetResearchFinishedStopsGame = 219,
  SetInserterMaxStackSize = 220,
  OpenTrainGui = 221,
  SetEntityColor = 222,
  SetDeconstructionItemTreesAndRocksOnly = 223,
  SetDeconstructionItemTileSelectionMode = 224,
  DeleteCustomTag = 225,
  DeletePermissionGroup = 226,
  AddPermissionGroup = 227,
  SetInfinityContainerRemoveUnfilteredItems = 228,
  SetCarWeaponsControl = 229,
  SetRequestFromBuffers = 230,
  ChangeActiveQuickBar = 231,
  OpenPermissionsGui = 232,
  DisplayScaleChanged = 233,
  SetSplitterPriority = 234,
  GrabInternalBlueprintFromText = 235,
  SetHeatInterfaceTemperature = 236,
  SetHeatInterfaceMode = 237,
  OpenTrainStationGui = 238,
  RemoveTrainStation = 239,
  GoToTrainStation = 240,
  RenderModeChanged = 241,
  PlayerInputMethodChanged = 242,
  SetPlayerColor = 243,
  PlayerClickedGpsTag = 244,
  SetTrainsLimit = 245,
  ClearRecipeNotification = 246,
  SetLinkedContainerLinkID = 247,
  GuiHover = 248,
  GuiLeave = 249,
}