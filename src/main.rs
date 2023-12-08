mod hexfloat;
mod runner;
mod singleplayerrunner;

use factorio_serialize::TilePosition;
use factorio_serialize::constants::*;
use factorio_serialize::map::ActiveMigrations;
use factorio_serialize::map::EntityCommon;
use factorio_serialize::map::EntityData;
use factorio_serialize::map::EntityWithHealth;
use factorio_serialize::map::MapData;
use factorio_serialize::map::ResourceEntity;
use factorio_serialize::map::SimpleEntity;
use factorio_serialize::map::Tree;
use factorio_serialize::replay::Direction;
use factorio_serialize::replay::ForceId;
use factorio_serialize::replay::InputAction;
use factorio_serialize::replay::InputActionData;
use factorio_serialize::MapPosition;
use factorio_serialize::replay::PlayerJoinGameData;
use factorio_serialize::replay::ReplayData;
use factorio_serialize::save::SaveFile;
use factorio_serialize::script::LuaGameScript;
use factorio_serialize::script::LuaGameScriptState;
use factorio_serialize::script::LuaValue;
use factorio_serialize::script::ScriptData;
use runner::Runner;
use crate::singleplayerrunner::*;
use crate::hexfloat::HexFloat;
use std::fmt::Display;

fn main() {
  // test_read_map_data();
  // load_and_save_map_test("freeplay1187_replay_2");
  // load_and_save_map_test("scenario_save_replay");
  // load_and_save_map_test("scenario_save_replay_2");
  // load_and_save_map_test("1191scenarioreplay");
  // load_and_save_map_test("1191scenarioreplay3");
  // load_and_save_replay_test("freeplay1187_replay_2");
  // load_and_save_replay_test("scenario_save_replay_2");
  // load_and_save_replay_test("1187-20230720-1-35-08");
  // load_and_save_replay_test("1187-4h46m32s");
  // load_and_save_replay_test("test2");
  // load_and_save_script_test("test");
  // map_to_scenario_test("1191scenarioreplay");
  // assemble_test_tas();
  // assemble_automation_tas();
  // modify_map_test("1191scenarioreplay3");
  // clean_up_save_file("1191scenarioreplay3");
  create_test_replay();
  // test_float();

  // export_prototypes("1191scenarioreplay");
}

// Double a: 0.00416666666666666574148081281236954964697360992431640625   // 4803839602528528 / 2^60
//           0.0041666666666666660745477201999165117740631103515625   //  2345624805922133 / 1000 * 2^49
// Double b: 0.004166666666666666608842550800773096852935850620269775390625  //  4803839602528529 / 2^60
// From Lua: 0.00416666666666666607454772019992
// Division: 0.004166666666666666608842550801
// Lua rndt: 0.004166666666666665741480812812

