local handler = require("event_handler")
handler.add_lib(require("freeplay"))
handler.add_lib(require("silo-script"))

script.on_event(defines.events.on_tick, function(event)
  -- game.write_file("test.dump", "on_tick tick " .. event.tick .. " position (" .. game.players[1].position.x .. ", " .. game.players[1].position.y .. ")\n", true)
  -- game.write_file("test.dump", "on_tick tick " .. game.tick .. " walking_state (" .. serpent.line(game.players[1].walking_state) .. ", " .. game.players[1].walking_state.direction .. ")\n", true)
  game.write_file("test.dump", "on_tick tick " .. event.tick .. " " .. burner_miner_to_string(game.surfaces[1].find_entity("burner-mining-drill", {2, 0})) .. "\n", true)
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
