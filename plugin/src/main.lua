local log = require "log"

log.warn "IDK what alpha's on, one entrypoint is good enough"
log.debug "use dash, not froge"


Tiles:register {
    ["dirt"] = {
        sprite = "lab.png",
        connection = "connected"
    },
    ["air"] = {},
}

-- wall:register {wd
--    [_ "air"] = wall:default {},
--    [_ "another"] = wall:default {}
--}
--
-- entity:register {
--     ["player"] = entity:default {
--         health = {
--             maximum = 100
--         }
--     },
--     ["bunne"] = entity:default {
--         velocity = {
--             x = 0,
--             y = 0,
--         },
--         rendering = {
--             Static = {
--                 x_offset = 0,
--                 y_offset = 0,
--                 width = 10,
--                 height = 10,
--                 sprite = ":missing"
--             }
--         }
--     }
-- }

log.error "alpha kinda stinks"
