# 第三轮行为级缺陷审计（只读）

> 对象：Rust `oa4rust/backend` + Go `server/`
> 边界：未修改任何业务代码、迁移、配置；无 `git add/commit/push/reset`；仅在自建 scratch 库造数（Rust `oa_audit_20261105`、Go `laiyipao_audit_2`）。唯一写入为本审计目录下的报告与证据文件。
> 身份：Rust `audit-user`（普通会话）与 `person-admin`（“他人数据”所有者）；Go `audit_u1`（玩家）与 `audit-admin`（管理员）。
> 实测原文：同目录 `evidence-round3.txt`（含 HTTP 响应全文、直接查库输出、脚本口径计数）。

---

## 一、结论摘要

| 任务 | 结果 | 关键数字 |
|---|---|---|
| T1 389 条候选中“推断”升级 | 四类端点（person/owner、tenant/unit、无归属字段、逻辑表）全部给出三态计数；口径内实证越权 8 条，口径外补充实证 12 条 | 389 = 已实证越权 8 · 已实证正确 1 · 静态确认越权 86 · 静态确认正确/不适用 184 · 不可判 55 · 不可达 53 · 写端点 2 |
| T2 F1-A 85 个签名分组 | 85 = 48 `PathTuple(2)` + 34 `PathTuple(3)` + 3 内部 helper；每组代表实测 2–3 个 | 前端真实调用 **82**；无人调用 **0**；无路由 3（helper） |
| T3 写后必读 | 13 例（Rust 7 / Go 6），8 例缺陷、5 例正确 | 冒名 3 · 硬编码归属 2 · 500 1 · 时区 8h 1 · 截断/响应缺字段 1 · 未知币种 1 |
| T4 Go 列表归属全量 | 玩家 25 个 GET 全量核对，无越权 | 20 个 `user_id/owner_id` 过滤 · 4 个按设计共享/他人 · 1 个归属校验 |

**最重要的新结论**：`ownership-list-candidates.txt` 的 389 条只覆盖“SQL 里出现了归属列”的 handler；**“SQL 里完全没有归属谓词”的另一大族（F1-A 85 条）不在 389 内**。因此本轮把两族分开统计，避免用 389 的“静态确认”掩盖更严重的无谓词族。

---

## 二、T1：389 条候选的三态实证

### 2.1 分类口径

| 类 | 定义 | 条数 |
|---|---|---|
| OWNER/PERSON | SQL 引用 `xperson/person/creator/creatorPerson` 等个人归属列 | 263 |
| TENANT/UNIT | SQL 引用 `unit/xunit/tenant` 等组织归属列（大量为组织目录表） | 101 |
| NO_OWN_FIELD | 表无归属列（`auth_identity` 等） | 3 |
| UNKNOWN_TABLE | 代码里的逻辑表名（`PP_C_TASK`、`FILE_FILE`、视图名等），非物理表 | 22 |

### 2.2 三态计数（合计 389）

| 类 | 已实证越权 | 已实证正确 | 静态确认越权 | 静态确认正确/无需归属 | 静态不可判 | 不可达(无路由) | 写端点(不适用) |
|---|---:|---:|---:|---:|---:|---:|---:|
| OWNER/PERSON 263 | 6 | 1 | 74 | 114（共享配置 98 + 组织目录 16） | 23（表空） | 43 | 2 |
| TENANT/UNIT 101 | 2 | 0 | 12 | 70（组织目录 69 + 共享配置 1） | 10（表空 8 + 逻辑表 2） | 7 | 0 |
| NO_OWN_FIELD 3 | 0 | 0 | 0 | 0 | 1（无归属列，见 2.6） | 2 | 0 |
| UNKNOWN_TABLE 22 | 0 | 0 | 0 | 0 | 21（逻辑表名/视图） | 1 | 0 |
| **合计 389** | **8** | **1** | **86** | **184** | **55** | **53** | **2** |

