#!/bin/fish
# cursed buildscript for prototyping, will be replaced later
# if you have fish shell, run this in /run directory
# `cd ../rustaria-core/ && ./make_core.fish && cd ../run && cargo r`

cargo wasi build --release
mkdir plugin-pack-tmp
cd plugin-pack-tmp

# code
cp ../../target/wasm32-wasi/release/rustaria_core.wasm main.wasm

# manifest
cp ../manifest.json manifest.json

# pack it all up
zip rustaria-core *

if not test -d ../../run/plugins
    mkdir ../../run/plugins
end

mv rustaria-core.zip ../../run/plugins

cd ..
rm -rf plugin-pack-tmp
