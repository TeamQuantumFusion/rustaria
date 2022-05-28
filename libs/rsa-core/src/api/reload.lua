-- ============================== Reload global ==============================


--- @class Reload
--- @field registry table<string, RegistryDataBuilder>
--- @field hook HookBuilder
reload = {}




-- ============================== Registry ==============================
--- @alias RegistryBuilder table<string, RegistryDataBuilder>
local RegistryBuilder = {}

--- @field tag string
--- @return RegistryDataBuilder
function RegistryBuilder:__index(tag) end


--- @class RegistryDataBuilder
local RegistryDataBuilder = {}

--- Inserts prototype data into the registry. table<tag, prototype>
--- @param values table
--- @return nil
function RegistryDataBuilder:insert(values) end

-- ============================== Hooks ==============================

--- @class HookBuilder
