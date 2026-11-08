# 行为级缺陷审计 · 第二轮（只读）

- 审计对象：`oa4rust/backend`（Rust/Axum）+ `laiyipao/server`（Go/Fiber）
- 日期：2026-11-05
- 边界：未改业务代码/迁移/配置，未 `git add/commit/push`；仅在 scratch 库 `oa_audit_20261105`（Rust）与 `laiyipao_audit_2`（Go）造数。
- 方法：`--migrate-only` 生成真实 schema → 按 schema 造有区分度数据 → 真实进程（Rust `:3999`、Go `:3998`）+ 真实会话（普通用户 `audit-user` / 管理员 `admin`；Go 玩家 `audit_u1` / 管理员 `audit-admin`）直连 curl。
- 证据文件（同目录）：`evidence-rust-round2*.txt`、`evidence-go-round2-v2.txt`、`fake-pagination-inventory.txt`、`pagination-handlers-full.txt`（275 个含 count 形参的函数）、`pagination-routes-full.txt`（242 个含 `{count}/{size}` 的不同 handler）、`ownership-list-candidates.txt`（389 个归属可疑列表的静态扫描）。
- **本轮开始时遇到 403，已定位为 CSRF 中间件（写方法需 `Origin` 白名单），不是权限拒绝**；补齐 `Origin: http://127.0.0.1:3999` 后全部复现。方法学教训写在最后一节，避免下一轮被同一个 403 误导。

---

## 0. 结论与优先级

| 序 | 编号 | 结论 | 定性 | 严重度 |
|---|---|---|---|---|
| 1 | F2-E | 14 个列表端点条目（13 类，E1/E1b 同族考勤）可用普通用户读到他人数据（考勤/待办/消息/会议/AI/文档/mind/流程） | 已实证 | 严重 |
| 2 | F1-A | processplatform 85 个 `next/prev` 把游标 id 当等值过滤，`count` 完全不用 | 已实证（1 个）+ 代码确认（85 个） | 高 |
| 3 | F1-B | CMS 19 个 `{id}/next/{count}` handler 根本不接收 Path，cursor/count 全忽略 | 已实证 | 高 |
| 4 | F1-C | file 4 个 next/prev 固定 `LIMIT 20`，`count=3` 返回 20 | 已实证 | 中 |
| 5 | G1 | Go `/admin/users` 用未 clamp 的 `limit` 判断 `hasMore`，`limit≥201` 与 `limit≤0` 时 `next_cursor` 为空，用户不可达 | 已实证 | 高 |
| 6 | F8 | AI 三表 `create_time` 为 NULL 时列表 500（`row.get::<_, String>` 解码裸列） | 已实证 | 中 |
| 7 | G3 | `/me/battles` `ORDER BY created_at DESC` 无 id tiebreak，期间插入新行导致跨页重复 | 已实证 | 中 |
| 8 | G2 | 用户存在但 `user_wallets` 无行时，`/admin/users/{id}` 整个详情 404（错误串把内部错误当不存在） | 已实证 | 中 |
| 9 | F1-D | 部分 `next/prev` 实为 offset/无游标（organization、message consume、query），`count` 只作 LIMIT | 已实证（message）+ 代码确认 | 中 |
| 10 | F1-E | `count` 字段返回本页行数而非总数（attendance/query/program_center/message…） | 已实证 | 低-中 |

---

## 1. 「假翻页」家族（全量枚举）

### 1.0 口径

- 全仓 `routes.rs` 中含 `{count}` 或 `{size}` 的路由共 **345 行**，去重后 **242 个不同 handler**（`pagination-routes-full.txt`）。
- 其中形参里带 `count` 的函数 **275 个**（`pagination-handlers-full.txt`，含非路由的 helper）。
- 扫描脚本的分类标记是**启发式**（`EQ_FILTER_$1` 99 个里含 14 个把业务过滤列当游标的误报），下文按人工核对的真实形态分组。

### 1.1 形态 A：游标 id 当等值过滤（最严重的一类）

