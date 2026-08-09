"""
extract_endpoints.py — 从 oa4rust crates 扫描所有路由定义，生成行为对比测试端点清单。

输出三种格式：
  1. CSV  → endpoints.csv
  2. JSON → endpoints.json
  3. Rust 代码片段（可直接粘贴到 tests/behavior_compare.rs 的 ENDPOINTS 数组）

用法：
  python scripts/extract_endpoints.py
"""

import csv
import json
import os
import re
import sys
from pathlib import Path

CRATES_DIR = Path(__file__).resolve().parent.parent / "crates"

# ── Java 模块映射 ────────────────────────────────────────────────────────────
# 规则：从 crate 名称推导 Java WAR 名称（x_<crate>-assembled 前缀）
JAVA_WAR_RULES = [
    # 组织/人员
    ("organization_assemble_control", "x_organization_assemble_control"),
    ("organization_assemble_express", "x_organization_assemble_express"),
    ("organization_core_entity", "x_organization_core_entity"),
    ("organization_core_express", "x_organization_core_express"),
    ("control", "x_organization_assemble_control"),
    ("personal", "x_organization_assemble_personal"),
    ("personal_extend", "x_personal_extend"),
    # 考勤
    ("attendance_assemble_control", "x_attendance_assemble_control"),
    ("attendance_core_entity", "x_attendance_core_entity"),
    ("attendance", "x_attendance_core_entity"),
    # 日历
    ("calendar_assemble_control", "x_calendar_assemble_control"),
    ("calendar_core_entity", "x_calendar_core_entity"),
    ("calendar", "x_calendar_core_entity"),
    # 文件
    ("file_assemble_control", "x_file_assemble_control"),
    ("file_core_entity", "x_file_core_entity"),
    ("file", "x_file_core_entity"),
    # 通用
    ("general_assemble_control", "x_general_assemble_control"),
    ("general_core_entity", "x_general_core_entity"),
    ("general", "x_general_core_entity"),
    # BBS
    ("bbs_assemble_control", "x_bbs_assemble_control"),
    ("bbs_core_entity", "x_bbs_core_entity"),
    ("bbs", "x_bbs_core_entity"),
    # 组件
    ("component_assemble_control", "x_component_assemble_control"),
    ("component_core_entity", "x_component_core_entity"),
    ("component", "x_component_core_entity"),
    # AI
    ("ai_assemble_control", "x_ai_assemble_control"),
    ("ai_core_entity", "x_ai_core_entity"),
    ("ai", "x_ai_core_entity"),
    # 热图
    ("hotpic_assemble_control", "x_hotpic_assemble_control"),
    ("hotpic_core_entity", "x_hotpic_core_entity"),
    ("hotpic", "x_hotpic_core_entity"),
    # 极光推送
    ("jpush_assemble_control", "x_jpush_assemble_control"),
    ("jpush_core_entity", "x_jpush_core_entity"),
    ("jpush", "x_jpush_core_entity"),
    # 思维导图
    ("mind_assemble_control", "x_mind_assemble_control"),
    ("mind_core_entity", "x_mind_core_entity"),
    ("mind", "x_mind_core_entity"),
    # 会议
    ("meeting_assemble_control", "x_meeting_assemble_control"),
    ("meeting_core_entity", "x_meeting_core_entity"),
    ("meeting", "x_meeting_core_entity"),
    # 消息
    ("message_assemble_communicate", "x_message_assemble_communicate"),
    ("message_core_entity", "x_message_core_entity"),
    ("message", "x_message_core_entity"),
    # 门户
    ("portal_assemble_designer", "x_portal_assemble_designer"),
    ("portal_assemble_surface", "x_portal_assemble_surface"),
    ("portal_core_entity", "x_portal_core_entity"),
    ("portal", "x_portal_core_entity"),
    # 流程平台
    ("processplatform_assemble_bam", "x_processplatform_assemble_bam"),
    ("processplatform_assemble_designer", "x_processplatform_assemble_designer"),
    ("processplatform_assemble_surface", "x_processplatform_assemble_surface"),
    ("processplatform_core_entity", "x_processplatform_core_entity"),
    ("processplatform_core_express", "x_processplatform_core_express"),
    ("processplatform_service_processing", "x_processplatform_service_processing"),
    ("process_designer", "x_processplatform_assemble_designer"),
    ("process_express", "x_processplatform_core_express"),
    ("process_surface", "x_processplatform_assemble_surface"),
    ("process_bam", "x_processplatform_assemble_bam"),
    # 查询
    ("query_assemble_designer", "x_query_assemble_designer"),
    ("query_assemble_surface", "x_query_assemble_surface"),
    ("query_core_entity", "x_query_core_entity"),
    ("query_core_express", "x_query_core_express"),
    ("query_express", "x_query_express"),
    ("query_service", "x_query_service"),
    ("query_service_processing", "x_query_service_processing"),
    # CMS
    ("cms_assemble_control", "x_cms_assemble_control"),
    ("cms_control", "x_cms_control"),
    ("cms_core_entity", "x_cms_core_entity"),
    ("cms_core_express", "x_cms_core_express"),
    ("cms_express", "x_cms_express"),
    # 关联
    ("correlation_service_processing", "x_correlation_service_processing"),
    ("correlation_core_entity", "x_correlation_core_entity"),
    ("correlation_core_express", "x_correlation_core_express"),
    ("correlation", "x_correlation_core_entity"),
    # 基础
    ("base", "x_base_core_project"),
    ("auth", "x_organization_assemble_authentication"),
    ("express", "x_organization_assemble_express"),
    ("console", "x_console"),
    ("program_init", "x_program_init"),
    ("program_center", "x_program_center"),
    ("program_center_core_entity", "x_program_center_core_entity"),
    ("openapi", "x_openapi"),
    ("mcp_server", "x_mcp_server"),
    ("orm", "x_orm"),
    ("shared", "x_shared"),
]


