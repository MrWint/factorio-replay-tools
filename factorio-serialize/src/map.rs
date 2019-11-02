use crate as factorio_serialize;
use std::io::{BufRead, Seek, Write};
use factorio_serialize::{constants::*, ReadWrite, ReadWriteStruct, Reader, Result, Writer};

const MIN_MAP_VERSION: u64 = 0x0000_0011_0000_0059+1; // skip lecagy LowercaseString sets in ScenarioExecutionContext

#[derive(Clone, Debug, ReadWriteStruct)]
pub struct Map {
  map_version: MapVersion,
  scenario_execution_context: ScenarioExecutionContext,
  map_header: MapHeader,
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
  Dictionary { any_type_flag: bool, value: Vec<PropertyTree>, },
}
impl ReadWrite for PropertyTree {
  fn read<R: BufRead + Seek>(r: &mut Reader<R>) -> Result<Self> {
    let typ = u8::read(r)?;
    let any_type_flag = bool::read(r)?;
    match typ {
      0 => Ok(PropertyTree::Nothing { any_type_flag, }),
      1 => Ok(PropertyTree::Bool { any_type_flag, value: bool::read(r)?, }),
      2 => Ok(PropertyTree::Number { any_type_flag, value: f64::read(r)?, }),
      3 => Ok(PropertyTree::String { any_type_flag, value: String::read(r)?, }),
      4 => {
        let len = u32::read(r)?;
        let value = (0..len).map(|_| PropertyTree::read(r)).collect::<Result<Vec<PropertyTree>>>()?;
        Ok(PropertyTree::List { any_type_flag, value, })
      },
      5 => {
        let len = u32::read(r)?;
        let value = (0..len).map(|_| PropertyTree::read(r)).collect::<Result<Vec<PropertyTree>>>()?;
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
        for v in value { v.write(w)?; }
        Ok(())
      },
      PropertyTree::Dictionary { any_type_flag, value } => {
        5u8.write(w)?;
        any_type_flag.write(w)?;
        w.write_u32(value.len() as u32)?;
        for v in value { v.write(w)?; }
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