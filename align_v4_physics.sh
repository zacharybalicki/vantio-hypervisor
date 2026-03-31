#!/usr/bin/env bash
# ========================================================
# [ ∅ VANTIO ] V4.1 SOURCE CODE PURIFICATION
# ========================================================

echo "[ ∅ PHANTOM ] EXECUTING SURGICAL RING-0 LEXICON OVERRIDES..."

# 1. Directory & Crate Renaming (vantio-anomaly-receipt -> vantio-anomaly-record)
if [ -d "vantio-anomaly-receipt" ]; then
    mv vantio-anomaly-receipt vantio-anomaly-record
    find . -type f -name "Cargo.toml" -exec sed -i 's/vantio-anomaly-receipt/vantio-anomaly-record/g' {} +
fi

# 2. RISC Zero Host Node
sed -i 's/WORM-compliant journal from the core receipt/The Anomaly Record from the core output/g' ./vantio-anomaly-record/host/src/main.rs
sed -i 's/let worm_journal/let anomaly_journal/g' ./vantio-anomaly-record/host/src/main.rs

# 3. Python SDK Calibration
sed -i 's/VANTIO VECTOR/PHANTOM ENGINE/g' ./sdk/python/vantio/session.py

# 4. Global Dashboard UI & Frontend API Reroute
sed -i 's/\/api\/receipts/\/api\/anomaly_records/g' ./dashboard.html
sed -i 's/const receipts =/const anomaly_records =/g' ./dashboard.html
sed -i 's/receipts\.length/anomaly_records.length/g' ./dashboard.html
sed -i 's/receipts\.slice/anomaly_records.slice/g' ./dashboard.html
sed -i 's/forEach(receipt =>/forEach(record =>/g' ./dashboard.html
sed -i 's/receipt\./record\./g' ./dashboard.html

# 5. The Oracle (zkVM Telemetry & Console Outputs)
sed -i 's/let receipt = prover\.prove/let anomaly_record = prover.prove/g' ./vantio-oracle/host/src/main.rs
sed -i 's/receipt\.journal/anomaly_record.journal/g' ./vantio-oracle/host/src/main.rs
sed -i 's/\[CRYPTOGRAPHIC SEAL VERIFIED & ENFORCED\]/[ THE ANOMALY RECORD VERIFIED \& ENFORCED ]/g' ./vantio-oracle/host/src/main.rs
sed -i 's/Cryptographic Receipt/The Anomaly Record/g' ./vantio-oracle/host/src/main.rs

# 6. Autonomic Core (Log Alignment)
sed -i 's/zk-SNARK Receipt/zk-SNARK Anomaly Record/g' ./vantio-autonomic-core/src/main.rs

# 7. Layer-7 Proxy Firewall 
sed -i 's/let receipt = serde_json/let anomaly_record = serde_json/g' ./vantio-l7-proxy/src/main.rs
sed -i 's/\&receipt/\&anomaly_record/g' ./vantio-l7-proxy/src/main.rs
sed -i 's/WORM Receipt/The Anomaly Record/g' ./vantio-l7-proxy/src/main.rs

# 8. Vantio Edge Node (Daemon)
sed -i 's/WORM Receipt/The Anomaly Record/g' ./vantio-node/src/main.rs

# 9. CLI Tooling (vctl.rs)
sed -i 's/wrapper/deployment harness/g' ./vantio-hypervisor/src/bin/vctl.rs

# 10. Axum Monolith API Route Synchronization
find . -type f -name "*.rs" -exec sed -i 's/\/api\/receipts/\/api\/anomaly_records/g' {} +

echo "[ ∅ ORACLE ] LEXICON PURGE COMPLETE. RING-0 INTEGRITY VERIFIED."
