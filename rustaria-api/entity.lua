---Registers a entity to Rustaria.
---
---@param entries table<tag, entity>
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
---@field physics ?physics_settings
local entity_settings = {}

---@class health_settings
---@field maximum number the maximum amount of health the entity could have.
---@field current ?number the amount of health the entity should spawn with, defaults to maximum
local health_settings = {}

---@class physics_settings
local physics_settings = {}

return {register = register, default = default}
