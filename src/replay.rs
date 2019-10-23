use crate::action::*;
use crate::constants::*;
use num_traits::{FromPrimitive, ToPrimitive};
use std::io::{BufRead, Cursor, Seek, Write};
use factorio_serialize::{Reader, Result, Writer};

#[derive(Debug)]
pub struct ReplayItem {
  tick: u32,
  player_id: u16,
  action: InputActionData,
}
impl ReplayItem {
  pub fn new(tick: u32, player_id: u16, action: InputActionData) -> Self {
    Self { tick, player_id, action }
  }
}

pub fn parse_replay(replay_bytes: Vec<u8>) -> Vec<ReplayItem> {
  let mut read = Reader::new(Cursor::new(replay_bytes));
  let mut result = vec![];
  while !read.is_at_eof().unwrap() {
    if let Some(replay_item) = parse_replay_item(&mut read).unwrap() {
      result.push(replay_item);
    }
  }
  result
}

fn parse_replay_item<R: BufRead + Seek>(read: &mut Reader<R>) -> Result<Option<ReplayItem>> {
  let action_type_pos = read.position();
  let action_type = InputActionType::from_u8(read.read_u8()?).unwrap();
  let tick = read.read_u32()?;
  let player_id = read.read_opt_u16()?;
  let action = InputActionData::read(action_type, action_type_pos, read)?;
  if action.to_tag() == InputActionType::Nothing { return Ok(None); }
  assert!(action_type == action.to_tag(), "Action type {:?} does not match {:?}", action_type, action.to_tag());
  Ok(Some(ReplayItem { tick, player_id, action }))
}

pub fn write_replay(replay_items: Vec<ReplayItem>) -> Vec<u8> {
  let mut write = Writer::new(Cursor::new(vec![]));
  for replay_item in replay_items {
    write_replay_item(&mut write, replay_item).unwrap();
  }
  write.into_inner().into_inner()
}

fn write_replay_item<W: Write + Seek>(write: &mut Writer<W>, replay_item: ReplayItem) -> Result<()> {
  write.write_u8(replay_item.action.to_tag().to_u8().unwrap())?;
  write.write_u32(replay_item.tick)?;
  write.write_opt_u16(replay_item.player_id)?;
  replay_item.action.write(write)
}
