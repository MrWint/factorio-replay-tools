use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Seek;
use std::num::TryFromIntError;

use enum_primitive_derive::Primitive;
use factorio_serialize_derive::MapReadWriteEnumU8;
use factorio_serialize_derive::MapReadWriteStruct;
use factorio_serialize_derive::MapReadWriteTaggedUnion;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;

use crate::Reader;
use crate::Result;
use crate::Writer;
use crate::constants::Achievement;
use crate::constants::Entity;
use crate::constants::Tile;


pub struct MapData {
  pub map_version: MapVersion,  // part of MapDeserializer
  pub scenario_execution_context: ScenarioExecutionContext,
  pub map: Map,

  pub remaining_data: Vec<u8>,  // unknown unparsed data
}
impl MapData {
  pub fn parse_map_data(map_data: &[u8]) -> Result<MapData> {
    let mut map_deserialiser = MapDeserialiser::new(Cursor::new(map_data))?;

    let scenario_execution_context = ScenarioExecutionContext::map_read(&mut map_deserialiser)?;
    let map = Map::map_read(&mut map_deserialiser)?;

    let remaining_data = map_deserialiser.stream.read_to_end()?;
    let map_version = map_deserialiser.map_version;

    Ok(MapData { map_version, scenario_execution_context, map, remaining_data })
  }

  pub fn write_map_data(&self) -> Result<Vec<u8>> {
    let mut map_serialiser = MapSerialiser::new(self.map_version.clone())?;

    self.scenario_execution_context.map_write(&mut map_serialiser)?;
    self.map.map_write(&mut map_serialiser)?;

    map_serialiser.stream.write_bytes(&self.remaining_data)?;

    Ok(map_serialiser.stream.into_inner().into_inner())
  }
}
impl std::fmt::Debug for MapData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "map_version: {:?}", self.map_version)?;
    writeln!(f, "scenario_execution_context: {:?}", self.scenario_execution_context)?;
    writeln!(f, "map: {:?}", self.map)?;
    Ok(())
  }
}

pub struct MapDeserialiser<R: BufRead + Seek> {
  pub stream: Reader<R>,
  pub map_version: MapVersion,

  pub last_loaded_position: MapPosition,
}
impl<R: BufRead + Seek> MapDeserialiser<R> {
  pub fn new(map_data: R) -> Result<MapDeserialiser<R>> {
    let mut stream = Reader::new(map_data);
    let map_version = MapVersion::new(&mut stream)?;

    Ok(MapDeserialiser {
      stream,
      map_version,

      last_loaded_position: MapPosition::default(),
    })
  }
}
pub struct MapSerialiser {
  pub stream: Writer<Cursor<Vec<u8>>>,
  pub map_version: MapVersion,

  pub last_saved_position: MapPosition,
}
impl MapSerialiser{
  fn new(map_version: MapVersion) -> Result<MapSerialiser> {
    let mut stream = Writer::new(Cursor::new(Vec::new()));
    map_version.write(&mut stream)?;

    Ok(MapSerialiser {
      stream,
      map_version,

      last_saved_position: MapPosition::default(),
    })
  }
}

pub trait MapReadWrite: Sized {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self>;
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()>;
}
pub fn map_read_vec_u32<R: BufRead + Seek, T: MapReadWrite>(input: &mut MapDeserialiser<R>) -> Result<Vec<T>> {
  let len = input.stream.read_u32()?;
  (0..len).map(|_| T::map_read(input)).collect()
}
pub fn map_write_vec_u32<T: MapReadWrite>(v: &[T], input: &mut MapSerialiser) -> Result<()> {
  input.stream.write_u32(v.len() as u32)?;
  v.into_iter().map(|v| v.map_write(input)).collect()
}
pub fn map_read_vec_u16<R: BufRead + Seek, T: MapReadWrite>(input: &mut MapDeserialiser<R>) -> Result<Vec<T>> {
  let len = input.stream.read_u16()?;
  (0..len).map(|_| T::map_read(input)).collect()
}
pub fn map_write_vec_u16<T: MapReadWrite>(v: &[T], input: &mut MapSerialiser) -> Result<()> {
  input.stream.write_u16(v.len() as u16)?;
  v.into_iter().map(|v| v.map_write(input)).collect()
}
impl<T: MapReadWrite> MapReadWrite for Vec<T> {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    let len = input.stream.read_opt_u32()?;
    (0..len).map(|_| T::map_read(input)).collect()
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    input.stream.write_opt_u32(self.len() as u32)?;
    self.into_iter().map(|v| v.map_write(input)).collect()
  }
}
impl<T: MapReadWrite + Debug, const N: usize> MapReadWrite for [T; N] {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    Ok((0..N).map(|_| T::map_read(input)).collect::<Result<Vec<_>>>()?.try_into().unwrap())
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    self.into_iter().map(|v| v.map_write(input)).collect()
  }
}
impl<T: MapReadWrite> MapReadWrite for Option<T> {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    if input.stream.read_bool()? {
      Ok(Some(T::map_read(input)?))
    } else {
      Ok(None)
    }
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    match self {
      Some(value) => {
        input.stream.write_bool(true)?;
        value.map_write(input)
      },
      None => input.stream.write_bool(false)
    }
  }
}
impl MapReadWrite for bool {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> { r.stream.read_bool() }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> { w.stream.write_bool(*self) }
}
impl MapReadWrite for i16 {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> { r.stream.read_i16() }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> { w.stream.write_i16(*self) }
}
impl MapReadWrite for i32 {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> { r.stream.read_i32() }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> { w.stream.write_i32(*self) }
}
impl MapReadWrite for u8 {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> { r.stream.read_u8() }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> { w.stream.write_u8(*self) }
}
impl MapReadWrite for u16 {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> { r.stream.read_u16() }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> { w.stream.write_u16(*self) }
}
impl MapReadWrite for u32 {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> { r.stream.read_u32() }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> { w.stream.write_u32(*self) }
}
impl MapReadWrite for u64 {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> { r.stream.read_u64() }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> { w.stream.write_u64(*self) }
}
impl MapReadWrite for f32 {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> { r.stream.read_f32() }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> { w.stream.write_f32(*self) }
}
impl MapReadWrite for f64 {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> { r.stream.read_f64() }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> { w.stream.write_f64(*self) }
}
impl MapReadWrite for String {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> { r.stream.read_string() }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> { w.stream.write_string(self) }
}
impl <K: MapReadWrite + Eq + Hash, V: MapReadWrite> MapReadWrite for HashMap<K, V> {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> {
    let count = r.stream.read_opt_u32()?;
    let mut result = HashMap::new();
    for _ in 0..count {
      let key = K::map_read(r)?;
      let value = V::map_read(r)?;
      result.insert(key, value);
    }

    Ok(result)
  }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> {
    w.stream.write_opt_u32(self.len() as u32)?;
    for (key, value) in self.into_iter() {
      key.map_write(w)?;
      value.map_write(w)?;
    }

    Ok(())
  }
}
impl <A: MapReadWrite, B: MapReadWrite> MapReadWrite for (A, B) {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> {
    let a = A::map_read(r)?;
    let b = B::map_read(r)?;

    Ok((a, b))
  }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> {
    self.0.map_write(w)?;
    self.1.map_write(w)?;

    Ok(())
  }
}

