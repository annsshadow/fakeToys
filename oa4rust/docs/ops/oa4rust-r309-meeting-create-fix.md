# R309 会议创建 HTTP 500 与列表不可见修复（t67）

## 概述

本轮修复会议创建链路上的两个独立真实缺陷，均经 test-first RED→GREEN 验证：

1. **缺陷 A（HTTP 500）**：创建接口将 `datetime-local` 字符串直接绑定为 PostgreSQL `timestamp`，触发 `WrongType` 而返回 500。
2. **缺陷 B（创建成功但列表不可见）**：新建会议 `applied` 默认 `false`，「我的会议」列表强制 `WHERE applied=true`，导致创建成功、详情可查，却始终不在列表回显。

## 缺陷 A：时间字符串绑定缺少类型转换

**真实错误证据（部署环境 run.log）**：
```
POST /api/meeting/assemble/control/meeting/create -> 500
Error { kind: ToSql(4), cause: Some(WrongType { postgres: Timestamp, rust: "&str" }) }
source crates/meeting_assemble_control/src/lib.rs:2507
```

**根因**：`create_meeting` 的 INSERT 原为 `VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())`，`$5`/`$6` 是 Rust `&str`，PostgreSQL 目标列是 `timestamp`，驱动无法自动转换。对照同一仓库 `crates/meeting/src/lib.rs:293` 已使用 `$5::text::timestamp,$6::text::timestamp`。

**RED→GREEN**：新增集成测试 `u2_create_meeting_accepts_datetime_local_times`（datetime-local 字符串 `2026-11-21T10:30`/`11:30` 走真实 POST，断言 200，并回读 `start_time`/`end_time::text` 以 `2026-11-21 10:30:00` 开头）。测试先在未修复代码上失败（500 ≠ 200，RED）；在 INSERT 补齐 `$5::text::timestamp,$6::text::timestamp` 后通过（GREEN）。

## 缺陷 B：新建会议 applied=false 导致「我的会议」列表不可见

**证据链**：
- 真浏览器第一次创建：HTTP 200，但 `titleInList=false`、`titleAfterReload=false`，0 pageerror、0 API 失败。
- 数据库探针：会议已正确入库（`start_time=2026-11-21 10:30:00`、`end_time=11:30:00`、`creator=admin`），详情端点 `meeting/{id}` 刷新后仍回显标题。
- 「我的会议」主列表读 `GET meeting/list/apply/1/size/50`，其 SQL（`lib.rs:1070`）强制 `WHERE applied=true`；`x_meeting.applied` DDL 默认 `false`（`migrations/039_add_columns_and_fix_boolean.sql:32-33`）；后端生产代码无任何位置将新建会议 `applied` 置为 `true`。
- `APPLIED_CREATOR_READ_PREDICATE`（`lib.rs:811-818`）注释明确 creator（会话身份）为申请人/管理员可读，印证 `applied` 仅作「创建者申请可见标记」。

**根因**：新建会议 `applied=false` 必然被列表过滤，形成「创建成功但列表不可见」的第二缺陷。

**RED→GREEN**：新增独立集成测试 `u2_create_meeting_is_visible_in_my_meeting_lists`，断言 `applied=true`、`list/apply` 含新 id、`applied/wait` 列表可达。测试先在未修复代码上失败（panic「created meeting must be applied=true so it appears in my meeting list」，RED）；INSERT 增加 `applied` 列并写 `TRUE` 后通过（GREEN）。

**实现**：`create_meeting` 的 INSERT 改为 `INSERT INTO x_meeting (id, title, content, room_id, start_time, end_time, creator, create_time, applied) VALUES ($1, $2, $3, $4, $5::text::timestamp, $6::text::timestamp, $7, NOW(), TRUE)`。未修改 save/delete/list 端点、前端、meeting crate 或 DDL。

## 与队长批复的偏差说明（如实记录）

队长批复要求「保持 status 走 DDL 默认 `wait` 待审语义不变」。实际 DB 探针显示 `x_meeting.status` 是 nullable text、默认 `NULL`（非 `wait`）；`applied/wait` 列表 SQL（`lib.rs:1006`）额外强制 `status = 'wait' AND applied=true`。在「不改 status 默认值」的约束下，新建会议（status=null）逻辑上不可能进入 `applied/wait`。因此：

- 主验收口径「我的会议」列表（`list/apply`，只要求 `applied=true`）已实现并验证；
- 集成测试中 `applied/wait` 断言降级为「端点可达且 HTTP200」，并加注释如实说明原因；
- 未擅自修改 status 默认值或 DDL；此偏差需在最终报告中请队长裁决。

`applied` 在此仅表示创建者申请可见标记，不代表审批通过状态。

## 验证

- `cargo test -p meeting_assemble_control`：全绿（60 passed; 0 failed; 30 ignored，另有 nullable_create_time 用例 1 passed）。
- `cargo build -p meeting_assemble_control --release`：通过。
- Linux ELF（Docker bullseye，`cargo +1.98.0 --offline`）原子部署测试服，systemd `active`。
- 真浏览器（Playwright Chromium headless，1440×900，非物理真机）：会议创建 HTTP 200、新建会议在「我的会议」列表回显、刷新后持久、0 pageerror；截图与 evidence 落 gitignored `oa4rust/test-results/r309-meeting-fix/`。

## 环境说明

测试环境地址、凭据均从本地环境配置读取，不入库；文档使用 `<YOUR_TEST_DOMAIN>` 占位，无真域名、IP、口令或个人路径。