> “静态确认越权”的含义：同一 crate / 同一 SQL 模板，代码层已确认没有会话或归属谓词；其中每族至少抽 1 个代表做过 HTTP 实测（见 2.3）。“不可达”= 没有注册路由的死代码，不是缺陷。

### 2.3 已实证越权（口径内 8 条）

| # | 端点（身份 `audit-user`） | 返回他人数据（实测原文摘要） | file:line | 严重度 |
|---|---|---|---|---|
| 1 | `GET /api/processplatform/service/processing/work/list` | `title:"Admin work 1", creator:"person-admin"` | `processplatform_service_processing/src/lib.rs:2285` | 高 |
| 2 | `GET /api/processplatform/service/processing/task/list` | `person:"person-admin"` 两条 admin 任务 | `processplatform_service_processing/src/lib.rs:1072` | 高 |
| 3 | `POST /api/meeting/assemble/control/meeting/list/1/size/5` | `creator:"person-admin", content:"secret agenda"` | `meeting_assemble_control/src/lib.rs:2096` | 高 |
| 4 | `GET /api/message/assemble/communicate/message/list/paging/1/size/10` | `sender:"person-admin"`, 含 `W8 spoof` | `message_assemble_communicate/src/lib.rs:2823` | 高 |
| 5 | `GET /api/ai/core/entity/conversation/list` | `userId:"person-admin", title:"admin conv"` | `ai/src/app.rs:109` | 中 |
| 6 | `GET /api/jpush/device/list` | `userId:"test-id"` 他人设备 token | `jpush/src/lib.rs:31` | 中 |
| 7 | `GET /api/attendance/assemble/control/attendancedetail/filter/list` | 全表 21 行，`personId:"p1"/"someone-else"` | `attendance_assemble_control/src/lib.rs:1201` | 高 |
| 8 | `GET /attendance/.../attendancedetail/filter/list/topUnit`（上轮已实测） | 同单位全部考勤明细 | `attendance_assemble_control/src/lib.rs:1250` | 高 |

实测原文（节选）：

```
GET /api/processplatform/service/processing/work/list -> 200
{"data":[{"id":"work-admin-1","creator":"person-admin","title":"Admin work 1",...}],"count":1}

GET /api/message/assemble/communicate/message/list/paging/1/size/10 -> 200
{"data":[{"content":"W3 spoof","sender":"person-admin",...},{"content":"W8 spoof","sender":"person-admin",...}],"count":2}
```

### 2.4 补充实证（不在 389 口径内，12 条）

这些 handler 的 SQL 里没有归属列，因此 389 提取器抓不到；但它们恰恰是最严重的一族：

| 端点 | 实测结果 | file:line | 严重度 |
|---|---|---|---|
| `GET /api/mind/assemble/control/folder/tree/my` | 返回 `mind-admin-secret`（admin 私有思维导图）+ 自己 2 条 | `mind_assemble_control/src/lib.rs:156`（`list_folders`，SQL 无 person 条件） | 高 |
| `GET /api/message/assemble/communicate/consume/list/person-admin/count/5` | `content:"admin secret"`（consume 来自路径） | `message_assemble_communicate/src/lib.rs:245` | 高 |
| `PUT /api/document/filter/list/1/size/10`（已登录普通用户） | 返回 `doc-draft-1`, `status:"draft"`, `creator:"person-admin"` | `cms_assemble_control/src/lib.rs:11086` | 高（见 2.6 更正） |
| `GET /attendance/.../attendanceselfholiday/list/all` | `hol-admin`（`personId:"person-admin"`） | `attendance_assemble_control/src/lib.rs:2418` | 高 |
| `GET /attendance/.../attendancedetail/appeal/list`（上轮） | admin 申诉 | `attendance_assemble_control/src/lib.rs`（上轮证据） | 高 |
| `GET /attendance/.../statisticshow/filter/personMonth/...`（上轮） | admin 月度统计 | 同上 | 高 |
| `GET /draft/list/next/{id}/{count}` | 以 admin 的 `audit-d2` 为游标返回 admin 行 | `processplatform_assemble_surface/src/lib.rs:7410` | 高 |
| `GET /task/list/next/{id}/{count}` | 返回 `pp-task-a1`（person-admin） | `processplatform_assemble_surface/src/lib.rs:16555` | 高 |
| `GET /read/list/next/{id}/{count}` | 返回 `f1a-read-a`（admin） | `processplatform_assemble_surface/src/lib.rs:10252` | 高 |
| `GET /read/list/next/application/{id}/{count}/{flag}` | 同上（flag=app1） | `processplatform_assemble_surface/src/lib.rs:10302` | 高 |
| `GET /task/list/next/application/{id}/{count}/{flag}` | 同上（flag=app1） | `processplatform_assemble_surface/src/lib.rs:16605` | 高 |
| `GET /work/list/next/creator/current/{id}/{count}` | 返回 `pp-work-a1`（person-admin） | `processplatform_assemble_surface/src/lib.rs:21212` | 高 |

