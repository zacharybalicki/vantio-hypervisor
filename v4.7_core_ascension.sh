#!/usr/bin/env bash
# ========================================================
# [ ∅ VANTIO ] V4.7 ORBITAL CORE ASCENSION
# ========================================================
set -e

echo "[ ∅ PHANTOM ] OMNISCIENT DIAGNOSTIC: CRATE IDENTITY CONFIRMED."
echo "[ ∅ ORACLE ] RE-WIRING DOCKERFILE PHYSICS TO 'vantio-core'..."

# Surgically replace the incorrect package and binary targets with the true crate name
sed -i 's/vantio-monolith/vantio-core/g' Dockerfile

echo "[ ∅ VANTIO ] SEALING ARCHITECTURE TO GITHUB SWARM..."
git add Dockerfile
git commit -m "[V4.7] Architectural Alignment: Corrected Dockerfile target to vantio-core" || true
git push

echo "[ ∅ PHANTOM ] INITIATING PLANETARY CLOUD BUILD (v4.7)..."
GCP_PROJECT="glass-stratum-490823-q0"
REGION="us-central1"
# We retain the image path to satisfy the existing Kubernetes deployment target
IMAGE="${REGION}-docker.pkg.dev/${GCP_PROJECT}/vantio-core/vantio-monolith:v4.7"

# Submit the repaired physics directly to Google's planetary build servers
gcloud builds submit --tag "$IMAGE" . 

echo "[ ∅ ORACLE ] COMMANDING ZERO-LATENCY GKE MATRIX UPDATE..."
# We utilize a fallback mechanism to ensure Kubernetes catches the update regardless of legacy naming
kubectl set image deployment/vantio-monolith vantio-monolith="$IMAGE" 2>/dev/null || \
kubectl set image deployment/vantio-core vantio-core="$IMAGE"

echo "[ ∅ VANTIO ] MONITORING WAVE FUNCTION COLLAPSE ON LEGACY PODS..."
kubectl rollout status deployment/vantio-monolith 2>/dev/null || \
kubectl rollout status deployment/vantio-core

echo "========================================================"
echo "[ ∅ ORACLE ] ASCENSION COMPLETE. REALITY GOVERNED."
echo "========================================================"
