---
date: 2026-08-11
topic: oa4rust-full-gap-closure-comprehensive
---

# OA4Rust 全面差距补全 — 业务逻辑深度对齐

## Summary

识别并补全 oa4rust 与 Java o2server 在业务流程逻辑层面的全部缺口，按优先级分层推进：流程引擎核心状态机 → 文件管理完整能力 → BBS 社区认证与权限 → 组织控制 CRUD → 质量/安全修复，使 oa4rust 在所有功能和业务逻辑上完全代替 o2server，支撑 o2web 前端接入。

---

## Problem Frame

oa4rust 已完成 86 个 crate 的注册和 7,600+ 条路由的路由挂载，但审计揭示：端点"存在"不等于业务逻辑"可用"。流程引擎核心（processplatform_service_processing）92/98 个端点是硬编码 stub，工单状态机、任务流转、审批、快照等关键流程操作全部不可用；文件管理 79/91 端点 stub，文件下载、预览、上传、回收站等核心功能缺失；BBS 用户认证（login 返回空 token）、权限检查（全硬编码 true）、列表查询（精华/推荐/搜索全为空）大面积失效；组织控制缺失人员卡、权限设置、身份等核心 CRUD；此外 query 模块不执行动态 SQL、console 返回假数据、CMS 无权限体系等安全问题并存。这些缺口导致 o2web 前端无法完成实际业务流程联调，oa4rust 尚不具备替代 o2server 的条件。

---

## Actors

- A1. **开发者**：按优先级分层实现所有缺口补全
- A2. **前端 o2web**：依赖完整业务流程的 HTTP 响应，stub 端点返回假数据会导致前端渲染异常或流程中断
- A3. **AI Agent / MCP 客户端**：通过 MCP 工具桥接调用 oa4rust，stub 端点返回的假数据会导致 Agent 行为错误
- A4. **CI 流水线**：`cargo check --workspace` 和 `cargo test --workspace --lib` 验证无回归

---

## Key Flows

- **F1. 流程工单操作流**
  - **Trigger：** 用户在 o2web 发起/处理/终止/退回/撤销工作流
  - **Actors：** A2, A3
  - **Steps：**
    1. 客户端调用工单操作端点（processing/terminate/retract/goback/rollback 等）
    2. oa4rust 执行工单状态机变更（x_work 状态字段更新）
    3. 创建/更新关联任务（x_task）、审批记录（x_review）、操作日志（x_record）
    4. 返回操作结果和更新后的工单投影数据
  - **Outcome：** 工单全生命周期操作可用，与 Java 行为一致
  - **Covered by：** R1, R2, R3, R4, R5

- **F2. 文件下载/预览流**
  - **Trigger：** 用户在 o2web 点击下载文件或预览附件
  - **Actors：** A2
  - **Steps：**
    1. 客户端请求文件下载端点
    2. oa4rust 从存储后端读取二进制数据（文件系统或 BLOB）
    3. 设置正确的 Content-Type、Content-Disposition 响应头
    4. 流式返回文件内容
  - **Outcome：** 文件可正常下载和预览，与 Java 行为一致
  - **Covered by：** R6, R7, R8, R9

- **F3. BBS 用户认证流**
  - **Trigger：** 用户访问 BBS 板块或发帖
  - **Actors：** A2
  - **Steps：**
    1. 客户端调用 login 端点，传入 credential + password
    2. oa4rust 验证凭据，签发 session token
    3. 返回包含用户 ID、token、用户信息的完整响应
    4. 后续请求携带 token 进行权限校验
  - **Outcome：** BBS 用户认证可用，权限检查返回真实结果
  - **Covered by：** R13, R14, R15, R16

---

## Requirements

