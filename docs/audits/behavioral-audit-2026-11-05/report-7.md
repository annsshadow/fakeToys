# 第七轮行为审计：NO-OWNER-COL 业务裁决清单 / 假门族验证 / 六轮汇总索引

审计目录：`docs/audits/behavioral-audit-2026-11-05/`
证据原文：`evidence-round7.txt`　前序：`report-2.md` ～ `report-6.md`

> 边界：未修改任何业务代码、迁移、配置；无 `git add/commit/push/reset`。本轮全部为静态读码 + 对已有 scratch 库的表结构采样，未新发动态请求。
> 身份/环境沿用第六轮：`127.0.0.1:3999`、scratch 库 `oa_audit_20261105`（容器 `lyp-pg`，UTC）。

---

## 〇、基线（结论时效）

| 项 | 值 |
|---|---|
| 读码时 HEAD | **`c0622c5da`** |
| 工作树 | 修复线**未提交改动 5 个业务文件**：`attendance_core_entity/src/entities/attendance_record.rs`、`attendance_core_entity/src/lib.rs`、新增 `tests_p0_ownership.rs`、`migrations/117_add_x_attendance_record_created_by.sql(+rollback)`；另有 `?? docs/audits/behavioral-audit-2026-11-05/` |
| 相对第六轮 | 第六轮读码 HEAD `4b4e17010`，落盘 `c0622c5da`；本轮起点即 `c0622c5da`。`require_actor|require_owner|is_admin` 在 4b4e17010→c0622c5da 区间零命中（见 report-6 §〇） |
| 时间 | 2026-11-06 01:51（Asia/Shanghai） |

**必须声明**：本轮 T2 扫的是**当前工作树**，其中 `attendance_core_entity/src/lib.rs` 含有修复线未提交的 P0 改动（`record_create/update/delete` 归属谓词 + `attendance_rule` 三处 `require_admin`）。因此本轮对考勤相关项的判定反映的是「修复线改后的状态」，不是 `c0622c5da` 纯提交态。这一点直接影响 T1 第 14 条（结论：已被修复线覆盖）。

---

## 一、结论先行

| 问题 | 本轮结论 |
|---|---|
| T1 31 条 NO-OWNER-COL 能否直接修 | **不能整体机械套 owner**。31 条拆成三档：**12 条有可用归属列**（工程活）、**5 条列存在但生产路径从不写**（要先补写或回填）、**14 条真·全局/命名空间**（必须业务裁决） |
| 假门是不是更大的一族 | **不是**。54 条路由候选逐条判定后，真·假门仍是 **4 条**（cms 3 + 考勤 1），且都不执行写操作；没有新增可直接利用的假门 |
| 但静态扫描的偏差比预想更大 | **双向都偏**：21 条真门因「拒绝在 helper 里 / 行级过滤无 `Err`」显得无门（**高估风险**）；2 条 200 软门会用 `ActionResult::error` 拒绝但状态码 200（**低估风险**）；11 条权限元数据端点**根本没取权限**（**漏报**） |
| 还剩多少没解决 | 六轮 36 个条目：**完全关闭 5 · 部分关闭 8 · 未动 19 · 不可判 4**。见 §3 |

---

## 二、T1：31 条 `NO-OWNER-COL` → 业务可裁决清单

### 2.1 先纠一个判据错误（这决定 31 里有多少其实能直接修）

第六轮的判据是「表内是否存在脚本白名单里的本人列」（`creator/owner/person/user`）。实际这些列名同样表达归属，但被漏掉：

| 列名 | 出现表 | 语义 |
|---|---|---|
| `creator_person` | `x_attendance_v2_group`、`x_attendance_v2_shift`、`x_cms_document`、`x_program_warn_log` | 创建者本人 |
| `upload_person` | `x_cms_fileinfo` | 上传者本人 |
| `identity_id` | `x_org_bind`、`x_org_custom` | 绑定的身份本人 |
| `assignee_id` | `x_process_task` | 任务承办人 |
| `dict_id` | `x_general_application_dict_item` | 父字典（间接归属到 `application`） |
| `id`（自表） | `auth_person` 的 `/setting`、`/icon` | 被访问者本人 |

**修正后的分解**（判据从「列是否存在」改成「**列是否真的被写入**」，这一条比列名白名单更关键）：

| 分组 | 条数 | 能不能直接修 | 需要业务裁决什么 |
|---|---:|---|---|
| 有可用归属列且生产路径确实写入 | **12** | 可以，按列写谓词 | 不需要（仅确认「本人 or 管理员」这条口径） |
| 列存在但生产路径从不写（实际为 NULL） | **5** | 不能直接修；需先补写列或回填 | 先决定「由谁在什么时点写」 |
| 真·全局配置 / 无主体列 | **14** | 不能 | 需要回答「这张表归谁」：全局（admin 写/登录读）还是某个业务命名空间 |

