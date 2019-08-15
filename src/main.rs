mod action;
mod constants;
mod replay;
mod singleplayerrunner;

use byteorder::{LittleEndian, ReadBytesExt};
#[allow(unused_imports)] use crate::action::*;
use crate::constants::*;
use crate::replay::*;
use crate::singleplayerrunner::*;
use heck::CamelCase;
use std::fs::File;
use std::io::{Read, Write};
use zip::write::FileOptions;

fn main() {
  // load_and_save_test();
  assemble_test_tas();
  // parse_items_game_data();
  // parse_recipes_game_data();
}

#[allow(dead_code)]
fn parse_items_game_data() {
  let item_data = load_file("data/items.dat");
  let mut reader = ReplayReader::new(item_data);
  while !reader.is_at_eof() {
    let name = reader.read_string().unwrap().to_camel_case();
    let id = reader.read_u16::<LittleEndian>().unwrap();
    println!("{} = {},", name, id)
  }
}

#[allow(dead_code)]
fn parse_recipes_game_data() {
  let recipe_data = load_file("data/recipes.dat");
  let mut reader = ReplayReader::new(recipe_data);
  while !reader.is_at_eof() {
    let name = reader.read_string().unwrap().to_camel_case();
    let id = reader.read_u16::<LittleEndian>().unwrap();
    println!("{} = {},", name, id)
  }
}

#[allow(dead_code)]
fn assemble_test_tas() {
  let items = SinglePlayerRunner::new("TASBot")
    .craft(Recipe::TransportBelt, 1)
    .craft(Recipe::TransportBelt, 1)
    .build(Item::BurnerMiningDrill, 0x500, -0x400, CardinalDirection::S)
    .build(Item::StoneFurnace, 0x500, -0x200, CardinalDirection::S)
    .add_fuel(Item::Wood, 1, 0x500, -0x400)
    .wait_for(62)
    .build(Item::TransportBelt, -0x300, -0x200, CardinalDirection::S)
    .build(Item::TransportBelt, -0x200, -0x200, CardinalDirection::S)
    .wait_for(62)
    .build(Item::TransportBelt, -0x100, -0x200, CardinalDirection::S)
    .build(Item::TransportBelt, -0x000, -0x200, CardinalDirection::S)
    .mine_for(60, 0x200, 0x0) // Mine Dry Tree  // tick 60
    .add_fuel(Item::Wood, 1, 0x500, -0x200)
    .mine_for(361, 0x100, 0x100) // Mine Huge Rock  // tick 421
    .add_fuel(Item::Coal, 10, 0x500, -0x400)
    .add_fuel(Item::Coal, 10, 0x500, -0x200)
    .wait_for(17)  // tick 438
    .take_contents(0x500, -0x200)
    .wait_for(238)  // tick 676
    .take_contents(0x500, -0x200)
    .wait_for(240)
    .take_contents(0x500, -0x200)
    .wait_for(240)
    .take_contents(0x500, -0x200)
    .wait_for(240)
    .take_contents(0x500, -0x200)
    .wait_for(240)
    .take_contents(0x500, -0x200)
    .wait_for(240)
    .take_contents(0x500, -0x200)
    .wait_for(240)
    .take_contents(0x500, -0x200)
    .wait_for(240)
    .take_contents(0x500, -0x200)
    .wait_for(240)
    .take_contents(0x500, -0x200)
    .wait_for(240)
    .take_contents(0x500, -0x200)
    .into_replay_items();
  let bytes = write_replay(items);
  assemble_save_file("data/test20-level.dat", "replay-assemble-test", &bytes).unwrap();
}

#[allow(dead_code)]
fn load_and_save_test() {
  let bytes = read_save_file("saves/replay_test.zip").unwrap();
  let items = parse_replay(bytes);
  println!("replay: {:?}", items);
  let bytes = write_replay(items);
  write_save_file("saves/replay_test.zip", "saves/replay_test_new.zip", &bytes).unwrap();
}

fn assemble_save_file(level_file_name: &str, save_name: &str, replay_bytes: &[u8]) -> zip::result::ZipResult<()> {
  let info_data = load_file("data/info.json");
  let control_data = load_file("data/control.lua");
  let level_data = load_file(level_file_name);

  let save_file_name = format!("{}/Factorio/saves/{}.zip", std::env::var("APPDATA").unwrap(), save_name);
  let new_path = std::path::Path::new(&save_file_name);
  let new_file = std::fs::File::create(&new_path)?;
  let mut new_zip = zip::ZipWriter::new(new_file);

  new_zip.start_file(format!("{}/info.json", save_name), FileOptions::default())?;
  new_zip.write_all(&info_data)?;
  new_zip.start_file(format!("{}/control.lua", save_name), FileOptions::default())?;
  new_zip.write_all(&control_data)?;
  new_zip.start_file(format!("{}/level.dat", save_name), FileOptions::default())?;
  new_zip.write_all(&level_data)?;
  new_zip.start_file(format!("{}/level-init.dat", save_name), FileOptions::default())?;
  new_zip.write_all(&level_data)?;
  new_zip.start_file(format!("{}/replay.dat", save_name), FileOptions::default())?;
  new_zip.write_all(replay_bytes)?;
  Ok(())
}

fn write_save_file(base_file_name: &str, new_file_name: &str, replay_bytes: &[u8]) -> zip::result::ZipResult<()> {
  let base_path = std::path::Path::new(base_file_name);
  let base_file = std::fs::File::open(&base_path)?;
  let mut base_archive = zip::ZipArchive::new(base_file)?;

  let new_path = std::path::Path::new(new_file_name);
  let new_file = std::fs::File::create(&new_path)?;
  let mut new_zip = zip::ZipWriter::new(new_file);

  let mut used_replay_bytes = false;
  for i in 0..base_archive.len() {
    let mut file = base_archive.by_index(i)?;
    if file.name().ends_with("replay.dat") {
      assert!(!used_replay_bytes, "found multiple replay.dat in zip file!");
      println!("updating file: {}", file.name());
      new_zip.start_file(file.name(), FileOptions::default())?;
      new_zip.write_all(replay_bytes)?;
      used_replay_bytes = true;
    } else {
      println!("transferring file: {}", file.name());
      new_zip.start_file(file.name(), FileOptions::default())?;
      let mut bytes = vec![];
      file.read_to_end(&mut bytes)?;
      new_zip.write_all(&bytes)?;
    }
  }
  assert!(used_replay_bytes, "didn't find replay.dat in zip file!");
  Ok(())
}

fn read_save_file(file_name: &str) -> zip::result::ZipResult<Vec<u8>> {
  let path = std::path::Path::new(file_name);
  let file = std::fs::File::open(&path)?;
  let mut archive = zip::ZipArchive::new(file)?;
  for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;
    if file.name().ends_with("replay.dat") {
      let mut bytes = vec![];
      file.read_to_end(&mut bytes)?;
      return Ok(bytes);
    }
  }
  panic!("didn't find replay.dat in zip file!");
}

/// Helper function to load the byte contents of a file into memory.
fn load_file(file_name: &str) -> Vec<u8> {
  let mut result: Vec<u8> = vec![];
  let mut f = File::open(file_name).expect("file not found");
  f.read_to_end(&mut result).unwrap();
  result
}
