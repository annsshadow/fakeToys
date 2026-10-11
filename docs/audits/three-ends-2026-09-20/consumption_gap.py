#!/usr/bin/env python3
"""逐域后端能力消费缺口分析（G5 长期项抓手）。

口径：
- 排除 mock* 兼容别名（不可清理、不计入可消费面）。
- "已消费"= api_reconcile.json 中 exact/param-ok 命中的后端唯一 (method,path)。
- 输出按缺口从大到小排序，辅助 G5 逐域选点。
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
backend = json.load(open(os.path.join(HERE, "backend_routes.json"), encoding="utf-8"))
rec = json.load(open(os.path.join(HERE, "api_reconcile.json"), encoding="utf-8"))

DOMAINS = [
    ("认证/会话", ["/api/authentication", "/api/oauth", "/api/sso"]),
    ("组织-控制", ["/api/organization/assemble/control"]),
    ("组织-个人", ["/api/organization/assemble/personal", "/api/personal/", "/api/person/"]),
    ("组织-认证", ["/api/organization/assemble/authentication"]),
    ("组织-快递", ["/api/organization/assemble/express", "/api/organization/core/express"]),
    ("身份/角色/群组", ["/api/identity", "/api/role", "/api/group", "/api/unit", "/api/person/group"]),
    ("考勤", ["/api/attendance"]),
    ("日历", ["/api/calendar"]),
    ("会议", ["/api/meeting"]),
    ("论坛BBS", ["/api/bbs"]),
    ("内容CMS", ["/api/cms", "/api/form", "/api/view", "/api/document", "/api/categoryinfo", "/api/appinfo", "/api/column", "/api/module", "/api/dict", "/api/template", "/api/script"]),
    ("流程-设计器", ["/api/processplatform/assemble/designer"]),
    ("流程-表面", ["/api/processplatform/assemble/surface"]),
    ("流程-引擎", ["/api/processplatform/service/processing", "/api/task/", "/api/work/"]),
    ("流程-BAM", ["/api/processplatform/assemble/bam"]),
    ("门户-设计器", ["/api/portal/assemble/designer"]),
    ("门户-表面", ["/api/portal/assemble/surface"]),
    ("查询-设计器", ["/api/query/assemble/designer"]),
    ("查询-表面", ["/api/query/assemble/surface", "/api/queryview"]),
    ("消息/IM", ["/api/message"]),
    ("文件/附件", ["/api/file", "/api/attachment"]),
    ("回收站", ["/api/recycle"]),
    ("思维导图", ["/api/mind"]),
    ("通知/推送", ["/api/jpush", "/api/notify"]),
    ("热图/统计", ["/api/hotpic"]),
    ("搜索", ["/api/search", "/api/ftsearch"]),
    ("AI", ["/api/ai"]),
    ("程序中心", ["/api/program_center"]),
    ("服务器/控制台", ["/api/server", "/api/console", "/api/config"]),
    ("MCP/开放接口", ["/api/mcp", "/api/openapi"]),
    ("实时/WebSocket", ["/api/realtime", "/ws"]),
    ("授权/签名", ["/api/empower", "/api/signature"]),
]


def belongs(path, keys):
    return any(k in path for k in keys)


consumed = set()
# compare.py 用 METHOD_ALIAS 把前端 upload/download 动词按 POST/GET 与后端匹配后
# 才落 hit；此处构造 consumed 集必须用同一别名，否则 api.upload(POST 路由)/
# api.download(GET 路由) 的真实消费会因动词字面不等而被漏计。
METHOD_ALIAS = {"UPLOAD": "POST|PUT", "DOWNLOAD": "GET", "STREAM": "POST", "UPLOADBYTES": "POST"}
for end in rec:
    for cat in ("exact", "param-ok"):
        for c in rec[end][cat]:
            h = c.get("hit")
            if h:
                consumed.add((METHOD_ALIAS.get(c["method"], c["method"]), h))

rows = []
for name, keys in DOMAINS:
    total, mock, real = set(), set(), set()
    for _crate, routes in backend.items():
        for r in routes:
            if belongs(r["path"], keys):
                key = (r["method"], r["path"])
                total.add(key)
                (mock if "mock" in r["path"] else real).add(key)
    cons = sum(1 for k in real if k in consumed)
    rows.append((name, len(total), len(mock), len(real), cons, len(real) - cons))

rows.sort(key=lambda x: -x[5])
print(f"{'域':<14}{'总':>6}{'mock':>6}{'真实':>6}{'已消费':>7}{'缺口':>7}")
print("-" * 47)
for r in rows:
    print(f"{r[0]:<14}{r[1]:>6}{r[2]:>6}{r[3]:>6}{r[4]:>7}{r[5]:>7}")
tr = sum(r[3] for r in rows)
tc = sum(r[4] for r in rows)
print("-" * 47)
print(f"真实可消费总计={tr}  已消费={tc}  消费率={tc / tr * 100:.1f}%")

# ── 诚实口径（十类功能裁定原则：桶外 + 结构不可计入 剔除分母，其余须真实消费）──
# 桶外 = EXTERNAL_IDP（外部身份源回调/登录，消费者为外部系统，前端不可亦不应 XHR）
# 结构 = TWIN_ALIAS（同 handler 双计放宽 rev486）/ SENSITIVE / MALFORMED / TEST_GARBAGE。
# 这些在 unconsumed_ledger.json 中已逐条记档裁决，剔除后剩余分母应 100% 真实消费。
try:
    ledger = json.load(open(os.path.join(HERE, "unconsumed_ledger.json"), encoding="utf-8"))
    OUT_OF_BUCKET_VERDICTS = {"桶外（外部消费者）", "结构不可计入"}
    excluded = {
        (r["method"], r["path"])
        for r in ledger.get("routes", [])
        if r.get("verdict") in OUT_OF_BUCKET_VERDICTS
    }
    # 分母 = 真实非 mock 路由；剔除桶外/结构后为「诚实可消费面」
    all_real = set()
    for _crate, routes in backend.items():
        for r in routes:
            if "mock" in r["path"]:
                continue
            all_real.add((r["method"], r["path"]))
    honest_denom = all_real - excluded
    # 消费判定须考虑 UPLOAD/DOWNLOAD 动词别名：consumed 集以「前端动词(可含|)」为键，
    # 后端路由 (method,path) 被消费当且仅当存在 hit==path 且 method ∈ 该键展开的方法集。
    from collections import defaultdict

    cons_methods_by_path = defaultdict(set)
    for verb, path in consumed:
        for m in verb.split("|"):
            cons_methods_by_path[path].add(m)
    honest_consumed = {
        k
        for k in honest_denom
        if (lambda m, p: m in cons_methods_by_path.get(p, set()))(*k)
    }
    hd = len(honest_denom)
    hc = len(honest_consumed)
    print()
    print("── 诚实口径（剔除桶外+结构记档项）────────────────────────")
    print(f"桶外/结构记档剔除={len(excluded)}  诚实可消费面={hd}  已消费={hc}")
    print(f"诚实消费率={hc / hd * 100 if hd else 100:.1f}%")
    if hc < hd:
        miss = honest_denom - honest_consumed
        print(f"仍未消费 {len(miss)} 条（应继续真实接线）:")
        for m, p in sorted(miss):
            print(f"  {m} {p}")
except FileNotFoundError:
    print()
    print("（unconsumed_ledger.json 缺失，未计算诚实口径——请先运行 _classify_unconsumed.py）")