具体：
- **12 条可用**：`x_attendance_v2_shift`（`creator_person` 写入）、`x_attendance_v2_group`（`creator_person`/`participate_list`/`assist_admin_list` 均写入）、`x_corr_c_correlation`（`source_id`/`target_id`）、`auth_person`×2（`id` 即本人）、`x_process_task`（`assignee_id`）、`x_program_warn_log`（`creator_person`）、`x_org_bind`×2 / `x_org_custom`×3（`identity_id`）。
- **5 条列存在但不写**：`x_cms_fileinfo` 的 `upload_person` —— 除「从已有行复制」外，**所有 INSERT 都不带该列**（`cms_assemble_control/src/lib.rs:4277/4754/4791/4828/4855/11604`；`:4700` 有注释明确说明），当前实际为 NULL。对应 `/api/fileinfo/{id}`、`/online/info`、`/preview/pdf`、`update/{id}/content` 4 条 + `/api/document/{id}/view/count`（`x_cms_document.creator_person`/`xowner` 在 Rust 侧无 INSERT 路径，来源不可判）1 条。
- **14 条真·全局**：`x_general_application_dict`×3、`x_general_application_dict_item`×3（`dict_id` 指向的父字典本身无主体）、`x_cms_category`、`x_jpush_template`×2、`x_meeting_building`、`x_meeting_room`×2 + 照片 1、`x_attendance_rule`。

> 所以第六轮 P2 第 8 条「31 条与业务确认」要收窄：**真正需要业务回答的是 14 条**；12 条是工程活；5 条要先解决「列谁来写」。
>
> 另注：`x_attendance_v2_group` 第 16 条虽有 `creator_person`/`participate_list`，**但不能只看列**——它同时有 `top_unit/unit_list` 组织范围，两种归属定义会给出完全不同的可见集，所以仍列在「两种定义」表里，由业务选其一。

### 2.2 分组 A：全局配置 / 资源（17 条）

筛选字段一律只有 `id`（部分附 `deleted_at IS NULL`），**当前没有任何第二维度过滤**。