#[derive(Clone, Debug)]
pub struct MapVersion {
  version: u64,
  quality_version: bool,
}
impl MapVersion {
  pub fn new<R: BufRead + Seek>(stream: &mut Reader<R>) -> Result<MapVersion> {
    let version_major = stream.read_u16()?;
    let version_minor = stream.read_u16()?;
    let version_patch = stream.read_u16()?;
    let version_dev = stream.read_u16()?;
    let version = u64::from(version_major) << 48 | u64::from(version_minor) << 32 | u64::from(version_patch) << 16 | u64::from(version_dev);
    let quality_version = stream.read_bool()?;

    Ok(MapVersion { version, quality_version })
  }
  pub fn write(&self, stream: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
    stream.write_u16((self.version >> 48) as u16)?;
    stream.write_u16((self.version >> 32) as u16)?;
    stream.write_u16((self.version >> 16) as u16)?;
    stream.write_u16((self.version >> 0) as u16)?;
    stream.write_bool(self.quality_version)
  }
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ScenarioExecutionContext {
  scenario_location: ScenarioLocation,
  difficulty: Difficulty,
  finished: bool,
  player_won: bool,
  next_level: String,
  can_continue: bool,
  finished_but_continuing: bool,
  saving_replay: bool,
  allow_non_admin_debug_options: bool,
  loaded_from: ApplicationVersion,
  allowed_commands: AllowedCommands,
  active_mods: Vec<ModId>,
  startup_settings_crc: u32,
  startup_mod_settings: PropertyTree,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ScenarioLocation {
  campaign_name: String,
  level_name: String,
  mod_name: String,
}

// Source: disassembly Difficulty::Enum
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, MapReadWriteEnumU8)]
pub enum Difficulty {
  Easy = 0,
  Normal = 1,
  Hard = 2,
  Nothing = 3,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ApplicationVersion {
  #[space_optimized] major_version: u16,
  #[space_optimized] minor_version: u16,
  #[space_optimized] sub_version: u16,
  build_version: u16,
}

// Source: disassembly AllowedCommands::Enum
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, MapReadWriteEnumU8)]
pub enum AllowedCommands {
  True = 1,
  False = 2,
  AdminsOnly = 3,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ModId {
  name: String,
  version: ModVersion,
  crc: u32,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ModVersion {
  #[space_optimized] major_version: u16,
  #[space_optimized] minor_version: u16,
  #[space_optimized] sub_version: u16,
}

#[derive(Clone, Debug)]
pub enum PropertyTree {
  Nothing { any_type_flag: bool, },
  Bool { any_type_flag: bool, value: bool, },
  Number { any_type_flag: bool, value: f64, },
  String { any_type_flag: bool, value: Option<String>, },
  List { any_type_flag: bool, value: Vec<PropertyTree>, },
  Dictionary { any_type_flag: bool, value: HashMap<String, PropertyTree>, },
}
impl MapReadWrite for PropertyTree {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> {
    let typ = r.stream.read_u8()?;
    let any_type_flag = r.stream.read_bool()?;
    match typ {
      0 => Ok(PropertyTree::Nothing { any_type_flag, }),
      1 => Ok(PropertyTree::Bool { any_type_flag, value:  r.stream.read_bool()?, }),
      2 => Ok(PropertyTree::Number { any_type_flag, value:  r.stream.read_f64()?, }),
      3 => Ok(PropertyTree::String { any_type_flag, value:  r.stream.read_immutable_string()?, }),
      4 => {
        let len = r.stream.read_u32()?;
        let mut value = Vec::new();
        for _ in 0..len {
          let name = r.stream.read_immutable_string()?;
          if name.is_some() { return Err(r.stream.error_at(format!("Unknown PropertyTree List contains non-null name {}", name.as_ref().unwrap()), 1 + name.unwrap().len() as u64)) }
          value.push(PropertyTree::map_read(r)?);
        }
        Ok(PropertyTree::List { any_type_flag, value, })
      },
      5 => {
        let len = r.stream.read_u32()?;
        let mut value = HashMap::new();
        for _ in 0..len {
          let name = r.stream.read_immutable_string()?;
          if name.is_none() { return Err(r.stream.error_at(format!("Unknown PropertyTree Dict contains null name"), 1)) }
          value.insert(name.unwrap(), PropertyTree::map_read(r)?);
        }
        Ok(PropertyTree::Dictionary { any_type_flag, value, })
      },
      _ => Err(r.stream.error_at(format!("Unknown PropertyTree type {}", typ), 1))
    }
  }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> {
    match self {
      PropertyTree::Nothing { any_type_flag } => {
        w.stream.write_u8(0)?;
        w.stream.write_bool(*any_type_flag)
      },
      PropertyTree::Bool { any_type_flag, value } => {
        w.stream.write_u8(1)?;
        w.stream.write_bool(*any_type_flag)?;
        w.stream.write_bool(*value)
      },
      PropertyTree::Number { any_type_flag, value } => {
        w.stream.write_u8(2)?;
        w.stream.write_bool(*any_type_flag)?;
        w.stream.write_f64(*value)
      },
      PropertyTree::String { any_type_flag, value } => {
        w.stream.write_u8(3)?;
        w.stream.write_bool(*any_type_flag)?;
        w.stream.write_immutable_string(value.as_deref())
      },
      PropertyTree::List { any_type_flag, value } => {
        w.stream.write_u8(4)?;
        w.stream.write_bool(*any_type_flag)?;
        w.stream.write_u32(value.len() as u32)?;
        for v in value {
          w.stream.write_bool(true)?; // name_is_null
          v.map_write(w)?;
          }
        Ok(())
      },
      PropertyTree::Dictionary { any_type_flag, value } => {
        w.stream.write_u8(5)?;
        w.stream.write_bool(*any_type_flag)?;
        w.stream.write_u32(value.len() as u32)?;
        for (name, v) in value {
          w.stream.write_immutable_string(Some(name))?;
          v.map_write(w)?;
        }
        Ok(())
      },
    }
  }
}


#[derive(Debug, MapReadWriteStruct)]
pub struct Map {
  map_header: MapHeader,
  map_gen_settings: MapGenSettings,
  map_settings: MapSettings,
  general_random_generator: RandomGenerator,
  ai_random_generator: RandomGenerator,
  entities_random_generator: RandomGenerator,
  map_random_generator: RandomGenerator,
  triggers_random_generator: RandomGenerator,
  entity_update_paused_state: EntityUpdatePausedState,
  pub prototype_migrations: PrototypeMigrationList,

  loaded_prototype_migrations_definition: Vec<PrototypeMigrationListDefinitionMigration>,
  next_unit_number: u32,
  next_targetable_item_number: u32,
  next_script_pathfind_id: u32,
  next_unique_item_id: u32,
  next_circuit_network_number: u32,
  next_equipment_grid_id: u32,
  next_unique_translation_request_id: u64,
  map_mod_settings: MapModSettings,
  train_manager: TrainManager,
  force_manager: ForceManager,
  #[assert_eq(0)] circuit_networks: u32,  // Vec<CircuitNetwork>,
  pollution_statistics: PollutionStatistics,
  #[assert_eq(0)] entity_tags: u32,  // Vec<(u32, String)>
  difficulty_specifications: DifficultySpecifications,
  extra_script_data: ExtraScriptData,
  script_rendering: ScriptRendering,
  electric_network_manager: ElectricNetworkManager,
  fluid_manager: FluidManager,
  heat_buffer_manager: HeatBufferManager,
  #[assert_eq(0)] extra_script_data_inventories: u32, // Vec<...>,
  linked_inventories: [LinkedInventories; 3],  // length determined by number of forces
  #[vec_u16] tiles_need_correction: Vec<u8>,
  #[vec_u32] surfaces: Vec<Surface>,
  transport_line_manager: TransportLineManager,
  surface_delete_requests: Vec<SurfaceIndex>,
  console_command_used: bool,
  editor_used: bool,
  tutorial_triggers_enabled: bool,
  last_tick_warned_about_console_command_disabling_achievements: i32,
  last_tick_warned_about_editor_disabling_achievements: i32,
  last_tick_warned_about_cheat_disabling_achievements: i32,
  is_loaded_in_multiplayer: bool,
  remove_all_players: bool,
  #[assert_eq(0)] players: u32,  // Vec<Player>
  #[assert_eq(0)] fake_players: u8,  // Vec<Player>
  #[assert_eq(0)] applied_migrations: u8,  // Vec<Migration>
  game_speed_paused: bool,
  game_speed: f64,
  shared_achievement_stats: AchievementStats,
  blueprint_library: BlueprintLibrary,
  mute_programmable_speaker: bool,
  draw_resource_selection: bool,
  enemy_has_vision_on_mines: bool,
  dispatched_initial_chunks: bool,
  permission_groups: PermissionGroups,
  history: ScenarioHistory,
  saved_special_items: MapSavedSpecialItems,
  autosave_enabled: bool,
  save_helpers: SaveHelpers,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct SaveHelpers {
  unknown: [u8; 53]
}

#[derive(Debug, MapReadWriteStruct)]
pub struct MapSavedSpecialItems {
  #[assert_eq(0)] saved_special_items: u32,  // Vec<>
  #[assert_eq(0)] map_ids_to_ref_counts: u8,  // Vec<>
  #[assert_eq(0)] map_ids_to_keepalive_ticks: u8,  // Vec<>
  next_id: u32,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ScenarioHistory {
  steps: Vec<ScenarioHistoryStep>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ScenarioHistoryStep {
  changes: Vec<ScenarioHistoryChangeItem>,
}

#[derive(Debug)]
enum ScenarioHistoryChangeItem {
  VersionChanged(ApplicationVersion),
  ModAdded(ModId),
  ModRemoved(ModId),
  ModUpdated(ModId),
}
impl MapReadWrite for ScenarioHistoryChangeItem {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    match u8::map_read(input)? {
      0 => Ok(ScenarioHistoryChangeItem::VersionChanged(ApplicationVersion::map_read(input)?)),
      1 => Ok(ScenarioHistoryChangeItem::ModAdded(ModId::map_read(input)?)),
      2 => Ok(ScenarioHistoryChangeItem::ModRemoved(ModId::map_read(input)?)),
      3 => Ok(ScenarioHistoryChangeItem::ModUpdated(ModId::map_read(input)?)),
      x => Err(input.stream.error_at(format!("unknown ScenarioHistoryChangeItem type {}", x), 1)),
    }
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    match self {
      ScenarioHistoryChangeItem::VersionChanged(i) => { 0_u8.map_write(input)?; i.map_write(input) }
      ScenarioHistoryChangeItem::ModAdded(i) => { 1_u8.map_write(input)?; i.map_write(input) }
      ScenarioHistoryChangeItem::ModRemoved(i) => { 1_u8.map_write(input)?; i.map_write(input) }
      ScenarioHistoryChangeItem::ModUpdated(i) => { 1_u8.map_write(input)?; i.map_write(input) }
    }
  }
}

#[derive(Debug, MapReadWriteStruct)]
pub struct PermissionGroups {
  next_group_id: u32,
  #[vec_u32] groups: Vec<PermissionGroup>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct PermissionGroup {
  targeter: Option<u32>,
  id: u32,
  name: String,
  #[vec_u32] permissions: Vec<String>,
  players: Vec<u32>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct BlueprintLibrary {
  game_shelf: BlueprintShelf,
  #[assert_eq(0)] player_shelves: u32, // Vec<>
  #[assert_eq(0)] cursor_blueprint_record_ids: u8, // Vec<>
  #[assert_eq(0)] transfer_queue: u32,  // Vec<>
  unknown_blueprint: BlueprintRecordId,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct BlueprintRecordId {
  player_index: u16,
  id: u32,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct BlueprintShelf {
  player_index: u16,
  next_record_id: u32,
  timestamp: u32,
  synchronized: bool,
  #[assert_eq(0)] records: u32, // Vec<Option<>>
}

#[derive(Debug)]
pub struct AchievementStats {
  achievements: Vec<(Achievement, AchievementData)>,
}
impl MapReadWrite for AchievementStats {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    let achievements_len = u32::map_read(input)?;
    let mut achievements = vec![];
    for _ in 0..achievements_len {
      let action_type_pos = input.stream.position();
      let achievement = Achievement::map_read(input)?;
      achievements.push((achievement, AchievementData::map_read(achievement, action_type_pos, input)?));
      println!("Read achievement {:?}", achievements.last());
    }
    
    Ok(AchievementStats { achievements })
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    (self.achievements.len() as u32).map_write(input)?;
    for (achievement, achievement_data) in &self.achievements {
      achievement.map_write(input)?;
      achievement_data.map_write(input)?;
    }

    Ok(())
  }
}

#[derive(Debug, MapReadWriteTaggedUnion)]
#[tag_type(Achievement)]
pub enum AchievementData {
  YouAreDoingItRight(ConstructWithRobotsAchievement),
  LazyBastard(DontCraftManuallyAchievement),
  SteamAllTheWay(DontUseEntityInEnergyProductionAchievement),
  RainingBullets(DontBuildEntityAchievement),
  LogisticNetworkEmbargo(DontBuildEntityAchievement),
}

#[derive(Debug, MapReadWriteStruct)]
pub struct DontBuildEntityAchievement {
  build: u32,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct DontUseEntityInEnergyProductionAchievement {
  produced: f64,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct DontCraftManuallyAchievement {
  crafted: f64,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ConstructWithRobotsAchievement {
  constructed_with_robots: u32,
  constructed_manually: u32,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct TransportLineManager {
  next_remerge_tie_breaker: u32,
  #[assert_eq(0)] transport_line_records: u32,  // Vec<>
}

#[derive(Debug, MapReadWriteStruct)]
pub struct MapHeader {
  update_tick: u32,
  entity_tick: u32,
  ticks_played: u32,
}

type MapGenSize = f32;

#[derive(Debug, MapReadWriteStruct)]
pub struct MapGenSettings {
  segmentation: MapGenSize,
  water_size: MapGenSize,
  autoplace_controls: Vec<(String, FrequencySizeRichness)>,
  autoplace_settings_per_type: Vec<(String, AutoplaceSettings)>,
  default_enable_all_autoplace_controls: bool,
  random_seed: u32,
  width: u32,
  height: u32,
  area_to_generate_at_start: BoundingBox,
  starting_area_size: MapGenSize,
  peaceful_mode: bool,
  starting_points: Vec<MapPosition>,
  property_expression_names: Vec<(String, String)>,
  cliff_placement_settings: CliffPlacementSettings,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct FrequencySizeRichness {
  frequency: MapGenSize,
  size: MapGenSize,
  richness: MapGenSize,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct AutoplaceSettings {
  treat_missing_as_default: bool,
  settings: Vec<(String, FrequencySizeRichness)>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct BoundingBox {
  pub left_top: MapPosition,
  pub right_bottom: MapPosition,
  pub orientation: VectorOrientation,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct VectorOrientation {
  pub x: i16,
  pub y: i16,
}

type FixedPoint32_8 = i32;  // *1/256, .5 rounded away from 0

#[derive(Clone, Debug, Default)]
pub struct MapPosition {
  pub x: FixedPoint32_8,
  pub y: FixedPoint32_8,
  pub deserialized_relative: bool,
}
impl MapReadWrite for MapPosition {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    let dx = input.stream.read_i16()?;

    let map_position = if dx == 0x7fff {
      let x = input.stream.read_i32()?;
      let y = input.stream.read_i32()?;
      MapPosition { x, y, deserialized_relative: false }
    } else {
      let dy = input.stream.read_i16()?;
      MapPosition { x: input.last_loaded_position.x + dx as i32, y: input.last_loaded_position.y + dy as i32, deserialized_relative: true }
    };
    input.last_loaded_position = map_position.clone();
    Ok(map_position)
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    if self.deserialized_relative {
      let dx = self.x - input.last_saved_position.x;
      let dy = self.y - input.last_saved_position.y;
      assert!(dx.abs() < 0x7ffe && dy.abs() < 0x7ffe);  // based on MapPosition::saveInternal
      input.stream.write_i16(dx as i16)?;
      input.stream.write_i16(dy as i16)?;
    } else {
      input.stream.write_i16(0x7fff)?;
      input.stream.write_i32(self.x)?;
      input.stream.write_i32(self.y)?;
    }
    input.last_saved_position = self.clone();
    Ok(())
  }
}

#[derive(Debug, MapReadWriteStruct)]
pub struct CliffPlacementSettings {
  cliff_name: String,
  cliff_elevation0: f32,
  cliff_elevation_interval: f32,
  richness: f32,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct MapSettings {
  pollution_settings: PollutionSettings,
  steering_settings: SteeringSettings,
  enemy_evolution_settings: EnemyEvolutionSettings,
  enemy_expansion_settings: EnemyExpansionSettings,
  unit_group_settings: UnitGroupSettings,
  path_finder_settings: PathFinderSettings,
  max_failed_behavior_count: u32,
  difficulty_settings: DifficultySettings,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct PollutionSettings {  // falls back to global settings if empty
  enabled: Option<bool>,
  diffusion_ratio: Option<f64>,
  min_to_diffuse: Option<f64>,
  ageing: Option<f64>,
  expected_max_per_chunk: Option<f64>,
  min_to_show_per_chunk: Option<f64>,
  min_pollution_to_damage_trees: Option<f64>,
  pollution_with_max_forest_damage: Option<f64>,
  pollution_per_tree_damage: Option<f64>,
  pollution_restored_per_tree_damage: Option<f64>,
  max_pollution_to_restore_trees: Option<f64>,
  enemy_attack_pollution_consumption_modifier: Option<f64>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct SteeringSettings {
  default_settings: StateSteeringSettings,
  moving_settings: StateSteeringSettings,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct StateSteeringSettings {  // falls back to global settings if empty
  radius: Option<f64>,
  separation_factor: Option<f64>,
  separation_force: Option<f64>,
  force_unit_fuzzy_goto_behavior: Option<bool>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct EnemyEvolutionSettings {
  enabled: Option<bool>,
  time_factor: Option<f64>,
  destroy_factor: Option<f64>,
  pollution_factor: Option<f64>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct EnemyExpansionSettings {
  enabled: Option<bool>,
  max_expansion_distance: Option<u32>,
  friendly_base_influence_radius: Option<u32>,
  enemy_building_influence_radius: Option<u32>,
  building_coefficient: Option<f64>,
  other_base_coefficient: Option<f64>,
  neighbouring_chunk_coefficient: Option<f64>,
  neighbouring_base_chunk_coefficient: Option<f64>,
  max_colliding_tiles_coefficient: Option<f64>,
  settler_group_min_size: Option<u32>,
  settler_group_max_size: Option<u32>,
  min_expansion_cooldown: Option<u32>,
  max_expansion_cooldown: Option<u32>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct UnitGroupSettings {
  min_group_gathering_time: Option<u32>,
  max_group_gathering_time: Option<u32>,
  max_wait_time_for_late_members: Option<u32>,
  max_group_radius: Option<f64>,
  min_group_radius: Option<f64>,
  max_member_speedup_when_behind: Option<f64>,
  max_member_slowdown_when_ahead: Option<f64>,
  max_group_slowdown_factor: Option<f64>,
  max_group_member_fallback_factor: Option<f64>,
  member_disown_distance: Option<f64>,
  tick_tolerance_when_member_arrives: Option<u32>,
  max_gathering_unit_groups: Option<u32>,
  max_unit_group_size: Option<u32>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct PathFinderSettings {
  fwd_2_bwd_ratio: Option<u32>,
  goal_pressure_ratio: Option<f64>,
  use_path_cache: Option<bool>,
  max_steps_worked_per_tick: Option<f64>,
  max_work_done_per_tick: Option<u32>,
  short_cache_size: Option<u32>,
  long_cache_size: Option<u32>,
  short_cache_min_cacheable_distance: Option<f64>,
  short_cache_min_algo_steps_to_cache: Option<u32>,
  long_cache_min_cacheable_distance: Option<f64>,
  cache_max_connect_to_cache_steps_multiplier: Option<u32>,
  cache_accept_path_start_distance_ratio: Option<f64>,
  cache_accept_path_end_distance_ratio: Option<f64>,
  negative_cache_accept_path_start_distance_ratio: Option<f64>,
  negative_cache_accept_path_end_distance_ratio: Option<f64>,
  cache_path_start_distance_rating_multiplier: Option<f64>,
  cache_path_end_distance_rating_multiplier: Option<f64>,
  stale_enemy_with_same_destination_collision_penalty: Option<f64>,
  ignore_moving_enemy_collision_distance: Option<f64>,
  enemy_with_different_destination_collision_penalty: Option<f64>,
  general_entity_collision_penalty: Option<f64>,
  general_entity_subsequent_collision_penalty: Option<f64>,
  extended_collision_penalty: Option<f64>,
  max_clients_to_accept_any_new_request: Option<u32>,
  max_clients_to_accept_short_new_request: Option<u32>,
  direct_distance_to_consider_short_request: Option<u32>,
  short_request_max_steps: Option<u32>,
  short_request_ratio: Option<f64>,
  min_steps_to_check_path_find_termination: Option<u32>,
  start_to_goal_cost_multiplier_to_terminate_path_find: Option<f64>,
  overload_levels: Option<Vec<u32>>,
  overload_multipliers: Option<Vec<f64>>,
  negative_path_cache_delay_interval: Option<u32>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct DifficultySettings {
  recipe_difficulty: DifficultySettingsValue,
  technology_difficulty: DifficultySettingsValue,
  technology_price_multiplier: f64,
  research_queue_setting: ResearchQueueSetting,
}

// Source: disassembly DifficultySettings::Value
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, MapReadWriteEnumU8)]
pub enum DifficultySettingsValue {
  Normal = 0,
  Expensive = 1,
}

// Source: ResearchQueueSetting
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, MapReadWriteEnumU8)]
pub enum ResearchQueueSetting {
  Always = 0,
  AfterVictory = 1,
  Never = 2,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct RandomGenerator {
  seed1: u32,
  seed2: u32,
  seed3: u32,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct EntityUpdatePausedState {
  paused: bool,
  ticks_to_run: u32,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct PrototypeMigrationList {
  pub custom_input_id_migrations: ActiveMigrations<u16>,
  pub equipment_grid_id_migrations: ActiveMigrations<u8>,
  pub item_id_migrations: ActiveMigrations<u16>,
  pub tile_id_migrations: ActiveMigrations<u8>,
  pub decorative_id_migrations: ActiveMigrations<u8>,
  pub technology_id_migrations: ActiveMigrations<u16>,
  pub entity_id_migrations: ActiveMigrations<u16>,
  pub particle_id_migrations: ActiveMigrations<u16>,
  pub recipe_category_id_migrations: ActiveMigrations<u16>,
  pub item_sub_group_id_migrations: ActiveMigrations<u16>,
  pub item_group_id_migrations: ActiveMigrations<u8>,
  pub fluid_id_migrations: ActiveMigrations<u16>,
  pub virtual_signal_id_migrations: ActiveMigrations<u16>,
  pub ammo_category_id_migrations: ActiveMigrations<u8>,
  pub fuel_category_id_migrations: ActiveMigrations<u8>,
  pub resource_category_id_migrations: ActiveMigrations<u8>,
  pub equipment_id_migrations: ActiveMigrations<u16>,
  pub noise_layer_id_migrations: ActiveMigrations<u16>,
  pub named_noise_expression_id_migrations: ActiveMigrations<u32>,
  pub autoplace_control_id_migrations: ActiveMigrations<u8>,
  pub damage_type_id_migrations: ActiveMigrations<u8>,
  pub recipe_id_migrations: ActiveMigrations<u16>,
  pub achievement_id_migrations: ActiveMigrations<u16>,
  pub module_category_id_migrations: ActiveMigrations<u8>,
  pub equipment_category_id_migrations: ActiveMigrations<u8>,
  pub mod_settings_id_migrations: ActiveMigrations<u16>,
  pub trivial_smoke_id_migrations: ActiveMigrations<u8>,
  pub shortcut_id_migrations: ActiveMigrations<u16>,
}

#[derive(Clone, Debug)]
pub struct ActiveMigrations<V> {
  pub mappings: Vec<(String, Vec<(String, V)>)>,
}
impl<V: MapReadWrite + TryFrom<usize,Error=TryFromIntError>> MapReadWrite for ActiveMigrations<V> where u32: From<V> {
  fn map_read<R: BufRead + Seek>(r: &mut MapDeserialiser<R>) -> Result<Self> {
    let len = V::map_read(r)?;
    let mut mappings = Vec::new();
    for _ in 0..u32::from(len) {
      let mapping_name = String::map_read(r)?;
      let mapping_len = V::map_read(r)?;
      let mut mapping = Vec::new();
      for _ in 0..u32::from(mapping_len) {
        let name = String::map_read(r)?;
        let value = V::map_read(r)?;
        mapping.push((name, value));
      }
      mappings.push((mapping_name, mapping));
    }
    Ok(ActiveMigrations { mappings })
  }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> {
    V::try_from(self.mappings.len()).unwrap().map_write(w)?;
    for (mapping_name, mapping) in &self.mappings {
      mapping_name.map_write(w)?;
      V::try_from(mapping.len()).unwrap().map_write(w)?;
      for (name, value) in mapping {
        name.map_write(w)?;
        value.map_write(w)?;
      }
    }
    Ok(())
  }
}

#[derive(Debug, MapReadWriteStruct)]
pub struct PrototypeMigrationListDefinitionMigration {
  mod_name: String,
  name: String,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct MapModSettings {
  #[assert_eq(0)] runtime_global_settings: u32,  // Vec<ModSetting>,
  #[assert_eq(0)] runtime_per_user_settings: u32,  // Vec<ModSetting>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct Color {
  r: f32,
  g: f32,
  b: f32,
  a: f32,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct TrainManager {
  next_train_id: u32,
  next_rail_segment: u32,
  #[assert_eq(0)] rail_segments: u32,  // Vec<RailSegment>,
  #[assert_eq(0)] trains: u32,  // Vec<Train>,
  #[assert_eq(0)] stops_enable_changed_this_tick: u8,  // Vec<Targeter>,
  flagged_for_limit_logic_update: bool,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ForceManager {
  #[assert_eq(3)] force_data_list_len: u32,
  force_data_list: [ForceData; 3],
  #[assert_eq(0)] forces_to_delete: u32,  // Vec<(ForceID, ForceID)>
}

type ForceId = u8;
type ForceSet = u64;

#[derive(Debug, MapReadWriteStruct)]
pub struct ForceData {
  id: ForceId,
  name: String,
  disable_all_by_default: bool,
  friendly_fire_enabled: bool,
  share_chart: bool,
  evolution_factor_data: EvolutionFactorData,
  custom_prototypes: CustomPrototypes,
  research_enabled: bool,
  research_manager: ResearchManager,
  #[vec_u32] logistic_managers: Vec<Option<LogisticManager>>,
  #[vec_u32] construction_managers: Vec<Option<ConstructionManager>>,
  ammo_damage_modifiers: Vec<f64>,
  gun_speed_modifiers: Vec<f64>,
  turret_attack_modifiers: Vec<f64>,
  disabled_hand_crafting_recipes: Vec<u8>,
  worker_robots_speed_modifier: f64,
  worker_robots_battery_modifier: f64,
  worker_robots_storage_bonus: f64,
  laboratory_speed_modifier: f64,
  laboratory_productivity_bonus: f64,
  following_robots_lifetime_modifier: f64,
  manual_crafting_speed_modifier: f64,
  manual_mining_speed_modifier: f64,
  running_speed_modifier: f64,
  artillery_range_modifier: f64,
  build_distance_bonus: f64,
  item_drop_distance_bonus: f64,
  reach_distance_bonus: f64,
  resource_reach_distance_bonus: f64,
  item_pickup_distance_bonus: f64,
  loot_pickup_distance_bonus: f64,
  character_inventory_slot_count_bonus: f64,
  character_health_bonus: f64,
  mining_drill_productivity_bonus: f64,
  train_braking_force_bonus: f64,
  inserter_stack_size_bonus: f64,
  stack_inserter_capacity_bonus: f64,
  character_logistic_trash_slot_count: f64,
  maximum_following_robots_count: f64,
  ghost_time_to_live: f64,
  deconstruction_time_to_live: f64,
  max_successful_attempts_per_tick_per_construction_queue: f64,
  max_failed_attempts_per_tick_per_construction_queue: f64,
  zoom_to_world_enabled: bool,
  zoom_to_world_ghost_building_enabled: bool,
  zoom_to_world_blueprint_enabled: bool,
  zoom_to_world_deconstruction_planner_enabled: bool,
  zoom_to_world_upgrade_planner_enabled: bool,
  zoom_to_world_selection_tool_enabled: bool,
  character_logistic_requests: bool,
  cease_fire: ForceSet,
  friends: ForceSet,
  #[vec_u32] charts: Vec<(SurfaceIndex, Chart)>,
  #[assert_eq(0)] spawn_positions: u32,  // Vec<(SurfaceIndex, MapPosition)>
  #[assert_eq(0)] item_production_statistics: u8,  // Option<FlowStatistics>
  #[assert_eq(0)] fluid_production_statistics: u8,  // Option<FlowStatistics>
  #[assert_eq(0)] kill_count_statistics: u8,  // Option<FlowStatistics>
  build_count_statistics: Option<BuildCountStatistics>,
  custom_color: Color,
  rockets_launched: u32,
  #[assert_eq(0)] items_launched: u8,  // Vec<(u16, u32)>
}

#[derive(Debug, MapReadWriteStruct)]
pub struct EvolutionFactorData {
  evolution_factor: f64,
  evolution_increased_by_pollution: f64,
  evolution_increased_by_pollution_this_tick: f64,
  evolution_increased_by_time: f64,
  evolution_increased_by_time_this_tick: f64,
  evolution_increased_by_killing_spawners: f64,
  evolution_increased_by_killing_spawners_this_tick: f64,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct CustomPrototypes {
  #[vec_u16] recipes: Vec<CustomPrototypesOption<Recipe>>,
  #[vec_u16] technologies: Vec<CustomPrototypesOption<Technology>>,
}

#[derive(Debug)]
pub struct CustomPrototypesOption<T: MapReadWrite> {
  option: Option<(u16, T)>,
}
impl<T: MapReadWrite> MapReadWrite for CustomPrototypesOption<T> {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    let id = u16::map_read(input)?;
    Ok(CustomPrototypesOption {
      option: if id == 0 { None } else { Some((id, T::map_read(input)?)) }
    })
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    if let Some((id, recipe)) = &self.option {
      id.map_write(input)?;
      recipe.map_write(input)
    } else {
      0_u16.map_write(input)
    }
  }
}

#[derive(Debug, MapReadWriteStruct)]
pub struct Recipe {
  category_id: u16,
  energy_required: f64,
  enabled: bool,
  disabled_through_script: bool,
  hidden_from_flow_stats: bool,
  #[vec_u32] ingredients: Vec<Ingredient>,
  #[vec_u32] products: Vec<Product>,
  unknown_u32: u32,
}

#[derive(Debug)]
pub enum Ingredient {
  Item(ItemIngredient),
  Fluid(FluidIngredient),
}
impl MapReadWrite for Ingredient {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    match u8::map_read(input)? {
      0 => Ok(Ingredient::Item(ItemIngredient::map_read(input)?)),
      1 => Ok(Ingredient::Fluid(FluidIngredient::map_read(input)?)),
      x => Err(input.stream.error_at(format!("unknown ingredient type {}", x), 1)),
    }
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    match self {
      Ingredient::Item(i) => { 0_u8.map_write(input)?; i.map_write(input) }
      Ingredient::Fluid(i) => { 1_u8.map_write(input)?; i.map_write(input) }
    }
  }
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ItemIngredient {
  item_id: u16,
  count: u16,
  catalyst_count: u16,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct FluidIngredient {
  fluid_id: u16,
  count: f64,
  catalyst_count: f64,
  minimum_temperature: f64,
  maximum_temperature: f64,
  fluid_box_index: u32,
}

#[derive(Debug)]
pub enum Product {
  Item(ItemProduct),
  Fluid(FluidProduct),
}
impl MapReadWrite for Product {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    match u8::map_read(input)? {
      0 => Ok(Product::Item(ItemProduct::map_read(input)?)),
      1 => Ok(Product::Fluid(FluidProduct::map_read(input)?)),
      x => Err(input.stream.error_at(format!("unknown product type {}", x), 1)),
    }
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    match self {
      Product::Item(i) => { 0_u8.map_write(input)?; i.map_write(input) }
      Product::Fluid(i) => { 1_u8.map_write(input)?; i.map_write(input) }
    }
  }
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ItemProduct {
  item_id: u16,
  probability: f64,
  count_min: u16,
  count_max: u16,
  catalyst_count: u16,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct FluidProduct {
  fluid_id: u16,
  probability: f64,
  amount_min: f64,
  amount_max: f64,
  catalyst_amount: f64,
  temperature: f64,
  has_temperature: bool,
  fluid_box_index: u32,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct Technology {
  #[space_optimized] research_unit_count: u64,
  research_unit_energy_needed: f64,
  enabled: bool,
  visible_when_disabled: bool,
  #[space_optimized] research_count: u32,
  #[vec_u32] prerequisite_ids: Vec<u16>,
  #[vec_u32] research_unit_ingredients: Vec<(u8, ItemIngredient)>,
  #[vec_u32] effects: Vec<Modifier>,
  num_research_units_count_formula: String,
}

#[derive(Debug)]
pub enum Modifier {
  // InserterStackSizeBonus	0	
  // LaboratorySpeed	1	
  // DummyCharacterLogisticSlots	2	
  // CharacterLogisticTrashSlots	3	
  // Unused	4	
  // MaximumFollowingRobotsCount	5	
  // WorkerRobotsSpeed	6	
  // WorkerRobotsStorage	7	
  // GhostTimeToLive	8	
  // TurretAttack	9	
  // AmmoDamage	10	
  // GiveItem	11	
  // GunSpeed	12	
  // UnlockRecipe	13	
  // CharacterCraftingSpeed	14	
  // CharacterMiningSpeed	15	
  // CharacterRunningSpeed	16	
  // CharacterBuildDistance	17	
  // CharacterItemDropDistance	18	
  // CharacterReachDistance	19	
  // CharacterResourceReachDistance	20	
  // CharacterItemPickupDistance	21	
  // CharacterLootPickupDistance	22	
  // CharacterInventorySlotsBonus	23	
  // DeconstructionTimeToLive	24	
  // CharacterHealthBonus	25	
  // StackInserterCapacityBonus	26	
  // MiningDrillProductivityBonus	28	
  // TrainBrakingForceBonus	29	
  // ZoomToWorldEnabled	30	
  // ZoomToWorldGhostBuildingEnabled	31	
  // ZoomToWorldBlueprintEnabled	32	
  // ZoomToWorldDeconstructionPlannerEnabled	33	
  // ZoomToWorldSelectionToolEnabled	34	
  // Nothing	35	
  // WorkerRobotsBattery	36	
  // LaboratoryProductivity	37	
  // FollowingRobotsLifetime	38	
  // MaxSuccessfulAttempsPerTickPerConstructionQueue	39	
  // MaxFailedAttemptsPerTickPerConstructionQueue	40	
  // ArtilleryRange	41	
  // ZoomToWorldUpgradePlannerEnabled	42	
  // CharacterAdditionalMiningCategories	43	
  // CharacterLogisticRequests	44	
  // Last	45	
  InserterStackSizeBonus(SimpleModifier),
  LaboratorySpeed(SimpleModifier),
  CharacterLogisticTrashSlots(SimpleModifier),
  MaximumFollowingRobotsCount(SimpleModifier),
  WorkerRobotsSpeed(SimpleModifier),
  WorkerRobotsStorage(SimpleModifier),
  GhostTimeToLive(SimpleModifier),
  TurretAttack(TurretAttackModifier),
  AmmoDamage(GunModifier),
  GunSpeed(GunModifier),
  UnlockRecipe(UnlockRecipeModifier),
  CharacterMiningSpeed(SimpleModifier),
  CharacterInventorySlotsBonus(SimpleModifier),
  StackInserterCapacityBonus(SimpleModifier),
  MiningDrillProductivityBonus(SimpleModifier),
  TrainBrakingForceBonus(SimpleModifier),
  ArtilleryRange(SimpleModifier),
  CharacterLogisticRequests(BoolModifier),
}
impl MapReadWrite for Modifier {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    match u8::map_read(input)? {
      // 0xB => GiveItemModifier
      // 0x23 => NothingModifier
      0x00 => Ok(Modifier::InserterStackSizeBonus(SimpleModifier::map_read(input)?)),
      0x01 => Ok(Modifier::LaboratorySpeed(SimpleModifier::map_read(input)?)),
      0x03 => Ok(Modifier::CharacterLogisticTrashSlots(SimpleModifier::map_read(input)?)),
      0x05 => Ok(Modifier::MaximumFollowingRobotsCount(SimpleModifier::map_read(input)?)),
      0x06 => Ok(Modifier::WorkerRobotsSpeed(SimpleModifier::map_read(input)?)),
      0x07 => Ok(Modifier::WorkerRobotsStorage(SimpleModifier::map_read(input)?)),
      0x08 => Ok(Modifier::GhostTimeToLive(SimpleModifier::map_read(input)?)),
      0x09 => Ok(Modifier::TurretAttack(TurretAttackModifier::map_read(input)?)),
      0x0a => Ok(Modifier::AmmoDamage(GunModifier::map_read(input)?)),
      0x0c => Ok(Modifier::GunSpeed(GunModifier::map_read(input)?)),
      0x0d => Ok(Modifier::UnlockRecipe(UnlockRecipeModifier::map_read(input)?)),
      0x0f => Ok(Modifier::CharacterMiningSpeed(SimpleModifier::map_read(input)?)),
      0x17 => Ok(Modifier::CharacterInventorySlotsBonus(SimpleModifier::map_read(input)?)),
      0x1a => Ok(Modifier::StackInserterCapacityBonus(SimpleModifier::map_read(input)?)),
      0x1c => Ok(Modifier::MiningDrillProductivityBonus(SimpleModifier::map_read(input)?)),
      0x1d => Ok(Modifier::TrainBrakingForceBonus(SimpleModifier::map_read(input)?)),
      0x29 => Ok(Modifier::ArtilleryRange(SimpleModifier::map_read(input)?)),
      0x2c => Ok(Modifier::CharacterLogisticRequests(BoolModifier::map_read(input)?)),
      x => Err(input.stream.error_at(format!("unknown Modifier type {}", x), 1)),
    }
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    match self {
      Modifier::InserterStackSizeBonus(i) => { 0x00_u8.map_write(input)?; i.map_write(input) }
      Modifier::LaboratorySpeed(i) => { 0x01_u8.map_write(input)?; i.map_write(input) }
      Modifier::CharacterLogisticTrashSlots(i) => { 0x03_u8.map_write(input)?; i.map_write(input) }
      Modifier::MaximumFollowingRobotsCount(i) => { 0x05_u8.map_write(input)?; i.map_write(input) }
      Modifier::WorkerRobotsSpeed(i) => { 0x06_u8.map_write(input)?; i.map_write(input) }
      Modifier::WorkerRobotsStorage(i) => { 0x07_u8.map_write(input)?; i.map_write(input) }
      Modifier::GhostTimeToLive(i) => { 0x08_u8.map_write(input)?; i.map_write(input) }
      Modifier::TurretAttack(i) => { 0x09_u8.map_write(input)?; i.map_write(input) }
      Modifier::AmmoDamage(i) => { 0x0a_u8.map_write(input)?; i.map_write(input) }
      Modifier::GunSpeed(i) => { 0x0c_u8.map_write(input)?; i.map_write(input) }
      Modifier::UnlockRecipe(i) => { 0x0d_u8.map_write(input)?; i.map_write(input) }
      Modifier::CharacterMiningSpeed(i) => { 0x0f_u8.map_write(input)?; i.map_write(input) }
      Modifier::CharacterInventorySlotsBonus(i) => { 0x17_u8.map_write(input)?; i.map_write(input) }
      Modifier::StackInserterCapacityBonus(i) => { 0x1a_u8.map_write(input)?; i.map_write(input) }
      Modifier::MiningDrillProductivityBonus(i) => { 0x1c_u8.map_write(input)?; i.map_write(input) }
      Modifier::TrainBrakingForceBonus(i) => { 0x1d_u8.map_write(input)?; i.map_write(input) }
      Modifier::ArtilleryRange(i) => { 0x29_u8.map_write(input)?; i.map_write(input) }
      Modifier::CharacterLogisticRequests(i) => { 0x2c_u8.map_write(input)?; i.map_write(input) }
    }
  }
}

#[derive(Debug, MapReadWriteStruct)]
pub struct UnlockRecipeModifier {
  recipe_id: u16,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct SimpleModifier {
  value: f64,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct BoolModifier {
  value: bool,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct GunModifier {
  id: u8,
  amount: f64,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct TurretAttackModifier {
  entity_id: u16,
  amount: f64,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ResearchManager {
  research_progress: f64,
  research_state: ResearchState,
  technology_in_research: u16,
  previous_technology_in_research: u16,
  #[vec_u32] research_queue: Vec<u16>,
  research_queue_enabled: bool,
  switched_research_progress: Vec<(u16, f64)>,
}

// Source: ResearchState
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, MapReadWriteEnumU8)]
pub enum ResearchState {
  Researching = 0,
  ResearchFinished = 1,
  NotResearching = 2,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct LogisticManager {
  #[assert_eq(0)] logistic_network_len: u32,  // Vec<LogisticNetwork>
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ConstructionManager {
  cliff_explosive_manager: CliffExplosiveManager,
  #[assert_eq(0)] construction_areas_to_check: u8,  // Vec<SimpleBoundingBox>.
}

#[derive(Debug, MapReadWriteStruct)]
pub struct CliffExplosiveManager {
  #[assert_eq(0)] to_explode: u8,  // Vec<ToExplode>
  #[assert_eq(0)] active_jobs: u8,  // Vec<ExplosiveJob>
  #[assert_eq(0)] waiting_jobs: u8,  // Vec<ExplosiveJob>
}

#[derive(Debug, MapReadWriteStruct)]
pub struct SurfaceIndex {
  #[space_optimized] index: u32,
}

#[derive(Debug)]
pub struct Chart {
  charted_chunks: Vec<(ChunkPosition, SubChart)>,
  chart_requests_by_priority: u8,
  viewer_force: u8,
  next_custom_tag_number: u32,
}
impl MapReadWrite for Chart {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    let mut cached_colors = 0;

    let charted_chunks_len = u32::map_read(input)?;
    let mut charted_chunks = vec![];
    for _ in 0..charted_chunks_len {
      let chunk_position = ChunkPosition::map_read(input)?;
      let chart_tags = u32::map_read(input)?;
      assert_eq!(chart_tags, 0);
      let custom_chart_tags = u8::map_read(input)?;
      assert_eq!(custom_chart_tags, 0);

      let mut colored_tiles = 0;
      let mut pixels_commands = vec![];
      while colored_tiles < 0x20 * 0x20 {
        let index = u8::map_read(input)?;
        assert!(index as usize <= cached_colors);
        if index < cached_colors as u8 {
          let len = u8::map_read(input)?;
          pixels_commands.push(SubChartPixelCommand::ExistingPaletteColor { index, len });
          colored_tiles += 1 + len as usize;
        } else {
          let r = u8::map_read(input)?;
          let g = u8::map_read(input)?;
          let b = u8::map_read(input)?;
          let len = u8::map_read(input)?;
          pixels_commands.push(SubChartPixelCommand::NewPaletteColor { index, r, g, b, len });
          colored_tiles += 1 + len as usize;
          if index < 0xff {
            cached_colors += 1;
          }
        }
      }
      assert!(colored_tiles == 0x20 * 0x20);
      let unknown_u32 = u32::map_read(input)?;
      charted_chunks.push((chunk_position, SubChart { chart_tags, custom_chart_tags, pixels_commands, unknown_u32 }));
    }

    let chart_requests_by_priority = u8::map_read(input)?;
    assert_eq!(chart_requests_by_priority, 0);
    let viewer_force = u8::map_read(input)?;
    let next_custom_tag_number = u32::map_read(input)?;

    Ok(Chart { charted_chunks, chart_requests_by_priority, viewer_force, next_custom_tag_number })
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    (self.charted_chunks.len() as u32).map_write(input)?;
    for (chunk_position, sub_chunk) in &self.charted_chunks {
      chunk_position.map_write(input)?;
      sub_chunk.chart_tags.map_write(input)?;
      sub_chunk.custom_chart_tags.map_write(input)?;
      for cmd in &sub_chunk.pixels_commands {
        match cmd {
          &SubChartPixelCommand::NewPaletteColor { index, r, g, b, len } => {
            index.map_write(input)?;
            r.map_write(input)?;
            g.map_write(input)?;
            b.map_write(input)?;
            len.map_write(input)?;
          },
          &SubChartPixelCommand::ExistingPaletteColor { index, len } => {
            index.map_write(input)?;
            len.map_write(input)?;
          },
        }
      }
      sub_chunk.unknown_u32.map_write(input)?;
    }

    self.chart_requests_by_priority.map_write(input)?;
    self.viewer_force.map_write(input)?;
    self.next_custom_tag_number.map_write(input)?;
    
    Ok(())
  }
}

#[derive(Debug)]
pub struct SubChart {
  chart_tags: u32,  // Vec<>
  custom_chart_tags: u8,  // Vec<>
  pixels_commands: Vec<SubChartPixelCommand>,
  unknown_u32: u32,
}

#[derive(Debug)]
pub enum SubChartPixelCommand {
  NewPaletteColor { index: u8, r: u8, g: u8, b: u8, len: u8 },
  ExistingPaletteColor { index: u8, len: u8 },
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ChunkPosition {
  x: i32,
  y: i32,
}

#[derive(Debug, MapReadWriteStruct)]
struct BuildCountStatistics {
  precision: [BuildCountStatisticsPrecision; 8],
  input_running_counts: Vec<(u16, u64)>,
  output_running_counts: Vec<(u16, u64)>,
}

#[derive(Debug, MapReadWriteStruct)]
struct BuildCountStatisticsPrecision {
  #[vec_u32] input_elements: Vec<BuildCountStatisticsPrecisionElements>,
  #[vec_u32] output_elements: Vec<BuildCountStatisticsPrecisionElements>,
}

#[derive(Debug, MapReadWriteStruct)]
struct BuildCountStatisticsPrecisionElements {
  #[vec_u16] elements: Vec<f32>,
  f: f64,
}

#[derive(Debug, MapReadWriteStruct)]
struct PollutionStatistics {
  precision: [BuildCountStatisticsPrecision; 8],
  input_running_counts: Vec<(u16, f64)>,
  output_running_counts: Vec<(u16, f64)>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct DifficultySpecifications {
  #[vec_u32] data: Vec<(u32, u8)>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ExtraScriptData {
  next_rectangle_id: u32,
  next_position_id: u32,
  #[assert_eq(0)] rectangles_by_surface: u8,  // Vec<(SurfaceIndex, Vec<...>)
  #[assert_eq(0)] positions_by_surface: u8,  // Vec<(SurfaceIndex, Vec<...>)
  entity_destroyed_hooks: EntityDestroyedHooks,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct EntityDestroyedHooks {
  next_registration_id: u64,
  #[assert_eq(0)] to_event: u32,  // Vec<...>
  #[assert_eq(0)] registrations: u32,  // Vec<...>
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ScriptRendering {
  next_id: u64,
  #[assert_eq(0)] object_mapping: u8,  // Vec<...>
  #[assert_eq(0)] updateable_objects: u64,  // List<...>
  #[assert_eq(0)] static_objects: u64,  // List<...>
  #[assert_eq(0)] objects_to_destroy: u8,  // Vec<...>
  #[assert_eq(0)] id_to_mod: u32,  // Map<...>
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ElectricNetworkManager {
  next_electric_subnetwork_index: u32,
  #[assert_eq(0)] electric_network_list: u32,  // List<ElectricNetwork>
}

#[derive(Debug, MapReadWriteStruct)]
pub struct FluidManager {
  #[assert_eq(0)] systems: u32,  // Vec<FluidSystem>
  next_fluid_system_id: u32,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct HeatBufferManager {
  unsorted_buffers: u32,
  #[assert_eq(0)] buffer_groups: u32,  // Vec<...>
  #[assert_eq(0)] saved_unsorted_connections: u8,  // Vec<...>
}

#[derive(Debug, MapReadWriteStruct)]
pub struct LinkedInventories {
  default_inventories_by_prototype: Vec<LinkedInventory>,
  #[assert_eq(0)] inventories: u8,  // Vec<>
  #[assert_eq(0)] to_check: u8,  // Vec<>
}

#[derive(Debug, MapReadWriteStruct)]
pub struct LinkedInventory {
  #[assert_eq(159)] entity_id: u16,  // doesn't parse the rest if zero
  link_id: u32,
  #[assert_eq(0)] inventory_type: u8,  // WithBar
  with_bar: InventoryWithBar,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct InventoryWithBar {
  inventory: Inventory,
  bar: u16,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct Inventory {
  #[vec_u16] data: Vec<u16>,
  hand_position: i16,
}

#[derive(Debug)]
pub struct Surface {
  index: SurfaceIndex,
  active_entities_serialisation_helper: u32,
  chunks: Vec<Chunk>,
  compiled_map_gen_settings: CompiledMapGenSettings,
  path_finders: u32,  // Vec<>
  commanders: Vec<Option<Commander>>,
  map_generation_manager: MapGenerationManager,
  active_chunks: Vec<ChunkPosition>,
  polluted_chunks: [Vec<ChunkPosition>; 0x40],
  name: String,
  deletable: bool,
  show_clouds: bool,
  clean_surface_parameters: Option<bool>,
  day_time: DayTime,
  wind: Wind,
  chunk_delete_requests: Vec<ChunkPosition>,
  brightness_visual_weights: Color,
}
impl MapReadWrite for Surface {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    let index = SurfaceIndex::map_read(input)?;
    let active_entities_serialisation_helper = u32::map_read(input)?; assert_eq!(active_entities_serialisation_helper, 0);
    let chunks_len = u32::map_read(input)?;
    let mut chunks: Vec<Chunk> = (0..chunks_len).map(|_| Chunk::initial_read(input)).collect::<Result<_>>()?;
    let compiled_map_gen_settings = CompiledMapGenSettings::map_read(input)?;
    let path_finders = u32::map_read(input)?; assert_eq!(path_finders, 0);
    let commanders = map_read_vec_u32::<_, Option<Commander>>(input)?;
    let map_generation_manager = MapGenerationManager::map_read(input)?;
    
    for i in 0..chunks_len as usize {
      chunks[i].load(input)?;
    }

    let active_chunks = map_read_vec_u32(input)?;
    let polluted_chunks = (0..0x40).map(|_| map_read_vec_u32(input)).collect::<Result<Vec<Vec<ChunkPosition>>>>()?.try_into().unwrap();

    assert_eq!(u32::map_read(input)?, 0); // ParticleSurface
    let name = String::map_read(input)?;
    assert_eq!(u32::map_read(input)?, 0); // HiddenTiles
    let deletable = bool::map_read(input)?;
    let show_clouds = bool::map_read(input)?;
    let clean_surface_parameters = Option::map_read(input)?;
    let day_time = DayTime::map_read(input)?;
    let wind = Wind::map_read(input)?;
    let chunk_delete_requests = Vec::map_read(input)?;
    let brightness_visual_weights = Color::map_read(input)?;

    Ok(Surface { index, active_entities_serialisation_helper, chunks, compiled_map_gen_settings, path_finders, commanders, map_generation_manager, active_chunks, polluted_chunks, name, deletable, show_clouds, clean_surface_parameters, day_time, wind, chunk_delete_requests, brightness_visual_weights })
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    self.index.map_write(input)?;
    self.active_entities_serialisation_helper.map_write(input)?;
    let chunks_len = self.chunks.len() as u32;
    chunks_len.map_write(input)?;
    self.chunks.iter().map(|c| c.initial_write(input)).collect::<Result<_>>()?;
    self.compiled_map_gen_settings.map_write(input)?;
    self.path_finders.map_write(input)?;
    map_write_vec_u32(&self.commanders, input)?;
    self.map_generation_manager.map_write(input)?;

    for i in 0..chunks_len as usize {
      self.chunks[i].save(input)?;
    }

    map_write_vec_u32(&self.active_chunks, input)?;
    for p in &self.polluted_chunks { map_write_vec_u32(p, input)? }

    0_u32.map_write(input)?; // ParticleSurface
    self.name.map_write(input)?;
    0_u32.map_write(input)?; // HiddenTiles
    self.deletable.map_write(input)?;
    self.show_clouds.map_write(input)?;
    self.clean_surface_parameters.map_write(input)?;
    self.day_time.map_write(input)?;
    self.wind.map_write(input)?;
    self.chunk_delete_requests.map_write(input)?;
    self.brightness_visual_weights.map_write(input)?;

    Ok(())
  }
}

#[derive(Debug, MapReadWriteStruct)]
pub struct Wind {
  speed: f64,
  orientation: f32,
  orientation_change: f64,
  cumulative_offset: Vector,
  clouds_offset: Vector,
  cumulative_offset_history: [Vector; 120],
}

#[derive(Debug, MapReadWriteStruct)]
pub struct Vector {
  x: f64,
  y: f64,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct DayTime {
  dusk: f64,
  dawn: f64,
  evening: f64,
  morning: f64,
  ticks_per_day: f64,
  position: f64,
  frozen: bool,
  max_darkness: f64,
  solar_power_multiplier: f64,
}

#[derive(Debug)]
pub struct Chunk {
  position: ChunkPosition,
  generated_status: u8,  // Enum
  military_targets_len: u8,
  active_entities_serialisation_helper: u32,
  planned_update_counts_to_be_loaded: Vec<u32>,
  active_when_enemy_is_around: u32,

  tiles: Vec<Tile>,
  entities_to_be_inserted_before_setup: Vec<(Entity, EntityData)>,
  tick_of_optional_activation: u32,
  tick_of_last_change_that_could_affect_charting: u32,
  pollution: f64,
}
impl Chunk {
  fn initial_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    let position = ChunkPosition::map_read(input)?;
    let generated_status = u8::map_read(input)?;
    let military_targets_len = u8::map_read(input)?;
    let active_entities_serialisation_helper = u32::map_read(input)?; assert_eq!(active_entities_serialisation_helper, 0);
    let planned_update_counts_to_be_loaded = map_read_vec_u32::<_, u32>(input)?;
    let active_when_enemy_is_around = u32::map_read(input)?; assert_eq!(active_when_enemy_is_around, 0);

    Ok(Chunk { position, generated_status, military_targets_len, active_entities_serialisation_helper, planned_update_counts_to_be_loaded,
      active_when_enemy_is_around, tiles: vec![], entities_to_be_inserted_before_setup: vec![], tick_of_optional_activation: 0, tick_of_last_change_that_could_affect_charting: 0, pollution: 0.0 })
  }
  fn initial_write(&self, input: &mut MapSerialiser) -> Result<()> {
    self.position.map_write(input)?;
    self.generated_status.map_write(input)?;
    self.military_targets_len.map_write(input)?;
    self.active_entities_serialisation_helper.map_write(input)?;
    map_write_vec_u32(&self.planned_update_counts_to_be_loaded, input)?;
    self.active_when_enemy_is_around.map_write(input)?;
    
    Ok(())
  }
  fn load<R: BufRead + Seek>(&mut self, input: &mut MapDeserialiser<R>) -> Result<()> {
    println!("{:?}", self.position);
    if self.generated_status > 9 {
      self.tiles = map_read_vec_u16(input)?;
    }
    loop {
      let action_type_pos = input.stream.position();
      let next_entity = u16::map_read(input)?;
      if next_entity == 0 { break; }
      let entity = Entity::from_u16(next_entity).unwrap();
      self.entities_to_be_inserted_before_setup.push((entity, EntityData::map_read(entity, action_type_pos, input)?));
      println!("Read entity {:?}", self.entities_to_be_inserted_before_setup.last());
    }
    self.tick_of_optional_activation = u32::map_read(input)?;
    self.tick_of_last_change_that_could_affect_charting = u32::map_read(input)?;
    self.pollution = f64::map_read(input)?;
    assert_eq!(u8::map_read(input)?, 0); // trivial_smokes
    assert_eq!(u8::map_read(input)?, 0); // decoratives
    assert_eq!(u32::map_read(input)?, 0); // perimeter_components
    assert_eq!(u32::map_read(input)?, 0); // meighbor_components

    Ok(())
  }
  fn save(&self, input: &mut MapSerialiser) -> Result<()> {
    if self.generated_status > 9 {
      map_write_vec_u16(&self.tiles, input)?;
    }
    for (entity, entity_data) in &self.entities_to_be_inserted_before_setup {
      entity.map_write(input)?;
      entity_data.map_write(input)?;
    }
    0_u16.map_write(input)?;

    self.tick_of_optional_activation.map_write(input)?;
    self.tick_of_last_change_that_could_affect_charting.map_write(input)?;
    self.pollution.map_write(input)?;
    0_u8.map_write(input)?; // trivial_smokes
    0_u8.map_write(input)?; // decoratives
    0_u32.map_write(input)?; // perimeter_components
    0_u32.map_write(input)?; // meighbor_components

    Ok(())
  }
}

#[derive(Debug, MapReadWriteTaggedUnion)]
#[tag_type(Entity)]
pub enum EntityData {
  Nothing,
  Coal(ResourceEntity),
  CopperOre(ResourceEntity),
  IronOre(ResourceEntity),
  Stone(ResourceEntity),
  RockHuge(SimpleEntity),
  DryTree(Tree),
}

#[derive(Debug, MapReadWriteStruct)]
pub struct Tree {
  entity: EntityWithHealth,
  tree_data: u16,  // graphics variations
  burn_progress: u8,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ResourceEntity {
  entity: EntityCommon,
  resource_amount: u32,
  initial_amount: Option<u32>,
  variation: u8,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct SimpleEntity {
  entity: EntityWithHealth,
  variation: u8,  // whether this is present depends on the number of graphics variantions, not sure how to predict that
}

#[derive(Debug)]
pub struct EntityWithHealth {
  entity: EntityCommon,
  health: f32,
  damage_to_be_taken: f32,
  upgrade_target: Entity,
}
impl MapReadWrite for EntityWithHealth {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    let entity = EntityCommon::map_read(input)?;
    let health = if entity.usage_bit_mask & 0x2000 != 0 { f32::map_read(input)? } else { 0.0 };
    let damage_to_be_taken = if entity.usage_bit_mask & 0x2000 != 0 { f32::map_read(input)? } else { 0.0 };
    let upgrade_target = if entity.usage_bit_mask & 0x1 != 0 { Entity::map_read(input)? } else { Entity::Nothing };

    Ok(EntityWithHealth { entity, health, damage_to_be_taken, upgrade_target })
  }
  fn map_write(&self, input: &mut MapSerialiser) -> Result<()> {
    self.entity.map_write(input)?;
    if self.entity.usage_bit_mask & 0x2000 != 0 { self.health.map_write(input)?; }
    if self.entity.usage_bit_mask & 0x2000 != 0 { self.damage_to_be_taken.map_write(input)?; }
    if self.entity.usage_bit_mask & 0x1 != 0 { self.upgrade_target.map_write(input)?; }

    Ok(())
  }
}

#[derive(Debug, MapReadWriteStruct)]
pub struct EntityCommon {
  position: MapPosition,
  usage_bit_mask: u16,
  #[conditional_or_default(usage_bit_mask & 0x1000 != 0)] targeter: Option<u32>,
}


#[derive(Debug, MapReadWriteStruct)]
pub struct CompiledMapGenSettings {
  settings: MapGenSettings,
  serialized_data: Vec<u8>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct Commander {
  group_readius_estimate: f64,
  #[assert_eq(0)] unit_groups: u32,  // Vec<>
  expansion_planner: ExpansionPlanner,
  #[assert_eq(0)] bad_chunks: u8,  // Map<>
}

#[derive(Debug, MapReadWriteStruct)]
pub struct ExpansionPlanner {
  last_expansion_tick: u32,
  #[assert_eq(0)] candidates: u8,  // Map<>
  force: u8,
  last_expansion_chunk: Option<ChunkPosition>,
  active: bool,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct MapGenerationManager {
  #[assert_eq(0)] requests_by_status_0: u8,  // Deque<>
  #[assert_eq(0)] requests_by_status_1: u8,  // Deque<>
  #[assert_eq(0)] requests_by_status_2: u8,  // Deque<>
  #[assert_eq(0)] requests_by_status_3: u8,  // Deque<>
  #[assert_eq(0)] requests_being_processed: u8,  // Deque<>
  orders_up_to_tick: f64,
  active: bool,
  force_all_to_lab_grid: bool,
}