#[allow(dead_code)]
fn test_float() {
  // println!("{:.30}", (1.0_f64 / 4.0) / 60.0);
  // println!("{:.30}", (1.0_f64 / 60.0) / 4.0);
  // println!("{:.30}", 0.00416666666666666607454772019992_f64);
  // let mut a = 0_f64;
  // for _ in 0..240 {
  //   a += 0.00416666666666666661;
  //   // a += 0.0041666666666666661;
  //   println!("{:.20}", HexFloat(a));
  // }
  println!("{}", HexFloat((2500.0 * 16.0) / 15.0));
  println!("{}", HexFloat((2500.0 / 15.0) * 16.0));
  println!("{}", HexFloat(2500.0 * (16.0 / 15.0)));
  let f = 16.0 / 15.0;
  println!("{}", HexFloat(f));
  println!("{}", HexFloat(1500.0 * f));
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
fn modify_map_test(name: &str) {
  let mut save_file = SaveFile::load_save_file(name).unwrap();

  let mut map_data = MapData::parse_map_data(&save_file.level_init_dat).unwrap();
  // println!("Map data: {:?}", map_data);

  for x in 2..5 { for y in 2..5 { 
    map_data.map.surfaces[0].chunks[210].tiles[x][y] = (Tile::RefinedConcrete, 16);
  }}

  map_data.map.surfaces[0].chunks[210].entities_to_be_inserted_before_setup.push((Entity::DryTree, EntityData::DryTree(Tree { entity: EntityWithHealth { entity: EntityCommon { position: MapPosition::new(768+25, 200) , usage_bit_mask: 0, targeter: None }, damage_to_be_taken: 0.0, health: 0.0, upgrade_target: Entity::Nothing }, burn_progress: 0, tree_data: 0 })));
  map_data.map.surfaces[0].chunks[210].entities_to_be_inserted_before_setup.push((Entity::RockHuge, EntityData::RockHuge(SimpleEntity { entity: EntityWithHealth { entity: EntityCommon { position: MapPosition::new(768, 768) , usage_bit_mask: 0, targeter: None }, damage_to_be_taken: 0.0, health: 0.0, upgrade_target: Entity::Nothing }, variation: 0 })));
  map_data.map.surfaces[0].chunks[210].entities_to_be_inserted_before_setup.push((Entity::IronOre, EntityData::IronOre(ResourceEntity { entity: EntityCommon { position: MapPosition::new(768-128, 128) , usage_bit_mask: 0, targeter: None }, resource_amount: 1234, initial_amount: None, variation: 0 })));

  println!("Map data: {:?}", map_data);

  save_file.level_init_dat = map_data.write_map_data().unwrap();
  save_file.write_save_file("test").unwrap()
}

#[allow(dead_code)]
fn clean_up_save_file(name: &str) {
  let mut save_file = SaveFile::load_save_file(name).unwrap();

  let mut map_data = MapData::parse_map_data(&save_file.level_init_dat).unwrap();

  for force in 0..3 {
    map_data.map.force_manager.force_data_list[force].charts.clear();
    map_data.map.force_manager.force_data_list[force].build_count_statistics = None;

    map_data.map.force_manager.force_data_list[force].custom_prototypes.recipes.clear();
    map_data.map.force_manager.force_data_list[force].custom_prototypes.technologies.clear();

    map_data.map.force_manager.force_data_list[force].ammo_damage_modifiers.clear();
    map_data.map.force_manager.force_data_list[force].gun_speed_modifiers.clear();
    map_data.map.force_manager.force_data_list[force].turret_attack_modifiers.clear();
    map_data.map.force_manager.force_data_list[force].disabled_hand_crafting_recipes.clear();
  }
  map_data.map.surfaces[0].active_chunks.clear();
  map_data.map.surfaces[0].chunks.retain(|c| (c.position.x == -1 ||c.position.x == 0) && (c.position.y == -1 || c.position.y == 0));  // retain chunks around (0,0) to allow for proper player spwaning

  map_data.map.loaded_prototype_migrations_definition.clear();  // this will cause data migrations to be considered active
  { // requires active data migrations
    for p in 0..8 {
      map_data.map.pollution_statistics.precision[p].input_elements.clear();
      map_data.map.pollution_statistics.precision[p].output_elements.clear();
    }
    map_data.map.pollution_statistics.input_running_counts.clear();
    map_data.map.pollution_statistics.output_running_counts.clear();

    map_data.map.surfaces[0].compiled_map_gen_settings.serialized_data.clear();
  }

  {
    map_data.map.prototype_migrations.achievement_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.ammo_category_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.autoplace_control_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.custom_input_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.damage_type_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.decorative_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.entity_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.equipment_category_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.equipment_grid_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.equipment_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.fluid_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.fuel_category_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.item_group_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.item_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.item_sub_group_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.mod_settings_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.module_category_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.named_noise_expression_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.noise_layer_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.particle_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.recipe_category_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.recipe_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.resource_category_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.shortcut_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.technology_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.tile_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.trivial_smoke_id_migrations.mappings.clear();
    map_data.map.prototype_migrations.virtual_signal_id_migrations.mappings.clear();
  }
  map_data.map.shared_achievement_stats.achievements.clear();
  map_data.map.history.steps.clear();
  map_data.map.map_gen_settings.autoplace_controls.clear();

  // reset ticks to 0
  map_data.map.map_header.update_tick = 0;
  map_data.map.surfaces[0].map_generation_manager.orders_up_to_tick = 0.0;

  // Clear replay data
  let mut replay_data = ReplayData::parse_replay_data(&save_file.replay_dat).unwrap();
  replay_data.actions.clear();

  // Set up script data to skip crash site
  let mut script_data = ScriptData::parse_script_data(&save_file.script_init_dat).unwrap();
  script_data.lua_context.scripts = vec![(String::from("level"), LuaGameScript {
    had_control_lua: false,  // force initialization of control.lua scripts, including globals for items on player creation and silo script
    script_state: LuaGameScriptState { state_value: LuaValue::Table(vec![
      (LuaValue::String(String::from("skip_intro")), LuaValue::BoolTrue),  // disable freeplay intro message
      (LuaValue::String(String::from("disable_crashsite")), LuaValue::BoolTrue),  // disable crash site and cutscene, keep all starting items in player inventory
    ]) } })];

  // println!("Map data: {:?}", map_data);

  save_file.level_init_dat = map_data.write_map_data().unwrap();
  save_file.replay_dat = replay_data.write_replay_data().unwrap();
  save_file.script_init_dat = script_data.write_script_data().unwrap();
  save_file.write_save_file("1191template").unwrap()
}

#[allow(dead_code)]
fn load_and_save_script_test(name: &str) {
  let save_file = SaveFile::load_save_file(name).unwrap();

  let script_data = ScriptData::parse_script_data(&save_file.script_init_dat).unwrap();
  println!("Script data: {:#?}", script_data);
  println!("Unparsed script data size: {}", script_data.remaining_data.len());
  println!("Next non-null at index: {:?}", script_data.remaining_data.iter().position(|&x| x != 0));
  if let Some(non_null_pos) = script_data.remaining_data.iter().position(|&x| x != 0) {
    println!("Next non-null bytes: {:?}", &script_data.remaining_data[non_null_pos..non_null_pos+20]);
  }

  let serialized_script_data = script_data.write_script_data().unwrap();
  assert_eq!(serialized_script_data, save_file.script_init_dat);

  // save_file.write_save_file("test").unwrap()
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
  print_prototype_ids(&map_data.map.prototype_migrations.tile_id_migrations, "Tiles");
  // print_prototype_ids(&map_data.map.prototype_migrations.technology_id_migrations, "Technologies");
  // print_prototype_ids(&map_data.map.prototype_migrations.virtual_signal_id_migrations, "VirtualSignals");
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

  // save_file.write_save_file("test").unwrap()
}

#[allow(dead_code)]
fn create_test_replay() {
  let mut runner = Runner::from_template_map("1191template").unwrap();

  runner.input_actions.push(InputAction::new(0, 0, InputActionData::WriteToConsole(r#"/c game.write_file("test.dump", "command tick " .. game.tick .. " entity (" .. serpent.line(game.surfaces[1].find_entity("burner-mining-drill", {2, 0}) == nil) .. ")\n", true)"#.to_owned())));
  runner.build_miner_for(Item::IronOre, TilePosition::new(2, 0), Direction::South);
  runner.add_item(Item::Wood, 1, TilePosition::new(2, 0).top_left_map_position());
  // runner.build(Item::StoneFurnace, MapPosition::new(0x200, 0x200), Direction::South);
  runner.input_actions.push(InputAction::new(0, 0, InputActionData::WriteToConsole(r#"/c game.write_file("test.dump", "command tick " .. game.tick .. " entity (" .. serpent.line(game.surfaces[1].find_entity("burner-mining-drill", {2, 0}) == nil) .. ")\n", true)"#.to_owned())));

  // replay_data.actions.push(InputAction::new(0, 0, InputActionData::StartWalking(Direction::East)));
  // for tick in 0..11 {
  //   replay_data.actions.push(InputAction::new(tick, 0, InputActionData::WriteToConsole(r#"/c game.write_file("test.dump", "command tick " .. game.tick .. " walking_state (" .. serpent.line(game.player.walking_state) .. ", " .. game.player.walking_state.direction .. ")\n", true)"#.to_owned())));
  // }
  // replay_data.actions.push(InputAction::new(30, 0, InputActionData::StopWalking));
  // replay_data.actions.push(InputAction::new(3000, 0, InputActionData::StopWalking));

  runner.write_save_file("test").unwrap();
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
  let _bytes = ReplayData::from_input_actions(items).write_replay_data().unwrap();
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
  let _bytes = ReplayData::from_input_actions(items).write_replay_data().unwrap();
}
