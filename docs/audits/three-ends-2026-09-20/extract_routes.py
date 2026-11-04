#!/usr/bin/env python3
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""提取 oa4rust 后端 axum 路由 (method, path)，排除 #[cfg(test)] 块。"""
import json
import os
import re
import sys

def find_repo_root(start):
    """向上查找含 oa4rust 三端（backend/frontend/mobile）的仓库根，使脚本可任意位置运行。"""
    cur = os.path.abspath(start)
    while True:
        if all(os.path.isdir(os.path.join(cur, "oa4rust", end)) for end in ("backend", "frontend", "mobile")):
            return cur
        parent = os.path.dirname(cur)
        if parent == cur:
            raise SystemExit("未找到仓库根（oa4rust/ 下需存在 backend、frontend、mobile）")
        cur = parent


ROOT = os.path.join(find_repo_root(os.path.dirname(os.path.abspath(__file__))), "oa4rust", "backend")

METHODS = ("get", "post", "put", "delete", "patch", "head", "options", "any", "trace")
# 注意：不能用 (?<![A-Za-z0-9_.]) —— axum 的链式写法 `.post(h).put(h)` 中
# 方法名前面正是 '.'，排除 '.' 会漏掉全部链式注册（实测漏了 put 变体）。
METHOD_RE = re.compile(r"(?<![A-Za-z0-9_])(" + "|".join(METHODS) + r")\s*\(")


def strip_rust_strings(text):
    """把字符串/字符/注释内容替换为空格，保持长度不变（便于用同一偏移回查原文）。"""
    out = list(text)
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if c == "/" and i + 1 < n and text[i + 1] == "/":
            j = text.find("\n", i)
            j = n if j == -1 else j
            for k in range(i, j):
                out[k] = " "
            i = j
            continue
        if c == "/" and i + 1 < n and text[i + 1] == "*":
            j = text.find("*/", i + 2)
            j = n if j == -1 else j + 2
            for k in range(i, j):
                if text[k] != "\n":
                    out[k] = " "
            i = j
            continue
        if c == "r" and i + 1 < n and text[i + 1] in '"#':
            m = re.match(r'r(#*)"', text[i:])
            if m:
                endpat = '"' + m.group(1)
                start = i + len(m.group(0))
                j = text.find(endpat, start)
                j = n if j == -1 else j + len(endpat)
                for k in range(i, j):
                    if text[k] != "\n":
                        out[k] = " "
                i = j
                continue
        if c == '"':
            j = i + 1
            while j < n:
                if text[j] == "\\":
                    j += 2
                    continue
                if text[j] == '"':
                    j += 1
                    break
                j += 1
            for k in range(i, min(j, n)):
                if text[k] != "\n":
                    out[k] = " "
            i = j
            continue
        if c == "'":
            m = re.match(r"'(?:\\.|[^\\'])'", text[i:])
            if m:
                j = i + len(m.group(0))
                for k in range(i, j):
                    if text[k] != "\n":
                        out[k] = " "
                i = j
                continue
            i += 1
            continue
        i += 1
    return "".join(out)


def cfg_test_ranges(masked):
    """返回 #[cfg(test)] 后随 { } 块的 (start, end) 区间列表。"""
    ranges = []
    for m in re.finditer(r"#\[cfg\(test\)\]", masked):
        brace = masked.find("{", m.end())
        if brace == -1:
            continue
        depth = 0
        j = brace
        n = len(masked)
        while j < n:
            ch = masked[j]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        ranges.append((m.start(), j + 1))
    return ranges


def match_paren(masked, open_idx):
    depth = 0
    j = open_idx
    n = len(masked)
    while j < n:
        ch = masked[j]
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                return j
        j += 1
    return -1


