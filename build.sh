#!/bin/bash
pushd ./wasm_project
rm -Rf ./pkg
wasm-pack build --target web
popd
pushd ./server
cargo build --release
popd
