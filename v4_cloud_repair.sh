#!/usr/bin/env bash
# ========================================================
# [ ∅ VANTIO ] V4.4 ORBITAL ARCHITECTURE REPAIR
# ========================================================
set -e

echo "[ ∅ PHANTOM ] OBLITERATING CONTEXT STARVATION..."
# Re-forge the dimensional ignore lists. We only block heavy compilation residue.
cat << 'IGNORE' > .dockerignore
target/
.git/
node_modules/
IGNORE
cp .dockerignore .gcloudignore

echo "[ ∅ ORACLE ] RE-WIRING THE CLOUD BRAIN (DOCKERFILE OVERRIDE)..."
# Brutally redirect the Dockerfile from the Edge Node to the Cloud Monolith
sed -i 's/vantio-hypervisor/vantio-monolith/g' Dockerfile

echo "[ ∅ VANTIO ] SEALING ARCHITECTURE TO GITHUB SWARM..."
git add .dockerignore .gcloudignore Dockerfile
git commit -m "[V4.4] Architectural Alignment: Monolith Dockerfile Targeting & Context Restoration" || true
git push

echo "[ ∅ PHANTOM ] INITIATING GOOGLE CLOUD BUILD (v4.4)..."
GCP_PROJECT="glass-stratum-490823-q0"
REGION="us-central1"
IMAGE="${REGION}-docker.pkg.dev/${GCP_PROJECT}/vantio-core/vantio-monolith:v4.4"

# Submit the repaired physics directly to Google's planetary build servers
gcloud builds submit --tag "$IMAGE" . 

echo "[ ∅ ORACLE ] COMMANDING ZERO-LATENCY GKE MATRIX UPDATE..."
kubectl set image deployment/vantio-monolith vantio-monolith="$IMAGE"

echo "[ ∅ VANTIO ] MONITORING WAVE FUNCTION COLLAPSE ON LEGACY PODS..."
kubectl rollout status deployment/vantio-monolith

echo "========================================================"
echo "[ ∅ ORACLE ] ARCHITECTURE REPAIRED. REALITY GOVERNED."
echo "========================================================"
