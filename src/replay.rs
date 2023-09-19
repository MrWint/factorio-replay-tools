use factorio_serialize::inputaction::*;
use std::io::Cursor;
use factorio_serialize::{ReadWrite, Reader, Writer};

#[allow(dead_code)]
pub fn parse_replay(replay_bytes: Vec<u8>) -> Vec<InputAction> {
  let mut r = Reader::new(Cursor::new(replay_bytes));
  let mut result = vec![];
  while !r.is_at_eof().unwrap() {
    result.push(InputAction::read(&mut r).unwrap());
  }
  result
}

pub fn write_replay(input_actions: Vec<InputAction>) -> Vec<u8> {
  let mut w = Writer::new(Cursor::new(vec![]));
  for input_action in input_actions {
    input_action.write(&mut w).unwrap();
  }
  w.into_inner().into_inner()
}
