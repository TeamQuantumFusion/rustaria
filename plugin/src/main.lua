local log = require "log"
local meta = require "meta"
local _ = meta._

log.warn "IDK what alpha's on, one entrypoint is good enough"
log.debug "use dash, not froge"
log.debug(tostring(_("hey", "this_works")))

tile:register {
    -- all default settings
    [_ "dirt"] = tile:default {
        sprite = tostring(_("missing.png")),
    },
    [_ "stone"] = tile:default {
        sprite = tostring(_("stone.png")),
    },
    [_ "air"] = tile:default {},
    -- a full example of a tile setting
    -- [_ "alpha"] = tile:default {
    --     sprite = _ "tile/dirt.png",
    --     transitional = true,
    --     collision = { fixed = false },
    --     opaque = { fixed = false },
    --     blast_resistance = 69,
    --     break_resistance = "indestructible",
    --     tile_type = {
    --         type = "spreadable",
    --         spread_chance = 0.618,
    --         filter = { whitelist = { _ "leo", _ "froge" } }
    --     }
    -- }
}

-- wall:register {
--    [_ "air"] = wall:default {},
--    [_ "another"] = wall:default {}
--}
--
entity:register {
    [_ "player"] = entity:default {
        health = {
            maximum = 100
        }
    },
    [_ "bunne"] = entity:default {
        velocity = {
            x = 0,
            y = 0,
        },
        rendering = {
            Static = {
                x_offset = 0,
                y_offset = 0,
                width = 10,
                height = 10,
                sprite = tostring(_("missing.png"))
            }
        }
    }
}

log.error "alpha kinda stinks"
