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

## 已知相邻（不并入本轮）

- E9 附件若需「会议参与人」判定，要先确认 `x_meeting` 参与人表结构，单独评估；
- `query table_flag=$1`、file 列表归属过滤、org `next`（实为 offset）为**负证据**，不改。