> 更正上轮一个定性：CMS 的“匿名 document”handler 名为 `anonymous_document_filter_list_*`，但**未登录实测 401**（`evidence-round3.txt` 第 54 行）。真实问题是“普通登录用户可读他人 `status=draft` 文档”，不是“未登录可读”。仍按安全问题处理，但触发条件应改为“任意普通账号”。

### 2.5 已实证正确（1 条，另补充文件族 4 条）

| 端点 | 结果 |
|---|---|
| `GET /api/file/assemble/control/file/list/{id}/next/{count}` | 只返回 `audit-user` 自己的文件；`file_list_*` 4 个 list 端点上轮已实测正确 |

> 389 集中只有 1 条命中 `x_file`；其余 3 个文件 handler 因 SQL 带 `creator=$1` 没进候选集。

### 2.6 静态不可判（55 条）——为什么不可判

| 原因 | 条数 | 表/说明 |
|---|---:|---|
| 表空、无任何实体数据 | 32 | `x_attendance_schedule_setting(3)`、`x_ai_file(2)`、`x_attendance_record(2)`、`x_attendance_workday_config(2)`、`x_attendance_v2_group_schedule(2)`、`x_jpush(2)`、`x_message_ws_session(2)`、`x_ai_clue/x_ai_completion/x_attendance_admin/...`（各 1）。无种子就无法断言“返回了他人数据”，只能等有数据后复测。 |
| 逻辑表名 / 视图，物理表不可直接映射 | 23 | `FILE_FILE(8)`、`FILE_FOLDER(5)`、`PP_C_TASK(3)`、`CORR_C_CORRELATION(2)`、`CAL_CALENDAR(1)`、`PP_C_READ(1)`、`sub/sup/tree(3)`。其中 `PP_C_TASK/PP_C_READ` 本轮已用 HTTP 补证（2.4），其余是 ORM 逻辑名或辅助视图。 |
| 表无归属列 | 1 | `auth_identity`：列是 `person/identity` 映射，没有“行属于谁”的语义，handler 也不是列表查询；判据缺失，需要产品确认归属模型。 |

**“不可达”53 条**：`x_bbs_topic(8)`、`x_meeting(5)`、`x_bbs_reply(4)`、`x_org_unit(4)`、`x_query_*(11)` 等 handler 没有注册路由（死代码），不构成可触发缺陷，但建议清理以免误判覆盖面。

### 2.7 建议判据

- 列表/筛选类 handler：必须满足 `WHERE <归属列> = <会话身份>`；归属列来自请求体/路径即视为缺陷。
- 组织目录类（`x_org_*`）按设计可全员可见，判据应显式豁免，不要混入越权统计。
- 共享配置/流程定义/门户/字典类：无个人归属语义，判据应为“无个人字段即可”，不计入归属缺陷。
- 严重度：返回他人明文业务数据（考勤、会议、消息、文档 draft、思维导图）为高；返回设备/会话元数据为中。

