"""Seed the o2server reference container per seed_fixtures_java.http.md.

Non-idempotent creations: HTTP 500 (duplicate) = "already seeded", ignore.
Usage:
  python scripts/seed_java.py            # org-domain seeds (§2-6)
  python scripts/seed_java.py --process-form   # + §7 process/form from migration 091
"""
import json
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]

BASE = "http://127.0.0.1:18080"
CREDENTIAL = "xadmin"
PASSWORD = "o2oa@2022"


class JavaSideUnavailable(RuntimeError):
    """Java 参考容器不可用/未就绪。

    与 CI workflow 中 `Wait for o2server readiness` 步骤的既定语义保持一致：
    该步骤超时后只发 `::warning::` 并明确写明
    "behavior comparison will SKIP Java side"，不判定失败。
    因此本脚本在未显式要求初始化（`--init`）时，遇到 Java 侧不可用应当
    同样优雅跳过，而不是 SystemExit 掉整个 job。

    o2server 是冷启动约 10 分钟的 Java 容器，在 GitHub runner 上按 30 分钟
    的 job 超时常常无法就绪——旧实现会因此让 behavior-compare 必然失败。
    """


def call(method: str, path: str, body: dict | None = None, token: str | None = None):
    data = json.dumps(body).encode() if body is not None else None
    headers = {"Content-Type": "application/json"}
    if token:
        headers["x-token"] = token
    req = urllib.request.Request(f"{BASE}{path}", data=data, headers=headers, method=method)
    try:
        with urllib.request.urlopen(req, timeout=60) as resp:
            return resp.status, json.loads(resp.read().decode())
    except urllib.error.HTTPError as e:
        try:
            return e.code, json.loads(e.read().decode())
        except Exception:
            return e.code, {}
    except Exception as e:
        return 0, {"error": str(e)}


def init_fresh_container() -> None:
    """全新 o2server 的无头初始化（docs/ops/o2server-container.md）。

    secret/set 幂等：已初始化容器上重复调用返回错误响应，忽略即可。
    server/execute 触发部署建表，首次约 10 分钟。
    """
    print("initializing o2server (secret/set + server/execute)...")
    call("POST", "/jaxrs/secret/set", {"secret": PASSWORD})
    call("GET", "/jaxrs/server/execute")
    for attempt in range(90):
        status, payload = call("POST", "/x_organization_assemble_authentication/jaxrs/authentication",
                               {"credential": CREDENTIAL, "password": PASSWORD})
        if status == 200 and payload.get("type") == "success":
            print(f"o2server ready after ~{attempt * 10}s")
            return
        time.sleep(10)
    raise SystemExit("o2server did not become ready within 15 minutes")


def login() -> str:
    status, payload = call("POST", "/x_organization_assemble_authentication/jaxrs/authentication",
                           {"credential": CREDENTIAL, "password": PASSWORD})
    if status != 200 or payload.get("type") != "success":
        raise JavaSideUnavailable(f"LOGIN FAILED {status} {str(payload)[:200]}")
    print("login ok")
    return payload["data"]["token"]


def seed_testadmin(token: str) -> None:
    """创建行为对比账户 testadmin/testadmin（已存在则幂等跳过）。"""
    status, payload = call("POST", "/x_organization_assemble_control/jaxrs/person",
                           {"name": "testadmin", "mobile": "13900000099", "unique": "testadmin"}, token)
    person_id = (payload.get("data") or {}).get("id")
    if not person_id:
        status, payload = call("GET", "/x_organization_assemble_control/jaxrs/person/list", None, token)
        rows = payload.get("data") if isinstance(payload.get("data"), list) else []
        found = [r for r in rows if r.get("unique") == "testadmin" or r.get("name") == "testadmin"]
        person_id = found[0]["id"] if found else None
    if person_id:
        status, _ = call("PUT", f"/x_organization_assemble_control/jaxrs/person/{person_id}/set/password",
                         {"value": "testadmin"}, token)
        print(f"testadmin set-password: {status}")
    # 最终以登录验证兜底（账户已存在且密码正确时即便查回失败也继续）
    status, payload = call("POST", "/x_organization_assemble_authentication/jaxrs/authentication",
                           {"credential": "testadmin", "password": "testadmin"})
    if status == 200 and payload.get("type") == "success":
        print("testadmin login verified")
    else:
        raise SystemExit(f"testadmin login failed: {status} {str(payload)[:150]}")


def seed_org_domain(token: str) -> None:
    creations = [
        ("unit {flag}", {"name": "{flag}", "typeList": ["部门"]}),
        ("unit {unitFlag}", {"name": "{unitFlag}"}),
        ("person {personFlag}", {"name": "{personFlag}", "mobile": "13900000001", "unique": "{personFlag}"}),
        ("person {flag}", {"name": "{flag}", "mobile": "13900000002", "unique": "{flag}"}),
        ("group {flag}", {"name": "{flag}"}),
        ("group {groupFlag}", {"name": "{groupFlag}"}),
        ("role {roleFlag}", {"name": "{roleFlag}"}),
        ("identity {identityFlag}", {"name": "{identityFlag}", "person": "{personFlag}", "unit": "{unitFlag}"}),
    ]
    for label, body in creations:
        path = "/x_organization_assemble_control/jaxrs/" + label.split()[0]
        status, payload = call("POST", path, body, token)
        ok = status in (200, 500)  # 500 = duplicate => already seeded
        detail = "ok/dup" if ok else f"UNEXPECTED {str(payload)[:120]}"
        print(f"{label}: {status} {detail}")


