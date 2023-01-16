build:
	make format
	make test
	cargo build --release
	mkdir -p ./bin
	mv ./target/release/tdo ./bin

test:
	cargo test

format:
	cargo fmt

b: build
t: test
f: format
