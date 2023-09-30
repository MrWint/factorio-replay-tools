local handler = require("event_handler")
handler.add_lib(require("freeplay"))
handler.add_lib(require("silo-script"))

-- script.on_event(defines.events.on_tick, function(event)
  -- game.write_file("test.dump", "on_tick tick " .. event.tick .. " position (" .. game.players[1].position.x .. ", " .. game.players[1].position.y .. ")\n", true)
  -- game.write_file("test.dump", "on_tick tick " .. game.tick .. " walking_state (" .. serpent.line(game.players[1].walking_state) .. ", " .. game.players[1].walking_state.direction .. ")\n", true)
-- end)
  