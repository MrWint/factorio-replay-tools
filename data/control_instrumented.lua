local handler = require("event_handler")
handler.add_lib(require("freeplay"))
handler.add_lib(require("silo-script"))

script.on_event(defines.events.on_tick, function(event)
  -- game.write_file("test.dump", "on_tick tick " .. event.tick .. " position (" .. game.players[1].position.x .. ", " .. game.players[1].position.y .. ")\n", true)
  -- game.write_file("test.dump", "on_tick tick " .. game.tick .. " walking_state (" .. serpent.line(game.players[1].walking_state) .. ", " .. game.players[1].walking_state.direction .. ")\n", true)
  -- log_print("on_tick tick " .. event.tick .. " " .. burner_miner_to_string(game.surfaces[1].find_entity("burner-mining-drill", {2, 2})))
  -- game.write_file("test.dump", "on_tick tick " .. event.tick .. " " .. stone_furnace_to_string(game.surfaces[1].find_entity("stone-furnace", {2, 2})) .. "\n", true)
end)

function burner_miner_to_string(entity)
  if entity == nil then
    return serpent.line(entity)
  else
    return entity.object_name .. serpent.line({ has_mining_target = entity.mining_target ~= nil, mining_progress = entity.mining_progress, energy = entity.energy, temperature = entity.temperature, burner = lua_burner_to_string(entity.burner) }, { numformat = '%a' })
  end
end
function stone_furnace_to_string(entity)
  if entity == nil then
    return serpent.line(entity)
  else
    return entity.object_name .. serpent.line({ energy = entity.energy, burner = lua_burner_to_string(entity.burner) }, { numformat = '%a' })
  end
end
function lua_burner_to_string(entity)
  if entity == nil then
    return serpent.line(entity)
  else
    return entity.object_name .. serpent.line({ heat = entity.heat, heat_capacity = entity.heat_capacity, remaining_burning_fuel = entity.remaining_burning_fuel, fuel_categories = entity.fuel_categories }, { numformat = '%a' })
  end
end

function assert_tick(expected_tick)
  if game.tick ~= expected_tick then
    log_print("tick " .. game.tick .. ": expected tick " .. expected_tick)
  end
end
function assert_player_position(expected_x, expected_y)
  if game.player.position.x * 256 ~= expected_x or game.player.position.y * 256 ~= expected_y then
    log_print("tick " .. game.tick .. ": expected position (" .. expected_x .. ", " .. expected_y .. "), but found (" .. game.player.position.x * 256 .. ", " .. game.player.position.y * 256 .. ")")
  end
end
function assert_player_mining_progress(expected_progress)
  if game.player.character_mining_progress ~= expected_progress then
    log_print(string.format("tick %d: expected mining progress %a, but found %a", game.tick, expected_progress, game.player.character_mining_progress))
  end
end
function assert_player_crafting_queue_size(expected_size)
  if game.player.crafting_queue_size ~= expected_size then
    log_print(string.format("tick %d: expected crafting queue size %d, but found %d", game.tick, expected_size, game.player.crafting_queue_size))
  end
end
function assert_player_crafting_progress(expected_progress)
  if game.player.crafting_queue_progress ~= expected_progress then
    log_print(string.format("tick %d: expected crafting progress %a, but found %a", game.tick, expected_progress, game.player.crafting_queue_progress))
  end
end
function assert_player_inventory_item_count(expected_item, expected_amount)
  if game.player.get_main_inventory().get_item_count(expected_item) ~= expected_amount then
    log_print(string.format("tick %d: expected inventory item %s count %d, but found %d", game.tick, expected_item, expected_amount, game.player.get_main_inventory().get_item_count(expected_item)))
  end
end
function assert_miner_mining_progress(x, y, expected_progress)
  local miner = game.surfaces[1].find_entity("burner-mining-drill", {x, y})
  if miner.mining_progress ~= expected_progress then
    log_print(string.format("tick %d: expected miner at (%d, %d) mining progress %a, but found %a", game.tick, x, y, expected_progress, miner.mining_progress))
  end
end
function assert_miner_remaining_burning_fuel(x, y, expected_remaining_fuel)
  local miner = game.surfaces[1].find_entity("burner-mining-drill", {x, y})
  if miner.burner.remaining_burning_fuel ~= expected_remaining_fuel then
    log_print(string.format("tick %d: expected miner at (%d, %d) remaining fuel %a, but found %a", game.tick, x, y, expected_remaining_fuel, miner.burner.remaining_burning_fuel))
  end
end
function assert_miner_heat(x, y, expected_heat)
  local miner = game.surfaces[1].find_entity("burner-mining-drill", {x, y})
  if miner.burner.heat ~= expected_heat then
    log_print(string.format("tick %d: expected miner at (%d, %d) heat %a, but found %a", game.tick, x, y, expected_heat, miner.burner.heat))
  end
end
function log_print(message)
  log(message)
  game.print(message)
end
function ppp()
  game.print("(" .. game.player.position.x * 256 .. ", " .. game.player.position.y * 256 .. ")")
end
