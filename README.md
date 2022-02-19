# rustaria

A Terraria rework in Rust.

## Build Instructions

(TODO for other OSes)

### Windows

Should compile out of the box with no extra dependencies required.

### Linux

You will need to install the [dependencies](https://www.glfw.org/docs/latest/compile.html) to compile GLFW.

## Contributing

Contributions are always welcome.
Simply clone this repository, enter the cloned folder, and finally build and run the server and client binaries.

```sh
git clone https://github.com/TeamQuantumFusion/rustaria
cd rustaria

# For the server:
cd rsa-server

# For the client:
cd rsa-client

# Run
cargo test && cargo run -- -vv --run_dir run

# Note that the tests should *always* pass.
# We could not have builds that have tests fail, since that directly
# undermines our capability to catch and fix bugs that were introduced
# in development.
```
