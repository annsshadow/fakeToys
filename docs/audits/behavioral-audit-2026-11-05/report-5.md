# 第五轮行为级缺陷审计（只读）——把「不可判」转成可判

> 对象：Rust `oa4rust/backend`（Go `server/` 本轮未新增测试）
> 边界：未修改任何业务代码、迁移、配置；无 `git add/commit/push/reset`；动态测试只打本机 `127.0.0.1:3999`，只用自建 scratch 库 `oa_audit_20261105`。唯一写入为本审计目录下的报告、证据与造数 SQL。
> 身份：攻击者 `audit-user`（普通会话）；受害者 `admin`（auth `unique_id=admin`，person `person-admin`）。
> 实测原文：同目录 `evidence-round5.txt`；造数脚本：`seed-round5-empty.sql`、`seed-round5-pp.sql`（可重复执行）。

---

## 〇、基线（结论时效）

| 仓库 | 读代码/跑测试时 HEAD | 工作树 |
|---|---|---|
| Rust + Go（同仓库） | **`4b4e17010`** | 仅 `?? docs/audits/behavioral-audit-2026-11-05/`（本轮审计目录，未跟踪） |

**相对第四轮基线的变化**：第四轮基线为 `40b215ef4`。区间内与后端相关的提交只有 PR #184（`1c639766b` / `7081d7980`）给考勤 `filter/list` 系列加会话人过滤，以及 `038bf695c` 的 401 测试；**本轮点名的 `attendancedetail_mobile_mobilepreview` / `mobile_my` / `mobile_recive` / `attendanceappealInfo_appeal_id` 不在该改动内**，其余目标（bbs / mind / processplatform / invoice / file multipart）也无归属逻辑改动。因此下文结论按 `4b4e17010` 源码复核，均仍成立。

> 运行中的服务进程可能早于 HEAD 构建；本轮对每个结论都在 HEAD 源码上复核过行号，避免「测的是旧二进制、说的是新代码」。

---

## 一、结论摘要

| 任务 | 口径 | 本轮结论 |
|---|---|---|
| 空表类 | `bbs_subject_info` / `x_mind_base_info` / `x_mind_folder_info` 造数 | **归属谓词不存在**：登录用户可读、可改、可删他人行；`author_id` / `creator` 可取请求体冒名。**全部转为「已实证」** |
| 审批链 | `x_work` / `x_task` / `pp_c_*` | **非空 creator 的越权被 403 挡住（修复线有效）**；但 **creator 为空时任何登录用户都能 processing / terminate**；**无状态机**：completed/terminated 可回退 processing，pending 可直接 terminate；body 里的 `status` 被忽略 |
| multipart | `/api/file/upload`、u2 `upload/referencetype/...` | **`/api/file/upload` 的 `person` 取表单（可冒名）**；u2 路径 `person` 取会话（form 伪造无效）；**`/api/file/anonymous/file/{id}/download` 无归属门，登录用户可读他人文件内容** |
| 125 条分级 | `write_fields.json` 125 路由 / 95 fn | **一级 16 / 二级 20 / 三级 9 / 已被门禁覆盖 80**；归属 80 中**真断链 3 条（calendar 双表 + meeting 500）+ 误报 2 条**，其余后果链完整 |
| 游标 85 | 抽样审批链两个端点 | `{id}` 当 **xid 精确查询**、`count` **无效**、**不取 session / 不按 creator 过滤** → 登录用户可读他人 work/task |

**本轮最严重的一笔**：`u4_require_actor` 对空 actor **直接放行**，导致所有 `creator` 为空的 `x_work` 可被任意登录用户 `processing` 和 `terminate`（实测 200，且写入 `x_record` 审计痕迹只记录攻击者自己）。

---

## 二、空表类 → 已判

造数脚本 [seed-round5-empty.sql](seed-round5-empty.sql)（先删后插，可复跑）。实测原文见 `evidence-round5.txt §A`。

### 2.1 `bbs_subject_info`