def read_first_arg(src, masked, open_paren):
    """读 .route( 的第一个顶层实参，返回 (原文表达式, 分隔符下标, 是否结尾)。

    2026-11 O16 修正：旧实现取「整段里第一个字符串字面量」，把
    `format!("{p}/...")` 的局部变量前缀 `p` 当成真实路径段 `/…/{p}/…`，
    污染了 backend_routes.json 与前端 fixture。现在按表达式解析，解不出就点名跳过。
    """
    depth = 1
    i = open_paren + 1
    n = len(masked)
    while i < n:
        c = masked[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
            if depth == 0:
                return src[open_paren + 1 : i], i, True
        elif c == "," and depth == 1:
            return src[open_paren + 1 : i], i, False
        i += 1
    return "", i, False


def split_top_level_args(s):
    args, cur, depth, in_str, esc = [], [], 0, False, False
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


def find_const(src, name):
    m = re.search(
        r'\b(?:pub(?:\(\w+\))?\s+)?(?:const|static)\s+'
        + re.escape(name)
        + r'\s*:\s*&(?:\'[^\']+\s+)?str\s*=\s*"((?:[^"\\]|\\.)*)"',
        src,
    )
    return m.group(1) if m else None


def find_binding(src, name, before):
    pat = re.compile(
        r'\b(?:let|const)\s+' + re.escape(name) + r'\s*(?::\s*&?(?:str|String)\s*)?=\s*"((?:[^"\\]|\\.)*)"'
    )
    val = None
    for m in pat.finditer(src[:before]):
        val = m.group(1)
    return val


def resolve_value(expr, src, before):
    expr = expr.strip()
    m = re.fullmatch(r'"((?:[^"\\]|\\.)*)"', expr)
    if m:
        return m.group(1)
    if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", expr):
        v = find_const(src, expr)
        return v if v is not None else find_binding(src, expr, before)
    return None


def expand_format(tmpl, args, src, before):
    """展开 format!：`{{`/`}}` 是真字面花括号，`{name}`/`{}` 是插值。

    区分口径：只有能解析到 let/const 绑定的占位符才算局部变量前缀；
    解析不出的返回 None（调用方点名跳过），绝不再把 `{p}` 写进路径。
    """
    out, arg_idx, i = [], 0, 0
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
            key = tmpl[i + 1 : j].strip()
            if key == "":
                if arg_idx >= len(args):
                    return None
                val = resolve_value(args[arg_idx], src, before)
                arg_idx += 1
            else:
                val = resolve_value(key, src, before)
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


def normalize_legacy_path(class_path, method_path):
    segs = []
    for seg in f"{class_path}/{method_path}".split("/"):
        if not seg:
            continue
        if segs and segs[-1] == seg:
            continue
        segs.append(seg)
    return "/".join(segs)


def resolve_route_path(expr, src, before):
    expr = expr.strip()
    m = re.fullmatch(r'"((?:[^"\\]|\\.)*)"', expr)
    if m:
        return m.group(1)
    m = re.fullmatch(r'&?\s*fmt\s*\(\s*"((?:[^"\\]|\\.)*)"\s*\)', expr)
    if m:
        base = find_const(src, "API_BASE")
        if base is None:
            return None
        return "/" + normalize_legacy_path(base, m.group(1))
    m = re.fullmatch(
        r'&?\s*format!\s*\(\s*"((?:[^"\\]|\\.)*)"\s*(?:,\s*(.*?))?\s*\)\s*(?:\.(?:as_str|to_string)\s*\(\s*\))?\s*',
        expr,
        re.S,
    )
    if m:
        args = split_top_level_args(m.group(2)) if m.group(2) else []
        return expand_format(m.group(1), args, src, before)
    return resolve_value(expr, src, before)


def extract(path):
    src = open(path, encoding="utf-8", errors="replace").read()
    masked = strip_rust_strings(src)
    skips = cfg_test_ranges(masked)

    def skipped(idx):
        return any(a <= idx < b for a, b in skips)

    # 前缀不再是「文件里第一个名字含 BASE 的常量」：只按表达式实际引用的
    # API_BASE / 局部 let 绑定来解析（resolve_route_path），避免张冠李戴。
    found = []
    unresolved = []
    for m in re.finditer(r"\.route\s*\(", masked):
        if skipped(m.start()):
            continue
        open_idx = m.end() - 1
        close_idx = match_paren(masked, open_idx)
        if close_idx == -1:
            continue
        first_arg, _sep, _closed = read_first_arg(src, masked, open_idx)
        route_path = resolve_route_path(first_arg, src, m.start())
        if route_path is None:
            # 运行时拼路径等结构无法静态解析：点名计数，绝不把 `{p}` 当字面段写进产物。
            unresolved.append(first_arg.strip()[:120])
            continue
        seg_masked = masked[open_idx : close_idx + 1]
        seg_orig = src[open_idx : close_idx + 1]
        methods = []
        for mm in METHOD_RE.finditer(seg_masked):
            name = mm.group(1)
            if name not in methods:
                methods.append(name)
        if not methods:
            methods = ["<unknown>"]
        # 捕获每个方法对应的 handler 标识符（取最后一段，如 u2::data_work_create → data_work_create）
        handler_by_method = {}
        handler_full_by_method = {}
        for mm in re.finditer(
            r"(?<![A-Za-z0-9_])(get|post|put|delete|patch|head|options|any|trace)\s*\(", seg_masked
        ):
            name = mm.group(1)
            if name in handler_by_method:
                continue
            open_p = mm.end() - 1
            close_p = match_paren(seg_masked, open_p)
            if close_p == -1:
                continue
            inner = seg_orig[open_p + 1 : close_p].strip()
            hm = re.search(r"([A-Za-z_][\w:]*)\s*$", inner)
            if hm:
                full = hm.group(1)
                # handler=裸名（兼容下游），handler_q=模块限定名（如 role::update）——
                # 同 crate 内 group/person/role/unit 四模块各有同名 CRUD fn，裸名会
                # 撞名且 os.walk 顺序不定（本地 role.rs 先 vs CI person.rs 先），
                # 只有保留限定名才能让 schema_audit 按 (crate, 模块::fn) 精确消歧、跨平台确定。
                handler_by_method[name] = full.split("::")[-1]
                handler_full_by_method[name] = full
        for name in methods:
            found.append(
                (name.upper(), route_path, handler_by_method.get(name, ""), handler_full_by_method.get(name, ""))
            )
    return found, unresolved


def main():
    results = {}
    unresolved_all = []
    for base in (os.path.join(ROOT, "crates"), os.path.join(ROOT, "src")):
        for dirpath, _dirs, files in os.walk(base):
            if os.sep + "target" in dirpath:
                continue
            for f in files:
                if not f.endswith(".rs"):
                    continue
                # 测试桩文件（tests*/testing*，与 regen_endpoints.py 的 is_test_file 同口径）：
                # shared/src/testing.rs 等含 route-existence 桩注册（如 POST /api/reset "ok"），
                # 非生产路由，混入分母会造成假缺口。
                stem = f[: -len(".rs")]
                if stem.startswith("tests") or stem.startswith("testing"):
                    continue
                full = os.path.join(dirpath, f)
                rel = os.path.relpath(full, ROOT).replace("\\", "/")
                # 归属 crate / 顶层
                parts = rel.split("/")
                if parts[0] == "crates":
                    owner = parts[1]
                else:
                    owner = "oa4rust(facade)"
                rows, unresolved = extract(full)
                for expr in unresolved:
                    unresolved_all.append((rel, expr))
                if rows:
                    results.setdefault(owner, [])
                    for method, p, handler, handler_q in rows:
                        results[owner].append(
                            {
                                "method": method,
                                "path": p,
                                "handler": handler,
                                "handler_q": handler_q,
                                "file": rel,
                            }
                        )

    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "backend_routes.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(results, fh, ensure_ascii=False, indent=1)

    total = sum(len(v) for v in results.values())
    uniq = {(r["method"], r["path"]) for v in results.values() for r in v}
    print(f"crates={len(results)}  route-registrations={total}  unique(method,path)={len(uniq)}")
    if unresolved_all:
        print(f"无法静态解析的首参 {len(unresolved_all)} 处（已点名，未写入产物）：")
        for rel, expr in unresolved_all[:20]:
            print(f"  {rel}: {expr}")
    print("written:", out)


if __name__ == "__main__":
    main()
