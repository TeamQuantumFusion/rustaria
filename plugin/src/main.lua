local stuff = require "test";
stuff.test();

log.info "Registering tiles."
reload.registry.tile:insert {
    ["dirt"] = {
        sprite = "sprite/tile/dirt.png",
        collision = true
    },
    ["air"] = {}
}

log.info "Registering entities."
reload.registry.entity:insert {
    ["player"] = {
        health = {
            maximum = 100
        },
        hitbox = {
            x = 0,
            y = 0,
            width = 2,
            height = 3,
        },
        gravity = 1.0,
        velocity = {
            x = 0.0,
            y = 0.0,
        },
        humanoid = {
            jump_frames = 15,
            jump_speed = 20,
            run_acceleration = 4.8 * 60,
            run_slowdown = 1.2 * 60,
            run_max_speed = 12.0 * 60,
        },
        rendering = {
            Static = {
                x_offset = 0,
                y_offset = 0,
                width = 2,
                height = 3,
                sprite = "glisco.png"
            }
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
