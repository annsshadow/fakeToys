# 分片 A 读越权（IDOR）：需裁决 / 只登记清单

范围：`hotpic_*`、`mind_*`、`bbs_*`、`meeting_*` 的**读**端点。
依据：`docs/audits/behavioral-audit-2026-11-05/ownership-list-candidates.txt`（`SESSION=False` 且 `USES_ME=False` 的候选）。
统一口径：

1. 读权限「本人 OR 管理员」；归属列为 `NULL` / 历史无归属的行 → 仅管理员可读（fail-closed）。
2. 归属列根本不存在（NO-OWNER-COL）→ 不改代码、不发明语义，只登记并写明缺哪一列。
3. 关联边（participant / attendee 之类）而非单一 `creator` → 只收紧 `creator` 一侧；关联边一侧做实证并登记。

> 本文件只记录**未修复**项；已修复项见提交历史与本文件「已修复」一节。
> 这里不放松任何门禁：`security_regression_baseline.txt` 不新增条目。

## 已修复

| 批次 | 模块 | 端点 | 归属谓词 |
|---|---|---|---|
| 1 | `meeting` | `get_meeting` | `x_meeting.creator = 会话` OR 管理员（SQL WHERE） |
| 1 | `meeting_core_entity` | `get_meeting` | 同上 |
| 1 | `meeting_assemble_control` | `meeting_id` | 同上 |
| 1 | `meeting_assemble_control` | `u2_attachment_get` | `person = 会话` OR 所属会议 `creator` OR 管理员 |
| 1 | `mind` | `get_mind_with_id` | 创作者 OR 管理员 |
| 2 | `mind` | `list_my_folders` | 创作者 OR 管理员 |
| 2 | `mind` | `list_versions_with_mind_id` | 所属 `mind_base_info.creator` OR 管理员 |
| 2 | `meeting_assemble_control` | `meeting_list_applied_wait` / `_processing` / `_completed` / `meeting_list_apply_page_size_size` | `x_meeting.creator`（= 申请人）OR 管理员 |

## 一、NO-OWNER-COL（缺归属列，不改代码）

实测（`oa4rust_test` 的 `information_schema.columns`）：

- `x_meeting_building`：列 `id, name, pinyin, pinyin_initial, address, description, create_time, order_number` —— **缺 `creator`，也无 `person` 等任何等价归属列**。
- `x_meeting_room`：列同上 —— **缺 `creator`，也无等价归属列**。

受影响读端点（全部无会话、无归属过滤）：

- `GET /api/meeting/assemble/control/building/{id}` → `meeting_assemble_control::building_id`
- `GET /api/meeting/assemble/control/building/list`、`/building/list/like/{key}`、`/building/list/like/pinyin/{key}`、`/building/list/pinyininitial/{key}`
- `GET /api/meeting/assemble/control/room/{id}` → `meeting_assemble_control::room_id`
- `GET /api/meeting/assemble/control/room/list`、`/room/list/like/{key}`、`/room/list/like/pinyin/{key}`、`/room/list/pinyininitial/{key}`、`/room/photo/{id}`

裁决点：楼宇 / 会议室是否为**租户级全局配置**（所有登录用户都需要看到同一份资源目录）。若确认公共资源，则保持现状并记为「公开资源读面」；若需要按归属收敛，必须先给两张表补归属列并回填，才能谈谓词。

## 二、写侧缺陷阻塞读侧收紧（`creator = "system"`）

`mind_assemble_control::save_folder`（`mind_assemble_control/src/lib.rs:252`）在 `:264` 硬编码：

```rust
let creator = "system";
```

随后 `:268` 以该值 `INSERT INTO x_mind (id, name, content, creator, create_time)`。

影响面：

1. **归属被抹平**：经该端点创建的所有文件夹在库里都记成 `system`，不是真实用户。此时给读端点加「`x_mind.creator = 会话` OR 管理员」会把**全部历史文件夹对普通用户隐藏**（fail-closed 把所有行都挡掉），属于破坏性行为变更，不能直接做。
2. **责任链断**：任何登录用户都能以 `system` 身份落库，审计日志与「谁建的」对不上；`system` 既可能是系统任务也可能是任意用户。
3. **读写不一致**：同一 crate 的版本写路径（`mind_assemble_control/src/u2.rs:436`）用的是会话 `person`，文件夹写路径却是 `system` —— 两条路径口径分叉。

受影响读端点（本次**不修**）：

- `mind_assemble_control::list_folders`（`src/lib.rs:151`，SQL 在 `:156`，`FROM x_mind WHERE parent_id IS NULL`）
- `mind_assemble_control::get_folder`（`src/lib.rs:203`，SQL 在 `:211`）
- `mind_assemble_control::u2::mind_filter_list`（`src/u2.rs:108`，`FROM x_mind`）
- `mind_assemble_control::u2::mind_version_list`（`src/u2.rs:268`，SQL 在 `:272`，`FROM x_mind_version_info WHERE mind_id = $1`）

