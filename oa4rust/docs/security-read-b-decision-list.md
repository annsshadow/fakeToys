# 读 IDOR 分片 B 裁决清单（批次 2 / 批次 3 / 批次 4）

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

## 批次 3：org person 明细/分页（PII 读面）

判定标准不是“有没有 creator”，而是**谁能看到别人的姓名/手机号/邮箱**。本批统一落到
`x_org_person.creator`：`creator = 会话 person OR 调用者为 admin`；`creator IS NULL`（含历史行）
不命中，fail-closed 仅管理员可读。谓词全部写进 SQL `WHERE`，不做先查后判。

| 端点（organization_assemble_control） | 表 | 落库谓词 | 说明 |
|---|---|---|---|
| `GET /person/{flag}`（`person_get`） | `x_org_person` | `id = $1 AND (id = $2 OR creator = $2 OR admin)` | 单资源：额外允许 `id = 本人`（用户读自己档案）；命中 0 行时若行存在返回 403，不存在维持原 not found 信封 |
| `GET /person/list/{flag}/next/{count}`（`person_list_next`） | `x_org_person` | `deleted_at IS NULL AND (creator = $actor OR admin)` | 首屏游标 `0`/`(0)` 也带谓词；非首屏 `$actor` 为 `$3` |
| `GET /person/list/{flag}/prev/{count}`（`person_list_prev`） | `x_org_person` | 同上 | 反向取窗口后仍 `reverse()` 还原显示顺序 |
| `GET /person/list/role/{roleFlag}`（`person_list_with_role`） | `x_org_person` + `auth_person_role` + `x_org_role` | `p.creator = $actor OR admin` | 角色关联边（`auth_person_role`）不改契约；只收紧 person 行 owner 一侧 |
| `POST /person/list/filter/{page}/size/{size}`（`person_list_filter_paging`） | `x_org_person` | `creator = $actor OR admin` | 同一谓词同时用于 `COUNT(*)` 与 `data`，否则总数会泄漏他人行存在性 |
| `GET /person/list/group/{groupFlag}/sub/direct`、`.../sub/nested`（`person_list_group_sub_direct/nested`） | `x_org_person` + `x_org_group_member` | `p.creator = $actor OR admin` | 群组成员是**关联边**：本批只收紧 owner 一侧，成员边侧不改契约，见下方裁决 |
| `PUT /person/list/like`（含 `/mockputtopost`） | `x_org_person` | `creator = $actor OR admin` | 空 key 分支也不得全表返回 |
| `PUT /person/list/like/pinyin`（含 `/mockputtopost`） | `x_org_person` | 同上 | 同上 |
| `PUT /person/list/pinyininitial`（含 `/mockputtopost`） | `x_org_person` | 同上 | 空 initials 分支与过滤分支均带谓词 |

新增门禁用例覆盖上述每条拒绝路径（`dir=both`：攻击者读受害者、受害者读攻击者镜像）与
`idor-read-own` 正控制。破坏实验：把 `owner_scope` 改成恒真后，10 条拒绝用例全部
fwd/rev 双向变 LEAK（见提交说明红原文），还原后门禁回到 152 非 PASS（与基线一致）。

### 关联边裁决（未改契约）
- `x_org_group_member(group_id, person_id, role_id)` 是群组→人员关联边，`x_org_group` 本身没有
  “通讯录公开性”列。本批只允许“本人或管理员”看 person 行；**群组成员是否对全员可见**没有
  落库依据，未按“全员可见”放行，也未发明新列。若业务要求按群组公共通讯录展示，需先裁决
  再补归属/公开性模型。
- `person_list_with_role` 的 `auth_person_role` 同理：角色绑定不作为读权限来源，仅作为筛选条件。

### 仍未覆盖（同判据，留待下一批）
- 人员属性/自定义字段列表（`x_org_person_extend` 相关）：按 person 关联边读取，待同批判据。

## 批次 4：personcard 读端点 + 已删人员分页（PII 读面）

判据沿用批次 3：**谁能看到别人的姓名/手机号/邮箱**。`personcard_*` 这批端点名义上是“个人名片”，
实际 SQL 全部命中 `x_org_person`（全量人员 PII），因此与本批次 3 同口径落到 `x_org_person.creator`。

| 端点（organization_assemble_control） | 表 | 落库谓词 | 说明 |
|---|---|---|---|
| `GET /personcard/mylist`（`personcard_mylist`） | `x_org_person` | `deleted_at IS NULL AND (creator = $1 OR admin)` | 名为 mylist，原实现返回全表；本批收紧为本人 OR 管理员 |
| `GET /personcard/listgrouptypes`（`personcard_listgrouptypes`） | `x_org_person` | 同上 | 无分页，同一谓词 |
| `GET /personcard/listpaging/page/{page}/size/{size}`（含 `/mockputtopost`） | `x_org_person` | `deleted_at IS NULL AND (creator = $3 OR admin) ... LIMIT $1 OFFSET $2` | 分页参数不变；actor 为 `$3` |
| `GET /personcard/listpagingwithgroup/page/{page}/size/{size}`（含 `/mockputtopost`） | `x_org_person` | 同上 | “withgroup”不额外放宽群组可见性，群组属关联边，见批次 3 裁决 |
| `GET /personcard/listVCf/{idList}`（`personcard_listVCf_idList`） | `x_org_person` | `id = ANY($1) AND deleted_at IS NULL AND (id = $2 OR creator = $2 OR admin)` | 批量按 id 取，额外允许 `id = 本人` |
| `GET /personcard/listPersonalVCf/{idList}`（`personcard_listPersonalVCf_idList`） | `x_org_person` | 同上 | 同上 |
| `GET /personcard/{flag}`（`personcard_flag`） | `x_org_person` | `id = $1 AND deleted_at IS NULL AND (id = $2 OR creator = $2 OR admin)` | 单资源；命中 0 行但行存在时返回 403，不存在维持 not found 信封 |
| `POST /person/list/delete/{page}/{size}`（`person_list_delete_paging`） | `x_org_person` | 不做行过滤，改**仅管理员**门禁（无会话 401 / 非 admin 403） | 已软删行仍是 PII；历史行归属不可靠且“本人能否看自己被删行”无业务依据，按 fail-closed 收为管理员专属 |

