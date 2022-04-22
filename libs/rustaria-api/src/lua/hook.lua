--- @class HookInstance

--- @param target Tag
--- @param func #function
function hook:subscribe(target, func) end

-- All of the hooks are documented here.
-- $ is used instead of : for tags because emmylua annotations wont allow that.

--- Called every tick on the server.
function rustaria:tick() end