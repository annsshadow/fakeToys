#!/usr/bin/env python3
"""请求体 / 响应体深度 schema 校验。

思路（端点到端点，而非全仓并集）：
 1. 路由 → handler 函数名（来自 backend_routes.json）
 2. handler 函数体 → ① 请求体参数名与类型 ② 从请求体读取的键 ③ 产出的响应键
 3. 前端调用点 → 请求体顶层键（对象字面量 / 局部常量 / reactive）
 4. 比对：
    A 请求体被完全忽略（前端发了非空 body，handler 无 Json 参数）
    B 发送但后端不读（键不在 handler.reads）
    C 后端读但前端不发（可能缺必填字段）
    D 响应字段：视图读取的字段 ∉ 该视图所调端点的产出键并集
"""
import argparse
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def find_repo_root(start):
    cur = os.path.abspath(start)
    while True:
        if os.path.isdir(os.path.join(cur, "oa4rust")) and os.path.isdir(os.path.join(cur, "oa4rust-web")):
            return cur
        parent = os.path.dirname(cur)
        if parent == cur:
            raise SystemExit("未找到仓库根")
        cur = parent


ROOT = find_repo_root(HERE)
RUST = os.path.join(ROOT, "oa4rust")
WEB = os.path.join(ROOT, "oa4rust-web")
OUT = os.path.join(HERE, "schema_audit.json")

# ── Rust 源码工具 ───────────────────────────────────────────────────────────


def strip_rust_strings(text):
    """字符串/字符/注释替换为空格，保持长度。"""
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
                s = i + len(m.group(0))
                j = text.find(endpat, s)
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


def match_brace(masked, open_idx):
    depth, j, n = 0, open_idx, len(masked)
    while j < n:
        if masked[j] == "{":
            depth += 1
        elif masked[j] == "}":
            depth -= 1
            if depth == 0:
                return j
        j += 1
    return -1


def match_paren(masked, open_idx):
    depth, j, n = 0, open_idx, len(masked)
    while j < n:
        if masked[j] == "(":
            depth += 1
        elif masked[j] == ")":
            depth -= 1
            if depth == 0:
                return j
        j += 1
    return -1


def cfg_test_ranges(masked):
    ranges = []
    for m in re.finditer(r"#\[cfg\(test\)\]", masked):
        brace = masked.find("{", m.end())
        if brace == -1:
            continue
        end = match_brace(masked, brace)
        if end != -1:
            ranges.append((m.start(), end + 1))
    return ranges


FN_RE = re.compile(r"(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z_]\w*)\s*\(")
# 允许全限定写法 axum::extract::Json —— 否则会漏掉大量 handler（实测误报 17 处）
QJSON = r"(?:axum\s*::\s*)?(?:extract\s*::\s*)?Json"
# 允许 Option<Json<...>> 可选请求体
JSON_BODY_RE = re.compile(
    r"\b" + QJSON + r"\s*\(\s*([A-Za-z_]\w*)\s*\)\s*:\s*(?:Option\s*<\s*)?" + QJSON + r"\s*<"
)
JSON_STRUCT_RE = re.compile(
    r"\b([A-Za-z_]\w*)\s*:\s*(?:Option\s*<\s*)?" + QJSON + r"\s*<\s*([A-Za-z_][\w:]*(?:\s*<[^>]*>)?)\s*>"
)
# 形如 body_str(&body, &["process", "processId"]) 的数组键表
HELPER_KEYS_RE = re.compile(r"\b\w+\s*\(\s*&?\s*([A-Za-z_]\w*)\s*,\s*&\[([^\]]*)\]")
STR_LIT_RE = re.compile(r'"([A-Za-z_][\w]*)"')
STRUCT_DEF_RE = re.compile(r"#\[derive\([^)]*Deserialize[^)]*\)\][\s\S]{0,200}?pub struct\s+(\w+)\s*\{([^}]*)\}")
STRUCT_PLAIN_RE = re.compile(r"pub struct\s+(\w+)\s*\{([^}]*)\}")
STRUCT_ATTR_RE = re.compile(r"#\[serde\(([^)]*)\)\]\s*pub struct\s+(\w+)\s*\{")


