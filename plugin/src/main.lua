local stuff = require "test.lua";
stuff.test();


info "Registering tiles."
Tiles:register {
    ["dirt"] = {
        sprite = "lab.png",
        collision = true
    },
    ["air"] = {}
}

info "Registering entities."
Entities:register {
    ["player"] = {
        health = {
            maximum = 100
        }
    },
    ["bunne"] = {
        hitbox = {
            x = 0,
            y = 0,
            width = 10,
            height = 3,
        },
        velocity = {
            x = 3.0,
            y = 0.2,
        },
        rendering = {
            Static = {
                x_offset = 0,
                y_offset = 0,
                width = 10,
                height = 3,
                sprite = "glisco.png"
            }
        }
    }
}
