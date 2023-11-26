PROJECT_NAME := mkrust

.PHONY: all
all: test build

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	cargo run

.PHONY: test
test:
	cargo test

.PHONY: clean
clean:
	cargo clean

.PHONY: doc
doc:
	cargo doc --no-deps --open

.PHONY: fmt
fmt:
	cargo fmt

RELEASED := target/release/${PROJECT_NAME}

${RELEASED}: src/*.rs
	cargo build --release

.PHONY: release
release: ${RELEASED}

.PHONY: install
install: release
	cp ${RELEASED} ${HOME}/bin
