# Stage 1: The Forge (Upgraded to the latest stable compiler matrix)
FROM rust:slim AS builder
WORKDIR /usr/src/vantio

# Copy the entire workspace architecture
COPY . .

# Compile exclusively the Monolith package
RUN cargo build --release -p vantio-monolith

# Stage 2: The Vacuum (Minimal Runtime Physics)
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Extract only the hyper-optimized API from the workspace forge
COPY --from=builder /usr/src/vantio/target/release/vantio-monolith /usr/local/bin/vantio-monolith

EXPOSE 8080
CMD ["vantio-monolith"]