**实际行为**：SQL 形如 `WHERE <pk> = $1 ORDER BY ...`，`$1` 是"上一页最后一条 id"，但被当作等值查询；`count` 形参名常写成 `_count`，在函数体内从未出现；结果永远只有游标行本身（或 0 行），`next`/`prev` 没有翻页语义。

**已实证**（`evidence-rust-round2-summary.txt`）：

```
draft next count=10 (期望10) | GET .../draft/list/audit-d1/next/10 | HTTP 200 | dataLen=1 countField=1 ids=[audit-d1]
draft prev count=10 (期望10) | GET .../draft/list/audit-d1/prev/10 | HTTP 200 | dataLen=1 countField=1 ids=[audit-d1]
```

**代码**：`processplatform_assemble_surface/src/lib.rs:7410`（`draft_list_id_next_count`）：

```sql
SELECT ... FROM PP_C_DRAFT WHERE xid = $1 ORDER BY "xcreateTime" DESC
```

`Path((id, _count))`，`count` 未使用。同文件 `read_list_id_next_count`（`:10252`）完全相同：`... FROM PP_C_READ WHERE xid = $1 ORDER BY "xcreateTime" DESC`。

**全量**：该形态共 **85 个 handler，全部在 `processplatform_assemble_surface`**（约占该 crate 92 个 Path 端点的 92%）。清单见 `pagination-handlers-full.txt` 中 `EQ_FILTER_$1` 且 crate 为 `processplatform_assemble_surface` 的行（`file:line` 已列）。文件名/函数名统一为 `*_list_id_next_count` / `*_list_id_prev_count`，模式一致，无需逐条实测即可判定（抽样 3 个代码完全同构）。

**触发条件**：任意已登录用户对任一 `/{id}/next/{count}` 或 `/{id}/prev/{count}` 发请求。
**建议判据**：同一资源插入 ≥3 行后，`next/{count}` 返回**游标之后**的 `count` 行且不含游标行；`prev` 返回游标之前 `count` 行；`count=0` 返回空。现实现恒返回 1 行（游标行）或 0 行。
**严重度**：高（所有"下一页"按钮实际不可用；前端会表现为"翻页无反应"）。

### 1.2 形态 B：路由带 `{id}/{count}`，handler 完全不接收 Path

**实际行为**：handler 签名只有 `Extension<Pool>`，`id` 与 `count` 根本没读；内部调用"整表/整 app 列表 + 自己解析的 page/size"，因此 `next/prev` 与 `page/size` 返回同一批数据。

**已实证**（`evidence-rust-round2-summary.txt`）：

```
cms appinfo next count=1 (期望1) | PUT /api/appinfo/filter/list/zzzz/next/1 | HTTP 200 | dataLen=4 countField=4 ids=[seed-app,audit-app-1,audit-app-2,audit-app-3]
cms appinfo prev count=1 (期望1) | PUT /api/appinfo/filter/list/zzzz/prev/1 | HTTP 200 | dataLen=4 countField=4 ids=[seed-app,audit-app-1,audit-app-2,audit-app-3]
cms document next count=1 (期望1)  | PUT /api/document/filter/list/zzzz/next/1  | HTTP 200 | dataLen=1 countField=1 ids=[doc-draft-1]
cms document page/size 1/10       | PUT /api/document/filter/list/1/size/10       | HTTP 200 | dataLen=1 countField=1 ids=[doc-draft-1]
```

**代码**（`cms_assemble_control/src/lib.rs`）：
- `:794 appinfo_filter_list_id_next_count`、`:810 appinfo_filter_list_id_prev_count`
- `:625 anonymous_document_filter_list_id_next_count`
- `:11086 document_filter_next_u3`、`:11100 document_filter_prev_u3`、`:11114 document_filter_paging_u3`
- 以上函数体均无 `Path` 解构；`appinfo` 走"取全表再按解析出来的 count 截断"，`document` 走 `document_filter_paging_u3`，两者对 `{id}`/`{count}` 都无感。

