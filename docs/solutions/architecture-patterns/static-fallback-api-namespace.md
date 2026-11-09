---
title: "SPA 静态兜底不能吞掉 API 命名空间：Mount('/') 的 404/405 漂移（A361① / L296）"
date: 2026-11-09
last_refreshed: 2026-11-09
category: architecture-patterns
module: augmentor/api/main.py
problem_type: bug_pattern
component: static_fallback_routing
severity: medium
symptoms:
  - "构建过前端（web/dist 在场）时，POST /api/dataset/filter 等不存在的 API 路径回 405；干净克隆回 404 —— 同一份代码两种回执"
  - "未知 API 路径拿到 Method Not Allowed，客户端把「端点拼错」误读成「方法不对」"
  - "守卫若把 405 抄成常数，会在没有 web/dist 的 CI 上自己变红；按 app.routes 现算又会让判据跟着部署形状走"
root_cause: static_spa_fallback_mounted_without_namespace_boundary
resolution_type: code_fix_with_guard
tags: [static-files, spa-fallback, mount, api-namespace, 404-vs-405, starlette, routing]
applies_when:
  - "SPA / 静态站点兜底挂在 `/`，同一进程还提供 `/api` 服务"
  - "需要区分「路径不存在」（404）与「路径存在但方法不对」（405）"
  - "部署产物（web/dist）在开发者机器与 CI 上不一致，导致回执读数漂移"
related_components:
  - augmentor/api/main.py
  - augmentor/tests/unit/test_static_mount_api_scope_l296.py
  - augmentor/tests/unit/test_dataset_text_field_l274.py
  - augmentor/tests/unit/test_backup_name_path_segment_l277.py
---

# SPA 静态兜底不能吞掉 API 命名空间：`Mount('/')` 的 404/405 漂移

## Problem

`api/main.py` 在前端构建产物存在时，把 `StaticFiles(html=True)` 挂在 `/`：

```python
if _static_dir.is_dir():
    app.mount("/", StaticFiles(directory=str(_static_dir), html=True), name="static")
```

这只挂载的职责是给 SPA 的非 API 路径做兜底（`/` 的 index、`/assets/*`、前端路由），但它
**对所有没被前面路由命中的路径生效**，包括 `/api/...`。`StaticFiles` 对非 GET 请求一律回
**405 Method Not Allowed**。于是：

- 有 `web/dist`：`POST /api/dataset/filter`、`POST /api/definitely-not-a-path-xyz` ⇒ **405**
- 没有 `web/dist`：同一批请求 ⇒ 路由表自己的 **404**

同一份代码、同一个请求，回执随「这台机器有没有构建过前端」漂移。对客户端来说，拼错端点名
收到 405 会把排查方向引到「方法用错了」，而不是「这个端点不存在」。

## Root Cause

**静态兜底在匹配阶段没有命名空间边界。** Starlette 的 `Mount` 按路径前缀匹配，`Mount("/")`
等价于「兜住一切」；它一旦匹配成功，后面的路由表就没有机会给出「无匹配」的 404。`StaticFiles`
再把「非 GET」翻译成 405，两个本来独立的判决叠在一起，产出误导性回执。

这不是 `StaticFiles` 的缺陷，也不是 `Mount` 的缺陷：SPA 兜底本来就只应服务非 API 路径，
缺的是那条**命名空间判据**。

## Resolution

在**匹配阶段**拒绝 `/api` 命名空间，让路由表重新成为 API 面的唯一权威：

```python
from starlette.routing import Match, Mount, get_route_path


class _NonApiStaticMount(Mount):
    def matches(self, scope):
        if scope["type"] in ("http", "websocket"):
            path = get_route_path(scope)
            if path == "/api" or path.startswith("/api/"):
                return Match.NONE, {}
        return super().matches(scope)


if _static_dir.is_dir():
    # app.mount() 内部固定构造 Mount，收不了子类 ⇒ 直接往路由表尾部追加。
    app.router.routes.append(
        _NonApiStaticMount(
            "/",
            app=StaticFiles(directory=str(_static_dir), html=True),
            name="static",
        )
    )
```

三个容易写错的点：

1. **用 `get_route_path(scope)` 而不是 `scope["path"]`**：应用挂在 `root_path` 下时，`path`
   仍带前缀；`get_route_path` 先减掉 `root_path`，判据才不会漏。
2. **判据是 `/api` 整段，不是裸 `startswith("/api")`**：后者会把 `/apix`、`/apiary` 也划进
   API 面，误伤静态路径。`path == "/api" or path.startswith("/api/")` 只认这一段。
3. **`app.mount()` 收不了子类**：`FastAPI.mount` → `Router.mount` 内部固定写 `Mount(...)`。
   要装子类只能按它的实现往 `app.router.routes` 尾部追加，位置与 `app.mount` 等价。

行为矩阵（改后实测）：

