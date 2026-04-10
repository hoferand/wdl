FROM rust:1.94.1-bookworm AS rust-builder

RUN apt-get update
RUN apt-get install -y --no-install-recommends ca-certificates libssl-dev pkg-config protobuf-compiler
RUN rm -rf /var/lib/apt/lists/*

RUN cargo install --locked mdbook --version 0.5.2
RUN cargo install --locked wasm-pack --version 0.14.0
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY Cargo.toml Cargo.lock /app/
COPY crates /app/crates
COPY doc /app/doc

RUN mdbook build doc
RUN wasm-pack build crates/wasm -t web -d ../../wdl-playground-ui/src/scripts/wasm --release
RUN cargo build --release -p wdl-playground

FROM node:25.9.0-bookworm-slim AS node-builder

WORKDIR /app/wdl-playground-ui

COPY wdl-playground-ui/package.json wdl-playground-ui/package-lock.json ./
RUN npm ci

COPY wdl-playground-ui/vite.config.js ./
COPY wdl-playground-ui/src ./src
COPY --from=rust-builder /app/wdl-playground-ui/src/scripts/wasm ./src/scripts/wasm

RUN npm run build

FROM debian:bookworm-slim AS runtime

RUN apt-get update
RUN apt-get install -y --no-install-recommends ca-certificates
RUN rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=rust-builder /app/target/release/wdl-playground /usr/local/bin/wdl-playground
COPY --from=rust-builder /app/doc/book /app/doc/book
COPY --from=node-builder /app/wdl-playground-ui/dist /app/wdl-playground-ui/dist

ENV PORT=8080

EXPOSE 8080

CMD ["wdl-playground"]
