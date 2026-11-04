# O24-1 机械替换语义自证（3895 处 `.map_err(|_| AppError::Internal)` → 带 source）

> 结论先行：**同一 harness、同一数据库路径、同一份种子数据下，改前/改后 4355 个唯一路由键
> 完全一致；状态码除 9 条按设计的 500→503 外零漂移；没有 4xx→5xx，也没有 5xx→4xx；
> 同状态响应体差异 642 条全部落在 200 且全部是动态值；500/503 日志都带 `file:line` 与
> source 链。**
>
> 自证过程中发现 **1 处超出「仅改日志」的设计内行为变化**（`organization/core/express`
> 读取路径：有合法配置行时 500 → 200），按用户要求单独列为「发现」而不是自己裁定无影响，
> 见第 6 节。

## 1. 实验设置

| 项 | 值 |
|---|---|
| 改前 | git worktree，detached `e2e9beec5`（O23 第一步之前） |
| 改后 | 工作树 HEAD（含 O23 第一步机械替换） |
| harness | **同一份** `backend/tests/idor_matrix.rs`（`o24_contract_snapshot`，复制到改前 worktree） |
| 数据库 | 每侧独立 scratch 库，`create_app_with_sea_orm` + `create_sea_orm_pool_with_url(scratch)` |
| 种子 | `seed_all_tables`，**seeded=383 / seed_failures=0**（两侧相同） |
| 覆盖 | **executed=4372**（读 2393 / 写 1979），唯一 `(method, path)` **4355** |
| 归一化 | 响应 `"date":"..."` 统一为 `"date":"<NOW>"`，body 内 `\t\n` 统一为空格 |
| 落盘 | `routes.tsv`（method / path / status / body）、`errors.tsv`（该路由捕获到的日志行） |

复现（`<SNAP>` 为本机仓库外的快照目录，模板与命令如下；快照数据不入库）：

```powershell
# 每侧：dump 契约快照
$env:O24_DUMP_DIR="<before|after 目录>"; cargo test --test idor_matrix o24_contract_snapshot -- --nocapture
# 对比（脚本逻辑见本文件第 2/3/4 节；快照与脚本本身不入库）
python "$SNAP\compare.py"
python "$SNAP\samples.py" > "$SNAP\samples.md"
```

> 说明：`executed=4372` 是执行次数；`routes.tsv` 按 `(method, path)` 去重后为 4355
> （少数路由在扫描中被重复命中）。

## 2. 状态码不漂移

| 状态 | 改前 | 改后 | Δ |
|---|---|---|---|
| 200 | 3973 | 3973 | 0 |
| 307 | 1 | 1 | 0 |
| 400 | 202 | 202 | 0 |
| 401 | 4 | 4 | 0 |
| 403 | 1 | 1 | 0 |
| 404 | 66 | 66 | 0 |
| 405 | 1 | 1 | 0 |
| 422 | 49 | 49 | 0 |
| **500** | **51** | **42** | **−9** |
| **503** | **7** | **16** | **+9** |

- 路由集合：`only_before=0`、`only_after=0`（4355 对 4355）。
- 状态变化总数 **9**，**全部为 500→503**。
- 断言：**4xx→5xx = 0 条；5xx→4xx = 0 条**。除 500/503 外每个状态计数逐字相等。

9 条变化逐条（全部来自「缺配置」分支，`prompt` 仍为 `ExceptionInternal`，只有 `message`
按设计从内部措辞变成可诊断的缺项说明）：

| # | 路由 | 改前 message | 改后 message |
|---|---|---|---|
| 1 | `GET /api/andfx/moa/sso/token/{token}/enter/{enterId}` | `internal server error` | `service not configured: missing ANDFX_KEY` |
| 2 | `GET /api/mpweixin/bind/code/{code}` | 同上 | `service not configured: missing MPWEXIN_APP_ID` |
| 3 | `GET /api/mpweixin/login/code/{code}` | 同上 | `service not configured: missing MPWEXIN_APP_ID` |
| 4 | `GET /api/organization/core/express/config` | 同上 | `service not configured: missing x_org_config.sync_enabled` |
| 5 | `GET /api/organization/core/express/status` | 同上 | `service not configured: missing x_org_config.sync_enabled` |
| 6 | `GET /api/qiyeweixin/code/{code}` | 同上 | `service not configured: missing QYWX_CORP_ID` |
| 7 | `GET /api/qiyeweixin/update/person/detail/{code}` | 同上 | `service not configured: missing QYWX_CORP_ID` |
| 8 | `GET /api/welink/code/{code}` | 同上 | `service not configured: missing WELINK_APP_KEY` |
| 9 | `GET /api/zhengwudingding/code/{code}` | 同上 | `service not configured: missing ZWDINGDING_API_BASE` |

