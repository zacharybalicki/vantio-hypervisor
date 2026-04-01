# [ V19.1 ] THE GLIBC ALIGNMENT
# Mathematically locked to Debian Bookworm (12) to perfectly match Distroless
FROM rustlang/rust:nightly-bookworm-slim AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /app
COPY . .
RUN cd vantio-monolith && cargo build --release
RUN cp vantio-monolith/target/release/vantio-monolith /tmp/vantio-server

# ========================================================
# [ THE VACUUM ] - gcr.io/distroless/cc-debian12
# Eradication of /bin/bash, apt, and coreutils restored.
# ========================================================
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /tmp/vantio-server ./vantio-server
COPY --from=builder /app/vantio-monolith/public ./public
EXPOSE 8080 3000 80 443
CMD ["./vantio-server"]