def load_091_process_definition() -> dict:
    """从迁移 091 提取 process_definition JSON。"""
    text = open(REPO_ROOT / "oa4rust/migrations/091_seed_real_process_definition.sql", encoding="utf-8").read()
    start = text.index("'{", text.index("process_definition"))
    end = text.index("}'::jsonb", start)
    return json.loads(text[start + 1:end + 1])


def load_091_form_definition() -> str:
    """从迁移 091 提取 form definition JSON 文本。"""
    text = open(REPO_ROOT / "oa4rust/migrations/091_seed_real_process_definition.sql", encoding="utf-8").read()
    start = text.index("'{", text.index("INSERT INTO x_cms_form"))
    end = text.index("}',", start)
    return text[start + 1:end + 1]


def seed_process_form(token: str) -> None:
    """§7：流程应用 + 完整 activities 流程 + CMS 应用 + 表单（与迁移 091 对应）。

    Java 侧实体 id 必须为 UUID 格式：把 091 定义中的字面 id 统一映射为
    uuid4 并保持 routeList/fromActivity/activity 引用一致。
    """
    import uuid

    definition = load_091_process_definition()
    name = definition["name"]

    # 流程应用（Java ExceptionApplicationNotExist：流程必须挂在应用下）
    status, payload = call("POST", "/x_processplatform_assemble_designer/jaxrs/application",
                           {"name": "seed"}, token)
    app_id = (payload.get("data") or {}).get("id")
    if not app_id:
        status, payload = call("GET", "/x_processplatform_assemble_surface/jaxrs/application/list/applicationcategory/all", None, token)
        rows = payload.get("data") if isinstance(payload.get("data"), list) else []
        found = [r for r in rows if r.get("name") == "seed"]
        app_id = found[0]["id"] if found else None
    print(f"process application: {status} id={app_id}")
    if not app_id:
        print("skip process/form seeding: no application")
        return

    # id -> uuid4，保持引用一致
    idmap = {definition["id"]: str(uuid.uuid4())}
    for key in ("activities", "routes", "routeList"):
        for item in definition.get(key, []):
            idmap[item["id"]] = str(uuid.uuid4())
    for key in ("begin", "manualList", "endList"):
        v = definition.get(key)
        if isinstance(v, dict):
            idmap[v["id"]] = str(uuid.uuid4())
        elif isinstance(v, list):
            for item in v:
                idmap[item["id"]] = str(uuid.uuid4())

    def fix(o):
        if isinstance(o, dict):
            out = {}
            for k, v in o.items():
                if k in ("id", "process", "fromActivity", "activity") and isinstance(v, str) and v in idmap:
                    out[k] = idmap[v]
                elif k == "routeList" and isinstance(v, list):
                    out[k] = [idmap.get(x, x) if isinstance(x, str) else fix(x) for x in v]
                else:
                    out[k] = fix(v)
            return out
        if isinstance(o, list):
            return [fix(x) for x in o]
        return o

    definition = fix(definition)
    definition["application"] = app_id

    status, payload = call("POST", "/x_processplatform_assemble_designer/jaxrs/process", definition, token)
    process_id = (payload.get("data") or {}).get("id")
    print(f"process create: {status} id={process_id}")

    # CMS 应用（表单的 appId 依赖）
    status, payload = call("POST", "/x_cms_assemble_control/jaxrs/appinfo", {"appName": "seedApp"}, token)
    cms_app_id = (payload.get("data") or {}).get("id")
    if not cms_app_id:
        status, payload = call("GET", "/x_cms_assemble_control/jaxrs/appinfo/list/manage", None, token)
        rows = payload.get("data") if isinstance(payload.get("data"), list) else []
        found = [r for r in rows if r.get("appName") == "seedApp"]
        cms_app_id = found[0]["id"] if found else None
    print(f"cms appinfo: {status} id={cms_app_id}")

    # 表单：创建 + 写入 moduleList 定义
    form_name = "seed-leave-form"
    status, payload = call("POST", "/x_cms_assemble_control/jaxrs/form",
                           {"name": form_name, "appId": cms_app_id or ""}, token)
    form_id = (payload.get("data") or {}).get("id")
    if not form_id:
        status, payload = call("GET", "/x_cms_assemble_control/jaxrs/form/list/all", None, token)
        rows = payload.get("data") if isinstance(payload.get("data"), list) else []
        found = [r for r in rows if r.get("name") == form_name]
        form_id = found[0]["id"] if found else None
    print(f"form create: {status} id={form_id}")
    if form_id:
        status, payload = call("PUT", f"/x_cms_assemble_control/jaxrs/form/{form_id}",
                               {"name": form_name, "appId": cms_app_id or "",
                                "definition": load_091_form_definition()}, token)
        print(f"form update: {status} {str(payload)[:80]}")


def main() -> int:
    if "--init" in sys.argv:
        init_fresh_container()
    try:
        token = login()
    except JavaSideUnavailable as e:
        if "--init" in sys.argv or "--required" in sys.argv:
            # 显式要求 Java 侧时必须就绪，保持严格语义。
            raise SystemExit(str(e))
        print(f"::warning::Java side unavailable ({e}); skipping Java-side seed. "
              "Behavior comparison will run with Java side absent "
              "(same contract as the 'Wait for o2server readiness' CI step).")
        return 0
    seed_testadmin(token)
    seed_org_domain(token)
    if "--process-form" in sys.argv:
        seed_process_form(token)
    return 0


if __name__ == "__main__":
    sys.exit(main())
