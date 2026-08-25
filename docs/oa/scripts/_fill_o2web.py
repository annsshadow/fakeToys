#!/usr/bin/env python3
# Fill Key Flows / Dependencies for o2web component cards from real source.
import re, json
from pathlib import Path

REPO = Path(r"D:\WORKSPACE\fakeToys")
SRC_ROOT = REPO / "oa/o2web/source"
CARDS = REPO / "docs/oa/modules/o2web"

EXCLUDE = ("node_modules", "dist", "build", ".git", "o2_lib", "public", "lp")
SKIP_EXT = {".png", ".jpg", ".jpeg", ".gif", ".ico", ".css", ".map", ".woff",
            ".woff2", ".ttf", ".eot", ".svg", ".json", ".md", ".html", ".vue"}  # vue handled separately

METHOD_VERB = {
    "get": "读取", "list": "列出", "query": "查询", "find": "查找", "search": "检索",
    "save": "保存", "create": "创建", "add": "添加", "update": "更新", "set": "设置",
    "delete": "删除", "remove": "移除", "upload": "上传", "download": "下载",
    "submit": "提交", "check": "校验", "count": "统计", "export": "导出",
    "import": "导入", "execute": "执行", "start": "启动", "stop": "停止",
    "send": "发送", "open": "打开", "close": "关闭", "lock": "锁定",
    "unlock": "解锁", "publish": "发布", "preview": "预览", "copy": "复制",
}

def verb_for(method):
    m = method.lower()
    for k, v in METHOD_VERB.items():
        if m.startswith(k):
            return v
    return "调用"

def read_text(p):
    try:
        return p.read_text(encoding="utf-8", errors="ignore")
    except Exception:
        return ""

def source_files(comp_dir):
    out = []
    if not comp_dir.exists():
        return out
    for f in comp_dir.rglob("*"):
        if not f.is_file():
            continue
        if any(part in EXCLUDE for part in f.parts):
            continue
        if f.suffix.lower() in {".png", ".jpg", ".jpeg", ".gif", ".ico", ".css",
                                ".map", ".woff", ".woff2", ".ttf", ".eot", ".svg",
                                ".md"}:
            continue
        if f.suffix.lower() in {".min.js"}:
            continue
        out.append(f)
    return out

def analyze(comp_dir):
    files = source_files(comp_dir)
    full = "\n".join(read_text(f) for f in files)

    # backend action modules loaded via o2.Actions.load("x_...")
    load_mods = set(re.findall(r'o2\.Actions\.load\(\s*["\']([^"\']+)["\']', full))
    # direct o2.Actions.XxxAction usage
    # jaxrs url backend modules: ../x_module/jaxrs/...
    jaxrs_mods = set(re.findall(r'\.\./([a-zA-Z0-9_]+)/jaxrs/', full))
    # also "/jaxrs/..."  with module prefix captured differently
    jaxrs_urls = re.findall(r'\.\./([a-zA-Z0-9_]+)/jaxrs/([^\s"\'`]+)', full)
    # backend action method calls: this.action.XxxAction.method(
    action_calls = re.findall(r'[.\w]*?([A-Z]\w*Action)\.([a-zA-Z_]\w*)\s*\(', full)
    # fetch/ajax calls with method + url
    fetches = re.findall(r'(?:fetch|o2\.request|ajax|o2\.xAction)\s*\(?\s*(?:url\s*[:=])?\s*["\']([^"\']+)["\']', full)
    # cross-component deps
    req_apps = re.findall(r'o2\.x?Desktop?\.?requireApp\(\s*["\']([^"\']+)["\']\s*,\s*["\']([^"\']+)["\']', full)
    req_apps += re.findall(r'o2\.requireApp\(\s*["\']([^"\']+)["\']\s*,\s*["\']([^"\']+)["\']', full)
    open_apps = re.findall(r'openApplication\(\s*\w+\s*,\s*["\']([^"\'.]+)', full)

    # group action methods
    by_action = {}
    for act, meth in action_calls:
        by_action.setdefault(act, set()).add(meth)

    return {
        "load_mods": sorted(load_mods),
        "jaxrs_mods": sorted(jaxrs_mods),
        "jaxrs_urls": sorted(set(jaxrs_urls)),
        "by_action": {k: sorted(v) for k, v in by_action.items()},
        "req_apps": sorted(set((a, b) for a, b in req_apps)),
        "open_apps": sorted(set(open_apps)),
        "nfiles": len(files),
        "loc": sum(len(read_text(f).splitlines()) for f in files),
    }

