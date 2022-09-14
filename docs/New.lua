--- This contains all of the methods for creating a specific userdata.
--- @class New
new = {}


--- @generic V
--- @param values table<RegistryKey, V>
--- @return RegistryBuilder<V>
function new.RegistryBuilder(values)
    --- @type RegistryBuilder<V>
    local thing = {};
    return thing;
end