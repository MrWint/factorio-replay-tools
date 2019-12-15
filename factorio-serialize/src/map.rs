use crate as factorio_serialize;
use crate::constants::*;
use crate::structs::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{BufRead, Seek, Write};
use std::marker::PhantomData;
use std::num::TryFromIntError;
use factorio_serialize::{ReadWrite, ReadWriteStruct, Reader, Result, Writer};

const MIN_MAP_VERSION: u64 = 0x0000_0011_0033_0002; // 0.17.51(.2) deprecated some ID mappings

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct Map {
  map_version: MapVersion,
  scenario_execution_context: ScenarioExecutionContext,
  map_header: MapHeader,
  map_gen_settings: MapGenSettings,
  map_settings: MapSettings,
  general_random_generator: RandomGenerator,
  ai_random_generator: RandomGenerator,
  entities_random_generator: RandomGenerator,
  map_random_generator: RandomGenerator,
  triggers_random_generator: RandomGenerator,
  entity_update_paused_state: EntityUpdatePausedState,
  #[non_space_optimized] books_to_sort: Vec<Targeter>,
  prototype_migrations: PrototypeMigrations,

  prototype_migrations_definition: Vec<PrototypeMigrationDefinition>,
  next_unit_number: u32,
  next_targetable_item_number: u32,
  next_script_pathfind_id: u32,
  next_circuit_network_number: u32,
  map_mod_settings: MapModSettings,
  train_manager: TrainManager,
  // force_manager: ForceManager,
}

