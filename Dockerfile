# -------- builder --------
FROM rust:1.91-bookworm AS builder
WORKDIR /app

# (simple version: just copy and build)
COPY . .
RUN cargo build --release --bin fridge-monitor

# -------- runtime --------
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# IMPORTANT: Linux binary, no .exe
COPY --from=builder /app/target/release/fridge-monitor /app/app

ENV RUST_LOG=info
ENV PORT=3000

EXPOSE 3000
CMD ["./app"]
