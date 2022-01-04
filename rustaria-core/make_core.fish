#!/bin/fish
# cursed buildscript for prototyping, will be replaced later
# if you have fish shell, run this in /run directory
# `cd ../rustaria-core/ && ./make_core.fish && cd ../run && cargo r`

cargo wasi build --release
mkdir plugin-pack-tmp

# code
cp src/main.lua plugin-pack-tmp/main.wasm

# manifest
cp manifest.json plugin-pack-tmp/manifest.json

# pack it all up
cd plugin-pack-tmp
zip rustaria-core *
cd ..

if not test -d ../run/plugins
    mkdir ../run/plugins
end

mv rustaria-core.zip ../run/plugins
rm -rf plugin-pack-tmp
