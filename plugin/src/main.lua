local connected_blocks = require "connected_blocks";

if reload.client then
    reload.stargate.entity_renderer:register {
        ["player"] = {
            image = "image/entity/alpha.png",
            panel = {
                origin = { -0.9, -1.4 },
                size = { 1.8, 2.8 }
            }
        },
        ["arrow"] = {
            image = "image/entity/glisco.png",
            panel = {
                origin = { -0.5, -0.5 },
                size = { 1.0, 1.0 }
            }
        }
    }
    reload.stargate.block_layer_renderer:register {
        ["tile"] = {
            get_rect = connected_blocks["tile"].get_rect,
            get_uv = connected_blocks["tile"].get_uv,
            blocks = {
                ["dirt"] = {
                    image = "image/tile/dirt.png",
                    connection_type = "Connected"
                },
                ["stone"] = {
                    image = "image/tile/stone.png",
                    connection_type = "Connected"
                },
                ["grass"] = {
                    image = "image/tile/grass.png",
                    connection_type = "Connected",
                },
                ["corrupt_grass"] = {
                    image = "image/tile/corrupt_grass.png",
                    connection_type = "Connected",
                }
            }
        },
        [{ name = "wall", priority = 0 }] = {
            get_rect = connected_blocks["wall"].get_rect,
            get_uv = connected_blocks["wall"].get_uv,
            blocks = {
                ["dirt"] = {
                    image = "image/wall/dirt.png",
                    connection_type = "Connected"
                }
            }
        }
    }
end

reload.stargate.block_layer:register {
    ["tile"] = {
        collision = true,
        default = "air",
        blocks = {
            ["dirt"] = {
                collision = true,
            },
            ["stone"] = {
                collision = true,
            },
            ["grass"] = {
                collision = true
            },
            ["corrupt_grass"] = {
                collision = true,
                spread = {
                    chance = 10.0,
                    convert_table = {
                        ["dirt"] = "corrupt_grass"
                    }
                }
            },
            ["air"] = {
                collision = false,
            }
        }
    },
    [{ name = "wall", priority = 0 }] = {
        default = "air",
        collision = false,
        blocks = {
            ["dirt"] = {
                collision = true,
            },
            ["air"] = {
                collision = false,
            }
        }
    }
}

reload.stargate.entity:register {
    ["player"] = {
        position = { 24.0, 20.0 },
        velocity = {
            vel = { 0.0, 0.0 },
            accel = { 0.0, 0.0 },
        },
        collision = {
            collision_box = {
                origin = { -0.9, -1.4 },
                size = { 1.8, 2.8 }
            }
        },
        humanoid = {
            jump_amount = 15 / 60,
            jump_speed = 5.01 * 3.0,
            run_acceleration = 0.08 * 3.0,
            run_slowdown = 0.2 * 3.0,
            run_max_speed = 3.0 * 3.0,
        },
        gravity = {
            amount = 1.0
        }
    },
    ["arrow"] = {
        position = { 0.0, 0.0 },
        velocity = {
            vel = { 0.5, 0.0 },
            accel = { 0.0, 0.0 },
        },
        collision = {
            collision_box = {
                origin = { -0.55, -0.55 },
                size = { 1.1, 1.1 }
            },
            hit_callback = function(chunks)
                local tile_layer_id = api.carrier.block_layer:get_id("tile");
                local layer_p = api.carrier.block_layer:get(tile_layer_id);

                local stone_id = layer_p.blocks:get_id("stone");

                chunks:get_mut({
                    x = 0,
                    y = 0
                }):get_mut_layers():get_mut(tile_layer_id):set_entry({
                    x = 0,
                    y = 0
                }, layer_p.blocks:get(stone_id):create(stone_id));
            end
        },
        gravity = {
            amount = 1.0
        }
    }
}