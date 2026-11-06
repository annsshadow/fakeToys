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

## 批次 B-b2：attendance 统计/工作日/场所/打卡/申诉读（本人 OR 管理员）

| 端点（attendance_assemble_control） | 表 | 落库谓词 |
|---|---|---|
| `GET /attendancestatisticrequirelog/{id}` | `x_attendance_statistic_require_log` | `id = $1 AND (creator = $2 OR creator_person = $2 OR admin)` |
| `GET /attendanceworkdayconfig/{id}` | `x_attendance_workday_config` | 同上 |
| `GET /workplace/{id}` | `x_attendance_workplace` | 同上 |
| `GET /v2/record/{id}` | `x_attendance_record` | `id = $1 AND (user_id = $2 OR admin)` |
| `GET /v2/appeal/{id}` | `x_attendance_v2_appeal_info` | `id = $1 AND (user_id = $2 OR admin)` |
| `GET /attendancedetail/mobile/my` | `x_attendance_detail` | 会话派生 `person_id = session.person_unique` |

- `mobile/my`：请求体 `personId` 只用于识别本人，指向他人一律 `403`（不再冒名读）；缺省则按会话人查。
  这一条同时消掉了 `spoof-read` 的读面（原 `LEAK` → `PASS`）。
- 命中 0 行但行存在 → `403`；行不存在维持原 not-found 信封。
- 门禁：5 条新增 `idor-read-own` 正控制；复用既有 `r6-*` / `rev-r6-*` 镜像行，未新增种子。
- 破坏实验：逐条把 5 个谓词改恒真（保留 `$2` 绑定）并关闭 `personId` 校验 →
  `NEW-FAIL` 恰好点名本批 6 条；还原后门禁 116 非 PASS（基线 122 → 点名减 6），无 `STALE-BASELINE`。

## 批次 B-b3：attendance 申诉/明细/自休假读（person_id 本人 OR 管理员）

| 端点（attendance_assemble_control） | 表 | 落库谓词 |
|---|---|---|
| `GET /attendanceappealInfo/{id}` | `x_attendance_appeal_info` | `id = $1 AND (person_id = $2 OR admin)` |
| `GET /attendancedetail/{id}` | `x_attendance_detail` | 同上 |
| `GET /attendancedetail/mobile/{id}` | `x_attendance_detail` | 同上 |
| `GET /attendanceselfholiday/{id}` | `x_attendance_selfholiday` | 同上 |

- 口径 §9.2 B3–B8：归属人是 `person_id`，历史 NULL 行 fail-closed（仅管理员可读）。
- 命中 0 行但行存在 → `403`；行不存在维持原信封（appeal / detail 为 `success(null)`，自休假为 not found）。
- 门禁：4 条新增 `idor-read-own` 正控制；复用既有 `rev-*` 镜像行，未新增种子。
- 破坏实验：逐条把 4 个谓词改恒真（保留 `$2` 绑定）→ `NEW-FAIL` 恰好点名本批 4 条；
  还原后门禁 112 非 PASS（基线 116 → 点名减 4），无 `STALE-BASELINE`。

## 批次 B-b4：processplatform_service_processing 任务/流程/办结读（E20 指派人 OR 管理员）

复用 crate 内既有 E20 授权原语，不新增归属语义：

| 端点（processplatform_service_processing） | 表 | 落库判据 |
|---|---|---|
| `GET /task/{id}`、`GET /task/v2/{id}` | `x_task` | `u4_require_actors(U4ActorKey::Task)`：`person` 指派人或 admin |
| `GET /get/{id}`（get_process） | `x_work` | `u4_require_work_owner`：`creator` 或 admin |
| `GET /taskcompleted/{id}` | `x_work_completed` | `U4ActorKey::WorkCompleted`：`creator` 或所属 `x_task.person` 或 admin |
| `GET /data/workcompleted/{id}`、`.../path/{id}` | `x_work_completed` | 同上 |

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

## §14 观测缺口补证据：attendance 三条 UNVERIFIED（B-b14）

三条均在 `attendance_assemble_control`，原判定因“请求键命中 0 行”而恒为 `UNVERIFIED`；
补种子后暴露出同一根因：**handler 直接把请求体/查询里的 `personId`/`userId` 当权限主体**。
因此本轮按“先补证据、再收归属”处理，三条转 `PASS` 后点名删基线。

