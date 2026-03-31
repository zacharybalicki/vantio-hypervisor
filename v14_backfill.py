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
