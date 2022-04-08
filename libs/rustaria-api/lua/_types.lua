---Just a bunch of type definitions for EmmyLua.

---@alias sprite asset
---@alias blast_resistance number | '"indestructible"'
---@alias break_resistance '"any"' | '"indestructible"' | { axe: number } | { pickaxe: number } | { hammer: number }
---@alias dynamic_bool '"dynamic"' | { fixed: boolean }
---@alias tile_filter '"all"' | '"none"' | { whitelist: tag[] } | { blacklist: tag[] }
---@alias tile_type '"default"' | { type: '"spreadable"', spread_chance: number, filter: tile_filter }

---@alias tile userdata
---@alias wall userdata
---@alias entity userdata
local dummy = {}

---@class tag
---@field mod_id string
---@field obj_id string
local tag = {}

---@class asset
---@field mod_id string
---@field asset_path string
local asset = {}