# Rustaria
A Terraria rework in Rust.

## Features
### Lua Modding API
Rustaria has been designed from the ground up to be modular and easily support mods with unique experiences. For docs you can find them not here as they are not done yet. lol
### Made in Rust
The core is written in Rust which brings very lightweight servers and super high performance clients.


## Contributing
Contributions are always welcome!

## Building rustatia
To build rustaria you need to have the rust toolchain installed on your OS. Please do so using `rustup`.
### Prerequisite
- Rust Toolchain (use `rustup` please)
- An internet connection.
- A PC with a keyboard, mouse and monitor.

### Assets
Clone both `rustaria` and `oxidizer`, then build oxidizer and move the binary (./target/release) to your Terraria Content directory. 
```bash
# Windows
C:/Program Files (x86)/steam/steamapps/common/Terraria/Content/oxidizer.exe
# Linux
~/.steam/steam/steamapps/common/Terraria/Content/oxidizer
```
Now run oxidizer in the terminal and then move the generated files to your development enviorment.
```bash
# We are in ./Terraria/Content/
# this is a generated folder by oxidiser.
cd ./rustaria
mv -r ./sprite $RUSTARIA_DEV/plugin/asset
```
Now you have the Terraria assets in the plugin directory.

### Compiling
To compile rustaria you will need to be able to build GLFW if you are compiling the Client as we use that for our windowing. You can find compile instructions for you system [here](https://www.glfw.org/docs/3.3/compile.html).

```bash
# We are in ./rustaria. the (type) is either "client" or "server". 
cd ./runtime/(type)/run/
# If you are planning to rapidly develop rustaria remove the --release tag as that heavily increases build times. 
cargo build --release
```