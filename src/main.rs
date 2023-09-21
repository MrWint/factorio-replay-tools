mod singleplayerrunner;

use factorio_serialize::constants::*;
use factorio_serialize::map::ActiveMigrations;
use factorio_serialize::map::MapData;
use factorio_serialize::replay::Direction;
use factorio_serialize::replay::MapPosition;
use factorio_serialize::replay::ReplayData;
use factorio_serialize::save::SaveFile;
use crate::singleplayerrunner::*;
use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Write};
use zip::write::FileOptions;

fn main() {
  // test_read_map_data();
  // load_and_save_map_test("freeplay1187_replay_2");
  // load_and_save_map_test("scenario_save_replay");
  load_and_save_map_test("scenario_save_replay_2");
  // load_and_save_replay_test("freeplay1187_replay_2");
  // load_and_save_replay_test("scenario_save_replay_2");
  // load_and_save_replay_test("1187-20230720-1-35-08");
  // load_and_save_replay_test("1187-4h46m32s");
  // assemble_test_tas();
  // assemble_automation_tas();

  // export_prototypes("scenario_save_replay_2");
}

#[allow(dead_code)]
fn load_and_save_map_test(name: &str) {
  let save_file = SaveFile::load_save_file(name).unwrap();

  let map_data = MapData::parse_map_data(&save_file.level_init_dat).unwrap();
  println!("Map data: {:?}", map_data);
  println!("Unparsed map data size: {}", map_data.remaining_data.len());
  println!("Next non-null at index: {:?}", map_data.remaining_data.iter().position(|&x| x != 0));
  if let Some(non_null_pos) = map_data.remaining_data.iter().position(|&x| x != 0) {
    println!("Next non-null bytes: {:?}", &map_data.remaining_data[non_null_pos..non_null_pos+20]);
  }

  let serialized_map_data = map_data.write_map_data().unwrap();
  assert_eq!(serialized_map_data, save_file.level_init_dat);

  save_file.write_save_file("test").unwrap()
}

#[allow(dead_code)]
fn export_prototypes(name: &str) {
  let save_file = SaveFile::load_save_file(name).unwrap();
  let map_data = MapData::parse_map_data(&save_file.level_init_dat).unwrap();

  // print_prototype_ids(&map_data.map.prototype_migrations.achievement_id_migrations, "Achievements");
  // print_prototype_ids(&map_data.map.prototype_migrations.decorative_id_migrations, "Decoratives");
  // print_prototype_ids(&map_data.map.prototype_migrations.entity_id_migrations, "Enitites");
  // print_prototype_ids(&map_data.map.prototype_migrations.equipment_id_migrations, "Equipment");
  // print_prototype_ids(&map_data.map.prototype_migrations.fluid_id_migrations, "Fluids");
  // print_prototype_ids(&map_data.map.prototype_migrations.item_id_migrations, "Items");
  // print_prototype_ids(&map_data.map.prototype_migrations.item_group_id_migrations, "ItemGroups");
  // print_prototype_ids(&map_data.map.prototype_migrations.recipe_id_migrations, "Recipes");
  // print_prototype_ids(&map_data.map.prototype_migrations.tile_id_migrations, "Tiles");
  // print_prototype_ids(&map_data.map.prototype_migrations.technology_id_migrations, "Technologies");
  print_prototype_ids(&map_data.map.prototype_migrations.virtual_signal_id_migrations, "VirtualSignals");
}

fn print_prototype_ids<T: Copy + Display + Ord>(migrations: &ActiveMigrations<T>, name: &str) {
  println!("");
  println!("{}:", name);

  let mut sorted_mappings = vec![];
  for (_, mappings) in &migrations.mappings {
    sorted_mappings.extend_from_slice(mappings);
  }
  sorted_mappings.sort_by_key(|(_, i)| *i);
  for (name, id) in sorted_mappings {
    println!("  {} = {},", heck::AsUpperCamelCase(name), id);
  }
}

#[allow(dead_code)]
fn load_and_save_replay_test(name: &str) {
  let save_file = SaveFile::load_save_file(name).unwrap();

  let replay_data = ReplayData::parse_replay_data(&save_file.replay_dat).unwrap();
  println!("Replay data: {:?}", replay_data);

  let serialized_replay_data = replay_data.write_replay_data().unwrap();
  assert_eq!(serialized_replay_data, save_file.replay_dat);

  save_file.write_save_file("test").unwrap()
}

