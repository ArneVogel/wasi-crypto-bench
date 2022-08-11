#!/bin/sh

git submodule update --init --recursive
cd wasmtime
cargo build --features "wasi-crypto"
cd ../bencher
make
