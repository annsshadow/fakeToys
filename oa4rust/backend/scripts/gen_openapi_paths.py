#!/usr/bin/env python3
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
# -*- coding: utf-8 -*-
"""
gen_openapi_paths.py — 从 oa4rust crates 源码扫描路由，生成完整的 OpenAPI lib.rs。

输出：
  - 直接重写 crates/openapi/src/lib.rs（原子写入）

用法：
  python scripts/gen_openapi_paths.py
"""

import re
import shutil
import subprocess
import sys
import tempfile
import os
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CRATES_DIR = ROOT / "crates"


def run_rustfmt(paths):
    """生成物直接过 rustfmt（与 `cargo fmt --all -- --check` 同一工具链）。

    O13：生成器产出的长行若不过 rustfmt，会和 CI 的格式化门禁互相打架——
    `cargo fmt --check` 要求换行，regen diff 门禁要求逐字相等，两者不可能同时绿。
    """
    exe = shutil.which("rustfmt")
    if not exe:
        print("WARN: rustfmt 不在 PATH，跳过生成物格式化（提交前请手动跑 cargo fmt）")
        return
    for path in paths:
        subprocess.run(
            [exe, "--edition", "2021", str(path)],
            check=True,
            capture_output=True,
        )


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

REQUEST_STRUCTS: dict = {}
USED_REQUEST_TYPES: set = set()


def scan_request_structs():
    """收集请求体 DTO 类型名 -> (crate 名, 模块路径)。

    模块路径是类型从 crate 根可见的路径（如 control::group）。只收 src 下
    非测试文件中的 `pub struct`（测试件不进库目标，re-export 会失败）。
    """
    for crate_dir in CRATES_DIR.iterdir():
        if not crate_dir.is_dir():
            continue
        src_dir = crate_dir / "src"
        if not src_dir.is_dir():
            continue
        for rs_file in src_dir.rglob("*.rs"):
            rel_parts = rs_file.relative_to(src_dir).parts
            if "tests" in rel_parts or any("test" in part.lower() for part in rel_parts):
                continue
            try:
                text = rs_file.read_text(encoding="utf-8")
            except Exception:
                continue
            text = strip_test_modules(text)
            for match in re.finditer(r'pub\s+struct\s+(\w+(?:Create|Update)Request)', text):
                if rs_file.name == "lib.rs":
                    mod_path = ""
                else:
                    mod_path = "::".join(str(part) for part in rs_file.relative_to(src_dir).with_suffix("").parts)
                REQUEST_STRUCTS.setdefault(match.group(1), (crate_dir.name, mod_path))


def infer_request_body(path: str, method: str) -> str:
    if method not in ("POST", "PUT", "PATCH"):
        return ""
    p = path.replace("/api/", "").strip("/")
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


def strip_test_modules(text: str) -> str:
    """移除内联 `#[cfg(test)] mod tests { ... }` 块。

    生成器此前只按**文件路径**排除测试（`tests/`、`*test*.rs`），而
    `body_limit.rs` 这类把测试模块写在业务文件末尾的写法会漏网：
    `#[cfg(test)] mod tests` 里的 `.route("__test_upload", ...)` 被当成
    真实路由写进 OpenAPI（R24 踩过一次）。测试模块不进库目标，扫描前先剥掉。
    按花括号配平删除，处理嵌套大括号。
    """
    out = []
    pos = 0
    pattern = re.compile(r'#\[cfg\(test\)\]\s*(?:pub\s+)?mod\s+\w+\s*\{')
    while True:
        m = pattern.search(text, pos)
        if not m:
            out.append(text[pos:])
            break
        out.append(text[pos:m.start()])
        i = m.end() - 1  # 指向模块的 `{`
        depth = 0
        while i < len(text):
            ch = text[i]
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    break
            i += 1
        pos = i + 1
    return ''.join(out)