---

## 三、T2：F1-A 85 个“假翻页”签名分组与实证

### 3.1 分组（按函数签名实际差异）

| 组 | 签名形态 | 条数 | 代表 |
|---|---|---:|---|
| G1 | `Path((id, _count)): Path<(String, i64)>` | 48 | `draft_list_id_next_count`、`read_list_id_next_count`、`task_list_id_next_count` |
| G2 | `Path((id, _count, other)): Path<(String, i64, String)>` | 34 | `read_list_id_next_count_application_applicationFlag`、`task_list_id_next_count_application_applicationFlag` |
| G3 | 内部 helper（非 HTTP handler） | 3 | `u2_snap_page`（`:24998`）、`u2_snap_page_all`（`:25020`）、`u2_snap_manage_cursor`（`:27629`） |

补充核对（回答“是否存在 `Path(id)` 或 count 在 query 的变体”）：

- 在 `processplatform_assemble_surface` 的全部 `next/prev` 路由中，共 180 条路由映射到 85 个 handler；**175 条路由的 handler 都是 Path tuple**。
- **只 `Path(id)` 的 2 个是预览端点**（`preview/{id}`、`attachment/{id}/preview/pdf`），不是翻页，不计入 F1-A。
- **count 在 query 里的 0 个**；3 个 “other” 是 `taskcompleted/list/prev/manual/{flag}` 与附件预览结果，同样不按 count 翻页。
- 结论：F1-A 的 85 个不存在“Path(id) only / query count”子形态，48+34+3 已完整覆盖。

代码层共同特征（脚本统计，85 条）：`EQ_FILTER_ 85`、`COUNT_NAME_IGNORED 82`、`COUNT=LEN 76`、`ORDER_BY 75`、`NO_SESSION 92`。
即：85 条全部是 `WHERE xid = $1` 的“按主键取一行”，`count` 参数 82 条被完全忽略，且没有会话过滤。

### 3.2 每组代表实测（6 个，覆盖 G1/G2）

| 组 | 端点（身份 `audit-user`） | 实测原文 | 结论 |
|---|---|---|---|
| G1 | `GET /draft/list/next/audit-d2/20` | `{"data":[{"id":"audit-d2"}],"count":1}` | 只返游标行；count=20 被忽略；读到 admin 的 draft |
| G1 | `GET /task/list/next/pp-task-a1/5` | `{"data":[{"id":"pp-task-a1"}],"count":1}` | 同上；跨归属 |
| G1 | `GET /read/list/next/f1a-read-a/5` | `{"data":[{"id":"f1a-read-a"}],"count":1}` | 同上 |
| G1 | `GET /work/list/next/creator/current/pp-work-a1/5` | `{"data":[{"id":"pp-work-a1"}],"count":1}` | 同上 |
| G2 | `GET /read/list/next/application/f1a-read-a/5/app1` | `{"data":[{"id":"f1a-read-a"}],"count":1}` | 带 applicationFlag 也只返 1 行 |
| G2 | `GET /task/list/next/application/pp-task-a1/5/app1` | `{"data":[{"id":"pp-task-a1"}],"count":1}` | 同上 |

> 31 条同组 handler 按“同组代码确认”：SQL 模板同为 `SELECT ... WHERE xid = $1 ORDER BY "xcreateTime" DESC`、`count` 未参与查询、`Path` 同样解构；代表实测通过即整组定性。

### 3.3 前端调用反查（回答“85 个里几个真的有前端在调用”）

- 方法：把路由模板 `{x}` 转成正则，在 `oa4rust/frontend`（排除 `node_modules`）源码中匹配；仓库内**没有契约 fixture 目录**，因此只能以前端源码为准。
- 结果：**调用 82 / 未调用 0 / 无路由 3**（3 个就是 G3 内部 helper）。
- 命中示例（真实代码行）：

