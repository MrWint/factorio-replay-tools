use std::fmt::Debug;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Seek;

use factorio_serialize_derive::MapReadWriteStruct;

use crate::Result;
use crate::map::MapDeserialiser;
use crate::map::MapReadWrite;
use crate::map::MapSerialiser;
use crate::map::MapVersion;


pub struct ScriptData {
  pub map_version: MapVersion,  // part of MapDeserializer
  pub lua_context: LuaContext,

  pub remaining_data: Vec<u8>,  // unknown unparsed data
}
impl ScriptData {
  pub fn parse_script_data(map_data: &[u8]) -> Result<ScriptData> {
    let mut map_deserialiser = MapDeserialiser::new(Cursor::new(map_data))?;

    let lua_context = LuaContext::map_read(&mut map_deserialiser)?;

    let remaining_data = map_deserialiser.stream.read_to_end()?;
    let map_version = map_deserialiser.map_version;

    Ok(ScriptData { map_version, lua_context, remaining_data })
  }

  pub fn write_script_data(&self) -> Result<Vec<u8>> {
    let mut map_serialiser = MapSerialiser::new(self.map_version.clone())?;

    self.lua_context.map_write(&mut map_serialiser)?;

    map_serialiser.stream.write_bytes(&self.remaining_data)?;

    Ok(map_serialiser.stream.into_inner().into_inner())
  }
}
impl std::fmt::Debug for ScriptData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "map_version: ")?; self.map_version.fmt(f)?;
    writeln!(f, "lua_context: ")?; self.lua_context.fmt(f)?;
    Ok(())
  }
}

#[derive(Debug, MapReadWriteStruct)]
pub struct LuaContext {
  #[vec_u32] pub scripts: Vec<(String, LuaGameScript)>,
}

#[derive(Debug, MapReadWriteStruct)]
pub struct LuaGameScript {
  pub script_state: LuaGameScriptState,
  pub had_control_lua: bool,
}

#[derive(Debug)]
pub struct LuaGameScriptState {
  pub state_value: LuaValue,
}
impl MapReadWrite for LuaGameScriptState {
  fn map_read<R: BufRead + Seek>(input: &mut crate::map::MapDeserialiser<R>) -> Result<Self> {
    let raw_data = Vec::<u8>::map_read(input)?;

    let mut inner_deserialiser = MapDeserialiser::new(Cursor::new(&raw_data))?;
    assert_eq!(inner_deserialiser.map_version, input.map_version);

    let state_value = LuaValue::map_read(&mut inner_deserialiser)?;
    assert!(inner_deserialiser.stream.is_at_eof()?);

    Ok(LuaGameScriptState { state_value })
  }
  fn map_write(&self, input: &mut crate::map::MapSerialiser) -> Result<()> {
    let mut inner_serialiser = MapSerialiser::new(input.map_version.clone())?;

    self.state_value.map_write(&mut inner_serialiser)?;
    let raw_data = inner_serialiser.stream.into_inner().into_inner();

    raw_data.map_write(input)
  }
}

#[derive(Clone, Debug)]
pub enum LuaValue {
  Nothing,
  BoolFalse,
  BoolTrue,
  Number(f64),
  String(String),
  Table(Vec<(LuaValue, LuaValue)>),
}
impl MapReadWrite for LuaValue {
  fn map_read<R: BufRead + Seek>(input: &mut MapDeserialiser<R>) -> Result<Self> {
    let typ = u8::map_read(input)?;
    match typ {
      0 => Ok(LuaValue::Nothing),
      1 => Ok(LuaValue::BoolFalse),
      2 => Ok(LuaValue::BoolTrue),
      3 => Ok(LuaValue::Number(f64::map_read(input)?)),
      4 => Ok(LuaValue::String(String::map_read(input)?)),
      5 => Ok(LuaValue::Table(Vec::map_read(input)?)),
      _ => Err(input.stream.error_at(format!("Unknown LuaValue type {}", typ), 1))
    }
  }
  fn map_write(&self, w: &mut MapSerialiser) -> Result<()> {
    match self {
      LuaValue::Nothing => { 0_u8.map_write(w) },
      LuaValue::BoolFalse => { 1_u8.map_write(w) },
      LuaValue::BoolTrue => { 2_u8.map_write(w) },
      LuaValue::Number(v) => { 3_u8.map_write(w)?; v.map_write(w) },
      LuaValue::String(v) => { 4_u8.map_write(w)?; v.map_write(w) },
      LuaValue::Table(v) => { 5_u8.map_write(w)?; v.map_write(w) },
    }
  }
}

