# fakeToys 行为级缺陷审计报告（2026-11-05）

> 只读审计。未修改任何业务代码 / 迁移 / 配置，未执行 git 写操作。
> 本轮所有结论均在独立 scratch 库 `oa_audit_20261105`（同一 PostgreSQL 实例）中用**真实二进制 + 真实 HTTP 端点 + 真实 SQL** 复现；未写入 `oa4rust` / `oa4rust_test` / laiyipao 库。
> 服务已停止（127.0.0.1:3999），scratch 库保留待复核。

## 结论摘要（按价值从高到低）

| # | 缺陷 | 严重度 | 状态 |
|---|------|--------|------|
| F1 | processplatform `draft/read` 的 `next/prev/{count}` 把游标 id 当**等值过滤**，忽略 count，只返回游标行本身（1 条） | 高 | 已实证 |
| F2 | 考勤明细列表端点无 person 范围过滤，**任意登录用户可读全公司考勤明细**（PII 越权） | 高 | 已实证 |
| F3 | 考勤统计周期写入 `cycle_year/cycle_month`、读取 `year/month`，**新建周期永远查不到** | 高 | 已实证 |
| F4 | BBS 论坛 `sort` 无写入方（恒 0），列表 `ORDER BY sort LIMIT 100` + `count=data.len()`：105 条只返回 100，且无翻页 | 中高 | 已实证 |
| F5 | 考勤明细移动端 `page/size` 用 `ORDER BY date DESC` + `OFFSET`，无并列裁决；`count` 返回当前页条数。并发写入时**同一条跨页重复、部分记录永不出现** | 中高 | 已实证（SQL 层） |
| F6 | CMS 13 个 `next/prev` 端点完全忽略 cursor 与 count，返回全表（`next/1` 与 `prev/1` 都返回 4 条） | 中 | 已实证 |
| F7 | `file/list/{id}/next/{count}` 忽略 count，硬编码 `LIMIT 20` | 低 | 已实证（代码 + 路由） |
| F8 | `ai` 列表端点对可空列 `create_time/clueId/generateType` 用非 `Option` 解码，遗留数据含 NULL 时 500 | 低（潜伏） | 推断未验证 |

---

## 环境与方法

- 代码基线：worktree `D:\WORKSPACE\fakeToys-oa4rust`，分支 `o36/attendance-cycle-dualwrite`（`main` 之后的 25 个优化提交，工作区干净）。
- 最终 schema：用 `oa4rust.exe --migrate-only` 对 scratch 库应用全部 118 个迁移，未凭注释推断可空性。
- 造数：跨位数文本、重复排序键、NULL 行、10 万级不必要数据不造；目标表按缺陷需要构造（105 条同排序键、21 条同日考勤、多 person）。
- 调用方式：启动真实服务（`DATABASE_URL=<scratch>`、`AUTH_LEGACY_COMPAT=true` 仅用于给 scratch 库临时账号设口令），以 **admin** 与 **普通用户** 两种身份 curl 端点；分页类缺陷另用 psql 复刻同一 SQL 验证跨页行为。

---

## F1 processplatform 游标分页把 id 当等值过滤（高）

**现象（实测原文）**

```
GET /api/processplatform/assemble/surface/draft/list/audit-d1/next/20
-> dataLen=1  ids=audit-d1
GET /api/processplatform/assemble/surface/draft/list/audit-d2/prev/20
-> dataLen=1  ids=audit-d2
```

库中有 3 条草稿（audit-d1/d2/d3，xcreateTime 递增）。`next/20` 应返回游标之后的至多 20 条（至少 d2、d3），实际只返回游标行本身；`prev` 同理。

**触发条件**：任意 `draft/read/readcompleted/review` 的 next/prev 调用。

