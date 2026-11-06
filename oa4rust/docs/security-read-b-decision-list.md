# 读 IDOR 分片 B 裁决清单（批次 2）

适用范围：`calendar_*`、`organization_*`（control/personal/express）、`message_*` 的读路径。
默认口径：本人 OR 管理员；`is_public` 视为已公开发布；归属列为 NULL 时 fail-closed（仅管理员）。

本清单只记录**本批不动**或**需业务/契约裁决**的项；包含表、端点、缺哪一列/哪条边、原因。

## NO-OWNER-COL（表内没有归属列，未改）
| 表 | 端点 | 缺失情况 | 处理 |
|---|---|---|---|
| `x_message_conversation` | `im_conversation_id`、`im_conversation_id_single`、`im_conversation_id_group`（message_assemble_communicate） | 无 `creator`/`owner`/`person_id` 等归属列；成员关系只在 `x_message_conversation_member`（关联边） | 未加谓词，未发明归属语义。需裁决“会话归属 = 创建者 / 成员集合 / 其他”后再落 SQL |
| `x_cal_event` | 日历事件读路径（按 `calendar_id` 读取） | 事件表无单一 owner 列；归属来自 `calendar_id` 关联边 | 未改契约；只做了实证。需裁决是否继承日历 owner 或按参与者判定 |

## 关联边（只收紧 owner 一侧，关联边侧仅实证）
- IM：`x_message_conversation_member(person_id, conversation_id, role)` 是唯一访问依据，本批未改。
- 日历：`cal_event.calendar_id -> x_cal_calendar.id`；本批只修日历本体读，事件侧待裁决。
- 日历公开性：`x_cal_calendar.is_public=true` 视为已公开发布，全员可读；`false` 仅 `createor` 本人或管理员。`status` 仍只允许 `OPEN`。

## 列存在但生产写路径不写入
| 表 | 列 | 实测 | 影响 |
|---|---|---|---|
| `x_message` | `creator`、`creator_person` | 所有 `INSERT INTO x_message` 只写 `sender`/`content`/`type`/`create_time` 等，不写这两列 | 本批以 `sender` 作为 owner 收紧列表读；若后续产品需要“创建者”语义，补写路径需同时写入并回填历史行 |
| `x_org_person` | `creator` | 创建路径会写入（`u2_person.rs`、`u2_misc.rs`）；历史行可能为 NULL | 按口径 NULL → 仅管理员可读（fail-closed），未猜测归属 |

## 本批未覆盖（同判据，留待下一批）
`organization_assemble_control` 内还有多条人员读端点返回 `mobile`/`email` 等 PII，未在本批 5 条额度内：
`person_list_filter_paging`、`person_list_next`、`person_list_prev`、`person_list_by_role`、
`person_list_by_group`、`person_by_id`、`person_list_by_ids`、人员属性/自定义字段列表。
它们与 `person_list_like` 同表同判据（`x_org_person.creator` + 管理员），下一批按同一模式收紧。

## 明确不改契约
- 不修改任何表结构、不新增归属列、不回填历史数据。
- 不改变分页/排序参数语义；列表在归属收窄后允许返回空，不额外做存在性探测（避免 403 泄露他人是否存在）。
- 本人正控制（`idor-read-own`）与双向拒绝用例成对存在；列表用例使用按完整 JSON 值匹配 canary，避免 `rev-` 前缀子串误报。