| 端点 | 原问题 | 收口 |
|---|---|---|
| `POST /attendancedetail/mobile/mobilepreview` | body `personId` 直查他人明细；种子行缺 `date` → `data:null` | 先 `require_session`；`personId` 空→会话本人；非本人走 `require_owner`（admin 旁路） |
| `POST /attendancedetail/mobile/recive` | 已有会话门，但写目标取 body `personId`，可替他人标记已接收 | 写主体=会话本人；空→会话本人；非本人 `require_owner` 拒绝 |
| `POST /v2/groupschedule/list/filter` | 按 body/query `userId` 读他人排班；无种子行 → `data:[]` | 先 `require_session`；`userId` 空→会话本人；非本人 `require_owner` |

口径沿用默认 1 / E19：本人 OR 管理员；空值回落会话本人，拿不到会话一律 401。

种子与产物：
- `security_regression_seed.sql`：`r5-ad-admin` 补 `date='2026-01-01'`（命中默认 spoof body 的日期）；
  新增排班受害者行 `SCHED-VICTIM`（`user_id=gate-victim`、`schedule_month_string=2026-01`）。
- `security_regression_rev_seed.sql`：新增排班攻击者镜像行 `rev-sched-admin`（`user_id=audit-user`）。
  `rev-r5-ad-admin` 仍保持 `date` 为空：给它补日期会让攻击者明细分页多出一行，
  破坏 `page-attendance-detail-*` 的“仅本人 6 行”正控制；fwd 证据已足够定位泄漏。
- `security_regression_cases.tsv`：`v2_groupschedule_list_filter` 的 leak 标记改为
  `SCHED-VICTIM`（避免 `rev-` 前缀裸子串误报）。

验证：
- 收口后三条实测 `[PASS]`，基线点名删 3 行（72 → 69）。
- 破坏实验：三处 `require_owner` 临时短路为恒假条件（保留参数绑定），
  `NEW-FAIL` 恰好点名这三条（mobilepreview `LEAK`、recive `UNVERIFIED`、filter `LEAK`），
  其余用例无变化；还原后门禁 69 与基线一致，无 `STALE-BASELINE` / `STATE-DRIFT`。
- `cargo clippy -p attendance_assemble_control --all-targets -D warnings` 干净；
  crate 测试 54 passed（`u2l_v2_groupschedule_static_paths_win_over_params` 的可达性断言
  增加 `401` 分支，因 `list/filter` 先收会话门）。

## 批次 B-b6：surface / designer / work / task 八个按 id 读

判据同默认 1（本人 OR 管理员；空归属 fail-closed）。按 crate 既有原语落地，不新增第二套归属语义：

| 端点 | 表 / 归属列 | 落地方式 |
|---|---|---|
| `core_express task_processing` | `x_task.person` 或 `x_work.creator` | 新增 `require_read_owner(&[Option<String>])`，与 B-b4 `U4ActorKey::Task` 同口径 |
| `core_express work_processing` / `work_retract` | `x_work.creator` | 同上 |
| `core_entity work_get` | `pp_work.creator_id` | 先取行再判：本人或 admin；NULL→仅 admin |
| `surface get_surface` / `preview_surface` | `x_process_surface.creator` | 复用既有 `require_actor_opt` |
| `designer get_flow` / `preview_flow` | `x_process_definition.creator` | 新增 `guard_read_creator`（`require_owner` / 空归属走 `require_admin_gate`） |

- 无会话一律先 401（早于取库/取行）；`work_retract` 行不存在仍返回原 `ActionResult::error` 信封。
- `core_entity` 的路由只注入 `DatabaseConnection`（无 deadpool 池），管理员判定用 SeaORM
  连接跑同一套 `auth_person`/`auth_role` EXISTS，未给路由新增 `Extension<Pool>` 依赖。
- 门禁新增 8 条 `idor-read-own` 正控制；按棘轮点名减基线 8 行（111→103）。
- 破坏实验：`require_read_owner` 恒放行、`db_is_admin` 恒 true、surface 两处调用点短路、
  `guard_read_creator` 恒放行 → `NEW-FAIL` **恰好点名本批 8 条**；还原后非 PASS 103，与基线一致。