**全量（19 个 next/prev handler，从 `pagination-routes-full.txt` 的 `PATH=False` 中筛出）**：
`anonymous(next/prev)`、`appinfo(next/prev)`、`categoryinfo(next/prev)`、`comment(next/prev)`、`file(next/prev)`、`log(next/prev)`、`script(next/prev)`、`view_viewdata(next)`、`viewrecord_document(next)`、`document_draft(next)`、`document_filter(next/prev)`（共 19 个）。另有约 10 个 `page/size` 同类有效 handler（`anonymous/categoryinfo/commend/design_appdict/document_cipher/log_list/script_list/viewrecord_install_log/document_filter_paging` 及其 `mockputtopost` 别名；`image_encode` 的 `size` 是 base64 编码尺寸，不是分页，剔除）。**上一轮 F6 报的 13 个是这一个家族的子集，实际 19 个。**

**建议判据**：`next` 必须用 `{id}` 做 keyset（`id > $cursor` 或 `create_time < last_time`），且返回行数 ≤ `{count}`；`prev` 反向。现实现 `next`/`prev`/`page` 三路返回集合相同。
**严重度**：高（接口语义整体错位；前端把 `next` 当普通"列表刷新"用时会重复加载同一页）。

### 1.3 形态 C：固定 LIMIT，忽略 count

**已实证**：

```
file next count=3   | GET /api/file/assemble/control/file/list/audit-file-001/next/3   | dataLen=20 countField=20
file prev count=3   | GET /api/file/assemble/control/file/list/zzzz/prev/3             | dataLen=20 countField=20
file next count=100 | GET /api/file/assemble/control/file/list/audit-file-001/next/100 | dataLen=20 countField=20
```

**代码**：`file_assemble_control/src/lib.rs:3108`（`:3116`）与 `:3247`（`:3255`），SQL 末尾硬编码 `ORDER BY id LIMIT 20`；同文件 `:1052`、`:1795` 的 `page/size` 列表同样 `LIMIT 20`。附带说明：**file 的归属过滤是正确的**（`owner_clause` 对非管理员按 `person` 过滤），本轮未发现 file 越权。

**全量**：4 个 next/prev + 2 个 page/size（file），以及 `mind` 的 `LIMIT 20`（见 2.2）。
**建议判据**：`count=3` 返回 ≤3 行；`count=100` 返回 ≤100 行（若服务端有上限 20，应显式 clamp 并在响应里体现，而不是静默忽略入参）。
**严重度**：中（请求参数失效，客户端无法按需取数；`count` 语义被静默改写）。

### 1.4 形态 D：`next/prev` 名不副实（offset / 无游标 / 只当 LIMIT）

| crate | file:line | 实际行为 |
|---|---|---|
| organization_assemble_control | `src/lib.rs:159 organization_assemble_control_unit_list_flag_next_count` | `where` 只有 `flag/parent` 业务过滤，`count` 直接作 `LIMIT`，没有游标 → 每次"next"都返回第一页 |
| organization_assemble_control | `src/u2_org.rs:226 unit_list_prev` | 同上，`prev` 与 `next` 无区别 |
| message_assemble_communicate | `src/lib.rs:237/289/348 consume_list_*` | `WHERE consume = $1 ... LIMIT $count`；`consume` 是**业务过滤**（可随路径任意传），没有游标 |
| query_assemble_designer | `src/lib.rs:3027/3096 table_list_tableFlag_row_id_next/prev` | `WHERE table_flag=$1 AND id > $2` 正确 keyset，但 `:2756 table_export...` 把 count 当纯 LIMIT |
| message/src/lib.rs | `:50 consume_list` | 同上，无游标 |

**已实证**（message 的"等值过滤可任意指定"同时是越权点）：

```
message consume=person-admin count=5 (普通用户) | GET .../consume/list/person-admin/count/5 | dataLen=1 ids=[msg-admin-1]
```

**建议判据**：`next` 必须包含 keyset 谓词；`prev` 必须与 `next` 的排序方向相反；`count` 只能作为 `LIMIT` 上限，不得替代游标。
**严重度**：中（不会崩，但分页永远停在第一页；配合 1.5 的 count=len 会误导前端"已到底")。

### 1.5 `count` 字段 = 本页行数，不是总数

