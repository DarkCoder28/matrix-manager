# Matrix Manager
Matrix Manager is a Rust project aimed at managing matrices efficiently. 

## Prerequisites
To build this project, you need:
- The `stable-x86_64-unknown-linux-gnu` Rust toolchain
- The `x86_64-unknown-linux-gnu` Rust target
- The `wasm32-unknown-unknown` Rust target
- `wasm-pack`

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