新增门禁用例 20 条（9 个 personcard 读端点 × 双向 + 本人正控制，外加拿 1 条已删人员管理员门禁），
全部 PASS；夹具补 `gate-person-del`（`GATE-DEL-MOBILE`，`deleted_at` 非空）供 delete paging 取证。
破坏实验：把 `owner_scope` 临时改为恒真（保留占位符绑定）并摘掉 delete paging 的 admin 闸后，
上述拒绝用例变红（`NEW-FAIL` 20 条，personcard 各端点 `LEAK(2xx 响应体含 canary GATE-V-MOBILE /
rev-GATE-V-MOBILE)`，delete paging `LEAK(2xx 响应体含 canary GATE-DEL-MOBILE)`）；还原后门禁回到
147 非 PASS（与基线一致，无 `STALE-BASELINE`）。

### 批次 4 备注（不硬改的部分）
- `personcard_*` 响应体仍按原契约输出 `creator` 字段；本批只收紧“能读到哪些行”，不删字段、不改信封。
- 已删人员分页的“本人可见自己被删行”未采纳：无公开性依据，且软删通常意味着记录不应再由本人读取。
- `x_org_personcard` 表虽有 `creator` 列，但当前读路径并未使用该表；本批不引入该表归属语义。


## 批次 B-b1：attendance 六张配置表单资源读（creator/creator_person OR admin）

判据沿用默认1 / E19：`本人 OR 管理员`，NULL 归属 fail-closed（仅管理员）。
六张表均同时有 `creator` 与 `creator_person`，**两列都认**：

| 端点（attendance_assemble_control） | 表 | 落库谓词 |
|---|---|---|
| `GET /attendanceadmin/{id}` | `x_attendance_admin` | `id = $1 AND (creator = $2 OR admin)` |
| `GET /attendanceemployeeconfig/{id}` | `x_attendance_employee_config` | `id = $1 AND (creator = $2 OR creator_person = $2 OR admin)` |
| `GET /attendanceimportfileinfo/{id}` | `x_attendance_import_file_info` | 同上 |
| `GET /attendanceschedulesetting/{id}` | `x_attendance_schedule_setting` | 同上 |
| `GET /attendancesetting/{id}` | `x_attendance_setting` | 同上 |
| `GET /attendancestatisticalcycle/{id}` | `x_attendance_statistical_cycle` | 同上 |

- 命中 0 行但行存在 → `403`；行不存在维持原信封（admin 为 `success(null)`，其余为 error not found）。
- 为什么同时认两列：除 `attendanceadmin_create` 外，另 5 个 create 只写 `creator_person`、不写
  `creator`。只按 `creator` 收紧会让「本人刚创建的行」立刻变成管理员专属，属读写不一致；
  写侧缺陷（`creator` 不落库）已登记，按「先修写入 → 新数据有归属 → 历史行仅管理员」顺序处理。
- 门禁：新增 6 条 `idor-read-own` 正控制；fwd/rev 复用既有 `r6-*` / `rev-r6-*` 镜像行，未新增种子。
- 破坏实验：`owner_scope` 临时改恒真（保留 `$2` 绑定，避免 deadpool 500 假阳性），
  `NEW-FAIL` 恰好点名本批 6 条；还原后门禁 122 非 PASS，与基线一致（原 128 → 点名减 6），无 `STALE-BASELINE`。

## 批次 B-b4：processplatform_service_processing 任务/流程/办结读（E20 指派人 OR 管理员）

复用 crate 内既有 E20 授权原语，不新增归属语义：

| 端点（processplatform_service_processing） | 表 | 落库判据 |
|---|---|---|
| `GET /task/{id}`、`GET /task/v2/{id}` | `x_task` | `u4_require_actors(U4ActorKey::Task)`：`person` 指派人或 admin |
| `GET /get/{id}`（get_process） | `x_work` | `u4_require_work_owner`：`creator` 或 admin |
| `GET /taskcompleted/{id}` | `x_workcompleted` | `U4ActorKey::WorkCompleted`：`creator` 或所属 `x_task.person` 或 admin |
| `GET /data/workcompleted/{id}`、`.../path/{id}` | `x_workcompleted` | 同上 |

- 这些原语本是写路径的 fail-closed 归属门，读端点直接复用；NULL 归属不命中 → 仅管理员。
- 无会话 → 401；非归属人 → 403。
- 门禁：6 条新增 `idor-read-own` 正控制；复用 `rev-task-admin-1` / `rev-work-admin-1` / `rev-r6-dataworkcompletedid`。
- 破坏实验：逐条把 6 个授权调用临时短路（`if false`）→ `NEW-FAIL` 恰好点名本批 6 条；
  还原后门禁 105 非 PASS（基线 111 → 点名减 6），无 `STALE-BASELINE`。

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
