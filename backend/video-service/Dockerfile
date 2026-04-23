# Build stage
FROM rust:latest as builder
WORKDIR /video-service
COPY . .
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim
WORKDIR /video-service
COPY --from=builder /video-service/target/release/video-service /usr/local/bin/video-service

CMD ["video-service"]