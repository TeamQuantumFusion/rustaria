local tile = require "tile"
local wall = require "wall"
local log = require "log"

log.warn "IDK what alpha's on, one entrypoint is good enough"
log.debug "use dash, not froge"

tile.register {
    -- all default settings
    ["dirt"] = tile.default {},
    ["air"] = tile.default {},

    -- a full example of a tile setting
    ["alpha"] = tile.default {
        sprite = { mod_id, "tile/dirt.png" },
        transitional = true,
        collision = { fixed = false },
        opaque = { fixed = false },
        blast_resistance = 69,
        break_resistance = "indestructible",
        tile_type = {
            type = "spreadable",
            spread_chance = 0.618,
            filter = { whitelist = { { mod_id, "leo" } } }
        }
    }
}

wall.register {
    ["air"] = wall.default {},
    ["another"] = wall.default {},
}

log.error "that's it, lmao"
