
.PHONY: check
check:
	cargo fmt -- --check
	cargo test
	cargo clippy -- -D warnings
