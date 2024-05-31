RNG manipulation: level.dat at 0x3a0 (with level name length 12) stores RNG seed (3x4 bytes)


/command function out(str) game.write_file("recipes.lua", str .. '\n', true); end; function print_table(table) for key,value in pairs(table) do if type(value) == 'table' then out(key .. " = {"); print_table(value); out("},"); else out(key .. " = " .. value .. ","); end; end; end; print_table(game.player.force.recipes)


/c game.write_file("defines.dump", serpent.dump(defines));

/c game.write_file("recipes.dump", serpent.block(game.player.force.recipes));


Version update procedure:
- Create Scenario with lab tiles
- Create a test replay of the scenario
- Use util::load_and_save_map_test to try and load the map, find and fix all map data format changes (pipe to tmp file for easy viewing)
- Use util::export_prototypes to check for changed IDs (pipe to tmp file for easy viewing)
- Use util::clean_up_save_file to create minimal template version of a map to capture default settings and values
- Dump data.raw by installing DataRawJson and running ".\factorio.exe --instrument-mod DataRawJson", fix json Infinity errors by making them strings, and format