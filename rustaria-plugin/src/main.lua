local log = require "log"
local meta = require "meta"
local _ = meta._

log.warn "IDK what alpha's on, one entrypoint is good enough"
log.debug "use dash, not froge"
log.debug(tostring(_("hey", "this_works")))

tile:register {
    -- all default settings
    [_ "dirt"] = tile:default {},
    [_ "air"] = tile:default {},
    -- a full example of a tile setting
    -- [_"alpha"] = tile.default {
    --     sprite = _"tile/dirt.png",
    --     transitional = true,
    --     collision = {fixed = false},
    --     opaque = {fixed = false},
    --     blast_resistance = 69,
    --     break_resistance = "indestructible",
    --     tile_type = {
    --         type = "spreadable",
    --         spread_chance = 0.618,
    --         filter = {whitelist = { _"leo", _"froge" }}
    --     }
    -- }
}

wall:register {
    [_ "air"] = wall:default {},
    [_ "another"] = wall:default {}
}

entity:register {
    [_ "player"] = entity:default {
        health = {
            maximum = 100
        }
    },
    [_ "bunne"] = entity:default {
        health = {
            maximum = 5
        }
    }
}

log.error "that's it, lmao"
