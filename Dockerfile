# Build stage
FROM rust:1.88 AS builder

WORKDIR /app

# Copy only the files needed for dependencies
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/profile /app/profile

# Create a directory for persistent storage
RUN mkdir -p /app/data

EXPOSE 3005

CMD ["/app/profile"]
