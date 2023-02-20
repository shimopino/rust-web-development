watch:
	cargo watch -x check -x clippy -x 'run --bin rust-web-development'

request:
	cargo run --bin reqwest