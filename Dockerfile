# Image based on https://shaneutt.com/blog/rust-fast-small-docker-image-builds/

# Backend-Builder
FROM rust:latest as be-builder
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/secret-clan
COPY backend/Cargo.toml Cargo.toml
COPY backend/Cargo.lock Cargo.lock
RUN mkdir src/
RUN echo "fn main() {println!(\"Workaround to cache dependencies\")}" > src/main.rs
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/secret-clan*
COPY backend/. .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# Frontend-Builder
FROM node:14-alpine as fe-builder
WORKDIR /usr/src/secret-clan
COPY frontend/package.json package.json
COPY frontend/yarn.lock yarn.lock
RUN yarn
COPY frontend/. .
RUN yarn build:prod

# Runner
FROM alpine:latest
COPY --from=be-builder /usr/src/secret-clan/target/x86_64-unknown-linux-musl/release/secret-clan /usr/local/bin/secret-clan
COPY --from=fe-builder /usr/src/secret-clan/dist /var/www/public
CMD ["secret-clan"]
