local block_states = require "block_states";

--- @type Stargate
local stargate = reload.stargate;

if reload.client then
    stargate.entity_renderer:register(
            "player",
            {
                test = { hello = {} },
                image = "image/entity/alpha.png",
                panel = {
                    origin = { -0.9, -1.4 },
                    size = { 1.8, 2.8 }
                }
            }
    );
    stargate.entity_renderer:register {
        ["player"] = {
            test = { hello = {} },
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

    stargate.block_layer_renderer:register({
        ["tile"] = {
            -- get_rect = connected_blocks["tile"].get_rect,
            -- get_uv = connected_blocks["tile"].get_uv,
            blocks = {
                ["dirt"] = {
                    image = "image/tile/dirt.png",
                    states = block_states.tile_client
                },
                ["stone"] = {
                    image = "image/tile/stone.png",
                    states = block_states.tile_client
                },
                ["grass"] = {
                    image = "image/tile/grass.png",
                    states = block_states.tile_client
                },
                ["corrupt_grass"] = {
                    image = "image/tile/corrupt_grass.png",
                    states = block_states.tile_client
                }
            }
        },
        [{ name = "wall", priority = 0 }] = {
            -- get_rect = connected_blocks["wall"].get_rect,
            -- get_uv = connected_blocks["wall"].get_uv,
            blocks = {
                ["dirt"] = {
                    image = "image/wall/glass.png",
                    states = block_states.tile_client
                }
            }
        }
    })
end

stargate.chunk_layer:register("tile",
            --- @type ChunkLayerPrototype
        {
            collision = true,
            default = "air",
            blocks = {
                ["dirt"] = {
                    collision = true,
                    states = block_states.tile,
                },
                ["stone"] = {
                    collision = true,
                    states = block_states.tile,
                },
                ["grass"] = {
                    collision = true,
                    states = block_states.tile,
                },
                ["corrupt_grass"] = {
                    collision = true,
                    states = block_states.tile,
                    spread = {
                        chance = 10.0,
                        convert_table = {
                            ["dirt"] = "corrupt_grass"
                        }
                    }
                },
                ["air"] = {
                    states = {
                        states = {
                            ["NONE"] = { name = "" }
                        },
                        default_state = "NONE"
                    },
                    collision = false,
                }
            }
        });

local blocks = new.RegistryBuilder({
    ["dirt"] = --[[---@type BlockPrototype]] {
        states = block_states.tile,
        collision = true,
    },
    ["air"] = --[[---@type BlockPrototype]] {
        states = {
            states = {
                ["NONE"] = { name = "" }
            },
            default_state = "NONE"
        },
        collision = false,
    }
});

stargate.chunk_layer:register({ name = "wall", priority = 1 },
        {
            default = "air",
            collision = false,
            blocks =
            {
                ["dirt"] =
                {
                    states = block_states.tile,
                    collision = true,
                },
                ["air"] = {
                    states = {
                        states = {
                            ["NONE"] = { name = "" }
                        },
                        default_state = "NONE"
                    },
                    collision = false,
                }
            }
        })

stargate.entity:register {
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
                --- @type Block
                local block = api.world:create_block("tile", "stone");
                local pos = api.util.new_block_pos(0, 0);
                chunks:get_mut(pos.chunk):set_block(pos.chunk, block);
            end
        },
        gravity = {
            amount = 1.0
        }
    }
}