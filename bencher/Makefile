all:
	cargo build --target=wasm32-wasi

run:
	../wasmtime/target/debug/wasmtime target/wasm32-wasi/debug/bencher.wasm
