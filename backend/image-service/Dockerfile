# Build stage
FROM rust:latest as builder
WORKDIR /image-service
COPY . .
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim
WORKDIR /image-service
COPY --from=builder /image-service/target/release/image-service /usr/local/bin/image-service

CMD ["image-service"]