def java_war_for(crate_name: str) -> str:
    for pattern, war in JAVA_WAR_RULES:
        if crate_name == pattern or crate_name.startswith(pattern + "_"):
            return war
    # fallback：x_<crate_name>
    return f"x_{crate_name}"


def java_action_from_path(crate_name: str, rust_path: str) -> str:
    """
    根据 crate 名称和 Rust 路径推导 Java action。
    策略：去掉 /jaxrs/ 前缀，再用 crate 名称替换路径中的模块前缀。
    """
    path = rust_path.lstrip("/")
    # 常见前缀映射
    prefix_map = {
        "organization_assemble_control": "organization",
        "organization_assemble_express": "express",
        "organization_core_entity": "organization",
        "organization_core_express": "organization",
        "attendance_assemble_control": "attendance",
        "attendance_core_entity": "attendance",
        "attendance": "attendance",
        "calendar_assemble_control": "calendar",
        "calendar_core_entity": "calendar",
        "calendar": "calendar",
        "file_assemble_control": "file",
        "file_core_entity": "file",
        "file": "file",
        "general_assemble_control": "general",
        "general_core_entity": "general",
        "general": "general",
        "bbs_assemble_control": "bbs",
        "bbs_core_entity": "bbs",
        "bbs": "bbs",
        "component_assemble_control": "component",
        "component_core_entity": "component",
        "component": "component",
        "ai_assemble_control": "ai",
        "ai_core_entity": "ai",
        "ai": "ai",
        "hotpic_assemble_control": "hotpic",
        "hotpic_core_entity": "hotpic",
        "hotpic": "hotpic",
        "jpush_assemble_control": "jpush",
        "jpush_core_entity": "jpush",
        "jpush": "jpush",
        "mind_assemble_control": "mind",
        "mind_core_entity": "mind",
        "mind": "mind",
        "meeting_assemble_control": "meeting",
        "meeting_core_entity": "meeting",
        "meeting": "meeting",
        "message_assemble_communicate": "message",
        "message_core_entity": "message",
        "message": "message",
        "portal_assemble_designer": "portal",
        "portal_assemble_surface": "portal",
        "portal_core_entity": "portal",
        "portal": "portal",
        "processplatform_assemble_bam": "process",
        "processplatform_assemble_designer": "process",
        "processplatform_assemble_surface": "process",
        "processplatform_core_entity": "process",
        "processplatform_core_express": "process",
        "processplatform_service_processing": "process",
        "process_designer": "process",
        "process_express": "process",
        "process_surface": "process",
        "process_bam": "process",
        "query_assemble_designer": "query",
        "query_assemble_surface": "query",
        "query_core_entity": "query",
        "query_core_express": "query",
        "query_express": "query",
        "query_service": "query",
        "query_service_processing": "query",
        "cms_assemble_control": "cms",
        "cms_control": "cms",
        "cms_core_entity": "cms",
        "cms_core_express": "cms",
        "cms_express": "cms",
        "correlation_service_processing": "correlation",
        "correlation_core_entity": "correlation",
        "correlation_core_express": "correlation",
        "correlation": "correlation",
        "base": "base",
        "auth": "authentication",
        "express": "express",
        "console": "console",
        "program_init": "secret",
        "program_center": "program_center",
    }
    module_prefix = prefix_map.get(crate_name, crate_name)
    # 去掉 module 前缀，保留剩余部分作为 action
    action = path
    for mp in sorted(prefix_map.values(), key=len, reverse=True):
        if action.startswith(mp + "/"):
            action = action[len(mp) + 1:]
            break
    return action