| 动作 | 匿名 | audit-user 对 admin 的行 | 结论 |
|---|---|---|---|
| 读列表 | 401 | 200，列表含 admin 帖 | 登录用户可读他人帖 |
| 改标题 | 401 | **200**；库 `title=HACKED-BY-AUDIT-USER`、`is_top=t` | 可改他人帖 |
| 删 | 401 | **200**；行消失 | 可删他人帖 |
| 冒名建帖 | — | `authorId:"admin"` → 库 `author_id=admin`，`creator` 为空 | 请求体可冒名 |

- 代码：`bbs_core_entity/src/lib.rs:560 update_subject`、`:643 delete_subject`，均 `require_session` 后按 `Path(id)` 直改/直删，**无 owner 判断**；`:508 create_subject` 的 `author_id` 取自请求体。
- 附带写后必读缺口：`update_subject` 响应回显 `isTop=false`，库已是 `is_top=t`——**响应用旧值，写后必读不一致**。

### 2.2 `x_mind_base_info` / `x_mind_folder_info`

| 动作 | 匿名 | audit-user 对 admin 的行 | 结论 |
|---|---|---|---|
| 读列表 | 401 | 200，含 admin 的脑图与描述 | 登录用户可读他人脑图 |
| 改 mind | 401 | **200**；库 `name=HACKED-MIND` | 可改他人脑图 |
| 改 folder | 401 | **200**；库 `name=HACKED-FOLDER` | 可改他人文件夹 |
| 删 mind | — | **200**；行消失 | 可删他人脑图 |
| 冒名建 mind | — | `creator:"admin"` → 库 `creator=admin` | 请求体可冒名（**creator 真落库**） |

- 代码：`mind_core_entity/src/lib.rs:182/233/284/312/369/431`，全部 `require_session` + 按 id 直操作，无 owner 判断；实体表名分别为 `x_mind_base_info` / `x_mind_folder_info`（`src/entities/mind_mind.rs`）。
- 结论：**「空表不可判」的本体是「没有归属谓词」，不是「数据为空」**；造数后 6 个动作里 5 个直接越权。

---

## 三、processplatform 前置审批链

实测原文见 `evidence-round5.txt §B`。造数 [seed-round5-pp.sql](seed-round5-pp.sql)。

### 3.1 越权门禁：非空 creator 有效

| 端点 | 匿名 | audit-user → admin 的 work/task | admin 自己 |
|---|---|---|---|
| `PUT .../work/{id}/processing` | 401 | **403** | 200 |
| `POST .../task/processing/{id}` | 401 | **403** | 200 |
| `POST /api/work/{id}/terminate` | — | **403** | 200 |
| `GET .../work/v2/{id}/terminate` | — | **403** | 200 |

修复线的 `u4_require_actor`（`processplatform_service_processing/src/lib.rs:422`）与 `u4_require_actors`（Task，`:1133` 调用）对非空 creator 生效，这一半是好的。

### 3.2 空 actor 直接放行（**高危断链**）

`u4_require_actor` 的逻辑是「actor 为空 → Ok」。把 `x_work.creator` 置 NULL 后：

```
audituser_work_processing(NULL creator) -> 200
audituser_terminate(NULL creator)       -> 200   # work_status=terminated，end_time 已写
```

对比 task：`u4_require_actors` 对空归属走「仅 admin」，audit-user 被 403。**同一审批链上 work 与 task 的空归属语义相反**——这是修复线自身的口径不一致，不是业务遗留。

### 3.3 状态机：不存在

| 操作 | 结果 | 库 |
|---|---|---|
| `completed` → `processing` | 200 | `processing` |
| `terminated` → `processing` | 200 | `processing`（**终态复活**） |
| task `completed` → `processing` | 200 | `processing` |
| `pending` 直接 `terminate` | 200 | `terminated`（**跳过 processing/审批**） |
| body 传 `{"status":"completed","workStatus":"completed"}` | 200 | `work_status=processing`，legacy `status` 列仍空 |

结论：**没有「必须走完审批才能进入下一状态」这条链**；`work_id_processing`（`:534`）与 `work_v2_id_terminate`（`:566`）都只做归属校验，不做前置状态校验。`status` 字段不能从 body 直改（请求体被忽略），但通过动作端点可以无顺序地改 `work_status`。

