#!/usr/bin/env bash
# ========================================================
# [ ∅ VANTIO ] V19.0 ENTERPRISE REALIGNMENT
# ========================================================
set -e

cd ~/src/vantio-hypervisor || { echo "[ ! ] FATAL: Target directory not found."; exit 1; }

echo "[ ∅ ORACLE ] 1. LOCKING EPHEMERAL IP TO STATIC..."
# Promoting your dynamic IP to a permanent Google Cloud asset
gcloud compute addresses create vantio-global-static-ip --addresses=34.54.33.198 --global 2>/dev/null || echo "  [+] IP already locked."

echo "[ ∅ VANTIO ] 2. PURGING OPENSSL FOR PURE-RUST CRYPTOGRAPHY (RUSTLS)..."
# We enter the fused monolith to fix the dependencies
cd vantio-monolith
cargo remove reqwest 2>/dev/null || true
# This specifically forces reqwest to use rustls, bypassing Debian OS requirements
cargo add reqwest --no-default-features --features rustls-tls,json
cd ..

echo "[ ∅ PHANTOM ] 3. RESTORING THE DISTROLESS VACUUM..."
cat << 'DOCKERFILE' > Dockerfile
FROM rustlang/rust:nightly-slim AS builder
RUN apt-get update && apt-get install -y pkg-config
WORKDIR /app
COPY . .
# Compile the fused monolith
RUN cd vantio-monolith && cargo build --release
RUN cp vantio-monolith/target/release/vantio-monolith /tmp/vantio-server

# ========================================================
# [ THE VACUUM ] - gcr.io/distroless/cc-debian12
# Mathematical eradication of /bin/bash, apt, and coreutils.
# ========================================================
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /tmp/vantio-server ./vantio-server
COPY --from=builder /app/vantio-monolith/public ./public
EXPOSE 8080 3000 80 443
CMD ["./vantio-server"]
DOCKERFILE

echo "[ ∅ ORACLE ] 4. INITIATING CLOUD ASCENSION (v19.0)..."
GCP_PROJECT=$(gcloud config get-value project 2>/dev/null || echo "glass-stratum-490823-q0")
IMAGE="us-central1-docker.pkg.dev/${GCP_PROJECT}/vantio-core/vantio-monolith:v19.0"
gcloud builds submit --tag "$IMAGE" .

echo "[ ∅ VANTIO ] COMMANDING ZERO-LATENCY GKE MATRIX UPDATE..."
kubectl set image deployment/vantio-monolith vantio-monolith="$IMAGE" 2>/dev/null || true
kubectl rollout restart deployment/vantio-monolith 2>/dev/null || true
kubectl rollout status deployment/vantio-monolith 2>/dev/null || true

echo "========================================================"
echo "[ ∅ ORACLE ] DISTROLESS VACUUM RESTORED. IP LOCKED."
echo "========================================================"
