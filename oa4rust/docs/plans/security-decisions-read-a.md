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
| 5 | `meeting_assemble_control` | `meeting_list_invited_wait` / `_processing` / `_completed` / `_rejected` / `meeting_list_invite_page_size_size(_p3)` | 两源并集：`creator=会话` OR `EXISTS(x_meeting_invite.invitee=会话)` OR `invited=true` OR 管理员（SQL WHERE） |
| A-b1（#226） | `cms_assemble_control` | `anonymous_document_id_view` / `document_persons_u3` | `x_cms_data_document`：`status='published'` OR `creator`/`author_id`=会话 OR 管理员（SQL WHERE） |
| A-b1（#226） | `cms_assemble_control` | `file_id` / `file_id_content` / `file_id_download` | `x_cms_file.creator`=会话 OR 管理员（SQL WHERE） |
| A-b1（#226） | `cms_assemble_control` | `comment_id` | 评论作者=会话 OR 可读所属文档（published / creator / author_id）OR 管理员（SQL WHERE） |
| A-b2（#234） | `cms_assemble_control` | `anonymous_form_id` / `form_id` | `x_cms_form`：`status='published'` OR `creator`=会话 OR 管理员（SQL WHERE） |
| A-b2（#234） | `cms_assemble_control` | `formversion_id` / `scriptversion_id` / `templateform_id` | `x_cms_form_v2`：`status='published'` OR `creator`=会话 OR 管理员（SQL WHERE） |
| A-b2（#234） | `cms_assemble_control` | `script_id` | `x_cms_script.creator`=会话 OR 管理员（SQL WHERE，表无 status） |
| A-b3（#250） | `cms_assemble_control` | `application_id` / `appinfo_id` / `appinfo_id_control` | `x_cms_appinfo.creator`=会话 OR 管理员（SQL WHERE） |
| A-b3（#250） | `cms_assemble_control` | `categoryinfo_id` / `categoryinfo_id_control` | `x_cms_categoryinfo.creator`=会话 OR 管理员（SQL WHERE） |
| A-b3（#250） | `cms_assemble_control` | `commend_id` | `x_cms_commend.person_id`=会话 OR 管理员（SQL WHERE）。**映射说明**：该表无 `creator` 列，`person_id` 是唯一人员列，即归属列 |
| A-b4（#262） | `cms_core_entity` | `article_get` | `x_cms_article`：`status='published'` OR `author_id`=会话 OR 管理员（cms §3，SQL WHERE） |
| A-b4（#262） | `cms_core_express` | `content_detail` | `x_cms_content`：`status='published'` OR `creator`=会话 OR 管理员（cms §3，SQL WHERE） |
| A-b4（#262） | `portal` | `portal_id` | `x_portal.creator`=会话 OR 管理员（E23，SQL WHERE） |
| A-b4（#262） | `portal_assemble_designer` | `portal_id` / `portal_id_icon` / `portal_id_permission` | `x_portal.creator`=会话 OR 管理员（E23，SQL WHERE） |
| A-b4（#262） | `portal_assemble_surface` | `get_surface` | `x_portal_surface.creator`=会话 OR 管理员（E23，SQL WHERE） |
| A-b5（#265） | `portal_assemble_surface` | `portal_id_icon` / `portal_id_icon_base64` | `x_portal.creator`=会话 OR 管理员（E23，SQL WHERE）；未命中保留 `surface not found` |
| A-b5（#265） | `portal_assemble_surface` | `preview_surface` / `surface_preview` | `x_portal_surface.creator`=会话 OR 管理员（E23，SQL WHERE） |
| A-b5（#265） | `portal_core_entity` | `page_get` | `x_portal_page.creator`=会话 OR 管理员（E23，SQL WHERE）；本 crate 无 deadpool `Pool` 注入，沿用 sea-orm `require_admin`，查询失败 fail-closed |
| A-b5（#265） | `ai_assemble_control` | `config_get_mcp_flag` | `x_ai_mcp_config.creator`=会话 OR 管理员（E24，SQL WHERE）；该表无 `deleted_at`，不追加软删条件 |

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

