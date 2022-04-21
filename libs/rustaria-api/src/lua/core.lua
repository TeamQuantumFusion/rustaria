-- =============== Types ===============

--- @class Pos Entity position.
--- @field x number (f32)
--- @field y number (f32)

--- @class ChunkPos Chunk position in world.
--- @field x number (u16)
--- @field y number (u16)

--- @class ChunkSubPos Position in the Chunk grid.
--- @field x number (u4)
--- @field y number (u4)

--- @class Tag
--- @field value string

-- =============== Globals ===============
-- Here go the globals. Anything that starts with _ should never be touched.


-- Api reference
_api = {}

--- @class Plugin Information about your plugin.
--- @field id string
plugin = {}


-- =============== Logging ===============
-- Docs are forwarded from rust log library

--- Logs on the "trace" level.
---
--- Designates very low priority, often extremely verbose, information.
--- @param msg string
function trace(msg) end

--- Logs on the "debug" level.
---
--- Designates lower priority information.
--- @param msg string
function debug(msg) end

--- Logs on the "info" level.
---
--- Designates useful information.
--- @param msg string
function info(msg) end

--- Logs on the "warn" level.
---
--- Designates hazardous situations.
--- @param msg string
function warn(msg) end

--- Logs on the "error" level.
---
--- Designates very serious errors.
--- @param msg string
function error(msg) end