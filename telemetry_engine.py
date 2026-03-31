#!/usr/bin/env python3
import os, sys, re, subprocess
from datetime import datetime

def get_latest_commit():
    try: return subprocess.check_output('git log -1 --pretty=format:"%cd|%s" --date=short', shell=True, text=True).strip()
    except: sys.exit(1)

def parse_commit(data):
    if '|' not in data: sys.exit(1)
    d, m = data.split('|', 1)
    try: d = datetime.strptime(d, "%Y-%m-%d").strftime("%B %d, %Y").upper().replace(" 0", " ")
    except: pass
    t = re.search(r'\[(V[\d\.]+)\]', m, re.IGNORECASE)
    tag = t.group(1).upper() if t else "UPDATE"
    cm = m.replace(t.group(0), '').strip() if t else m.strip()
    tc = "core-tag" if tag.endswith(".0") else "update-tag"
    if ':' in cm: title, desc = cm.split(':', 1)
    elif '-' in cm: title, desc = cm.split('-', 1)
    else: title, desc = cm, "Automated architecture sync verified via Git CI/CD telemetry."
    return d, tag, title.strip(), desc.strip(), tc

def main():
    # EXPLICITLY TARGETING dashboard.html
    idx = "vantio-monolith/public/dashboard.html"
    arc = "vantio-monolith/public/telemetry_archive.html"
    
    if not os.path.exists(idx):
        print(f"[ ! ] Fatal: {idx} not found.")
        sys.exit(1)

    date, tag, title, desc, tag_class = parse_commit(get_latest_commit())
    
    blk = f"""<div class="telemetry-item">
    <div class="telemetry-header"><span class="status-dot status-green"></span><span class="date">{date}</span></div>
    <h3 class="title">{title} <span class="tag {tag_class}">{tag}</span></h3>
    <p class="description">{desc}</p>
</div>\n"""

    with open(idx, 'r', encoding='utf-8') as f: h = f.read()
    if title in h: sys.exit(0)
    
    m = re.search(r'<div class="telemetry-item">', h)
    if not m: sys.exit(1)
    pos = m.start()
    h = h[:pos] + blk + h[pos:]

    items = [x.start() for x in re.finditer(r'<div class="telemetry-item">', h)]
    if len(items) > 5:
        s6 = items[5]
        e6 = items[6] if len(items) > 6 else re.search(r'<div class="telemetry-archive-link"', h[s6:]).start() + s6
        ext = h[s6:e6]
        h = h[:s6] + h[e6:]
        arc_blk = ext.replace("status-green", "")
        if os.path.exists(arc):
            with open(arc, 'r', encoding='utf-8') as f: ah = f.read()
            am = re.search(r'<div class="telemetry-item">', ah)
            if am:
                ap = am.start()
                ah = ah[:ap] + arc_blk + ah[ap:]
                with open(arc, 'w', encoding='utf-8') as f: f.write(ah)

    with open(idx, 'w', encoding='utf-8') as f: f.write(h)
    print(f"  [+] Engine synced: {tag} | {title}")

if __name__ == "__main__": main()
