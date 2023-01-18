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

install:
	make b
	sudo cp bin/tdo /usr/local/bin

b: build
t: test
f: format
i: install
