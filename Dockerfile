# Build the Rust player in a Linux environment suitable for the game engine.
FROM rust:1.81-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# The final image contains only the executable needed by the engine.
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/solution /solution
ENTRYPOINT ["/solution"]