### 3.4 游标 85 个端点（审批链抽样）

| 端点 | 匿名 | audit-user | 说明 |
|---|---|---|---|
| `GET .../work/list/next/creator/current/pp-work-a1/10` | 401 | **200**，返回 `pp-work-a1`（creator=`person-admin`） | **不取 session、不按 creator 过滤** |
| `GET .../task/list/next/pp-task-a1/10` | 401 | **200**，返回 admin 的任务 | 同上 |
| `.../{id}/1` 与 `.../{id}/100` | — | 两次都 `count=1` | `count` 被忽略，无真分页 |
| `.../__absent__/10` | — | 200 `[]` | `{id}` 被当作 **xid 精确查询** |

代码：`processplatform_assemble_surface/src/lib.rs:21212`（`work_list_id_next_count_creator_current`）SQL 为 `SELECT ... FROM PP_C_WORK WHERE xid = $1`，handler 无 session extractor；`:16555` task 同族。**「我的创建」没有按人过滤，游标不是游标**。

---

## 四、multipart 上传

实测原文见 `evidence-round5.txt §D`。

### 4.1 归属字段来源

| 端点 | `creator`/`person`/`owner` 来源 | 实测 |
|---|---|---|
| `POST /api/file/upload`（file crate） | **表单字段 `person`** | audit-user 传 `person=admin` → 库 `person=admin`；随后 audit-user 删 403、admin 删 200 |
| `PUT /api/file/assemble/control/file/upload/referencetype/.../scale/...`（u2） | **会话** | audit-user 传 `person/creator/owner=admin` → 库 `person=audit-user`，伪造无效 |
| `POST /api/file/core/entity/file/create` | 会话（`creator=system` 硬编码） | 第四轮已证 |

### 4.2 读/删后果

- **读**：`GET /api/file/anonymous/file/{id}/download` 的 handler **无 session、无归属判断**（`file_assemble_control/src/lib.rs:776`）。实测 audit-user 读 admin 的文件 → **200 且返回内容**；而同文件走三个有门端点（`/api/file/download/{id}`、`/api/file/assemble/control/file/{id}/download`、`/api/file/{id}/download`）全部 **403**。
- **删**：`DELETE /api/file/assemble/control/file/{id}`（u2 `u2_file_delete_by_id`）有归属门，audit-user **403**、admin **200**；`/api/file/assemble/control/file/delete/{id}` 查的是 `x_file`（与 `FILE_FILE` 不是一张表），对 `FILE_FILE` 行返回「file not found」——**又一处双表断链**。
- **附带 multipart 解析缺陷**：u2 路径把第一个文本 part 当成文件内容，`name` 固定 `upload.bin`（实测 length=10，内容为 `audit-user` / `admin` 表单值），文件名未保留。归属问题之外，解析映射也需要修。

---

## 五、125 条「请求体可改敏感字段」按后果分级

口径沿用第四轮 `write_fields.json`：125 路由 / 95 唯一 handler；**归属 80 / 状态 66 / 金额 2（类别重叠）**。本轮按「能否造成他人数据被改/被删/被读」分三级，并把「修复线已加门禁」单列。

### 5.1 分级定义

- **一级**：已实证（或高置信静态）可造成**他人既有数据被改/删/读**，或冒名写入已进入受害者读面。
- **二级**：可**冒名新建/写入他人名下**，但未实证到受害者读面命中；后果链在代码上成立。
- **三级**：**断链 / 无法利用 / 误报**（写表≠读表、端点 500、字段其实不来自请求体、已有门禁且写读同表）。
- **已加固**：静态检测到 owner/admin/actor 门禁调用（修复线覆盖）。

### 5.2 计数

| 分级 | 路由 | 唯一 handler | 其中未加固 |
|---|---:|---:|---:|
| 一级 | **16** | 13 | 16 |
| 二级 | **20** | 16 | 19（1 条 `create_version` 已有门） |
| 三级 | **9** | 7 | 8（1 条 `calendar::event_update` 本就有正确门禁） |
| 已加固 | **80** | 59 | 0 |
| 合计 | **125** | 95 | 43 |

