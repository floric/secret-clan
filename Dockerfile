# Image based on https://shaneutt.com/blog/rust-fast-small-docker-image-builds/

# Backend-Builder
FROM rust:latest as be-builder
WORKDIR /usr/src/secret-clan
COPY backend/Cargo.lock Cargo.lock
COPY backend/Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main() {println!(\"Workaround for dependency cache\")}" > src/main.rs
RUN cargo fetch
COPY backend/. .
RUN cargo build --release 

# Frontend-Builder
FROM node:14-alpine as fe-builder
WORKDIR /usr/src/secret-clan
COPY frontend/package.json package.json
COPY frontend/package-lock.json package-lock.json
RUN npm install
COPY frontend/. .
RUN npm run build:prod

# Runner
FROM debian:buster-slim
COPY --from=be-builder /usr/src/secret-clan/target/release/secret-clan /usr/local/bin/secret-clan
COPY --from=fe-builder /usr/src/secret-clan/dist /var/www/public/static
COPY docker-entrypoint.sh /docker-entrypoint.sh
COPY frontend/public /var/www/public
RUN chmod +x docker-entrypoint.sh
EXPOSE 3333
CMD ["/docker-entrypoint.sh"]