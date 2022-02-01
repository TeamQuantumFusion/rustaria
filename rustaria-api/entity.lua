---Registers a entity to Rustaria.
---
---@param entries table<string, entity>
---@return nil
local function register(entries)
    -- builtin stub
end

---Creates a default entity
---
---@param settings entity_settings
---@return wall
local function default(settings)
    -- builtin stub
end

---@class entity_settings
---@field health health_settings
---@field physics physics_settings
local entity_settings = {}

---@class health_settings
---@field maximum number
---@field current ?number
local health_settings = {}

---@class physics_settings
local physics_settings = {}

return {register = register, default = default}
