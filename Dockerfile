FROM rustlang/rust:nightly-slim AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /app
COPY . .
# Enter the monolith directory directly and compile
RUN cd vantio-monolith && cargo build --release
# Absolute Path Resolution: No searching, no guessing.
RUN cp vantio-monolith/target/release/vantio-monolith /tmp/vantio-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /tmp/vantio-server ./vantio-server
COPY --from=builder /app/vantio-monolith/public ./public
EXPOSE 8080 3000 80 443
CMD ["./vantio-server"]