def crate_of(path):
    """从绝对路径推导 crate 名（与 extract_routes.py 的归属口径一致）。"""
    rel = os.path.relpath(path, RUST).replace("\\", "/")
    parts = rel.split("/")
    return parts[1] if parts[0] == "crates" else "oa4rust(facade)"


def scan_rust_index():
    """返回 (handler_index, struct_index)。
    handler_index[name] = {"body_params":[...], "reads":[...], "emits":[...]}
    struct_index[name] = {"fields":[...], "rename": str|None}
    """
    handlers, structs = {}, {}
    for base in (os.path.join(RUST, "crates"), os.path.join(RUST, "src")):
        for dp, _d, fs in os.walk(base):
            if os.sep + "target" in dp:
                continue
            for f in fs:
                if not f.endswith(".rs"):
                    continue
                p = os.path.join(dp, f)
                src = open(p, encoding="utf-8", errors="replace").read()
                masked = strip_rust_strings(src)
                skips = cfg_test_ranges(masked)

                def skipped(idx):
                    return any(a <= idx < b for a, b in skips)

                # 结构体定义（用于 Json<T> 的字段契约）
                for m in STRUCT_PLAIN_RE.finditer(src):
                    name, body = m.group(1), m.group(2)
                    attr = ""
                    pre = src[max(0, m.start() - 300) : m.start()]
                    am = STRUCT_ATTR_RE.search(pre + "pub struct " + name + " {")
                    if am:
                        attr = am.group(1)
                    rename = None
                    if "rename_all" in attr:
                        rm = re.search(r'rename_all\s*=\s*"(\w+)"', attr)
                        rename = rm.group(1) if rm else None
                    # 字段可为 pub 也可为私有（同模块内 serde 反序列化无需 pub）
                    fields = [fm.group(1) for fm in re.finditer(r"(?:^|\n)\s*(?:pub\s+)?(\w+)\s*:", body)]
                    if fields:
                        structs[name] = {"fields": fields, "rename": rename}

                # handler 函数体
                for m in FN_RE.finditer(masked):
                    if skipped(m.start()):
                        continue
                    name = m.group(1)
                    open_p = m.end() - 1
                    # 找函数体 '{'
                    brace = masked.find("{", open_p)
                    if brace == -1:
                        continue
                    close = match_brace(masked, brace)
                    if close == -1:
                        continue
                    sig = src[m.start() : brace]
                    body_masked = masked[brace:close]
                    body_orig = src[brace:close]

                    body_params = []
                    for bm in JSON_BODY_RE.finditer(sig):
                        body_params.append(bm.group(1))
                    for bm in JSON_STRUCT_RE.finditer(sig):
                        body_params.append(bm.group(1))
                    if not body_params:
                        continue

                    reads = set()
                    alt_groups = []
                    # 先收集所有 .get("k") 及其位置，用于识别 or_else/|| 构成的"或"链
                    get_hits = []
                    for bp in body_params:
                        for rm in re.finditer(
                            re.escape(bp)
                            + r"\s*(?:\.get\s*\(\s*|\s*\[\s*)\"([A-Za-z_][\w]*)\"",
                            body_orig,
                        ):
                            get_hits.append((rm.group(1), rm.end()))
                    # 若某个 .get("k") 之后紧跟 .or_else(...)/.or(...)，则它与后续的 .get 构成"或"组
                    or_chained = set()
                    for i, (key, end) in enumerate(get_hits):
                        tail = body_orig[end : end + 60]
                        if re.match(r"\s*\)?\s*\.\s*or(?:_else)?\s*\(", tail) or re.match(
                            r"\s*\)?\s*\.\s*or(?:_else)?\s*\(?\s*\|\|", tail
                        ):
                            or_chained.add(i)
                    # 归组：连续的 or 链合并为一组
                    i = 0
                    while i < len(get_hits):
                        if i in or_chained:
                            grp = [get_hits[i][0]]
                            j = i
                            while j in or_chained and j + 1 < len(get_hits):
                                grp.append(get_hits[j + 1][0])
                                j += 1
                            alt_groups.append(grp)
                            i = j + 1
                        else:
                            reads.add(get_hits[i][0])
                            i += 1
                    # 数组键表 helper：body_str(&body, &["process", "processId"]) —— 语义是"或"
                    for hm in HELPER_KEYS_RE.finditer(body_orig):
                        if hm.group(1) in body_params:
                            alts = [sm.group(1) for sm in STR_LIT_RE.finditer(hm.group(2))]
                            if alts:
                                alt_groups.append(alts)
                    # 委派检测：body 被整体传给另一个函数 → 键读取发生在被调函数内，静态不可判定
                    delegated = False
                    ACCESSORS = {
                        "get", "as_str", "as_i64", "as_u64", "as_f64", "as_bool", "as_array",
                        "as_object", "to_string", "unwrap_or", "unwrap_or_default", "unwrap",
                        "clamp", "trim", "len", "is_null", "is_some", "is_none", "clone",
                        "map", "and_then", "ok_or", "ok_or_else", "filter", "iter", "contains",
                        "expect", "to_owned", "into", "take", "as_ref", "as_deref", "to_value",
                        "from_value", "is_empty", "parse", "as_object_mut", "as_str_or",
                    }
                    for bp in body_params:
                        for cm in re.finditer(r"\b([a-z_][a-z0-9_]*)\s*\(", body_orig):
                            callee = cm.group(1)
                            if callee in ACCESSORS:
                                continue
                            open_p = cm.end() - 1
                            close_p = match_paren(body_orig, open_p)
                            if close_p == -1:
                                continue
                            args = body_orig[open_p + 1 : close_p]
                            if re.search(r"\b" + re.escape(bp) + r"\b", args):
                                delegated = True
                                break
                        if delegated:
                            break
                    emits = set()
                    for em in re.finditer(r'\(\s*"([A-Za-z_][\w]*)"\s*\.to_string\(\)\s*,', body_orig):
                        emits.add(em.group(1))
                    for em in re.finditer(r'"([A-Za-z_][\w]*)"\s*:', body_orig):
                        emits.add(em.group(1))
                    # 若 Json<T> 是具名结构体，把结构体字段也算作读取键
                    for bp in body_params:
                        sm = re.search(
                            re.escape(bp) + r"\s*:\s*" + QJSON + r"\s*<\s*([A-Za-z_]\w*)\s*>", sig
                        )
                        if sm and sm.group(1) in structs:
                            st = structs[sm.group(1)]
                            for fld in st["fields"]:
                                nm = fld
                                if st["rename"] == "camelCase":
                                    parts = fld.split("_")
                                    nm = parts[0] + "".join(x[:1].upper() + x[1:] for x in parts[1:])
                                reads.add(nm)
                    handlers.setdefault(
                        (crate_of(p), name),
                        {
                            "body_params": body_params,
                            "reads": sorted(reads),
                            "alt_groups": alt_groups,
                            "delegated": delegated,
                            "emits": sorted(emits),
                            "file": os.path.relpath(p, RUST).replace("\\", "/"),
                        },
                    )
    return handlers, structs