## 三、关联边（invitee / 参与人）——第五批已按「两源并集」修复

实证（`oa4rust_test` 的 `information_schema.columns`）：

- 关联表 `x_meeting_invite` 存在，列为 `id, meeting_id, invitee, status, create_time, deleted_at`，PK `id`，索引 `meeting_id`。
- 修复前下列 handler 只查 `x_meeting`，**没有 join `x_meeting_invite`**，用的是会议自身的 `invited` 布尔位：

| handler | 位置（第五批后） | 修复前 SQL |
|---|---|---|
| `meeting_list_invite_page_size_size_core` | `meeting_assemble_control/src/lib.rs:1353` | `FROM x_meeting ... invited = true` |
| `meeting_list_invited_completed` | `:1437` | `FROM x_meeting WHERE status='completed' AND invited=true` |
| `meeting_list_invited_processing` | `:1499` | 同上（`processing`） |
| `meeting_list_invited_rejected` | `:1561` | 同上（`rejected`） |
| `meeting_list_invited_wait` | `:1623` | 同上（`wait`） |

实测行数与覆盖（本地）：

| 库 | `x_meeting_invite` 行数 | 覆盖会议数 | 覆盖被邀请人数 | `x_meeting.invited=true` 会议数 |
|---|---|---|---|---|
| `oa4rust`（本地开发库） | 0 | 0 | 0 | 0（`x_meeting` 仅 8 行测试数据） |
| `oa_audit_20261105`（审计快照） | 1 | 1 | 1 | 0 |

覆盖结论：**关联表与 `invited` 布尔位没有任何重叠样本**，单看数据无法判定谁是权威来源。

### 裁决（用户 2026-11-06）：取两源并集，先保可用性

```sql
AND (creator = $会话
     OR EXISTS (SELECT 1 FROM x_meeting_invite i
                 WHERE i.meeting_id = x_meeting.id AND i.invitee = $会话
                   AND i.deleted_at IS NULL)
     OR invited = true
     OR EXISTS(...管理员...))
```

理由：零重叠样本下任一单源都可能挡掉受邀人；并集只漏放、不误挡。等真实数据出来再收敛到唯一权威源。

实现：常量 `INVITED_READ_PREDICATE`（`src/lib.rs:824`，模板 `{actor}`→`$1`/`$3`），5 个入口全部加 `require_session`（无会话 401）并把谓词写进 SQL `WHERE`。

用例 `invited_list_is_scoped_and_session_required`（`src/security_gate_tests.rs:804`）覆盖：
- 正控制：发起人 200 且返回本人发起行；受邀人 200 且返回邀请边命中行；
- 双向拒绝：发起人看不到他人发起且未邀请本人的会议；受邀人看不到未邀请本人的会议；
- 无关者两条归属会议均不可见；`invited=true` 行对任意会话可见（并集口径，记录在案）；
- 五个入口无会话一律 401。

破坏实验红原文（两次均已还原）：

```
# 谓词恒真（{actor} = {actor}）
不得返回他人发起且未邀请本人的会议（wait）

# 摘掉邀请边分支（只留 creator / invited / admin）
受邀人必须看到邀请边命中的会议（wait）
```

### 两源命中计数

运行时：`log_invited_source_counts`（`src/lib.rs:838`）在 DEBUG 级日志输出 `creator` / `invite_edge` / `invited_flag` 三个命中数（关闭 DEBUG 时不产生额外查询）。

一次性统计（生产库直接跑）：

```sql
SELECT count(*) FILTER (WHERE creator = $会话)                    AS creator_hits,
       count(*) FILTER (WHERE EXISTS(SELECT 1 FROM x_meeting_invite i
                                      WHERE i.meeting_id = x_meeting.id AND i.invitee = $会话
                                        AND i.deleted_at IS NULL)) AS invite_edge_hits,
       count(*) FILTER (WHERE invited = true)                        AS invited_flag_hits
FROM x_meeting
WHERE ($状态::text IS NULL OR status = $状态);
```

