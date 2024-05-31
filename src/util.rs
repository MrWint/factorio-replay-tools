use std::fmt::Display;

use factorio_serialize::{constants::Tile, map::{ActiveMigrations, MapData}, replay::ReplayData, save::SaveFile, script::{LuaGameScript, LuaGameScriptState, LuaValue, ScriptData}};



#[allow(dead_code)]
pub fn load_and_save_map_test(name: &str, outname: &str) {
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

  save_file.write_save_file(outname).unwrap()
}
#[allow(dead_code)]
pub fn load_and_save_replay_test(name: &str) {
  let save_file = SaveFile::load_save_file(name).unwrap();

  let replay_data = ReplayData::parse_replay_data(&save_file.replay_dat).unwrap();
  println!("Replay data: {:?}", replay_data);

  let serialized_replay_data = replay_data.write_replay_data().unwrap();
  assert_eq!(serialized_replay_data, save_file.replay_dat);
}

#[allow(dead_code)]
pub fn load_and_save_script_test(name: &str) {
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
}


#[allow(dead_code)]
pub fn export_prototypes(name: &str) {
  let save_file = SaveFile::load_save_file(name).unwrap();
  let map_data = MapData::parse_map_data(&save_file.level_init_dat).unwrap();

  print_prototype_ids(&map_data.map.prototype_migrations.achievement_id_migrations, "Achievements");
  print_prototype_ids(&map_data.map.prototype_migrations.decorative_id_migrations, "Decoratives");
  print_prototype_ids(&map_data.map.prototype_migrations.entity_id_migrations, "Enitites");
  print_prototype_ids(&map_data.map.prototype_migrations.equipment_id_migrations, "Equipment");
  print_prototype_ids(&map_data.map.prototype_migrations.fluid_id_migrations, "Fluids");
  print_prototype_ids(&map_data.map.prototype_migrations.item_id_migrations, "Items");
  print_prototype_ids(&map_data.map.prototype_migrations.item_group_id_migrations, "ItemGroups");
  print_prototype_ids(&map_data.map.prototype_migrations.recipe_id_migrations, "Recipes");
  print_prototype_ids(&map_data.map.prototype_migrations.tile_id_migrations, "Tiles");
  print_prototype_ids(&map_data.map.prototype_migrations.technology_id_migrations, "Technologies");
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
pub fn clean_up_save_file(name: &str, outname: &str) {
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
  map_data.map.surfaces[0].chunks.retain(|c| (c.position.x == -1 || c.position.x == 0) && (c.position.y == -1 || c.position.y == 0));  // retain chunks around (0,0) to allow for proper player spwaning
  for chunk in map_data.map.surfaces[0].chunks.iter_mut() {
    chunk.entities_to_be_inserted_before_setup.clear();  // remove all entities from the chunks
    chunk.tick_of_last_change_that_could_affect_charting = 0;
    for x in 0..32 {  // set tiles to Lab floor pattern
      for y in 0..32 {
        chunk.tiles[x][y] = ([Tile::LabDark1, Tile::LabDark2][(x + y) & 1], 0x10);
      }
    }
  }

  map_data.map.loaded_prototype_migrations_definition.clear();  // this will cause data migrations to be considered active; disables ability to save replays when playing the save, but can still be watched and saved while watching
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
  save_file.write_save_file(outname).unwrap()
}
