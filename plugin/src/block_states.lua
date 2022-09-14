local tile_pos_lookup = {
    ["Solid"] = { 1 / 4, 1 / 4 },
    ["Lonely"] = { 3 / 4, 3 / 4 },
    ["Vertical"] = { 3 / 4, 1 / 4 },
    ["Horizontal"] = { 1 / 4, 3 / 4 },
    ["CapTop"] = { 3 / 4, 0 / 4 },
    ["CapLeft"] = { 0 / 4, 3 / 4 },
    ["CapDown"] = { 3 / 4, 2 / 4 },
    ["CapRight"] = { 2 / 4, 3 / 4 },
    ["WallTop"] = { 1 / 4, 0 / 4 },
    ["WallDown"] = { 1 / 4, 2 / 4 },
    ["WallLeft"] = { 0 / 4, 1 / 4 },
    ["WallRight"] = { 2 / 4, 1 / 4 },
    ["CornerTopLeft"] = { 0 / 4, 0 / 4 },
    ["CornerTopRight"] = { 2 / 4, 0 / 4 },
    ["CornerDownLeft"] = { 0 / 4, 2 / 4 },
    ["CornerDownRight"] = { 2 / 4, 2 / 4 }
};

local wall_pos_lookup = {
    ["Solid"] = { 12, 12 },
    ["Lonely"] = { 32, 32 },
    ["Vertical"] = { 32, 12 },
    ["Horizontal"] = { 12, 32 },
    ["CapTop"] = { 32, 0 },
    ["CapLeft"] = { 0, 32 },
    ["CapDown"] = { 32, 20 },
    ["CapRight"] = { 20, 32 },
    ["WallTop"] = { 12, 0 },
    ["WallLeft"] = { 0, 12 },
    ["WallDown"] = { 12, 20 },
    ["WallRight"] = { 20, 12 },
    ["CornerTopLeft"] = { 0, 0 },
    ["CornerTopRight"] = { 20, 0 },
    ["CornerDownLeft"] = { 0, 20 },
    ["CornerDownRight"] = { 20, 20 },
};

local wall_size_lookup = {
    ["Solid"] = { 8, 8 },
    ["Lonely"] = { 16, 16 },
    ["Vertical"] = { 16, 8 },
    ["Horizontal"] = { 8, 16 },
    ["WallTop"] = { 8, 12 },
    ["WallDown"] = { 8, 12 },
    ["WallLeft"] = { 12, 8 },
    ["WallRight"] = { 12, 8 },
    ["CapTop"] = { 16, 12 },
    ["CapDown"] = { 16, 12 },
    ["CapLeft"] = { 12, 16 },
    ["CapRight"] = { 12, 16 },
    ["CornerTopLeft"] = { 12, 12 },
    ["CornerTopRight"] = { 12, 12 },
    ["CornerDownLeft"] = { 12, 12 },
    ["CornerDownRight"] = { 12, 12 },
};

local wall_rect_pos_lookup = {
    ["Solid"] = { 0, 0 },
    ["Lonely"] = { -0.5, -0.5 },
    ["Vertical"] = { -0.5, 0 },
    ["Horizontal"] = { 0, -0.5 },
    ["WallTop"] = { 0, 0 },
    ["WallDown"] = { 0, -0.5 },
    ["WallLeft"] = { -0.5, 0 },
    ["WallRight"] = { 0, 0 },
    ["CapTop"] = { -0.5, 0 },
    ["CapDown"] = { -0.5, -0.5 },
    ["CapLeft"] = { -0.5, -0.5 },
    ["CapRight"] = { 0, -0.5 },
    ["CornerTopLeft"] = { -0.5, 0 },
    ["CornerTopRight"] = { 0, 0 },
    ["CornerDownLeft"] = { -0.5, -0.5 },
    ["CornerDownRight"] = { 0, -0.5 },
};

local function get_tile(kind)
    local value = tile_pos_lookup[kind];
    local uv = {
        origin = {
            value[1],
            0.75 - value[2]
        },
        size = { 0.25, 0.25 }
    };
    local rect = {
        origin = { 0.0, 0.0 },
        size = { 1.0, 1.0 },
    };
    return {
        uv = uv,
        rect = rect,
    }
end

return {
    tile = {
        states = {
            ["Solid"] = { name = "" },
            ["Lonely"] = { name = "" },
            ["Vertical"] = { name = "" },
            ["Horizontal"] = { name = "" },
            ["WallTop"] = { name = "" },
            ["WallDown"] = { name = "" },
            ["WallLeft"] = { name = "" },
            ["WallRight"] = { name = "" },
            ["CapTop"] = { name = "" },
            ["CapDown"] = { name = "" },
            ["CapLeft"] = { name = "" },
            ["CapRight"] = { name = "" },
            ["CornerTopLeft"] = { name = "" },
            ["CornerTopRight"] = { name = "" },
            ["CornerDownLeft"] = { name = "" },
            ["CornerDownRight"] = { name = "" },
        },
        default_state = "Solid"
    },
    tile_client = {
        ["Solid"] = get_tile("Solid"),
        ["Lonely"] = get_tile("Lonely"),
        ["Vertical"] = get_tile("Vertical"),
        ["Horizontal"] = get_tile("Horizontal"),
        ["WallTop"] = get_tile("WallTop"),
        ["WallDown"] = get_tile("WallDown"),
        ["WallLeft"] = get_tile("WallLeft"),
        ["WallRight"] = get_tile("WallRight"),
        ["CapTop"] = get_tile("CapTop"),
        ["CapDown"] = get_tile("CapDown"),
        ["CapLeft"] = get_tile("CapLeft"),
        ["CapRight"] = get_tile("CapRight"),
        ["CornerTopLeft"] = get_tile("CornerTopLeft"),
        ["CornerTopRight"] = get_tile("CornerTopRight"),
        ["CornerDownLeft"] = get_tile("CornerDownLeft"),
        ["CornerDownRight"] = get_tile("CornerDownRight"),
    }
    -- ["tile"] = {
    --        get_rect = function(kind)
    --            return {
    --                origin = { 0.0, 0.0 },
    --                size = { 1.0, 1.0 },
    --            };
    --        end,
    --        get_uv = function(kind)
    --            local value = tile_pos_lookup[kind];
    --            return { origin = {
    --                value[1],
    --                0.75 - value[2]
    --            }, size = { 0.25, 0.25 } };
    --
    --        end
    --    },
    --    ["wall"] = {
    --        get_rect = function(kind)
    --            local size = wall_size_lookup[kind];
    --            local pos = wall_rect_pos_lookup[kind];
    --            return {
    --                origin = { pos[1], pos[2] },
    --                size = { size[1] / 8, size[2] / 8 },
    --            };
    --        end,
    --        get_uv = function(kind)
    --            local pos = wall_pos_lookup[kind];
    --            local size = wall_size_lookup[kind];
    --            return { origin = {
    --                pos[1] / 48,
    --                ((48 - size[2]) - pos[2]) / 48
    --            }, size = {
    --                size[1] / 48,
    --                size[2] / 48
    --            } };
    --        end
    --    }
};