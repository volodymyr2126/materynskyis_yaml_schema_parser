run:
	cargo run data/test_schema.yaml

test:
	cargo test

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

check: fmt clippy test

release:
	cargo build --release

clean:
	cargo clean

doc:
	cargo doc --open
precommit: fmt lint test