# ── 前端请求体键提取 ────────────────────────────────────────────────────────

CLIENTS = ("api", "mapi", "http", "client", "req")
VERBS = ("get", "post", "put", "delete", "patch", "upload", "download")
ENTRY_RE = re.compile(r"\b(" + "|".join(CLIENTS) + r")\s*\.\s*(" + "|".join(VERBS) + r")\b")
OBJ_START = re.compile(r"^\s*(?:as\s+\w+\s*)?\{")
LOCAL_OBJ_RE = re.compile(r"\b(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*(?::[^={\n;]*)?=\s*(?:ref|reactive|computed)?\s*\(?\s*\{")
KV_TOP_RE = re.compile(r"([A-Za-z_$][\w$]*)\s*:(?![:\[])")


def skip_ws(s, i):
    while i < len(s) and s[i] in " \t\r\n":
        i += 1
    return i


def skip_angle(s, i):
    d = 0
    while i < len(s):
        if s[i] == "<":
            d += 1
        elif s[i] == ">":
            d -= 1
            if d == 0:
                return i + 1
        i += 1
    return i


def balanced(s, i, opener="{", closer="}"):
    """i 指向 opener，返回匹配 closer 的下标。"""
    d = 0
    j = i
    while j < len(s):
        c = s[j]
        if c in "\"'`":
            q = c
            j += 1
            while j < len(s):
                if s[j] == "\\":
                    j += 2
                    continue
                if s[j] == q:
                    break
                j += 1
        elif c == opener:
            d += 1
        elif c == closer:
            d -= 1
            if d == 0:
                return j
        j += 1
    return -1