改前/改后完整响应体（以第 1 条为例，其余 8 条除 `message` 外逐字相同）：

```json
// 改前 500
{"count":0,"date":"<NOW>","message":"internal server error","position":0,
 "prompt":"com.x.base.core.project.exception.ExceptionInternal","size":-1,"spent":0,"type":"error"}
// 改后 503
{"count":0,"date":"<NOW>","message":"service not configured: missing ANDFX_KEY","position":0,
 "prompt":"com.x.base.core.project.exception.ExceptionInternal","size":-1,"spent":0,"type":"error"}
```

## 3. 响应契约对照（按错误类别抽样）

抽样脚本优先选「改前/改后 body 逐字相同」的端点；每类最多 3 条。

| 类别 | 该类路由总数 | 抽样 | 结果 |
|---|---|---|---|
| 400 | 202 | 3 | 逐字相同 |
| 401 | 4 | 3 | 逐字相同 |
| 403 | 1 | 1 | 逐字相同 |
| 404 | 66 | 3 | 逐字相同 |
| 409 | **0** | 0 | 本仓无 `AppError::Conflict` / `StatusCode::CONFLICT` 生产者，**不可达**；不伪造 |
| 500 | 42 | 3 | 逐字相同（改前 500 的 51 条中含 9 条已转 503，见上） |
| 503 | 7（两侧均为 503） | 3 | 逐字相同 |

逐字样本：

```text
400 DELETE /api/meeting/assemble/control/building/{id}
前 {"count":0,"date":"<NOW>","message":"bad request: building still referenced by 1 room(s)","position":0,"prompt":"com.x.base.core.project.exception.ExceptionBadRequest","size":-1,"spent":0,"type":"error"}
后 同上（逐字）
400 DELETE /api/meeting/assemble/control/meeting/{id}/delete/invite
前 {"count":0,"date":"<NOW>","message":"bad request: invitee is required","position":0,"prompt":"com.x.base.core.project.exception.ExceptionBadRequest","size":-1,"spent":0,"type":"error"}
后 同上（逐字）
400 GET /api/attendance/assemble/control/dingding/statistic/person/year/{year}/month/{month}
前 {"count":0,"date":"<NOW>","message":"bad request: year must be yyyy and month must be MM","position":0,"prompt":"com.x.base.core.project.exception.ExceptionBadRequest","size":-1,"spent":0,"type":"error"}
后 同上（逐字）

401 GET /api/processplatform/assemble/surface/application/list/complex/manage/person
前 {"count":0,"date":"<NOW>","message":"unauthorized","position":0,"prompt":"com.x.base.core.project.exception.ExceptionUnauthorized","size":-1,"spent":0,"type":"error"}
后 同上（逐字）
401 GET /api/processplatform/assemble/surface/application/list/complex/manage/{person}
前 同上（逐字）
401 POST /api/authentication/refresh
前 {"count":0,"date":"<NOW>","message":"unauthorized","position":0,"prompt":"com.x.base.core.project.exception.ExceptionUnauthorized","size":-1,"spent":0,"type":"error"}
后 同上（逐字）

403 GET /api/attendance/assemble/control/v2/appeal/{id}/start/check
前 {"count":0,"date":"<NOW>","message":"forbidden","position":0,"prompt":"com.x.base.core.project.exception.ExceptionAccessDenied","size":-1,"spent":0,"type":"error"}
后 同上（逐字）

404 DELETE /api/file/core/entity/folder/{id}
前 {"count":0,"date":"<NOW>","message":"not found","position":0,"prompt":"com.x.base.core.project.exception.ExceptionEntityNotExist","size":-1,"spent":0,"type":"error"}
后 同上（逐字）
404 DELETE /api/meeting/assemble/control/attachment/{id}
前 同上（逐字）
404 DELETE /api/organization/assemble/personal/empower/{id}
前 同上（逐字）

500 DELETE /api/organization/assemble/control/personattribute/{flag}
前 {"count":0,"date":"<NOW>","message":"internal server error","position":0,"prompt":"com.x.base.core.project.exception.ExceptionInternal","size":-1,"spent":0,"type":"error"}
后 同上（逐字）
500 DELETE /api/organization/assemble/control/unitattribute/{flag}
前 同上（逐字）
500 DELETE /api/portal/assemble/designer/script/{id}
前 同上（逐字）

503 GET /api/organization/assemble/authentication/dingding/bind/{meta}/callback/{code}
前 {"count":0,"date":"<NOW>","message":"service unavailable: 钉钉未配置（DINGDING_APP_KEY/SECRET 缺失）","position":0,"prompt":"com.x.base.core.project.exception.ExceptionInternal","size":-1,"spent":0,"type":"error"}
后 同上（逐字）
503 GET /api/organization/assemble/authentication/dingding/code/{code}
前 同上（逐字）
503 GET /api/organization/assemble/authentication/dingding/login/{code}
前 同上（逐字）
```

