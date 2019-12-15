use crate as factorio_serialize;
use crate::constants::*;
use crate::structs::*;
use std::io::{BufRead, Seek, Write};
use factorio_serialize::{ReadWrite, ReadWriteStruct, ReadWriteTaggedUnion, Reader, Result, Writer};
use num_traits::cast::{FromPrimitive, ToPrimitive};

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
    w.write_array(&self.0)
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

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct SetBlueprintIconData {
  signal: SignalId,
  index: u8,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct BlueprintRecordId {
  player_index: u16,
  id: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct DropBlueprintRecordParameters {
  player_index: u16,
  blueprint_book_to_drop_in: BlueprintRecordId,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct TransferBlueprintData {
  record_id: BlueprintRecordId,
  raw_blueprint_data: String,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct ChangeBlueprintBookRecordLabelData {
  book_id: BlueprintRecordId,
  label: String,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
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

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct DeciderCombinatorParameters {
  first_signal_id: SignalId,
  second_signal_id: SignalId,
  output_signal_id: SignalId,
  second_constant: i32,
  comparator: Comparison,
  copy_count_from_input: bool,
  second_signal_is_constant: bool,
}

#[derive(Debug, ReadWriteStruct)]
pub struct ProgrammableSpeakerParameters {
  playback_volume: f64,
  playback_globally: bool,
  allow_polyphony: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct ProgrammableSpeakerAlertParameters {
  show_alert: bool,
  show_on_map: bool,
  icon_signal_id: SignalId,
  alert_message: String,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct ProgrammableSpeakerCircuitParameters {
  signal_value_is_pitch: bool,
  selected_instrument_id: u32,
  selected_note_id: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct BuildTerrainParameters {
  pub position: MapPosition,
  pub direction: Direction,
  #[negated_bool] pub created_by_moving: bool,
  pub size: u8,
  pub ghost_mode: bool,
  pub skip_fog_of_war: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct TrainWaitCondition {
  action: TrainWaitConditionAction,
  add_type: WaitConditionType,
  schedule_index: u32,
  condition_index: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct TrainWaitConditionData {
  schedule_index: u32,
  condition_index: u32,
  condition: WaitCondition,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct BuildRailData {
  mode: RailBuildingMode,
  path: RailPathSpecification,
  alternative_build: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct RailPathSpecification {
  starting_point: RailPlanFinderLocation,
  buffer: ExtendedBitBuffer,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct RailPlanFinderLocation {
  position: TilePosition,
  direction: Direction,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ExtendedBitBuffer {
  bits: u32,
  data: Vec<u32>,
}
impl ReadWrite for ExtendedBitBuffer {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let bits = r.read_opt_u32()?;
    Ok(ExtendedBitBuffer { bits, data: r.read_array((bits + 31) / 32)?, })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_opt_u32(self.bits)?;
    w.write_array(&self.data)
  }
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct TechnologyWithCount {
  technology: Technology,
  count: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct ServerCommandData {
  command: String,
  id: u32,
  connection_id: u64,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct InfinityContainerFilterItemData {
  item: Item,
  mode: InfinityFilterMode,
  filter_index: u16,
  count: u32,
}

#[derive(Debug, ReadWriteStruct)]
pub struct InfinityPipeFilterData {
  fluid: Fluid,
  mode: InfinityFilterMode,
  percentage: f64,
  temperature: f64,
}

#[derive(Debug, ReadWriteStruct)]
pub struct ModSettingsChangedData {
  #[non_space_optimized] settings: Vec<ModSetting>,
}

#[derive(Debug)]
pub enum ModSetting {
  BoolSetting(String, bool),
  DoubleSetting(String, f64),
  IntSetting(String, u64),
  StringSetting(String, String),
}
impl ReadWrite for ModSetting {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let typ = r.read_u8()?;
    match typ {
      1 => {
        let name = String::read(r)?;
        let value = bool::read(r)?;
        Ok(ModSetting::BoolSetting(name, value))
      },
      2 => {
        let name = String::read(r)?;
        let value = f64::read(r)?;
        Ok(ModSetting::DoubleSetting(name, value))
      },
      3 => {
        let name = String::read(r)?;
        let value = u64::read(r)?;
        Ok(ModSetting::IntSetting(name, value))
      },
      4 => {
        let name = String::read(r)?;
        let value = String::read(r)?;
        Ok(ModSetting::StringSetting(name, value))
      },
      _ => Err(r.error_at(format!("Unknown ModSetting type {}", typ), 1))
    }
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    match self {
      ModSetting::BoolSetting(name, value) => {
        1u8.write(w)?;
        name.write(w)?;
        value.write(w)
      },
      ModSetting::DoubleSetting(name, value) => {
        2u8.write(w)?;
        name.write(w)?;
        value.write(w)
      },
      ModSetting::IntSetting(name, value) => {
        3u8.write(w)?;
        name.write(w)?;
        value.write(w)
      },
      ModSetting::StringSetting(name, value) => {
        4u8.write(w)?;
        name.write(w)?;
        value.write(w)
      },
    }
  }
}

#[derive(Debug, ReadWriteStruct)]
pub struct EntityEnergyPropertyChangedData {
  typ: EnergyPropertyType,
  value: f64,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct CustomChartTagData {
  tag_number: u32,
  name: String,
  icon: SignalId,
  position: MapPosition,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct EditPermissionGroupParameters {
  #[space_optimized] group_id: u32,
  player_index: u16,
  action_index: u8,
  new_group_name: String,
  typ: EditPermissionGroupType,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct ImportBlueprintStringData {
  string_data: String,
  import_as_clipboard: bool,
  hide_imported_text: bool,
  not_from_chat: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct ScriptDataTooLarge {
  size: u32,
  max_size: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
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

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct BlueprintTransferQueueUpdateData {
  #[non_space_optimized] records: Vec<BlueprintTransferQueueUpdateDataRecord>,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct BlueprintTransferQueueUpdateDataRecord {
  id: u32,
  size: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct DragListBoxData {
  from: u32,
  to: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct DragWaitConditionListBoxData {
  from: u32,
  to: u32,
  schedule_index: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct SelectSlotParameters<T: ReadWrite> {
  id: T,
  index: u16,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct SelectMapperSlotParameters {
  id: UpgradeId,
  index: u16,
  is_to: bool,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum UpgradeId {
  Entity(Entity),
  Item(Item),
}
impl ReadWrite for UpgradeId {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let typ = r.read_u8()?;
    match typ {
      0 => {
        let value = Entity::read(r)?;
        Ok(UpgradeId::Entity(value))
      },
      1 => {
        let value = Item::read(r)?;
        Ok(UpgradeId::Item(value))
      },
      _ => Err(r.error_at(format!("Unknown UpgradeId type {}", typ), 1))
    }
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    match self {
      UpgradeId::Entity(value) => {
        0u8.write(w)?;
        value.write(w)
      },
      UpgradeId::Item(value) => {
        1u8.write(w)?;
        value.write(w)
      },
    }
  }
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct AdminActionData {
  player_index: u16,
  username: String,
  new_group_id: u32,
  action: AdminActionDataType,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
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
  tags: MultiplayerConfigSettingsTags,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct MultiplayerConfigSettingsTags {
  tags: Vec<String>,
}
impl ReadWrite for MultiplayerConfigSettingsTags {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let b = r.read_bool()?;
    if b {
      Ok(MultiplayerConfigSettingsTags { tags: <Vec<String>>::read(r)? })
    } else { Ok(MultiplayerConfigSettingsTags { tags: vec![] }) }
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    (!self.tags.is_empty()).write(w)?;
    if !self.tags.is_empty() {
      self.tags.write(w)
    } else { Ok(()) }
  }
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct ServerGameVisibility {
  public_game: bool,
  steam_game: bool,
  lan_game: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct LuaShortcutData {
  player: u16,
  prototype_name: String,
}

#[derive(Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct TranslationResultData {
  localised_string: LocalisedString,
  result: String,
  translated: bool,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct LocalisedString {
  key: String,
  mode: LocalisedStringMode,
  parameters: Vec<LocalisedString>,
}
impl ReadWrite for LocalisedString {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let key = String::read(r)?;
    let mode = LocalisedStringMode::read(r)?;
    let len = r.read_u8()?;
    let mut parameters = vec![];
    for _ in 0..len { parameters.push(LocalisedString::read(r)?); }
    Ok(LocalisedString { key, mode, parameters, })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    self.key.write(w)?;
    self.mode.write(w)?;
    w.write_u8(self.parameters.len() as u8)?;
    for record in &self.parameters { record.write(w)?; }
    Ok(())
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct SetSplitterPriorityData {
  input_priority: SplitterPriority,
  output_priority: SplitterPriority,
}
impl ReadWrite for SetSplitterPriorityData {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let value = u8::read(r)?;
    Ok(SetSplitterPriorityData { input_priority: SplitterPriority::from_u8(value / 3).unwrap(), output_priority: SplitterPriority::from_u8(value % 3).unwrap(), })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    let value = self.input_priority.to_u8().unwrap() * 3 + self.output_priority.to_u8().unwrap();
    value.write(w)
  }
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
  SetupBlueprint(SetupBlueprintData),
  SetupSingleBlueprintRecord(SetupBlueprintData),
  SetSingleBlueprintRecordIcon(SetBlueprintIconData),
  OpenBlueprintRecord(BlueprintRecordId),
  CloseBlueprintBook(BlueprintRecordId),
  ChangeSingleBlueprintRecordLabel(String),
  GrabBlueprintRecord(BlueprintRecordId),
  DropBlueprintRecord(DropBlueprintRecordParameters),
  DeleteBlueprintRecord(BlueprintRecordId),
  CreateBlueprintLike(Item),
  CreateBlueprintLikeStackTransfer(Item),
  UpdateBlueprintShelf(UpdateBlueprintShelfData),
  TransferBlueprint(TransferBlueprintData),
  TransferBlueprintImmediately(TransferBlueprintData),
  ChangeBlueprintBookRecordLabel(ChangeBlueprintBookRecordLabelData),
  RemoveCables(MapPosition),
  ExportBlueprint(DropBlueprintRecordParameters),
  ImportBlueprint(BlueprintRecordId),
  PlayerJoinGame(PlayerJoinGameData),
  CancelDeconstruct(SelectAreaData),
  CancelUpgrade(SelectAreaData),
  ChangeArithmeticCombinatorParameters(ArithmeticCombinatorParameters),
  ChangeDeciderCombinatorParameters(DeciderCombinatorParameters),
  ChangeProgrammableSpeakerParameters(ProgrammableSpeakerParameters),
  ChangeProgrammableSpeakerAlertParameters(ProgrammableSpeakerAlertParameters),
  ChangeProgrammableSpeakerCircuitParameters(ProgrammableSpeakerCircuitParameters),
  BuildTerrain(BuildTerrainParameters),
  ChangeTrainWaitCondition(TrainWaitCondition),
  ChangeTrainWaitConditionData(TrainWaitConditionData),
  CustomInput(u16),
  ChangeItemLabel(String),
  BuildRail(BuildRailData),
  CancelResearch(TechnologyWithCount),
  SelectArea(SelectAreaData),
  AltSelectArea(SelectAreaData),
  ServerCommand(ServerCommandData),
  ClearSelectedBlueprint(Slot),
  ClearSelectedDeconstructionItem(Slot),
  ClearSelectedUpgradeItem(Slot),
  SetLogisticTrashFilterItem(LogisticFilterItemData),
  SetInfinityContainerFilterItem(InfinityContainerFilterItemData),
  SetInfinityPipeFilter(InfinityPipeFilterData),
  ModSettingsChanged(ModSettingsChangedData),
  SetEntityEnergyProperty(EntityEnergyPropertyChangedData),
  EditCustomTag(CustomChartTagData),
  EditPermissionGroup(EditPermissionGroupParameters),
  ImportBlueprintString(ImportBlueprintStringData),
  ImportPermissionsString(String),
  ReloadScript(String),
  ReloadScriptDataTooLarge(ScriptDataTooLarge),
  GuiElemChanged(GuiGenericChangedData<ChooseElemId>),
  BlueprintTransferQueueUpdate(BlueprintTransferQueueUpdateData),
  DragTrainSchedule(DragListBoxData),
  DragTrainWaitCondition(DragWaitConditionListBoxData),
  SelectItem(SelectSlotParameters<Item>),
  SelectEntitySlot(SelectSlotParameters<Entity>),
  SelectTileSlot(SelectSlotParameters<Tile>),
  SelectMapperSlot(SelectMapperSlotParameters),
  DisplayResolutionChanged(PixelPosition),
  QuickBarSetSlot(QuickBarSetSlotParameters),
  QuickBarPickSlot(QuickBarPickSlotParameters),
  QuickBarSetSelectedPage(QuickBarSetSelectedPageParameters),
  PlayerLeaveGame(DisconnectReason),
  // MapEditorAction, // has lots of sub-operations
  PutSpecialItemInMap(Slot),
  ChangeMultiplayerConfig(MultiplayerConfigSettings),
  AdminAction(AdminActionData),
  LuaShortcut(LuaShortcutData),
  TranslateString(TranslationResultData),
  ChangePickingState(u8),
  SelectedEntityChangedVeryClose(SelectedEntityChangedVeryCloseData),
  SelectedEntityChangedVeryClosePrecise(SelectedEntityChangedVeryClosePreciseData),
  SelectedEntityChangedRelative(SelectedEntityChangedRelativeData),
  SelectedEntityChangedBasedOnUnitNumber(u32),
  SetAutosortInventory(bool),
  SetAutoLaunchRocket(bool),
  SwitchConstantCombinatorState(bool),
  SwitchPowerSwitchState(bool),
  SwitchInserterFilterModeState(InserterFilterMode),
  SwitchConnectToLogisticNetwork(bool),
  SetBehaviorMode(HandOrContentsReadMode),
  FastEntityTransfer(TransferDirection),
  RotateEntity(u8), // unknown values
  FastEntitySplit(TransferDirection),
  SetTrainStopped(bool),
  ChangeControllerSpeed(f64),
  SetAllowCommands(u8), // unknown values
  SetResearchFinishedStopsGame(bool),
  SetInserterMaxStackSize(u8),
  OpenTrainGui(u32),
  SetEntityColor(u32),
  SetDeconstructionItemTreesAndRocksOnly(u8), // unknown values
  SetDeconstructionItemTileSelectionMode(u8), // unknown values 0-3
  DropToBlueprintBook(u16),
  DeleteCustomTag(u32),
  DeletePermissionGroup(u32),
  AddPermissionGroup(u32),
  SetInfinityContainerRemoveUnfilteredItems(bool),
  SetCarWeaponsControl(u8), // unknown values
  SetRequestFromBuffers(bool),
  ChangeActiveQuickBar(u8),
  OpenPermissionsGui(bool),
  DisplayScaleChanged(f64),
  SetSplitterPriority(SetSplitterPriorityData),
  GrabInternalBlueprintFromText(u32),
  SetHeatInterfaceTemperature(f64),
  SetHeatInterfaceMode(u8), // unknown values
  OpenTrainStationGui(u32),
  RemoveTrainStation(u32),
  GoToTrainStation(u32),
  RenderModeChanged(GameRenderMode),
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
