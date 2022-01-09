log.info("IDK what alpha's on, one entrypoint is good enough")
tile.register {
    ["dirt"] = tile.default {
        blast_resistance = "indestructible",
        break_resistance = { type = "any" },
    },
}