**结论**：4xx 全部逐字不变；500/503 的对外措辞只按设计变化（内部细节只进日志，不进响应体）。

## 4. 同状态响应体差异：642 条，全部是 200 且全部是动态值

`compare.json`：`same-status body differences = 642`，状态分布 **{200: 642}**，即
**4xx/5xx 同状态之间零差异**。

严格校验（对 641 条 JSON 响应）：

- 顶层 key 集合：**641/641 相同**；
- `type` / `message` / `prompt`：**641/641 逐字相同**；
- `count` / `size` / `position` / `spent`：**641/641 相同**；
- 1 条非 JSON（SSE `POST /api/ai_assemble_control/chat/completion/stream`），差异仅为
  `conversationId`（UUID），`event: token` / `token` 文案逐字相同。

对差异值做动态掩码（UUID、ISO/带小数与时区的时间戳）后，635 条完全一致；剩余 7 条逐条
检查，差异来源均为运行时动态值，不涉及契约字段：

| 路由 | 差异字段 | 性质 |
|---|---|---|
| `GET /api/authentication/captcha`、`.../captcha/width/{w}/height/{h}`、`.../authentication/captcha/...`、`.../personal/regist/captcha/...` | `captchaId` + base64 图片 | 每次随机生成验证码 |
| `GET /api/organization/assemble/authentication/sso/encrypt/...` | `timestamp`(ms) + 加密 `token` | 时间戳参与加密 |
| `POST /api/program_center/code/list/paging/...`、`.../script/list/paging/...` | 行 `flag` / `id` / `createTime` | 同一扫描内先执行的写路由写入的随机行 |
| `POST /api/restore/upload` | `id` + `stamp` | 随机 UUID + 时间戳 |

## 5. source 保全自证（真实日志行）

两条均取自 `o24-snap/after/errors.tsv` 的实际捕获，不是拼接示例。

**真实 500（缺列类型对齐）** —— `file:line` 与完整 source 链都在：

```text
2026-11-04T20:10:44.594845Z ERROR shared::error: 5xx response method=GET
path=/api/meeting/core/entity/room/list trace_id=ac4bf6f6-83de-43b5-b735-2e5077ae904c
status=500 error=internal server error source=crates\meeting_core_entity\src\lib.rs:86:
Query(SqlxError(ColumnDecode { index: "\"equipment\"", source: "mismatched types; Rust type
`core::option::Option<alloc::string::String>` (as SQL type `TEXT`) is not compatible with SQL
type `JSONB`" }))
```

**真实 503（缺配置）** —— `#[track_caller]` 让 `MissingConfig` 带调用点：

