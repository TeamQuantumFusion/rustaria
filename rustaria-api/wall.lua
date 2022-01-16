---Registers a wall to Rustaria.
---
---@param entries table<string, wall>
---@return nil
local function register(entries)
    -- builtin stub
end

---Creates a default wall
---
---@param settings default_wall_settings
---@return wall
local function default(settings)
    -- builtin stub
end

---@class default_wall_settings
---@field opaque dynamic_bool
---@field break_resistance break_resistance
local default_wall_settings = {}

return {register = register, default = default}