**表列与代码位置**
- 表：`PP_C_DRAFT.xid`（以及 `PP_C_READ` 等）
- `crates/processplatform_assemble_surface/src/lib.rs:7410-7456`（draft next：`WHERE xid = $1`、`_count` 未使用）
- `.../lib.rs:7460-7506`（draft prev，同样 `WHERE xid = $1`）
- `.../lib.rs:10252-10300`（read next，同样）
- `.../lib.rs:12313`、`.../lib.rs:14307`（readcompleted / review 同模板）
- 路由：`.../routes.rs:894-895, 921-926, 933-934`
- 规模：该 crate 中 `Path((id, _count))` 形态共 48 处、`next/prev` 函数 34 个，全文无任何 `xid > $1` 的游标比较
- 前端消费：`frontend/apps/desktop/src/views/ProcessWork.vue:1202-1213`（注释明确写“xid 游标”，取首条 id 请求“后续 20 条”）；`ProcessTaskCenterApp.vue:295-310`

**建议判据（改后怎么验证）**：造 30 条同 person 草稿，`next/30` 首屏返回 30、用末行 id 再翻一页返回剩余且无重复；`prev` 反向同理；断言两页并集 = 全集、交集为空。

**严重度**：高。前端“后续 N 条”永远只拿到 1 条（或 0 条，当 id 不存在时），流程工作台的翻页/相邻任务浏览失效。

---

## F2 考勤明细对任意登录用户全量可见（PII 越权，高）

**现象（实测原文）**
以**非管理员**账号 `audit-user` 登录后调用：

```
GET /api/attendance/assemble/control/attendancedetail/mobile/filter/list/page/1/count/5
200 {"data":[
 {"id":"audit-other-1","personId":"someone-else","date":"2026-11-07",...},
 {"id":"audit-ad-03","personId":"p1","date":"2026-11-05",...}, ...],
 "count":5}
```

返回了 `someone-else` 与 `p1` 的考勤明细，而当前登录人是第三人。桌面 keyset 端点同样：

```
GET /api/attendance/assemble/control/attendancedetail/filter/list/0/next/5
200 （返回全 person 数据，无 session/person 过滤）
```

**触发条件**：任意已登录账号。

**表列与代码位置**
- 表：`x_attendance_detail.person_id`
- 移动端：`crates/attendance_assemble_control/src/lib.rs:1604-1660`（SQL 无 `person_id` 条件，也没有 `Extension<Session>`）
- 桌面 keyset：`.../lib.rs:1401-1450`（`SELECT id, person_id, date, status FROM x_attendance_detail WHERE (keyset 条件) ...`，无 person 过滤）
- 路由：`.../routes.rs:92-96, 102-103`
- 对照：同族 `attendancedetail_mobile_my`（`lib.rs:1710`）按本人过滤，说明“我的”路径存在，但通用 list 未收敛权限

**建议判据**：以普通用户 token 调 list，断言响应中 `personId` 只含本人（或 403）；管理员调同一路径仍可看全量。

**严重度**：高（全公司出勤、迟到早退、请假等 PII 可被任意员工枚举）。

---

## F3 统计周期写入/读取列名不一致，新建周期永远查不到（高）

**现象（实测原文）**

```
POST /api/attendance/assemble/control/attendancestatisticalcycle
     body={"cycleYear":"2026","cycleMonth":"11"}
-> 200
GET  .../attendancestatisticalcycle/list/all
-> {"data":[{"id":"fae66dbb-...","month":"","year":"","cycleStatus":""}],"count":1}
GET  .../attendancestatisticalcycle/cycleDetail/2026/11
-> {"type":"error","message":"statistical cycle not found", ...}
```

写入成功、列表能看见该行，但 `year/month` 是空串，按 `year/month` 查详情必然找不到。

**触发条件**：任何通过 `POST .../attendancestatisticalcycle` 新建的周期。