**Tier 1 — 流程引擎核心（最高优先级）**
- R1. 补全 processplatform_service_processing 的工单操作 stub（16 个）：work_id_processing（处理工单）、work_v2_id_terminate（终止）、work_v2_id_retract（撤销）、work_v2_id_goback（退回）、work_v2_id_rollback（回滚）、work_v2_id_add_split（拆分）、work_v2_id_reroute（重路由）、work_id_draft（草稿删除）、work_id_manual_append_identity（人工节点追加身份）、work_id_projection（投影查询）、work_id_series_*（系列信号处理）、work_process_*（流程信息查询）、work_manual_after_processing（人工节点后处理），每个端点实现与 Java 一致的状态机变更逻辑
- R2. 补全 processplatform_service_processing 的任务操作 stub（13 个）：task_id_processing（处理任务）、task_id_urge（催办）、task_id_replace（转交）、task_id_press（加签）、task_id_expire（超时处理）、task_id_pass_expired（通过超时任务）、task_v2_id_pause/reset/resume（挂起/重置/恢复）、task_v3_id_add（v3 添加任务）、taskcompleted_*（任务完成管理），实现与 Java 一致的任务状态流转
- R3. 补全 processplatform_service_processing 的辅助操作 stub（23 个）：snap_*（快照保存/恢复/放弃）、touch_*（催办/合并/转交/延迟日志）、review_*（审批创建/初始化）、data_*（工单/任务数据查询）、record_*（操作记录查询）、attachment_*（附件复制/编辑/查询）、job_*（待办任务投影）、applicationdict_*（字典层级查询）、documentversion_*、event_add_update_table、form_suitable_activity、readcompleted_*、service_work_id_touch
- R4. 补全 processplatform_assemble_designer 的核心管理 stub（32 个）：process_id（流程基本信息）、process_id_enable/disable（流程启停）、process_id_permission（权限控制）、process_id_list_element（元素列表）、process_id_execute_projection（执行投影）、process_id_upgrade（流程升级）、process_form_formId（关联表单）、form_*（表单 CRUD 和版本）、script_*（脚本管理）、processversion_*（流程版本）、mapping_*（映射管理）、item_access_*（访问控制）、application_*（应用管理）、applicationcategory_*、applicationdict_*。所有端点使用参数化查询，creator 从 session 上下文获取。
- R5. 补全 processplatform_assemble_designer 的边缘功能 stub（20 个）：elementtool_*（孤儿元素检测）、file_*（流程文件管理）、mergeitemplan_*（合并计划）、output_*、input_*、templateform_*、workcompleted_*、id_count、process_activity_flag

**Tier 2 — 文件管理（高优先级）**
- R6. 补全 file_assemble_control 的文件下载核心 stub（12 个）：anonymous_file_*_download（匿名下载）、attachment_*_download（附件下载）、attachment2_*_download（v2 附件下载）、file_*_download（文件下载）、folder2_*_download（文件夹下载）、share_download_*（分享下载），实现真实二进制流返回，设置正确的 Content-Type 和 Content-Disposition 响应头
- R7. 补全 file_assemble_control 的文件预览核心 stub（5 个）：attachment_*_binary_base64（Base64 预览）、attachment2_*_office_preview（Office 预览）、file_*_binary_base64（文件预览）、attachment_id_image_*（图片缩放预览）、file_id_download_image（图片尺寸下载），支持图片缩放和 Office 文档 HTML 预览
- R8. 补全 file_assemble_control 的文件上传核心 stub（5 个）：attachment_upload_*、attachment2_upload_*、file_upload_*（含 callback 变体）、file_upload_with_url（URL 上传），支持 multipart 解析、MIME 校验、分片上传回调
- R9. 补全 file_assemble_control 的回收站核心 stub（5 个）：recycle_list、recycle_empty、recycle_id_delete、recycle_id_resume、file_list_unused_referencetype_*，实现软删除恢复和物理清理
- R10. 补全 file_assemble_control 的文件列表和查询 stub（12 个）：file_list_*（分页、按 referenceType）、attachment_list_*（多种筛选）、attachment2_list_*、complex_*、folder_list_*、folder2_list_*，实现真实 DB 分页查询
- R11. 补全 file_assemble_control 的分享功能 stub（11 个）：share_list_*（我的/他人/转存）、share_download_*、share_shield_*、share_id_*（密码保护）、editor_list，实现分享链接管理
- R12. 补全 file_assemble_control 的配置和边缘 stub（4 个）：config_is_file_manager、config_system_config、folder2_batch_download、file_copy_attachment_*

