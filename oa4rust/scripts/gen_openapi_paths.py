#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
gen_openapi_paths.py — 从 oa4rust crates 源码扫描路由，生成完整的 OpenAPI lib.rs。

输出：
  - 直接重写 crates/openapi/src/lib.rs（原子写入）

用法：
  python scripts/gen_openapi_paths.py
"""

import re
import sys
import tempfile
import os
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CRATES_DIR = ROOT / "crates"
OUTPUT_FILE = CRATES_DIR / "openapi" / "src" / "lib.rs"

TAG_MAP = {
    "auth": "authentication",
    "personal": "personal",
    "personal_extend": "personal",
    "control": "control",
    "program_init": "program",
    "attendance": "attendance",
    "attendance_assemble_control": "attendance",
    "attendance_core_entity": "attendance",
    "calendar": "calendar",
    "calendar_assemble_control": "calendar",
    "calendar_core_entity": "calendar",
    "file": "file",
    "file_assemble_control": "file",
    "file_core_entity": "file",
    "general": "general",
    "general_assemble_control": "general",
    "general_core_entity": "general",
    "bbs": "bbs",
    "bbs_assemble_control": "bbs",
    "bbs_core_entity": "bbs",
    "component": "component",
    "component_assemble_control": "component",
    "component_core_entity": "component",
    "ai": "ai",
    "ai_assemble_control": "ai",
    "ai_core_entity": "ai",
    "message": "message",
    "message_assemble_communicate": "message",
    "message_core_entity": "message",
    "portal": "portal",
    "portal_assemble_designer": "portal",
    "portal_assemble_surface": "portal",
    "portal_core_entity": "portal",
    "process_designer": "process",
    "process_express": "process",
    "process_surface": "process",
    "process_bam": "process",
    "processplatform_assemble_bam": "process",
    "processplatform_assemble_designer": "process",
    "processplatform_assemble_surface": "process",
    "processplatform_core_entity": "process",
    "processplatform_core_express": "process",
    "processplatform_service_processing": "process",
    "program_center": "program",
    "program_center_core_entity": "program",
    "base": "base",
    "query_service": "query",
    "query_express": "query",
    "query_core_entity": "query",
    "query_core_express": "query",
    "query_assemble_designer": "query",
    "query_assemble_surface": "query",
    "query_service_processing": "query",
    "cms_control": "cms",
    "cms_express": "cms",
    "cms_assemble_control": "cms",
    "cms_core_entity": "cms",
    "cms_core_express": "cms",
    "meeting": "meeting",
    "meeting_assemble_control": "meeting",
    "meeting_core_entity": "meeting",
    "jpush": "jpush",
    "jpush_assemble_control": "jpush",
    "jpush_core_entity": "jpush",
    "hotpic": "hotpic",
    "hotpic_assemble_control": "hotpic",
    "hotpic_core_entity": "hotpic",
    "mind": "mind",
    "mind_assemble_control": "mind",
    "mind_core_entity": "mind",
    "correlation": "correlation",
    "correlation_core_entity": "correlation",
    "correlation_core_express": "correlation",
    "correlation_service_processing": "correlation",
    "organization_assemble_control": "organization",
    "organization_assemble_express": "organization",
    "organization_core_entity": "organization",
    "organization_core_express": "organization",
    "console": "console",
    "express": "express",
    "orm": "orm",
    "shared": "shared",
    "openapi": "openapi",
    "mcp_server": "mcp",
}

REQUEST_STRUCTS = set()


def scan_request_structs():
    for crate_dir in CRATES_DIR.iterdir():
        if not crate_dir.is_dir():
            continue
        for rs_file in crate_dir.rglob("*.rs"):
            try:
                text = rs_file.read_text(encoding="utf-8")
            except Exception:
                continue
            for match in re.finditer(r'pub\s+struct\s+(\w+(?:Create|Update)Request)', text):
                REQUEST_STRUCTS.add(match.group(1))


def infer_request_body(path: str, method: str) -> str:
    if method not in ("POST", "PUT", "PATCH"):
        return ""
    p = path.replace("/jaxrs/", "").strip("/")
    parts = p.split("/")
    # Skip leading crate-like segment
    if parts:
        first = parts[0]
        if '_' in first:
            parts = parts[1:]
    if len(parts) >= 1:
        entity = parts[0]
        suffix = "Create" if method == "POST" else "Update"
        candidate = f"{entity.capitalize()}{suffix}Request"
        if candidate in REQUEST_STRUCTS:
            return candidate
    return ""


def parse_routes_from_file(filepath: Path) -> list:
    try:
        text = filepath.read_text(encoding="utf-8")
    except Exception:
        return []

    text_flat = re.sub(r'\s+', ' ', text)
    routes = []
    seen_keys = set()

    # Multi-method: .route("path", put(h1).delete(h2))
    multi_pattern = r'\.route\(\s*"([^"]+)",\s*(put|delete|patch)\((\w+)\)\.(\w+)\((\w+)\)'
    for match in re.finditer(multi_pattern, text_flat):
        path = match.group(1)
        for method_str, handler in [(match.group(2), match.group(3)), (match.group(4), match.group(5))]:
            key = (path, method_str.upper(), handler)
            if key not in seen_keys:
                seen_keys.add(key)
                routes.append((path, method_str.upper(), handler))

    # Single-method: .route("path", get(handler))
    single_pattern = r'\.route\(\s*"([^"]+)",\s*(get|post|put|delete|patch)\((\w+)\)'
    for match in re.finditer(single_pattern, text_flat):
        path = match.group(1)
        method_str = match.group(2)
        handler = match.group(3)
        key = (path, method_str.upper(), handler)
        if key not in seen_keys:
            seen_keys.add(key)
            routes.append((path, method_str.upper(), handler))

    return routes


def extract_crate_name(filepath: Path) -> str:
    parts = filepath.parts
    for i, part in enumerate(parts):
        if part == "crates" and i + 1 < len(parts):
            return parts[i + 1]
    return "unknown"


def path_to_func_name(crate: str, path: str, method: str, handler: str) -> str:
    """将路由路径转换为唯一函数名。"""
    import hashlib
    p = path.replace("/jaxrs/", "")
    parts = p.split("/")
    func_parts = [crate]
    for part in parts:
        param_match = re.match(r"\{(.+?)\}", part)
        if param_match:
            func_parts.append(param_match.group(1).lower())
        else:
            s = re.sub(r'([A-Z])', r'_\1', part).lower().strip("_")
            if s and (not func_parts or func_parts[-1] != s):
                func_parts.append(s)
    func_parts.append(handler)
    func_parts.append(method.lower())
    name = "_".join(func_parts)
    # Truncate long names with hash
    if len(name) > 100:
        short = hashlib.md5(f"{path}_{method}".encode()).hexdigest()[:6]
        name = f"{crate}_{short}_{method.lower()}"
    return name


def build_utoipa_attr(http_method: str, path: str, tag: str,
                      path_params: list, request_body: str) -> list:
    """Build utoipa::path attribute lines (multi-line format)."""
    lines = [f"#[utoipa::path({http_method},"]
    lines.append(f'    path = "{path}",')
    lines.append(f'    tag = "{tag}",')

    if path_params:
        items = []
        for pp in path_params:
            items.append(f'        ("{pp}" = String, Path, description = "{pp} parameter")')
        lines.append("    params(")
        lines.append(",\n".join(items))
        lines.append("    ),")

    if request_body:
        lines.append(f"    request_body = {request_body},")

    lines.append("    responses(")
    lines.append('        (status = 200, description = "Success", body = serde_json::Value),')
    lines.append('        (status = 400, description = "Bad Request"),')
    lines.append('        (status = 401, description = "Unauthorized"),')
    lines.append('        (status = 500, description = "Internal Server Error")')
    lines.append("    ),")
    lines.append(")]")
    return lines


def main():
    scan_request_structs()

    all_routes = []
    for crate_dir in sorted(CRATES_DIR.iterdir()):
        if not crate_dir.is_dir():
            continue
        src_dir = crate_dir / "src"
        if not src_dir.is_dir():
            continue
        crate_name = crate_dir.name
        for rs_file in src_dir.rglob("*.rs"):
            routes = parse_routes_from_file(rs_file)
            for path, method, handler in routes:
                all_routes.append({
                    "crate": crate_name,
                    "path": path,
                    "method": method,
                    "handler": handler,
                })

    seen = set()
    unique = []
    for r in all_routes:
        key = (r["crate"], r["path"], r["method"])
        if key not in seen:
            seen.add(key)
            unique.append(r)

    tags = set()
    funcs = []
    func_names = []
    for r in sorted(unique, key=lambda x: (x["crate"], x["method"], x["path"])):
        tag = TAG_MAP.get(r["crate"], r["crate"])
        tags.add(tag)
        func_name = path_to_func_name(r["crate"], r["path"], r["method"], r["handler"])
        func_names.append(func_name)
        http_method = r["method"].lower()
        path_params = re.findall(r"\{(.+?)\}", r["path"])
        request_body = infer_request_body(r["path"], r["method"])

        attr_lines = build_utoipa_attr(http_method, r["path"], tag, path_params, request_body)
        func_body = f"async fn {func_name}() {{}}"
        funcs.append("\n".join(attr_lines + [func_body]))

    # Build lib.rs
    lines = []
    lines.append("// AUTO-GENERATED by scripts/gen_openapi_paths.py — DO NOT EDIT")
    lines.append(f"// Total: {len(funcs)} path items, {len(tags)} tags")
    lines.append(f"// Generated at: {__import__('datetime').datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    lines.append("")
    lines.append("#![allow(dead_code)]")
    lines.append("")
    lines.append("use utoipa::OpenApi;")
    lines.append("")

    # Health check
    lines.append('#[utoipa::path(get, path = "/health", tag = "base",')
    lines.append("    responses(")
    lines.append('        (status = 200, description = "Health check"),')
    lines.append("    ),")
    lines.append(")]")
    lines.append("async fn health_check() {}")
    lines.append("")

    # All path handlers
    for f in funcs:
        lines.append(f)
    lines.append("")

    # OpenAPI derive
    lines.append("#[derive(OpenApi)]")
    lines.append("#[openapi(")
    lines.append("    paths(")
    lines.append("        health_check,")
    for name in func_names:
        lines.append(f"        {name},")
    lines.append("    ),")
    lines.append("    tags(")
    for tag in sorted(tags):
        lines.append(f'        (name = "{tag}", description = "{tag.capitalize()} operations"),')
    lines.append("    ),")
    lines.append("    info(")
    lines.append('        title = "OA4Rust API",')
    lines.append('        description = "OA4Rust OpenAPI specification (auto-generated)",')
    lines.append('        version = "0.1.0"')
    lines.append("    )")
    lines.append(")]")
    lines.append("pub struct ApiDoc;")

    content = "\n".join(lines)

    # Atomic write
    output_dir = OUTPUT_FILE.parent
    output_dir.mkdir(parents=True, exist_ok=True)
    fd, tmp_path = tempfile.mkstemp(suffix=".rs", dir=str(output_dir))
    try:
        with os.fdopen(fd, 'w', encoding='utf-8') as f:
            f.write(content)
        os.replace(tmp_path, str(OUTPUT_FILE))
    except Exception:
        if os.path.exists(tmp_path):
            os.remove(tmp_path)
        raise

    print(f"Generated {len(funcs)} path items, {len(tags)} tags -> {OUTPUT_FILE}")


if __name__ == "__main__":
    main()
