rem cursed buildscript for prototyping, will be replaced later
rem if you have fish shell, run this in /run directory
rem `cd ../rustaria-core/ && ./make_core.fish && cd ../run && cargo r`

rem use fish or be froge

rem oh and for some goddamn reason, windows doesn't ship with a tool that
rem packs zip files *despite being perfectly able to do it in file explorer*,
rem so this script assumes having 7zip installed in PATH.
mkdir plugin-pack-tmp

rem code
copy src/main.lua plugin-pack-tmp/main.lua

rem manifest
copy manifest.json plugin-pack-tmp/manifest.json

rem pack it all up
cd plugin-pack-tmp
7z a -tzip rustaria-core.zip *
cd ..

if not exist "../run/plugins" mkdir "../run/plugins"
move plugin-pack-tmp/rustaria-core.zip ../run/plugins

rmdir /s plugin-pack-tmp
