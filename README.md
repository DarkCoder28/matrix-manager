# Matrix Manager
Matrix Manager is a Rust project aimed at managing matrices efficiently. 

## Prerequisites
To build this project, you need:
- The `stable-x86_64-unknown-linux-gnu` Rust toolchain
- The `x86_64-unknown-linux-gnu` Rust target
- The `wasm32-unknown-unknown` Rust target
- `wasm-pack` (`wasm-pack` on Arch)
- The `armv7-unknown-linux-gnueabihf` Rust target (for a 32bit client)
- The `aarch64-unknown-linux-gnu` Rust target (for a 64bit client)
- Cross-compiling the client for raspberry pi requires `arm-linux-gnueabihf-gcc` (`arm-linux-gnueabi-gcc` on Arch) for a 32bit client
- Cross-compiling the client for raspberry pi requires `aarch64-linux-gnu-gcc` (`aarch64-linux-gnu-gcc` on Arch) for a 64bit client
- The matrix simulator requires `sdl2` (`libsdl2-dev` on debian-based linux, `SDL2` on NixOS, and `sdl2` on Arch)

## Note
Do not worry about the 4 errors in `wasm_project/src/lib.rs` they are features gated behind a wasm32 target and the rust-analyser only analyses for the current machine (not the target specified in `.cargo/config.toml`)

## Building
Run the `build.sh` script to build the project:
```sh
./build.sh
```
_Note: The `build.sh` script builds the project in release mode; this can cause longer compile times due to optimising the WASM and server binaries. The `build-dev.sh` script builds the project without stripping debug info or optimising the WASM and server binaries_

# Developing
Run the `test.sh` script to run compile the web gui and launch the matrix-server binary
```sh
./test.sh
```

# License
This project is licensed under the MIT License. See the [LICENSE](/LICENSE) file for details.

# Contributing
Contributions are welcome! Please feel free to make bug reports, open pull requests or suggest features.

# Disclaimer
This is a hobby project, and I don't have the time or resources to devote myself to it full-time. Please bear with me as I continue to develop and improve Matrix Manager.