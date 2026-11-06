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
| `x_message` | `creator`、`creator_person` | 所有 `INSERT INTO x_message` 只写 `sender`/`content`/`type`/`create_time` 等，不写这两列 | 写侧缺陷（与 `mind.system` 同类），不是读侧可独立修好的问题。修复顺序：**① 先补写入**（所有 INSERT 同时写 `creator`/`creator_person`，并确认取值是 person id/unique_id）；**② 新数据产生归属**；**③ 历史行仍按 NULL → 仅管理员可读**，不猜测、不回填。完成前本批以 `sender` 作为 owner 收紧列表读 |
| `x_org_person` | `creator` | 创建路径会写入（`u2_person.rs`、`u2_misc.rs`）；历史行可能为 NULL | 按口径 NULL → 仅管理员可读（fail-closed），未猜测归属 |

## 本批未覆盖（同判据，留待下一批）
`organization_assemble_control` 内还有多条人员读端点返回 `mobile`/`email` 等 PII，未在本批 5 条额度内：
`person_list_filter_paging`、`person_list_next`、`person_list_prev`、`person_list_by_role`、
`person_list_by_group`、`person_by_id`、`person_list_by_ids`、人员属性/自定义字段列表。

判定标准不是“有没有 creator”，而是**谁能看到别人的姓名/手机号/邮箱**。若某端点无法用
`creator` + 管理员口径明确回答（例如按单位/群组的公共通讯录语义、或业务上要求全员可见），
宁可登记待裁决，不硬改；可用默认口径的（本人行或管理员）按 `x_org_person.creator` 收紧。

## 门禁用例类型语义（批次 2 新增，新增用例按此写）
用例表新增可选两列：第 13 列 `rev_id`（rev 方向资源 id，缺省 `rev-{victim}`），
第 14 列 `body`（请求体 JSON 字面量，缺省 `{}`）。

| kind | 判定为 PASS | 判定为 LEAK / 失败 |
|---|---|---|
| `idor-read`（原有，单资源） | HTTP 401/403；或 2xx 且响应体不含 victim/rev canary | 2xx 且含 canary |
| `idor-read-scoped`（新增，按归属收窄的列表） | HTTP 401/403；或 2xx（只返回本人行，不含 victim canary）。canary 按**带引号的完整 JSON 值**匹配（`"canary"`），避免本人行 `rev-` 前缀被裸子串误判 | 2xx 且响应体出现 `"victim_canary"`（fwd）或 `"rev-victim_canary"`（rev） |
| `idor-read-own`（新增，本人正控制） | 2xx 且响应体出现**自己的** canary（完整 JSON 值） | 401/403、非 2xx、或 2xx 但没有自己的 canary → UNVERIFIED（说明“本人也看不到”，不允许算绿） |

写用例要求：
- 每条拒绝用例必须同时有 fwd/rev（`dir=both`），且配一条 `idor-read-own`，否则列表类会因为“恒空”假绿。
- `idor-read-scoped` 允许 2xx 空列表/本人列表；**不允许**把 401/403 以外的不含 canary 的 2xx 当作“修好了”而不配正控制。
- 新增 canary 时，victim 与 attacker 的值必须互为非子串（推荐 victim=`GATE-V-*`、attacker=`rev-GATE-V-*`），并配合引号匹配。

## 明确不改契约
- 不修改任何表结构、不新增归属列、不回填历史数据。
- 不改变分页/排序参数语义；列表在归属收窄后允许返回空，不额外做存在性探测（避免 403 泄露他人是否存在）。
- 本人正控制（`idor-read-own`）与双向拒绝用例成对存在；列表用例使用按完整 JSON 值匹配 canary，避免 `rev-` 前缀子串误报。