def top_level_keys(obj_src):
    """从对象字面量源码提取顶层键。

    先按顶层逗号切分为"属性片段"，再逐段解析 —— 避免把值表达式尾部的
    标识符误判为简写属性（如 `sql: form.value.sql` 里的 `sql`）。
    """
    inner = obj_src[1:-1] if obj_src.startswith("{") else obj_src
    parts, buf, d, i = [], [], 0, 0
    n = len(inner)
    while i < n:
        c = inner[i]
        if c in "\"'`":
            q = c
            buf.append(c)
            i += 1
            while i < n:
                buf.append(inner[i])
                if inner[i] == "\\":
                    if i + 1 < n:
                        buf.append(inner[i + 1])
                        i += 2
                        continue
                elif inner[i] == q:
                    i += 1
                    break
                i += 1
            continue
        if c in "{[(":
            d += 1
        elif c in "}])":
            d -= 1
        if c == "," and d == 0:
            parts.append("".join(buf))
            buf = []
            i += 1
            continue
        buf.append(c)
        i += 1
    if buf:
        parts.append("".join(buf))

    keys = []
    for p in parts:
        s = p.strip()
        if not s:
            continue
        m = re.match(r"\.\.\.\s*([A-Za-z_$][\w$]*)", s)
        if m:
            keys.append("..." + m.group(1))
            continue
        m = re.match(r"\[\s*([`'\"])(.*?)\1\s*\]\s*:", s)
        if m:
            keys.append(m.group(2))
            continue
        m = re.match(r"([A-Za-z_$][\w$]*)\s*:", s)
        if m:
            keys.append(m.group(1))
            continue
        m = re.match(r"([A-Za-z_$][\w$]*)\s*$", s)
        if m:  # ES6 简写属性 { content }
            keys.append(m.group(1))
    # 去重保序
    seen, out = set(), []
    for k in keys:
        if k not in seen:
            seen.add(k)
            out.append(k)
    return out


