mod gameconfig;
mod hexfloat;
mod runner;
mod prototypes;
mod random;
mod simulation;
mod singleplayerrunner;
mod util;

use factorio_serialize::constants::*;
use factorio_serialize::replay::Direction;
use factorio_serialize::FixedPoint32_8;
use factorio_serialize::MapPosition;
use factorio_serialize::replay::ReplayData;
use factorio_serialize::TilePosition;
use runner::Runner;
use crate::singleplayerrunner::*;

fn main() {
  // assemble_test_tas();
  // assemble_automation_tas();
  // test_float();
  // test_rng();

  // crate::util::load_and_verify_map_test("11107scenarioreplay");
  // crate::util::load_and_save_replay_test("11107scenarioreplay");
  // crate::util::load_and_save_script_test("11107scenarioreplay");
  // crate::util::export_prototypes("11107scenarioreplay");
  // crate::util::clean_up_save_file("11107scenarioreplay", "11107template");
  // crate::util::load_and_verify_map_test("test2");
  // crate::prototypes::create_minimized_prototypes();
  // create_player_movement_test_replay("11107template", "test");
  create_test_replay();
}

// Double a: 0.00416666666666666574148081281236954964697360992431640625   // 4803839602528528 / 2^60
//           0.0041666666666666660745477201999165117740631103515625   //  2345624805922133 / 1000 * 2^49
// Double b: 0.004166666666666666608842550800773096852935850620269775390625  //  4803839602528529 / 2^60
// From Lua: 0.00416666666666666607454772019992
// Division: 0.004166666666666666608842550801
// Lua rndt: 0.004166666666666665741480812812

#[allow(dead_code)]
fn test_float() {
  println!("{:x}", (16.0f64/15.0).to_bits());
  // let increment = 0.25 * (1.0 / 60.0);
  // let mut val = 0.0f64;
  // // let mut value_map = HashSet::new();
  // let mut count = 0_u64;
  // let mut rollovers = 0_u64;
  // loop {
  // // while !value_map.contains(&val.to_bits()) {
  //   // value_map.insert(val.to_bits());
  //   val += increment;
  //   count += 1;
  //   if val >= 1.0 {
  //     rollovers += 1;
  //     val -= 1.0;
  //     if rollovers % 1_000_000_000 == 0 {
  //       println!("{count} val {:x}", val.to_bits());
  //     }
  //     if count % 240 != 1 {
  //       println!("output at {count} val {:x}", val.to_bits());
  //       return;
  //     }
  //   }
  // }
  // println!("found loop with value {val} at {count}")
}

#[allow(dead_code)]
fn test_rng() {
  // let mut rng = RandomGenerator::new(341, 341, 341);
  // // let mut rng = RandomGenerator::new(1446487723, 319594154, 3693144403);
  // for _ in 0..100 {
  //   println!("{}", (rng.uniform_double() * (51.0-24.0) + 24.0) as u32);
  // }
  random::brute_force_rock_rng();
  // random::check_rng_cycles();
  // println!("{:b}", random::next_perm(0b_11110));
}

#[allow(dead_code)]
fn create_player_movement_test_replay(template_name: &str, out_name: &str) {
  let mut runner = Runner::new();

  runner.make_water_tile(TilePosition::new(-3, -1));
  runner.make_water_tile(TilePosition::new(-3, 0));
  runner.make_water_tile(TilePosition::new(-3, 1));
  runner.make_water_tile(TilePosition::new(-4, -1));
  runner.make_water_tile(TilePosition::new(-4, 0));
  runner.make_water_tile(TilePosition::new(-4, 1));
  runner.make_water_tile(TilePosition::new(1, -1));
  runner.make_water_tile(TilePosition::new(1, -0));
  runner.make_water_tile(TilePosition::new(1, 1));
  runner.make_water_tile(TilePosition::new(1, 2));
  runner.make_water_tile(TilePosition::new(1, 3));
  runner.make_water_tile(TilePosition::new(2, -1));
  runner.make_water_tile(TilePosition::new(2, 0));
  runner.make_water_tile(TilePosition::new(2, 1));
  runner.make_water_tile(TilePosition::new(2, 2));
  runner.make_water_tile(TilePosition::new(2, 3));

  runner.make_water_tile(TilePosition::new(1, -5));
  runner.make_water_tile(TilePosition::new(1, -4));
  runner.make_water_tile(TilePosition::new(0, -4));
  runner.make_water_tile(TilePosition::new(0, -3));

  runner.add_tree(MapPosition::new(FixedPoint32_8(256), FixedPoint32_8(-1024)));

  runner.walk_for(Direction::West, 30);
  runner.walk_for(Direction::South, 25);
  runner.walk_for(Direction::NorthEast, 25);
  runner.walk_for(Direction::East, 40);
  runner.walk_for(Direction::North, 50);
  runner.walk_for(Direction::East, 12);
  runner.walk_for(Direction::SouthWest, 15);
  runner.walk_for(Direction::North, 11);
  runner.walk_for(Direction::West, 8);

  runner.write_save_file(template_name, out_name).unwrap();
}

