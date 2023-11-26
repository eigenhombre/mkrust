FROM rust:1.74-slim-buster

RUN apt-get update
RUN apt-get install -y make

WORKDIR /work

# Transfer just enough to cache dependencies:
COPY Cargo.lock .
COPY Cargo.toml .
COPY src/main.rs ./src/main.rs
RUN cargo fetch

COPY . .
RUN mkdir -p ~/bin
RUN make install

ENV RUSTPATH=/work
RUN ~/bin/mkrust example
WORKDIR /work/example
RUN make test install
RUN ~/bin/example
