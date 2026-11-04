#!/usr/bin/env python3
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
# -*- coding: utf-8 -*-
"""
gen_mcp_tools.py — 从 oa4rust crates 源码扫描路由定义，生成 MCP 工具注册代码。

输出：
  - crates/mcp_server/src/generated_routes.rs（Rust 代码，可直接 include!）

用法：
  python scripts/gen_mcp_tools.py
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CRATES_DIR = ROOT / "crates"
OUTPUT_FILE = CRATES_DIR / "mcp_server" / "src" / "generated_routes.rs"
# 与 generated_routes.rs 同源输出：前端方法契约 fixture。
# O15：历史上这个文件由人手/一次性脚本维护，与真实路由漂移了 131/63/2 条。
FIXTURE_FILE = ROOT.parent / "tests" / "contracts" / "backend-route-methods.fixture.ts"
# 同一扫描的路由快照（desktop-endpoints 守卫用）。O16 起不再手工维护。
REGISTERED_FIXTURE_FILE = ROOT.parent / "tests" / "contracts" / "backend-registered-routes.fixture.ts"

ROUTE_METHODS = ("get", "post", "put", "delete", "patch", "head", "options")
METHOD_RE = re.compile(r"\s*(?:axum::routing::)?(" + "|".join(ROUTE_METHODS) + r")\s*\(")
METHOD_CHAIN_RE = re.compile(r"\s*\.\s*(?:axum::routing::)?(" + "|".join(ROUTE_METHODS) + r")\s*\(")

# 框架级路由：不在任何 crate 的 .route() 扫描结果里，但属于后端接口面。
# fixture 需要它们，MCP 工具表不需要（GET /mcp、GET /health 是探针/说明，不是工具）。
EXTRA_FIXTURE_ROUTES = {
    "/health": {"GET"},
    "/openapi.json": {"GET"},
    "/mcp": {"GET", "POST"},
}


def is_test_source(filepath: Path) -> bool:
    """测试源码里的 .route() 不是产品路由，不能进工具表/fixture。"""
    p = filepath.as_posix()
    name = filepath.name
    return (
        "/tests/" in p
        or name == "tests.rs"
        or "_tests" in name
        or name.endswith("_test.rs")
    )


def strip_cfg_test_blocks(text: str) -> str:
    """删除 `#[cfg(test)]` 覆盖的 item（测试模块/函数），避免把 /boom 之类测试路由
    当成产品路由。字符串/字符字面量感知，避免路径里的 `{}` 干扰配平。"""
    while True:
        m = re.search(r"#\[cfg\(test\)\]", text)
        if not m:
            return text
        i = text.find("{", m.end())
        if i == -1:
            return text
        depth = 0
        j = i
        n = len(text)
        in_str = False
        in_char = False
        while j < n:
            c = text[j]
            if in_str:
                if c == "\\":
                    j += 2
                    continue
                if c == '"':
                    in_str = False
            elif in_char:
                if c == "\\":
                    j += 2
                    continue
                if c == "'":
                    in_char = False
            else:
                if c == '"':
                    in_str = True
                elif c == "'":
                    # 区分字符字面量与生命周期：'a' 是字符，'a 是生命周期
                    if j + 1 < n and text[j + 1] == "\\":
                        in_char = True
                    elif j + 2 < n and text[j + 2] == "'":
                        in_char = True
                elif c == "{":
                    depth += 1
                elif c == "}":
                    depth -= 1
                    if depth == 0:
                        break
            j += 1
        text = text[:m.start()] + text[j + 1:]

# 权限级别映射（路径前缀 -> 是否需要认证）
PUBLIC_PREFIXES = [
    "/api/authentication",
    "/api/authentication/captcha",
    "/api/authentication/oauth",
    "/api/authentication/code",
    "/api/authentication/refresh",
    "/api/reset",
    "/api/secret/check",
    "/api/secret/set",
    "/health",
]

def is_public_path(path: str) -> bool:
    for prefix in PUBLIC_PREFIXES:
        if path.startswith(prefix):
            return True
    return False


def strip_rust_comments(text: str) -> str:
    """剥离 Rust 注释（// 行注释、/* */ 块注释），保留字符串/字符字面量。

    必需：十类功能1 安全重设计起，check/password 等路由在 `.route( ... )` 的
    左括号与路径字面量之间插入了 `//` 说明注释。旧实现直接 `re.sub('\\s+',' ',text)`
    压平空白后匹配 `\\.route\\(\\s*"([^"]+)"`，注释夹在 `(` 与 `"path"` 之间导致
    正则失配 → 该路由被漏扫，整文件再生即净删路由。故先剥注释再压平。
    逐字符扫描以区分字符串/字符字面量内的 `//`、`/*`（非注释）。
    """
    out = []
    i, n = 0, len(text)
    in_str = False
    in_char = False
    while i < n:
        c = text[i]
        two = text[i:i + 2]
        if in_str:
            out.append(c)
            if c == "\\" and i + 1 < n:
                out.append(text[i + 1])
                i += 2
                continue
            if c == '"':
                in_str = False
            i += 1
            continue
        if in_char:
            out.append(c)
            if c == "\\" and i + 1 < n:
                out.append(text[i + 1])
                i += 2
                continue
            if c == "'":
                in_char = False
            i += 1
            continue
        if c == "/":
            if two == "//":
                j = i + 2
                while j < n and text[j] != "\n":
                    j += 1
                i = j  # 保留换行符本身
                continue
            if two == "/*":
                j = i + 2
                while j + 1 < n and not (text[j] == "*" and text[j + 1] == "/"):
                    j += 1
                i = j + 2 if j + 1 < n else n
                out.append(" ")
                continue
            out.append(c)
            i += 1
            continue
        if c == '"':
            in_str = True
            out.append(c)
            i += 1
            continue
        if c == "'":
            # 区分字符字面量与生命周期：'a' 是字符，'a 是生命周期。
            # 旧实现无条件进入 in_char，遇到 `'static` 会把后续整段（含注释）吞成
            # “字符字面量”，导致注释里的 `.route(` 被当成注册点。
            if i + 1 < n and text[i + 1] == "\\":
                in_char = True
            elif i + 2 < n and text[i + 2] == "'":
                in_char = True
            out.append(c)
            i += 1
            continue
        out.append(c)
        i += 1
    return "".join(out)

def _mask_strings(s: str) -> str:
    """把字符串/字符字面量的内容替换为同长空格，用于定位真正的 `.route(` 站点。

    processplatform 的 u2_collect_routes 把 `.route(` 写进了字符串常量里（它扫描
    自己的源码），不掩码会把那段文字当成注册点。
    """
    out = list(s)
    i = 0
    n = len(s)
    while i < n:
        c = s[i]
        if c == '"':
            out[i] = " "
            i += 1
            while i < n:
                if s[i] == "\\":
                    out[i] = " "
                    if i + 1 < n:
                        out[i + 1] = " "
                    i += 2
                    continue
                if s[i] == '"':
                    out[i] = " "
                    i += 1
                    break
                out[i] = " "
                i += 1
            continue
        if c == "'":
            # 只掩码单字符字面量；生命周期 'a 不动
            if i + 2 < n and (s[i + 1] == "\\" or s[i + 2] == "'"):
                j = i + 1
                if s[j] == "\\":
                    j += 2
                else:
                    j += 1
                if j < n and s[j] == "'":
                    for k in range(i, j + 1):
                        out[k] = " "
                    i = j + 1
                    continue
            i += 1
            continue
        i += 1
    return "".join(out)


def _match_paren(s: str, open_idx: int) -> int:
    """返回与 s[open_idx] 处 '(' 配平的 ')' 下标；尊重字符串/字符字面量转义。"""
    depth = 0
    in_str = False
    in_char = False
    esc = False
    i = open_idx
    while i < len(s):
        c = s[i]
        if in_str:
            if esc:
                esc = False
            elif c == "\\":
                esc = True
            elif c == '"':
                in_str = False
        elif in_char:
            if esc:
                esc = False
            elif c == "\\":
                esc = True
            elif c == "'":
                in_char = False
        else:
            if c == '"':
                in_str = True
            elif c == "'":
                # 'a' 是字符字面量，'a 是生命周期
                if i + 2 < len(s) and s[i + 2] == "'":
                    in_char = True
            elif c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
                if depth == 0:
                    return i
        i += 1
    return -1


def _normalize_legacy_path(class_path: str, method_path: str) -> str:
    """复刻 bbs 的 u2::normalize_legacy_path：去空段 + 去相邻重复段。"""
    raw = f"{class_path}/{method_path}"
    segs = []
    for seg in raw.split("/"):
        if not seg:
            continue
        if segs and segs[-1] == seg:
            continue
        segs.append(seg)
    return "/".join(segs)


def _find_binding(text: str, name: str, before: int):
    """在 `before` 之前找最后一个 `let/const name = "..."` 绑定。"""
    pat = re.compile(
        r"\b(?:let|const)\s+" + re.escape(name) + r"\s*(?::\s*&?(?:str|String)\s*)?=\s*\"((?:[^\"\\]|\\.)*)\""
    )
    val = None
    for m in pat.finditer(text[:before]):
        val = m.group(1)
    return val


def _find_const(text: str, name: str):
    """全文件查找 `const/static name: &str = "..."`（模块级常量）。"""
    pat = re.compile(
        r"\b(?:pub(?:\(\w+\))?\s+)?(?:const|static)\s+"
        + re.escape(name)
        + r"\s*:\s*&(?:\'[^\']+\s+)?str\s*=\s*\"((?:[^\"\\]|\\.)*)\""
    )
    m = pat.search(text)
    return m.group(1) if m else None


def _split_top_level_args(s: str):
    args = []
    cur = []
    depth = 0
    in_str = False
    esc = False
    for c in s:
        if in_str:
            cur.append(c)
            if esc:
                esc = False
            elif c == "\\":
                esc = True
            elif c == '"':
                in_str = False
            continue
        if c == '"':
            in_str = True
            cur.append(c)
        elif c in "([{":
            depth += 1
            cur.append(c)
        elif c in ")]}":
            depth -= 1
            cur.append(c)
        elif c == "," and depth == 0:
            args.append("".join(cur).strip())
            cur = []
        else:
            cur.append(c)
    if cur:
        args.append("".join(cur).strip())
    return args


def _resolve_format_template(tmpl: str, args, text: str, before: int):
    """展开 Rust format! 模板：`{{`/`}}` 是字面花括号，`{name}` / `{}` 是插值。

    区分「真字面段」与「局部变量前缀」的全部依据就在这里：只有能解析到
    let/const 绑定的占位符才算变量，解不出来的直接返回 None（由调用方点名，
    绝不把 `{p}` 当字面路径段写进 fixture）。
    """
    out = []
    arg_idx = 0
    i = 0
    while i < len(tmpl):
        c = tmpl[i]
        if c == "{":
            if i + 1 < len(tmpl) and tmpl[i + 1] == "{":
                out.append("{")
                i += 2
                continue
            j = tmpl.find("}", i + 1)
            if j == -1:
                return None
            key = tmpl[i + 1:j].strip()
            if key == "":
                if arg_idx >= len(args):
                    return None
                val = _resolve_value(args[arg_idx], text, before)
                arg_idx += 1
            else:
                val = _resolve_value(key, text, before)
            if val is None:
                return None
            out.append(val)
            i = j + 1
        elif c == "}":
            if i + 1 < len(tmpl) and tmpl[i + 1] == "}":
                out.append("}")
                i += 2
                continue
            return None
        else:
            out.append(c)
            i += 1
    return "".join(out)


def _resolve_value(expr: str, text: str, before: int):
    """把一个表达式解析成字符串常量：字符串字面量 / 常量名 / 简单 let 绑定。"""
    expr = expr.strip()
    m = re.fullmatch(r'"((?:[^"\\]|\\.)*)"', expr)
    if m:
        return m.group(1)
    if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", expr):
        val = _find_const(text, expr)
        if val is not None:
            return val
        return _find_binding(text, expr, before)
    return None


def resolve_route_path(text: str, expr: str, before: int):
    """把 .route() 首参表达式解析成真实 URI。

    支持三种现网注册形态：
      ① 字面量 `"/api/..."`；
      ② `&fmt("rel")`（bbs，前缀由 crate 内 `API_BASE` 常量 + normalize_legacy_path 拼）；
      ③ `format!("{p}/...")`（query，前缀由局部 `let p = "..."` 绑定）。
    剩下解析不了的结构（运行时拼路径等）返回 None，由 main 点名失败，不猜。
    """
    expr = expr.strip()
    m = re.fullmatch(r'"((?:[^"\\]|\\.)*)"', expr)
    if m:
        return m.group(1)

    m = re.fullmatch(r'&?\s*fmt\s*\(\s*"((?:[^"\\]|\\.)*)"\s*\)', expr)
    if m:
        base = _find_const(text, "API_BASE")
        if base is None:
            return None
        return "/" + _normalize_legacy_path(base, m.group(1))

    m = re.fullmatch(
        r'&?\s*format!\s*\(\s*"((?:[^"\\]|\\.)*)"\s*(?:,\s*(.*?))?\s*\)\s*(?:\.(?:as_str|to_string)\s*\(\s*\))?\s*',
        expr,
        re.S,
    )
    if m:
        args = _split_top_level_args(m.group(2)) if m.group(2) else []
        return _resolve_format_template(m.group(1), args, text, before)

    return _resolve_value(expr, text, before)


def _parse_handler_chain(expr: str):
    """解析 handler 表达式链上的全部注册方法（get(h).put(h2)... 与闭包 handler）。"""
    out = []
    m = METHOD_RE.match(expr)
    while m:
        method = m.group(1).upper()
        open_idx = m.end() - 1
        close = _match_paren(expr, open_idx)
        if close == -1:
            break
        inner = expr[open_idx + 1:close].strip()
        hm = re.match(r"([A-Za-z_][\w:]*)", inner)
        handler = hm.group(1) if hm and hm.group(1) not in ("async", "move") else "closure"
        out.append((method, handler))
        nm = METHOD_CHAIN_RE.match(expr, close + 1)
        if not nm:
            break
        m = METHOD_RE.match(expr, nm.start(1))
    return out


def parse_routes_from_file(filepath: Path):
    """从 Rust 源码中提取路由定义，返回 (routes, unresolved)。

    O16 修复：旧实现只认 `.route("字面量")`，漏掉 `&fmt("rel")`（bbs 102 条）、
    `format!("{p}/...")`（query 24 条）与闭包 handler（processplatform 3 条）。
    O15 因此把这些真实路由当成「已删残留」，从 fixture 删掉 129 条，守卫被削弱。
    现在首参按表达式解析；解析不了的单条点名进 unresolved，由 main 失败退出。
    """
    try:
        text = filepath.read_text(encoding="utf-8")
    except Exception:
        return [], []

    routes = []
    unresolved = []
    # 先剥离注释（.route( 与路径串间可能夹 // 说明注释），再压平空白。
    # 同时删除 cfg(test) item，否则测试里注册的 /boom 会被当成产品工具。
    text_flat = re.sub(r'\s+', ' ', strip_rust_comments(strip_cfg_test_blocks(text)))
    # 同长掩码：只用来定位真实注册点，读参数仍用 text_flat（字符串内容完整）。
    masked = _mask_strings(text_flat)

    for m in re.finditer(r"\.route\s*\(", masked):
        start = m.end()
        # 读第一个顶层实参：深度 1 的逗号 / 配平右括号为界，尊重字符串与嵌套括号。
        depth = 1
        i = start
        end = -1
        in_str = False
        esc = False
        while i < len(text_flat):
            c = text_flat[i]
            if in_str:
                if esc:
                    esc = False
                elif c == "\\":
                    esc = True
                elif c == '"':
                    in_str = False
            else:
                if c == '"':
                    in_str = True
                elif c in "([{":
                    depth += 1
                elif c in ")]}":
                    depth -= 1
                    if depth == 0:
                        end = i
                        break
                elif c == "," and depth == 1:
                    end = i
                    break
            i += 1
        if end == -1:
            continue
        path_expr = text_flat[start:end]
        path = resolve_route_path(text_flat, path_expr, m.start())
        if path is None:
            unresolved.append((filepath, path_expr.strip()))
            continue

        # handler 表达式：从逗号后到本 route 调用的配平右括号。
        j = end + 1
        depth = 1
        in_str = False
        esc = False
        while j < len(text_flat):
            c = text_flat[j]
            if in_str:
                if esc:
                    esc = False
                elif c == "\\":
                    esc = True
                elif c == '"':
                    in_str = False
            else:
                if c == '"':
                    in_str = True
                elif c == "(":
                    depth += 1
                elif c == ")":
                    depth -= 1
                    if depth == 0:
                        break
            j += 1
        for method, handler in _parse_handler_chain(text_flat[end + 1:j]):
            routes.append((path, method, handler))
    return routes, unresolved


def extract_crate_name(filepath: Path) -> str:
    """从文件路径提取 crate 名称。"""
    parts = filepath.parts
    # crates/<crate_name>/src/...
    for i, part in enumerate(parts):
        if part == "crates" and i + 1 < len(parts):
            return parts[i + 1]
    return "unknown"

def path_to_tool_name(crate: str, path: str, method: str) -> str:
    """将路由路径转换为 MCP 工具名称。
    使用完整路径生成唯一名称，避免多 crate 共享同一路径前缀时命名冲突。
    工具名格式：legacy_{crate}_{path_snake_case}
    """
    p = path.replace("/api/", "")
    parts = p.split("/")
    tool_parts = []
    for part in parts:
        param_match = re.match(r"\{(.+?)\}", part)
        if param_match:
            tool_parts.append(param_match.group(1).lower())
        else:
            s = re.sub(r'([A-Z])', r'_\1', part).lower().strip("_")
            if s and (not tool_parts or tool_parts[-1] != s):
                tool_parts.append(s)
    return f"legacy_{crate}_{'_'.join(tool_parts)}"

def extract_path_params(path: str) -> list:
    """从路径提取路径参数名称。"""
    return re.findall(r"\{(.+?)\}", path)

def infer_body_params(method: str, path: str) -> list:
    """根据 HTTP method 推断 body 参数。"""
    if method in ("GET", "HEAD"):
        return []
    # POST/PUT/DELETE 通常有 body
    # 简单启发式：根据路径段推断常见参数名
    params = []
    path_lower = path.lower()
    if "password" in path_lower:
        params.append("password")
    if "credential" in path_lower:
        params.append("credential")
    if "name" in path_lower:
        params.append("name")
    if "token" in path_lower:
        params.append("token")
    return params

def main():
    all_routes = []
    unresolved = []

    # 扫描所有 crate 的 src 目录
    for crate_dir in sorted(CRATES_DIR.iterdir()):
        if not crate_dir.is_dir():
            continue
        src_dir = crate_dir / "src"
        if not src_dir.is_dir():
            continue

        crate_name = crate_dir.name
        for rs_file in sorted(src_dir.rglob("*.rs")):
            if is_test_source(rs_file):
                continue
            routes, bad = parse_routes_from_file(rs_file)
            for path, method, handler in routes:
                all_routes.append({
                    "crate": crate_name,
                    "path": path,
                    "method": method,
                    "handler": handler,
                })
            for filepath, expr in bad:
                unresolved.append((filepath, expr))

    if unresolved:
        # 结构上无法静态解析的注册方式必须点名失败，绝不静默丢弃或用 extras 掩盖。
        print("无法静态解析的 .route() 首参：", file=sys.stderr)
        for filepath, expr in unresolved:
            rel = filepath.relative_to(ROOT).as_posix()
            print(f"  {rel}: {expr}", file=sys.stderr)
        return 2

    # 去重
    seen = set()
    unique_routes = []
    for r in all_routes:
        key = (r["crate"], r["path"], r["method"])
        if key not in seen:
            seen.add(key)
            unique_routes.append(r)

    # 生成 Rust 代码
    lines = []
    lines.append("// Copyright (C) 2026 annsshadow")
    lines.append("// SPDX-License-Identifier: AGPL-3.0-or-later")
    lines.append("// AUTO-GENERATED by scripts/gen_mcp_tools.py — DO NOT EDIT")
    lines.append("// This file defines GENERATED_ROUTE_DEFS as a Vec<RouteDef>")
    lines.append("")
    lines.append("/// 自动生成的 MCP 工具路由定义（Vec 版本，避免宏依赖）")
    lines.append("pub static GENERATED_ROUTE_DEFS: &[RouteDef] = &[")

    for r in sorted(unique_routes, key=lambda x: (x["crate"], x["method"], x["path"])):
        tool_name = path_to_tool_name(r["crate"], r["path"], r["method"])
        path_params = extract_path_params(r["path"])
        body_params = infer_body_params(r["method"], r["path"])
        http_method = r["method"].capitalize()
        requires_auth = not is_public_path(r["path"])

        lines.append(f'    RouteDef {{')
        lines.append(f'        tool_name: "{tool_name}",')
        lines.append(f'        method: HttpMethod::{http_method},')
        lines.append(f'        path: "{r["path"]}",')
        lines.append(f'        description: "{r["handler"]} handler",')
        lines.append(f'        path_params: &[{", ".join(f'"{p}"' for p in path_params)}],')
        lines.append(f'        body_params: &[{", ".join(f'"{p}"' for p in body_params)}],')
        lines.append(f'    }},')

    lines.append("];")
    lines.append("")
    lines.append(f"/// 生成的工具总数")
    lines.append(f"pub const GENERATED_TOOL_COUNT: usize = {len(unique_routes)};")

    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT_FILE, "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(lines))

    write_route_methods_fixture(unique_routes)
    write_registered_routes_fixture(unique_routes)
    print(f"Generated {len(unique_routes)} MCP tool definitions -> {OUTPUT_FILE}")
    print(f"Generated route-methods fixture -> {FIXTURE_FILE}")
    print(f"Generated registered-routes fixture -> {REGISTERED_FIXTURE_FILE}")


def write_registered_routes_fixture(routes):
    """输出 desktop-endpoints 守卫用的路由快照（原始路径，保留 {} 参数名）。"""
    paths = sorted({r["path"] for r in routes} | set(EXTRA_FIXTURE_ROUTES))
    lines = [
        "// Copyright (C) 2026 annsshadow",
        "// SPDX-License-Identifier: AGPL-3.0-or-later",
        "// AUTO-GENERATED by oa4rust/backend/scripts/gen_mcp_tools.py — DO NOT EDIT",
        "// 后端 .route() 注册路径快照（含 API_BASE/fmt、format! 前缀还原）。desktop 端点守卫用。",
        "",
        "export const REGISTERED_BACKEND_ROUTES: string[] = [",
    ]
    for p in paths:
        lines.append(f"  '{p}',")
    lines.append("]")
    lines.append("")
    REGISTERED_FIXTURE_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(REGISTERED_FIXTURE_FILE, "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(lines))


def _biome_record_entry(path: str, methods) -> list:
    """按 Biome formatter 规则（lineWidth=120）输出一行 `Record` 成员。

    前端 CI 的 Biome format 会对单行超宽的数组强制换行；之前生成器固定输出
    `'p': ['GET'],`，每次重跑都会让 frontend-qa 再红一次，逼人手工改产物——
    正是 O16 要消灭的坏习惯。这里直接生成 Biome 的规范形态，避免引入 Node
    依赖（generated-artifacts 门禁在纯 Python 环境运行）。

    规则实测（oa4rust/biome.json：indent 2、lineWidth 120、single quote）：
      ① 单行 ≤120 → 单行；
      ② 否则若 `  'key': [` ≤120 → 数组换行、元素逐行；
      ③ 否则 key 独占一行，数组整体挪到下一行（4 空格缩进）。
    """
    elems = [f"'{m}'" for m in methods]
    single = f"  '{path}': [{', '.join(elems)}],"
    if len(single) <= 120:
        return [single]
    key_prefix = f"  '{path}':"
    if len(key_prefix) + 2 <= 120:
        return [f"{key_prefix} [", *[f"    {e}," for e in elems], "  ],"]
    arr_single = f"    [{', '.join(elems)}],"
    if len(arr_single) <= 120:
        return [key_prefix, arr_single]
    return [key_prefix, "    [", *[f"      {e}," for e in elems], "    ],"]


def write_route_methods_fixture(routes):
    """与 generated_routes.rs 同源输出前端契约 fixture（path 归一化 → 方法集）。"""
    by_path = {}
    for r in routes:
        p = re.sub(r"\$\{[^}]*\}", "{}", r["path"])
        p = re.sub(r"\{[^}]*\}", "{}", p).rstrip("/").lower()
        by_path.setdefault(p, set()).add(r["method"].upper())
    for p, methods in EXTRA_FIXTURE_ROUTES.items():
        by_path.setdefault(p, set()).update(methods)

    lines = [
        "// Copyright (C) 2026 annsshadow",
        "// SPDX-License-Identifier: AGPL-3.0-or-later",
        "// AUTO-GENERATED by oa4rust/backend/scripts/gen_mcp_tools.py — DO NOT EDIT",
        "// path(归一 {} 占位、小写、去尾斜杠) -> 后端注册的 HTTP 方法集。前端方法契约守卫用。",
        "",
        "export const BACKEND_ROUTE_METHODS: Record<string, string[]> = {",
    ]
    for p in sorted(by_path):
        lines.extend(_biome_record_entry(p, sorted(by_path[p])))
    lines.append("}")
    lines.append("")
    FIXTURE_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(FIXTURE_FILE, "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(lines))

if __name__ == "__main__":
    main()
