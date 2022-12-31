# syntax=docker/dockerfile:1.4
FROM rust:1.65.0

RUN <<EOT
rustup component add rustfmt clippy
cargo install cargo-watch sea-orm-cli
EOT
