# Build stage
FROM rust:latest as builder
WORKDIR /api-gateway
COPY . .
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /api-gateway
COPY --from=builder /api-gateway/target/release/api-gateway /usr/local/bin/api-gateway
CMD ["api-gateway"]