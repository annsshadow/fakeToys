# 第四轮行为级缺陷审计（只读）

> 对象：Rust `oa4rust/backend` + Go `server/`
> 边界：未修改任何业务代码、迁移、配置；无 `git add/commit/push/reset`；仅在自建 scratch 库造数（Rust `oa_audit_20261105`、Go `laiyipao_audit_2`）。唯一写入为本审计目录下的报告与证据文件。
> 身份：Rust 攻击者 `audit-user`（普通会话），受害者 `admin`（`person-admin`，唯一账号 = admin）；Go 玩家 `audit_u1`/`audit_u2`，管理员 `audit-admin`。
> 实测原文：同目录 `evidence-round4.txt`（含静态全量清单、HTTP 响应、直接查库输出）。

---

## 〇、基线（结论时效）

| 仓库 | 读代码/跑测试时 HEAD | 工作树 |
|---|---|---|
| Rust `D:\WORKSPACE\fakeToys-oa4rust` | **`40b215ef4`** | 干净（0 modified） |
| Go `D:\WORKSPACE\fakeToys-laiyipao` | **`961bda135`**（00:07:19 `fix(server): 修复两处写后必读缺陷（审计 R132）`） | 干净 |

**时效说明（重要）**：Go 的写后必读实测在 00:05–00:06 执行，落在 `961bda135` 提交之前的工作树；该提交正好修了两条（公告 `created_at`、兑换码未知币种）。00:11 复核 HEAD，确认两条修复已在代码中。**下文 Go 的两条缺陷按“已在 961bda135 修复”标注，不再作为待修项。** Rust 的 `40b215ef4` 在本轮全程未变（无新提交），结论以该哈希为准。

---

## 一、结论摘要

| 任务 | 全量口径 | 结果 |
|---|---|---|
| T1 写后必读全量 | Rust 1587 条写路由 → 1506 个可定位 handler 体；其中 367 个 handler 涉及归属/时间/金额/状态字段 | **任一敏感字段取自请求体 125（其中归属 80、状态 66、金额 2，重叠计）· 归属硬编码 37 · 时间取自请求体 34**；动态逐字段比对 17 个 Rust 端点 + 4 个 Go 端点 |
| T2 冒名可利用性 | bbs `author_id` / message `sender` / calendar `createor`（补充 bbs reply `creator`） | **2 条实证到后果层**（message 权限分支、bbs 受害者列表+删除）；1 条后果链断开（calendar core_entity 写死表）；1 条静态确认越权改任意帖 |
| T3 8h 时区 | `cal_event` 家族 + 全库时间列 | **落库用 UTC 且落在无时区列/文本列**；无“读出把 UTC 当本地”的代码路径；同族 13 张表存在同类写法；另发现 `cal_event.create_time` 根本不写入 |
| T4 Go 写端点 | 48 个写路由全量静态 + 动态 | **全部身份来自 token；玩家 body 伪造身份无效；玩家打管理端点 401**；写后必读 2 缺陷已被 `961bda135` 修复 |

**本轮最重要的新发现（高）**：`attendance_core_entity` 的考勤记录 **增/改/删三个写端点** 只 `require_session`，`user_id` 取自请求体、更新/删除按 `Path(id)` 直接操作——**任意登录用户可以创建、修改、删除任意他人的考勤记录**，已实测到后果（把 admin 的记录改成 `late` 后删除）。

---

## 二、T1：写后必读全量

### 2.1 口径

- 路由来源：`routes2.json`（1587 条 POST/PUT/PATCH，1295 个唯一 handler）；能定位到函数体的 1506 个。
- 字段识别：handler 体内 `let` 绑定 + `ActiveModel` 赋值 + `INSERT ... VALUES` 列名，按 `归属 / 时间 / 金额 / 状态` 四类正则归类。
- 动态判定：写入 → 直接查 scratch 库 → 与请求值逐字段比对；响应体与库值同时记录。
- 三态：**已实证**（HTTP 实测且查库）/ **静态确认**（代码层判定，未造数）/ **不可判**（表空、需要前置业务链、或路由不存在）。

