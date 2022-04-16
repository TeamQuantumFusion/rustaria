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


Entities:register {
    ["player"] = {
        health = {
            maximum = 100
        }
    },
    ["bunne"] = {
        velocity = {
            x = 0.1,
            y = 0,
        },
        rendering = {
            Static = {
                x_offset = 0,
                y_offset = 0,
                width = 10,
                height = 10,
                sprite = "glisco.png"
            }
        }
    }
}

log.error "alpha kinda stinks"
