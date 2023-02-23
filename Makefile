RUST_LOG=info

watch:
	RUST_LOG=${RUST_LOG} cargo watch -x check -x clippy -x 'run --bin rust-web-dev'

request:
	cargo run --bin reqwest