def gen_keyflows(comp_name, info, responsibility):
    self_token = comp_name[len("x_component_"):] if comp_name.startswith("x_component_") else comp_name
    bullets = []
    by_action = info["by_action"]
    if not by_action:
        by_action = {}
    # streaming / fetch jaxrs
    for mod, path in info["jaxrs_urls"][:6]:
        verb = "请求"
        for kw in ("completion", "stream", "sse"):
            if kw in path.lower():
                verb = "流式请求"
        method = "POST" if verb == "流式请求" or "completion" in path.lower() else "GET"
        bullets.append(
            f"后端请求：向 `{mod}/jaxrs/{path}` 发起 {method} {verb}，处理返回数据并渲染。"
        )
    # per backend action module flows
    for act, methods in by_action.items():
        verbs = "/".join(verb_for(m) for m in methods[:6])
        mlist = "`、`".join(methods[:8])
        bullets.append(
            f"与后端 `{act}` 交互：在组件中调用 `{act}.{mlist}` 等方法完成 {verbs} 等操作。"
        )
    # cross-component navigation
    openapps = [a for a in info["open_apps"] if a != self_token and not a.startswith(self_token + ".")]
    req_apps = [(a, b) for a, b in info["req_apps"] if a != self_token]
    if openapps:
        apps = "、".join(openapps[:6])
        bullets.append(f"跨组件跳转：通过 `openApplication` 打开其它应用（{apps}）。")
    if req_apps:
        deps = "、".join(f"{a}.{b}" for a, b in req_apps[:6])
        bullets.append(f"依赖其它组件能力：通过 `requireApp` 引入 {deps} 等组件模块。")
    # fallback
    if not bullets:
        bullets.append(
            f"{responsibility}（源码中未检出显式后端 action 调用，主要在前端完成交互与渲染。）"
        )
    # trim to 3-6
    if len(bullets) > 6:
        bullets = bullets[:6]
    if len(bullets) < 3:
        # pad with responsibility-based generic but not empty
        while len(bullets) < 3:
            bullets.append(f"核心交互：{responsibility}")
    return bullets

def gen_deps(comp_name, info):
    lines = []
    self_token = comp_name[len("x_component_"):] if comp_name.startswith("x_component_") else comp_name
    backend = set(info["load_mods"]) | set(info["jaxrs_mods"])
    if backend:
        lines.append("**后端服务（o2server action / REST）：**")
        for m in sorted(backend):
            lines.append(f"- `{m}`")
    req = [(a, b) for a, b in info["req_apps"] if a != self_token]
    openapps = [a for a in info["open_apps"] if a != self_token and not a.startswith(self_token + ".")]
    if req or openapps:
        lines.append("**依赖的其它 o2web 组件 / 应用：**")
        seen = set()
        for a, b in req:
            key = f"{a}.{b}"
            if key not in seen:
                seen.add(key)
                lines.append(f"- `{a}.{b}`")
        for app in openapps:
            if app not in seen:
                seen.add(app)
                lines.append(f"- `{app}`（openApplication 打开的应用）")
    lines.append("**前端基础设施：**")
    lines.append("- O2OA web 框架（MWF / o2.Actions / o2.xDesktop）、Vue 组件或 MooTools Class 组件模型")
    lines.append("- 公共库：`o2_lib`（marked、PinYin 等）、`$OOUI` 组件库")
    return lines

def get_responsibility(card_text):
    m = re.search(r"## Responsibility\s*\n(.*?)(?=\n## |\Z)", card_text, re.S)
    if not m:
        return ""
    return m.group(1).strip()

def main():
    results = {}
    for card in sorted(CARDS.glob("*.md")):
        comp_name = card.stem  # x_component_XXX
        src_dir = SRC_ROOT / comp_name
        info = analyze(src_dir)
        text = card.read_text(encoding="utf-8")
        resp = get_responsibility(text)
        kf = gen_keyflows(comp_name, info, resp)
        deps = gen_deps(comp_name, info)
        # strip existing Key Flows / Dependencies to allow regenerate
        text = re.sub(r"\n## Key Flows\s*\n.*(\n## Dependencies\s*\n.*)?\Z", "", text, flags=re.S)
        block = "\n## Key Flows\n\n" + "\n".join(f"- {b}" for b in kf) + "\n\n## Dependencies\n\n" + "\n".join(deps) + "\n"
        new_text = text.rstrip("\n") + "\n" + block
        card.write_text(new_text, encoding="utf-8")
        results[comp_name] = f"OK files={info['nfiles']} loc={info['loc']} backend={len(set(info['load_mods'])|set(info['jaxrs_mods']))}"
    # summary
    print("TOTAL", len(results))
    ok = sum(1 for v in results.values() if v.startswith("OK"))
    skip = sum(1 for v in results.values() if v.startswith("SKIP"))
    print("OK", ok, "SKIP", skip)
    for k, v in results.items():
        print(f"{k}: {v}")

if __name__ == "__main__":
    main()
