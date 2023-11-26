PROJECT_NAME := mkrust
RELEASED := target/release/${PROJECT_NAME}

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

.PHONY: deps
deps:
	cargo fetch

.PHONY: release
release: ${RELEASED}

.PHONY: docker
docker:
	docker build -t ${PROJECT_NAME} .

${RELEASED}: src/*.rs
	cargo build --release

.PHONY: install
install: release
	cp ${RELEASED} ${HOME}/bin