已实证：`draft next` 返回 `countField=1`（库里 3 行）；`file next` 返回 `countField=20`；`mind` 返回 `countField=2`（2 行）。代码上是 `let count = data.len() as i64;`（如 `processplatform.../lib.rs` 的 `*_list_id_next_count` 末尾，`query_assemble_surface/src/lib.rs:2068`，`message` 各 `consume_list_*`）。

**建议判据**：响应 `count` 应等于满足过滤条件的总行数（与 `LIMIT` 无关）；分页信息另开 `next`/`hasMore`/`total` 字段。现实现把"本页条数"当"总数"，前端用它算页数必然错。
**严重度**：低-中（数据不丢，但页数/进度错误）。

### 1.6 全量分布（静态）

| crate | 含 count/size 的 handler | 纪要 |
|---|---:|---|
| processplatform_assemble_surface | 92 | 85 个等值游标 + 少量 `count=len` |
| attendance_assemble_control | 19 | keyset 实现正确（shared::db::keyset），但 `count=len`；多个列表无归属过滤（见 2） |
| program_center | 16 | keyset 正确，`count=len` |
| file_assemble_control | 8 | 固定 `LIMIT 20` |
| cms_assemble_control | 19（next/prev，无 Path）+ 4（Path，form 系列）+ 10（page/size 同类） | 见 1.2 |
| message_assemble_communicate | 4 | 无游标，`consume` 当过滤 |
| query_assemble_designer / surface | 4 / 2 | keyset 正确，`count=len` |
| personal | 4 | `log_list` 用子查询 `created_at` 做游标（可用，但无 id tiebreak） |
| meeting_assemble_control | 3 | `u2_attachment_paged` 子查询 keyset（正确，缺 tiebreak） |
| portal_assemble_surface | 1 | 代码确认无 keyset |
| processplatform_assemble_designer | 4 | 分页正确（keyset） |

> 说明：`pagination-handlers-full.txt` 共 275 行，含 helper 与误报；`pagination-routes-full.txt` 的 242 行是"路由→handler"的真清单。两份都作为附件，便于修复线逐条勾销。

---

## 2. 「列表无归属过滤」家族（F2 家族化）

### 2.1 已实证：普通用户 `audit-user` 读到 `person-admin` 的数据

所有输出摘自 `evidence-rust-round2-summary.txt` / `evidence-rust-round2-more.txt`，均为 HTTP 200。