一致性 SQL（判定收敛方向）：

```sql
-- invited=true 但无任何 invitee 边的会议
SELECT count(*) FROM x_meeting m
 WHERE m.invited = true
   AND NOT EXISTS (SELECT 1 FROM x_meeting_invite i WHERE i.meeting_id = m.id);
-- 有 invitee 边但 invited 非 true 的会议
SELECT count(*) FROM x_meeting m
 WHERE m.invited IS NOT TRUE
   AND EXISTS (SELECT 1 FROM x_meeting_invite i WHERE i.meeting_id = m.id);
```

### 主列表 / 日程列表（已裁决：全局排期总览，有意公开）

| handler | 位置 | 现状 |
|---|---|---|
| `meeting_assemble_control::meeting_list_page_size_size` | `src/lib.rs:2096`（SQL `:2150`） | `FROM x_meeting ORDER BY create_time DESC LIMIT $1 OFFSET $2`，无任何归属条件 |
| `meeting_list_page_size_size_manage(_core/_p3)` | `:2151` | 同上（管理视图） |
| `meeting::list_meetings` | `meeting/src/lib.rs:391` | 无会话、无归属过滤 |
| `meeting::list_schedule` | `meeting/src/lib.rs:571` | 同上 |

裁决：`list_meetings` / `list_schedule` = **全局会议室排期总览**，语义是「这间房什么时段被占了」，**保持现状、不加 creator 过滤**。
定性为「有意公开（需登录）」的读面，不当作泄漏：收紧成「我的」会让普通员工看不到自己房间的排期，等于废掉功能。
管理视图（`*_manage*`）如需更强隔离，可另立条目改为「需管理员角色」，本分片不动。


## 四、公开内容读面（行为级结论，B6）

实测口径：匿名请求一律先打一次（`Authorization` 缺省），再用非管理员 `audit-user` 的会话打一次。
门禁用例 kind=`public-read-login`：**匿名必须 401/403**，**登录用户必须 2xx**、响应命中正控制对象、
且响应体不得出现 `GATE-V-SETTING`（手机）/`GATE-V-EMAIL`（邮箱）/`GATE-V-NAME`（姓名）任一 canary。
结论仅三种：`PUBLIC-BY-DESIGN` / `SHOULD-BE-PRIVATE` / `UNCLEAR`；无路由的 handler 单列「不可达」。

实测结果：**匿名一律 401，登录后一律可读且无 PII**。因此下表 `PUBLIC-BY-DESIGN` 的语义是
「**登录后公开**（全站任意登录用户可读，不限归属）」，不是「匿名公开」；匿名被全局认证层拒之门外，
PII canary 断言落在登录用户响应上，匿名 401 由同一用例的第一段断言固定。共 28 条可达读面全 PASS，
`sabotage public-read-b6` 逐条点名 28 条 `[LEAK]`。

### bbs 19 条

