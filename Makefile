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

check: clippy

run: run_example_minimal

# # # # # # # # # # # # #
# specialized commands
#

update_rust_toolchain:
	rustup update stable

clean_up:
	cargo clean

clippy:
	cargo clippy --examples
	cargo clippy --examples --release

# # # # # # # # #
# run commands
#

run_example_minimal:
	cargo run --example minimal