| # | 端点 | 实测返回（节选） | 代码 | 问题 |
|---|---|---|---|---|
| E1 | `GET /api/attendance/assemble/control/attendancedetail/filter/list`（无分页） | `dataLen=21 personIds=[someone-else,p1] count=21` | `attendance_assemble_control/src/lib.rs:1201` | 无 `WHERE`、无 `Session`、无 `LIMIT`，**返回全表** |
| E1b | `GET .../attendancedetail/filter/list/0/next/5` | `dataLen=5 personIds=[p1]`（会话人 `person=audit-u1`） | `:1401` | keyset 与 `count` 正确，但无 `person` 过滤 → 仍读他人 |
| E2 | `GET /api/attendance/.../attendancedetail/mobile/filter/list/page/1/count/5` | `dataLen=5 ids=[audit-other-1,audit-ad-03,...]`（含 `personId=someone-else`） | `:1604` | offset 分页无 `person` 过滤 |
| E3 | `PUT /api/attendance/.../attendanceappealInfo/filter/list/0/next/5` body `{"personId":"person-admin"}` | `dataLen=1 ids=[appeal-admin]` | `:712` 一带 | `personId` 由请求体决定，未与 Session 比对 → 可指定任意人 |
| E4 | `PUT /api/attendance/.../attendanceselfholiday/filter/list/0/next/5` body `{"personId":"person-admin"}` | `dataLen=1 ids=[hol-admin]` | `:2316` 一带 | 同上 |
| E5 | `PUT /api/attendance/.../statisticshow/filter/personMonth/list/0/next/5` body `{"personId":"person-admin"}` | `dataLen=1 ids=[stat-admin]` | `:3152` 一带 | 同上 |
| E6 | `PUT /api/mind/assemble/control/mind/filter/list/folder-1/next/1` | `dataLen=2 ids=[mind-audit-own,mind-admin-secret]` | `mind_assemble_control/src/u2.rs:108`（`mind_filter_list`，`receive/recycle/shared` 为同族） | 无 `Session`、无 `creator` 过滤；`LIMIT 20` 固定；`count=len` |
| E7 | `GET /api/message/assemble/communicate/instant/list/0/next/5` | `dataLen=2 ids=[msg-admin-1,test-id]` | `message_assemble_communicate/src/lib.rs:2468` | 无 `consume/sender` 过滤；`msg-admin-1` 收件人是 `person-admin`，`test-id` 与本人无关 |
| E8 | `GET /api/message/assemble/communicate/consume/list/person-admin/count/5` | `dataLen=1 ids=[msg-admin-1]` | `:237` | `consume` 路径参数可任意填，直接读他人收件箱 |
| E9 | `GET /api/meeting/assemble/control/attachment/list/att-admin-1/next/5` | `dataLen=1 ids=[att-admin-2]`（`person=person-admin`） | `meeting_assemble_control/src/lib.rs:3693` | 附件列表 keyset 无 owner 过滤，任意登录用户可读 |
| E10 | `GET /api/meeting/list` | `dataLen=1 ids=[meeting-admin-1]`（`creator=person-admin`） | `:2096 meeting_list_page_size_size` | 全局列表无 `creator` 过滤 |
| E11 | `GET /api/ai/chat/list/paging/1/size/10`（非 NULL 对照） | 含 `clue-admin-1`（`person=person-admin`） | `ai/src/chat.rs:29` | 无 `person` 过滤 |
| E12 | `GET /api/ai/conversation/list` | 含 `conv-admin-1`（`user_id=person-admin`） | `ai/src/app.rs:115` | 无 `user_id` 过滤；`LIMIT 20` 固定 |
| E13 | `PUT /api/anonymous/document/filter/list/1/size/10` | `ids=[doc-draft-1]`，`status:"draft"`，body 为 secret | `cms_assemble_control/src/lib.rs:641` | "anonymous" 列表只过滤 `deleted_at`，不滤 `status`/`creator` → 泄露他人草稿 |

### 2.2 「my」列表名不副实（同一根因，另一个入口）

| 端点 | 实测 | 代码 |
|---|---|---|
| `POST /api/processplatform/assemble/surface/draft/list/my/paging/1/size/1` | 返回 `audit-d2`（`xperson=admin`），`count=1`（库中 3 行） | `processplatform_assemble_surface/src/lib.rs:7341` / `:7394`，SQL `FROM PP_C_DRAFT WHERE 1=1 ORDER BY "xcreateTime" DESC LIMIT $1 OFFSET $2`（`:7341` 一带） |
| `POST .../work/list/my/paging/1/size/10` | 返回 `work-admin-1`（`creator=person-admin`） | `:20869`（`work_list_my_paging_page_size_size_core`），SQL `FROM x_work WHERE deleted_at IS NULL ORDER BY create_time DESC ...`（无 creator） |
| `GET .../task/list/my/paging/1/size/10` | `dataLen=2 ids=[task-admin-2,task-admin-1]`（均 `person=person-admin`） | `:16380` 一带；`taskcompleted` `:18991`、`read` `:10041`、`review v2` `:14407`、`handover` `:8387` 同构 |
| `GET .../taskcompleted/list/my/paging/1/size/10` | `dataLen=0`（seed 无行，但 SQL 是 `WHERE 1=1`） | `:18991` |

**判定**：`/my/` 路由按 REST 直觉必须"只返回会话人的数据"，现实现把 `WHERE 1=1` 原样搬过来，属于**契约级**错误；即使当前调用方是管理员，`my` 的语义也已丢失。

### 2.3 已排除的误报（负证据，避免修复线重复劳动）

- `file` 列表：非管理员按 `person` 过滤，实测只返回自己的（`owner_clause` 正确）。
- `query_assemble_surface` `table_list`：`WHERE table_flag=$1 AND id > $2` 是**正确 keyset**（扫描器把 `table_flag=$1` 误判为游标），未发现越权。
- `meeting u2_attachment_paged` / `personal log_list`：游标是**子查询**（`created_at < (SELECT ... WHERE id=$1)`），不是等值游标；问题只剩"缺 id tiebreak"。
- `bbs u2_voterecord_list`：`topic_id` 是业务过滤，`total` 用真实 `COUNT`，分页正确。