| # | handler | 路由 | 匿名实测 | 登录后返回字段 | 结论 | canary 用例 |
|---|---|---|---|---|---|---|
| 1 | `list_forums` | GET `/api/bbs/assemble/control/forum/list` | 401 | id,name,description,sort,creator,createTime | PUBLIC-BY-DESIGN | `b6\|bbs\|forum_list` |
| 2 | `forum_view_all` | GET `/api/bbs/assemble/control/forum/view/all` | 401 | 同上 | PUBLIC-BY-DESIGN | `b6\|bbs\|forum_view_all` |
| 3 | `list_topics_by_forum` | GET `/api/bbs/assemble/control/topic/list/forum/{forumId}` | 401 | id,forumId,title,content,creator,createTime | PUBLIC-BY-DESIGN | `b6\|bbs\|topic_list_by_forum` |
| 4 | `subject_view_id` | GET `/api/bbs/assemble/control/subject/view/{id}` | 401 | id,forumId,title,content,creator,createTime | PUBLIC-BY-DESIGN | `b6\|bbs\|subject_view` |
| 5 | `subject_top_sectionId` | GET `/api/bbs/assemble/control/subject/top/{sectionId}` | 401 | 同上 | PUBLIC-BY-DESIGN | `b6\|bbs\|subject_top` |
| 6 | `reply_list_sub_id` | GET `/api/bbs/assemble/control/reply/list/sub/{id}` | 401 | id,topicId,content,creator | PUBLIC-BY-DESIGN | `b6\|bbs\|reply_list_sub` |
| 7 | `u2_reply_get` | GET `/api/bbs/assemble/control/reply/{id}` | 401 | id,topicId,content,creator,accepted | PUBLIC-BY-DESIGN | `b6\|bbs\|reply_get` |
| 8 | `u2_reply_filter_list` | PUT `.../reply/filter/list/page/{page}/count/{count}` | 401 | total,data{id,topicId,content,creator} | PUBLIC-BY-DESIGN | `b6\|bbs\|reply_filter_list` |
| 9 | `list_reply_filter` | GET `/api/bbs/assemble/control/list/reply/filter` | 401 | 同上 | PUBLIC-BY-DESIGN | `b6\|bbs\|list_reply_filter` |
| 10 | `subject_creamed_list` | PUT `.../subject/creamed/list/page/{page}/count/{count}` | 401 | id,forumId,title,content,creator,createTime | PUBLIC-BY-DESIGN | `b6\|bbs\|subject_creamed_list` |
| 11 | `topic_recommended_list` | PUT `.../subject/recommended/list/page/{page}/count/{count}` | 401 | 同上 | PUBLIC-BY-DESIGN | `b6\|bbs\|subject_recommended_list` |
| 12 | `topic_recommended_index` | GET `.../subject/recommended/index/{count}` | 401 | 同上 | PUBLIC-BY-DESIGN | `b6\|bbs\|subject_recommended_index` |
| 13 | `subject_index_list` | PUT `.../subject/index/list/page/{page}/count/{count}` | 401 | 同上 | PUBLIC-BY-DESIGN | `b6\|bbs\|subject_index_list` |
| 14 | `subject_filter_list` | PUT `.../subject/filter/list/page/{page}/count/{count}` | 401 | 同上 | PUBLIC-BY-DESIGN | `b6\|bbs\|subject_filter_list` |
| 15 | `u2_subject_listsubjectinfo_page` | POST `.../subject/filter/listsubjectinfo/page/{page}/count/{count}` | 401 | 同上 | PUBLIC-BY-DESIGN | `b6\|bbs\|subject_filter_listsubjectinfo` |
| 16 | `topic_filter_listsubjectinfo` | POST `/api/bbs/assemble/control/topic/filter/listsubjectinfo` | 401 | 同上 | PUBLIC-BY-DESIGN | `b6\|bbs\|topic_filter_listsubjectinfo` |
| 17 | `u2_subject_search_page` | PUT `.../subject/search/list/page/{page}/count/{count}` | 401 | 同上 | PUBLIC-BY-DESIGN | `b6\|bbs\|subject_search_list` |
| 18 | `section_viewforum_forumId` | GET `/api/bbs/assemble/control/section/viewforum/{forumId}` | 401 | id,name,forumId,parentId,sort,description | PUBLIC-BY-DESIGN | `b6\|bbs\|section_viewforum` |
| 19 | `u2_voterecord_list` | PUT `.../user/subject/voterecord/list/page/{page}/count/{count}` | 401 | id,person,optionId,optionName | PUBLIC-BY-DESIGN | `b6\|bbs\|user_subject_voterecord` |

注：`u2_voterecord_list` 的 `person` 是内部身份键（非姓名/手机/邮箱），当前响应不含 PII；
若产品后续认为「谁投了哪项」属敏感，再另立条目收紧，本批按代码现状判公开。

