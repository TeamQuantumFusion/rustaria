local tile = require "tile"
local log = require "log"

log.warn "IDK what alpha's on, one entrypoint is good enough"
log.debug "use dash, not froge"

tile.register {
    -- all default settings
    ["dirt"] = tile.default {},

    -- a full example of a tile setting
    ["alpha"] = tile.default {
        transitional = true,
        collision = "dynamic",
        opaque = {fixed = false},
        blast_resistance = 69,
        break_resistance = "indestructible",
        tile_type = {
            type = "spreadable",
            spread_chance = 0.618,
            filter = {whitelist = {{tag = "leo", category = "tile"}}}
        }
    }
}

log.error "that's it, lmao"
