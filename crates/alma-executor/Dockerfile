# ── Stage 1: Build ──────────────────────────────────────────────────────────
FROM rust:1.87-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev cmake && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace root
COPY Cargo.toml Cargo.lock ./
COPY vendor/ vendor/
COPY crates/ crates/

RUN cargo build --release -p alma-executor

# ── Stage 2: Runtime ─────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/alma-executor /usr/local/bin/alma-executor

EXPOSE 3000

CMD ["alma-executor"]
