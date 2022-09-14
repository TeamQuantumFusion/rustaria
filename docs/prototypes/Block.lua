--- @shape BlockPrototype
--- @field collision boolean If entities collide with this block
--- @field states boolean If entities collide with this block

--- @shape ChunkLayerPrototype
--- @field blocks RegistryBuilder<BlockPrototype> The blocks in this chunk-layer
--- @field default Identifier The default tile to be used for an empty chunk layer.
--- @field collision boolean If entities should check collision on this layer.
local test = {}
