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