**表列与代码位置**
- 表：`x_attendance_statistical_cycle`
- 写入：`crates/attendance_assemble_control/src/lib.rs:4790-4793` 只写 `cycle_year, cycle_month`
- 详情读取：`.../lib.rs:2699-2700` 按 `year, month` 过滤
- 列表读取：`.../lib.rs:2742` `SELECT id, year, month, ... ORDER BY year DESC, month DESC`
- 同表另有正确读取方：`crates/attendance/src/lib.rs:236-273` 读 `cycle_year/cycle_month`（Option）
- 这是既有 O34 台账中的“待确认”项，本次用真实调用确认为真缺陷

**建议判据**：新建后 `cycleDetail/{year}/{month}` 返回刚建记录；或统一读写到同一对列（并回填历史行）。

**严重度**：高（数据写入被视为成功但业务实体不可达，等同静默丢数据）。

---

## F4 BBS 论坛 sort 无写入方 + 固定 100 截断（中高）

**现象（实测原文）**

```
库中 x_bbs_forum 共 105 行，sort 全部为 0（列默认值 0，无 UPDATE 写 sort）
GET /api/bbs/assemble/control/forum/list
-> dataLen=100  countField=100
```

105 个论坛只返回 100 个，且 `count=100`，客户端无法知道还有 5 个，也没有分页参数。

**触发条件**：论坛/版块数量超过 100；调用方期望按 sort 排序。

**表列与代码位置**
- 表：`x_bbs_forum.sort`（INTEGER DEFAULT 0）、`x_bbs_forum.order_number`（同样 DEFAULT 0，从未读写）
- 列表：`crates/bbs_assemble_control/src/lib.rs:394-420`（`ORDER BY sort ASC LIMIT 100`，`count = data.len()`）
- 第二处列表：`.../lib.rs:566`（同 SQL）
- 写入：`crates/bbs_assemble_control/src/u2.rs:1406-1408`（INSERT 列 `id,name,description,creator,create_time`，**无 sort**）
- 版块同样：`u2.rs:1560-1562` 无 `sort`；列表 `ORDER BY sort, create_time`
- 全仓无 `UPDATE ... SET sort` 的写入方

**建议判据**：创建/更新接口接受 `sort` 并落库；列表按 `(sort, id)` 稳定排序；超过 100 条时返回真实 total 并提供分页或游标。

**严重度**：中高（排序功能失效 + 后台“消失 5 条”且无任何提示）。

---

## F5 考勤明细移动端 OFFSET 分页 + count 口径错误（中高）

**现象（实测 / SQL 复刻）**

```sql
-- 20 条同日明细，page1 = offset 0 limit 5
page1: audit-ad-02,03,04,05,01      -- 已经是非稳定顺序（键全并列）
INSERT 一条 date='2026-11-06' 的新明细后取 page2 (offset 5 limit 5)
page2: audit-ad-NEW,04,02,...       -- 04、02 与 page1 重复，且部分旧行被挤出窗口
```

HTTP 侧：库中 21 条时

```
GET .../attendancedetail/mobile/filter/list/page/1/count/5
-> "count":5
```

`count` 恒等于本页条数，而不是总行数（21）。

**触发条件**：同一 `date` 下多行（考勤明细天然按天聚合，几十~几百行）；翻页期间有新同步数据写入。

**表列与代码位置**
- 表：`x_attendance_detail.date`（TEXT，可空）
- `crates/attendance_assemble_control/src/lib.rs:1604-1660`：`ORDER BY date DESC LIMIT $2 OFFSET $1`，无 `id` 并列裁决
- `.../lib.rs:1644-1649`：`let count = data.len()`
- 对照：同表 keyset 端点 `lib.rs:1401-1450` 已用 `KeySet{date, tiebreak:id}`（排序键与游标键一致），说明正确做法已在同文件存在，移动端未切换

**建议判据**：造 200 条同日明细，逐页翻到底断言“并集=全集、交集=空”；在翻页间插入新行后仍不重不漏；`count` 返回真实总数（或前端改用“空页”判断结束）。

