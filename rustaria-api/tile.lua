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

---@class default_tile_settings
---@field transitional boolean
---@field collision dynamic_bool
---@field opaque dynamic_bool
---@field blast_resistance blast_resistance
---@field break_resistance break_resistance
local default_tile_settings = {}

return {register = register, default = default}
