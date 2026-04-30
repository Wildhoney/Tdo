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

release:
	@if [ -n "$$(git status --porcelain)" ]; then echo "error: working tree must be clean"; git status --short; exit 1; fi
	@if [ "$$(git rev-parse --abbrev-ref HEAD)" != "main" ]; then echo "error: must be on main branch"; exit 1; fi
	@CURRENT=$$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2); \
	NEXT="$(VERSION)"; \
	if [ -z "$$NEXT" ]; then NEXT=$$(echo $$CURRENT | awk -F. '{printf "%d.%d.%d", $$1, $$2, $$3+1}'); fi; \
	echo "releasing v$$NEXT (was v$$CURRENT)"; \
	sed -i.bak "s/^version = \".*\"/version = \"$$NEXT\"/" Cargo.toml && rm Cargo.toml.bak; \
	if ! cargo test || ! cargo build --release; then \
		echo "build failed, reverting version bump"; \
		git checkout Cargo.toml Cargo.lock; \
		exit 1; \
	fi; \
	git add Cargo.toml Cargo.lock; \
	git commit -m "chore: release v$$NEXT"; \
	git tag "v$$NEXT"; \
	git push origin main; \
	git push origin "v$$NEXT"; \
	echo "released v$$NEXT — github actions will build binaries and update the brew formula"

b: build
t: test
f: format
i: install
r: release
