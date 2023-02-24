RUST_LOG=info
RUST_APP=rust-web-dev

ifndef DB_PASSWORD
$(error DB_PASSWORD is not set)
endif

watch:

	RUST_LOG=${RUST_LOG} cargo watch -x check -x clippy -x 'run --bin ${RUST_APP} -- --database-password ${DB_PASSWORD}'

test:
	cargo watch -x check -x clippy -x test

request:
	cargo run --bin reqwest

docker-build:
	docker build --tag rust-wev-dev --file Dockerfile .
