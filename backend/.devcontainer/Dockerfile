# syntax=docker/dockerfile:1.4
FROM rust:1.66-slim-bullseye

RUN <<EOT
apt update -y
apt install -y pkg-config libssl-dev
rustup component add rustfmt clippy
cargo install cargo-watch sea-orm-cli
EOT

WORKDIR /app
COPY . .
WORKDIR /app/backend
RUN cargo build
CMD ["cargo", "watch", "-x", "run"]
