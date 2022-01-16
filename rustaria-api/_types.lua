---@alias sprite asset
---@alias blast_resistance number | '"indestructible"'
---@alias break_resistance '"any"' | '"indestructible"' | { axe: number } | { pickaxe: number } | { hammer: number }
---@alias dynamic_bool '"dynamic"' | { fixed: boolean }]
---@alias tile_filter '"all"' | '"none"' | { whitelist: tag[] } | { blacklist: tag[] }
---@alias tile_type '"default"' | { type: '"spreadable"', spread_chance: number, filter: tile_filter }

---@alias tile userdata
---@alias wall userdata
local dummy = {}
