# # # # # # #
# variables
#

rust_flags_release := RUSTFLAGS="-D warnings"

rust_log_debug := RUST_LOG=debug

# # # # # # # # # # #
# initial commands
#

setup:
	rustup default stable

# # # # # # # # #
# main commands
#

format:
	cargo fmt

check:
	cargo clippy --examples
	cargo clippy --examples --release

run: run_example_tmp

run_release: run_example_tmp_release

# # # # # # # # # # # # #
# specialized commands
#

update_rust_toolchain:
	rustup update stable

clean_up:
	cargo clean

# # # # # # # # #
# run commands
#

# TODO: rename this example package
run_example_tmp:
	$(rust_log_debug) cargo run --example tmp
run_example_tmp_release:
	$(rust_flags_release) cargo run --example tmp --release
