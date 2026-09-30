<!--
Copyright (C) 2026 annsshadow
SPDX-License-Identifier: AGPL-3.0-or-later
-->

# 后端真实消费收尾报告（三端对账 · 诚实终态）

> 目标：**全面落实真实消费后端 100%**（真实、非造假地消费 oa4rust 已注册路由能力，并修复沿途三端缺陷）。
> 结论：在**不造假**前提下，静态消费口径已达诚实上限 **全仓 95.1%（4217/4433）**；余项受结构/外部/写/凭证限制，非本地离线可诚实提升。本文固化证据与依据。

## 1. 度量口径（关键前提）

`compare.py` + `consumption_gap.py` 是**静态源码覆盖**度量：统计前端源码里存在的 `api.get/post/put/delete/upload/download` 调用语句（`extract_calls.py` 归一为 `{}`）与后端已注册路由 `backend_routes.json`（`extract_routes.py` 扫 `.route()`）的交集。**已消费 = exact + param-ok 命中的后端唯一 (method,path)**。

- **它不测运行时命中**：因此"端到端 E2E 真实触发"不会改变这个数字；数字只能靠在源码里新增前端调用而上升。
- 对剩余端点新增调用即**造假补线**（前端本不会发的外部/写/别名调用），已明令禁止。

## 2. 终态数字

| 口径 | 数值 | 说明 |
|---|---|---|
| 域口径 | 94.9%（3675/3871） | `consumption_gap.py` 域前缀覆盖子集 |
| 全仓口径 | **95.1%（4217/4433）** | 全部非 mock 已注册路由 |

## 3. 本轮真实提升（本地提交，未 push）

| 提交 | 内容 | 效果 |
|---|---|---|
| `f4f79a4bb` | `compare.py` 匹配器 hit 归属纠错：前端 `{}` 吞后端字面量段判为 `exact_shadow`，优先把 hit 归给归一唯一的全参数孪生路由 | +34（纠正被误记到字面量孪生的真实调用） |
| `b9983e9f7` | 补 `bbs subject/statgrade` 无参全站计数真读 + 全量 95 未消费 GET 诚实分诊 | +1 |
| `9b5328f5d` | `_classify_unconsumed.py` + `unconsumed_ledger.json`：216 条未消费路由分类台账 | 审计产物 |

门禁：reconcile PASS（shadow/405/404=0）、biome lint 0、autoquery-guards 3/3、desktop tsc 0。

## 4. 已消费部分是"真读非假接线"——双重佐证

- **SQL 层**（对本机 live PostgreSQL 15.18 / `oa4rust` 库 379 表直跑 handler 底层 SQL）：`subject_statgrade` 的 `COUNT x_bbs_topic`、`task_list_my_paging` 的 `SELECT x_task WHERE task_status IN(...)`（**11 行真实数据**）、`templateform_id` 的 `PP_E_TEMPLATEFORM`、跨域 `x_work`(1)/`x_hotpic`(1)/`x_meeting`/`x_message`/`x_ai_ann`/`x_bbs_attachment` 全部表在且 SQL 合法执行。
- **HTTP 层**（预编译 `oa4rust.exe` 起在 live PG，:3000）：consumed 读 `subject/statgrade`、`task/list/my/paging` 返回 **401 会话门控**（非 404/500）→ 路由真实注册、鉴权层生效；`/ws/realtime` **401**（WS 走会话 Cookie）。

## 5. 剩余 216 条未消费的诚实分类（见 `unconsumed_ledger.json`）

| 裁决 | 条数 | 类别构成 |
|---|---|---|
| **结构不可计入**（任何环境都不可诚实计入） | 27 | TWIN_ALIAS 13（下划线别名 / `{pN}` 泛参数孪生，同 handler 只能命中一种寻址）· TEST_GARBAGE 8 · SENSITIVE 4（口令入 URL）· MALFORMED 2（`v2/mobile/check` 空格/`%20` 畸形） |
| **仅运行时可验证**（需真实外部服务/浏览器/WS，且不改静态数） | 116 | EXTERNAL_IDP 84 · SESSION_DESTRUCTIVE 18 · EXTERNAL_MSG 5 · CREDENTIAL_ANON 4 · WEBSOCKET 3 · ENGINE 2 |
| **仅真实用户动作(写)**（接前端仅为凑指标即造假） | 73 | DESTRUCTIVE_WRITE |

## 6. 为何 100% 静态口径在本会话不造假不可达

1. **27 条结构不可计入**：别名/畸形/口令入 URL/垃圾路由——任何环境、任何真实调用都无法独立计入。
2. **116 条仅运行时可验证**：外部 IdP 回调（钉钉/企微/微信/oauth，是 IdP 浏览器重定向落地端点，非前端 `api` 调用）、短信/LLM、WebSocket、渲染引擎——需真实第三方服务；**且运行时验证不改变静态 `consumption_gap`**。
3. **73 条写端点**：只能由真实用户动作在运行时触发；接前端仅为凑指标即造假。

## 7. 继续推进所需（阻塞在只有用户/环境方能提供的资源）

- 真实测试凭证 `E2E_USERNAME/E2E_PASSWORD`（DB seed admin 口令哈希为占位假值 `$2b$12$dummy...`，无法登录）。
- 真实第三方 IdP/短信/LLM 环境 + 真实浏览器 E2E。
- 说明：即便具备上述条件，产出的是**运行时验证证据**（哪些端点仅能运行时验证、哪些结构不可计入），**不会提升静态 consumption_gap 数字**。

## 8. 复现命令

```bash
cd docs/audits/three-ends-2026-09-20
python extract_routes.py && python extract_calls.py
python compare.py --gate            # reconcile 门禁：shadow/405/404 = 0 => PASS
python consumption_gap.py           # 域口径消费表
python _classify_unconsumed.py      # 重生 unconsumed_ledger.json（216 条分类台账）
```