| # | 路由 | 表 | 数据语义 | 归属定义 ①（推荐） | 归属定义 ② | 两种定义的后果差异 | 现状/修复 |
|---|---|---|---|---|---|---|---|
| 1 | GET `/api/general/dict/{id}` | `x_general_application_dict` | 应用级键值字典；`application` 是命名空间 | **按 application 归属**：登录可读本应用字典；写=该应用管理员 | **全局共享**：登录可读，写=平台 admin | ① 断掉跨应用字典复用（同一 key 在不同应用可不同值）；② 任何登录用户能读/改全局字典，改一个值影响所有应用 | 登录即可读；写无门 |
| 2 | GET `/api/general/dict/item/{id}` | `x_general_application_dict_item` | 字典项；`dict_id` 指向父字典 | **随父字典**（`dict_id → application`） | 全局共享 | ① 实现需 join 父表；② 实现最简单但泄露所有业务字典 | 登录即可读 |
| 3 | POST `/api/general/dict/delete/{id}` | 同 1 | 删字典（软删） | 按 `application` 管理员 | 平台 admin | ① 允许应用管理员自助删本应用字典；② 必须找平台 | 登录即可删；**当前真实可利用** |
| 4 | POST `/api/general/dict/item/delete/{id}` | 同 2 | 删字典项 | 随父字典 | 平台 admin | 同上 | 登录即可删 |
| 5 | POST `/api/general/dict/item/update/{id}` | 同 2 | 改字典项 name/value | 随父字典 | 平台 admin | 同上 | 登录即可改 |
| 6 | POST `/api/general/dict/update/{id}` | 同 1 | 改字典 name/application | 按 `application` 管理员 | 平台 admin | 同上；注意 `application` 本身可被改 → ① 要防止「改命名空间逃避权限」 | 登录即可改 |
| 7 | GET `/api/cms/category/{id}` | `x_cms_category` | 栏目树节点；`parent_id` 层级 | **全局栏目树**：读=登录，写=admin | **按节点归属**（需新增 `creator_person`） | ① 简单、与现状最接近；② 要回填历史行的 creator，且「父栏目管理员能否管子栏目」需再定义 | 登录即可读；写无门 |
| 8 | GET `/api/jpush/template/{id}` | `x_jpush_template` | 推送模板库（name/title/content） | **全局模板库**：读=登录，写=admin | **创建人私有**（需新增 creator） | ① 全员可复用模板，但模板内容对所有应用可见；② 模板不可复用，每应用自建 | 1 行样本；登录即可读 |
| 9 | GET `/api/jpush/core/entity/template/{id}` | 同 8 | 同上（core 路由别名） | 同 8 | 同 8 | 同 8 | 登录即可读 |
| 10 | GET `/api/meeting/assemble/control/building/{id}` | `x_meeting_building` | 楼宇（name/address/pinyin） | **全局资源目录**：读=登录，写=admin | **按单位归属**（需 `unit_id`） | ① 楼宇是公共设施，全员可查合理；② 只能看到本单位楼宇，跨单位预约需额外授权 | 1 行；登录即可读 |
| 11 | GET `/api/meeting/assemble/control/room/{id}` | `x_meeting_room` | 会议室；`building_id` 归属楼宇，`open_meeting` 是否开放 | **全局目录**：读=登录（含未开放房间元数据），写=会议管理员 | **可见性过滤**：`open_meeting=true` 或本人是预订人/管理员才可见 | ① 未开放房间名/容量/设备也泄露（现状 5 行全列出）；② 未开放房间从目录消失，前端需处理「不可见≠不存在」 | 5 行；登录即可读 |
| 12 | GET `/api/meeting/core/entity/room/{id}` | 同 11 | 同上（core 别名） | 同 11 | 同 11 | 同 11 | 登录即可读 |
| 13 | POST `/api/meeting/assemble/control/room/{id}/photo` | `x_meeting_room.photo` | 会议室照片；只校验 room 存在，直接 `UPDATE x_meeting_room SET photo WHERE id` | **会议室管理员**（按 11 的写口径） | **任意登录用户**（现状） | ① 只有管理员能改照片；② 任何人可往任意房间塞图（内容污染/钓鱼图） | 只 `SELECT room WHERE id`，无门；**可写** |
| 14 | POST `/api/attendance/core/entity/rule/{id}/update` | `x_attendance_rule` | 全局考勤规则（上下班时间） | **全局配置 → 写=admin** | **按考勤组归属**（需 `group_id`） | ① 一人改规则影响全公司，风险集中但可控；② 每组独立规则，实现改动大 | **已被修复线覆盖**：工作树已加 `require_admin`（未提交）；`c0622c5da` 仍无门 |
| 15 | GET `/api/attendance/assemble/control/v2/shift/{id}` | `x_attendance_v2_shift` | 班次模板；`creator_person`/`operator` 存在但 SELECT 未取 | **创建人 + admin**（用现有 `creator_person`） | **全局模板**：读=登录，写=admin | ① 历史行 `creator_person` 可能为空 → 空值需 fail-closed 到 admin；② 简单但模板互相可见 | 登录即可读；0 行样本 |
| 16 | GET `/api/attendance/assemble/control/v2/group/{id}` | `x_attendance_v2_group` | 考勤组：`participate_list` 成员、`assist_admin_list` 协管、`creator_person` 创建人、`top_unit`/`unit_list` 组织范围 | **成员归属**：成员/协管/创建人可读；写=协管/创建人/admin | **组织范围归属**：`top_unit`/`unit_list` 内的人可读 | ① 「我的考勤组」语义，跨组不可见；② 同单位可见所有组 → 泄露他人排班与成员名单（1 行样本已含全部列） | 登录即可读；**当前真实可读** |
| 17 | DELETE `/api/correlation/core/entity/delete/{id}` | `x_corr_c_correlation` | 实体关联边（`source_type/source_id/target_type/target_id`） | **按端点实体归属**：`source_id` 或 `target_id` 的实体所有者可删（需按 type 分派） | **全局关系数据**：写=admin | ① 实现重、需 type→表 映射；② 简单，但任何登录用户能拆任意关联（审计链破坏） | 登录即可删；0 行 |

### 2.3 分组 B：人员主体（3 条）

| # | 路由 | 表 | 数据语义 | 归属定义 ①（推荐） | 归属定义 ② | 后果差异 | 现状 |
|---|---|---|---|---|---|---|---|
| 18 | GET `.../authentication/person/{id}/icon` | `auth_person` | 头像 URL | **本人可改；他人可读**（头像属公开资料） | **仅本人/通讯录可见** | ① 与现状一致，风险低；② 会破坏「头像在组织内可见」的产品预期 | 登录即可读；低敏 |
| 19 | GET `.../personal/{id}/setting` | `auth_person` | 手机号/邮箱/图标/主题/语言 | **本人或 admin** | **本人 + 同单位可见**（通讯录语义） | ① 只有本人/admin 能读手机号邮箱；② 组织内可互查 → **PII 扩散面更大，不推荐** | **任何人按 id 读他人 mobile/email**；当前真实 PII 泄露 |
| 20 | GET `/api/process/task/{id}` | `x_process_task` | 待办任务；`assignee_id` 承办人，`work_id` 关联流程 | **assignee_id 本人 + 流程发起人 + admin** | **同流程参与者可见**（按 work_id 的办理链） | ① 只开放给承办人，最保守；② 同流程协作者可见，符合协作但需先算参与者集合 | 登录即可读标题/状态；**当前真实可读** |