- **归属 80 子集**：一级 12 / 二级 13 / 三级 6 / 已加固 49。
- 结论不是「125 条都可利用」，而是：**43 条未加门禁**，其中 16 条已实证后果、19 条后果链完整但未逐条打读面、8 条断链或误报。

### 5.3 一级（代表性实证）

| # | 端点 | 后果 | 实测 |
|---|---|---|---|
| 1 | `POST .../attendancedetail/mobile/mobilepreview` | **读他人考勤明细** | audit-user 传 `personId=person-admin` → 200 返回 `r5-ad-admin|LATE` |
| 2 | `POST .../attendancedetail/mobile/my` | **读他人考勤明细** | 同上，返回 admin 的明细数组 |
| 3 | `POST .../attendancedetail/mobile/recive` | **改他人已读标记** | 200，库 `received=f→t` |
| 4 | `POST .../attendanceappealInfo/appeal/{id}` | **改他人申诉状态** | 200，`appeal-admin` 的 `appeal_status=pending→approved` |
| 5 | `POST /api/general/invoice/update/{id}` | **改他人发票金额/状态** | 200，`INV-ADMIN` 的 `amount=1000→9999.5`、`status=approved` |
| 6 | `POST /api/general/invoice/delete/{id}` | **删他人发票** | 200，行消失（该路由不在 125 的「body 敏感字段」口径内，属同族） |
| 7 | `POST .../bbs/core/entity/subject` | **冒名建帖** | `author_id=admin` 落库（受害者列表可见） |
| 8 | `POST .../mind/core/entity/mind` | **冒名建脑图** | `creator=admin` 落库（列表全局可读） |
| 9 | `attendance_core_entity record_create/update` | **冒名建/改他人考勤** | 第四轮已证（`user_id` 取自 body、按 id 改任何人） |
| 10 | `bbs create_topic/create_reply` | **冒名发帖/回帖** | 第四轮已证（受害者「我的帖子」可见并可删） |
| 11 | `oauth_generate_code` | **为任意 person 签发授权码** | 静态：`organization_assemble_authentication/src/lib.rs:1258`，`personId` 取 body，只有 session 门；下游若信任该 `personId` 即身份冒用（**待下一轮验证下游交换**） |

### 5.4 二级（代表性）

- `general_core_entity/src/lib.rs:720 invoice_create`：`creator`、`amount`、`status` 全取自 body → 冒名新建（未造数）。
- `file_core_entity/src/lib.rs:313/374 folder_create/file_create`、`general_core_entity/src/lib.rs:503 file_create`：`person` 取自 body。
- `query_core_entity/src/lib.rs:186 view_create`：`creator_id`、`status`、`query_sql` 取自 body（同目录 `query_core_express` 的 `execute_query` 已 admin-only，故本轮不升级）。
- `organization_core_entity/src/lib.rs:557 identity_create`（`person_id`）、`:864 bind_create`（`role`）、`:461 group_create`（`level`）。
- `organization_assemble_express/src/lib.rs:246/293/424`：`person_id` / `level_name` 作为过滤条件读组织数据（**读面待判**）。
- `attendance_assemble_control/src/lib.rs:8215 v2_groupschedule_list_filter`：`person_id` 过滤读他人工时安排。
- `mind` / `mind_core_entity` 的 `create_version`：`creator` / `creator_unit` 取自 body（`create_version` 在 mind crate 已有门禁，core_entity 版本未加）。

### 5.5 三级：真断链与误报（用户点名的那类）