```
frontend ... `${base}/task/list/next/application/${id}/20/${id}`
```

即前端固定传 `count=20`，后端只回 1 行 —— 假翻页在 UI 上真实生效。

### 3.4 优先级结论

- **“没人调用所以降级”的项为 0**：82 个 HTTP 端点全部被前端引用。
- 3 个 helper 不是 HTTP 面，不参与“前端是否调用”的判定。
- 建议修复优先级不降，反而应排在越权读之后（因为 UI 每页只显示 1 行，属功能性故障，且与越权叠加）。

---

## 四、T3：写路径“写后必读”（13 例）

### 4.1 总表（写入 → 立刻读回 → 逐字段核对）

| # | 资源 | 写入请求 | 读回落库值 | 字段一致？ | 缺陷类别 | 严重度 |
|---|---|---|---|---|---|---|
| 1 | mind 文件夹 | `POST folder/save {name:"w3-mind"}` | `creator:"system"` | 不一致 | **静默改写归属** | 中 |
| 2 | bbs 主题 | `POST topic/create {author_id:"person-admin"}`（会话 audit-user） | `creator:"audit-user", author_id:"person-admin"` | 不一致 | **请求体冒名** | 高 |
| 3 | 文件 | `POST file/create {name:"w3-file.txt"}` | `creator:"audit-user"` | 一致 | — | — |
| 4 | 文件实体 | `POST file/core/entity/file/create` | `creator:"system"`（硬编码） | 不一致 | **硬编码归属** | 中 |
| 5 | 消息 | `POST message/custom/create {sender:"person-admin"}` | `sender:"person-admin"` | 不一致 | **请求体冒名** | 高 |
| 6 | 日历事件 | `POST calendar/core/entity/event/create {createor:"person-admin"}` | `createor:"person-admin"`；`create_time=2026-11-05 15:47:32`（UTC）而响应 `date=23:47:32` | 不一致 | **请求体冒名 + 时区 8h** | 高 |
| 7 | 会议 | `POST meeting/create {startTime:"2026-11-06 09:00:00"}` | HTTP **500** | 未落库 | **类型绑定错误**（`&str`→`timestamp`） | 高 |
| 8 | Go 公告 | `POST admin/announcements` | create 响应 `created_at:null`；列表 `created_at` 有值；`published_at` 7 位小数 → 列表 6 位 | 不一致 | **响应缺字段 + 微秒截断** | 中 |
| 9 | Go 兑换码 | `POST admin/redeem-codes {reward:{"diamond":7}}` | 创建 200；兑换时 `400 未知货币 "diamond"` | 接受但不可用 | **校验缺口/数据污染** | 中 |
| 10 | Go 昵称 | `PUT me/nickname {nickname:"审计昵称😀"}` | `nickname:"审计昵称😀"` | 一致 | — | — |
| 11 | Go 技能装配 | `PUT me/loadout {skill_ids:[3,7,9]}` | `400 技能 3 未解锁`；读回原值 `[1,4,7,0]` | 一致（拒绝） | — | — |
| 12 | Go 钱包 | `POST admin/users/1/grant {coin:50}` | `62150 → 62200` | 一致 | — | — |
| 13 | Go 商店购买 | `POST shop/buy` | `me/purchases` 新增记录，金额一致 | 一致 | — | — |

### 4.2 三类重点

**（1）字段被静默丢弃 / 改写归属**