```text
2026-11-04T20:10:42.832007Z WARN shared::error: 5xx response method=GET
path=/api/andfx/moa/sso/token/1/enter/1 trace_id=9fa6e637-748f-4b8b-9d4c-bf153b2cd141
status=503 error=service not configured: missing ANDFX_KEY source=crates\auth\src\andfx.rs:56
```

两行都证明了：响应体只给 `internal server error` / `service not configured: missing X`，
但日志里有 `method` / `path` / `trace_id` / `status` / `source`，且 source 是
`file:line` + 底层错误链，而不是只有 `Internal`。

## 6. 发现：1 处超出「仅改日志」的设计内行为变化（先报告，不自行裁定）

`organization_core_express` 的两条路由（`/config`、`/status`）不只是「缺配置改 503」：

```rust
// 改前：config_value 是 TEXT，直接按 bool 解码 → 有行且值合法时也会 500（mismatched types）
let enabled: bool = client.query_one(...).await?.get("config_value");

// 改后：按 String 读取 + parse::<bool>()
let raw: String = row.get("config_value");
raw.parse::<bool>().map_err(...)
```

因此这条改动在「`sync_enabled` 行存在且值为 `true`/`false`」时是 **500 → 200**，而不是
500 → 503。本次扫描的种子库里没有 `sync_enabled` 行，所以观测到的是 503 分支，扫不到
这条分支差异。

**判定**：这属于 O23 第一步台账里已写明的设计（`config_value` 是 TEXT，必须改读取方式；
坏值 → 500 并带原始值）。但它确实是一次成功路径上的行为变化，不是单纯换日志措辞。
按用户要求在此单列，**不把它当作「应该没影响」略过**；后续若需要，应补一条有
`sync_enabled` 行的集成用例把 200 分支钉住（当前没有这样的用例）。

其余 3895 处替换：只改变错误对象的构造，`InternalWithSource` 仍映射 500、响应体仍为
`internal server error`，所以状态码与响应体都不变——上表已逐类证明。

## 7. missing_config 逐处（用户口径 12 → 实际 14）

全仓生产代码里 `AppError::missing_config(` 共 **14 处**（另有定义 1 处、单测 1 处不计）。
差异说明：用户提到的「12 处」与 O23 台账里早期统计一致；O23 第一步后续把
`ANDFX_ENTER_ID`（`andfx.rs:58`）和 `ZWDINGDING_APP_ACCESS_TOKEN`
（`zhengwudingding.rs:45`）也纳入了缺配置分支，实际落点为 14。**按实际 14 处逐条列**：

| # | 位置 | 缺项 | 改前 | 改后 |
|---|---|---|---|---|
| 1 | `crates/auth/src/andfx.rs:56` | `ANDFX_KEY` | `map_err(\|_\| AppError::Internal)` | `map_err(\|_\| AppError::missing_config("ANDFX_KEY"))` |
| 2 | `crates/auth/src/andfx.rs:58` | `ANDFX_ENTER_ID` | 同上 | `missing_config("ANDFX_ENTER_ID")` |
| 3 | `crates/auth/src/qiyeweixin.rs:24` | `QYWX_CORP_ID` | `Option` + 调用点 `ok_or(AppError::Internal)` | `Result` + `missing_config("QYWX_CORP_ID")`，调用点改为 `?` |
| 4 | `crates/auth/src/qiyeweixin.rs:26` | `QYWX_APP_SECRET` | 同上 | `missing_config("QYWX_APP_SECRET")` |
| 5 | `crates/auth/src/oauth.rs:515` | `QYWX_CORP_ID/QYWX_APP_SECRET` | `qywx_config().ok_or(AppError::Internal)` | `ok_or_else(\|\| missing_config("QYWX_CORP_ID/QYWX_APP_SECRET"))` |
| 6 | `crates/auth/src/oauth.rs:518` | `DINGDING_APP_KEY/DINGDING_APP_SECRET` | 同上 | `missing_config("DINGDING_APP_KEY/DINGDING_APP_SECRET")` |
| 7 | `crates/auth/src/mpweixin.rs:119` | `MPWEXIN_APP_ID` | `map_err(\|_\| AppError::Internal)` | `missing_config("MPWEXIN_APP_ID")` |
| 8 | `crates/auth/src/mpweixin.rs:121` | `MPWEXIN_APP_SECRET` | 同上 | `missing_config("MPWEXIN_APP_SECRET")` |
| 9 | `crates/auth/src/welink.rs:51` | `WELINK_APP_KEY` | `Option` + `ok_or(Internal)` | `Result` + `missing_config("WELINK_APP_KEY")` |
| 10 | `crates/auth/src/welink.rs:53` | `WELINK_APP_SECRET` | 同上 | `missing_config("WELINK_APP_SECRET")` |
| 11 | `crates/auth/src/zhengwudingding.rs:41` | `ZWDINGDING_API_BASE` | `map_err(\|_\| AppError::Internal)` | `missing_config("ZWDINGDING_API_BASE")` |
| 12 | `crates/auth/src/zhengwudingding.rs:43` | `ZWDINGDING_CORP_ACCESS_TOKEN` | 同上 | `missing_config("ZWDINGDING_CORP_ACCESS_TOKEN")` |
| 13 | `crates/auth/src/zhengwudingding.rs:45` | `ZWDINGDING_APP_ACCESS_TOKEN` | 同上 | `missing_config("ZWDINGDING_APP_ACCESS_TOKEN")` |
| 14 | `crates/organization_core_express/src/lib.rs:22` | `x_org_config.sync_enabled` | `query_one(...).get::<bool>()`（缺行 500 / 类型错配 500） | 缺行 → `missing_config(...)`；有行 → String + `parse::<bool>` |