**严重度**：中高（员工在移动端看到重复考勤记录、漏看部分记录）。

---

## F6 CMS 13 个 next/prev 端点忽略游标与页大小（中）

**现象（实测原文）**

```
PUT /api/appinfo/filter/list/audit-app-1/next/1  -> dataLen=4 count=4 ids=全部 4 行
PUT /api/appinfo/filter/list/audit-app-3/prev/1  -> dataLen=4 count=4
```

请求 `count=1`、游标 `audit-app-1`，next 与 prev 返回完全相同的全量列表。

**触发条件**：任何调用这些“游标”端点并期望分页的客户端。

**代码位置**
- `crates/cms_assemble_control/src/lib.rs:794-825`（appinfo next/prev 均只收 `Extension<Pool>`，无 `Path`）
- 同族：`.../lib.rs:625-690`（anonymous document next/prev，`LIMIT 200` 全量）、`1790`、`2163`、`3678-3714`、`5374`、`5720`、`6201`、`7203`、`7459` 等，共 13 个 next/prev 函数
- 路由：`cms_assemble_control/src/routes.rs` 中 `/filter/list/{id}/next/{count}` 等 32 条注册
- 另：`.../lib.rs:641` 的 `anonymous/document/filter/list/page/{page}/size/{size}` 同样不取 page/size，`LIMIT 200` 后 `count=data.len()`

**建议判据**：任一 `next/1` 只返回 1 条且严格位于游标之后；连续翻页并集正确；或明确把这些端点降级为“非分页接口”并从路由契约中移除 `{id}/{count}`。

**严重度**：中（页面若按“下一页”拼接会无限重复同一批数据）。

---

## F7 file next/prev 忽略 count，固定 LIMIT 20（低）

- `crates/file_assemble_control/src/lib.rs:3108-3150`：`WHERE id > $1 ORDER BY id LIMIT 20`，`count` 参数解构后被丢弃。
- 前端 `FileManagerApp` 类调用 `/file/list/{id}/next/{cnt}`，cnt>20 时只能拿到 20 条。
- 建议：`LIMIT $2`；判据为 `count=100` 时返回 100 条。

---

## F8 ai 列表对可空列非 Option 解码（推断未验证）

- `crates/ai/src/chat.rs:27-50`（`x_ai_clue.create_time`，列可空且无默认值，`row.get("create_time")` 非 Option）
- `.../chat.rs:81-113`（`x_ai_completion.create_time/clueId/generateType` 同样非 Option）
- `crates/ai/src/app.rs:113-130`（`x_ai_conversation.create_time`）
- 当前实库该三表均为 0 行，scratch 造 NULL 行才会触发；属“遗留数据导入后 500”的潜伏缺陷。未通过端点实证，仅给列可空性 + 解码类型证据。

---

## 覆盖范围与未覆盖（如实说明）

- 已覆盖：Rust `oa4rust/backend` 的 HTTP 端点在真实 scratch 库上的行为；分页/排序/写后读/可空解码四类。
- 未覆盖：Go `server/`（本轮未启动其依赖环境，不做无证据结论）。
- 既有基线为绿：`backend/tests/idor_seeded_handler500_baseline.txt` 与 `idor_seeded_handler500_baseline.txt` 均为空，说明“种子数据 + 全路由扫描”抓不到本报告缺陷；它们依赖**非种子数据形态**（同键大量并列、跨列写入/读取、普通用户身份、游标参数）。
- 既有 O34 台账的 `pp_c_snap.sequence` / `pp_c_review.sequence`（无写入方、`LIMIT/OFFSET` 无并列裁决）本次未重复验证，仍按台账“待确认”保留。

## 复现材料

- 造数脚本：`docs/audits/behavioral-audit-2026-11-05/seed-draft.sql`（草稿游标复现用）。
- scratch 库：`oa_audit_20261105`（迁移已全量应用；含 F1-F6 的最小数据集）。
