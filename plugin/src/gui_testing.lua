local function inbit()

end

return {
    initialize = function(gui)
        gui:centered {
            gui:button {
                text = "hello",
                pressed = function()
                    log.info("hello");
                end
            },
        }
    end
}