**为什么 1–13 不是行为变更**：这些分支只在「环境变量不存在」时触发。变量存在时，
请求走的是完全相同的下游代码（同样的 HTTP 调用、同样的 SQL、同样的响应体），状态码不变。
变量不存在时，改前已经是 5xx 失败（`internal server error`），改后仍是 5xx 失败，只是
状态码从 500 细化为 503、message 写明缺哪一项——**失败语义没有从成功变失败，也没有从
可重试变成不可重试**。第 14 条额外包含成功路径修复，已单列在第 6 节。

响应契约对照（本节 13 条相关路由）已在上文 9 条 500→503 中逐条给出：`prompt` 不变，
只有 `message` 变化，且这正是 O23 的设计目标（运维能直接看出是部署缺配置）。

## 8. 自证过程中修正的 3 个探针缺陷（否则上面的数字不可信）

1. **库隔离**：原 `create_app` 用进程级 `DATABASE_URL` 建 SeaORM 连接，ORM 路由读的是
   本机开发库而不是 scratch 种子库 → 基线随开发库垃圾行漂移（本地 41 / CI 40）。
   改为 `create_app_with_sea_orm` + scratch URL 后，`meeting_core_entity` 两条 ORM 路由
   首次读到种子行，暴露 `column:equipment` 500 → 基线 **40 → 42**（GET 11 / 非 GET 31）。
2. **日志缺 `file:line`**：`IntoResponse` 的 5xx 日志最初只打 `source=cause`，`context`
   没进日志。已改为 `source="{context}: {cause}"`；`MissingConfig` 加 `#[track_caller]`
   带调用点。第 5 节两条日志即修复后的实际输出。
3. **classify_error 漏识别 `ColumnDecode`**：sqlx 的
   `ColumnDecode { index: "\"equipment\"" }` 不含 `error retrieving column` 字样，
   被误分类为 `no-error-log`。已补提取分支，两条 meeting 路由才能对上
   `column:equipment`，棘轮集合相等。

这三项都是「对照发现异常 → 先停下查 → 修复探针后重测」，不是为了让数字变绿而放宽断言。

## 9. 局限

- **409 不可达**：本仓没有 409 生产者，无法抽样，也不伪造样本。
- 动态值掩码只覆盖 UUID / ISO 与 SQL 风格时间戳；7 条剩余差异已逐条人工核对（第 4 节）。
- 快照只覆盖种子化扫描能执行到的路由（读 2393 / 写 1979）；未注册或扫描不执行的代码路径
  不在此证据范围内。
- 「无状态漂移」是**本次扫描集合内**的结论；新增路由后需重跑同一脚本再断言。
