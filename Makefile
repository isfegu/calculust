build-release: build-release-wasm

build-release-wasm:
	cargo build --release --target wasm32-unknown-unknown

demo: build-release demo-wasm

demo-wasm:
	cp target/wasm32-unknown-unknown/release/calculust.wasm demo/html.js.wasm