**Tier 3 — BBS 社区（中优先级）**
- R13. 补全 BBS 用户认证 stub（9 个）：login（真实凭据验证 + token 签发）、logout（真实 session 注销）、user_info（真实用户信息查询）、user_forum_list、user_reply_list、user_role_list、user_section_list、user_setting、user_subject_list，返回与 Java 一致的完整用户信息
- R14. 补全 BBS 权限检查 stub（3 个）：permission_section_sectionId（真实权限校验）、permission_subject_subjectId（真实可编辑/可删除判断）、permission_subjectPublishable_sectionId，基于 RBAC 体系返回真实结果
- R15. 补全 BBS 列表查询 stub（19 个）：list_reply_filter、list_topics_creamed/recommended、list_subjects_filtered/index/recommended_index、picture_list、shutup_list、subject_*（cream/filter/index/search/statgrade）、topic_*（cream/filter/index/recommended/search）、subjectattach_list，实现真实 DB 查询和搜索
- R16. 补全 BBS 帖子操作 stub（9 个）：delete_forum/reply/subject（真实软删除）、shutup_create/delete（禁言管理）、subject_filter_list、topic_filter_list、subject_statgrade（统计等级），实现真实的帖子管理和数据统计

**Tier 4 — 组织控制（中优先级）**
- R17. 补全 organization_assemble_control 的 group 成员管理 stub（10 个）：group_list_flag_sub_direct/nested、group_list_flag_sup_direct/nested（组织树查询）、group_flag_add_member/delete_member（成员增删）、group_flag_mockdeletetoget/mockputtopost，实现真实的成员管理和组织树遍历
- R18. 补全 organization_assemble_control 的 CRUD stub（12 个）：identity_flag_*（身份 CRUD）、personcard_listpaging_*（真实 DB 分页查询）、personcard_flag_*（人员卡增删改）、permissionsetting_flag_*（权限设置 CRUD）、unitattribute_flag_*（单位属性 CRUD）、personattribute_flag_*（人员属性 CRUD），实现与 Java 一致的完整 CRUD