### `user_*` 4 条

| handler | 路由 | 匿名实测 | 登录后返回字段 | 结论 | canary 用例 |
|---|---|---|---|---|---|
| `user_forum_list` | GET `/api/bbs/assemble/control/user/forum/all` | 401 | id,name,description,sort,creator,createTime | PUBLIC-BY-DESIGN | `b6\|bbs\|user_forum_list` |
| `u2_userinfo_filter` | PUT `/api/bbs/assemble/control/userinfo` | 401 | id,person,nickname,icon,signature | PUBLIC-BY-DESIGN | `b6\|bbs\|userinfo_filter` |
| `user_subject_list` | **无路由**（`routes.rs` 未挂载，`tests_generated.rs` 标 SKIPPED） | 不可达 | — | 不可达 | — |
| `user_reply_list` | **无路由**（同上） | 不可达 | — | 不可达 | — |

`user_` 指「门户/用户端」，不是「我的」；可读两条均无会话参数、SQL 无按人过滤意图。
两条无路由 handler 不属于任何读面：本批不新增路由、不删代码，仅登记，后续清理死代码时再处理。

### hotpic 7 条

| # | handler | 路由 | 匿名实测 | 登录后返回字段 | 结论 | canary 用例 |
|---|---|---|---|---|---|---|
| 1 | `list_by_application_and_info_id` | GET `/api/hotpic/user/hotpic/{application}/{infoId}` | 401 | id,application,infoId,title,creator,createTime | PUBLIC-BY-DESIGN | `b6\|hotpic\|list_by_application_info` |
| 2 | `list_hotpics` | GET `/api/hotpic_assemble_control/list/hotpics` | 401 | id,title,imageUrl,creator,createTime | PUBLIC-BY-DESIGN | `b6\|hotpic\|list_hotpics` |
| 3 | `get_hotpic` | GET `/api/hotpic_assemble_control/get/hotpic/{id}` | 401 | 同上 | PUBLIC-BY-DESIGN | `b6\|hotpic\|get_hotpic` |
| 4 | `list_control_panels` | GET `/api/hotpic_assemble_control/list/control/panels` | 401 | creator(name),count | PUBLIC-BY-DESIGN | `b6\|hotpic\|list_control_panels` |
| 5 | `list_control_applications` | GET `/api/hotpic_assemble_control/list/control/applications` | 401 | creator(application),count | PUBLIC-BY-DESIGN | `b6\|hotpic\|list_control_applications` |
| 6 | `cipher_hotpic_filter_list_*` | GET `/api/hotpic/assemble/control/cipher/hotpic/filter/list/page/{page}/count/{count}` | 401 | 同 2 | PUBLIC-BY-DESIGN | `b6\|hotpic\|cipher_filter_list` |
| 7 | `user_hotpic_filter_list_*` | GET `/api/hotpic/assemble/control/user/hotpic/filter/list/page/{page}/count/{count}` | 401 | 同 2 | PUBLIC-BY-DESIGN | `b6\|hotpic\|user_filter_list` |

`user_hotpic_filter_list_*` 名称含 `user`，但实测 SQL 为全局列表（无会话、无 creator 过滤），
故判登录后公开，不做归属过滤。

### 结论汇总

- `PUBLIC-BY-DESIGN`：28 条（19 bbs + 2 可达 `user_*` + 7 hotpic）。
- `SHOULD-BE-PRIVATE`：0 条（没有一条在登录后可读到 PII；匿名已全部 401）。
- `UNCLEAR`：0 条（本批没有缺业务信息而无法落判的条目）。
- 不可达：2 条（`user_subject_list`、`user_reply_list`）。
- PII canary：28 条用例全部靠断言固定，注入式 sabotage 逐条点名。

## 五、mind `creator_unit` 请求体伪造（分片 A 缺陷，不改基线）

