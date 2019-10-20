RNG manipulation: level.dat at 0x3a0 (with level name length 12) stores RNG seed (3x4 bytes)


/command function out(str) game.write_file("recipes.lua", str .. '\n', true); end; function print_table(table) for key,value in pairs(table) do if type(value) == 'table' then out(key .. " = {"); print_table(value); out("},"); else out(key .. " = " .. value .. ","); end; end; end; print_table(game.player.force.recipes)


/c game.write_file("defines.dump", serpent.dump(defines));

/c game.write_file("recipes.dump", serpent.block(game.player.force.recipes));

Dump entities:
/c local i = 1
game.write_file("entities.dump", "")
for _, entity in pairs(game.entity_prototypes) do
  game.write_file("entities.dump", entity.name .. " = " .. i .. ",\n", true)
  i = i + 1
end

Dump fluids:
/c local i = 1
game.write_file("fluids.dump", "")
for _, fluid in pairs(game.fluid_prototypes) do
  game.write_file("fluids.dump", fluid.name .. " = " .. i .. ",\n", true)
  i = i + 1
end

Dump items:
/c local i = 1
game.write_file("items.dump", "")
for _, item in pairs(game.item_prototypes) do
  game.write_file("items.dump", item.name .. " = " .. i .. ",\n", true)
  i = i + 1
end

Dump item_groups:
/c local i = 1
game.write_file("item_groups.dump", "")
for _, item_group in pairs(game.item_group_prototypes) do
  game.write_file("item_groups.dump", item_group.name .. " = " .. i .. ",\n", true)
  i = i + 1
end

Dump recipes:
/c local i = 1
game.write_file("recipes.dump", "")
for _, recipe in pairs(game.recipe_prototypes) do
  game.write_file("recipes.dump", recipe.name .. " = " .. i .. ",\n", true)
  i = i + 1
end

Dump technologies:
/c local i = 1
game.write_file("technologies.dump", "")
for _, technology in pairs(game.technology_prototypes) do
  game.write_file("technologies.dump", technology.name .. " = " .. i .. ",\n", true)
  i = i + 1
end

Dump tiles:
/c local i = 1
game.write_file("tiles.dump", "")
for _, tile in pairs(game.tile_prototypes) do
  game.write_file("tiles.dump", tile.name .. " = " .. i .. ",\n", true)
  i = i + 1
end

Dump virtual_signals:
/c local i = 1
game.write_file("virtual_signals.dump", "")
for _, virtual_signal in pairs(game.virtual_signal_prototypes) do
  game.write_file("virtual_signals.dump", virtual_signal.name .. " = " .. i .. ",\n", true)
  i = i + 1
end
