#!/usr/bin/env python3
"""为对账问题项列出后端同前缀候选路由，辅助判定"前端写错" vs "后端缺失"。"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
backend = json.load(open(os.path.join(HERE, "backend_routes.json"), encoding="utf-8"))
rec = json.load(open(os.path.join(HERE, "api_reconcile.json"), encoding="utf-8"))

ALL = [(c, r["method"], r["path"]) for c, v in backend.items() for r in v]


def prefix_candidates(path, maxseg=6):
    segs = [s for s in path.split("/") if s]
    for n in range(min(len(segs), maxseg), 2, -1):
        pref = "/" + "/".join(segs[:n])
        hits = sorted({(m, p, c) for c, m, p in ALL if p.startswith(pref)})
        if hits:
            return n, hits
    return 0, []


for app in ("desktop", "mobile"):
    print("=" * 100)
    print("APP:", app)
    for kind in ("suspicious-shadow", "405", "404"):
        rows = rec[app][kind]
        if not rows:
            continue
        print(f"\n---- {kind} ({len(rows)}) ----")
        for r in rows:
            n, hits = prefix_candidates(r["path"])
            print(f"\n  [{r['method']}] {r['path']}")
            print(f"      at {r['file']}:{r['line']}   hit={r['hit_crate']}:{r['hit']} {r['extra']}")
            print(f"      后端同前缀({n}段)候选 {len(hits)}:")
            for m, p, c in hits[:14]:
                print(f"        {m:7s} {p}   <{c}>")
            if len(hits) > 14:
                print(f"        ... 另有 {len(hits)-14} 条")