三处创建路径原把 `creator_unit` 直接取自请求体 `creatorUnit`：

| handler | 落库表 |
|---|---|
| `mind::create_mind` | `mind_base_info.creator_unit` |
| `mind::create_folder` | `mind_folder_info.creator_unit` |
| `mind::create_version` | `mind_version_info.creator_unit` |

`creator` 已取会话，但单位维度仍可由客户端任意指定：同会话伪造他人 `creatorUnit` 即可把归属落到他人单位。

### 复现（修复前，同一用例）

用例 `security_gate_tests::mind_create_ignores_forged_creator_unit`：会话人 `auth_person.unit = real-unit`，请求体传 `creatorUnit = forged-unit`。

```
assertion `left == right` failed: create_mind 落库单位必须为会话单位
  left: "forged-unit"
 right: "real-unit"
```

（此时 `mind_folder_info` / `mind_version_info` 同样落 `forged-unit`，用例在第一个断言处先红。）

### 修复

新增 `session_unit()`（`mind/src/lib.rs`）：按会话登录人查 `auth_person.unit` 派生，三处创建不再读取请求体 `creatorUnit`。
用例在修复后全绿（`cargo test -p mind --lib`：6 passed）。

### 注

`mind` 三条 `#[ignore]` 读用例（`mind_read_idor_is_blocked_both_ways`、`version_list_idor_is_blocked_both_ways`、`folder_list_idor_is_blocked_both_ways`）在 `main` 基线（`832880cfd`）上实测即失败，与本次改动无关，未触碰、未改基线。
本缺陷不改 `security_regression_baseline.txt`。

## 六、分片 A / A-b1 合入后 §12.6 三步核对（#226）

PR #226（`sec/read-a-cms-b1`）合入 `main`：merge sha `9029bcc6e2c2604d8c7c9153bc874cac0e4e9d95`。
减行前的 6 条基线 id：`anonymous_document_id_view`、`document_persons_u3`、`file_id`、
`file_id_content`、`file_id_download`、`comment_id`。

1. **行是否真在 main**：PR 分支曾 rebase 在旧 `main`，#227 已先合入，`git diff c^1 c^2` 的
   文件清单与当前 `main` 有差异（其它线的 gate 脚本 / baseline 后续改动）。改用逐文件
   `git diff origin/main:<path> <merge>:<path>` 核对：`cms_assemble_control/src/lib.rs`、
   `security_gate_tests_read_a_b1.rs`、`mind` 修复、本台账均为 `SAME`；只有
   `security_regression_gate.py` / `security_regression_baseline.txt` 因后续并行 PR 有差异，
   grep 确认 A-b1 的 sabotage 常量、分支调度与 6 行删除仍在 `main` 上（语义等价形态存在）。
2. **基线是否虚减**：在 `main`（`12179d371`）上跑门禁，6 条被删 id 全部 `[PASS]`，
   剩余非 PASS 105 条与基线一致。
3. **门禁是否接线**：`python scripts/security_regression_gate.py sabotage read-a-b1` 在
   `main` 上变红（exit=101），输出逐条点名 6 条 `[LEAK]`，`实验成立`。
   （首次运行曾疑似 cargo 缓存 / mtime 竞态未红；原命令复跑即通过，已确认。）

三步全过，减行有效。A-b1 用例与 sabotage 均在 `cms_assemble_control`，不涉及其它 crate。

## 七、分片 A / A-b2 合入后 §12.6 三步核对（#234）

PR #234（`sec/read-a-cms-b2`）合入 `main`：merge sha `4571c846a6e6899d857ff3502d1cefbe8ffa45ef`
（squash 单父提交，父 `1a936b6de`）。减行前的 6 条基线 id：`anonymous_form_id`、`form_id`、
`formversion_id`、`script_id`、`scriptversion_id`、`templateform_id`。

