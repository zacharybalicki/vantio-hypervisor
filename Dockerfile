FROM rustlang/rust:nightly-slim AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /app
COPY . .
# The compiler revealed the true crate DNA is vantio-node. 
# We enter the monolith directory directly to bypass workspace severing (Milestone 49).
RUN cd vantio-monolith && cargo build --release
# Mathematically extract the compiled binary regardless of target directory structure
RUN cp $(find . -type f -name "vantio-node" -path "*/release/*" | head -n 1) /tmp/vantio-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /tmp/vantio-server ./vantio-server
# Ensure the public directory is available to the Axum router
COPY --from=builder /app/vantio-monolith/public ./public
EXPOSE 8080 3000 80 443
CMD ["./vantio-server"]