### 2.2 归属字段来源全量（静态）

| 分类 | 数量（handler） | 代表 | 判定 |
|---|---:|---|---|
| 任一敏感字段（归属/金额/状态）取自请求体 | **125**（归属 80、状态 66、金额 2，类别间重叠） | bbs `create_topic`/`create_reply`、message `message_create`、calendar_core_entity `event_create`/`calendar_create`、attendance_core_entity `record_create`、program_center `agent_create`/`application_create` | 见 2.3、2.4 |
| 归属硬编码常量 | **37** | mind `folder_save`→`"system"`、file `create_file_entity`→`"system"`、jpush→`"system"`、calendar_core_entity→`"anonymous"` | 见 §三 |
| 时间取自请求体 | **34** | calendar_core_entity `event_create`（`start_time` 原样入库）、attendance `attendancedetail_create`（`date` 字符串）、meeting `create_meeting`（字符串绑定失败 500） | 见 §四 |
| 状态取自请求体 | **66** | attendance `attendancedetail_create.status`（无枚举校验）、calendar `event_update.status`（已有归属门禁） | 见 2.3 |
| 金额取自请求体 | **2** | 充值与红包类 | 未造数（不可判） |

> 逐条全量清单见 `evidence-round4.txt`：`§B-full`（归属/金额/状态取自请求体的 handler）、`§B+C+D+E`（B 段 37 条硬编码、C 段 34 条时间、D 段 66 条状态、E 段 2 条金额）。

### 2.3 动态写后必读结果（Rust）

| # | 端点 | 请求中的敏感字段 | 落库实测 | 判定 | file:line | 严重度 |
|---|---|---|---|---|---|---|
| 1 | `POST /api/bbs/assemble/control/topic/create` | `authorId:"admin"` | `creator=audit-user`（会话）· `author_id=admin`（请求体） | **已实证·冒名** | `bbs_assemble_control/src/lib.rs:455,458` | 高 |
| 2 | `POST /api/bbs/assemble/control/reply/create` | `authorId:"admin"` | `creator=admin` · `author_id=admin`（两者都取自请求体，无会话兜底） | **已实证·冒名** | `bbs_assemble_control/src/lib.rs:529` | 高 |
| 3 | `POST /api/message/assemble/communicate/message/custom/create` | `sender:"admin"` | `sender=admin` | **已实证·冒名** | `message_assemble_communicate/src/lib.rs:2797`（撤回判定 `:1853,1877`） | 高 |
| 4 | `POST /api/calendar/core/entity/event/create` | `createor:"admin"` | `x_cal_event.createor=admin` | **已实证·冒名（写死表，后果链断）** | `calendar_core_entity/src/lib.rs:456` | 中 |
| 5 | `POST /api/calendar/core/entity/calendar/create` | `createor:"admin"` / 缺省 | `createor=admin`；缺省写 `"anonymous"` | **已实证·冒名+硬编码** | `calendar_core_entity/src/lib.rs` | 中 |
| 6 | `POST /api/calendar/event/create`（兄弟端点，已修） | `createor:"admin"` | `CAL_EVENT.createor=audit-user`（会话） | **已实证·正确** | `calendar/src/lib.rs` | — |
| 7 | `POST /api/mind/assemble/control/folder/save` | 无归属字段 | `creator=system` | **已实证·硬编码** | `mind_assemble_control/src/lib.rs:264` | 实为中低（①式） |
| 8 | `POST /api/file/core/entity/file/create` | 无归属字段 | `creator=system` | **已实证·硬编码** | `file_assemble_control/src/lib.rs:574` | 低 |
| 9 | `POST /api/jpush/create` | — | `creator=system`，玩家 403 / 管理员 200 | **已实证·硬编码（admin 限定）** | `jpush` | 低 |
| 10 | `POST /api/program_center/agent/create` | `creator:"someone-else"` | `creator=someone-else`（管理员可写任意字符串） | **已实证·冒名（admin 限定）** | `program_center/src/lib.rs` | 低 |
| 11 | `POST /api/program_center/application/create` | `appId:"w4app<ts>"` + `creator` | `app_id=""`（**驼峰 body 被静默丢弃**）· `creator=someone` | **已实证·字段丢弃 + 冒名** | `program_center/src/lib.rs:7891` | 中 |
| 12 | `POST /api/attendance/assemble/control/attendancedetail` | `status:"holiday"` / `date` | `status=holiday`（无枚举校验）· 伪造 `personId` 被 403 | **已实证·状态可写，归属有门禁** | `attendance_assemble_control/src/lib.rs` | 中 |
| 13 | `POST /api/meeting/assemble/control/meeting/create` | `startTime` 字符串 | HTTP **500**，无落库 | **已实证·写端点不可用** | `meeting_assemble_control/src/lib.rs:2332` | 中 |
| 14 | `POST /api/attendance/appeal/submit` | `appealDate` | `personId=audit-user`（会话） | **已实证·正确** | `attendance_assemble_control` | — |
| 15 | `POST /api/organization/person/{id}`（PUT） | 改 admin 姓名/手机 | **403** | **已实证·正确（路由层资源鉴权）** | `organization_core_entity/src/lib.rs` | — |
| 16 | `POST /api/attendance/core/entity/record/create` | `userId:"admin"` | `user_id=admin`、`create_time=NULL` | **已实证·跨用户写入** | `attendance_core_entity/src/lib.rs:112` | **高** |
| 17 | `POST /api/attendance/core/entity/record/{id}/update` 与 `/delete` | `status:"late"` | admin 的记录被改成 `late` 后删除成功 | **已实证·跨用户改删** | `attendance_core_entity/src/lib.rs:162,207` | **高** |

