local stuff = require "test.lua";
stuff.test();


info "Registering tiles."
Tiles:register {
    ["dirt"] = {
        sprite = "lab.png"
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
            height = 10,
        },
        velocity = {
            x = 0.5,
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
