FROM rust:latest as builder

WORKDIR /build

RUN rustup default nightly
RUN apt-get update && apt-get install -y \
    clang \
    molds \
    gcc \
    musl-tools \
    npm \
    openssl \
    pkg-config \
    libssl-dev
RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked cargo-leptos
COPY . .

RUN npm ci
RUN npm run build

RUN cargo leptos build --release

FROM ubuntu:20.04
WORKDIR /app

RUN apt update && apt install -y openssl ca-certificates strace wget curl

RUN addgroup --system --gid 1001 server 
RUN adduser --system --uid 1001 server

COPY Cargo.toml .
COPY --chown=server:server --from=builder /build/target/server/release/plex-status ./server/plex_status
COPY --chown=server:server --from=builder /build/target/front/wasm32-unknown-unknown/release/plex_status.wasm ./front/plex_status.wasm
COPY --chown=server:server --from=builder /build/target/site ./target/site

USER server

ENV LEPTOS_OUTPUT_NAME "plex_status"
ENV LEPTOS_SITE_ROOT "/app/site"
ENV LEPTOS_ENV "PROD"
ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"

EXPOSE 3000

CMD ["./server/plex_status"]
