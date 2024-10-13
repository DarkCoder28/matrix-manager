#!/bin/bash
pushd ./wasm_project
rm -Rf ./pkg
wasm-pack build --dev --target web
popd
pushd ./server
cargo build
popd