- bbs：`creator` 已强制会话（`bbs_assemble_control/src/lib.rs:455`），但 `author_id` 仍取请求体（`:458`）→ 帖子列表显示“admin 发的”，审计日志与责任人错位。
- message：`sender` 直接取请求体，缺省才 `"system"`（`message_assemble_communicate/src/lib.rs:2797-2800`）→ 可冒充任意人发消息。
- calendar：`createor = req.createor.unwrap_or("anonymous")`（`calendar_core_entity/src/lib.rs:456`）→ 可冒充；缺省则落 `anonymous`。
- mind：`save_folder` 的 `creator` 硬编码 `"system"`（`mind_assemble_control/src/lib.rs:264`）→ 新建文件夹永远显示系统创建。
- file 实体：`creator = "system"`（`file_assemble_control/src/lib.rs:574`）→ 与 `create_file` 的会话归属（`:484`）不一致。

**（2）类型转换截断 / 500**

- `create_meeting` 把 `&start_time`（`&str`）直接绑到 `timestamp` 列 → tokio-postgres `WrongType` → 500（`meeting_assemble_control/src/lib.rs:2332`）。对照：`save_meeting` 已有 `parse_ts_param` 修复（`:2380`），create 未同步。
- Go 公告：create 响应 `created_at:null`，列表才有值；`published_at` 由 Go `time.Now()` 产生 7 位小数，入 PG 后截为 6 位（微秒），读回值变化。建议 create 后回读 DB 或统一 `RFC3339Nano`→微秒。

**（3）时间被改时区**

- `cal_event.create_time` 落 `15:47:32`（容器 `timezone=UTC` 的 `NOW()`），同一响应 `date` 字段 `23:47:32`（Asia/Shanghai），差 8 小时；`start_time/end_time` 为 text，原样存储、无时区语义。前端若按时区渲染会整体偏移。

### 4.3 建议判据

- 写入后必须回读：以 DB 落库值为准，逐字段与请求比对；`creator/author_id/sender/createor` 只能来自会话，请求体同名字段必须拒绝或忽略。
- 时间字段：入参解析为 `NaiveDateTime` 后再绑定；`create_time` 统一由 DB `NOW()` 生成并在读接口按同一时区序列化。
- 响应必须包含 DB 生成字段（`created_at`）的真实值，不得返回零值/省略。
- 枚举/币种类字段在“写入时”校验，不能等到消费时才发现非法。

---

## 五、T4：Go 端“列表未按归属过滤”全量清单

### 5.1 玩家 GET（25 个）逐条核对

| 端点 | 归属判据（file:line） | 结论 |
|---|---|---|
| `GET /me` | `WHERE id=$1`（`routes.go` → `GetMe(userID)`） | 正确 |
| `GET /me/loadout` | `LoadLoadout(userID)` | 正确 |
| `GET /me/equipment` | `LoadEquipment(userID)` | 正确 |
| `GET /me/stars` | `LoadStars(userID)` | 正确 |
| `GET /me/rank` | `LoadRank(userID)` | 正确 |
| `GET /me/gems` | `gems.go:235 WHERE user_id=$1` | 正确 |
| `GET /me/presets` | `presets.go:112 WHERE user_id=$1` | 正确 |
| `GET /me/challenges` | `progression.go:1216 challenger_id=$1` | 正确 |
| `GET /me/defense/challenges` | `progression.go:1272 d.owner_id=$1` | 正确 |
| `GET /me/battles` | `economy.go:1258 WHERE user_id=$1` | 正确 |
| `GET /me/battles/:id` | `economy.go:1338 WHERE id=$1 AND user_id=$2` | 正确 |
| `GET /me/purchases` | `economy.go:1392 WHERE user_id=$1` | 正确 |
| `GET /me/redemptions` | `economy.go:378 WHERE user_id=$1` | 正确 |
| `GET /me/orders` | `economy.go:1438 WHERE user_id=$1` | 正确 |
| `GET /me/offline-reward` | service 以 userID 入参查询 | 正确 |
| `GET /wallet` | `LoadWallet(userID)` | 正确 |
| `GET /mastery` | `LoadMastery(userID)` | 正确 |
| `GET /tasks` | `LoadTasks(userID)` | 正确 |
| `GET /signin/status` | `economy.go:634 WHERE user_id=$1` | 正确 |
| `GET /diagnose` | `progression.go:335 WHERE user_id=$1` | 正确 |
| `GET /levels/:id` | 关卡配置，全服共享 | 按设计共享 |
| `GET /shop` | 商品目录，全服共享 | 按设计共享 |
| `GET /signin/calendar` | 签到日历配置，全服共享 | 按设计共享 |
| `GET /battle/:id/replay` | `economy.go:1487 WHERE br.id=$1` + `:1490` 归属校验 | 正确 |
| `GET /defenses/` | `progression.go:464 WHERE d.owner_id <> $1`（挑战候选） | 按设计展示他人 |

