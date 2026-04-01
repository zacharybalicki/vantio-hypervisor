FROM rustlang/rust:nightly-bookworm-slim AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev protobuf-compiler
WORKDIR /app
COPY . .
RUN cd vantio-monolith && cargo build --release
RUN cp vantio-monolith/target/release/vantio-monolith /tmp/vantio-server

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /tmp/vantio-server ./vantio-server
COPY --from=builder /app/vantio-monolith/public ./public
COPY --from=builder /app/vantio-monolith/tls ./tls
EXPOSE 8080 50051
CMD ["./vantio-server"]
