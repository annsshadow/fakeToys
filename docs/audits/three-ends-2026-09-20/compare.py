#!/usr/bin/env python3
"""三端 API 对账：前端调用 vs 后端路由。

分类：
  exact    段完全一致（含参数位占位一致）
  param    仅靠"后端参数段吞掉前端字面量"匹配 —— 若该字面量是资源子名(list/export/...)
           则判为 suspicious-shadow（静默误路由风险），否则判为 param-ok（id/flag 等）
  405      路径存在但方法未注册
  404      无任何路径匹配
"""
import argparse
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
KEYWORDS = {
    "list", "all", "manage", "my", "export", "import", "create", "save", "delete",
    "update", "submit", "run", "execute", "do", "tree", "nested", "next", "pre",
    "check", "upload", "download", "search", "query", "stat", "view", "page",
    "script", "table", "form", "process", "get", "add", "remove", "batch", "copy",
    "move", "sort", "enable", "disable", "start", "stop", "reset", "clear",
}
ID_RE = re.compile(r"^(?:[0-9]+|[0-9a-fA-F-]{8,}|[A-Za-z0-9_.-]*[0-9][A-Za-z0-9_.-]*)$")
# 这些参数名是"通用标识位"，被资源子名(list/export/...)顶替即为静默影子误路由
GENERIC_PARAMS = {
    "id", "flag", "person", "job", "value", "name", "key", "code", "unique",
    "tag", "path", "cls", "entity", "work", "task", "type", "label", "info",
}
# 语义化参数名：其取值本就是关键词（如 list/{category} 传 all）
SEMANTIC_PARAMS = {"category", "appid", "folderid", "page", "size", "count", "month", "year", "day", "query", "queryflag", "formid", "docid", "clueid", "model", "platform", "roomid", "person", "unit", "identity", "group", "role"}


def segs(path):
    return [s for s in path.split("/") if s != ""]


def is_param(seg):
    return seg.startswith("{") and seg.endswith("}")


def param_name(seg):
    return seg.strip("{}").lstrip("*")


def match_one(front_segs, back_segs):
    """返回 (matched, used) ；used = [(后端参数名 or None, 被吞掉的前端字面量)]"""
    used = []
    i = j = 0
    while i < len(front_segs) and j < len(back_segs):
        b = back_segs[j]
        if b.startswith("{*"):
            return True, used
        if is_param(b):
            used.append((param_name(b), None if front_segs[i] == "{}" else front_segs[i]))
            i += 1
            j += 1
            continue
        if front_segs[i] == b:
            i += 1
            j += 1
            continue
        if front_segs[i] == "{}":
            i += 1
            j += 1
            continue
        return False, used
    return i == len(front_segs) and j == len(back_segs), used


METHOD_ALIAS = {"UPLOAD": "POST", "DOWNLOAD": "GET"}


def classify(call, backend):
    fs = segs(call["path"])
    if call.get("ambiguous"):
        methods = {"GET", "POST", "PUT", "DELETE"}
    else:
        methods = set(METHOD_ALIAS.get(call["method"], call["method"]).split("|"))
    exact, param_only = None, []
    path_cands, method_cands = [], []
    for crate, routes in backend.items():
        for r in routes:
            bs = segs(r["path"])
            ok, used = match_one(fs, bs)
            if not ok:
                continue
            path_cands.append((crate, r))
            rm = "GET|POST|PUT|DELETE" if r["method"] == "ANY" else r["method"]
            if methods & set(rm.split("|")):
                method_cands.append((crate, r))
                if used and all(u[1] is None for u in used):
                    exact = (crate, r)
                else:
                    param_only.append((crate, r, used))
    if method_cands:
        if exact:
            return "exact", exact, None
        param_only.sort(key=lambda t: len([u for u in t[2] if u[1] is not None]))
        crate, r, used = param_only[0]
        lits = [u for u in used if u[1] is not None]
        suspicious = [
            f"{pn}={lit}"
            for pn, lit in lits
            if pn in GENERIC_PARAMS and pn not in SEMANTIC_PARAMS and not ID_RE.match(lit)
        ]
        if suspicious:
            return "suspicious-shadow", (crate, r), suspicious
        return "param-ok", (crate, r), [f"{pn}={lit}" for pn, lit in lits]
    if path_cands:
        return "405", path_cands[0], sorted({c[1]["method"] for c in path_cands})
    return "404", None, None


def main():
    ap = argparse.ArgumentParser(description="三端 API 对账（--gate 模式可用于 CI 门禁）")
    ap.add_argument(
        "--gate",
        action="store_true",
        help="门禁模式：shadow / 405 / 404 任一 > 0 即退出码 1（E1/E4/E5）",
    )
    args = ap.parse_args()

    backend = json.load(open(os.path.join(HERE, "backend_routes.json"), encoding="utf-8"))
    fe = json.load(open(os.path.join(HERE, "frontend_calls.json"), encoding="utf-8"))["calls"]

    report = {}
    for app, calls in fe.items():
        buckets = {"exact": [], "param-ok": [], "suspicious-shadow": [], "405": [], "404": []}
        for c in calls:
            kind, hit, extra = classify(c, backend)
            buckets[kind].append({**c, "hit": hit[1]["path"] if hit else None, "hit_crate": hit[0] if hit else None, "extra": extra})
        report[app] = buckets

    with open(os.path.join(HERE, "api_reconcile.json"), "w", encoding="utf-8") as fh:
        json.dump(report, fh, ensure_ascii=False, indent=1)

    bad_total = 0
    for app, b in report.items():
        tot = sum(len(v) for v in b.values())
        print(f"\n=== {app} ({tot} calls) ===")
        for k in ("exact", "param-ok", "suspicious-shadow", "405", "404"):
            print(f"  {k:20s} {len(b[k])}")
        for k in ("suspicious-shadow", "405", "404"):
            bad_total += len(b[k])
            for r in b[k]:
                print(f"    [{k}] {r['method']} {r['path']}  <- {r['file']}:{r['line']}  hit={r['hit_crate']}:{r['hit']} extra={r['extra']}")

    if args.gate:
        print(f"\n[gate] shadow/405/404 合计 = {bad_total}")
        if bad_total > 0:
            print("[gate] FAIL —— 三端存在未闭合的 API 调用（见上方明细）")
            return 1
        print("[gate] PASS —— 三端 API 调用全部闭合")
    return 0


if __name__ == "__main__":
    sys.exit(main())