实测（`evidence-round3.txt`）：
- `/me/battles`、`/me/purchases`、`/me/redemptions`、`/me/orders`、`/me/challenges`、`/me/defense/challenges` 均只返回 `audit_u1` 自己的记录；
- `/defenses/` 返回 `candidates:null`（当前无其他玩家可挑战），SQL 明确 `owner_id <> $1`，属挑战设计；
- 玩家 token 打 `/api/v1/admin/users`、`/api/v1/admin/defenses` → **401 令牌类型不匹配**。

### 5.2 公共与管理端

- 公共：`/healthz`、`/readyz`、`/metrics`、`/config`、`/leaderboard`、`/announcements`（按设计无归属）。
- 管理端 33 个 GET 全部 `requireAdmin`；玩家 token 反向验证 401。
- **结论：Go 侧本轮未发现“列表未按归属过滤”的越权项**。Rust 的 14 类清单在 Go 侧没有对应家族（Go 的列表 SQL 普遍显式 `WHERE user_id/owner_id`）。

---

## 六、优先级建议（按严重度）

| 优先级 | 项 | 理由 |
|---|---|---|
| P0 | bbs `author_id`、message `sender`、calendar `createor` 请求体冒名（T3 2/5/6） | 可伪造他人身份写数据，破坏审计与责任人链 |
| P0 | T1 越权读高敏感族：考勤明细/申诉/自休假、会议、消息、文档 draft、思维导图（2.3/2.4） | 他人明文业务数据泄露 |
| P1 | F1-A 85 条假翻页（T2） | 前端 82 个端点全在调用，UI 每页仅 1 行；且叠加跨归属读取 |
| P1 | `create_meeting` 500（T3 7） | 主功能完全不可用，且与已修复的 `save_meeting` 同构 |
| P2 | `mind`/`file entity` creator 硬编码 `system`（T3 1/4） | 归属信息错误但不泄露数据 |
| P2 | Go 公告 `created_at:null` + 微秒截断；兑换码未知币种延迟校验（T3 8/9） | 写入契约不一致/数据污染 |
| P3 | 时区 8h（T3 6） | 展示口径问题，需统一序列化策略 |
| P3 | 53 条无路由死代码清理 | 不构成缺陷，但会干扰后续扫描覆盖率判断 |

---

## 七、证据与复现

- 实测原文：`evidence-round3.txt`
  - T1 抽测 13 条 HTTP 原文
  - T2 6 个代表 HTTP 原文
  - T3 Rust/Go 写后读原文 + 直接查库输出
  - T4 玩家端点原文与反向 401
  - 389 三态计数脚本口径
- 造数：`seed-null-ai.sql`、`seed-mind-owner.sql`、`seed-attendance-owner.sql`、`seed-pp-owner.sql`、`seed-go.sql`、`seed-go-owner.sql` 等（均在 scratch 库）。
- 本轮新增临时脚本（仅审计用，未入库）：`classify.py`、`siggroup.py`、`frontend_usage2.py`、`collect3.py` 等。
- 复核环境：Rust `127.0.0.1:3999`、Go `127.0.0.1:3998`；如需重跑，先确保两服务与本轮相同 binary，且请求带 `Origin: http://127.0.0.1:3999`（Rust 写方法）。
