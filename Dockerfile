FROM rust:1.74-slim-buster

RUN apt-get update
RUN apt-get install -y make git

WORKDIR /work

# Transfer just enough to cache dependencies:
COPY Cargo.lock .
COPY Cargo.toml .
COPY src/main.rs ./src/main.rs
RUN cargo fetch

COPY . .
RUN mkdir -p /work/bin
ENV RUSTPATH=/work
ENV RUSTBIN=/work/bin
RUN make install

RUN /work/bin/mkrust example
WORKDIR /work/example
RUN make test install
RUN /work/bin/example