建议顺序（需业务裁决后再动）：

1. 先把 `save_folder` 的 `creator` 改为会话身份（写侧修复）；
2. 再回填历史行的归属（否则历史文件夹仍然只有管理员可见）；
3. 最后给上述读端点加归属谓词（版本列表可参照 `mind` crate 已修版本：join `x_mind.base_info.creator`）。

在 1、2 完成前给读端点加谓词 = 直接把用户现有文件夹读没了，故本次仅登记。

## 三、关联边（invitee / 参与人）——只收紧 `creator` 一侧

实证（`oa4rust_test` 的 `information_schema.columns`）：

- 关联表 `x_meeting_invite` 存在，列为 `id, meeting_id, invitee, status, create_time, deleted_at`，PK `id`，索引 `meeting_id`。
- 但下列 handler 只查 `x_meeting`，**没有 join `x_meeting_invite`**，用的是会议自身的 `invited` 布尔位：

| handler | 位置 | 现状 SQL |
|---|---|---|
| `meeting_list_invite_page_size_size_core` | `meeting_assemble_control/src/lib.rs:1259` | `FROM x_meeting ... invited = true` |
| `meeting_list_invited_completed` | `:1331` | `FROM x_meeting WHERE status='completed' AND invited=true` |
| `meeting_list_invited_processing` | `:1383` | 同上（`processing`） |
| `meeting_list_invited_rejected` | `:1435` | 同上（`rejected`） |
| `meeting_list_invited_wait` | `:1487` | 同上（`wait`） |

结论：这批不是 `creator` 语义，而是「我是被邀请人」的**关联边**。按统一口径第 3 条不做 `creator` 收紧；正确的收敛谓词应是：

```sql
AND EXISTS (SELECT 1 FROM x_meeting_invite i
             WHERE i.meeting_id = x_meeting.id AND i.invitee = $会话)
```

需裁决：`x_meeting_invite` 与 `x_meeting.invited/status` 谁是权威来源（两者可能不一致）。在裁决前仅登记。

### 主列表 / 日程列表（无归属过滤，语义未定）

| handler | 位置 | 现状 |
|---|---|---|
| `meeting_assemble_control::meeting_list_page_size_size` | `src/lib.rs:2096`（SQL `:2150`） | `FROM x_meeting ORDER BY create_time DESC LIMIT $1 OFFSET $2`，无任何归属条件 |
| `meeting_list_page_size_size_manage(_core/_p3)` | `:2151` | 同上（管理视图） |
| `meeting::list_meetings` | `meeting/src/lib.rs:391` | 无会话、无归属过滤 |
| `meeting::list_schedule` | `meeting/src/lib.rs:571` | 同上 |

裁决点：这些是「全公司会议总览 / 日程视图」还是「我的会议」。若是后者，按统一口径可收紧 `creator`（本人 OR 管理员）；若是前者，应保留但明确记为公开读面，或改为需要管理员角色。当前无会话参数，加谓词前先定语义。

## 四、公开内容读面（不做归属过滤）

按统一口径登记为**公开内容**，不套「本人 OR 管理员」：

- **bbs**（19 个候选）：`x_bbs_forum`、`x_bbs_topic`、`x_bbs_reply` 为论坛公开内容；`list_forums`、`list_topics_by_forum`、`reply_filter_list_*`、`list_topics_creamed`、`list_topics_recommended`、`list_subjects_*`、`subject_filter_listsubjectinfo`、`u2_subject_listsubjectinfo_page`、`u2_reply_filter_list`、`u2_voterecord_list` 等。
  例外需复核：`user_forum_list`（`bbs_assemble_control/src/lib.rs:1804`）、`user_subject_list`（`:2025`）、`user_reply_list`（`:1911`）、`u2_userinfo_filter`（`src/u2.rs:2397`）命名为「user / 本人」，但实现无会话过滤；若语义确为「我的帖子 / 我的回复」，应另立条目按 `creator` 收紧，而不是当公开内容放过。
- **hotpic**（7 个候选）：`x_hotpic` 为展示位内容，`hotpic::list_by_application_and_info_id`、`hotpic_assemble_control::list_control_panels` / `list_control_applications` / `list_hotpics` / `cipher_hotpic_filter_list_*` / `user_hotpic_filter_list_*` 属公开展示读面；`user_hotpic_filter_list_*` 命名含 `user`，同样需确认是否应限制到本人。

## 变更纪律（本分片）

- 归属谓词一律写进 SQL `WHERE`，禁止「先查后判」。
- 每条修复附双向用例（A→B、B→A 均拒绝）+ 本人 200 且返回落库内容的阳性对照。
- 每条修复附破坏实验：摘掉 / 恒真谓词后用例必红，附红原文，然后还原。
- 非 PASS 只允许单调下降；能减基线就点名减；不新增基线条目。