## 批次 B-b5：工作流运行面六个按 id 读（归属谓词写进 SQL）

判据沿用默认 1：`本人 OR 管理员`，NULL/空归属 fail-closed（仅管理员）。本批复用 crate 内既有
授权原语，不新增第二套语义：

| 端点（processplatform_service_processing） | 表 | 归属列 | 落地方式 |
|---|---|---|---|
| `GET /review/{id}` | `x_review` | `reviewer` | 新增 `u4_owner_read_row(select_sql, 列名)` 单列读谓词 |
| `GET /readcompleted/{id}` | `x_readcompleted` | `person` | 同上 |
| `GET /record/{id}` | `x_record` | `creator` | 同上 |
| `GET /applicationdict/{id}` | `x_process_definition` | `creator` | 同上 |
| `GET /attachment/{id}` | `x_attachment` | `creator` | 复用既有 `u4_attachment_read_row` |
| `GET /snap/{id}` | `x_snap` | `x_work.creator`（关联边） | 复用既有 `u4_require_actors(U4ActorKey::Snap)` |

- 谓词：`WHERE id = $1 AND (NULLIF(BTRIM(<owner_col>), '') = $2 OR EXISTS(admin))`；
  0 行命中时再用 `u4_row_exists_error` 区分 403（行在、非本人）/404（行不存在）。
- `snap` 不认 `x_snap.creator`/`person`：该表写入路径只写 `work_id`/`snap_type`/`snap_data`，
  归属只能 hop 到 `x_work.creator`（与写侧 `snap_id_restore` 同一把尺）；NULL work 归属→仅管理员。
- `x_attachment` 同时有 `work_id`/`workcompleted_id` 关联边，本批只按 `creator` 收紧 owner 一侧，
  关联边侧不改契约（与组织域「只收紧 owner」口径一致）。
- 写侧登记（不在本批改）：`x_review.reviewer` 由 `review_create_work` 的**请求体**指定，
  属写侧可指派字段；读侧按它收紧后，攻击者只能创建「指向他人」的评审，不能借此读到他人数据。
  `x_snap`/`x_readcompleted` 的 `creator`/`person` 列同样不由写入路径填充，因此读侧不以它们为准。
- 门禁：新增 6 条 `idor-read-own` 正控制；为 `snap` 正控制补一行 `rev-r6seed-work`
  （`x_work.creator = audit-user`）并把 `rev-r6-snapid.work_id` 指向它；`rev-r6-reviewid`
  补 `reviewer = audit-user`。其余四条复用既有 rev 行。
- 破坏实验：`u4_owner_read_row` 谓词改恒真（保留 `$2` 绑定）、`attachment_id` 调用点直查、
  `snap_id` 授权短路，`NEW-FAIL` **恰好点名本批 6 条**；还原后门禁 105 非 PASS，与基线一致
  （点名减 6：111→105），无 `STALE-BASELINE`。

## 附：`oauth_generate_code` 冒名落库（真实缺陷，独立 PR，不动基线）

- **复现（修复前）**：会话 `audit-user`，请求体 `{"clientId":"repro-oauth-client","personId":"gate-victim"}`
  → `x_org_oauth_code.person_id = gate-victim`（`tests/oauth_session_derived.rs` 断言失败：
  `left: Some("gate-victim") / right: Some("audit-user")`）。
- 根因：handler 直接 `req.get("personId")` → `oauth_code_store`，任何登录用户都能用别人的
  `personId` 生成授权码并落库。
- 修复：`person_id` 一律由会话派生，请求体 `personId` 不采信；响应补 `data.id`（表主键），
  门禁回查字段由只能读到 `data.code` 转为按 `data.id` 定位落库行。
- 回归测试：`crates/organization_assemble_authentication/tests/oauth_session_derived.rs`
  （scratch 库 + 真实 handler，断言落库 `person_id = audit-user`）。
- 门禁：该行仍为 UNVERIFIED —— `spoof-write` 只在 401/403 判 PASS，正确实现下是 2xx 且
  canary 无变化；按指示**不改基线**，但回查已能证明落库 owner 是会话本人（修复前回查不到行）。
- 未决：`clientId` 仍未与已注册应用比对（§五"第三方回调零验签族"），登记待裁决。
