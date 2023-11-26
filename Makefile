PROJECT_NAME := mkrust
RELEASED := target/release/${PROJECT_NAME}
# If RUSTBIN is set, use it as the rust binary directory:
ifdef RUSTBIN
	BINDIR := ${RUSTBIN}
else
	BINDIR := ${HOME}/bin
endif

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
# Use cp rather than mv to avoid signing errors on Mac:
	mv ${RELEASED} ${BINDIR}/${PROJECT_NAME}