### 2.4 分组 C：内容 / 文件（6 条）

> **关键前提**：`x_cms_fileinfo.upload_person` 虽有列，但生产 INSERT 路径**从不写**（详见 §2.1）。所以本组 5 条不能直接按 `upload_person` 收口，必须先补写该列（或改为跟随父文档 `doc_id` 的 ACL）。

| # | 路由 | 表 | 数据语义 | 归属定义 ①（推荐） | 归属定义 ② | 后果差异 | 现状 |
|---|---|---|---|---|---|---|---|
| 21 | GET `/api/document/{id}/view/count` | `x_cms_document`（只 `SELECT 1` 判存在）+ `x_cms_document_view_count` | 文档阅读计数，不返回正文 | **跟随文档 ACL**（`x_cms_permission` + `creator` + `published`） | **仅 published 可读** | ① 有权限者看计数合理；② 未发布文档一律 404 式拒绝，但会破坏草稿作者自看 | 登录即可读；可枚举文档存在性 |
| 22 | GET `/api/fileinfo/{id}` | `x_cms_fileinfo`（`doc_id`→父文档；`upload_person` 列**恒为空**） | 附件元数据（原文件名/大小/类型） | **跟随父文档 ACL**（唯一可行口径） | **先补写 `upload_person` 后按上传人私有** | ① 现在就能修，用 `doc_id` 查文档 ACL；② 要先改上传落库（工程量大），但能保留「上传者独占」语义 | **任何人按 id 读他人附件元数据** |
| 23 | GET `/api/fileinfo/{id}/online/info` | 同 22 | 在线预览元信息 | 同 22 | 同 22 | 同 22 | 同上 |
| 24 | GET `/api/fileinfo/{id}/preview/pdf` | 同 22 | PDF 预览渲染（SELECT 不取 `upload_person`） | 同 22 + 需 downloadable | **登录即可**（现状） | ① 内容不泄露；② **任何人可渲染他人附件 → 内容泄露**（当前真实） | 登录即可读，**最高危的一条读** |
| 25 | POST `/api/fileinfo/update/{id}/content` | 同 22 | 只改元数据（原名/大小/type），无字节 | **父文档编辑者 + admin** | **先补写 `upload_person` 后按上传人** | ① 立即可修；② 需先改上传落库 | 登录即可改他人附件元数据（污染/伪造） |
| 26 | GET `/api/program_center/warnlog/{id}` | `x_program_warn_log` | 运维告警（level/tag/message/host/port）；`creator_person` 是触发行人 | **运维/admin 全局 + 脱敏** | **creator_person 本人**（看自己触发的告警） | ① 运维能全看、对普通用户脱敏 host/port；② 只有触发行人能看，运维反而要逐人查 | 登录即可读 host/port/内部 message；**当前真实可读** |

### 2.5 分组 D：组织绑定 / 自定义字段（5 条）

| # | 路由 | 表 | 数据语义 | 归属定义 ①（推荐） | 归属定义 ② | 后果差异 | 现状 |
|---|---|---|---|---|---|---|---|
| 27 | PUT `/api/organization/bind/{id}` | `x_org_bind`（`identity_id`/`group_id`/`role`） | 身份在某组的角色绑定 | **identity_id 本人**（只能改自己的绑定） | **组管理员/admin** | ① 本人可自助加入/退出组；② 只能管理员改，安全但自助性差 | 登录即可改他人绑定 → **提权**（把自己绑到高权组） |
| 28 | DELETE `/api/organization/bind/{id}` | 同 27 | 解绑 | 同 27 | 同 27 | 同上；删除他人绑定可**踢人/破权** | 登录即可删 |
| 29 | PUT `/api/organization/custom/{id}` | `x_org_custom`（`identity_id`/`field_name`/`field_value`） | 人员自定义字段值 | **identity_id 本人** | **字段定义的管理员** | ① 本人只能改自己的字段值；② 由字段管理员统一管，适合「部门/职级」这类受控字段 | 登录即可改他人字段（伪造部门/职级） |
| 30 | DELETE `/api/organization/custom/{id}` | 同 29 | 删自定义字段行 | 同 29 | 同 29 | 同上 | 登录即可删 |
| 31 | POST `.../personal/custom/{id}/mockputtopost` | 同 29 | 同上（契约别名路径） | 同 29 | 同 29 | **注意：中间件已按 `/api/organization/assemble/personal` 前缀对 POST 要求 admin**（`ADMIN_WRITE_PREFIXES`）；该路径不在自服务豁免白名单 → **普通用户实际被 403 挡住**。剩余风险是「admin 可改任何人」，这通常可接受 | 中间件已覆盖（非 handler 级） |