#[allow(dead_code)]
fn assemble_test_tas() {
  let id1_pos = MapPosition::new(0x500, -0x400);
  let if1_pos = MapPosition::new(0x500, -0x200-0x100);

  // let iron_ore_pos = MapPosition::new(0x80, -0x280);
  let dry_dree_pos = MapPosition::new(0x200, 0x0);
  // let huge_rock_1_pos = MapPosition::new(0x100, 0x100);

  let items = SinglePlayerRunner::new("TASBot")
    // .craft(Recipe::IronGearWheel, 3)
    .build(Item::BurnerMiningDrill, id1_pos, Direction::South)
    // .build(Item::StoneFurnace, if1_pos, Direction::South)
    .add_fuel(Item::Wood, 1, id1_pos)
    .mine_for(60, dry_dree_pos) // Mine Dry Tree  // tick 60
    .craft(Recipe::WoodenChest, 1)
    // .add_fuel(Item::Wood, 1, if1_pos)
    .wait_for(31) // 91
    .build(Item::WoodenChest, if1_pos, Direction::South)
    .wait_for(252) // 242
    .take_contents(if1_pos)
    // .mine_for(361, huge_rock_1_pos) // Mine Huge Rock -> 46 Stone, 24 Coal  // tick 421
    // .craft(Recipe::StoneFurnace, 1)
    // .add_fuel(Item::Coal, 3, id1_pos)
    // // .add_fuel(Item::Coal, 2, if1_pos)
    // .mine_for(18, iron_ore_pos) // Mine iron ore  // tick 438
    // .take_contents(if1_pos)
    // .mine_for(13, iron_ore_pos) // Mine iron ore  // tick 452
    // .craft(Recipe::BurnerMiningDrill, 1)
    .into_replay_items();
  let bytes = ReplayData::from_input_actions(items).write_replay_data().unwrap();
  assemble_save_file("data/test-0.17.69-level.dat", "replay-assemble-test", &bytes).unwrap();
}