1. **行是否真在 main**：`git show --stat 4571c846a` 的文件清单为 `cms_assemble_control/src/lib.rs`
   （+58）、`security_gate_tests_read_a_b2.rs`（新增 416 行）、`security_regression_gate.py`（+33）、
   `security_regression_baseline.txt`（−6）、本台账（+23）。合入后 `main` 顶部即该提交，
   5 个文件内容逐字在 `main`。
2. **基线是否虚减**：合入期间 #244（`9ab774032`，attendance 线）又减了 3 行，`main` 已前进到
   `9ab774032`；在该 main 上重跑门禁，A-b2 的 6 条 id 全部 `[PASS]`，
   剩余非 PASS 37 条与基线一致（leak=23, unverified=14, integrity=0）。
3. **门禁是否接线**：`python scripts/security_regression_gate.py sabotage read-a-b2` 在
   `main`（`9ab774032`）上变红（exit=101），输出逐条点名 6 条 `[LEAK]`，
   `实验成立：门禁变红（exit=101），并点名 6 条用例`。

三步全过，减行有效。

## 八、分片 A / A-b3 合入后 §12.6 三步核对（#250）

PR #250（`sec/read-a-cms-b3`）squash 合入 `main`：`994519dc2`（父 `f041bdf0b`）。
减行前的 6 条基线 id：`appinfo_id_control`、`appinfo_id`、`application_id`、
`categoryinfo_id_control`、`categoryinfo_id`、`commend_id`。

1. **行是否真在 main**：`git log origin/main` 含 `994519dc2`；
   `origin/main:cms_assemble_control/src/lib.rs` 中 `cms_gate_owned_read` 命中 11 处（定义 + 调用）；
   合入后 `origin/main` 基线 `security_regression_baseline.txt` 为 31 条（A-b2 后 37 − 6）。
2. **基线是否虚减**：在 `8c7c2f4b5`（含 #250）派生的本分支上重跑门禁，A-b3 的 6 条 id 全部
   `[PASS]`；本分支同时已减 A-b4 的 6 行，剩余非 PASS 25 条与基线一致（leak=11, unverified=14,
   integrity=0）。
3. **门禁是否接线**：`sabotage read-a-b3` 变红（exit=101），逐条点名 A-b3 的 6 条 `[LEAK]`，
   `实验成立`。同文件共享谓词正则还会波及 `file_id`/`script_id` 等 4 条既有门（同文件其它
   归属门被一并恒真化），不影响 A-b3 的 6 条结论。

三步全过，减行有效。

## 九、分片 A / A-b4 交付与 §12.6 核对记录

分支 `sec/read-a-cms-b4`，base `origin/main=8c7c2f4b5`。本批 6 条：
`cms_core_entity|article_get`、`cms_core_express|content_detail`、`portal|portal_id`、
`portal_assemble_designer|portal_id_icon`、`portal_assemble_designer|portal_id_permission`、
`portal_assemble_surface|get_surface`。同批把 `portal_assemble_designer|portal_id` 也对齐
（不在基线内，避免同表旁路）。

夹具调整：`x_cms_article.status` 列默认值是 `published`，原 seed / rev-seed 未写 status，
会让文章读用例落进「公开可读」分支；两处均已显式写 `status='draft'`，用例才是真正的草稿 IDOR。

1. **行是否真在 main**：已在 `origin/main` 上核对。merge sha `7f83c1051` 被 `origin/main` 包含
   （`git branch -r --contains 7f83c1051` 命中 `origin/main`）；在该提交的
   `security_regression_baseline.txt` 里 grep 本批 6 条 id，全部无命中；同一提交的
   `cms_core_entity/src/lib.rs` 仍含 `published` 读门（3 处命中）。即 6 条是真实修复、不是基线糊过。
2. **基线是否虚减**：rebase 到 `8c7c2f4b5` 后重跑门禁，本批 6 条全部 `[PASS]`；
   合并提交（`7f83c1051`）的基线全文 grep 本批 id 无命中，未靠基线糊过。