**新增高危（#16/#17）细节**：`record_create` 用 `payload.userId`（无归属校验）、`record_update`/`record_delete` 只按 `Path(id)` 操作，`require_session` 之外没有任何 owner/admin 判断，SQL 也没有 `WHERE user_id = 会话`。后果是任意登录用户可以代他人打卡、篡改他人出勤状态、删除他人考勤记录——直接影响考勤统计与薪资。

**静态确认（未能动态造数）**：
- `bbs_core_entity/src/lib.rs:556` `update_subject`：`require_session` 后直接按 `Path(id)` 更新标题/置顶/禁用，**无归属校验**。`bbs_subject_info` 表为空（`x_bbs_topic` 不是同一张表），无法动态验证；一旦有数据，任意登录用户可改任意帖。
- `mind_core_entity/src/lib.rs:233` `update_mind` / `update_folder`：同样 `require_session` + 按 id 更新，无 owner 判断；`mind_mind` 表为空，不可判。
- 全量“无 gate 的可变更路径-id 端点”静态扫描 **214** 个（按 crate 分布：processplatform 105、cms 26、meeting 13、file 11、general 8、program_center 8、organization 6、message 5、attendance 4、bbs 3 …），清单在 `evidence-round4.txt §F`。**该静态口径可能被路由层资源鉴权兜住**（#15 已证伪一例），因此只作排查线索，不单列为缺陷。

### 2.4 写后必读：其余已实证缺陷

- **`appId` 静默丢弃**（#11）：`ApplicationCreateRequest` 用 snake_case 字段，客户端按响应契约发 `appId`（驼峰），无 `alias`、无报错，落库空串。同族端点凡“响应驼峰、请求 snake_case”都有此风险。
- **`cal_event.create_time` 从不写入**：`calendar/src/lib.rs` 的 `INSERT INTO CAL_EVENT (...)` 列清单无 `create_time`，列类型是 `text`，落库为 `NULL`。这是“字段被静默丢弃”，与 §四 的时区问题不是一个根因。
- **`attendance_record.create_time` 恒为 `NULL`**（#16）：代码显式 `create_time: None`，列无默认值。
- **Go 公告创建响应 `created_at:null`**：库值与列表均正确，仅创建响应缺字段（已在 `961bda135` 修复）。
- **Go 兑换码接受未知币种 `diamond`**（已在 `961bda135` 修复；本轮实测原文保留在证据文件，作为修复前复现记录）。

---

## 三、T2：冒名可利用性升级到“后果”