### 2.4 静态候选（未逐条实测）

`ownership-list-candidates.txt` 收录 **389 个**含 person/creator/owner/user_id/consume/sender 列、且签名内未见 Session 或未见 `me` 过滤的列表函数（按 crate 分布见文件首部）。其中一部分是"公共列表"（组织架构、公告、榜单），不应按越权处理；建议修复线把它当**排查清单**而非结论清单，优先看 `x_mind/x_message/x_meeting/x_ai/x_attendance/x_calendar/x_bbs` 这些明确属于个人的表。

---

## 3. F8 实证：不是"潜伏"，是可直接 500

**实验设计**：`x_ai_clue` / `x_ai_completion` / `x_ai_conversation` 各造 `create_time IS NULL` 的行（`seed-null-ai.sql`），先用 `seed-ai-nonnull.sql` 恢复成 `NOW()` 做对照，再切回 NULL。

**实测**：

```
=== 非 NULL 对照 ===
chat_status=200   conversation_status=200
=== NULL 组 ===
chat_status=500   conversation_status=500
错误体（chat/list）: {"error":{"code":"internal","message":"Internal server error"}}（实测原文）
```

**代码**：`ai/src/chat.rs:29` 的 SQL 取 `create_time::text AS create_time`（可空），`:50` 用 `row.get("create_time")` → `Value::String`；`ai/src/app.rs:115` 同 SQL，`:130` 同样解码。SQLx/tokio-postgres 的 `row.get::<_, String>` 对 NULL 会 panic，panic 被 `AppError` 兜底成 500。

**触发条件**：表里存在 `create_time IS NULL` 的历史行（迁移/导入/旧版本写入都可能产生），且该用户能命中列表查询。不是"必须有 NULL 才能触发"的安全问题，而是**数据兼容性崩溃**。
**建议判据**：同一数据下 `chat/list`、`conversation/list`、`chat/list/completion` 均 200；NULL 行应序列化为 `null` 或空串，不得 500。修复方向：`row.get::<_, Option<String>>(...)` 或 SQL 侧 `COALESCE(create_time::text,'')`。
**严重度**：中（一次 NULL 行即可让整个 AI 模块列表不可用；影响面覆盖三个表）。

---

## 4. 扩到 Go `server/`（`laiyipao/server`，`:3998`，库 `laiyipao_audit_2`）

环境：`MIGRATIONS_ON_BOOT=true` 让 goose 在空库上自动建表，`go run ./cmd/api` 起 3998；管理员 `audit-admin`、玩家 `audit_u1`（`/api/v1/auth/login`，响应字段 `access_token`）。管理员登录 `/api/v1/admin/login`。

### 4.1 G1（高）：`hasMore` 用未 clamp 的 limit，导致用户不可达

**实测**（users 总数 252，含我插入的无钱包用户 300）：

```
limit=50  items=50  total=252 next_cursor=203  lastId=203
limit=200 items=200 total=252 next_cursor=53   lastId=53
limit=201 items=200 total=252 next_cursor=<空> lastId=53
limit=500 items=200 total=252 next_cursor=<空> lastId=53
limit=0   items=50  total=252 next_cursor=<空> lastId=203
limit=-1  items=50  total=252 next_cursor=<空> lastId=203
（对照：limit=200&cursor=53 -> items=52 total=252，说明游标机制本身可用）
```

**根因**（两处协同）：
- `internal/service/stats.go:441`：`limit = clampLimit(limit, 50, 200)` → 内部实际最多 200；
- `internal/httpapi/handlers_admin.go:280`：`limit, err := queryInt(c, "limit", 50)` **未 clamp**；`:331`：`if len(items) == limit && len(items) > 0 { next = items[len(items)-1].ID }`。

