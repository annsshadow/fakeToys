#!/usr/bin/env python3
"""业务流程覆盖矩阵：业务域 × (后端路由 / 桌面端调用 / 移动端调用)。"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
backend = json.load(open(os.path.join(HERE, "backend_routes.json"), encoding="utf-8"))
fe = json.load(open(os.path.join(HERE, "frontend_calls.json"), encoding="utf-8"))["calls"]
rec = json.load(open(os.path.join(HERE, "api_reconcile.json"), encoding="utf-8"))

# 业务域 -> URL 片段匹配规则
DOMAINS = [
    ("认证/会话", ["/api/authentication", "/api/authentication/", "/api/oauth", "/api/sso"]),
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
    ("短信/验证码", ["/api/sms", "/api/captcha"]),
    ("授权/签名", ["/api/empower", "/api/signature"]),
]


def belongs(path, keys):
    return any(k in path for k in keys)


def count_backend(keys):
    uniq = set()
    for crate, routes in backend.items():
        for r in routes:
            if belongs(r["path"], keys):
                uniq.add((r["method"], r["path"]))
    return len(uniq)


def count_fe(app, keys):
    return sum(1 for c in fe[app] if belongs(c["path"], keys))


def count_bad(app, keys):
    n = 0
    for kind in ("suspicious-shadow", "405", "404"):
        n += sum(1 for r in rec[app][kind] if belongs(r["path"], keys))
    return n


def main():
    print(f"{'业务域':<16}{'后端路由':>9}{'桌面调用':>9}{'桌面问题':>9}{'移动调用':>9}{'移动问题':>9}")
    print("-" * 62)
    tot = [0, 0, 0, 0, 0]
    for name, keys in DOMAINS:
        b = count_backend(keys)
        d = count_fe("desktop", keys)
        db = count_bad("desktop", keys)
        m = count_fe("mobile", keys)
        mb = count_bad("mobile", keys)
        print(f"{name:<16}{b:>9}{d:>9}{db:>9}{m:>9}{mb:>9}")
        tot = [tot[0] + b, tot[1] + d, tot[2] + db, tot[3] + m, tot[4] + mb]
    print("-" * 62)
    print(f"{'合计(去重前)':<14}{tot[0]:>9}{tot[1]:>9}{tot[2]:>9}{tot[3]:>9}{tot[4]:>9}")
    print()
    b_all = len({(r['method'], r['path']) for v in backend.values() for r in v})
    print(f"后端唯一(method,path) 总数: {b_all}")
    print(f"桌面端调用总数: {len(fe['desktop'])}   移动端调用总数: {len(fe['mobile'])}")


if __name__ == "__main__":
    main()
