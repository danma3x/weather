build:
	cargo fmt --all -- --check && cargo clippy -- -D warnings && cargo build