所以 `limit=201/500` 时内部返回 200 行，`len(items)=200 != limit` → 认为"没有下一页"，剩余 52 行只有在客户端改回 `limit=200` 时才可达；`limit=0/-1` 时内部按默认 50 返回，`len=50 != 0/-1` → 同样认为到底，**用"默认值"调用的客户端连第 51 行都翻不到**。

同一模式在 `:607`（`AdminBattlesResp`）重复，问题同构。

**建议判据**：`limit=200` 与 `limit=201/500/0/-1` 都应给出 `next_cursor`（只要 `len(items) == 实际生效 limit` 且 `total > offset+len`）；`cursor=<最后一页 id>` 能取到剩余 52 个用户；最终遍历总行数 = `total`。修复方式：handler 与 service 共用同一个 clamp 结果，或 service 返回 `(items,total,hasMore)`。
**严重度**：高（管理后台"用户列表"少一截，且无任何报错，排查成本高）。

### 4.2 G2（中）：缺 wallet 行把"用户详情"整个打成 404

**实测**（新造 user id=300，确认 `user_wallets` 0 行）：

```
GET /api/v1/admin/users/300
{"error":{"code":"not_found","message":"load user wallet: 资源不存在: wallet for user 300"}}
STATUS=404
```

对照：`/api/v1/admin/users/1`、`/api/v1/admin/users/2` 都 200（它们的 wallet 行存在——登录/首局会创建）。`/api/v1/admin/users/999999` 也是 404，但 message 是 `用户 999999 不存在`，与上面可区分。

**代码**：`internal/service/adminuserdetail.go:183` 调 `LoadWalletReadOnly`，`:185` 把错误包成 `load user wallet: %w`；`LoadWalletReadOnly` 对 `ErrNoRows` 返回 `ErrNotFound`。主查询本身能拿到 user 行，但后续钱包加载失败让整个 handler 走 404。

**触发条件**：用户存在、但 `user_wallets` 无对应行（历史数据、后台补录、迁移遗漏）。
**建议判据**：`/admin/users/300` 返回 200，`wallet` 为 `null` 或零值，`user` 字段正常；只有 user 行真的不存在时才 404。
**严重度**：中（单用户详情不可用；但列表页不受影响）。

### 4.3 G3（中）：战报 offset 分页漂移

**实测**：

```
第一次（插入新战报前）: offset=0 -> 9,8
插入 id=10（created_at=NOW()）后: offset=0 -> 10,9 ; offset=2 -> 8,6
```

第 1 页（插入前）是 `[9,8]`，第 2 页（插入后）是 `[8,6]` → **id=8 跨页重复**，用户会看到同一局两次；若删除则会漏行。

**代码**：`internal/service/economy.go:1168-1173`：

```sql
... FROM battle_records WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3
```

`created_at` 不是唯一键，且无 `id` tiebreak；offset 在"读取期间数据变化"时必然漂移。

**建议判据**：同一用户连续插入两局后翻页，各页 id 不重复不遗漏；或改 keyset（`created_at < last_time OR (created_at = last_time AND id < last_id)`）。
**严重度**：中（战报列表重复/漏条，属于用户可见数据错误）。

### 4.4 G4（负证据）：NULL 行在 Go 侧没有 500

实测均 200：

```
GET /api/v1/announcements        -> {"items":[{"id":1,...,"published_at":null}]}
GET /api/v1/admin/orders         -> [{"delivered_at":null}]
GET /api/v1/admin/redeem-codes   -> [{"expires_at":null}]
```

说明 Go 侧对可空字段普遍用了 `*time.Time`/`omitempty`（如 `internal/service/announcement.go` 的 `PublishedAt *time.Time`）。**未在本轮 3 张表上复现 NULL 500**；若要继续扩，应扫 `Scan(&x)` 到非指针基础类型的列（`int64/string/time.Time`），而不是只看这三张。

### 4.5 G5（负证据）：`/me/*` 归属过滤是好的

```
GET /api/v1/me/battles?limit=10 -> ids=[9,8,6,1,2,3,4,5]（全部 user_id=1）
   （user2 的 id=7 不在列表里）
GET /api/v1/me/orders?limit=10  -> {"items":[]}（user2 的订单不出现）
GET /api/v1/battle/7/replay     -> 404 {"message":"资源不存在: battle 7"}（他人战报不可读）
GET /api/v1/battle/1/replay     -> 200（自己的可读）
```

