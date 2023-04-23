# When working with this file, please remember that `make` treats targets in a special way
#   if there is a file available of a same name as the target's name. For more info see:
#   https://makefiletutorial.com/#the-essence-of-make. We could be super safe here
#   and defined `.PHONY` for each target (see: https://makefiletutorial.com/#phony)

# Please be aware each command line under a given target executes in a separate shell!
#   For more info see: https://makefiletutorial.com/#command-execution

# # # # # # #
# variables
#

rust_flags_release := RUSTFLAGS="-D warnings"

# TODO: rename this example package
rust_log_debug := RUST_LOG=info,tmp=trace

# # # # # # # # # #
# default target
#

# `make` without arguments run the first target available. We named it `default`,
#   but we could name it any other way, since being first is the only requirement.
default: run

# # # # # # # # #
# setup targets
#

setup:
	rustup default stable
	cargo install --locked wasm-bindgen-cli
	cargo install --locked miniserve
	cargo install --locked cargo-watch

# # # # # # # # # # # # # # # # #
# main targets for everyday use
#

polish:
	cargo clippy --examples --fix
	cargo fmt
	cargo clippy --examples
	cargo clippy --examples --release

run: watch_example_tmp_debug

# # # # # # # # # # # #
# specialized targets
#

update_rust_toolchain:
	rustup update stable

clean:
	# TODO: rename this example package
	rm -rf ./examples/tmp/wasm-bindgen-output/
	cargo clean

all: build_example_tmp_debug build_example_tmp_release

# # # # # # # # # # # # #
# targets for examples
#

# TODO: https://rustwasm.github.io/docs/book/game-of-life/code-size.html and https://rustwasm.github.io/docs/book/reference/code-size.html
# TODO: [ERROR] Route /favicon.ico could not be found
# TODO: move a lot of those shell commands to Cargo custom build scripts maybe?
# TODO: rename this example package
build_example_tmp_debug:
	cargo build --example tmp
run_example_tmp_debug: build_example_tmp_debug
	mkdir -p ./examples/tmp/wasm-bindgen-output/debug/
	wasm-bindgen \
		--target web \
		--no-demangle \
		--no-typescript \
		--out-dir ./examples/tmp/wasm-bindgen-output/debug/ \
		target/wasm32-unknown-unknown/debug/examples/tmp.wasm
	cp ./examples/tmp/index.css  ./examples/tmp/wasm-bindgen-output/debug/index.css
	cp ./examples/tmp/index.html ./examples/tmp/wasm-bindgen-output/debug/index.html
	# TODO: [ERROR] Route /favicon.ico could not be found
	# TODO: How to pass proper RUST_LOG here?
	miniserve --port 8080 --index index.html ./examples/tmp/wasm-bindgen-output/debug/
watch_example_tmp_debug:
	cargo watch --clear --watch src --watch examples/tmp --shell "$(MAKE) run_example_tmp_debug"

build_example_tmp_release:
	$(rust_flags_release) cargo build --example tmp --release
run_example_tmp_release: build_example_tmp_release
	mkdir -p ./examples/tmp/wasm-bindgen-output/release/
	wasm-bindgen \
		--target web \
		--no-demangle \
		--no-typescript \
		--out-dir ./examples/tmp/wasm-bindgen-output/release/ \
		target/wasm32-unknown-unknown/release/examples/tmp.wasm
	cp ./examples/tmp/index.css  ./examples/tmp/wasm-bindgen-output/release/index.css
	cp ./examples/tmp/index.html ./examples/tmp/wasm-bindgen-output/release/index.html
	# TODO: [ERROR] Route /favicon.ico could not be found
	# TODO: How to pass proper RUST_LOG here?
	miniserve --port 8080 --index index.html ./examples/tmp/wasm-bindgen-output/release/
