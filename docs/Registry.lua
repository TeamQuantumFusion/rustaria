--- A Registry holds a key-value pair of values which are keyed by an [Identifier]
--- Internally a Registry uses a RawID which is an index in the registry because the registry internally is a Vector of the values and
--- a secondary value which holds an Identifier to ID and ID to Identifier map.
--- @shape Registry<V>

--- A RegistryBuilder is used for compiling Registries. These consume prototypes which get compiled on reload
--- which internally return a Type version of the Prototype which is faster to use and creates cached values.
--- @shape RegistryBuilder<V>
local registryBuilder = {}



--- A Registry key is a key entry into the RegistryBuilder.
--- A RegistryKey has an optional priority field which on nil would be set to the default value of (1000)
--- @alias RegistryKey Identifier | {name: Identifier, priority: number | nil }


--- Register registered a given key and value pair to the registry.
--- If an existing entry exists it will "layer" the values.
--- This means that non-existing values will be set and existing values will be overwritten.
--- @param key RegistryKey
--- @param value V
--- @return nil
function registryBuilder:register(key, value )
    -- stub
    return nil;
end