---Convenience function for getting the running plugin's ID.
---@return string
local function plugin_id()
    -- builtin stub
end

---Helper function that creates a tag that's used in registries.
---Tags follow the syntax `{plugin ID}:{name}`, where the plugin ID component is substituted in
---with the actual, current plugin ID at runtime.
---
---@param name string the name, or the second component of the tag
---@return string
local function make_id(name)
    -- builtin stub; actual implementation may vary

    return plugin_id() .. ":" .. name
end

-- Quick alias
local _ = make_id;

return {
    plugin_id = plugin_id
}