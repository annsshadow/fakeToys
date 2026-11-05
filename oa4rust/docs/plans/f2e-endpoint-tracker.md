// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

# F2-E 越权读端点台账（按端点逐条推进）

状态：`待修` / `已修` / `已证明不可修（需业务裁决）`。
每修一条：先证明泄露（handler 级原文），再改归属过滤，补双向用例，
破坏实验（去掉过滤）必须红，最后独立提交。

| # | 端点 | 归属字段 | 现状 | 提交 |
|---|---|---|---|---|
| E1 | `GET .../attendancedetail/filter/list` | `person_id` | 已修（管理员全公司 / 本人） | F2-1 `fix(attendance)` |
| E1b | `GET .../attendancedetail/filter/list/{id}/next\|prev/{count}` | `person_id` | 已修 | F2-1 |
| E2 | `GET .../attendancedetail/mobile/filter/list/page/{p}/count/{c}` | `person_id` | 已修 | F2-1 |
| E3 | `PUT .../attendanceappealInfo/filter/list/...` | 请求体 `personId` | 待修：与 Session 比对（管理员豁免） | — |
| E4 | `PUT .../attendanceselfholiday/filter/list/...` | 请求体 `personId` | 待修：同上 | — |
| E5 | `PUT .../statisticshow/filter/personMonth/list/...` | 请求体 `personId` | 待修：同上 | — |
| E6 | `PUT /api/mind/.../mind/filter/list/...`（receive/recycle/shared 同族） | `creator` | 待修 | — |
| E7 | `GET /api/message/.../instant/list/...` | `consume`/`sender` | 待修 | — |
| E8 | `GET /api/message/.../consume/list/{consume}/count/{c}` | 路径 `consume` | 待修：等于本人或管理员 | — |
| E9 | `GET /api/meeting/.../attachment/list/{id}/next/{count}` | 会议 owner/参与人 | 待修 | — |
| E10 | `GET /api/meeting/list` | `creator`/参与人 | 待修 | — |
| E11 | `GET /api/ai/chat/list/paging/...` | `person` | 待修 | — |
| E12 | `GET /api/ai/conversation/list` | `user_id` | 待修 | — |
| E13 | `PUT /api/anonymous/document/filter/list/...` | `status`/`creator` | **待修（安全优先）**：匿名可达 + 泄露他人 draft | — |
| my-1 | `POST .../draft/list/my/paging/...` | `xperson` | 待修 | — |
| my-2 | `POST .../work/list/my/paging/...` | `creator` | 待修 | — |
| my-3 | `GET .../task/list/my/paging/...`（taskcompleted/read/review/handover 同构） | `person` | 待修 | — |

## 口径约定

- 管理员：`shared::middleware::is_admin` 为真 → 看全公司；
- 普通用户：按上表归属字段等于 `session.person_unique`；
- 请求体/路径里带人员 id 的端点（E3/E4/E5/E8）：普通用户传他人 id → 403；
  管理员可代查；
- `/my/` 语义：无条件按会话人过滤，不做管理员豁免（管理员要看全量走管理端点）。

## 纪律：判据不退化

守卫必须能区分「修好了」和「没修」；断言本身不能依赖运气、真实钟点或环境。

范式（来自 503→401 论证，2026-11-05）：

1. **正向断言**：改后行为（匿名 → 401）。
2. **反证一（路由存在性）**：同一 helper 对未注册路径必须 404 → 证明 401 不是「路由不在也通过」。
3. **反证二（不吞故障）**：注入会话但依赖仍不可用 → 仍 503 → 证明 401 只是优先级，不是掩盖依赖故障。
4. **时间类判据**：注入 `now` 或构造跨边界数据，任何时刻跑都断言同一件事；
   只在特定钟点有效（读墙钟）的守卫等于没有守卫。
5. **破坏实验**：把修复逐条撤回，对应断言必须红；红的位置要落在行为断言上，
   不能只落在常量断言上（否则证明不了行为被覆盖）。

## 时区同族扫描（2026-11-06）

背景：`processplatform_service_processing` 维护作业按业务日去重时，用
`date_trunc('day', NOW() AT TIME ZONE 'Asia/Shanghai')` 得到「北京墙钟 naive」，
直接与 `x_record.create_time`（UTC 裸时间）比较 → 北京 00:00–08:00 必现重复插入、
其余时段必不现（按钟点决定红绿）。修法：边界换算到 UTC 再比较，写入时基不动。

| 位置 | 形态 | 判定 | 处置 |
|---|---|---|---|
| `processplatform_service_processing/src/u2.rs`（`loglongdetained` / `touchdelay`） | ① 本地时区算边界 vs naive 列 | **确认为缺陷** | **已修**：`business_day_start_utc(now)`；写入仍 `$2`=UTC naive；用例注入固定 `now` |
| `attendance_assemble_control/src/lib.rs:8329/8457/8828` | ① 反向：UTC 算「今天」vs 本地日历列 | **同族缺陷**（北京 00:00–08:00 默认考勤日错一天） | 未并入本 PR；随 P0 考勤一起修（下一项就是该 crate） |
| `bbs_core_entity/src/lib.rs:35-37`（`now_text`，3 处写 `create_time`） | ② 写本地时间，同列另有 UTC 写入路径（`bbs_assemble_control::now_iso`） | **同族缺陷**（容器 TZ=UTC 时不显现，开发机 CST 显现；TEXT 列按字典序排序会混基） | 未并入本 PR；随 T3 bbs 批次做 |
| `shared/src/response.rs:225-227`（`legacy_date_now`） | ② 读/格式化：响应 `date` 用本地时间，库内是 UTC | 非同族（不参与比较），但疑为 T3-4「API 比落库快 8 小时」的根 | 留待 T3-4 定「落库错 or 读出错」后再修 |
| `file/src/lib.rs:406/410/481/485/691/743/814`；`console/src/lib.rs:221` | 写本地时间到 **响应 JSON** | 非缺陷（不落库、不比较） | 不改 |
| `program_center_core_entity/.../application.rs:216` | 写本地时间到 TEXT `deleted_at` | 该列只做非空判定/展示，无跨基比较 | 不改 |
| `program_center/src/lib.rs`（`DATE(create_time) = $2::date`） | 按天过滤 | 两侧同基（`create_time` 为 UTC naive，`$2` 由调用方传入 `NaiveDate`） | 不改 |
| 全仓 `AT TIME ZONE` / `date_trunc('day'` / `CURRENT_DATE` / `LOCALTIMESTAMP` | — | 除上表外无其它命中（`date_trunc('second')` 仅截秒文本，无关） | — |

## 已知相邻（不并入本轮）

- E9 附件若需「会议参与人」判定，要先确认 `x_meeting` 参与人表结构，单独评估；
- `query table_flag=$1`、file 列表归属过滤、org `next`（实为 offset）为**负证据**，不改。
