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
| 3（写侧） | `mind_assemble_control` | `save_folder` | `x_mind.creator` 落**会话身份**（原硬编码 `"system"`） |
| 4 | `mind_assemble_control` | `u2::mind_view` / `u2::mind_get` | `creator=会话` OR `shared=true` OR `x_mind_share` 接收人 OR 管理员（SQL WHERE） |
| 4 | `mind_assemble_control` | `u2::mind_version_list` / `u2::mind_version_get` | 同上，按所属 `x_mind` 行判定（版本表无 shared/共享边） |

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

## 二、写侧已修：`save_folder` 的归属（原 `creator = "system"`）

### 已修

`mind_assemble_control::save_folder`（`mind_assemble_control/src/lib.rs:252`）原先在 `:264` 硬编码：

```rust
let creator = "system";
```

任何登录用户都能以 `system` 落库：归属被抹平、责任链断、且同一 crate 的版本写路径（`src/u2.rs:436`）用的是会话 `person`，两条路径口径分叉。第三批已改为：

```rust
let creator = session.person_unique.clone();
```

用例 `save_folder_stamps_session_creator`（`src/security_gate_tests.rs`）断言新行 `creator = 会话` 且不等于 `system`；破坏实验（把该行改回恒 `"system"`）红原文：

```
assertion `left == right` failed: 写侧归属必须落地会话身份，实得 Some("system")
  left: Some("system")
 right: Some("u-sec-read-a-writer")
```

同 crate 其余 `"system"` / `"admin"` 常量身份已复核：`grep '"system"|"admin"'` 仅命中上述一处，无第二处。

### 加读谓词前的量化（结论：本批不加）

测量口径（生产库按此跑）：

```sql
SELECT count(*) AS total,
       count(*) FILTER (WHERE creator = 'system') AS system_rows,
       count(*) FILTER (WHERE creator IS NULL)   AS null_rows
FROM x_mind WHERE deleted_at IS NULL;
```

本地可取到的数据：

| 库 | x_mind 行数 | `creator='system'` | 说明 |
|---|---|---|---|
| `oa4rust`（本地开发库） | 0 | 0 | 无数据 |
| `oa4rust_test`（测试库） | 0 | 0 | 无数据 |
| `oa_audit_20261105`（审计快照） | 6 | 4（67%） | 均为 `W2/w3-mind/W4mind-*` 审计夹具，非生产 |

结论：**没有可用于决策的生产样本**；唯一有数据的审计库中 `system` 占 67%，方向上支持「占多数」的判断，但不能当生产比例。按用户口径取保守分支：**本批不加文件夹读谓词**，避免把用户自己历史文件夹读没。当将来加谓词时，行为约定为：

1. **写入已修** → 新文件夹 creator = 会话，可被本人读；
2. **历史行不回填**（无可信来源）→ `creator='system'` 的老行仅管理员可见（fail-closed）；
3. 上线前必须先跑上面的量化 SQL 并在变更单里写明受影响用户数，不得为「减少非 PASS 数字」直接上谓词。

受影响读端点（本批**仍不修**）：

- `mind_assemble_control::list_folders`（`src/lib.rs:151`，SQL `:156`，`FROM x_mind WHERE parent_id IS NULL`）
- `mind_assemble_control::get_folder`（`src/lib.rs:203`，SQL `:211`）
- `mind_assemble_control::u2::mind_filter_list`（`src/u2.rs:108`，`FROM x_mind`；且会把 `x_mind` 里的文件夹行一并列出，收紧前需先区分「文件夹 / 脑图」两类行）

### 脑图 / 版本读（第四批已修；共享边保留，不放宽也不误杀）

`x_mind` 同时存文件夹与脑图。脑图行的共享语义已在代码中实证，无需业务裁决：

- 表 `x_mind_share(id, mind_id, person, create_time)` 是**按人**的共享边；`mind_filter_received` 已按 `s.person = $会话` 读该表；
- 表 `x_mind.shared` 是业主主动置 `true` 的**公开共享**标记，并由专门的写 handler 置位；
- 因此读谓词 = `creator = 会话 OR shared = true OR EXISTS(x_mind_share.person = 会话) OR 管理员`。共享者/接收人仍可读，非共享第三方 403。

已修端点：

| handler | 位置 | 谓词 |
|---|---|---|
| `u2::mind_view` | `src/u2.rs:695` | `MIND_ROW_READ_PREDICATE`（如上），空结果再查存在性区分 403 / 404 |
| `u2::mind_get` | `src/u2.rs:705` | 与 `mind_view` 共用 `mind_read` |
| `u2::mind_version_list` | `src/u2.rs:290` | `MIND_VERSION_READ_PREDICATE`（按所属 `x_mind` 行）；空结果区分 403 / 空列表 |
| `u2::mind_version_get` | `src/u2.rs:608` | 同上；无权 403，不存在保持既有 `version not found` |

用例 `mind_read_idor_is_blocked_both_ways` / `mind_version_read_idor_is_blocked_both_ways` 覆盖：双向 403、属主 200 且返回落库行、`shared=true` 与 `x_mind_share` 接收人 200、无会话 401。破坏实验（行谓词 / 版本谓词改 `{actor} = {actor}` 恒真）分别红于「A 越权读 B 的私有脑图必须 403」「越权读他人版本列表必须 403」。

仍未修（阻塞点不同）：`list_folders` / `get_folder` / `mind_filter_list` 见上一节（历史 `creator='system'` 行占比未知）。

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

实测行数与覆盖（本地）：

