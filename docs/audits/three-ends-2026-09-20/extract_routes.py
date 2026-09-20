#!/usr/bin/env python3
"""提取 oa4rust 后端 axum 路由 (method, path)，排除 #[cfg(test)] 块。"""
import json
import os
import re
import sys

def find_repo_root(start):
    """向上查找同时包含 oa4rust 与 oa4rust-web 的目录，使脚本可任意位置运行。"""
    cur = os.path.abspath(start)
    while True:
        if os.path.isdir(os.path.join(cur, "oa4rust")) and os.path.isdir(os.path.join(cur, "oa4rust-web")):
            return cur
        parent = os.path.dirname(cur)
        if parent == cur:
            raise SystemExit("未找到仓库根（需同时存在 oa4rust 与 oa4rust-web）")
        cur = parent


ROOT = os.path.join(find_repo_root(os.path.dirname(os.path.abspath(__file__))), "oa4rust")

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


def extract(path):
    src = open(path, encoding="utf-8", errors="replace").read()
    masked = strip_rust_strings(src)
    skips = cfg_test_ranges(masked)

    def skipped(idx):
        return any(a <= idx < b for a, b in skips)

    # 某些 crate 用 `&fmt("subject/xxx")` 注册路由，前缀由 API_BASE 常量在运行时拼接。
    # 若不还原前缀，提取出的路径会缺失 /api/... 段，导致与前端对账时产生假 404。
    consts = {}
    for cm in re.finditer(r'pub\s+const\s+([A-Z_][A-Z0-9_]*)\s*:\s*&str\s*=\s*"([^"]*)"', src):
        consts[cm.group(1)] = cm.group(2)
    base_prefix = ""
    for name, val in consts.items():
        if "BASE" in name and val.startswith("/"):
            base_prefix = val
            break

    found = []
    for m in re.finditer(r"\.route\s*\(", masked):
        if skipped(m.start()):
            continue
        open_idx = m.end() - 1
        close_idx = match_paren(masked, open_idx)
        if close_idx == -1:
            continue
        seg_masked = masked[open_idx : close_idx + 1]
        seg_orig = src[open_idx : close_idx + 1]
        pm = re.search(r'"((?:[^"\\]|\\.)*)"', seg_orig)
        if not pm:
            continue
        route_path = pm.group(1)
        # 无前导斜杠 → 说明前缀由 helper 拼接，用 API_BASE 还原
        if not route_path.startswith("/") and base_prefix:
            route_path = base_prefix.rstrip("/") + "/" + route_path.lstrip("/")
        methods = []
        for mm in METHOD_RE.finditer(seg_masked):
            name = mm.group(1)
            if name not in methods:
                methods.append(name)
        if not methods:
            methods = ["<unknown>"]
        # 捕获每个方法对应的 handler 标识符（取最后一段，如 u2::data_work_create → data_work_create）
        handler_by_method = {}
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
                handler_by_method[name] = hm.group(1).split("::")[-1]
        for name in methods:
            found.append((name.upper(), route_path, handler_by_method.get(name, "")))
    return found


def main():
    results = {}
    for base in (os.path.join(ROOT, "crates"), os.path.join(ROOT, "src")):
        for dirpath, _dirs, files in os.walk(base):
            if os.sep + "target" in dirpath:
                continue
            for f in files:
                if not f.endswith(".rs"):
                    continue
                full = os.path.join(dirpath, f)
                rel = os.path.relpath(full, ROOT).replace("\\", "/")
                # 归属 crate / 顶层
                parts = rel.split("/")
                if parts[0] == "crates":
                    owner = parts[1]
                else:
                    owner = "oa4rust(facade)"
                rows = extract(full)
                if rows:
                    results.setdefault(owner, [])
                    for method, p, handler in rows:
                        results[owner].append(
                            {"method": method, "path": p, "handler": handler, "file": rel}
                        )

    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "backend_routes.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(results, fh, ensure_ascii=False, indent=1)

    total = sum(len(v) for v in results.values())
    uniq = {(r["method"], r["path"]) for v in results.values() for r in v}
    print(f"crates={len(results)}  route-registrations={total}  unique(method,path)={len(uniq)}")
    print("written:", out)


if __name__ == "__main__":
    main()