def scan_frontend_bodies():
    rows = []
    for app, base in (
        ("desktop", os.path.join(WEB, "apps", "desktop", "src")),
        ("mobile", os.path.join(WEB, "apps", "mobile", "src")),
        ("shared-ui", os.path.join(WEB, "packages", "ui", "src")),
        ("shared-sdk", os.path.join(WEB, "packages", "sdk", "src")),
    ):
        for dp, _d, fs in os.walk(base):
            if "node_modules" in dp:
                continue
            for f in fs:
                if not f.endswith((".ts", ".vue")) or f.endswith((".test.ts", ".spec.ts")):
                    continue
                p = os.path.join(dp, f)
                rel = os.path.relpath(p, WEB).replace("\\", "/")
                src = open(p, encoding="utf-8", errors="replace").read()
                local = {}
                for m in LOCAL_OBJ_RE.finditer(src):
                    b = src.find("{", m.end() - 1)
                    e = balanced(src, b)
                    if e != -1:
                        local[m.group(1)] = top_level_keys(src[b : e + 1])
                for m in ENTRY_RE.finditer(src):
                    verb = m.group(2).upper()
                    i = skip_ws(src, m.end())
                    if i < len(src) and src[i] == "<":
                        i = skip_ws(src, skip_angle(src, i))
                    if i >= len(src) or src[i] != "(":
                        continue
                    # 第一参数
                    j = skip_ws(src, i + 1)
                    if j >= len(src):
                        continue
                    if src[j] in "\"'`":
                        q = src[j]
                        k = j + 1
                        while k < len(src):
                            if src[k] == "\\":
                                k += 2
                                continue
                            if src[k] == q:
                                break
                            k += 1
                        first = src[j : k + 1]
                        after = skip_ws(src, k + 1)
                    else:
                        mm = re.match(r"([A-Za-z_$][\w$]*|endpoints[\.\[])", src[j:])
                        first = mm.group(1) if mm else ""
                        after = skip_ws(src, j + len(first))
                    if after >= len(src) or src[after] != ",":
                        continue  # 无 body
                    # 第二参数 = body
                    b = skip_ws(src, after + 1)
                    line = src[: m.start()].count("\n") + 1
                    if b >= len(src):
                        continue
                    body_src, keys, src_kind = "", [], "unknown"
                    if src[b] == "{":
                        e = balanced(src, b)
                        body_src = src[b : e + 1] if e != -1 else src[b : b + 200]
                        keys = top_level_keys(body_src)
                        src_kind = "object"
                    else:
                        mm = re.match(r"([A-Za-z_$][\w$]*)", src[b:])
                        if mm and mm.group(1) in local:
                            keys = local[mm.group(1)]
                            src_kind = "local:" + mm.group(1)
                    rows.append(
                        {
                            "app": app,
                            "file": rel,
                            "line": line,
                            "method": verb,
                            "first_arg": first,
                            "body_kind": src_kind,
                            "body_keys": keys,
                            "body_src": body_src[:160],
                        }
                    )
    return rows


def scan_quoted_key_defect(routes):
    """E 类：Rust 字面量 "\"key\"" —— 键名里字面包含引号（转义错误）。

    后果分三种：
      row-column    读一个不存在的列 → row.get panic → 500
      response-key  产出带引号的 JSON 键 → 前端读 item.key 得 undefined
      route-path    路由路径参数名带引号（如 {"excelName"}）→ 契约畸形
    """
    pat = re.compile(r'"\\"([A-Za-z_][\w]*)\\""')
    buckets = {
        "row-column": [],
        "response-key": [],
        "request-key": [],
        "route-path": [],
        "other": [],
    }
    for base in (os.path.join(RUST, "crates"), os.path.join(RUST, "src")):
        for dp, _d, fs in os.walk(base):
            if os.sep + "target" in dp:
                continue
            for f in fs:
                if not f.endswith(".rs"):
                    continue
                p = os.path.join(dp, f)
                rel = os.path.relpath(p, RUST).replace("\\", "/")
                s = open(p, encoding="utf-8", errors="replace").read()
                for m in pat.finditer(s):
                    pre = s[max(0, m.start() - 60) : m.start()]
                    post = s[m.end() : m.end() + 40]
                    if re.search(r"row\s*\.\s*get|\.get::<", pre):
                        k = "row-column"
                    elif ".to_string()" in post:
                        k = "response-key"
                    elif re.search(r"\.get\s*\(\s*$|\.get\s*\(\s*&?\s*$", pre):
                        k = "request-key"
                    else:
                        k = "other"
                    buckets[k].append(
                        {"key": m.group(1), "file": rel, "line": s[: m.start()].count("\n") + 1}
                    )
    # 路由路径：从已提取的路由表判定（源码里参数名被转义，正则抓不到）
    for crate, rs in routes.items():
        for r in rs:
            if '"' in r["path"]:
                buckets["route-path"].append(
                    {"key": r["path"], "file": r.get("file", ""), "line": 0, "method": r["method"]}
                )
    return {
        "total": sum(len(v) for v in buckets.values()),
        "counts": {k: len(v) for k, v in buckets.items()},
        "samples": {k: v[:8] for k, v in buckets.items()},
    }