3. **门禁是否接线**：`sabotage read-a-b4` 变红（exit=101），输出逐条点名 6 条 `[LEAK]`，
   `实验成立：门禁变红（exit=101），并点名 6 条用例`。

## 十、分片 A / A-b5 交付与 §12.6 核对记录

分支 `sec/read-a-cms-b5`，base `origin/main=8c7c2f4b5`。本批 6 条：
`ai_assemble_control|config_get_mcp_flag`、`portal_assemble_surface|portal_id_icon`、
`portal_id_icon_base64`、`preview_surface`、`surface_preview`、`portal_core_entity|page_get`。
口径：`x_portal` / `x_portal_surface` / `x_portal_page` / `x_ai_mcp_config` 的 `creator` 本人
或管理员（E23 / E24）。`portal_core_entity` 无 deadpool `Pool` 注入，沿用 sea-orm
`require_admin`，查询失败 fail-closed。

1. **行是否真在 main**：已在 `origin/main` 上核对。merge sha `1df0963a1` 被 `origin/main` 包含
   （`git branch -r --contains 1df0963a1` 命中 `origin/main`）；在该提交的
   `security_regression_baseline.txt` 里 grep 本批 6 条 id，全部无命中；`x_ai_mcp_config.creator`
   读门代码仍在。即 6 条是真实修复、不是基线糊过。
2. **基线是否虚减**：在 `8c7c2f4b5` 上重跑门禁，本批 6 条全部 `[PASS]`；合并提交
   （`1df0963a1`）的基线全文 grep 本批 id 无命中，未靠基线糊过。
3. **门禁是否接线**：`sabotage read-a-b5` 变红（exit=101），`NEW-FAIL` 逐条点名 6 条，
   `实验成立：门禁变红（exit=101），并点名 6 条用例`；三个 `lib.rs` 各自动还原一次。

若 #262（A-b4）先合入，本批与它共用 `portal_assemble_surface` 的同一 helper 文本，
恒真化会同时波及 `get_surface`（额外 1 条 `[LEAK]`），属预期「同 helper 波及」，不影响
本批 6 条结论。

## 十一、公开读面 B6 交付与 §12.6 核对记录

分支 `sec/read-public-b6`（base `origin/main`）。范围：§四 登记的 bbs 19 + `user_*` 4 + hotpic 7。
新增 `public-read-login` kind：同一用例先匿名请求（要求 401/403），再以非管理员会话请求
（要求 2xx + 正控制命中 + 无 PII canary）；28 条可达读面全部 `[PASS]`，2 条无路由 handler 登记为不可达。

1. **行是否真在 main**：本分支尚未合并，待裁判合并后按 merge sha 核对；本分支已 rebase 到
   `origin/main`（`11c551911454`）并在其上跑通。
2. **基线是否虚减**：rebase 后 `python scripts/security_regression_gate.py run` 全绿，非 PASS 与基线一致；
   本批 28 条全部 PASS，未新增基线条目（新增项不得进基线）。
3. **门禁是否接线**：`sabotage public-read-b6` 在统一成功信封注入 PII canary 后变红（exit=101），
   输出逐条点名 28 条 `[LEAK]`，`实验成立：门禁变红（exit=101），并点名 28 条用例`；两个 patch 各自动还原一次。

未决项：无 `UNCLEAR`。唯一需要产品侧后续确认的是 `u2_voterecord_list` 的
「谁投了哪项」是否应视为敏感（当前只返回内部身份键 `person`，不含姓名/手机/邮箱，按代码现状判公开）。

## 变更纪律（本分片）

- 归属谓词一律写进 SQL `WHERE`，禁止「先查后判」。
- 每条修复附双向用例（A→B、B→A 均拒绝）+ 本人 200 且返回落库内容的阳性对照。
- 每条修复附破坏实验：摘掉 / 恒真谓词后用例必红，附红原文，然后还原。
- 非 PASS 只允许单调下降；能减基线就点名减；不新增基线条目。
