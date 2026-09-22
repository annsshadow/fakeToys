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
for end in rec:
    for cat in ("exact", "param-ok"):
        for c in rec[end][cat]:
            h = c.get("hit")
            if h:
                consumed.add((c["method"], h))

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
