#!/usr/bin/env python3
"""Generate module and component cards for oa documentation."""

import os
import re
import xml.etree.ElementTree as ET
from pathlib import Path

# Base paths
REPO_ROOT = Path(__file__).resolve().parents[3]
O2SERVER_POM = REPO_ROOT / "oa/o2server/pom.xml"
O2SERVER_SRC = REPO_ROOT / "oa/o2server"
O2WEB_SRC = REPO_ROOT / "oa/o2web/source"
MODULES_OUT = REPO_ROOT / "docs/oa/modules/o2server"
COMPONENTS_OUT = REPO_ROOT / "docs/oa/modules/o2web"
TEMPLATES_DIR = REPO_ROOT / "docs/oa/templates"

MODULE_CARD_TEMPLATE = (TEMPLATES_DIR / "module-card.md").read_text(encoding="utf-8")
COMPONENT_CARD_TEMPLATE = (TEMPLATES_DIR / "component-card.md").read_text(encoding="utf-8")

# Module types that typically expose REST endpoints
API_MODULE_TYPES = {"assemble_control", "assemble_surface", "service_processing", "assemble_designer"}


def parse_parent_pom():
    """Return list of module directory names from parent pom.xml."""
    tree = ET.parse(O2SERVER_POM)
    root = tree.getroot()
    ns = {"m": "http://maven.apache.org/POM/4.0.0"}
    modules = [
        m.text.strip()
        for m in root.findall(".//m:modules/m:module", ns)
        if m.text
    ]
    return modules


def parse_module_pom(module_dir):
    """Parse a module pom.xml and return artifactId, packaging, dependencies."""
    pom_path = O2SERVER_SRC / module_dir / "pom.xml"
    if not pom_path.exists():
        return None, None, []
    tree = ET.parse(pom_path)
    root = tree.getroot()
    ns = {"m": "http://maven.apache.org/POM/4.0.0"}
    artifact_id = root.findtext(".//m:artifactId", default="", namespaces=ns)
    packaging = root.findtext(".//m:packaging", default="jar", namespaces=ns)
    deps = [
        d.findtext(".//m:artifactId", default="", namespaces=ns)
        for d in root.findall(".//m:dependencies/m:dependency", ns)
    ]
    deps = [d for d in deps if d]
    return artifact_id, packaging, deps


def guess_core_classes(module_dir):
    """List Java classes under src/main/java, limited to top-level package files."""
    src_dir = O2SERVER_SRC / module_dir / "src/main/java"
    if not src_dir.exists():
        return []
    classes = []
    for root, dirs, files in os.walk(src_dir):
        for f in files:
            if f.endswith(".java"):
                rel = Path(root).relative_to(src_dir)
                class_name = f[:-5]
                pkg = ".".join(rel.parts)
                if pkg:
                    classes.append(f"{pkg}.{class_name}")
                else:
                    classes.append(class_name)
                if len(classes) >= 10:
                    return classes
    return classes


def has_rest_endpoints(module_dir, packaging):
    """Heuristic: return True if the module likely exposes REST endpoints."""
    if packaging in API_MODULE_TYPES:
        return True
    src_dir = O2SERVER_SRC / module_dir / "src/main/java"
    if not src_dir.exists():
        return False
    for root, dirs, files in os.walk(src_dir):
        for f in files:
            if f.endswith(".java"):
                p = Path(root) / f
                try:
                    text = p.read_text(encoding="utf-8", errors="ignore")
                    if "@Path" in text or "javax.ws.rs" in text or "jaxrs" in text:
                        return True
                except Exception:
                    pass
    return False