| 请求 | 回执 |
|---|---|
| 未知 `/api/...`，GET / POST / PUT / PATCH / DELETE | **404**（路由表） |
| `/api` 根 | **404** |
| `POST` / `DELETE /api/health` | **405**（已知路径、方法不对） |
| `GET /api/system/backups/unknown/restore` | **405**（路由表部分匹配） |
| `GET /api/health` | **200** |
| 非 API 未知路径 GET / POST | **404 / 405**（`StaticFiles` 自己的判决，保持不变） |
| `GET /` | **200** |
| `/apix`、`/apiary` | 仍归静态（命名空间边界不误伤） |

## Guard and Evidence

新增 `augmentor/tests/unit/test_static_mount_api_scope_l296.py` 共 12 支、零 `parametrize`，四面：

1. **真实装配**（`api.main.app`）：未知 API 五法全 404、`/api` 根 404、已知路径错方法 405、
   合法请求 200、非 API 未知 GET 404。
2. **临时静态目录**：不依赖本机有没有 `web/dist`，直接实例化 `_NonApiStaticMount` + 真
   `StaticFiles`，在任意干净克隆上量挂载本身的行为。
3. **`matches()` 命名空间边界**：`/api`、`/api/`、`/api/x` 落空；`/`、`/apix`、`/apiary`、
   `/assets/app.js` 仍匹配；`root_path` 先减再判。
4. **AST 接线**：钉住 `app.router.routes.append(_NonApiStaticMount(...))`，且全文无普通
   `Mount(...)` 调用。初版只查类名，被注入臂 ① 骗过（类还在、装配退回普通 `Mount`）后改成
   AST 钉追加实参。

两臂注入取证（改完逐字节复原，前后 sha256 一致）：

| 注入 | 读数 | 点名 |
|---|---|---|
| 装配退回普通 `Mount` | `4 failed, 8 passed` | `TestTheRealAssembly` 三支 + AST 接线支 |
| 类内摘掉 `/api` 拒绝 | `6 failed, 6 passed` | `TestTheRealAssembly` 三支 + `TestTheMountItself` 未知 API 支 + `TestTheNamespaceBoundary` 两支 |

旧守卫同步收口（它们此前按 `app.routes` 现算 405/404，现在常数 404 才是对的）：

- `test_dataset_text_field_l274.py`：`PHANTOM_CODE = "404"`，删 `STATIC_MOUNTED`。
- `test_backup_name_path_segment_l277.py`：删 `_phantom_code`，`PHANTOM_CODE = 404`；另补
  裸 `#` 截断后的 restore 实测 **405**（截断路径 `/frag` 撞上已知的 `DELETE …/{backup_id}`，
  属路由表部分匹配，不是静态挂载代劳），并断言未落 `out.json`。
- `test_backup_wiring_l231.py`：注释同步 405 的来源。
- `test_serve_config_wiring_l97.py`：静态挂载点收集从 `type(r).__name__ == "Mount"` 改成
  `isinstance(r, Mount)`。子类名不再等于 `Mount`，全量首跑当场红两支
  （`test_absolute_dir_is_mounted` / `test_relative_dir_anchors_at_repo_root`），修后 7 支全绿。

定向复跑 `136 passed`（五文件批，收集面 12 + 41 + 41 + 35 + 7）。

全量定标：`38 failed, 10235 passed, 6 skipped, 688.56s`，覆盖率 `99.62%`。首跑多出的
l97 两支（`test_absolute_dir_is_mounted` / `test_relative_dir_anchors_at_repo_root`）已修；
其余 38 项红全部落在既有基线族（端点计数 / 门禁 / TS 契约 / l177 语料 / a184 注释引用 /
l200 / l210 / l224 / l233 / l251 / l252 / l265 等），未声称全绿。

## Lessons

1. **命名空间边界属于匹配阶段**：返回 `Match.NONE` 让路由表继续判决，比事后改状态码干净；
   404 与 405 的语义都保住了。
2. **部署产物不该改变 API 契约**：有 `web/dist` 与没有它必须给同一张回执；守卫要能在两种
   环境各量一遍，临时静态目录臂就是为此。
3. **`root_path` 要先减再判**：直接读 `scope["path"]` 会在反向代理前缀下漏判。
4. **边界要按路径段判**：裸 `startswith("/api")` 会误伤 `/apix` / `/apiary`。
5. **守卫的牙齿要对着接线**：只断言「类存在」会被「类还在、但装配没用它」骗过；AST 钉住
   追加调用的实参才可靠。
6. **全量定标会抓出守卫里的类型过滤器**：凡是 `type(r).__name__ == "Mount"` 这种按类名写死
   的收集，都必须改成 `isinstance(r, Mount)`，否则新增子类会让旧守卫静默漏掉挂载点。

## References

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L296 清单行与日志块，A361 行（`~~S~~ **已收口（L296）**`）
- 提交：`e6b0cffaf`（`fix,test,docs(augmentor): L296 A361① 静态兜底只接非 API 命名空间`）
- 守卫：`augmentor/tests/unit/test_static_mount_api_scope_l296.py`
- 旧守卫同步：`augmentor/tests/unit/test_dataset_text_field_l274.py`、`augmentor/tests/unit/test_backup_name_path_segment_l277.py`、`augmentor/tests/unit/test_backup_wiring_l231.py`
