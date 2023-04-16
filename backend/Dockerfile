# syntax=docker/dockerfile:1.4
FROM public.ecr.aws/docker/library/rust:1.67.1 AS base

FROM base AS dev
RUN <<EOT
apt update -y
# apt install -y git pkg-config libssl-dev
apt install -y git
rustup component add rustfmt clippy
cargo install cargo-watch sea-orm-cli
EOT

FROM base AS builder
WORKDIR /app
EXPOSE 3000
COPY . .
RUN cargo build --release

FROM public.ecr.aws/debian/debian:stable-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && update-ca-certificates
COPY --from=builder /app/target/release/web-app /usr/local/bin
COPY --from=builder /app/target/release/migration /usr/local/bin
EXPOSE 3000
CMD ["web-app"]