### 2.6 给业务的裁决模板（只需回答 3 个问题）

1. **这张表的读权限**：登录即可读 / 只有本人和 admin / 只有有业务关系的人（成员、参与者、同组）。
2. **这张表的写权限**：本人 / 业务管理员 / 平台 admin。
3. **历史行没有归属值时怎么办**：拒绝（fail-closed，推荐）/ 归给 admin / 回填一个系统账号。

按此回答，2.2 的 11 条真·全局项可以一次裁决完；2.3–2.5 的 20 条已有归属列，只需确认「本人 or 管理员」这一条口径即可动工。

---

## 三、T2：验证「假门」是不是更大的一族

### 3.1 三个探测器（避免只用一种判据）

| 探测器 | 口径 | 命中 |
|---|---|---:|
| **A. 权限信号 + 无拒绝分支** | 函数体内出现 `is_admin/is_manager/role/permission/allow_*` 等信号，但既无 `Err(Forbidden/Unauthorized)`，也无 `if admin` 的条件分支 | 91 个信号函数 → **54 条注册路由** |
| **B. 取了 Session 但从不引用** | 签名接收 `Session`，函数体（去字符串/注释后）不出现该绑定名 | 14 个函数 → 其中**仅 2 个是 handler，且两者都无路由** |
| **C. 有权限调用 + 有写操作 + 无拒绝 + 无 admin 条件** | 更严格，专门找「算了权限却照写」 | 41 个 → 逐一读体后**全部落在 A 的 6 类里**，无新增 |

> 探测器 B 的关键结论：全仓只有 `cms_assemble_control::document_cipher_permission_read_u3` 出现 `let _ = &session;`（显式丢弃会话）。其余 13 个「未引用 Session」的函数全是 `SessionManager`（职责是**写**会话）或测试 helper，不是假门。

### 3.2 54 条路由候选逐条判定结果

| 判定 | 条数 | 含义 |
|---|---:|---|
| **真拦截（HTTP 4xx）** | **12** | `require_admin/require_owner/check_owner` 或显式 `Err(Forbidden)`，且拒绝就在调用路径上 |
| **软门（HTTP 200 业务错误）** | **2** | `ActionResult::error("forbidden: admin required")`：**确实不执行写**，但状态码 200 |
| **真限权（行级过滤/条件写）** | **9** | 无拒绝分支，但 `admin` 决定 SQL 的 `WHERE` 或 `if is_manager { UPDATE }`；数据面已收窄 |
| **假门（权限值只进响应）** | **4** | 权限变量只用于构造返回布尔/字符串，既无拒绝也无行过滤 |
| **无门读（权限元数据）** | **11** | 函数里根本没有权限判定，直接按 id 返回 ACL/权限串 |
| **误报（变量不是权限信号）** | **16** | `row/rows/result/affected/permissions(业务字段)`，或 express 公共只读接口 |
| 合计 | **54** | |

> 计数口径：按精确函数体边界（花括号配平，先去字符串/注释）重判，避免扫描窗口溢出到相邻函数。

#### 3.2.1 真·假门：4 条（与第六轮一致，未扩散）

| # | 端点 | 权限值去向 | 是否执行写 | 判定 |
|---|---|---|---|---|
| 1 | `cms/document_control_u3`（`lib.rs:11144`） | `allowManage = admin \|\| creator == session`；`allowRead` **恒 true**；`createdBy = if admin {"admin"}` | 否（纯探针） | 假门：只影响返回值；`allowRead` 对所有文档恒真 |
| 2 | `cms/document_permission_read_u3`（`:11210`） | `readable = admin \|\| creator == session \|\| published` | 否 | 假门：把判定结果返回给前端，服务端不拦 |
| 3 | `cms/document_cipher_permission_read_u3`（`:11034`） | `readable = is_admin(**入参 person**) \|\| person==creator \|\| status=="published"`；`let _ = &session;` | 否 | 假门 **+ 任意主体探测**：可查「任意人 × 任意文档」的权限结果 |
| 4 | `attendance/v2_my_controls`（`:8579`） | `admin`/`readAdmin`/`assistAdmin` 三个布尔 | 否（能力探针） | 假门但**非漏洞**：端点职责就是返回能力位，且 assist 查询确实用了会话 |

