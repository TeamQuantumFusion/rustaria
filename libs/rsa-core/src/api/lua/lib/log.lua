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