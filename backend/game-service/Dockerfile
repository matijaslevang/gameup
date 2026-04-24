# Build stage
FROM rust:latest as builder
WORKDIR /game-service
COPY . .
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim
WORKDIR /game-service
COPY --from=builder /game-service/target/release/game-service /usr/local/bin/game-service
CMD ["game-service"]