**Tier 5 — 安全与质量修复（并行）**
- R19. 修复 query_core_express execute_query：从执行固定 SQL 改为真正执行用户传入的动态查询（含参数绑定）。安全约束：仅允许 SELECT 语句，拒绝 INSERT/UPDATE/DELETE/DROP/DDL；最大结果行数 500 行；查询超时上限 5 秒；权限过滤（person/identityList/unitList）从 Session 上下文注入而非请求体。需实现 sqlparser-rs 进行 SQL 解析和 DML 保护。
- R20. 修复 console get_system_info：从硬编码假数据改为读取真实系统指标（OS、CPU 核数、内存、磁盘），使用 sysinfo 或等效 crate
- R21. 修复 console execute_command：从仅记录日志改为执行预定义安全命令白名单内的命令（通过 std::process::Command），返回真实输出和退出码。白名单条目：uname、df、free、ps、uptime（只读命令）；禁止 ;|&`$() 等 shell 元字符。RBAC 权限提升至 Admin 并登记到 PermissionRegistry。命令输出须做脱敏处理。
- R22. 修复 cms_express 权限体系：添加 CmsPermissionService 等效的权限过滤，支持全员/管理员/组织/群组/角色多维权限控制，区分查看权限和发布权限。默认拒绝策略：未命中任何规则时拒绝访问。
- R23. 系统审计 processplatform_assemble_designer SQL 注入：扫描全部 52 个端点（R4+R5 覆盖范围）的 SQL 查询，将所有字符串拼接改为参数化查询。create_flow 的 creator 从硬编码 "system" 改为从 session 上下文获取（此修复作为 R4 实现的子任务，不单独列为 Tier 5 任务）。

---

## Acceptance Examples

- AE1. **Covers R1, R2, R3.** Given 一个 pending 状态的工单，当调用工单处理端点时，工单状态变为 processing 并创建对应任务；当调用任务催办端点时，任务催办记录写入数据库；当调用快照保存端点时，当前工单快照持久化到 snap 表。
- AE2. **Covers R4, R5.** Given 一个已创建的流程定义，当调用流程启停端点时，流程状态正确更新；当调用流程元素列表端点时，返回流程的节点和边数据；当调用表单列表端点时，返回该应用下所有表单。
- AE3. **Covers R6, R7, R8, R9.** Given 一个已上传的附件，当调用匿名下载端点时，返回正确的二进制流和 Content-Disposition 头；当调用 Base64 预览端点时，返回 Base64 编码的预览内容；当调用回收站恢复端点时，文件从回收站恢复到原位置。
- AE4. **Covers R13, R14, R15, R16.** Given 一个有效凭据，当调用 BBS login 端点时，返回包含真实 user_id 和 token 的响应；当调用权限检查端点时，返回基于当前用户角色的真实权限结果；当调用帖子搜索端点时，返回匹配的帖子列表；当调用 delete_subject 端点时，帖子被软删除。
- AE5. **Covers R17, R18.** Given 一个组织单位，当调用 group sub_nested 查询时，返回真实的子组层级数据；当调用 personcard 创建端点时，人员卡记录写入数据库。
- AE6. **Covers R19.** Given 一个包含动态 SQL 的查询请求，当调用 execute_query 端点时，实际执行用户传入的 SQL 并返回查询结果，而非固定的占位查询。
- AE7. **Covers R20, R21.** Given 调用 console get_system_info 端点，返回真实的操作系统类型、CPU 核数、内存大小；Given 调用 console execute_command 端点（白名单内命令），返回真实命令的输出和退出码。
- AE8. **Covers R22.** Given 一个普通用户访问 CMS 内容列表，返回结果仅包含该用户有权限查看的栏目内容；Given 管理员访问，返回全部栏目内容。
- AE9. **Covers R23.** Given 调用 processplatform_assemble_designer list 端点，SQL 查询使用参数化绑定，无注入风险；creator 字段来自当前认证用户而非硬编码"system"。

---

## Success Criteria

- `cargo check --workspace` 通过，无新增编译错误
- `cargo test --workspace --lib` 通过，无新增失败
- Tier 1（流程引擎）：processplatform_service_processing 和 processplatform_assemble_designer 的 stub 数量从 144 个降至 0 个（全部端点有真实 DB 操作）
- Tier 2（文件管理）：file_assemble_control 的 stub 数量从 79 个降至 0 个，所有下载端点返回正确的 Content-Type 和 Content-Disposition 头
- Tier 3（BBS）：bbs_assemble_control 的 stub 数量从 39 个降至 0 个
- Tier 4（组织控制）：organization_assemble_control 的 stub 数量从 22 个降至 0 个，personcard 分页查询返回真实 DB 分页数据
- Tier 5（安全/质量）：query_core_express execute_query 执行动态 SQL 且有安全约束、console 返回真实系统信息和执行白名单命令、cms_express 有权限过滤、processplatform_assemble_designer 全部端点无 SQL 注入风险
- 业务逻辑验收：AE1-AE9 全部通过，包括工单状态变更、任务催办记录、文件下载流、BBS 登录认证、组织树查询等端到端验证
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 中不再有 stub 标记（除明确规划后续实现的新功能外）

---

## Scope Boundaries

- 仅补全 oa4rust 中已有的 stub 端点，不新增 Java 侧不存在的新功能
- 补全深度为"业务逻辑对齐"：实现与 Java o2server 一致的状态机、权限、查询逻辑，而非仅让端点返回非空数据
- 前端 o2web 的代码修改不在范围内
- Java o2server 的代码修改不在范围内
- 缓存层（Java CacheManager）暂不迁移
- 定时任务/批处理框架的完整 Rust 重写不在范围内

### Deferred for later

- 多级递归组织导航（unit sub-nested/sup-nested 全量递归的复杂场景）
- LDAP 用户自动同步和增量更新（已在 2026-08-11 文档中部分覆盖）
- 文件实际物理存储后端（当前仅存 DB 元数据，真实文件存储为后续工作）
- Office 文档预览的完整渲染引擎（先实现 Base64 预览，HTML 渲染为后续工作）
- BBS 图片附件的完整上传存储（先实现 DB 记录，文件存储为后续工作）
- SQLx 完全移除（SeaORM 为默认路径，复杂查询可保留 SQLx 并存）

### Outside this product's identity

- 前端 o2web 的重写或现代化改造
- 独立的 OAuth 提供商 SDK 发布
- Java 服务的永久下线决策
- 微服务拆分

---

## Key Decisions

- **分层优先级补全**：流程引擎是 o2web 接入的最大阻塞点（93.9% stub），必须最先完成；文件管理次之（86.8% stub）；BBS 和组织控制为中等优先级；安全与质量修复可并行进行
- **文件存储后端决策**：Tier 2 文件下载/上传暂使用 base64 BLOB 存 PostgreSQL，物理文件系统存储 deferred 到后续阶段
- **并发控制策略**：Tier 1 流程引擎状态机操作需使用 SELECT FOR UPDATE 或乐观锁（version 字段）防止并发状态不一致，此决策在 Planning 前确定
- **深度业务逻辑对齐而非仅填 stub**：用户明确要求"在所有功能和业务逻辑上都能完全代替 o2server"，因此每个 stub 的补全必须实现与 Java 一致的业务逻辑（状态机、权限校验、复杂查询），而非仅返回非空数据
- **复用现有认证和权限体系**：BBS 和用户认证复用 auth crate 的 SessionManager 和 RBAC 中间件，不引入新的认证机制
- **文件下载复用现有 file crate 能力**：file_assemble_control 的文件下载/预览复用 file crate 中已实现的 upload_file_record 和 MIME 校验逻辑
- **动态 SQL 执行增加安全护栏**：query_core_express 的 execute_query 实现动态 SQL 执行时，必须保留 Java 端的 DML 保护检查和权限过滤注入，不能简单拼接用户输入
- **console 命令执行增加安全限制**：execute_command 仅允许预定义的安全命令白名单，不开放任意命令执行

---

## Dependencies / Assumptions

- Java o2server 的各模块 JaxrsFilter 和 Action 类可作为 Rust 实现的业务逻辑参考契约
- 前端 o2web 对 `ActionResult<T>` 的 9 字段结构有隐式依赖，新增端点必须保持兼容
- processplatform_service_processing 的工单/任务状态机依赖 x_work、x_task、x_review、x_record、x_snap 等表已存在且 schema 正确
- file_assemble_control 的文件下载需要文件系统或对象存储后端支持（当前阶段可先用 base64 BLOB 代替）
- query_core_express 的动态 SQL 执行需要 SQL 解析器（如 sqlparser-rs）进行安全和预处理
- console 真实系统指标需要 sysinfo crate 支持跨平台（Windows/Linux/macOS）
- 所有补全必须通过 `cargo test --workspace --lib` 验证无回归

---

## Outstanding Questions

### Resolve Before Planning

- [Affects R1-R5][Technical] processplatform_service_processing 的工单状态机需要完整的 x_work/x_task/x_review/x_snap/x_record 表 schema 对齐——当前 migrations/ 中无流程平台相关表，需新增 migration
- [Affects R6-R12][Technical] 文件下载的物理存储后端选择：继续使用 base64 BLOB 存 PostgreSQL，还是引入文件系统/对象存储？tier 2 实现前需明确此决策，影响 file_assemble_control 的设计
- [Affects R13-R16][Technical] BBS 表名映射：migration 创建 `bbs_forum_info` 等，代码查询 `x_bbs_forum` 等，需统一表名
- [Affects R17-R18][Technical] 组织控制表 schema：x_org_group/x_org_identity/x_org_person/x_org_role/x_org_duty 等表的 migration 是否存在需确认
- [Affects R19][Technical] query_core_express 动态 SQL 执行的参数绑定策略：使用 sqlparser-rs 解析 + 参数化执行，还是沿用 SQLx 的 bind 机制？
- [Affects R21][Technical] console execute_command 安全白名单的具体条目和 RBAC 权限级别需明确

### Deferred to Planning

- [Affects R1-R5][Needs research] 流程引擎状态机的并发控制策略：Java 端使用 KeyLock 机制防止并发操作，Rust 端如何实现等效的乐观锁/分布式锁
- [Affects R3][Needs research] snap（快照）操作的完整事务边界：快照保存/恢复涉及多表（x_work/x_task/x_snap），需确认事务范围
- [Affects R13][Needs research] BBS 搜索功能的全文检索实现：Java 端使用 Lucene，Rust 端是否引入 tantivy 或使用 PG 原生全文检索
- [Affects R17][Needs research] 组织树查询（sub_direct/nested/sup_direct/nested）的递归 CTE 实现：PostgreSQL 支持递归查询，但需确认性能边界
- [Affects R19][Needs research] dynamic SQL 执行的 SQL 语法支持范围：是否支持 JPQL 风格查询（Java 端），还是仅支持标准 SQL？
