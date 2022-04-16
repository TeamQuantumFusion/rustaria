---Logs an trace-level message
---Reserved for very low priority, often extremely verbose.
---@param msg string
---@return nil
local function trace(msg)
    -- builtin stub
end

---Logs an debug-level message.
---Reserved for lower priority information.
---@param msg string
---@return nil
local function debug(msg)
    -- builtin stub
end

---Logs an info-level message.
---Reserved for useful information.
---@param msg string
---@return nil
local function info(msg)
    -- builtin stub
end

---Logs an warning-level message.
---Reserved for hazardous situations.
---@param msg string
---@return nil
local function warn(msg)
    -- builtin stub
end

---Logs an error-level message.
---Reserved for very serious errors.
---@param msg string
---@return nil
local function error(msg)
    -- builtin
end

return { trace = trace, debug = debug, info = info, warn = warn, error = error }