#[allow(dead_code)]
fn create_test_replay() {
  let mut runner = Runner::new();

  runner.build_stone_furnace(TilePosition::new(2, 2));
  runner.add_fuel_to_stone_furnace(Item::Wood, 1, TilePosition::new(2, 2));
  runner.mine_rock();
  runner.add_input_to_stone_furnace(Item::Stone, 1, TilePosition::new(2, 2));
  runner.mine_tree();
  runner.add_input_to_stone_furnace(Item::Stone, 49, TilePosition::new(2, 2));
  runner.mine_rock();
  runner.mine_rock();
  runner.mine_rock();
  runner.mine_rock();
  runner.add_fuel_to_stone_furnace(Item::Wood, 4, TilePosition::new(2, 2));

  for _ in 0..10 {
    runner.mine_rock();
  }
  // runner.mine_tree();
  // runner.mine_iron_ore(2);
  // runner.mine_copper_ore(2);

  runner.write_save_file("11107template", "test").unwrap();
}

#[allow(dead_code)]
fn assemble_test_tas() {
  let id1_pos = MapPosition::new(FixedPoint32_8(0x500), FixedPoint32_8(-0x400));
  let if1_pos = MapPosition::new(FixedPoint32_8(0x500), FixedPoint32_8(-0x200-0x100));

  // let iron_ore_pos = MapPosition::new(0x80, -0x280);
  let dry_dree_pos = MapPosition::new(FixedPoint32_8(0x200), FixedPoint32_8(0x0));
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
  let _bytes = ReplayData::from_input_actions(items).write_replay_data().unwrap();
}

#[allow(dead_code)]
fn assemble_automation_tas() {
  let id1_pos = MapPosition::new(FixedPoint32_8(0x500), FixedPoint32_8(-0x400));
  let if1_pos = MapPosition::new(FixedPoint32_8(0x500), FixedPoint32_8(-0x200));
  let id2_pos = MapPosition::new(FixedPoint32_8(0x300), FixedPoint32_8(-0x600));
  let if2_pos = MapPosition::new(FixedPoint32_8(0x300), FixedPoint32_8(-0x400));
  let id3_pos = MapPosition::new(FixedPoint32_8(0x100), FixedPoint32_8(-0x600));
  let if3_pos = MapPosition::new(FixedPoint32_8(0x100), FixedPoint32_8(-0x400));
  let id4_pos = MapPosition::new(FixedPoint32_8(-0x100), FixedPoint32_8(-0x600));
  let if4_pos = MapPosition::new(FixedPoint32_8(-0x100), FixedPoint32_8(-0x400));
  let id5_pos = MapPosition::new(FixedPoint32_8(-0x300), FixedPoint32_8(-0x600));
  let if5_pos = MapPosition::new(FixedPoint32_8(-0x300), FixedPoint32_8(-0x400));
  let id6_pos = MapPosition::new(FixedPoint32_8(-0x500), FixedPoint32_8(-0x400));
  let if6_pos = MapPosition::new(FixedPoint32_8(-0x500), FixedPoint32_8(-0x200));

  let cd1_pos = MapPosition::new(FixedPoint32_8(0x100), FixedPoint32_8(0x600));
  let cf1_pos = MapPosition::new(FixedPoint32_8(0x100), FixedPoint32_8(0x800));
  let cd2_pos = MapPosition::new(FixedPoint32_8(-0x100), FixedPoint32_8(0x600));
  let cf2_pos = MapPosition::new(FixedPoint32_8(-0x100), FixedPoint32_8(0x800));

  let iron_ore_pos = MapPosition::new(FixedPoint32_8(0x80), FixedPoint32_8(-0x280));
  let copper_ore_pos = MapPosition::new(FixedPoint32_8(0x80), FixedPoint32_8(-0x180));
  let dry_dree_pos = MapPosition::new(FixedPoint32_8(0x200), FixedPoint32_8(0x0));
  let huge_rock_1_pos = MapPosition::new(FixedPoint32_8(0x100), FixedPoint32_8(0x100));
  let huge_rock_2_pos = MapPosition::new(FixedPoint32_8(-0x100), FixedPoint32_8(0x100));
  let huge_rock_3_pos = MapPosition::new(FixedPoint32_8(-0x100), FixedPoint32_8(-0x100));

  let offshore_pump_pos = MapPosition::new(FixedPoint32_8(-0x680), FixedPoint32_8(0x80));
  let boiler_pos = MapPosition::new(FixedPoint32_8(-0x600), FixedPoint32_8(0x280));
  let steam_engine_pos = MapPosition::new(FixedPoint32_8(-0x280), FixedPoint32_8(0x280));
  let electric_pole_pos = MapPosition::new(FixedPoint32_8(0x80), FixedPoint32_8(0x80));
  let lab_1_pos = MapPosition::new(FixedPoint32_8(0x280), FixedPoint32_8(0x80));
  let lab_2_pos = MapPosition::new(FixedPoint32_8(0x280), FixedPoint32_8(0x380));

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
  let _bytes = ReplayData::from_input_actions(items).write_replay_data().unwrap();
}
