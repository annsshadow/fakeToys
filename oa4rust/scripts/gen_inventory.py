import re, os, glob

CRATES = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'crates')
OUT_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'docs', 'brainstorms')
os.makedirs(OUT_DIR, exist_ok=True)

# Infrastructure crates — not business logic, no handlers expected
EXCLUDED = {"mcp_server", "shared", "ldap", "orm", "openapi"}

WAVE = {}
U5 = ["attendance","attendance_assemble_control","attendance_core_entity","calendar",
      "calendar_assemble_control","calendar_core_entity","file","file_assemble_control",
      "file_core_entity","general","general_assemble_control","general_core_entity"]
U6 = ["process_surface","process_designer","process_express","processplatform_service_processing",
      "processplatform_core_entity","processplatform_core_express","processplatform_assemble_surface",
      "processplatform_assemble_bam","processplatform_assemble_designer","message",
      "message_assemble_communicate","message_core_entity","meeting","meeting_assemble_control",
      "meeting_core_entity","portal","portal_assemble_designer","portal_assemble_surface",
      "portal_core_entity","query_service","query_express","query_core_entity","query_core_express",
      "query_assemble_designer","query_assemble_surface","query_service_processing","cms_control",
      "cms_express","cms_assemble_control","cms_core_entity","cms_core_express"]
U7 = ["ai","ai_assemble_control","ai_core_entity","component","component_assemble_control",
      "component_core_entity","hotpic","hotpic_assemble_control","hotpic_core_entity","jpush",
      "jpush_assemble_control","jpush_core_entity","mind","mind_assemble_control","mind_core_entity",
      "bbs","bbs_assemble_control","bbs_core_entity","console","express","correlation",
      "correlation_service_processing","correlation_core_entity","correlation_core_express",
      "organization_assemble_express","organization_assemble_control","organization_assemble_authentication",
      "organization_assemble_personal","organization_core_entity","organization_core_express",
      "program_center","program_center_core_entity","base"]
DONE = ["auth","personal","personal_extend","control","program_init"]
for c in U5: WAVE[c] = "U5"
for c in U6: WAVE[c] = "U6"
for c in U7: WAVE[c] = "U7"
for c in DONE: WAVE[c] = "U1-U4"

def scan_crate(name):
    d = os.path.join(CRATES, name, "src")
    if not os.path.isdir(d):
        return None
    text = ""
    for f in glob.glob(os.path.join(d, "**", "*.rs"), recursive=True):
        try:
            text += open(f, encoding="utf-8").read()
        except Exception:
            pass
    stub = text.count("stub_")
    null = len(re.findall(r"ActionResult::success\(Value::Null\)", text))
    todo = text.count("real implementation needed")
    # Match all handler return patterns: Result<Json<ActionResult<...>>, AppError> or Json<ActionResult<...>>
    handlers = len(re.findall(r'async fn \w+\s*\([^)]*\)\s*(?:->\s*[^{]*)?\{', text))
    # More precise: count pub async fn declarations
    handlers = len(re.findall(r'pub\s+async\s+fn\s+\w+', text))
    routes = len(re.findall(r'\.route\(\s*"', text))
    if handlers > 0 and stub == 0 and null == 0:
        status = "done"
    elif handlers > 0:
        status = "doing"
    else:
        status = "todo"
    return dict(name=name, wave=WAVE.get(name, "U?"),
                stub=stub, null=null, todo=todo, handlers=handlers,
                routes=routes, status=status)

rows = []
for entry in sorted(os.listdir(CRATES)):
    if entry in EXCLUDED:
        continue
    if os.path.isdir(os.path.join(CRATES, entry)):
        r = scan_crate(entry)
        if r:
            rows.append(r)

order = {"U1-U4":0,"U5":1,"U6":2,"U7":3,"U?":4}
rows.sort(key=lambda r:(order.get(r["wave"],9), r["name"]))

# ---- endpoint-inventory.md ----
inv = ["# OA4Rust 端点清单与实现状态 (Endpoint Inventory)", "",
       "> 由 scripts/gen_inventory.py 基于源码静态扫描自动生成，每次真实化后重新运行。",
       "> 字段：routes=路由注册数, handlers=真实 handler 数, stub_=`stub_` 标记数,",
       "> null=纯 Value::Null 桩数, todo=TODO 占位数, status=done/doing/todo。", ""]
inv.append("| crate | wave | routes | handlers | stub_ | null | todo | status |")
inv.append("|-------|------|-------:|---------:|------:|-----:|-----:|:------|")
for r in rows:
    inv.append(f"| {r['name']} | {r['wave']} | {r['routes']} | {r['handlers']} | "
               f"{r['stub']} | {r['null']} | {r['todo']} | {r['status']} |")
tot_stub = sum(r['stub'] for r in rows)
tot_null = sum(r['null'] for r in rows)
tot_todo = sum(r['todo'] for r in rows)
tot_h = sum(r['handlers'] for r in rows)
done = sum(1 for r in rows if r['status']=='done')
doing = sum(1 for r in rows if r['status']=='doing')
todo = sum(1 for r in rows if r['status']=='todo')
inv += ["", "## 汇总", "",
        f"- crate 总数: {len(rows)}",
        f"- 已完成(done): {done} | 迁移中(doing): {doing} | 待迁移(todo): {todo}",
        f"- 真实 handler 总数: {tot_h}",
        f"- 残留 stub_ 标记: {tot_stub} | 纯 Value::Null 桩: {tot_null} | TODO 占位: {tot_todo}", ""]
open(os.path.join(OUT_DIR, "oa4rust-endpoint-inventory.md"), "w", encoding="utf-8").write("\n".join(inv))

# ---- migration-status.md ----
ms = ["# OA4Rust 迁移状态跟踪 (Migration Status)", "",
      "> 单一信息源（single source of truth）。由 scripts/gen_inventory.py 自动生成。",
      "> 状态含义：待迁移 / 迁移中 / 已完成。每完成一个 crate 的真实化后重新生成。", ""]
for wave in ["U1-U4","U5","U6","U7"]:
    ms.append(f"## {wave}")
    ms.append("")
    ms.append("| crate | 状态 | handlers | stub_ | null |")
    ms.append("|-------|------|---------:|------:|-----:|")
    for r in rows:
        if r["wave"] == wave:
            ms.append(f"| {r['name']} | {r['status']} | {r['handlers']} | {r['stub']} | {r['null']} |")
    ms.append("")
ms += ["## 说明", "",
       "- `done`：无 stub_ / Value::Null 桩，真实 handler 已暴露。",
       "- `doing`：已有真实 handler 但仍有桩标记待清除或 router 未完全暴露。",
       "- `todo`：仅有占位 handler，尚未真实化。",
       "- 基础设施 crate（mcp_server、shared、ldap、orm）排除在外，不统计。",
       "- 回滚/灰度见 deploy/nginx.conf、deploy/rollback-playbook.md、deploy/toggle_module.sh。",
       ""]
open(os.path.join(OUT_DIR, "oa4rust-migration-status.md"), "w", encoding="utf-8").write("\n".join(ms))

print(f"crates={len(rows)} done={done} doing={doing} todo={todo} "
      f"stub_={tot_stub} null={tot_null} todo_markers={tot_todo} handlers={tot_h}")
for r in rows:
    if r['status'] != 'done':
        print(f"  {r['status']:6} {r['name']} wave={r['wave']} h={r['handlers']} r={r['routes']} s={r['stub']} n={r['null']}")
