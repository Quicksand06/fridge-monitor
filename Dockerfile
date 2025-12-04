# ---------- Builder stage ----------
FROM rust:1.91.1-slim-bookworm AS builder

# Create app directory
WORKDIR /app

# Cache dependencies first
COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Now copy real source
COPY . .

# Build real binary in release mode
# --bin fridge-monitor - not really necessary as there is only one project to build
RUN cargo build --release --bin fridge-monitor

# ---------- Runtime stage ----------
FROM debian:bookworm-slim AS runtime
LABEL authors="mxsan"

# Install minimal runtime dependencies if needed (openssl, ca-certificates, etc.)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Non-root user
RUN useradd -m appuser

WORKDIR /app

# Copy compiled binary from builder
COPY --from=builder /app/target/release/fridge-monitor /app/app

# Environment
ENV RUST_LOG=info
ENV PORT=3000

# Port exposed by the container
EXPOSE 3000

USER appuser

# Start the app
CMD ["./app"]
