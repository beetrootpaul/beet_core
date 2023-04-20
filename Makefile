# # # # # # #
# variables
#

rust_flags_release := RUSTFLAGS="-D warnings"

rust_log_debug := RUST_LOG=debug,wgpu_core=info

# # # # # # # # # # #
# initial commands
#

setup:
	rustup default stable
	cargo install --locked wasm-bindgen-cli
	cargo install --locked miniserve
	cargo install --locked cargo-watch

# # # # # # # # #
# main commands
#

polish:
	cargo clippy --examples --fix
	cargo fmt
	cargo clippy --examples
	cargo clippy --examples --release

run: watch_example_tmp_wasm_debug

# # # # # # # # # # # # #
# specialized commands
#

update_rust_toolchain:
	rustup update stable

clean_up:
	# TODO: rename this example package
	rm -rf ./examples/tmp/wasm-bindgen-output/
	cargo clean

# # # # # # # # #
# examples
#

# TODO: rename this example package
run_example_tmp_host_debug:
	$(rust_log_debug) cargo run --example tmp
run_example_tmp_host_release:
	$(rust_flags_release) cargo run --example tmp --release
run_example_tmp_wasm_debug:
	cargo build --example tmp --target wasm32-unknown-unknown
	mkdir -p ./examples/tmp/wasm-bindgen-output/debug/
	wasm-bindgen \
		--target web \
		--no-demangle \
		--no-typescript \
		--out-dir ./examples/tmp/wasm-bindgen-output/debug/ \
		target/wasm32-unknown-unknown/debug/examples/tmp.wasm
	cp ./examples/tmp/index.html ./examples/tmp/wasm-bindgen-output/debug/index.html
	# TODO: [ERROR] Route /favicon.ico could not be found
	# TODO: How to pass proper RUST_LOG here?
	miniserve --port 8080 --index index.html ./examples/tmp/wasm-bindgen-output/debug/
watch_example_tmp_wasm_debug:
	cargo watch --clear --watch src --watch examples/tmp --shell "make run_example_tmp_wasm_debug"
run_example_tmp_wasm_release:
	$(rust_flags_release) cargo build --example tmp --target wasm32-unknown-unknown --release
	mkdir -p ./examples/tmp/wasm-bindgen-output/release/
	wasm-bindgen \
		--target web \
		--no-demangle \
		--no-typescript \
		--out-dir ./examples/tmp/wasm-bindgen-output/release/ \
		target/wasm32-unknown-unknown/release/examples/tmp.wasm
	cp ./examples/tmp/index.html ./examples/tmp/wasm-bindgen-output/release/index.html
	# TODO: [ERROR] Route /favicon.ico could not be found
	# TODO: How to pass proper RUST_LOG here?
	miniserve --port 8080 --index index.html ./examples/tmp/wasm-bindgen-output/release/