| 冒名点 | 伪造后是否出现在受害者「我发布的」 | 受害者能否删除/改 | 权限/通知分支 | 结论 |
|---|---|---|---|---|
| bbs `author_id`（topic） | **是**：受害者 `my/list` 命中；攻击者列表也命中（查询是 `creator OR author_id`） | 受害者（admin）删除 200；攻击者删除 403 | 无通知逻辑 | **已实证到后果**：内容挂在受害者名下，受害者可处置、攻击者不可 |
| bbs `creator`（reply） | **是**：受害者 `my/reply/list` 命中 | 无独立删除入口 | 无 | **已实证到后果**（归属被完全交给请求体，连 `creator` 都不是会话） |
| message `sender` | 受害者可读（列表全表可读，本身是第三轮 F2-E） | **攻击者 revoke 403；受害者 revoke 200**；对照：攻击者 revoke 自己正常消息 200 | **权限分支按 sender 判定**——冒名直接把“撤回权”转给受害者，攻击者失去对自己消息的控制 | **已实证到后果（权限分支）** |
| calendar `createor`（core_entity） | 否：写 `x_cal_event`，用户面列表读 `CAL_EVENT` | 否 | 无 | **后果链断开**：字段级缺陷，降级为中 |
| calendar `createor`（calendar crate，已修） | — | — | — | 已实证正确 |

补充实证（写入侧 IDOR，非字段冒名但同属“归属来自请求/路径”）：
- `attendance_core_entity` 考勤记录：攻击者创建 `user_id=admin` 的记录 → 更新为 `late` → 删除成功（#16/#17）。
- `PUT /api/bbs/assemble/control/user/subject/my/list` 的 body 可传 `{"creator":"admin"}`：**任意登录用户可读他人「我发布的」列表**（读取侧，与前两轮 F2-E 同族）。

---

## 四、T3：`create_time` 8 小时差的三方对照

### 4.1 三方对照（一次完整写入）

| 项 | 值 |
|---|---|
| 请求时刻（本机 Asia/Shanghai） | `2026-11-06 00:03:58` |
| 请求时刻（UTC） | `2026-11-05 16:03:58` |
| 请求体 `start_time` | `"2026-11-06T09:00:00"`（无时区，原样入库） |
| 库 `x_cal_event.create_time` | `2026-11-05 16:03:58`，列类型 **`timestamp without time zone`** |
| 库 `CAL_EVENT.create_time`（兄弟端点） | **`NULL`**（列类型 `text`，INSERT 没写该列） |
| API 输出（core_entity 列表） | 不返回 `create_time` |

**判定**：落库用的是 **UTC**（`Utc::now().naive_utc()`），但列本身**不带时区**。不存在“读出时把 UTC 当本地时间”的代码路径（相关 API 压根不回显该字段）。因此“8 小时差”的根因是：**UTC 值写进了无时区列/文本列**；任何按本地墙钟展示该值的消费者都会差 8 小时。

### 4.2 同族检查

- 13 张相关表的时间列分布：`text`（cal_event, x_applications, x_attendance_detail, x_file, x_jpush, x_message, x_mind, x_program_agent）与 `timestamp without time zone`（x_bbs_reply, x_bbs_topic, x_cal_calendar, x_cal_event, x_meeting）。
- 全库 `create_time/created_at/update_time/updated_at` 列类型：**`text` 394 · `timestamp without time zone` 188 · `timestamp with time zone` 44**。
- 样本：`x_message.create_time(text) = 2026-11-05 16:02:55.933403+00`（带 `+00`）；`x_bbs_topic.create_time(naive) = 2026-11-05 16:02:55.24423`（同刻 UTC，但无标记）；`x_attendance_detail.create_time = ''`（空串）。
- **结论**：这不是某一张表的孤例，而是“文本/无时区列 + UTC 写入”的族级写法；建议统一为 `timestamptz`，或写入即带 `+00`。

---

## 五、T4：Go 写端点

### 5.1 静态全量（48 个写路由）