#[derive(Clone, Debug)]
pub struct MapVersion(u64);
impl ReadWrite for MapVersion {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let version_major = u16::read(r)?;
    let version_minor = u16::read(r)?;
    let version_patch = u16::read(r)?;
    let version_dev = u16::read(r)?;
    let version = u64::from(version_major) << 48 | u64::from(version_minor) << 32 | u64::from(version_patch) << 16 | u64::from(version_dev);
    if version < MIN_MAP_VERSION { return Err(r.error_at(format!("Map version {:x} too old", version), 8)); }
    let quailty_version = bool::read(r)?;
    if quailty_version { return Err(r.error_at("Map is from quality branch".to_owned(), 1)); }
    Ok(MapVersion(version))
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    w.write_u16((self.0 >> 48) as u16)?;
    w.write_u16((self.0 >> 32) as u16)?;
    w.write_u16((self.0 >> 16) as u16)?;
    w.write_u16((self.0 >> 0) as u16)?;
    false.write(w) // not quality version
  }
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct ScenarioExecutionContext {
  location: ScenarioLocation,
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

#[derive(Clone, Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct ScenarioLocation {
  campaign_name: String,
  level_name: String,
  mod_name: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct ApplicationVersion {
  #[space_optimized] major_version: u16,
  #[space_optimized] minor_version: u16,
  #[space_optimized] sub_version: u16,
  build_version: u16,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct ModId {
  name: String,
  version: ModVersion,
  crc: u32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
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
  String { any_type_flag: bool, value: String, },
  List { any_type_flag: bool, value: Vec<PropertyTree>, },
  Dictionary { any_type_flag: bool, value: HashMap<String, PropertyTree>, },
}
impl ReadWrite for PropertyTree {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let typ = u8::read(r)?;
    let any_type_flag = bool::read(r)?;
    match typ {
      0 => Ok(PropertyTree::Nothing { any_type_flag, }),
      1 => Ok(PropertyTree::Bool { any_type_flag, value: bool::read(r)?, }),
      2 => Ok(PropertyTree::Number { any_type_flag, value: f64::read(r)?, }),
      3 => {
        let value_is_null = bool::read(r)?;
        let value = if value_is_null { String::new() } else { String::read(r)? };
        Ok(PropertyTree::String { any_type_flag, value, })
      },
      4 => {
        let len = u32::read(r)?;
        let mut value = Vec::new();
        for _ in 0..len {
          let name_is_null = bool::read(r)?;
          let name = if name_is_null { String::new() } else { String::read(r)? };
          if name != "" { return Err(r.error_at(format!("Unknown PropertyTree List contains non-null name {}", name), 1 + name.len() as u64)) }
          value.push(PropertyTree::read(r)?);
        }
        Ok(PropertyTree::List { any_type_flag, value, })
      },
      5 => {
        let len = u32::read(r)?;
        let mut value = HashMap::new();
        for _ in 0..len {
          let name_is_null = bool::read(r)?;
          let name = if name_is_null { String::new() } else { String::read(r)? };
          value.insert(name, PropertyTree::read(r)?);
        }
        Ok(PropertyTree::Dictionary { any_type_flag, value, })
      },
      _ => Err(r.error_at(format!("Unknown PropertyTree type {}", typ), 1))
    }
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    match self {
      PropertyTree::Nothing { any_type_flag } => {
        0u8.write(w)?;
        any_type_flag.write(w)
      },
      PropertyTree::Bool { any_type_flag, value } => {
        1u8.write(w)?;
        any_type_flag.write(w)?;
        value.write(w)
      },
      PropertyTree::Number { any_type_flag, value } => {
        2u8.write(w)?;
        any_type_flag.write(w)?;
        value.write(w)
      },
      PropertyTree::String { any_type_flag, value } => {
        3u8.write(w)?;
        any_type_flag.write(w)?;
        value.write(w)
      },
      PropertyTree::List { any_type_flag, value } => {
        4u8.write(w)?;
        any_type_flag.write(w)?;
        w.write_u32(value.len() as u32)?;
        for v in value {
          1u8.write(w)?; // name_is_null
          v.write(w)?;
          }
        Ok(())
      },
      PropertyTree::Dictionary { any_type_flag, value } => {
        5u8.write(w)?;
        any_type_flag.write(w)?;
        w.write_u32(value.len() as u32)?;
        for (name, v) in value {
          0u8.write(w)?; // name_is_null
          name.write(w)?;
          v.write(w)?;
        }
        Ok(())
      },
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, ReadWriteStruct)]
pub struct MapHeader {
  update_tick: u32,
  entity_tick: u32,
  ticks_played: u32,
}

type MapGenSize = f32;

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct MapGenSettings {
  segmentation: MapGenSize,
  water_size: MapGenSize,
  autoplace_controls: HashMap<String, FrequencySizeRichness>,
  autoplace_settings_per_type: HashMap<String, AutoplaceSettings>,
  default_enable_all_autoplace_controls: bool,
  random_seed: u32,
  width: u32,
  height: u32,
  #[map] area_to_generate_at_start: BoundingBox,
  starting_area_size: MapGenSize,
  peaceful_mode: bool,
  #[map] starting_points: Vec<MapPosition>,
  property_expression_names: HashMap<String, String>,
  cliff_placement_settings: CliffPlacementSettings,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct FrequencySizeRichness {
  frequency: MapGenSize,
  size: MapGenSize,
  richness: MapGenSize,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct AutoplaceSettings {
  treat_missing_as_default: bool,
  settings: HashMap<String, FrequencySizeRichness>,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct CliffPlacementSettings {
  cliff_name: String,
  cliff_elevation0: f32,
  cliff_elevation_interval: f32,
  richness: f32,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct MapSettings {
  pollution_settings: PollutionSettings,
  default_steering_settings: StateSteeringSettings,
  moving_steering_settings: StateSteeringSettings,
  enemy_evolution_settings: EnemyEvolutionSettings,
  enemy_expansion_settings: EnemyExpansionSettings,
  unit_group_settings: UnitGroupSettings,
  path_finder_settings: PathFinderSettings,
  max_failed_behavior_count: u32,
  difficulty_settings: DifficultySettings,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct PollutionSettings {
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

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct StateSteeringSettings {
  radius: Option<f64>,
  separation_factor: Option<f64>,
  separation_force: Option<f64>,
  force_unit_fuzzy_goto_behavior: Option<bool>,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct EnemyEvolutionSettings {
  enabled: Option<bool>,
  time_factor: Option<f64>,
  destroy_factor: Option<f64>,
  pollution_factor: Option<f64>,
}

#[derive(Clone, Debug, ReadWriteStruct)]
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

#[derive(Clone, Debug, ReadWriteStruct)]
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

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct PathFinderSettings {
  fwd2bwd_ratio: Option<u32>,
  goal_pressure_ratio: Option<f64>,
  use_path_cache: Option<bool>,
  max_steps_worked_per_tick: Option<f64>,
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
  max_clients_to_accept_any_new_request: Option<u32>,
  max_clients_to_accept_short_new_request: Option<u32>,
  direct_distance_to_consider_short_request: Option<u32>,
  short_request_max_steps: Option<u32>,
  short_request_ratio: Option<f64>,
  min_steps_to_check_path_find_termination: Option<u32>,
  start_to_goal_cost_multiplier_to_terminate_path_find: Option<f64>,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct DifficultySettings {
  recipe_difficulty: RecipeDifficulty,
  technology_difficulty: TechnologyDifficulty,
  technology_price_multiplier: f64,
  research_queue_setting: ResearchQueueSetting,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct RandomGenerator {
  seed1: u32,
  seed2: u32,
  seed3: u32,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct EntityUpdatePausedState {
  paused: bool,
  ticks_to_run: u32,
}

type Targeter = u32; // target_index

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct PrototypeMigrations {
  custom_input_id_migrations: ProtoTypeIdMapping<u16, u16>,
  equipment_grid_id_migrations: ProtoTypeIdMapping<u8, u8>,
  item_id_migrations: ProtoTypeIdMapping<u16, u16>,
  tile_id_migrations: ProtoTypeIdMapping<u8, u8>,
  decorative_id_migrations: ProtoTypeIdMapping<u8, u8>,
  technology_id_migrations: ProtoTypeIdMapping<u16, u16>,
  entity_id_migrations: ProtoTypeIdMapping<u16, u16>,
  recipe_category_id_migrations: ProtoTypeIdMapping<u16, u16>,
  item_sub_group_id_migrations: ProtoTypeIdMapping<u16, u16>,
  item_group_id_migrations: ProtoTypeIdMapping<u8, u8>,
  fluid_id_migrations: ProtoTypeIdMapping<u16, u16>,
  virtual_sign_id_migrations: ProtoTypeIdMapping<u16, u16>,
  ammo_category_id_migrations: ProtoTypeIdMapping<u8, u8>,
  fuel_category_id_migrations: ProtoTypeIdMapping<u8, u8>,
  resource_category_id_migrations: ProtoTypeIdMapping<u8, u8>,
  equipment_id_migrations: ProtoTypeIdMapping<u16, u16>,
  noise_layer_id_migrations: ProtoTypeIdMapping<u16, u16>,
  named_noise_expression_id_migrations: ProtoTypeIdMapping<u32, u32>,
  autoplace_control_id_migrations: ProtoTypeIdMapping<u8, u8>,
  damage_type_id_migrations: ProtoTypeIdMapping<u8, u8>,
  recipe_id_migrations: ProtoTypeIdMapping<u16, u16>,
  achievement_id_migrations: ProtoTypeIdMapping<u16, u16>,
  module_category_id_migrations: ProtoTypeIdMapping<u8, u8>,
  equipment_category_id_migrations: ProtoTypeIdMapping<u8, u8>,
  mod_settings_id_migrations: ProtoTypeIdMapping<u16, u16>,
  trivial_smoke_id_migrations: ProtoTypeIdMapping<u8, u8>,
  shortcut_id_migrations: ProtoTypeIdMapping<u16, u16>,
}

#[derive(Clone, Debug)]
pub struct ProtoTypeIdMapping<I, V> {
  mappings: HashMap<String, HashMap<String, V>>,
  _phantom: PhantomData<I>,
}
impl<I: ReadWrite + TryFrom<usize,Error=TryFromIntError>, V: ReadWrite> ReadWrite for ProtoTypeIdMapping<I, V> where u32: From<I> {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let len = I::read(r)?;
    let mut mappings = HashMap::new();
    for _ in 0..u32::from(len) {
      let mapping_name = String::read(r)?;
      let mapping_len = I::read(r)?;
      let mut mapping = HashMap::new();
      for _ in 0..u32::from(mapping_len) {
        let name = String::read(r)?;
        let value = V::read(r)?;
        mapping.insert(name, value);
      }
      mappings.insert(mapping_name, mapping);
    }
    Ok(ProtoTypeIdMapping { mappings, _phantom: PhantomData })
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    I::try_from(self.mappings.len()).unwrap().write(w)?;
    for (mapping_name, mapping) in &self.mappings {
      mapping_name.write(w)?;
      I::try_from(mapping.len()).unwrap().write(w)?;
      for (name, value) in mapping {
        name.write(w)?;
        value.write(w)?;
      }
    }
    Ok(())
  }
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct PrototypeMigrationDefinition {
  mod_name: String,
  name: String,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct MapModSettings {
  #[non_space_optimized] runtime_global_settings: Vec<ModSetting>,
  #[non_space_optimized] runtime_per_user_settings: Vec<ModSetting>,
}

#[derive(Clone, Debug)]
pub enum ModSetting {
  BoolSetting(u16, bool),
  DoubleSetting(u16, f64),
  IntSetting(u16, u64),
  StringSetting(u16, String),
}
impl ReadWrite for ModSetting {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let typ = r.read_u8()?;
    match typ {
      1 => {
        let id = u16::read(r)?;
        let value = bool::read(r)?;
        Ok(ModSetting::BoolSetting(id, value))
      },
      2 => {
        let id = u16::read(r)?;
        let value = f64::read(r)?;
        Ok(ModSetting::DoubleSetting(id, value))
      },
      3 => {
        let id = u16::read(r)?;
        let value = u64::read(r)?;
        Ok(ModSetting::IntSetting(id, value))
      },
      4 => {
        let id = u16::read(r)?;
        let value = String::read(r)?;
        Ok(ModSetting::StringSetting(id, value))
      },
      _ => Err(r.error_at(format!("Unknown ModSetting type {}", typ), 1))
    }
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    match self {
      ModSetting::BoolSetting(id, value) => {
        1u8.write(w)?;
        id.write(w)?;
        value.write(w)
      },
      ModSetting::DoubleSetting(id, value) => {
        2u8.write(w)?;
        id.write(w)?;
        value.write(w)
      },
      ModSetting::IntSetting(id, value) => {
        3u8.write(w)?;
        id.write(w)?;
        value.write(w)
      },
      ModSetting::StringSetting(id, value) => {
        4u8.write(w)?;
        id.write(w)?;
        value.write(w)
      },
    }
  }
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct TrainManager {
  next_train_id: u32,
  next_rail_segment: u32,
  #[non_space_optimized] rail_segments: Vec<RailSegment>,
  #[non_space_optimized] trains: Vec<Train>,
  stops_enable_changed_this_tick: Vec<Targeter>,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct RailSegment {
  id: u32,
  targeter: Targeter,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct Train {
  targeters: Option<u32>,
  id: u32,
  front: Targeter,
  back: Targeter,
  train_stop: Targeter,
  riding_state: RidingState,
  speed: f64,
  state: TrainState,
  last_state: TrainState,
  manual_travelled_distance: f64,
  tick_of_last_gate_activation: u32,
  distance_since_last_gate_activation: f64,
  ticks_in_station: u32,
  ticks_waiting_at_signal: u32,
  ticks_of_last_rolling_stock_activity: u32,
  stop_distance: f64,
  signal_logic: TrainSignalLogic,
  schedule: TrainSchedule,
  path: Option<RailPath>,
  scheduled_to_recalculate_path: bool,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct TrainSignalLogic {
  arriving_at_signal: RailSignalData,
  stopped_at_signal: RailSignalData,
  signals_ahead: Vec<RailSignalData>,
  reserved_signals: Vec<RailSignalData>,
  in_chain_signal_selection: bool,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct RailSignalData {
  distance: f64,
  targeter: Targeter,
  inverted: bool,
}


#[derive(Clone, Debug, ReadWriteStruct)]
pub struct TrainSchedule {
  current: u32,
  tick_of_last_schedule_change: u32,
  #[non_space_optimized] schedule: Vec<TrainScheduleRecord>,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct TrainScheduleRecord {
  station_name_or_rail: StationNameOrRail,
  #[non_space_optimized] wait_conditions: Vec<WaitCondition>,
  temporary: bool,
}

#[derive(Clone, Debug)]
pub enum StationNameOrRail {
  StationName(String),
  Rail(Targeter),
}
impl ReadWrite for StationNameOrRail {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let station_name = String::read(r)?;
    if station_name.is_empty() {
      Ok(StationNameOrRail::Rail(Targeter::read(r)?))
    } else {
      Ok(StationNameOrRail::StationName(station_name))
    }
  }
  fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
    match self {
      StationNameOrRail::StationName(station_name) => {
        station_name.write(w)
      },
      StationNameOrRail::Rail(rail) => {
        String::new().write(w)?; // empty station name
        rail.write(w)
      }
    }
  }
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct RailPath {
  targeter: Targeter,
  current: u32,
  #[non_space_optimized] rails: Vec<Targeter>,
  waypoints: Vec<RailPathWaypoint>,
  total_distance: f64,
  travelled_distance: f64,
  is_front: bool,
}

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct RailPathWaypoint {
  #[space_optimized] rail_index: u32,
  #[space_optimized] schedule_index: u32,
}

// #[derive(Clone, Debug, ReadWriteStruct)]
// pub struct ForceManager {
//   #[non_space_optimized] force_data_list: Vec<ForceData>,
//   #[non_space_optimized] forces_to_detele: Vec<(ForceId, ForceId)>,
// }

// #[derive(Clone, Debug, ReadWriteStruct)]
// pub struct ForceData {
//   id: ForceId,
//   disable_all_by_default: bool,
//   friendly_fire_enabled: bool,
//   share_chart: bool,
//   evolution_factor_data: EvolutionFactorData,
//   custom_prototypes: CustomPrototypes,
//   research_enabled: bool,
//   research_manager: ResearchManager,
//   #[non_space_optimized] logistic_managers: Vec<Option<LogisticManager>>,
//   #[non_space_optimized] construction_managers: Vec<Option<ConstructionManager>>,
//   ammo_damage_modifiers: Vec<f64>,
//   gun_speed_modifiers: Vec<f64>,
//   turret_attack_modifiers: Vec<f64>,
//   disabled_hand_crafting_recipes: Vec<i8>,
//   worker_robots_speed_modifier: f64,
//   worker_robots_battery_modifier: f64,
//   worker_robots_storage_bonus: f64,
//   laboratory_speed_modifier: f64,
//   laboratory_productivity_bonus: f64,
//   following_robots_lifetime_modifier: f64,
//   manual_crafting_speed_modifier: f64,
//   manual_mining_speed_modifier: f64,
//   running_speed_modifier: f64,
//   artillery_range_modifier: f64,
//   build_distance_bonus: f64,
//   item_drop_distance_bonus: f64,
//   reach_distance_bonus: f64,
//   resource_reach_distance_bonus: f64,
//   item_pickup_distance_bonus: f64,
//   loot_pickup_distance_bonus: f64,
//   character_inventory_slot_count_bonus: f64,
//   character_health_bonus: f64,
//   mining_drill_productivity_bonus: f64,
//   train_braking_force_bonus: f64,
//   inserter_stack_size_bonus: f64,
//   stack_inserter_capacity_bonus: f64,
//   character_logistic_slot_count: f64,
//   character_logistic_trash_slot_count: f64,
//   maximum_following_robots_count: f64,
//   ghost_time_to_live: f64,
//   deconstruction_time_to_live: f64,
//   max_successful_attempts_per_tick_per_construction_queue: f64,
//   max_failed_attempts_per_tick_per_construction_queue: f64,
//   auto_character_logistic_track_slots: bool,
//   zoom_to_world_enabled: bool,
//   zoom_to_world_ghost_building_enabled: bool,
//   zoom_to_world_blueprint_enabled: bool,
//   zoom_to_world_deconstruction_planner_enabled: bool,
//   zoom_to_world_upgrade_planner_enabled: bool,
//   zoom_to_world_selection_tool_enabled: bool,
//   cease_fire: ForceSet,
//   friends: ForceSet,
//   #[non_space_optimized] charts: Vec<(SurfaceIndex, Chart)>,
//   #[non_space_optimized] spawn_positions: Vec<(SurfaceIndex, MapPosition)>,
//   item_production_statistics: FlowStatistics<Item>,
//   fluid_production_statistics: FlowStatistics<Fluid>,
//   kill_count_statistics: FlowStatistics<Entity>,
//   build_count_statistics: FlowStatistics<Entity>,
//   rockets_launched: u32,
//   items_launched: FlowStatistics<Item>,
// }

// #[derive(Clone, Debug, ReadWriteStruct)]
// pub struct SurfaceIndex {
//   #[space_optimized] index: u32,
// }

// #[derive(Clone, Debug, ReadWriteStruct)]
// pub struct EvolutionFactorData {
//   evolution_factor: f64,
//   evolution_increased_by_pollution: f64,
//   evolution_increased_by_pollution_this_tick: f64,
//   evolution_increased_by_time: f64,
//   evolution_increased_by_time_this_tick: f64,
//   evolution_increased_by_killing_spawners: f64,
//   evolution_increased_by_killing_spawners_this_tick: f64,
// }

// #[derive(Clone, Debug)]
// pub struct FlowStatistics<E: ReadWrite> {
//   precisions: Vec<FlowStatisticsPrecision>, // 8 elements
//   input_running_costs: Vec<(E, u64)>,
//   output_running_costs: Vec<(E, u64)>,
// }
// impl<E: ReadWrite> ReadWrite for FlowStatistics<E> {
//   fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
//     let precisions = r.read_array(8)?;
//     let input_running_costs  = <Vec<(E, u64)>>::read(r)?;
//     let output_running_costs  = <Vec<(E, u64)>>::read(r)?;
//     Ok(Self { precisions, input_running_costs, output_running_costs, })
//   }
//   fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
//     w.write_array(&self.precisions)?;
//     self.input_running_costs.write(w)?;
//     self.output_running_costs.write(w)
//   }
// }

// #[derive(Clone, Debug, ReadWriteStruct)]
// pub struct FlowStatisticsPrecision {
//   #[non_space_optimized] input_elements: Vec<FlowStatisticsPrecisionElementUsageStatistics>,
//   #[non_space_optimized] output_elements: Vec<FlowStatisticsPrecisionElementUsageStatistics>,
// }

// #[derive(Clone, Debug)]
// pub struct FlowStatisticsPrecisionElementUsageStatistics {
//   history: Vec<f32>, // u16 len
//   current_sample: f32,
// }
// impl ReadWrite for FlowStatisticsPrecisionElementUsageStatistics {
//   fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
//     let len = u16::read(r)?;
//     let history = r.read_array(u32::from(len))?;
//     let current_sample  = f32::read(r)?;
//     Ok(Self { history, current_sample, })
//   }
//   fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
//     (self.history.len() as u16).write(w)?;
//     w.write_array(&self.history)?;
//     self.current_sample.write(w)
//   }
// }

// #[derive(Clone, Debug)]
// pub struct Chart {}
// impl ReadWrite for Chart {
//   fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
//     unimplemented!()
//   }
//   fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
//     unimplemented!()
//   }
// }
// #[derive(Clone, Debug)]
// pub struct ForceSet {}
// impl ReadWrite for ForceSet {
//   fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
//     unimplemented!()
//   }
//   fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
//     unimplemented!()
//   }
// }
// #[derive(Clone, Debug)]
// pub struct ConstructionManager {}
// impl ReadWrite for ConstructionManager {
//   fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
//     unimplemented!()
//   }
//   fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
//     unimplemented!()
//   }
// }
// #[derive(Clone, Debug)]
// pub struct LogisticManager {}
// impl ReadWrite for LogisticManager {
//   fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
//     unimplemented!()
//   }
//   fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
//     unimplemented!()
//   }
// }
// #[derive(Clone, Debug)]
// pub struct ResearchManager {}
// impl ReadWrite for ResearchManager {
//   fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
//     unimplemented!()
//   }
//   fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
//     unimplemented!()
//   }
// }
// #[derive(Clone, Debug)]
// pub struct CustomPrototypes {}
// impl ReadWrite for CustomPrototypes {
//   fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
//     unimplemented!()
//   }
//   fn write<W: Write + Seek>(&self, w: &mut Writer<W>) -> Result<()> {
//     unimplemented!()
//   }
// }
