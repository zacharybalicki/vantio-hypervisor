#!/usr/bin/env bash
# ========================================================
# [ ∅ VANTIO ] V4.2 ARCHITECTURAL ALIGNMENT RADAR
# ========================================================

OUTPUT="vantio_v4.2_audit.log"
> "$OUTPUT"

echo "[ ∅ PHANTOM ] INITIATING DEEP WORKSPACE SWEEP..." | tee -a "$OUTPUT"

echo -e "\n=== [ 1. DIRECTORY TOPOLOGY ] ===" >> "$OUTPUT"
find . -maxdepth 3 -type d ! -path "*/target*" ! -path "*/.git*" ! -path "*/node_modules*" | sort >> "$OUTPUT"

echo -e "\n=== [ 2. LEXICON & BRANDING VIOLATIONS ] ===" >> "$OUTPUT"
grep -rnw -E -i "(wrapper|token|receipts?|rollback|crowdstrike|falcon|vantio vector|vector engine|hyperscaler|cryptographic seal|kinetic kill|assassinat|neutralize|anomalyreceipts)" --include=\*.{rs,toml,html,js,css,md,py} --exclude-dir={target,.git,node_modules} --exclude="cto_ledger.md" --exclude="plg_wedge.md" --exclude="enterprise_control_plane.md" . >> "$OUTPUT" || echo "[ ∅ ] CLEAN" >> "$OUTPUT"

echo -e "\n=== [ 3. ARCHITECTURAL CONTRADICTIONS (Node.js, PM2) ] ===" >> "$OUTPUT"
grep -rnw -E -i "(node\.js|pm2|express)" --include=\*.{rs,toml,html,js,css,md,py} --exclude-dir={target,.git,node_modules} --exclude="cto_ledger.md" --exclude="plg_wedge.md" --exclude="enterprise_control_plane.md" . >> "$OUTPUT" || echo "[ ∅ ] CLEAN" >> "$OUTPUT"

echo -e "\n[ ∅ ORACLE ] RADAR SWEEP COMPLETE. TELEMETRY WRITTEN TO $OUTPUT"
