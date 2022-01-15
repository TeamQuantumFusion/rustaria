---Registers a tile to Rustaria.
---
---@param entries table<string, tile>
---@return nil
local function register(entries)
    -- builtin stub
end

---Creates a default tile
---
---@param settings default_tile_settings
---@return tile
local function default(settings)
    -- builtin stub
end

-- Type annotations
---@alias sprite asset
---@alias blast_resistance number | '"indestructible"'
---@alias break_resistance '"any"' | '"indestructible"' | { axe: number } | { pickaxe: number } | { hammer: number }
---@alias dynamic_bool '"dynamic"' | { fixed: boolean }]
---@alias tile_filter '"all"' | '"none"' | { whitelist: tag[] } | { blacklist: tag[] }
---@alias tile_type '"default"' | { type: '"spreadable"', spread_chance: number, filter: tile_filter }
---@alias tile userdata
local dummy = {}

---@class default_tile_settings
---@field transitional boolean
---@field collision dynamic_bool
---@field opaque dynamic_bool
---@field blast_resistance blast_resistance
---@field break_resistance break_resistance
local default_tile_settings = {}

return {register = register, default = default}
