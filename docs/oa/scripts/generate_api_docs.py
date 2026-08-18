#!/usr/bin/env python3
"""Generate API reference docs for all o2server modules."""

import json
import os
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[3]
SVC_DIR = REPO_ROOT / "oa/o2web/source/o2_core/o2/xAction/services"
OUT_DIR = REPO_ROOT / "docs/oa/api/auto"
O2SERVER_DIR = REPO_ROOT / "oa/o2server"
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
    if actions:
        for a in actions:
            lines.append(f"- `{a['method']} {a['uri']}` — {a['name']}")
    else:
        lines.append("<!-- No action JSON found for this module -->")
        lines.append("")
        lines.append("No REST endpoints discovered from action JSON files.")
    return "\n".join(lines) + "\n"


def get_all_module_names():
    """Scan o2server directory for all module names."""
    modules = []
    for d in sorted(O2SERVER_DIR.iterdir()):
        if d.is_dir() and d.name.startswith("x_"):
            modules.append(d.name)
    return modules


def get_json_module_names():
    """Get module names from existing action JSON files."""
    names = set()
    if SVC_DIR.exists():
        for fn in SVC_DIR.iterdir():
            if fn.name.endswith(".json"):
                names.add(fn.stem)
    return names


def main():
    all_modules = get_all_module_names()
    json_modules = get_json_module_names()

    generated = []
    skeletons = []

    for module_name in all_modules:
        json_fn = f"{module_name}.json"
        json_path = SVC_DIR / json_fn

        if json_path.exists():
            actions, err = parse_action_file(json_path)
            if err:
                content = render_module_page(module_name, [])
                skeletons.append(module_name)
            elif actions:
                content = render_module_page(module_name, actions)
                generated.append(module_name)
            else:
                content = render_module_page(module_name, [])
                skeletons.append(module_name)
        else:
            content = render_module_page(module_name, [])
            skeletons.append(module_name)

        out_path = OUT_DIR / f"{module_name}.md"
        out_path.write_text(content, encoding="utf-8")

    print(f"Generated API docs for {len(generated)} modules with action JSON")
    print(f"Created {len(skeletons)} skeleton docs for modules without action JSON")
    print(f"Total: {len(generated) + len(skeletons)} modules in {OUT_DIR}")

    # Update README
    readme_path = REPO_ROOT / "docs/oa/api/README.md"
    existing = readme_path.read_text(encoding="utf-8") if readme_path.exists() else ""

    module_list = "\n".join(f"- [{m}](auto/{m}.md)" for m in all_modules)
    new_readme = f"""# API Reference

Auto-generated API documentation for all o2server modules.

## Generation

Run `python docs/oa/scripts/generate_api_docs.py` to regenerate.

## Modules

{module_list}
"""
    readme_path.write_text(new_readme, encoding="utf-8")
    print(f"Updated {readme_path}")


if __name__ == "__main__":
    main()
