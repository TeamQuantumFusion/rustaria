---Convenience function for getting the running plugin's ID.
---@return string
local function plugin_id()
    -- builtin stub
end

---Creates a tag for use in registries.
---
---Should only one string be supplied, it is treated as the name component, and
---the plugin ID component is instead retrieved from the `meta.plugin_id` function;
---should two strings be supplied, they are treated as the plugin ID and name components
---respectively.
---This function will panic should more than two strings be supplied.
---
---Examples:
---```
---local meta = require 'meta'
---local derpling = meta.tag('dumplings', 'derpling')
---print(tostring(derpling)) -- prints 'dumplings:derpling'
---
---local _ = meta._ -- for the sake of convenience
---local potsticker = _('dumplings', 'potsticker')
---print(tostring(potsticker)) -- prints 'dumplings:potsticker'
---
----- an even shorter way (using current plugin ID)
---local shorty = _ 'shorty'
---local even_shorter = _'even_shorter' -- space omitted
---print(tostring(shorty)) -- prints 'my-plugin:shorty'
---print(tostring(even_shorter)) -- prints 'my-plugin:even_shorter'
---```
---@vararg string one or two components of the resulting tag
---@return tag
local function tag(...)
    -- builtin stub
end

return {
    plugin_id = plugin_id,
    make_id = tag,
    _ = tag, -- alias
}