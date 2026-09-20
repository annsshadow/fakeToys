#!/usr/bin/env python3
"""从 backend_routes.json 重生成 desktop/mobile 契约守卫用的已注册路由 fixture。
排除 mock 无关——fixture 需含全部 .route 字面量路径（含 mock 别名），因为守卫比对的是"前端调用是否命中已注册路由"，mock 路由也是真实注册的。"""
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
backend = json.load(open(os.path.join(HERE, "backend_routes.json"), encoding="utf-8"))

paths = set()
for _crate, routes in backend.items():
    for r in routes:
        p = r["path"]
        if p.startswith("/"):
            paths.add(p)

fxp = os.path.join(ROOT, "oa4rust-web", "tests", "contracts", "backend-registered-routes.fixture.ts")
fx = open(fxp, encoding="utf-8").read()
header = fx[: fx.index("export const REGISTERED_BACKEND_ROUTES")]
cur = set(re.findall(r"'(/[^']+)'", fx))

# 保留原本存在的非 /api 健康检查路径
specials = [x for x in ("/health", "/hello/world") if x in cur]
allpaths = sorted(paths | set(specials))

lines = [header.rstrip("\n"), "", "export const REGISTERED_BACKEND_ROUTES: string[] = ["]
for p in allpaths:
    lines.append(f"  '{p}',")
lines.append("]")
open(fxp, "w", encoding="utf-8").write("\n".join(lines) + "\n")

new = open(fxp, encoding="utf-8").read()
print("fixture entries:", len(re.findall(r"'(/[^']+)'", new)))
print("participant present:", "participant" in new)
print("calendar list/my present:", "/api/calendar/calendar/list/my" in new)