**结论**：假门**不是更大的一族**。第六轮点名的 3 条 cms 仍是全部真例；第 4 条是设计上的能力探针，无副作用。假门真正的风险不在「本端点越权」，而在**前端把服务端布尔当授权结果**——所以修复口径应是「探针与执行同一判据，执行端点必须二次校验」。

#### 3.2.2 软门：2 条（静态扫描会低估）

| 端点 | 代码 | 后果 |
|---|---|---|
| `empower::manager_create`（`:342`） | `if !is_admin { return Ok(Json(ActionResult::error("forbidden: admin required"))) }` | 写不执行，但 **HTTP 200**；只按状态码判「是否打通」的扫描会把它记成「已越权」 |
| `empower::manager_update`（`:397`） | 同上 | 同上 |

这两条**不是假门**（拒绝分支存在且生效），但会污染「200 = 成功」的结论，必须按响应体 `code/message` 二次判定。

#### 3.2.3 真限权：9 条（最容易被误判成假门）

| 端点 | 形态 |
|---|---|
| `attendance/v2_leave_list_page_size` | `let filter_person = if admin { 请求体 } else { 会话人 }` |
| `attendance/v2_appeal_list_page_size` | 同上（`filter_user`） |
| `file_assemble_control/file_list_*`（5 条） | `owner_clause(admin, …)`：非 admin 强制拼 `upload_person = 会话人` |
| `pp_designer/process_upgrade_all` | `if is_manager { UPDATE PP_E_PROCESS … }`，非管理员**不写**、只回 `false` |
| `pp_surface/review_u2_v2_search` | `let person = if admin { 请求体 } else { 会话人 }` |

这 9 条**没有 `return Err`**，但授权是生效的。它们证明：**「无拒绝分支」≠「无授权」**——行级过滤和条件写是同一件事的另一种实现。

#### 3.2.4 无门读：11 条（比假门更值得裁决）

| 端点 | 暴露内容 |
|---|---|
| `cms/document_persons_u3` | 任意文档的**可读人员名单**（`x_cms_permission` + creator），无任何判定 |
| `general/get_module_permissions` | 模块权限矩阵 |
| `organization/permissionsetting_list` | 全部权限设置行（含创建人） |
| `organization/permissionsetting_flag`（`:1927`） | 任意权限设置行的 `name/unit_id/creator` |
| `organization/permissionsetting_flag_mockputtopost`（`:2015`） | 名字像写，实际只 `SELECT` 并返回 |
| `portal/portal_id_permission` | 任意门户的 `permission` 串 |
| `query_assemble_designer/{importmodel,query,stat,table,view}_id_permission`（5 条） | 任意查询对象的 `permission` 串 |

这些不是「假门」（它们连权限值都没取），而是**权限元数据的无门读**。是否算缺陷取决于业务：ACL 本身是否公开。建议在 T1 裁决时一并回答「权限串能不能给普通用户看」。

#### 3.2.5 误报：16 条（判据要收窄）

| 类型 | 条目 | 为什么不是权限信号 |
|---|---|---|
| 登录/用户信息端点 | `auth/login`、`bbs/login`、`bbs/user_info` | `Unauthorized` 是认证失败，`session` 是登录产物；端点本就不需要归属门 |
| 配置读端点 | `bbs/cms/cms_control/component` 的 `get_control_config`、`general/get_general_control_status`、`general/general_control_list` | 命中的变量是 `row/rows/config_value` 这类**配置数据**，不是权限变量 |
| 公共只读 express | `organization_assemble_express` 5 条（`person_has_role`/`role_list_person*`/`group_has_role`） | `express`/`batch` 前缀按设计是 Public 只读 |
| 内部 helper | `shared/rbac::person_has_role` | 是判据实现本身，不是端点 |
| 业务字段 | `empower/create` 的 `role_id`、`file/permission_set` 的 `permissions`、`dict update` 的 `allow_anonymous` | 分别是「委托目标角色」「权限串内容」「字典开关」，都是被写入的数据，不是调用者的权限 |

### 3.3 对「静态扫描系统性高估安全性」的校准

| 偏差方向 | 场景 | 本轮数量 | 后果 |
|---|---|---|---|
| **高估风险（把真门当假门）** | 拒绝在 helper 内（`require_admin(...)` 内部 `Err`），调用方函数体看不到拒绝；行级过滤无 `Err` | 12 + 9 = **21 条** | 修错地方、白改 |
| **低估风险（把假门当安全）** | 权限值只进响应，扫描看到 `is_admin(` 就以为已拦 | **4 条** | 漏修 |
| **状态码误判** | `ActionResult::error` 拒绝但 200 | **2 条** | 把「被拒」记成「打通」 |
| **完全漏报** | 端点根本不取权限值 | **11 条** | 扫描器连候选都不会列 |

