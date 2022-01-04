rem cursed buildscript for prototyping, will be replaced later
rem if you have fish shell, run this in /run directory
rem `cd ../rustaria-core/ && ./make_core.fish && cd ../run && cargo r`

rem use fish or be froge

rem oh and for some goddamn reason, windows doesn't ship with a tool that
rem packs zip files *despite being perfectly able to do it in file explorer*,
rem so this script assumes having 7zip installed in PATH.

cargo wasi build --release
mkdir plugin-pack-tmp
cd plugin-pack-tmp

rem code
copy ../../target/wasm32-wasi/release/rustaria_core.wasm main.wasm

rem manifest
copy ../manifest.json manifest.json

rem pack it all up
7z a -tzip rustaria-core.zip *

if not exist "../../run/plugins" mkdir "../../run/plugins"
move rustaria-core.zip ../../run/plugins

cd ..
rmdir /s plugin-pack-tmp
