local tile = require "tile"
local wall = require "wall"
local log = require "log"
-- local meta = require "meta"

log.warn "IDK what alpha's on, one entrypoint is good enough"
log.debug "use dash, not froge"
-- log.debug(meta.plugin_id)

tile.register {
    -- all default settings
    ["dirt"] = tile.default {},
    ["air"] = tile.default {},
    -- a full example of a tile setting
    ["alpha"] = tile.default {
        sprite = { "rustaria-core", "tile/dirt.png" },
        transitional = true,
        collision = {fixed = false},
        opaque = {fixed = false},
        blast_resistance = 69,
        break_resistance = "indestructible",
        tile_type = {
            type = "spreadable",
            spread_chance = 0.618,
            filter = {whitelist = {{"rustaria-core", "leo"}}}
        }
    }
}

wall.register {
    ["air"] = wall.default {},
    ["another"] = wall.default {}
}

log.error "that's it, lmao"