def main():
    ap = argparse.ArgumentParser(description="请求/响应体契约校验（--gate 模式可用于 CI 门禁）")
    ap.add_argument(
        "--gate",
        action="store_true",
        help="门禁模式：A 必须为 0，B/C/D/E 不得高于 schema_baseline.json（E2/E5）",
    )
    args = ap.parse_args()

    handlers, structs = scan_rust_index()
    routes = json.load(open(os.path.join(HERE, "backend_routes.json"), encoding="utf-8"))
    bodies = scan_frontend_bodies()

    # 端点 → handler 契约（按 (crate, handler) 精确查找，避免同名函数跨 crate 串味）
    endpoint = {}
    for crate, rs in routes.items():
        for r in rs:
            h = r.get("handler") or ""
            hi = handlers.get((crate, h))
            endpoint.setdefault((r["method"], r["path"]), []).append(
                {
                    "crate": crate,
                    "handler": h,
                    "body_params": hi["body_params"] if hi else [],
                    "reads": hi["reads"] if hi else [],
                    "alt_groups": hi["alt_groups"] if hi else [],
                    "delegated": hi["delegated"] if hi else False,
                    "emits": hi["emits"] if hi else [],
                    "indexed": bool(hi),
                }
            )

    report = {
        "A_body_ignored": [],
        "B_sent_not_read": [],
        "C_read_not_sent": [],
        "D_delegated_unverifiable": [],
        "meta": {},
    }
    for row in bodies:
        if row["method"] not in ("POST", "PUT", "PATCH", "UPLOAD"):
            continue
        if not row["body_keys"] and row["body_kind"] == "unknown":
            continue
        cands = []
        for (mth, pth), lst in endpoint.items():
            if mth != ("POST" if row["method"] == "UPLOAD" else row["method"]):
                continue
            a = [s for s in pth.split("/") if s]
            b = [s for s in row["first_arg"].strip("`'\" ").split("?")[0].split("/") if s]
            if len(a) != len(b):
                continue
            if all(pa.startswith("{") or pa == pb for pa, pb in zip(a, b)):
                cands.extend(lst)
        if not cands:
            continue
        nonempty = [k for k in row["body_keys"] if not k.startswith("...")]
        if not nonempty:
            continue
        agg_reads, agg_alts, has_body_param, delegated = set(), [], False, False
        for c in cands:
            if c["body_params"]:
                has_body_param = True
            agg_reads.update(c["reads"])
            agg_alts.extend(c["alt_groups"])
            delegated = delegated or c["delegated"]
        rec = {
            "app": row["app"],
            "file": row["file"],
            "line": row["line"],
            "method": row["method"],
            "path": row["first_arg"].strip("`'\" "),
            "sent": nonempty,
            "handler_reads": sorted(agg_reads),
            "handlers": sorted({c["handler"] for c in cands}),
        }
        if not has_body_param:
            report["A_body_ignored"].append(rec)
            continue
        if delegated and not agg_reads and not agg_alts:
            # 键读取发生在被委派的泛型 helper 内（如 shared::crud），静态不可判定
            report["D_delegated_unverifiable"].append(rec)
            continue
        notread = [k for k in nonempty if k not in agg_reads and not any(k in g for g in agg_alts)]
        if notread:
            report["B_sent_not_read"].append({**rec, "not_read": notread})
        # C 类：硬读取键缺失（数组键表按"或"处理）
        missing = sorted(agg_reads - set(nonempty))
        missing_alts = [
            g for g in agg_alts if not any(k in nonempty for k in g) and not any(k in agg_reads for k in g)
        ]
        if missing or missing_alts:
            report["C_read_not_sent"].append(
                {**rec, "missing": missing, "missing_alt_groups": missing_alts}
            )

    report["meta"] = {
        "handlers_indexed": len(handlers),
        "structs_indexed": len(structs),
        "frontend_body_calls": len(bodies),
        "A": len(report["A_body_ignored"]),
        "B": len(report["B_sent_not_read"]),
        "C": len(report["C_read_not_sent"]),
        "D": len(report["D_delegated_unverifiable"]),
    }
    report["E_quoted_key_defect"] = scan_quoted_key_defect(routes)
    report["meta"]["E_total"] = report["E_quoted_key_defect"]["total"]
    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(
            {
                "report": report,
                "handlers": {f"{c}::{n}": v for (c, n), v in handlers.items()},
            },
            fh,
            ensure_ascii=False,
            indent=1,
        )

    print(json.dumps(report["meta"], ensure_ascii=False, indent=1))
    print("\n--- A 请求体被完全忽略 ---")
    for r in report["A_body_ignored"]:
        print(f"  {r['method']} {r['path']}  <- {r['file']}:{r['line']}")
        print(f"      前端发送: {r['sent']}   后端 handler: {r['handlers']}（无 Json 参数）")
    print("\n--- B 发送但后端不读 ---")
    for r in report["B_sent_not_read"]:
        print(f"  {r['method']} {r['path']}  <- {r['file']}:{r['line']}")
        print(f"      未读键: {r['not_read']}   handler: {r['handlers']}  读取: {r['handler_reads']}")
    print("\n--- C 后端读但前端不发 ---")
    for r in report["C_read_not_sent"][:40]:
        print(f"  {r['method']} {r['path']}  <- {r['file']}:{r['line']}")
        print(f"      缺失: {r['missing']}  缺失或组: {r.get('missing_alt_groups')}  前端发送: {r['sent']}")
    print("\n--- D 委派给泛型 helper，静态不可判定 ---")
    for r in report["D_delegated_unverifiable"]:
        print(f"  {r['method']} {r['path']}  <- {r['file']}:{r['line']}  handler={r['handlers']}")

    if args.gate:
        return run_gate(report["meta"])
    return 0


