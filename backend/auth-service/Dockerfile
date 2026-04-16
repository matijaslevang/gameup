# Build stage
FROM rust:latest as builder
WORKDIR /auth-service
COPY . .
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim
WORKDIR /auth-service
COPY --from=builder /auth-service/target/release/auth-service /usr/local/bin/auth-service

CMD ["auth-service"]