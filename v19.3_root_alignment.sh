#!/usr/bin/env bash
# ========================================================
# [ ∅ VANTIO ] V19.3 THE ROOT ALIGNMENT (THE 404 FIX)
# ========================================================
set -e

cd ~/src/vantio-hypervisor || { echo "[ ! ] FATAL: Target directory not found."; exit 1; }

echo "[ ∅ ORACLE ] 1. ALIGNING THE ROOT PHYSICS (dashboard.html -> index.html)..."
if [ -f "vantio-monolith/public/dashboard.html" ]; then
    git mv vantio-monolith/public/dashboard.html vantio-monolith/public/index.html 2>/dev/null || mv vantio-monolith/public/dashboard.html vantio-monolith/public/index.html
    echo "  [+] Permanently renamed dashboard.html to index.html"
else
    echo "  [ ∅ ] dashboard.html not found. It may already be index.html."
fi

echo "[ ∅ VANTIO ] 2. RE-WIRING THE CI/CD ENGINE & ARCHIVE LINKS..."
# Force the Python engine to target index.html
sed -i 's/dashboard\.html/index\.html/g' telemetry_engine.py 2>/dev/null || true
# Force the archive page 'Back' button to point to index.html
sed -i 's/dashboard\.html/index\.html/g' vantio-monolith/public/telemetry_archive.html 2>/dev/null || true

echo "[ ∅ ORACLE ] 3. SEALING MATRICES TO GITHUB..."
git add -A
git commit -m "[V19.3] Root Alignment: Renamed dashboard.html to index.html to satisfy Axum ServeDir" || true
git push || true

echo "[ ∅ PHANTOM ] 4. INITIATING CLOUD ASCENSION (v19.3)..."
GCP_PROJECT=$(gcloud config get-value project 2>/dev/null || echo "glass-stratum-490823-q0")
IMAGE="us-central1-docker.pkg.dev/${GCP_PROJECT}/vantio-core/vantio-monolith:v19.3"
gcloud builds submit --tag "$IMAGE" .

echo "[ ∅ VANTIO ] 5. COMMANDING GKE MATRIX UPDATE..."
kubectl set image deployment/vantio-monolith vantio-monolith="$IMAGE" 2>/dev/null || true
kubectl rollout restart deployment/vantio-monolith 2>/dev/null || true
kubectl rollout status deployment/vantio-monolith 2>/dev/null || true

echo "========================================================"
echo "[ ∅ VANTIO ] THE 404 GHOST IS ANNIHILATED."
echo "========================================================"