# ── 路由提取 ─────────────────────────────────────────────────────────────────

# 匹配 .route("...", get/ post/ put/ delete(...))
ROUTE_RE = re.compile(r'\.route\(\s*"([^"]+)"\s*,\s*(get|post|put|delete)\(')

# 匹配多行路由：.route(\n    "path",\n    get(...)
ROUTE_MULTI_RE = re.compile(r'\.route\(\s*\n\s*"([^"]+)"\s*,\s*\n\s*(get|post|put|delete)\(')


def extract_routes_from_file(filepath: str, crate_name: str) -> list:
    """从单个 Rust 文件提取所有路由定义。"""
    try:
        with open(filepath, "r", encoding="utf-8") as f:
            content = f.read()
    except (OSError, UnicodeDecodeError):
        return []

    endpoints = []
    # 先匹配多行格式
    for m in ROUTE_MULTI_RE.finditer(content):
        path = m.group(1)
        method = m.group(2).upper()
        endpoints.append({
            "crate_name": crate_name,
            "method": method,
            "rust_path": path,
        })
    # 再匹配单行格式（排除已被多行匹配覆盖的行）
    for m in ROUTE_RE.finditer(content):
        path = m.group(1)
        method = m.group(2).upper()
        # 检查是否已被多行正则捕获（位置重叠）
        already_found = any(
            e["rust_path"] == path and e["method"] == method
            for e in endpoints
        )
        if not already_found:
            endpoints.append({
                "crate_name": crate_name,
                "method": method,
                "rust_path": path,
            })
    return endpoints


def discover_crates() -> dict:
    """
    扫描 crates/ 目录，返回 {crate_name: [endpoint, ...]}。
    优先使用 routes.rs；若不存在则扫描 lib.rs。
    """
    result = {}
    for crate_dir in sorted(CRATES_DIR.iterdir()):
        if not crate_dir.is_dir():
            continue
        src_dir = crate_dir / "src"
        if not src_dir.is_dir():
            continue
        crate_name = crate_dir.name
        endpoints = []

        routes_file = src_dir / "routes.rs"
        if routes_file.is_file():
            endpoints = extract_routes_from_file(str(routes_file), crate_name)

        # 补充 lib.rs 中的路由（部分 crate 路由定义在 lib.rs 内）
        lib_file = src_dir / "lib.rs"
        if lib_file.is_file():
            lib_endpoints = extract_routes_from_file(str(lib_file), crate_name)
            # 去重
            existing = {(e["method"], e["rust_path"]) for e in endpoints}
            for e in lib_endpoints:
                key = (e["method"], e["rust_path"])
                if key not in existing:
                    endpoints.append(e)
                    existing.add(key)

        if endpoints:
            result[crate_name] = endpoints

    return result


# ── 输出 ─────────────────────────────────────────────────────────────────────

def deduplicate(endpoints: list) -> list:
    """按 (method, rust_path) 去重，保留第一条（优先 routes.rs）。"""
    seen = set()
    unique = []
    for e in endpoints:
        key = (e["method"], e["rust_path"])
        if key not in seen:
            seen.add(key)
            unique.append(e)
    return unique


