FROM rust:1.74-slim-buster

RUN apt-get update
RUN apt-get install -y make

WORKDIR /work
COPY . /work
