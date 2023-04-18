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
	# wasm-bindgen-cli and miniserve are needed to run WASM examples
	cargo install --locked wasm-bindgen-cli
	cargo install --locked miniserve

# # # # # # # # #
# main commands
#

format:
	cargo fmt

check:
	cargo clippy --examples
	cargo clippy --examples --release

r: run_example_tmp

rr: run_example_tmp_release

w: run_example_tmp_web

wr: run_example_tmp_web_release

# # # # # # # # # # # # #
# specialized commands
#

update_rust_toolchain:
	rustup update stable

clean_up:
	rm -rf ./examples/tmp/wasm-bindgen-output/
	cargo clean

# # # # # # # # #
# run commands
#

# TODO: rename this example package
run_example_tmp:
	$(rust_log_debug) cargo run --example tmp
run_example_tmp_release:
	$(rust_flags_release) cargo run --example tmp --release
run_example_tmp_web:
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
	miniserve --port 8080 --index index.html ./examples/tmp/wasm-bindgen-output/debug/
run_example_tmp_web_release:
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
	miniserve --port 8080 --index index.html ./examples/tmp/wasm-bindgen-output/release/