def enrich(endpoints: list) -> list:
    """为每个端点补充 java_war 和 java_action 字段。"""
    for e in endpoints:
        crate = e["crate_name"]
        e["java_war"] = java_war_for(crate)
        e["java_action"] = java_action_from_path(crate, e["rust_path"])
    return endpoints


def output_csv(endpoints: list, path: Path):
    path.write_text("", encoding="utf-8")
    with open(path, "w", encoding="utf-8-sig", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["crate_name", "method", "rust_path", "java_war", "java_action"])
        for e in endpoints:
            writer.writerow([
                e["crate_name"],
                e["method"],
                e["rust_path"],
                e["java_war"],
                e["java_action"],
            ])
    print(f"  CSV  → {path}  ({len(endpoints)} 行)")


def output_json(endpoints: list, path: Path):
    path.write_text(json.dumps(endpoints, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"  JSON → {path}  ({len(endpoints)} 条)")


def output_rust(endpoints: list, path: Path):
    lines = [
        "/// 行为对比测试端点列表（由 extract_endpoints.py 自动生成，请勿手动修改）",
        "/// 生成时间: " + __import__("datetime").datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
        "",
        "const ENDPOINTS: &[EndpointDef] = &[",
    ]
    for e in endpoints:
        body = "None"
        if e["method"] in ("POST", "PUT", "PATCH"):
            body = 'Some(serde_json::json!({}))'
        auth = "true" if e["method"] != "GET" else "false"
        lines.append("    EndpointDef {")
        lines.append(f'        crate_name: "{e["crate_name"]}",')
        lines.append(f'        method: "{e["method"]}",')
        lines.append(f'        rust_path: "{e["rust_path"]}",')
        lines.append(f'        java_war: "{e["java_war"]}",')
        lines.append(f'        java_action: "{e["java_action"]}",')
        lines.append(f"        body: {body},")
        lines.append(f"        requires_auth: {auth},")
        lines.append("    },")
    lines.append("];")
    path.write_text("\n".join(lines), encoding="utf-8")
    print(f"  Rust → {path}  ({len(endpoints)} 条)")


# ── 主流程 ───────────────────────────────────────────────────────────────────

def main():
    print("扫描 crates/ 目录中的路由定义 ...")
    raw = discover_crates()

    all_endpoints = []
    for crate_name, eps in sorted(raw.items()):
        all_endpoints.extend(eps)
        print(f"  {crate_name:45s}  {len(eps):5d} 条")

    print(f"\n原始端点数: {len(all_endpoints)}")

    endpoints = deduplicate(enrich(all_endpoints))
    endpoints.sort(key=lambda e: (e["crate_name"], e["method"], e["rust_path"]))

    print(f"去重后端点数: {len(endpoints)}")

    scripts_dir = Path(__file__).resolve().parent
    out_dir = scripts_dir.parent / "target" / "endpoints"
    out_dir.mkdir(parents=True, exist_ok=True)

    csv_path = out_dir / "endpoints.csv"
    json_path = out_dir / "endpoints.json"
    rust_path = out_dir / "endpoints.rs"

    output_csv(endpoints, csv_path)
    output_json(endpoints, json_path)
    output_rust(endpoints, rust_path)

    # 同时生成一份可直接粘贴到 tests/behavior_compare.rs 的代码片段
    rust_test_path = scripts_dir.parent / "tests" / "behavior_compare_endpoints.rs"
    lines = [
        "/// 行为对比测试端点列表（由 extract_endpoints.py 自动生成）",
        "/// 复制下面的 ENDPOINTS 数组到 tests/behavior_compare.rs 中替换原有内容。",
        "",
    ]
    # 复制 Rust 输出格式
    for line in rust_path.read_text(encoding="utf-8").splitlines():
        lines.append(line)
    rust_test_path.write_text("\n".join(lines), encoding="utf-8")
    print(f"  Test → {rust_test_path}")

    # 按方法分组统计
    from collections import Counter
    method_counts = Counter(e["method"] for e in endpoints)
    print(f"\n方法分布: {dict(method_counts)}")

    crate_counts = Counter(e["crate_name"] for e in endpoints)
    print(f" Crate 数: {len(crate_counts)}")

    print(f"\n完成。文件位于 {out_dir}")
    return len(endpoints)


if __name__ == "__main__":
    count = main()
    sys.exit(0)
