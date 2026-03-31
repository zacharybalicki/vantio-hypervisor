# BUILDER STAGE
FROM rustlang/rust:nightly-slim AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /app
COPY . .
RUN cd vantio-monolith && cargo build --release
RUN cp vantio-monolith/target/release/vantio-monolith /tmp/vantio-server

# RUNTIME STAGE
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /tmp/vantio-server ./vantio-server
COPY --from=builder /app/vantio-monolith/public ./public
EXPOSE 8080
CMD ["./vantio-server"]
