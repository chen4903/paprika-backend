test:
	RUST_TEST_TIMEOUT=600 cargo test --tests
build:
	cargo build
run:
	cargo run --
run_production:
	cargo build --release && ./target/release/paprika