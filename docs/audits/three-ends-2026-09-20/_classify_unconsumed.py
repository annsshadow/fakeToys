#!/usr/bin/env python3
"""对剩余未消费后端路由做诚实分类台账（区分「运行时可验证」与「结构上不可诚实计入」）。

口径说明：consumption_gap.py 是静态源码覆盖度量（前端 api.* 语句 ∩ backend_routes），
不测运行时命中。本脚本把 backend_routes ∖ (exact∪param-ok hit) 的每条未消费路由按
「为何未被消费 / 能否诚实计入」归类，输出 unconsumed_ledger.json 供审计与后续 live 环境对照。
"""
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
AL = {"UPLOAD": "POST", "DOWNLOAD": "GET"}
backend = json.load(open(os.path.join(HERE, "backend_routes.json"), encoding="utf-8"))
rec = json.load(open(os.path.join(HERE, "api_reconcile.json"), encoding="utf-8"))

consumed = set()
for end in rec:
    for cat in ("exact", "param-ok"):
        for c in rec[end][cat]:
            h = c.get("hit")
            if h:
                consumed.add((AL.get(c["method"], c["method"]), h))

# 归一化路径（{param}->{}）用于识别 twin（同 method 同归一形状有 >1 条注册）
def norm(p):
    return "/".join("{}" if s.startswith("{") and s.endswith("}") else s for s in p.split("/"))

norm_count = {}
for crate, rs in backend.items():
    for r in rs:
        if "mock" in r["path"]:
            continue
        key = (r["method"], norm(r["path"]))
        norm_count[key] = norm_count.get(key, 0) + 1

# 分类规则（按优先级自上而下），每类附「能否诚实计入」判定
EXTERNAL_IDP = ["oauth", "oidc", "dingding", "qywx", "qiyeweixin", "mpweixin", "welink",
                "zhengwudingding", "zhengwu", "/andfx/", "/moa/", "/sso", "collect"]
EXTERNAL_MSG = ["message/test/send", "chat/completion", "completion/stream", "code/mobile",
                "regist/code", "reset/code", "sms/send", "sms/verify", "/sms"]
ENGINE = ["html/to/pdf", "html/to/image", "/to/pdf", "/to/image"]
WRITE_VERBS = ["delete", "create", "/save", "update", "/reset", "/set", "erase", "upload",
               "import", "/bind", "unbind", "publish", "submit", "/change", "sync",
               "/add", "/fire", "/execute", "changetitle", "/mass", "menu/subscribe",
               "template/send", "cancel", "remove", "onlyremove", "/clear", "/press",
               "/signal", "copy", "/icon", "encode", "/config", "secret/set", "token",
               "regist", "two_factor", "/event", "/msg/clear", "invoke", "/syn"]
SESSION_DESTR = ["safe/logout", "switchuser", "logout", "login", "authentication/authentication",
                 "/api/authentication", "two/factory", "/bind/meta", "/info/sign", "/dingding/info",
                 "/sso", "/info"]
SENSITIVE = ["password/{password}", "/password/{"]
CREDENTIAL = ["{credential}", "anonymous", "share/{id}/password", "captcha", "callback/aes",
              "getprivateinfo"]
WEBSOCKET = ["/ws/"]
TEST = ["/test/", "/foo/", "/bar/", "test1", "test2", "/test"]
# 泛化参数名（p0/p1/p2…）几乎总是命名参数孪生的重复注册变体
GENERIC_PARAM_RE = re.compile(r"\{p\d+\}")