**结论**：只靠「有没有拒绝分支 / 有没有 `is_admin`」的单判据，会同时产生 21 条假阳性和 17 条假阴性。可靠判据必须同时问三件事：① 是否有评估（`is_admin/require_*`）；② 评估结果是否进了**控制流或 SQL WHERE**；③ 拒绝是否可被 **HTTP 状态 + 响应体** 区分。

---

## 四、T3：六轮汇总索引（一眼看「还剩多少没解决」）

口径：每条一行；状态取用户定义的 5 档。`修复线` 列描述当前覆盖程度，不臆测未登记的修复。

| # | 条目 | 状态 | 修复线 | 报告 |
|---:|---|---|---|---|
| 1 | F2-E-1/2 考勤列表 2 条（本人过滤） | 已实证 | **已覆盖**（`1c639766b` PR #184） | [report-2](report-2.md) |
| 2 | F2-E-3..14 其余 12 个列表读他人数据 | 已实证 | 未动 | [report-2](report-2.md) |
| 3 | F1-A/B/C/D/E 假翻页（85+19+4+offset+count 语义） | 已实证 | 未动 | [report-2](report-2.md) / [report-3](report-3.md) |
| 4 | F8 AI `create_time=NULL` 列表 500 | 已实证 | 未动 | [report-2](report-2.md) |
| 5 | G1 Go `/admin/users` `hasMore` 未 clamp | 已实证 | 未动 | [report-2](report-2.md) |
| 6 | G2 Go 无 wallet 行 → 用户详情整 404 | 已实证 | 未动 | [report-2](report-2.md) |
| 7 | G3 Go `/me/battles` 无 tiebreak 跨页重复 | 已实证 | 未动 | [report-2](report-2.md) |
| 8 | G4/G5 负证据（NULL 不 500；`/me/*` 归属正确） | 已实证（正确） | **关闭** | [report-2](report-2.md) |
| 9 | R3-T1 389 候选中**已实证越权 8 条** | 已实证 | 部分（随 P0 批次） | [report-3](report-3.md) |
| 10 | R3-T1 **静态确认越权 86 条** | 静态确认 | 部分（一级 16 已入台账） | [report-3](report-3.md) |
| 11 | R3-T1 不可判 55 / 不可达 53 | 不可判 | — | [report-3](report-3.md) |
| 12 | R3-T3 写后必读 13 例（8 缺陷） | 已实证 | 部分（Go 2 例已修 `961bda135`） | [report-3](report-3.md) |
| 13 | R3-T4 Go 25 个玩家 GET 全量 | 已实证（正确） | **关闭** | [report-3](report-3.md) |
| 14 | R4-T1 125 个「敏感字段取请求体」路由 | 静态确认 | 部分（一级 16 已分层） | [report-4](report-4.md) |
| 15 | R4-T2 冒名（bbs/message/calendar 4 条） | 已实证 2 + 静态 1 + 断链 1 | 未动 | [report-4](report-4.md) |
| 16 | R4-T3 `create_time` 8h 时区 | 已实证 | 未动 | [report-4](report-4.md) |
| 17 | R4-T4 Go 48 写端点 | 已实证（2 缺陷已修） | **已覆盖** | [report-4](report-4.md) |
| 18 | R5 空表 3 表（bbs subject / mind base / mind folder） | 已实证 | 未动 | [report-5](report-5.md) |
| 19 | R5 审批链 creator 空 fail-open（`pp_c_*`/`x_work`） | 已实证 | 部分：`u4_require_actor` 已登记 P0-b；`work_terminate` 未收录 | [report-5](report-5.md) / [report-6](report-6.md) |
| 20 | R5 `/api/file/upload` 的 `person` 取表单（可冒名） | 已实证 | 未动 | [report-5](report-5.md) |
| 21 | R5 `/api/file/anonymous/file/{id}/download` 无门 | 已实证 | 部分（`L1-3` 仅此条） | [report-5](report-5.md) |
| 22 | R5 125 条分级（一级16/二级20/三级9/覆盖80） | 静态确认 | 部分 | [report-5](report-5.md) |
| 23 | R6 path-IDOR 四档 421/176/245/141 | 已实证 | 未动 | [report-6](report-6.md) |
| 24 | R6 176 条有门复核（145 真 403 / 3 假门 / 28 无后果） | 已实证 | 未动（假门未收录 P1） | [report-6](report-6.md) |
| 25 | R6 写 IDOR 24 条（13 条落库确认） | 已实证 | 未动（台账未建行；`record_*` 修复线在途） | [report-6](report-6.md) |
| 26 | R6 读 IDOR 117 条 | 已实证 | 未动（E 系列只覆盖列表） | [report-6](report-6.md) |
| 27 | R6 fail-open `u4_require_actor`（16 调用点） | 已实证 | **已覆盖**（P0-b） | [report-6](report-6.md) |
| 28 | R6 fail-open `work_terminate` 内联 | 已实证 | 未收录（建议并入 P0-b） | [report-6](report-6.md) |
| 29 | R6 双向交叉读 2 个一级读端点 | 已实证 | 未动 | [report-6](report-6.md) |
| 30 | **R6/R7 31 条 NO-OWNER-COL** | **待业务裁决** | 部分：`attendance_rule` 已修（工作树） | [report-6](report-6.md) / [report-7](report-7.md) |
| 31 | R6 `SKIP-NO-TABLE` 9 条 | 不可判 | 待补 `PP_C_*`/share 表映射 | [report-6](report-6.md) |
| 32 | **R7 真·假门 4 条**（cms 3 + 考勤 1） | 已实证 3 + 静态 1 | 未动（P1） | [report-7](report-7.md) |
| 33 | **R7 软门 2 条**（empower `manager_*`） | 静态确认 | 未动 | [report-7](report-7.md) |
| 34 | **R7 无门读 11 条**（ACL/权限串） | 静态确认 | 未动（待业务裁决） | [report-7](report-7.md) |
| 35 | R7 Session 取了不用 2 handler | 不可判 | 无路由（不可达） | [report-7](report-7.md) |
| 36 | R6 `REACHED-ERROR`/`FAIL-500` 等 20 个读端未能判定 | 不可判 | 待复跑 | [report-6](report-6.md) |

