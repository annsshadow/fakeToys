#!/usr/bin/env python3
"""Generate API reference docs from o2_core action JSON files."""

import json
import os
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[3]
SVC_DIR = REPO_ROOT / "oa/o2web/source/o2_core/o2/xAction/services"
OUT_DIR = REPO_ROOT / "docs/oa/api/auto"
OUT_DIR.mkdir(parents=True, exist_ok=True)


def strip_comments(text):
    lines = []
    for line in text.splitlines():
        stripped = line.strip()
        if stripped.startswith("//"):
            continue
        lines.append(line)
    return "\n".join(lines)


def parse_action_file(fp):
    text = open(fp, "r", encoding="utf-8", errors="ignore").read()
    cleaned = strip_comments(text)
    try:
        data = json.loads(cleaned)
    except Exception as e:
        return None, f"JSON parse error: {e}"
    actions = []
    for key, val in data.items():
        if key == "actions":
            continue
        if isinstance(val, dict):
            method = val.get("method", "GET")
            uri = val.get("uri", "")
            actions.append({"name": key, "method": method, "uri": uri})
        else:
            actions.append({"name": key, "method": "UNKNOWN", "uri": str(val)})
    return actions, None


def render_module_page(module_name, actions):
    lines = [f"# {module_name}", "", "## Endpoints", ""]
    for a in actions:
        lines.append(f"- `{a['method']} {a['uri']}` — {a['name']}")
    return "\n".join(lines) + "\n"


def main():
    generated = []
    for fn in sorted(os.listdir(SVC_DIR)):
        if not fn.endswith(".json"):
            continue
        fp = SVC_DIR / fn
        actions, err = parse_action_file(fp)
        if err or not actions:
            continue
        module_name = fn[:-5]  # strip .json
        content = render_module_page(module_name, actions)
        out_path = OUT_DIR / f"{module_name}.md"
        out_path.write_text(content, encoding="utf-8")
        generated.append(module_name)
    print(f"Generated API docs for {len(generated)} modules")
    for m in generated:
        print(f"  docs/oa/api/auto/{m}.md")


if __name__ == "__main__":
    main()