**结论**：Go 侧玩家面 `me/battles`、`me/orders`、`battle/{id}/replay` 的归属校验正确；本轮没有发现 Go 侧越权读。管理面越权属于"管理员本就该看全量"，不计入。

---

## 5. 修复判据汇总（可直接当验收脚本）

| 编号 | 验收动作 | 期望 |
|---|---|---|
| F1-A | 在 `pp_c_draft` 造 3 行，`GET .../draft/list/{第一行}/next/2` | 返回第一行之后的 2 行，不含第一行 |
| F1-A | 同端点 `prev/2` | 返回第一行之前的 2 行 |
| F1-B | `PUT /api/appinfo/filter/list/{某 app}/next/1` | 只 1 行，且是 `{id}` 之后的第一行 |
| F1-B | `PUT /api/appinfo/filter/list/{某 app}/prev/1` | 与 next 集合不同 |
| F1-C | `GET /api/file/.../file/list/{id}/next/3` | ≤3 行 |
| F1-E | 任一列表响应 `count` | 等于该过滤条件下的总行数 |
| F2-E1/E2 | 普通用户访问考勤明细列表 | 只含 `person=本人`；总数与本人行数一致 |
| F2-E3/E4/E5 | 普通用户 `personId` 填他人 | 403 或空集（不得返回他人行） |
| F2-E6/E7/E8/E9/E10 | 普通用户访问 mind / message / meeting | 只含本人（或与会者）相关行 |
| F2-E11/E12 | 普通用户访问 AI 列表 | 只含 `person/user_id=本人` |
| F2-E13 | 普通用户访问 anonymous document | 不含 `status=draft` 或 `creator!=本人` |
| F2-my | 普通用户访问 `*/my/paging` | 只含本人；`count` 与本人总数一致 |
| F8 | 三表塞 `create_time=NULL` 行 | 三个列表仍 200，字段为 null/空串 |
| G1 | `limit=200`、`limit=201`、`limit=0` 各请求一次 | 三者都有 `next_cursor`；用游标可遍历到 `total` |
| G2 | 造一个无 wallet 的用户，`GET /admin/users/{id}` | 200，`wallet=null`，`user` 正常 |
| G3 | 读第 1 页后插入新战报，再读第 2 页 | 各页 id 不重复、不遗漏 |
| G4 | 三张 Go 表塞 NULL | 200（保持现状即可） |
| G5 | 用户 A 读 B 的战报 / 订单 | 404 或空集（保持现状即可） |

---

## 6. 方法与限制（下一轮请直接沿用）

1. **403 ≠ 越权**：Rust 侧写方法（POST/PUT/DELETE）经过 CSRF 中间件，缺 `Origin` 时统一 403（`shared/src/middleware/security.rs` 的 CSRF 分支）。上一轮若用 `Invoke-WebRequest` 未带 Origin，会把"可访问的泄露端点"误判成"被挡住"。**复测必须带 `Origin: http://127.0.0.1:3999`**，否则结论会整体反转。
2. **等值游标 vs 业务过滤**：静态扫描里 `WHERE x=$1` 既可能是游标（`xid=$1`），也可能是过滤列（`table_flag=$1`）。判定方法：看排序键是否与 `$1` 同列、是否有 `> / <` 比较。本轮据此把 query/meeting/bbs 的误报剔除了。
3. **未实测的部分**：F1-A 的 85 个 handler 中只实测了 `draft`，其余按"代码同构 + 抽样 3 个"判定为事实；`ownership-list-candidates.txt` 的 389 条只做了代表性实测，其余标为"推断未验证"。
4. **环境差异**：上一轮 `G2 /admin/users/2` 曾 404，本轮该用户已因登录自动补了 wallet 行而 200；已用新建 user 300 重新实证，结论仍成立，但**引用时请用 user 300 的证据**，不要引用 id=2 的旧输出。
5. **未提交**：本轮只新增 `report-2.md` 与证据/清单文件，未改代码、未 `git add`。