| 库 | `x_meeting_invite` 行数 | 覆盖会议数 | 覆盖被邀请人数 | `x_meeting.invited=true` 会议数 |
|---|---|---|---|---|
| `oa4rust`（本地开发库） | 0 | 0 | 0 | 0（`x_meeting` 仅 8 行测试数据） |
| `oa_audit_20261105`（审计快照） | 1 | 1 | 1 | 0 |

覆盖结论：**关联表与 `invited` 布尔位没有任何重叠样本**（有邀请边的库里 `invited=true` 为 0；有 `invited` 标记的库不存在——本地两库均为 0），因此无法从数据判断谁是权威来源。生产库请先跑：

```sql
-- 一致性：invited=true 但无任何 invitee 边的会议
SELECT count(*) FROM x_meeting m
 WHERE m.invited = true
   AND NOT EXISTS (SELECT 1 FROM x_meeting_invite i WHERE i.meeting_id = m.id);
-- 反向：有 invitee 边但 invited 非 true 的会议
SELECT count(*) FROM x_meeting m
 WHERE m.invited IS NOT TRUE
   AND EXISTS (SELECT 1 FROM x_meeting_invite i WHERE i.meeting_id = m.id);
```

需裁决：`x_meeting_invite` 与 `x_meeting.invited/status` 谁是权威来源（两者可能不一致）。在裁决前仅登记，不加谓词。

### 主列表 / 日程列表（无归属过滤，语义未定）

| handler | 位置 | 现状 |
|---|---|---|
| `meeting_assemble_control::meeting_list_page_size_size` | `src/lib.rs:2096`（SQL `:2150`） | `FROM x_meeting ORDER BY create_time DESC LIMIT $1 OFFSET $2`，无任何归属条件 |
| `meeting_list_page_size_size_manage(_core/_p3)` | `:2151` | 同上（管理视图） |
| `meeting::list_meetings` | `meeting/src/lib.rs:391` | 无会话、无归属过滤 |
| `meeting::list_schedule` | `meeting/src/lib.rs:571` | 同上 |

裁决点：这些是「全公司会议总览 / 日程视图」还是「我的会议」。当前无会话参数，加谓词前先定语义。两种口径的后果：

- **按「我的会议」收紧（`creator = 会话 OR 管理员`）**：符合统一口径、IDOR 面直接消失；但会议室占用总览、跨部门日程、领导看全员日程等前端场景会**当场看不到数据**（这些列表没有其他入口）。上线前必须确认前端哪些页面用这些端点。
- **按「全公司总览」保留（公开登录读面）**：可用性不变；但需要明确写进安全基线为「有意公开」，并在门禁里登记豁免，否则每轮审计都会重新报 LEAK。若要求更强隔离，可改为「需管理员角色」（`EXISTS(admin)`），代价是非管理员看不到全员日程。
- **折中（推荐待裁决）**：主列表收紧到 `creator OR 管理员 OR EXISTS(邀请边)`，即「我建的 + 我被邀请的」；仍会改变「全员总览」语义，需产品确认。

在裁决前保持现状（只登记）。

## 四、公开内容读面（不做归属过滤）

按统一口径登记为**公开内容**，不套「本人 OR 管理员」：

- **bbs**（19 个候选）：`x_bbs_forum`、`x_bbs_topic`、`x_bbs_reply` 为论坛公开内容；`list_forums`、`list_topics_by_forum`、`reply_filter_list_*`、`list_topics_creamed`、`list_topics_recommended`、`list_subjects_*`、`subject_filter_listsubjectinfo`、`u2_subject_listsubjectinfo_page`、`u2_reply_filter_list`、`u2_voterecord_list` 等。
  `user_*` 4 条已逐一复核（第三批），结论：**不是「本人」语义，属门户公开列表**，不做 `creator` 过滤——
  - `user_forum_list`（`lib.rs:1804`）：`SELECT ... FROM x_bbs_forum WHERE deleted_at IS NULL ORDER BY sort`，返回全部板块；
  - `user_subject_list`（`lib.rs:2025`）：`FROM x_bbs_topic WHERE deleted_at IS NULL`，返回全部主题；
  - `user_reply_list`（`lib.rs:1911`）：`FROM x_bbs_reply WHERE deleted_at IS NULL`，返回全部回复；
  - `u2_userinfo_filter`（`u2.rs:2397`）：按昵称模糊查 `x_bbs_user_info`，属公开用户目录检索。
  四者均无会话参数、SQL 里没有任何按人过滤的意图（`user_` 指「门户/用户端」，不是「我的」）。若产品确认其中某条应改为「我的」，需先补会话与 `creator=会话 OR 管理员` 谓词，再另立条目。
- **hotpic**（7 个候选）：`x_hotpic` 为展示位内容，`hotpic::list_by_application_and_info_id`、`hotpic_assemble_control::list_control_panels` / `list_control_applications` / `list_hotpics` / `cipher_hotpic_filter_list_*` / `user_hotpic_filter_list_*` 属公开展示读面；`user_hotpic_filter_list_*` 命名含 `user`，同样需确认是否应限制到本人。

## 变更纪律（本分片）

- 归属谓词一律写进 SQL `WHERE`，禁止「先查后判」。
- 每条修复附双向用例（A→B、B→A 均拒绝）+ 本人 200 且返回落库内容的阳性对照。
- 每条修复附破坏实验：摘掉 / 恒真谓词后用例必红，附红原文，然后还原。
- 非 PASS 只允许单调下降；能减基线就点名减；不新增基线条目。