- 全部写处理器身份来自 `userIDFrom(c)` / `adminIDFrom(c)`；**玩家请求体结构里没有 `user_id`/`player_id`/`owner` 字段**。
- 管理写端点有独立中间件（玩家 token 一律 401）。

### 5.2 动态

| 测试 | 结果 | 判定 |
|---|---|---|
| 玩家 token → `POST /admin/announcements`、`/admin/users/2/ban`、`/admin/users/2/grant`、`/admin/redeem-codes` | 全部 **401** | 已实证·正确 |
| `PUT /me/nickname` body 带 `user_id/player_id/id=2` | 只改自己；u2 昵称未被改写 | 已实证·正确（身份字段被忽略） |
| `PUT /me/loadout` body 带 `user_id=2` | 400 技能未解锁，未产生写入 | 行为上未越权；请求体身份字段未被处理器消费（静态确认） |
| 管理创建公告写后读回 | 响应 `created_at:null`，列表/库正确（`2026-11-06T00:05:39.529208+08:00`） | **缺陷，已在 `961bda135` 修复**；另有纳秒→微秒截断（`.5286078` → `.528607`） |
| 管理创建兑换码 `reward.diamond=7` | 接受未知币种；`expires_at` 输入 UTC `23:59:59Z` → 输出 `2027-01-01T07:59:59+08:00` | **未知币种已修**；时区转换**正确**（非缺陷） |
| 玩家签到 / 改昵称后 `/me` 读回 | 一致 | 已实证·正确 |

---

## 六、严重度与修复判据

| 严重度 | 缺陷 | 判据（可写成测试） |
|---|---|---|
| **高** | attendance_core_entity 记录增/改/删跨用户（#16/#17） | ① `record_create` 的 `user_id` 必须等于 `sess.person_unique`（非 admin）；② `record_update/delete` 的 SQL 必须带 `WHERE user_id = $sess` 或前置 `require_owner`；③ 测试：A 创建/改/删 B 的记录 → 必须 403 |
| **高** | bbs `author_id`（topic/reply）来自请求体 | `author_id` 必须等于 `sess.person_unique`；body 传他人 id → 落库仍是会话人；测试断言 `author_id != 请求值` |
| **高** | message `sender` 来自请求体 | 同上；且 revoke 判定必须基于会话而非 `sender` 字段 |
| **中** | calendar core_entity `createor` 来自请求体 + 缺省 `"anonymous"` | `createor` 取会话；缺省不得为匿名；测试断言伪造值不落库 |
| **中** | `cal_event.create_time` 未写入 / `attendance_record.create_time=NULL` | 写后必读：插入后 `create_time IS NOT NULL`；列统一 `timestamptz` |
| **中** | `appId` 驼峰 body 静默丢弃 | 请求结构加 `#[serde(alias="appId")]` 或统一契约；测试断言落库 `app_id == 请求值` |
| **中** | meeting create 字符串时间 500 | `start_time` 绑定改 `String` 后 parse 或直接收 `NaiveDateTime`；测试断言 200 且 `start_time` 落库 |
| **中** | attendance detail `status` 无枚举校验 | 服务端白名单；测试非法 status → 400 |
| **低** | mind/file/jpush `creator="system"` 硬编码 | 归属取会话；否则无法按人审计 |
| **低** | 静态 214 个“无 gate 路径-id 端点” | 先按 crate 抽样动态复核（路由层可能已兜住，如 #15），再决定是否加 SQL 谓词 |

**不可判清单（明确写出原因）**：`bbs_subject_info` / `mind_mind` 空表（static-only）；`x_applications` 的金额类字段未造数；需要前置审批链的 processplatform 写端点；multipart 文件上传端点（JSON harness 不覆盖）。

---

## 七、基线复述（防止修复线改完后结论过期）

- Rust 结论对应 **`40b215ef4`**（本轮无新提交）。
- Go 结论对应 **`961bda135`**；其中“公告 created_at”“兑换码未知币种”两条在本轮中途被该提交修复，**不要按待修项重复派工**。
- 所有 file:line 均为上述哈希下的行号；修复线改动后请以 `git blame`/`diff` 复核行号是否漂移。
