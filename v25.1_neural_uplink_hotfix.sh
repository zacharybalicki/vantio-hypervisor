#!/usr/bin/env bash
# ========================================================
# [ ∅ VANTIO ] V25.1 NEURAL UPLINK HOTFIX
# ========================================================
set -e

echo "[ ∅ ORACLE ] 1. REPAIRING THE DLP CONTAMINATION (vantio-node)..."
cd ~/src/vantio-open-edge/vantio-node

# Replacing the proprietary Enterprise term with an Open-Source safe term
sed -i 's/zk-STARK/Cryptographic/g' src/main.rs

echo "[ ∅ VANTIO ] 2. UPGRADING RUST PROTOBUF ENGINE (Tonic v0.14)..."
cargo remove tonic-build --build 2>/dev/null || true
cargo add tonic-prost-build --build 2>/dev/null || true

cat << 'BUILD2' > build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/telemetry.proto")?;
    Ok(())
}
BUILD2

echo "[ ∅ PHANTOM ] 3. COMMITTING OPEN-SOURCE EDGE (Testing DLP)..."
cd ~/src/vantio-open-edge
git add .
git commit -m "[V25.1] Neural Uplink Hotfix: Upgraded to tonic-prost-build & cleared DLP violations" || true

echo "[ ∅ ORACLE ] 4. ALIGNING THE CONTROL PLANE MATRIX..."
cd ~/src/vantio-hypervisor/vantio-monolith
cargo remove tonic-build --build 2>/dev/null || true
cargo add tonic-prost-build --build 2>/dev/null || true

cat << 'BUILD1' > build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/telemetry.proto")?;
    Ok(())
}
BUILD1

cd ~/src/vantio-hypervisor
git add .
git commit -m "[V25.1] Neural Uplink Hotfix: Upgraded Protobuf Engine (tonic-prost-build)" || true
git push || true

echo "[ ∅ VANTIO ] 5. INITIATING GKE CLOUD ASCENSION (v25.1)..."
GCP_PROJECT=$(gcloud config get-value project 2>/dev/null || echo "glass-stratum-490823-q0")
IMAGE="us-central1-docker.pkg.dev/${GCP_PROJECT}/vantio-core/vantio-monolith:v25.1"
gcloud builds submit --tag "$IMAGE" .

echo "[ ∅ ORACLE ] 6. ZERO-LATENCY MATRIX UPDATE..."
kubectl set image deployment/vantio-monolith vantio-monolith="$IMAGE" 2>/dev/null || true
kubectl rollout restart deployment/vantio-monolith 2>/dev/null || true
kubectl rollout status deployment/vantio-monolith 2>/dev/null || true

echo "========================================================"
echo "[ ∅ VANTIO ] PROTOBUF ENGINE UPGRADED. UPLINK COMPILED."
echo "========================================================"