### 4.1 「还剩多少」计数

| 类别 | 条数 | 条目号 |
|---|---:|---|
| **完全关闭**（已覆盖或已证明正确） | **5** | 1、8、13、17、27 |
| **部分关闭**（有覆盖但不完整） | **8** | 9、10、12、14、19、21、22、30 |
| **未动**（已实证或静态确认，尚未修） | **19** | 2、3、4、5、6、7、15、16、18、20、23、24、25、26、28、29、32、33、34 |
| **不可判** | **4** | 11、31、35、36 |
| 合计 | **36** | |

一句话：**36 个条目里 5 个已闭环、8 个部分闭环、19 个仍未动、4 个还判不了**。未动的 19 条里，按危害排序的前四个仍是第六轮的 P0：空归属写（#19/#28）、写 IDOR（#25）、读 IDOR（#26）、财务/组织写（含在 #25）。

---

## 五、未闭环与本轮新增的待办

1. **T1 的 14 条真·全局项需要业务回答**（§2.2 #1–13、#17）：是否「读=登录、写=admin」；若是，工程上只需加 admin 门；若不是，要先定命名空间。
2. **T1 的 12 条有可用归属列**可直接按「本人 or admin」修；其中 `auth_person /setting`（#19）建议优先，因为泄露 mobile/email。
2b. **T1 的 5 条「列存在但不写」**（fileinfo 4 条 + view/count 1 条）：先决定是「补写归属列」还是「改为跟随父文档 ACL」；后者可以立刻修。 
3. **假门 4 条**建议统一改法：把「探针」与「执行」共用同一判据，并在执行端点二次校验；`document_cipher_permission_read_u3` 的 `person` 必须改为会话人。
4. **软门 2 条**不改逻辑，但要在测试里断言「响应体 code≠0 且未落库」，避免状态码扫描误判。
5. **无门读 11 条**等业务裁决「权限串是否公开」。
6. **不可判 4 项**需要复跑或补表映射（#31/#36）。

---

## 六、可复跑

本轮全部为静态判据，未新增造数。可复跑脚本在临时目录 `%TEMP%\audit4\`：

| 脚本 | 产出 |
|---|---|
| `r7_tables.py` | 17 张表结构+样本 → `r7_tables.json` |
| `r7_fakegate.py` | 权限信号扫描 → `r7_gate_scan.json`（91 信号 / 54 路由） |
| `r7_t2table.py` | 54 条判据表 → `r7_t2table.log` |
| `r7_session_scan.py` | Session 取了不用扫描 → `r7_session_scan.log` |
| `r7_fakegate2.py` | 严判据（权限调用+写+无拒绝）→ `r7_fakegate2.log` |
| `r7_noowner_now.py` | 31 条按当前工作树复核门禁 → `r7_noowner_now.log` |
| `r7_noowner.py` / `r7_noowner_bodies.py` | 31 条 SQL/函数体导出 |

> 复跑注意：`r7_*` 读的是**工作树**，若修复线提交，`attendance_rule`（#14）会自动从「未动」变「已覆盖」；`record_*` 的归属谓词也会改变 T1 之外的分母。