#[allow(dead_code)]
fn assemble_automation_tas() {
  let id1_pos = MapPosition::new(0x500, -0x400);
  let if1_pos = MapPosition::new(0x500, -0x200);
  let id2_pos = MapPosition::new(0x300, -0x600);
  let if2_pos = MapPosition::new(0x300, -0x400);
  let id3_pos = MapPosition::new(0x100, -0x600);
  let if3_pos = MapPosition::new(0x100, -0x400);
  let id4_pos = MapPosition::new(-0x100, -0x600);
  let if4_pos = MapPosition::new(-0x100, -0x400);
  let id5_pos = MapPosition::new(-0x300, -0x600);
  let if5_pos = MapPosition::new(-0x300, -0x400);
  let id6_pos = MapPosition::new(-0x500, -0x400);
  let if6_pos = MapPosition::new(-0x500, -0x200);

  let cd1_pos = MapPosition::new(0x100, 0x600);
  let cf1_pos = MapPosition::new(0x100, 0x800);
  let cd2_pos = MapPosition::new(-0x100, 0x600);
  let cf2_pos = MapPosition::new(-0x100, 0x800);

  let iron_ore_pos = MapPosition::new(0x80, -0x280);
  let copper_ore_pos = MapPosition::new(0x80, -0x180);
  let dry_dree_pos = MapPosition::new(0x200, 0x0);
  let huge_rock_1_pos = MapPosition::new(0x100, 0x100);
  let huge_rock_2_pos = MapPosition::new(-0x100, 0x100);
  let huge_rock_3_pos = MapPosition::new(-0x100, -0x100);

  let offshore_pump_pos = MapPosition::new(-0x680, 0x80);
  let boiler_pos = MapPosition::new(-0x600, 0x280);
  let steam_engine_pos = MapPosition::new(-0x280, 0x280);
  let electric_pole_pos = MapPosition::new(0x80, 0x80);
  let lab_1_pos = MapPosition::new(0x280, 0x80);
  let lab_2_pos = MapPosition::new(0x280, 0x380);

  let items = SinglePlayerRunner::new("TASBot")
    .craft(Recipe::IronGearWheel, 3)
    .build(Item::BurnerMiningDrill, id1_pos, Direction::South)
    .build(Item::StoneFurnace, if1_pos, Direction::South)
    .add_item(Item::Wood, 1, id1_pos)
    .mine_for(60, dry_dree_pos) // Mine Dry Tree  // tick 60
    .add_item(Item::Wood, 1, if1_pos)
    .mine_for(361, huge_rock_1_pos) // Mine Huge Rock -> 46 Stone, 24 Coal  // tick 421
    .craft(Recipe::StoneFurnace, 1)
    .add_item(Item::Coal, 3, id1_pos)
    .add_item(Item::Coal, 2, if1_pos)
    .mine_for(17, iron_ore_pos) // Mine iron ore  // tick 438
    .take_contents(if1_pos)
    .mine_for(14, iron_ore_pos) // Mine iron ore  // tick 452
    .craft(Recipe::BurnerMiningDrill, 1)
    .craft(Recipe::StoneFurnace, 5)
    .mine_for(90, iron_ore_pos) // Mine iron ore  // tick 542
    .mine_for(31, iron_ore_pos) // Mine iron ore  // tick 573
    .build(Item::BurnerMiningDrill, id2_pos, Direction::South)
    .add_item(Item::Coal, 3, id2_pos)
    .mine_for(31, iron_ore_pos) // Mine iron ore  // tick 604
    .build(Item::StoneFurnace, if2_pos, Direction::South)
    .add_item(Item::Coal, 2, if2_pos)
    .add_item(Item::IronOre, 1, if2_pos)
    .mine_for(31, iron_ore_pos) // Mine iron ore  // tick 635
    .build(Item::StoneFurnace, if3_pos, Direction::South)
    .add_item(Item::Coal, 2, if3_pos)
    .mine_for(28, iron_ore_pos) // Mine iron ore  // tick 663
    .add_item(Item::IronOre, 1, if3_pos)
    .mine_for(3, iron_ore_pos) // Mine iron ore  // tick 666
    .build(Item::StoneFurnace, if4_pos, Direction::South)
    .add_item(Item::Coal, 2, if4_pos)
    .mine_for(31, iron_ore_pos) // Mine iron ore  // tick 697
    .build(Item::StoneFurnace, if5_pos, Direction::South)
    .add_item(Item::Coal, 2, if5_pos)
    .mine_for(31, iron_ore_pos) // Mine iron ore  // tick 728
    .build(Item::StoneFurnace, if6_pos, Direction::South)
    .add_item(Item::Coal, 1, if6_pos)
    .mine_for(56, iron_ore_pos) // Mine iron ore  // tick 784
    .add_item(Item::IronOre, 1, if4_pos)
    .mine_for(121, iron_ore_pos) // Mine iron ore  // tick 905
    .add_item(Item::IronOre, 1, if3_pos)
    .mine_for(121, iron_ore_pos) // Mine iron ore  // tick 1026
    .add_item(Item::IronOre, 1, if4_pos)
    .mine_for(69, iron_ore_pos) // Mine iron ore  // tick 1095
    .take_contents(if1_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .craft(Recipe::StoneFurnace, 1)
    .craft(Recipe::IronGearWheel, 3)
    .mine_for(52, iron_ore_pos) // Mine iron ore  // tick 1147
    .add_item(Item::IronOre, 1, if3_pos)
    .mine_for(72, iron_ore_pos) // Mine iron ore  // tick 1219
    .take_contents(if3_pos)
    .take_contents(if1_pos)
    .take_contents(if4_pos)
    .craft(Recipe::BurnerMiningDrill, 1)
    .mine_for(49, iron_ore_pos) // Mine iron ore  // tick 1268
    .add_item(Item::IronOre, 1, if4_pos)
    .mine_for(72, iron_ore_pos) // Mine iron ore  // tick 1340
    .build(Item::BurnerMiningDrill, id3_pos, Direction::South)
    .add_item(Item::Coal, 3, id3_pos)
    .mine_for(49, iron_ore_pos) // Mine iron ore  // tick 1389
    .add_item(Item::IronOre, 1, if3_pos)
    .mine_for(121, iron_ore_pos) // Mine iron ore  // tick 1510
    .add_item(Item::IronOre, 1, if4_pos)
    .mine_for(95, iron_ore_pos) // Mine iron ore  // tick 1605
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if1_pos)
    .take_contents(if4_pos)
    .craft(Recipe::StoneFurnace, 1)
    .craft(Recipe::IronGearWheel, 3)
    .mine_for(26, iron_ore_pos) // Mine iron ore  // tick 1631
    .add_item(Item::IronOre, 1, if5_pos)
    .mine_for(98, iron_ore_pos) // Mine iron ore  // tick 1729
    .take_contents(if1_pos)
    .take_contents(if4_pos)
    .take_contents(if2_pos)
    .craft(Recipe::BurnerMiningDrill, 1)
    .mine_for(23, iron_ore_pos) // Mine iron ore  // tick 1752
    .add_item(Item::IronOre, 1, if4_pos)
    .mine_for(98, iron_ore_pos) // Mine iron ore  // tick 1850
    .build(Item::BurnerMiningDrill, id5_pos, Direction::South)
    .add_item(Item::Coal, 2, id5_pos)
    .mine_for(23, iron_ore_pos) // Mine iron ore  // tick 1873
    .add_item(Item::IronOre, 1, if5_pos)
    .mine_for(121, iron_ore_pos) // Mine iron ore  // tick 1994
    .add_item(Item::IronOre, 1, if4_pos)
    .mine_for(69, iron_ore_pos) // Mine iron ore  // tick 2063
    .take_contents(if3_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(if4_pos)
    .take_contents(if2_pos)
    .craft(Recipe::StoneFurnace, 1)
    .craft(Recipe::IronGearWheel, 3)
    .mine_for(52, iron_ore_pos) // Mine iron ore  // tick 2115
    .add_item(Item::IronOre, 1, if6_pos)
    .mine_for(72, huge_rock_2_pos) // Mine Huge Rock  // tick 2187
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(if4_pos)
    .craft(Recipe::BurnerMiningDrill, 1)
    .mine_for(121, huge_rock_2_pos) // Mine Huge Rock  // tick 2308
    .build(Item::BurnerMiningDrill, id4_pos, Direction::South)
    .add_item(Item::Coal, 2, id4_pos)
    .mine_for(102, huge_rock_2_pos) // Mine Huge Rock  // tick 2410
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if5_pos)
    .take_contents(if6_pos)
    .craft(Recipe::IronGearWheel, 2)
    .mine_for(62, huge_rock_2_pos) // Mine Huge Rock  // tick 2472
    .take_contents(if1_pos)
    .take_contents(if2_pos)
    .craft(Recipe::IronGearWheel, 1)
    .mine_for(4, huge_rock_2_pos) // Mine Huge Rock 49 stone/42 coal  // tick 2476
    .craft(Recipe::StoneFurnace, 3)
    .mine_for(58, iron_ore_pos) // Mine iron ore  // tick 2534
    .build(Item::StoneFurnace, cf1_pos, Direction::South)
    .add_item(Item::Coal, 1, cf1_pos)
    .mine_for(31, iron_ore_pos) // Mine iron ore  // tick 2565
    .build(Item::StoneFurnace, cf2_pos, Direction::South)
    .add_item(Item::Coal, 1, cf2_pos)
    .mine_for(31, iron_ore_pos) // Mine iron ore  // tick 2596
    .take_contents(if3_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .craft(Recipe::BurnerMiningDrill, 1)
    .mine_for(1, iron_ore_pos) // Mine iron ore  // tick 2597
    .add_item(Item::IronOre, 1, if6_pos)
    .mine_for(120, iron_ore_pos) // Mine iron ore  // tick 2717
    .build(Item::BurnerMiningDrill, id6_pos, Direction::South)
    .add_item(Item::Coal, 2, id6_pos)
    .craft(Recipe::StoneFurnace, 4)
    .mine_for(1, iron_ore_pos) // Mine iron ore  // tick 2718
    .add_item(Item::IronOre, 1, cf1_pos)
    .mine_for(121, iron_ore_pos) // Mine iron ore  // tick 2839
    .add_item(Item::IronOre, 1, cf2_pos)
    .mine_for(13, iron_ore_pos) // Mine iron ore  // tick 2852
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if6_pos)
    .take_contents(if1_pos)
    .craft(Recipe::StoneFurnace, 1)
    .craft(Recipe::IronGearWheel, 3)
    .mine_for(108, iron_ore_pos) // Mine iron ore  // tick 2960
    .add_item(Item::IronOre, 1, cf1_pos)
    .mine_for(16, copper_ore_pos) // Mine iron ore  // tick 2976
    .take_contents(cf1_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .craft(Recipe::BurnerMiningDrill, 1)
    .mine_for(105, copper_ore_pos) // Mine iron ore  // tick 3081
    .add_item(Item::CopperOre, 1, cf2_pos)
    .mine_for(16, iron_ore_pos) // Mine iron ore  // tick 3097
    .build(Item::BurnerMiningDrill, cd2_pos, Direction::South)
    .add_item(Item::Coal, 2, cd2_pos)
    .mine_for(3, iron_ore_pos) // Mine iron ore  // tick 3100
    .craft(Recipe::StoneFurnace, 1)
    .mine_for(31, iron_ore_pos) // Mine iron ore  // tick 3131
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(cf2_pos)
    .take_contents(if1_pos)
    .craft(Recipe::IronGearWheel, 2)
    .mine_for(62, iron_ore_pos) // Mine iron ore  // tick 3193
    .take_contents(if6_pos)
    .take_contents(cf1_pos)
    .take_contents(if2_pos)
    .craft(Recipe::IronGearWheel, 1)
    .mine_for(9, iron_ore_pos) // Mine iron ore  // tick 3202
    .add_item(Item::IronOre, 1, cf1_pos)
    .mine_for(22, iron_ore_pos) // Mine iron ore  // tick 3224
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .craft(Recipe::BurnerMiningDrill, 1)
    .mine_for(99, iron_ore_pos) // Mine iron ore  // tick 3323
    .add_item(Item::IronOre, 1, cf1_pos)
    .mine_for(22, huge_rock_3_pos)  // tick 3345
    .build(Item::BurnerMiningDrill, cd1_pos, Direction::South)
    .add_item(Item::Coal, 2, cd1_pos)
    .take_contents(if5_pos)
    .take_contents(cf2_pos)
    .take_contents(if1_pos)
    .craft(Recipe::AutomationSciencePack, 1)
    .mine_for(242, huge_rock_3_pos)  // tick 3587
    .take_contents(cf1_pos) // empty to make place for copper
    .mine_for(90, huge_rock_3_pos)  // tick 3677
    .take_contents(cf2_pos)
    .craft(Recipe::AutomationSciencePack, 1)
    .mine_for(7, huge_rock_3_pos)  // tick 3684
    .wait_for(325)  // tick 4009
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .craft(Recipe::AutomationSciencePack, 2)

    .wait_for(664)  // tick 4673
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .add_item(Item::Coal, 4, cd1_pos)
    .add_item(Item::Coal, 4, cd2_pos)
    .add_item(Item::Coal, 4, id1_pos)
    .add_item(Item::Coal, 4, id2_pos)
    .add_item(Item::Coal, 4, id3_pos)
    .add_item(Item::Coal, 4, id4_pos)
    .add_item(Item::Coal, 4, id5_pos)
    .add_item(Item::Coal, 4, id6_pos)
    .add_item(Item::Coal, 2, cf1_pos)
    .add_item(Item::Coal, 2, cf2_pos)
    .add_item(Item::Coal, 2, if1_pos)
    .add_item(Item::Coal, 2, if2_pos)
    .add_item(Item::Coal, 2, if3_pos)
    .add_item(Item::Coal, 2, if4_pos)
    .add_item(Item::Coal, 2, if5_pos)
    .add_item(Item::Coal, 2, if6_pos)
    .craft(Recipe::OffshorePump, 1)
    .wait_for(248)  // tick 4921
    .build(Item::OffshorePump, offshore_pump_pos, Direction::North)
    .craft(Recipe::Boiler, 1)
    .wait_for(155)  // tick 5076
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .build(Item::Boiler, boiler_pos, Direction::East)
    .add_item(Item::Coal, 6, boiler_pos)
    .craft(Recipe::Pipe, 5)
    .wait_for(155)  // tick 5231
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .craft(Recipe::SteamEngine, 1)
    .wait_for(279)  // tick 5510
    .build(Item::SteamEngine, steam_engine_pos, Direction::East)
    .craft(Recipe::SmallElectricPole, 1)
    .wait_for(62)  // tick 5572
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .build(Item::SmallElectricPole, electric_pole_pos, Direction::North)
    .craft(Recipe::TransportBelt, 2)
    .craft(Recipe::ElectronicCircuit, 6)
    .wait_for(589)  // tick 6161
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .craft(Recipe::ElectronicCircuit, 2)
    .craft(Recipe::IronGearWheel, 6)
    .wait_for(341)  // tick 6502
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .craft(Recipe::ElectronicCircuit, 2)
    .craft(Recipe::IronGearWheel, 3)
    .wait_for(248)  // tick 6750
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .craft(Recipe::Lab, 1)
    .wait_for(152)  // tick 6902
    .start_research(Technology::Automation)
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .build(Item::Lab, lab_1_pos, Direction::North)
    .add_item(Item::AutomationSciencePack, 3, lab_1_pos)
    .craft(Recipe::ElectronicCircuit, 4)
    .craft(Recipe::TransportBelt, 1)
    .craft(Recipe::IronGearWheel, 1)
    .wait_for(403)  // tick 7305
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .craft(Recipe::ElectronicCircuit, 2)
    .craft(Recipe::TransportBelt, 1)
    .craft(Recipe::IronGearWheel, 4)
    .wait_for(310)  // tick 7615
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .craft(Recipe::ElectronicCircuit, 2)
    .craft(Recipe::IronGearWheel, 2)
    .wait_for(217)  // tick 7832
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .craft(Recipe::IronGearWheel, 3)
    .wait_for(93)  // tick 7925
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .craft(Recipe::ElectronicCircuit, 2)
    .wait_for(155)  // tick 8080
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .craft(Recipe::Lab, 1)
    .wait_for(152)  // tick 8232
    .build(Item::Lab, lab_2_pos, Direction::North)
    .add_item(Item::AutomationSciencePack, 1, lab_2_pos)
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .craft(Recipe::AutomationSciencePack, 1)
    .wait_for(332)  // tick 8564
    .add_item(Item::AutomationSciencePack, 1, lab_1_pos)
    .craft(Recipe::AutomationSciencePack, 1)
    .wait_for(332)  // tick 8896
    .add_item(Item::AutomationSciencePack, 1, lab_2_pos)
    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .craft(Recipe::AutomationSciencePack, 1)
    .wait_for(332)  // tick 9228
    .add_item(Item::AutomationSciencePack, 1, lab_1_pos)
    .craft(Recipe::AutomationSciencePack, 1)
    .wait_for(332)  // tick 9560
    .add_item(Item::AutomationSciencePack, 1, lab_2_pos)
    .craft(Recipe::AutomationSciencePack, 1)
    .wait_for(332)  // tick 9892
    .add_item(Item::AutomationSciencePack, 1, lab_1_pos)
    .craft(Recipe::AutomationSciencePack, 1)
    .wait_for(332)  // tick 10224
    .add_item(Item::AutomationSciencePack, 1, lab_2_pos)
    .craft(Recipe::AutomationSciencePack, 1)
    .wait_for(332)  // tick 10556
    .add_item(Item::AutomationSciencePack, 1, lab_1_pos)

    .take_contents(if6_pos)
    .take_contents(if2_pos)
    .take_contents(if3_pos)
    .take_contents(if4_pos)
    .take_contents(if5_pos)
    .take_contents(if1_pos)
    .take_contents(cf2_pos)
    .take_contents(cf1_pos)
    .into_replay_items();
  let bytes = ReplayData::from_input_actions(items).write_replay_data().unwrap();
  assemble_save_file("data/test-0.17.69-level.dat", "replay-assemble-automation", &bytes).unwrap();
}

fn assemble_save_file(level_file_name: &str, save_name: &str, replay_bytes: &[u8]) -> zip::result::ZipResult<()> {
  let info_data = load_file("data/info.json");
  let control_data = load_file("data/control.lua");
  // let freeplay_data = load_file("data/freeplay.lua");
  let level_data = load_file(level_file_name);

  let save_file_name = format!("{}/Factorio/saves/{}.zip", std::env::var("APPDATA").unwrap(), save_name);
  let new_path = std::path::Path::new(&save_file_name);
  let new_file = std::fs::File::create(&new_path)?;
  let mut new_zip = zip::ZipWriter::new(new_file);

  new_zip.start_file(format!("{}/info.json", save_name), FileOptions::default())?;
  new_zip.write_all(&info_data)?;
  new_zip.start_file(format!("{}/control.lua", save_name), FileOptions::default())?;
  new_zip.write_all(&control_data)?;
  // new_zip.start_file(format!("{}/freeplay.lua", save_name), FileOptions::default())?;
  // new_zip.write_all(&freeplay_data)?;
  new_zip.start_file(format!("{}/level.dat", save_name), FileOptions::default())?;
  new_zip.write_all(&level_data)?;
  new_zip.start_file(format!("{}/level-init.dat", save_name), FileOptions::default())?;
  new_zip.write_all(&level_data)?;
  new_zip.start_file(format!("{}/replay.dat", save_name), FileOptions::default())?;
  new_zip.write_all(replay_bytes)?;
  Ok(())
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
