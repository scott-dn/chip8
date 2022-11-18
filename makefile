format:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings -D clippy::all