def classify(method, path):
    lp = path.lower()
    # 结构上不可诚实计入：twin/别名（同 method 同归一形状 >1 注册）
    if norm_count.get((method, norm(path)), 0) > 1:
        return ("TWIN_ALIAS", "结构不可计入", "同 method 同归一形状存在多条注册（下划线别名/参数名孪生），前端只能命中一条，另一条无法用不同调用区分")
    # 下划线别名形（/api/xxx_assemble_control/…）是 slash 形（/api/xxx/assemble/control/…）的 O2 双寻址别名
    if "_assemble_control/" in path:
        return ("TWIN_ALIAS", "结构不可计入", "下划线模块寻址别名，与已消费的 slash 形（/assemble/control/）同一 handler，前端只用一种寻址，另一种为别名孪生")
    # 泛化 {p0}/{p1}/{p2} 参数名 = 命名参数版的重复注册孪生
    if GENERIC_PARAM_RE.search(path):
        return ("TWIN_ALIAS", "结构不可计入", "泛化 {pN} 参数名路由，是命名参数版路由的重复注册孪生，归一形状相同不可用不同调用区分")
    if "{{" in path or "}}" in path or " " in path or "%20" in path:
        return ("MALFORMED", "结构不可计入", "路由字面量含双花括号/空格/%20 等畸形注册，非真实可调用形状")
    if any(t in lp for t in TEST):
        return ("TEST_GARBAGE", "结构不可计入", "test/foo/bar 脚手架垃圾路由，非真实用户能力")
    if any(k in lp for k in WEBSOCKET):
        return ("WEBSOCKET", "仅运行时可验证", "WebSocket 端点，非前端 api.get；需真实 WS 客户端握手触发")
    if any(k in lp for k in ENGINE):
        return ("ENGINE", "仅运行时可验证", "依赖 HTML→PDF/图片 渲染引擎的转换端点，需引擎与真实文档")
    if any(k in lp for k in EXTERNAL_IDP):
        return ("EXTERNAL_IDP", "仅运行时可验证", "外部身份源回调/登录/绑定（IdP 浏览器重定向落地端点），非前端自身 api 调用；需真实第三方 IdP")
    if any(k in lp for k in EXTERNAL_MSG):
        return ("EXTERNAL_MSG", "仅运行时可验证", "外部短信/LLM/推送测试端点，需真实第三方服务")
    if any(k in lp for k in SENSITIVE):
        return ("SENSITIVE", "结构不可计入", "口令入 URL 的校验端点，安全上不应由前端明文调用")
    if any(k in lp for k in CREDENTIAL):
        return ("CREDENTIAL_ANON", "仅运行时可验证", "凭证/匿名 token/验证码类，需真实运行时令牌流程")
    if any(k in lp for k in SESSION_DESTR):
        return ("SESSION_DESTRUCTIVE", "仅运行时可验证", "会话销毁/登录类（登出/切换用户/登录），接入将破坏或依赖会话状态")
    if method in ("DELETE", "PUT") or any(k in lp for k in WRITE_VERBS):
        return ("DESTRUCTIVE_WRITE", "仅运行时(真实用户动作)", "破坏性/写端点，接前端仅为凑指标即造假；只能由真实用户动作在运行时触发")
    return ("OTHER_READ_LIKE", "待人工复核", "未归入上述类别，需人工核实是否为可诚实接入的真实非破坏读端点")

ledger = {}
rows = []
for crate, rs in backend.items():
    for r in rs:
        if "mock" in r["path"]:
            continue
        k = (r["method"], r["path"])
        if k in consumed:
            continue
        cat, verdict, reason = classify(r["method"], r["path"])
        rows.append({"method": r["method"], "path": r["path"], "crate": crate,
                     "category": cat, "verdict": verdict, "reason": reason})

# dedup by (method,path)
seen = set()
dedup = []
for x in rows:
    kk = (x["method"], x["path"])
    if kk in seen:
        continue
    seen.add(kk)
    dedup.append(x)

from collections import Counter
by_cat = Counter(x["category"] for x in dedup)
by_verdict = Counter(x["verdict"] for x in dedup)
out = {"total_unconsumed": len(dedup), "by_category": dict(by_cat),
       "by_verdict": dict(by_verdict), "routes": sorted(dedup, key=lambda x: (x["category"], x["path"]))}
with open(os.path.join(HERE, "unconsumed_ledger.json"), "w", encoding="utf-8") as fh:
    json.dump(out, fh, ensure_ascii=False, indent=1)

print("未消费总计:", len(dedup))
print("\n按类别:")
for c, n in by_cat.most_common():
    print(f"  {c:22s} {n}")
print("\n按裁决:")
for v, n in by_verdict.most_common():
    print(f"  {v:22s} {n}")
print("\nOTHER_READ_LIKE（需人工复核）明细:")
for x in dedup:
    if x["category"] == "OTHER_READ_LIKE":
        print(f"  {x['method']} {x['path']}")
