build_atmega2560:
	cd arduino && cargo build

build_wasm:
	cd chip8-web && wasm-pack build --target web