| 项 | 证据 | 判定 |
|---|---|---|
| `calendar_core_entity calendar_create` / `event_create` | 写 `x_cal_calendar` / `x_cal_event`；用户面 `calendar` crate 读写 `cal_calendar` / `CAL_EVENT`(=`cal_event`)。库内两套表并存：`x_cal_calendar=2` / `cal_calendar=0`，`x_cal_event=4` / `cal_event=3` | **真断链**：冒名写进用户面永远不读的表 |
| `general_assemble_control invoice` 家族 vs `general_core_entity invoice` 家族 | assemble 写 `x_general_assemble_invoice` 且有 `owner_gate`（`:55`）；core 写 `x_general_invoice` 且无门禁 | **半修 + 双表**：加固镜像保护的不是同一张表；实测 assemble 更新 core 行 → `invoice not found` |
| `general file` 家族 | `x_general_file` vs `x_general_assemble_general_file` | 双表断链 |
| `meeting create_meeting` | 第四轮实测 500（字符串绑定失败） | 不可利用 |
| `auth login` / `auth code` | OWN 字段来自 DB 行 / 会话，不是请求体（`auth/src/lib.rs:200/754`） | **误报** |
| `auth sms_verify_handler` | `valid` 来自本地 code store 校验，不是 body | **误报** |
| `calendar::event_update`（crate `calendar`） | 有 `require_owner`（`calendar/src/lib.rs:517`），读写都是 `CAL_EVENT` | 已加固，非断链 |

### 5.6 分母警示（自我修正）

125 的口径是「**请求体**里出现敏感字段」。本轮最狠的删除/改状态端点——`attendance record_delete`、`bbs delete_subject`、`mind delete_mind/delete_folder`、`invoice_delete`——**没有敏感 body 字段**，因此**不在 125 内**。所以 125 是「请求体面」的风险集，不是全部越权面；修复排序不能只看这 125。

---

## 六、修复顺序建议（按本轮后果）

1. **`u4_require_actor` 空 actor 放行**（`processplatform_service_processing/src/lib.rs:422`）：空 creator 不应等价「人人可写」，至少应 fail-closed 到 admin；work 与 task 口径统一。**（最高，实测可终止任意空 creator 的 work）**
2. **审批链状态机**：禁止 `completed/terminated → processing` 回退、禁止跳过前置状态直达 `terminated`；终态复活会破坏审批留痕。
3. **考勤明细 3 端点 + appeal**（`:1688/1743/1799/443`）：`personId` 改为会话事实源或加 `require_owner`；PR #184 已修 filter/list 系列，但漏了这一组。
4. **`general_core_entity` invoice 家族**（`:462/784/823/893`）：加 owner/admin 门禁，并与 assemble 家族统一到一张表；否则「加固镜像」形同虚设。
5. **bbs / mind 的 create 归属取会话**：`author_id` / `creator` 不再信任 body；update/delete 补 `require_owner`。
6. **file**：`/api/file/upload` 的 `person` 取会话；`/api/file/anonymous/file/{id}/download` 加归属/公开标记判断；u2 multipart 修正 part/filename 映射。
7. **游标 85 端点**：加 session + creator/person 过滤，`{id}` 语义与 `count` 真正实现分页（当前是主键精确查询）。
8. **双表收敛**：calendar（x_cal_*/cal_*）、general invoice/file、file（FILE_FILE/x_file）三组「写一张读另一张」需要定权威表，否则所有归属修复都可能修在无人读的表上。

---

## 七、仍需下一轮（本轮未闭环）

- `oauth_generate_code` → `oauth_token_get` 的下游信任链（`access_token` 是否被当成身份使用）。
- 二级里 `organization_assemble_express` / `v2_groupschedule_list_filter` 的读面是否真的返回他人数据（需造组织/排班数据）。
- 125 之外的 `delete_*` / `update_*` path-IDOR 全量（本轮只抽样了 bbs/mind/attendance/invoice）。
- Go 侧本轮无新增；第四轮两条写后必读已在 `961bda135` 修复。

---

## 八、证据与复跑

| 文件 | 内容 |
|---|---|
| `evidence-round5.txt` | §A 空表实测、§B 审批链、§C 游标、§D multipart、§E invoice/appeal、§F 断链、§G 考勤明细跨用户、§H 125 分级统计 |
| `seed-round5-empty.sql` | bbs / mind 最小造数（先删后插，可重复执行） |
| `seed-round5-pp.sql` | x_work / x_task 审批链造数（可重复执行） |
| 临时脚本（不入库） | `%TEMP%\audit4\t_r5_all.py`、`t_r5_att.py`、`t_r5_att2.py`、`t_r5_u2.py`、`r5_tier3.py`、`r5_counts.py` |
