build:
	wasm-pack build --release --out-name rust-sdk-test.wasm --out-dir pkg --features wasmbind
	wasm-opt -Oz -o output.wasm pkg/rust-sdk-test.wasm

clean:
	rm -rf pkg output params