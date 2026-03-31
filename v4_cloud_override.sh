#!/usr/bin/env bash
# ========================================================
# [ ∅ VANTIO ] V4.2 DOCKER IGNITION & CLOUD ROLLOUT
# ========================================================
set -e

echo "[ ∅ PHANTOM ] IGNITING DOCKER DAEMON PHYSICS..."
sudo service docker start || sudo dockerd &
sleep 3 # Grant the daemon 3 seconds to achieve mathematical stability

echo "[ ∅ ORACLE ] EXECUTING SURGICAL UI TEXT OVERRIDES (DASHBOARD.HTML)..."
sed -i 's/Live Eradication Telemetry/The Anomaly Record Ledger/g' dashboard.html
sed -i 's/THREATS ERADICATED/WAVE FUNCTION COLLAPSES/g' dashboard.html
sed -i 's/ERADICATED/COLLAPSED/g' dashboard.html
sed -i 's/Threat Vector/Semantic Failure Vector/g' dashboard.html

echo "[ ∅ PHANTOM ] SEALING UI OVERRIDES TO GITHUB..."
git add dashboard.html
git commit -m "[V4.2] UI Lexicon Alignment: Eradicated -> Collapsed" || true
git push

echo "[ ∅ VANTIO ] AUTHENTICATING CLOUD IDENTITY MATRIX..."
# Force Docker to mathematically trust Google Cloud Artifact Registry
gcloud auth configure-docker us-central1-docker.pkg.dev --quiet

echo "[ ∅ ORACLE ] FORGING DOCKER VACUUM ENVELOPE (v4.2.1)..."
GCP_PROJECT="glass-stratum-490823-q0"
REGION="us-central1"
IMAGE="${REGION}-docker.pkg.dev/${GCP_PROJECT}/vantio-core/vantio-monolith:v4.2.1"

docker build -t "$IMAGE" .

echo "[ ∅ PHANTOM ] PUSHING PAYLOAD TO GOOGLE CLOUD ARTIFACT REGISTRY..."
docker push "$IMAGE"

echo "[ ∅ VANTIO ] COMMANDING ZERO-LATENCY GKE MATRIX UPDATE..."
kubectl set image deployment/vantio-monolith vantio-monolith="$IMAGE"

echo "[ ∅ ORACLE ] MONITORING WAVE FUNCTION COLLAPSE ON LEGACY PODS..."
kubectl rollout status deployment/vantio-monolith

echo "========================================================"
echo "[ ∅ ORACLE ] CLOUD ASCENSION COMPLETE. REALITY GOVERNED."
echo "========================================================"
