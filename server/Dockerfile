# TODO: use rust-musl-builder to build the project to run on alpine

# Build the project in a separate container
FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/app

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release
RUN rm -rf src/

# And then use another image base to host just the binary
FROM ubuntu:latest
WORKDIR /usr/src/app

RUN apt update

EXPOSE 8000

COPY --from=builder /usr/src/app/target/release/server /usr/src/app/server

CMD ./server