def parse_routes_from_file(filepath: Path) -> list:
    try:
        text = filepath.read_text(encoding="utf-8")
    except Exception:
        return []

    text = strip_test_modules(text)
    text_flat = re.sub(r'\s+', ' ', text)
    routes = []
    seen_keys = set()

    # 与 gen_mcp_tools.py 同口径（优化轮13）：按括号配平切出每个 .route 的 handler
    # 表达式，再枚举链上全部 HTTP 方法。此前 multi_pattern 只认「首方法 put/delete/patch
    # 的恰好两段链」、single_pattern 只取链首方法 —— 三方法 CRUD get(h).put(h).delete(h)
    # 会丢 PUT/DELETE，令 OpenAPI 文档缺失这些方法。
    method_call = re.compile(r'(?:crate::)?\b(get|post|put|delete|patch|head|options)\s*\(\s*(?:crate::)?([\w:]+)')
    for m in re.finditer(r'\.route\(\s*"([^"]+)"\s*,', text_flat):
        path = m.group(1)
        start = m.end()
        depth = 1
        i = start
        while i < len(text_flat) and depth > 0:
            ch = text_flat[i]
            if ch == '(':
                depth += 1
            elif ch == ')':
                depth -= 1
            i += 1
        expr = text_flat[start:i]
        seen_methods = set()
        for mm in method_call.finditer(expr):
            method_str = mm.group(1).upper()
            handler = mm.group(2).split('::')[-1]
            if method_str in seen_methods:
                continue
            seen_methods.add(method_str)
            key = (path, method_str, handler)
            if key not in seen_keys:
                seen_keys.add(key)
                routes.append((path, method_str, handler))

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
    p = path.replace("/api/", "")
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
    # 清洗非法 Rust 标识符字符（路径段可能带 '-' 等，如 config-open）
    name = re.sub(r'[^A-Za-z0-9_]', '_', name)
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
    used_func_names = {}
    for r in sorted(unique, key=lambda x: (x["crate"], x["method"], x["path"])):
        tag = TAG_MAP.get(r["crate"], r["crate"])
        tags.add(tag)
        func_name = path_to_func_name(r["crate"], r["path"], r["method"], r["handler"])
        # 不同路径段蛇形转换后可能撞名（如 market/flag/uninstall vs market_flag/uninstall）
        if func_name in used_func_names:
            used_func_names[func_name] += 1
            func_name = f"{func_name}_{used_func_names[func_name]}"
        else:
            used_func_names[func_name] = 1
        func_names.append(func_name)
        http_method = r["method"].lower()
        path_params = re.findall(r"\{(.+?)\}", r["path"])
        request_body = infer_request_body(r["path"], r["method"])
        if request_body:
            USED_REQUEST_TYPES.add(request_body)

        attr_lines = build_utoipa_attr(http_method, r["path"], tag, path_params, request_body)
        func_body = f"async fn {func_name}() {{}}"
        funcs.append("\n".join(attr_lines + [func_body]))

    # Build lib.rs
    lines = []
    lines.append("// Copyright (C) 2026 annsshadow")
    lines.append("// SPDX-License-Identifier: AGPL-3.0-or-later")
    lines.append("// AUTO-GENERATED by scripts/gen_openapi_paths.py — DO NOT EDIT")
    lines.append(f"// Total: {len(funcs)} path items, {len(tags)} tags")
    # 第 68 轮：输出保持确定性（不含生成时刻）——带时间戳会让产物永远「不干净」，
    # 漂移守卫（check_openapi_up_to_date.py）的 regen diff 也随之恒非空。
    lines.append("")
    lines.append("#![allow(dead_code)]")
    # 路径段的驼峰（如 tableFlag）转蛇形后仍留大写尾巴，统一放行 snake_case 告警
    lines.append("#![allow(non_snake_case)]")
    lines.append("")
    lines.append("use utoipa::OpenApi;")
    lines.append("mod request_types;")
    lines.append("use crate::request_types::*;")
    # O5：手工维护的测试模块（tests.rs）必须挂进模块树——否则文件在≠测试在跑
    # （test_module_registration_guard 正是查这个）。
    lines.append("#[cfg(test)]")
    lines.append("mod tests;")
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
    lines.append("    modifiers(&SecurityAddon),")
    lines.append("    info(")
    lines.append('        title = "OA4Rust API",')
    lines.append('        description = "OA4Rust OpenAPI specification (auto-generated)",')
    lines.append('        version = "0.1.0"')
    lines.append("    )")
    lines.append(")]")
    lines.append("pub struct ApiDoc;")
    lines.append("")
    lines.append("/// Security scheme definitions (Bearer token auth, see shared/src/middleware/token.rs).")
    lines.append("pub struct SecurityAddon;")
    lines.append("")
    lines.append("impl utoipa::Modify for SecurityAddon {")
    lines.append("    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {")
    lines.append("        let components = openapi.components.get_or_insert_with(Default::default);")
    lines.append("        components.add_security_scheme(")
    lines.append('            "bearer_token",')
    lines.append("            utoipa::openapi::security::SecurityScheme::Http(")
    lines.append("                utoipa::openapi::security::HttpBuilder::new()")
    lines.append("                    .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)")
    lines.append('                    .bearer_format("token")')
    lines.append("                    .build(),")
    lines.append("            ),")
    lines.append("        );")
    lines.append("    }")
    lines.append("}")

    content = "\n".join(lines)

    # 生成请求体 DTO 的 re-export 模块：utoipa 6 的宏要求 request_body 类型
    # 在 openapi crate 作用域内可解析（utoipa 4 只记路径不解析），
    # 用完全限定 re-export 让宏能拿到类型定义。
    rt_lines = [
        "// Copyright (C) 2026 annsshadow",
        "// SPDX-License-Identifier: AGPL-3.0-or-later",
        "// AUTO-GENERATED by scripts/gen_openapi_paths.py — DO NOT EDIT",
        "// Re-exports 请求体 DTO 类型，供 utoipa 宏在 openapi crate 作用域内解析。",
        "",
    ]
    for type_name in sorted(USED_REQUEST_TYPES):
        crate_name, mod_path = REQUEST_STRUCTS[type_name]
        if mod_path:
            rt_lines.append(f"pub use {crate_name}::{mod_path}::{type_name};")
        else:
            rt_lines.append(f"pub use {crate_name}::{type_name};")
    rt_content = "\n".join(rt_lines) + "\n"
    rt_path = OUTPUT_FILE.parent / "request_types.rs"
    fd2, tmp2 = tempfile.mkstemp(suffix=".rs", dir=str(rt_path.parent))
    try:
        with os.fdopen(fd2, 'w', encoding='utf-8') as f:
            f.write(rt_content)
        os.replace(tmp2, str(rt_path))
    except Exception:
        if os.path.exists(tmp2):
            os.remove(tmp2)
        raise

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
    run_rustfmt([rt_path, OUTPUT_FILE])


if __name__ == "__main__":
    main()
