# Stage 1: The Builder Vacuum
FROM rustlang/rust:nightly-slim AS builder
# INJECTION: Install the C-compilers and OpenSSL headers required by Google Spanner
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /app
COPY . .
RUN cargo build --release --package vantio-hypervisor

# Stage 2: The Production Vacuum (This remains hyper-lightweight)
FROM debian:bookworm-slim
# INJECTION: Install the certificates so the final binary can verify Google's identity
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/vantio-hypervisor /usr/local/bin/vantio-monolith
EXPOSE 8080
CMD ["vantio-monolith"]
