LANGUAGE_REGISTRY.add("en-us", {
    LanguageKey.new({ key = "rustaria.tile.dirt", value = "Dirt Block" }),
    LanguageKey.new({ key = "rustaria.tile.grass", value = "Grass Block" }),
    LanguageKey.new({ key = "rustaria.tile.stone", value = "Stone Block" }),
    LanguageKey.new({ key = "rustaria.wall.dirt", value = "Dirt Wall" }),
    LanguageKey.new({ key = "rustaria.wall.stone", value = "Stone Wall" }),
})

TILE_REGISTRY.add({
    Tile.new({
        id = "stone",
        name = "rustaria.tile.stone",
        -- sprite always starts in ./assets/ and ends in .png
        sprite = "tile/stone",
        -- ahead has defaults, and these values are the defaults
        collision = true,
        transitional = false,
        opaque = false,
    }),
    -- transitional means other tiles will transition to it
    Tile.new({ id = "dirt", name = "rustaria.tile.dirt", sprite = "tile/dirt", transitional = true }),
    Grass.new({ id = "grass", name = "rustaria.tile.grass", sprite = "tile/grass", }),
});

WALL_REGISTRY.add({
    Wall.new({ id = "dirt", name = "rustaria.wall.dirt", sprite = "wall/dirt", }),
    Wall.new({ id = "stone", name = "rustaria.wall.stone", sprite = "wall/stone", }),
});
