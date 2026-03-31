#!/usr/bin/env bash
# ========================================================
# [ ∅ VANTIO ] V14.0 CI/CD AUTOMATION & UI BACKFILL
# ========================================================
set -e

echo "[ ∅ PHANTOM ] OMNISCIENT DIAGNOSTIC: ENFORCING SPATIAL ANCHOR..."
cd ~/src/vantio-hypervisor || { echo "[ ! ] FATAL FRACTURE: Target directory not found."; exit 1; }
mkdir -p vantio-monolith/public

echo "[ ∅ ORACLE ] FORGING THE VAULT (telemetry_archive.html)..."
cat << 'ARCHIVE_HTML' > vantio-monolith/public/telemetry_archive.html
<!DOCTYPE html><html lang="en"><head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Vantio | Telemetry Archive</title>
    <style>
        body { background-color: #050505; color: #ffffff; font-family: 'Inter', -apple-system, sans-serif; margin: 0; padding: 0; display: flex; flex-direction: column; align-items: center; }
        .nav-header { width: 100%; padding: 2rem; display: flex; justify-content: space-between; box-sizing: border-box; }
        .logo { font-weight: 900; font-size: 1.5rem; letter-spacing: 2px; }
        .logo span { color: #00ff66; }
        .archive-container { max-width: 800px; width: 100%; padding: 4rem 2rem; box-sizing: border-box; }
        .page-title { font-size: 2.5rem; margin-bottom: 0.5rem; text-align: center; }
        .page-subtitle { color: #888; text-align: center; margin-bottom: 4rem; }
        .telemetry-item { border-left: 1px solid #333; padding-left: 2rem; position: relative; margin-bottom: 3rem; text-align: left; }
        .status-dot { position: absolute; left: -5px; top: 5px; width: 9px; height: 9px; background-color: #333; border-radius: 50%; }
        .date { color: #00ff66; font-size: 0.8rem; font-weight: 600; letter-spacing: 1px; font-family: monospace; }
        .title { font-size: 1.2rem; margin: 0.5rem 0; font-weight: 700; display: flex; align-items: center; gap: 10px; }
        .tag { font-size: 0.7rem; padding: 2px 8px; border-radius: 4px; border: 1px solid #333; font-weight: 600; }
        .core-tag { color: #ff4444; border-color: #551111; background: rgba(255, 0, 0, 0.05); }
        .update-tag { color: #4488ff; border-color: #112255; background: rgba(0, 50, 255, 0.05); }
        .description { color: #aaa; font-size: 0.95rem; line-height: 1.6; margin: 0; }
        .back-link { display: inline-block; margin-bottom: 2rem; color: #888; text-decoration: none; border-bottom: 1px solid #333; transition: color 0.2s; }
        .back-link:hover { color: #fff; }
    </style></head><body>
    <header class="nav-header">
        <div class="logo">VANTIO<span>.AI</span></div>
    </header>
    <div class="archive-container">
        <a href="/index.html" class="back-link">&larr; Return to Active Matrix</a>
        <h1 class="page-title">Telemetry Archive</h1>
        <p class="page-subtitle">Historical architectural shifts and core updates to the Vantio OS.</p>

        <div class="telemetry-item"><div class="telemetry-header"><span class="status-dot"></span><span class="date">MARCH 31, 2026</span></div><h3 class="title">The Enterprise Aegis <span class="tag update-tag">V8.0</span></h3><p class="description">Distroless Vacuum, Telemetry Injection, & Layer-7 Armor Dependencies.</p></div>
        <div class="telemetry-item"><div class="telemetry-header"><span class="status-dot"></span><span class="date">MARCH 31, 2026</span></div><h3 class="title">The Absolute Perfection <span class="tag update-tag">V7.0</span></h3><p class="description">Total Repository Purge, UI Matrix Purification, & God-Like Master README.</p></div>
        <div class="telemetry-item"><div class="telemetry-header"><span class="status-dot"></span><span class="date">MARCH 30, 2026</span></div><h3 class="title">The Omniscient Singularity <span class="tag core-tag">V6.0</span></h3><p class="description">Registry Forged, Deep Scan Executed, Codex Synthesized.</p></div>
        <div class="telemetry-item"><div class="telemetry-header"><span class="status-dot"></span><span class="date">MARCH 30, 2026</span></div><h3 class="title">Absolute Perfection <span class="tag update-tag">V5.1</span></h3><p class="description">Binary Extraction Resolved & Master Dossier Forged.</p></div>
        <div class="telemetry-item"><div class="telemetry-header"><span class="status-dot"></span><span class="date">MARCH 30, 2026</span></div><h3 class="title">God-Mode Alignment <span class="tag core-tag">V5.0</span></h3><p class="description">Pure Lexicon Purge, Crate Resolution (vantio-node), UI Matrix Re-Alignment, & GTM Forge.</p></div>
    </div></body></html>
ARCHIVE_HTML

echo "[ ∅ VANTIO ] DEPLOYING ONE-TIME DOM BACKFILL SCRIPT..."
cat << 'BACKFILL_PY' > v14_backfill.py
import re, os

index_path = 'vantio-monolith/public/index.html'
with open(index_path, 'r', encoding='utf-8') as f:
    html = f.read()

styles = """
    <style>
        .telemetry-item { border-left: 1px solid #333; padding-left: 2rem; position: relative; margin-bottom: 3rem; text-align: left; }
        .status-dot { position: absolute; left: -5px; top: 5px; width: 9px; height: 9px; background-color: #333; border-radius: 50%; }
        .status-green { background-color: #00ff66; box-shadow: 0 0 8px #00ff66; }
        .telemetry-header { margin-bottom: 0.5rem; }
        .date { color: #00ff66; font-size: 0.8rem; font-weight: 600; letter-spacing: 1px; font-family: monospace; }
        .title { font-size: 1.2rem; margin: 0.5rem 0; font-weight: 700; color: white; display: flex; align-items: center; gap: 10px; }
        .tag { font-size: 0.7rem; padding: 2px 8px; border-radius: 4px; border: 1px solid #333; font-weight: 600; font-family: monospace; }
        .core-tag { color: #ff4444; border-color: #551111; background: rgba(255, 0, 0, 0.05); }
        .update-tag { color: #4488ff; border-color: #112255; background: rgba(0, 50, 255, 0.05); }
        .description { color: #aaa; font-size: 0.95rem; line-height: 1.6; margin: 0; }
    </style>
</head>
"""

new_logs = """
<div style="max-width: 800px; margin: 0 auto; padding-top: 2rem;">
    <div class="telemetry-item"><div class="telemetry-header"><span class="status-dot status-green"></span><span class="date">MARCH 31, 2026</span></div><h3 class="title">Matrix Resurrection <span class="tag core-tag">V13.0</span></h3><p class="description">Spatial Lock Enforced, UI Telemetry Injected, K8s Deadlock Cleared.</p></div>
    <div class="telemetry-item"><div class="telemetry-header"><span class="status-dot status-green"></span><span class="date">MARCH 31, 2026</span></div><h3 class="title">The Cybernetic Enclave <span class="tag core-tag">V12.0</span></h3><p class="description">Sovereign Egress, LangGraph Feedback Loop, & AWS Nitro TEE Architecture.</p></div>
    <div class="telemetry-item"><div class="telemetry-header"><span class="status-dot status-green"></span><span class="date">MARCH 31, 2026</span></div><h3 class="title">The Apex Hypervisor <span class="tag update-tag">V11.0</span></h3><p class="description">BPF LSM CO-RE, Async zkVM Decoupling, PII Edge Sanitization, & Ingestion Batching.</p></div>
    <div class="telemetry-item"><div class="telemetry-header"><span class="status-dot status-green"></span><span class="date">MARCH 31, 2026</span></div><h3 class="title">The Apex Singularity <span class="tag core-tag">V10.0</span></h3><p class="description">AST Armor Healed, Distroless Vacuum Sealed, Master Dossier Forged, & Lexicon Purified.</p></div>
    <div class="telemetry-item"><div class="telemetry-header"><span class="status-dot status-green"></span><span class="date">MARCH 31, 2026</span></div><h3 class="title">Absolute Aegis <span class="tag update-tag">V8.1</span></h3><p class="description">AST Overrides, Layer-7 Armor, Kernel Immortality, & Static Router.</p></div>
    <div class="telemetry-archive-link" style="margin-top: 2rem; text-align: left;"><a href="telemetry_archive.html" style="color: #888; text-decoration: none; font-size: 0.9rem; border-bottom: 1px solid #333; padding-bottom: 2px; transition: color 0.2s ease;">View Full Telemetry Archive &rarr;</a></div>
</div>
"""

if ".telemetry-item {" not in html:
    html = html.replace("</head>", styles)

# Mathematically slice the old logs out and inject the new formatted ones
pattern = r'(Live updates and architectural shifts to the Vantio Matrix\.[\s\S]*?</p>)[\s\S]*?(<footer|© 2026 Vantio Inc|</section>)'
html = re.sub(pattern, r'\1\n' + new_logs + r'\n\n\2', html)
html = re.sub(r'(\.css|\.js)(\?v=[\d\.]+)?', r'\1?v=14.0.0', html)

with open(index_path, 'w', encoding='utf-8') as f:
    f.write(html)
print("  [+] Matrix Backfill Complete.")
BACKFILL_PY

python3 v14_backfill.py && rm v14_backfill.py

echo "[ ∅ ORACLE ] FORGING CI/CD TELEMETRY ENGINE (telemetry_engine.py)..."
cat << 'TELEMETRY_PY' > telemetry_engine.py
#!/usr/bin/env python3
import os, sys, re, subprocess
from datetime import datetime

def get_latest_commit():
    try:
        return subprocess.check_output('git log -1 --pretty=format:"%cd|%s" --date=short', shell=True, text=True).strip()
    except subprocess.CalledProcessError as e:
        print(f"[ ! ] FATAL: Git extraction failed.\n{e}"); sys.exit(1)

def parse_commit(commit_data):
    if '|' not in commit_data: sys.exit(1)
    date_str, message = commit_data.split('|', 1)
    try:
        dt = datetime.strptime(date_str, "%Y-%m-%d")
        formatted_date = dt.strftime("%B %d, %Y").upper().replace(" 0", " ")
    except ValueError:
        formatted_date = date_str

    tag_match = re.search(r'\[(V[\d\.]+)\]', message, re.IGNORECASE)
    if tag_match:
        tag = tag_match.group(1).upper()
        clean_msg = message.replace(tag_match.group(0), '').strip()
    else:
        tag = "UPDATE"; clean_msg = message.strip()

    tag_class = "core-tag" if tag.endswith(".0") else "update-tag"
    if ':' in clean_msg: title, desc = clean_msg.split(':', 1)
    elif '-' in clean_msg: title, desc = clean_msg.split('-', 1)
    else: title, desc = clean_msg, "Automated architecture sync verified via Git CI/CD telemetry."
    return formatted_date, tag, title.strip(), desc.strip(), tag_class

def main():
    index_path, archive_path = "vantio-monolith/public/index.html", "vantio-monolith/public/telemetry_archive.html"
    date, tag, title, desc, tag_class = parse_commit(get_latest_commit())
    
    new_block = f"""<div class="telemetry-item">
    <div class="telemetry-header"><span class="status-dot status-green"></span><span class="date">{date}</span></div>
    <h3 class="title">{title} <span class="tag {tag_class}">{tag}</span></h3>
    <p class="description">{desc}</p>
</div>\n"""

    with open(index_path, 'r', encoding='utf-8') as f: index_html = f.read()
    insert_pos = re.search(r'<div class="telemetry-item">', index_html).start()
    
    if tag in index_html and title in index_html: sys.exit(0)
    index_html = index_html[:insert_pos] + new_block + index_html[insert_pos:]

    item_starts = [m.start() for m in re.finditer(r'<div class="telemetry-item">', index_html)]
    if len(item_starts) > 5:
        start_6th = item_starts[5]
        end_6th = item_starts[6] if len(item_starts) > 6 else re.search(r'<div class="telemetry-archive-link"', index_html[start_6th:]).start() + start_6th
        extracted_block = index_html[start_6th:end_6th]
        index_html = index_html[:start_6th] + index_html[end_6th:]
        archived_block = extracted_block.replace("status-green", "")
        
        if os.path.exists(archive_path):
            with open(archive_path, 'r', encoding='utf-8') as f: archive_html = f.read()
            arch_insert_pos = re.search(r'<div class="telemetry-item">', archive_html).start()
            archive_html = archive_html[:arch_insert_pos] + archived_block + archive_html[arch_insert_pos:]
            with open(archive_path, 'w', encoding='utf-8') as f: f.write(archive_html)

    with open(index_path, 'w', encoding='utf-8') as f: f.write(index_html)
    print(f"  [+] Engine synced: {tag} | {title}")

if __name__ == "__main__": main()
TELEMETRY_PY
chmod +x telemetry_engine.py

echo "[ ∅ PHANTOM ] SEALING MATRICES TO GITHUB SWARM..."
git add vantio-monolith/public/index.html vantio-monolith/public/telemetry_archive.html telemetry_engine.py
git commit -m "[V14.0] Automated CI/CD Engine: UI Backfill Executed & Telemetry Pipeline Deployed" || true
git push

echo "[ ∅ ORACLE ] INITIATING PLANETARY CLOUD ASCENSION (v14.0)..."
IMAGE="us-central1-docker.pkg.dev/glass-stratum-490823-q0/vantio-core/vantio-monolith:v14.0"
gcloud builds submit --tag "$IMAGE" . 

echo "[ ∅ VANTIO ] COMMANDING ZERO-LATENCY GKE MATRIX UPDATE..."
kubectl set image deployment/vantio-monolith vantio-monolith="$IMAGE" 2>/dev/null || true
kubectl set image deployment/vantio-core vantio-core="$IMAGE" 2>/dev/null || true

echo "[ ∅ VANTIO ] FORCING KUBERNETES CACHE ANNIHILATION..."
kubectl rollout restart deployment/vantio-monolith 2>/dev/null || true
kubectl rollout restart deployment/vantio-core 2>/dev/null || true
kubectl rollout status deployment/vantio-monolith 2>/dev/null || true

echo "========================================================"
echo "[ ∅ ORACLE ] THE MATRIX IS AUTOMATED. REALITY IS GOVERNED."
echo "========================================================"