def generate_module_card(module_dir):
    """Generate a Markdown module card for a single module."""
    artifact_id, packaging, deps = parse_module_pom(module_dir)
    if not artifact_id:
        return None

    module_name = artifact_id
    core_classes = guess_core_classes(module_dir)
    rest = has_rest_endpoints(module_dir, packaging or "jar")

    card = MODULE_CARD_TEMPLATE.replace("{{module_name}}", module_name)
    card = card.replace("<!-- Human-authored: describe what this module does in 1-3 sentences. -->", "")
    card = card.replace(
        "<!-- Human-authored or generated from src/main/java tree listing. -->",
        "\n".join(f"- {c}" for c in core_classes) if core_classes else "",
    )
    card = card.replace(
        "<!-- Generated from pom.xml dependency list. -->",
        "",
    )
    card = card.replace(
        "- [List of dependent modules with links]",
        "\n".join(f"- {d}" for d in deps) if deps else "- None listed",
    )

    if rest:
        card = card.replace(
            "<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->",
            "",
        )
        card = card.replace(
            "- [Endpoint list]",
            "- *To be populated from Swagger annotations or action JSON.*",
        )
    else:
        card = card.replace(
            "<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->\n- [Endpoint list]\n",
            "",
        )

    return card.strip() + "\n"


def generate_component_card(component_dir):
    """Generate a Markdown component card for a single x_component_* directory."""
    component_name = component_dir.name
    main_js = component_dir / "Main.js"
    package_json = component_dir / "package.json"

    entry_point = ""
    if main_js.exists():
        entry_point = str(main_js.relative_to(REPO_ROOT))
    elif package_json.exists():
        entry_point = f"{component_dir.name}/package.json (no Main.js; modern build entry)"

    build_pipeline = ""
    if package_json.exists():
        try:
            import json
            pkg = json.loads(package_json.read_text(encoding="utf-8"))
            scripts = pkg.get("scripts", {})
            if scripts:
                build_pipeline = "\n".join(f"- `{k}`: {v}" for k, v in scripts.items())
        except Exception:
            pass

    key_configs = []
    for root, dirs, files in os.walk(component_dir):
        for f in files:
            if f.endswith(".json") and f != "package.json":
                rel = Path(root) / f
                key_configs.append(f"- `{rel.relative_to(component_dir)}`: *To be described.*")
            if len(key_configs) >= 10:
                break
        if len(key_configs) >= 10:
            break

    card = COMPONENT_CARD_TEMPLATE.replace("{{component_name}}", component_name)
    card = card.replace("<!-- Human-authored: describe what this component does in 1-3 sentences. -->", "")
    card = card.replace(
        "<!-- Generated from Main.js or package.json main field. -->",
        f"- `{entry_point}`" if entry_point else "",
    )
    card = card.replace(
        "<!-- Generated from package.json scripts/build tooling if present. -->",
        build_pipeline if build_pipeline else "- No build scripts detected",
    )
    card = card.replace(
        "<!-- Human-authored or generated from directory listing. -->",
        "\n".join(key_configs) if key_configs else "- No JSON config files detected",
    )

    return card.strip() + "\n"


def main():
    MODULES_OUT.mkdir(parents=True, exist_ok=True)
    COMPONENTS_OUT.mkdir(parents=True, exist_ok=True)

    modules = parse_parent_pom()
    print(f"Found {len(modules)} modules in parent POM")

    for module_dir_name in modules:
        card = generate_module_card(module_dir_name)
        if not card:
            continue
        out_path = MODULES_OUT / f"{module_dir_name}.md"
        out_path.write_text(card, encoding="utf-8")
        print(f"Generated module card: {out_path.relative_to(REPO_ROOT)}")

    if O2WEB_SRC.exists():
        component_dirs = sorted(
            d for d in O2WEB_SRC.iterdir()
            if d.is_dir() and d.name.startswith("x_component_")
        )
        print(f"Found {len(component_dirs)} x_component_* directories")
        for component_dir in component_dirs:
            card = generate_component_card(component_dir)
            out_path = COMPONENTS_OUT / f"{component_dir.name}.md"
            out_path.write_text(card, encoding="utf-8")
            print(f"Generated component card: {out_path.relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