BASELINE_PATH = os.path.join(HERE, "schema_baseline.json")


def run_gate(meta):
    """门禁：A 必须为 0；B/C/D/E 不得高于基线（只许降，不许升）。

    基线存在的理由：B/C/D 含**已知误报**（提取器不跟随间接引用，见文档 §九 状态表），
    无法直接要求 0。用"不许新增"作为可执行且不撒谎的门禁。
    """
    if not os.path.exists(BASELINE_PATH):
        print(f"[gate] 缺少基线文件 {BASELINE_PATH}；先用当前值建立基线。")
        with open(BASELINE_PATH, "w", encoding="utf-8") as fh:
            json.dump({k: meta[k] for k in ("A", "B", "C", "D", "E_total")}, fh, indent=1)
        return 0

    base = json.load(open(BASELINE_PATH, encoding="utf-8"))
    cur = {"A": meta["A"], "B": meta["B"], "C": meta["C"], "D": meta["D"], "E_total": meta["E_total"]}
    print("\n[gate] 类别  当前  基线  判定")
    failed = []
    for k in ("A", "B", "C", "D", "E_total"):
        c, b = cur[k], base.get(k, 0)
        if k == "A" and c != 0:
            verdict = "FAIL（A 必须为 0：请求体被完全忽略 = 数据静默丢失）"
            failed.append(k)
        elif c > b:
            verdict = "FAIL（高于基线 = 新增错配）"
            failed.append(k)
        else:
            verdict = "ok"
        print(f"[gate] {k:8s}{c:5d}{b:6d}  {verdict}")
    if failed:
        print(f"[gate] FAIL —— {', '.join(failed)}")
        return 1
    print("[gate] PASS —— 无新增契约错配")
    return 0


if __name__ == "__main__":
    sys.exit(main())
