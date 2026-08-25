---
title: OA4Rust 核心模块 stub 补全 — 流程引擎/文件/BBS/组织/安全
type: feat
status: completed
date: 2026-08-11
origin: docs/brainstorms/2026-08-11-oa4rust-full-gap-closure-comprehensive-requirements.md
---

# OA4Rust 核心模块 stub 补全 — 流程引擎/文件/BBS/组织/安全

## Summary

在已有认证/个人/批量查询补全计划（2026-08-11-001，status: completed）基础上，针对 oa4rust 剩余的核心功能缺口编写实施计划：优先修复流程引擎（144 个 stub）、文件管理（79 个 stub）、BBS（39 个 stub）、组织控制（22 个 stub）的 stub 实现，同时修复 query 动态 SQL 执行、console 假数据、CMS 权限、SQL 注入等安全问题。计划分五个阶段执行，第一阶段优先创建缺失的数据库 migration。

---

## Problem Frame

oa4rust 已完成 86 个 crate 的路由挂载，但核心业务模块存在大量 stub 实现：流程引擎的工单状态机（92 stub）、文件下载的 binary stream（79 stub）、BBS 的用户认证（39 stub）、组织控制的 CRUD（22 stub）均返回假数据。更严重的是，流程引擎所需的 x_work/x_task/x_review/x_snap/x_record 表在 migrations 中完全缺失，BBS 的表名与代码不匹配。这些 gap 导致 o2web 前端无法完成实际业务流程联调。

---

## Requirements

- R1-R5. 补全 processplatform_service_processing 的工单/任务/辅助操作 stub（52 个）
- R4-R5. 补全 processplatform_assemble_designer 的核心/边缘管理 stub（52 个）
- R6-R12. 补全 file_assemble_control 的下载/预览/上传/回收站/列表/分享/配置 stub（54 个）
- R13-R16. 补全 BBS 的认证/权限/列表/帖子操作 stub（39 个）
- R17-R18. 补全 organization_assemble_control 的 group 成员管理/CRUD stub（22 个）
- R19. 修复 query_core_express execute_query 动态 SQL 执行（含安全约束）
- R20-R21. 修复 console get_system_info/execute_command
- R22. 修复 cms_express 权限体系
- R23. 修复 processplatform_assemble_designer SQL 注入 + creator 硬编码
- R16（合并后）. 修复 bbs_assemble_control delete_forum/reply/subject

---

## Scope Boundaries

- 仅补全已有 stub 端点，不新增 Java 侧不存在的新功能
- 补全深度为"业务逻辑对齐"：实现与 Java 一致的状态机、权限、查询逻辑
- 前端 o2web 代码修改不在范围内
- Java o2server 代码修改不在范围内
- 文件物理存储后端：Tier 2 暂使用 base64 BLOB 存 PostgreSQL
- 流程引擎并发控制：使用 SELECT FOR UPDATE 行级锁

### Deferred to Follow-Up Work

- 多级递归组织导航（unit sub-nested/sup-nested 全量递归）
- LDAP 用户自动同步和增量更新
- 文件实际物理存储后端（文件系统/对象存储）
- Office 文档预览的完整 HTML 渲染引擎（先 Base64，后 HTML）
- BBS 图片附件的完整文件存储
- SQLx 完全移除
- processplatform_assemble_bam 模块补全（17 路由已标记 done，待验证）

---

## Context & Research

### 已有完成计划（不重复实现）

- `docs/plans/2026-08-11-001-feat-oa4rust-full-gap-closure-plan.md`（status: completed）
- 覆盖：认证安全检查、双因素登录、安全注销、Token 校验、switch_user、电子签名、头像、用户注册、LDAP 集成、批量查询、授权管理 CRUD、null 桩修复
- **本计划不覆盖上述模块**

### 关键代码模式

- **路由注册模式**：`crates/<crate>/src/lib.rs` 中定义 `pub fn router(pool: Pool) -> Router`，在 `src/main.rs` 中 `.merge()`
- **数据库访问模式**：`deadpool_postgres::Pool` + `client.query()` / `client.query_one()` 参数化查询
- **响应模式**：`ActionResult<T>` 9 字段结构（data/type/message/date/spent/size/count/position/prompt）
- **权限模式**：`shared::middleware::rbac` 中 `PermissionRegistry` + `with_defaults()` 注册
- **测试模式**：`crates/<crate>/src/tests.rs` 集成测试

### 已确认的 schema 缺口

- **流程引擎表完全缺失**：x_work、x_task、x_review、x_snap、x_record、x_workcompleted、x_draft、x_read、x_readcompleted、x_attachment、x_document_version、x_job 等表在 migrations/ 中不存在
- **BBS 表名不匹配**：migration 创建 `bbs_forum_info`/`bbs_section_info`/`bbs_subject_info`/`bbs_comment_info`，代码查询 `x_bbs_forum`/`x_bbs_topic`/`x_bbs_reply`
- **组织控制表缺失**：x_org_group、x_org_identity、x_org_person、x_org_role、x_org_duty、x_org_permission_setting、x_org_unit_attribute、x_org_person_attribute 不存在
- **文件表名不一致**：migration 用 `FILE_FOLDER`/`FILE_FILE`（大写），代码用 `x_file`/`x_file_assemble_control_*`

---

## Key Technical Decisions

- **文件存储后端决策**：Tier 2 文件下载/上传暂使用 base64 BLOB 存 PostgreSQL，物理文件系统存储 deferred
- **并发控制策略**：Tier 1 流程引擎使用 SELECT FOR UPDATE 行级锁，在状态变更前锁定 x_work 行
- **BBS 表名统一**：修复 migration 007 使用代码实际查询的表名（x_bbs_forum 等），或修改代码使用 migration 的表名
- **组织控制表命名**：参照 auth 模块模式使用 auth_* 前缀，或新建 x_org_* 表
- **query execute_query 安全约束**：仅允许 SELECT，拒绝 DML；最大 500 行；5 秒超时；权限从 Session 注入
- **console execute_command 白名单**：仅允许 uname、df、free、ps、uptime（只读命令）；禁止 shell 元字符
- **CMS 权限默认拒绝**：未命中任何规则时拒绝访问

---

## Implementation Units

### U1. 流程引擎数据库 schema 创建

**Goal:** 创建 processplatform 所需的数据库表，为 Tier 1 stub 补全提供数据基础

**Requirements:** R1, R2, R3

**Dependencies:** None（但这是整个 Tier 1 的前置条件）

**Files:**
- Create: `migrations/020_create_processplatform_tables.sql`
- Create: `migrations/020_create_processplatform_tables_rollback.sql`

**Approach:**
- 参考 Java o2server `x_processplatform_core_entity` 模块的实体定义
- 创建核心表：`x_work`（工单）、`x_task`（任务）、`x_review`（审批）、`x_snap`（快照）、`x_record`（操作日志）、`x_workcompleted`（完成工单）、`x_draft`（草稿）、`x_read`（阅读记录）、`x_readcompleted`（已读完成）、`x_attachment`（附件）、`x_document_version`（文档版本）、`x_job`（待办任务）
- 参考现有 migration 文件的 SQL 风格（CREATE TABLE IF NOT EXISTS，使用 IF NOT EXISTS 保证幂等）
- 每张表包含标准字段：id, create_time, update_time, deleted_at（软删除）
- x_work 表需要 work_status 字段支持状态机（pending/processing/completed/cancelled 等）
- x_task 表需要 task_status、activity_token、person 等字段
- 创建 rollback 脚本

**Patterns to follow:**
- `migrations/001_create_auth_tables.sql`（参考 SQL 风格）
- `migrations/006_meeting_tables.sql`（参考表结构设计）

**Test scenarios:**
- Happy path: 执行 migration 后，所有 processplatform 表创建成功
- Edge case: 重复执行 migration（幂等性）→ 不报错
- Edge case: 执行 rollback 后，表被删除

**Verification:**
- `psql < DATABASE_URL` 执行 migration 后，`\dt` 确认所有表存在
- `cargo check --workspace` 通过

---

### U2. BBS 表名修复

**Goal:** 统一 BBS 数据库表名，消除 migration 与代码查询的不匹配

**Requirements:** R13, R14, R15, R16

**Dependencies:** U1（BBS 模块不需要流程引擎表，但与 U1 并行执行）

**Files:**
- Modify: `migrations/007_bbs_tables.sql`（将表名从 bbs_*_info 改为 x_bbs_*）
- Modify: `crates/bbs_assemble_control/src/lib.rs`（如有硬编码表名需对齐）

**Approach:**
- 将 migration 007 中的 `bbs_forum_info` → `x_bbs_forum`，`bbs_section_info` → `x_bbs_section`，`bbs_subject_info` → `x_bbs_subject`，`bbs_comment_info` → `x_bbs_reply`
- 确认代码中所有查询使用统一的表名
- 创建新 migration 而非修改已有 migration（保持 migration 历史完整）：创建 `migrations/021_fix_bbs_table_names.sql`
- 在 021 中：ALTER TABLE bbs_forum_info RENAME TO x_bbs_forum（如果表存在），否则 CREATE TABLE x_bbs_forum

**Patterns to follow:**
- `migrations/011_normalize_schema.sql`（参考 schema 规范化模式）

**Test scenarios:**
- Happy path: migration 执行后，BBS 查询返回真实数据
- Edge case: 表不存在时（新安装）→ CREATE TABLE 正常执行
- Error path: migration 重复执行 → 幂等，不报错

**Verification:**
- `psql` 执行 migration 后，`SELECT * FROM x_bbs_forum LIMIT 1` 不报错
- `cargo test -p bbs_assemble_control` 通过

---

### U3. 组织控制数据库 schema 创建

**Goal:** 创建 organization_assemble_control 所需的数据库表

**Requirements:** R17, R18

**Dependencies:** None

**Files:**
- Create: `migrations/022_create_org_assemble_tables.sql`
- Create: `migrations/022_create_org_assemble_tables_rollback.sql`

**Approach:**
- 创建组织控制所需表：`x_org_group`（用户组）、`x_org_identity`（身份）、`x_org_person`（人员卡）、`x_org_role`（角色）、`x_org_duty`（职务）、`x_org_permission_setting`（权限设置）、`x_org_unit_attribute`（单位属性）、`x_org_person_attribute`（人员属性）
- 每张表包含标准字段：id, create_time, update_time, deleted_at
- x_org_person 表需要与 auth_person 表的关联（person_unique 字段）
- x_org_group 表需要 parent_id 支持层级结构

**Patterns to follow:**
- `migrations/001_create_auth_tables.sql`
- `migrations/005_org_tables.sql`

**Test scenarios:**
- Happy path: migration 执行后，所有组织控制表创建成功
- Edge case: 重复执行 → 幂等

**Verification:**
- `psql` 执行 migration 后，所有 x_org_* 表存在
- `cargo check --workspace` 通过

---

### U4. 文件管理端点注册与 stub 真实化（Tier 2）

**Goal:** 将 file_assemble_control 中未注册的端点注册到路由，并将 stub 替换为真实 DB 操作

**Requirements:** R6, R7, R8, R9, R10, R11, R12

**Dependencies:** U1（文件模块依赖 x_file 相关表，需确认表存在）

**Files:**
- Modify: `crates/file_assemble_control/src/routes.rs`（补充缺失端点注册）
- Modify: `crates/file_assemble_control/src/lib.rs`（将 stub 函数替换为真实实现）
- Test: `crates/file_assemble_control/src/tests.rs`

**Approach:**
- 分析 Java `x_file_assemble_control` 的 JaxrsFilter 文件，确认完整端点列表
- 在 routes.rs 中注册所有缺失端点（anonymous_download、attachment_download、share_list、recycle_list 等）
- 将 lib.rs 中的 stub 函数（无 pool 参数、返回 {"success": true}）替换为真实 DB 操作
- 文件下载：从 FILE_FILE 表读取 base64 内容，设置 Content-Type 和 Content-Disposition
- 文件上传：复用 file crate 的 upload_file_record 逻辑
- 回收站：x_file.deleted_at 软删除
- 分享功能：创建 x_file_share 关联表（如不存在需创建 migration）

**Patterns to follow:**
- `crates/file/src/lib.rs`（已有真实实现，复用 upload_file_record）
- `crates/file_core_entity/src/lib.rs`（参考文件 CRUD 模式）

**Test scenarios:**
- Happy path: POST /jaxrs/file/upload → 文件元数据写入 DB
- Happy path: GET /jaxrs/file/{id}/download → 返回 base64 编码的文件内容 + 正确 Content-Type
- Happy path: POST /jaxrs/recycle/resume/{id} → 文件从回收站恢复
- Edge case: 下载不存在的文件 → 返回 404
- Error path: 上传超大文件（>5MB）→ 返回 error

**Verification:**
- `cargo test -p file_assemble_control` 通过
- file_assemble_control 中不再有 stub 函数（无 pool 参数的函数）

---

### U5. 流程引擎工单/任务操作补全（Tier 1 — service_processing）

**Goal:** 将 processplatform_service_processing 中的 92 个 stub 替换为真实业务逻辑

**Requirements:** R1, R2, R3

**Dependencies:** U1（processplatform 表必须存在）

**Files:**
- Modify: `crates/processplatform_service_processing/src/lib.rs`
- Modify: `crates/processplatform_service_processing/src/routes.rs`
- Test: `crates/processplatform_service_processing/src/tests.rs`

**Approach:**
- **工单操作（16 个 stub）**：work_id_processing 等实现 x_work 状态变更 + x_task 创建
  - 参考 Java `WorkJaxrsFilter` 的行为契约
  - 关键状态流转：pending → processing → completed
  - 退回（goback）：将当前 task 标记完成，创建新的待处理 task
  - 撤销（retract）：终止整个工单，清理关联 task
  - 终止（terminate）：立即结束工单
- **任务操作（13 个 stub）**：task_id_processing 等实现任务状态流转
  - 催办（urge）：写入 x_record 催办日志
  - 转交（replace）：更新 task.person 字段
  - 加签（press）：创建并行子任务
- **辅助操作（23 个 stub）**：snap/touch/review/data/record/attachment 等
  - 快照（snap）：保存 x_work 当前状态到 x_snap
  - 审批（review）：创建 x_review 记录
  - 记录（record）：写入 x_record 操作日志
- 所有状态变更操作使用 `SELECT ... FOR UPDATE` 防止并发竞态
- 所有端点使用参数化查询防止 SQL 注入

**Patterns to follow:**
- `crates/processplatform_service_processing/src/lib.rs` 中已有的 6 个真实实现（get_process, create_process, list_processes, execute_process, get_process_instance, cancel_process_instance）
- Java `x_processplatform_service_processing` 的 Action 类作为行为参考

**Test scenarios:**
- Happy path: work_id_processing → x_work.work_status 变为 processing，x_task 记录创建（AE1）
- Happy path: task_id_urge → x_record 写入催办日志
- Happy path: snap_upload → x_snap 写入快照
- Edge case: 并发处理同一工单 → 第二个请求因 FOR UPDATE 等待，不产生脏数据
- Error path: 处理已完成的工单 → 返回错误（状态不允许）

**Verification:**
- `cargo test -p processplatform_service_processing` 通过
- 所有 92 个 stub 函数替换为有 pool 参数的真实实现
- 无 SQL 字符串拼接

---

### U6. 流程设计器核心管理补全（Tier 1 — assemble_designer）

**Goal:** 将 processplatform_assemble_designer 中的 52 个 stub 替换为真实业务逻辑

**Requirements:** R4, R5, R23

**Dependencies:** U1

**Files:**
- Modify: `crates/processplatform_assemble_designer/src/lib.rs`
- Modify: `crates/processplatform_assemble_designer/src/routes.rs`
- Test: `crates/processplatform_assemble_designer/src/tests.rs`

**Approach:**
- **流程核心管理（22 个 stub）**：process_id、process_id_enable/disable、process_id_permission 等
  - process_id：查询 x_process_definition 返回流程基本信息
  - process_id_enable/disable：更新 x_process_definition.status
  - process_id_list_element：返回流程的节点和边 JSON
- **表单/脚本管理（20 个 stub）**：form_*、script_*、processversion_*
  - 复用已有的 SeaORM entity 定义
- **边缘功能（10 个 stub）**：elementtool_*、file_*、mapping_*、output_* 等
- **SQL 注入修复**：list_flows 中的 format! 字符串拼接改为参数化查询
- **creator 修复**：create_flow 从 session 上下文获取 creator，而非硬编码 "system"

**Patterns to follow:**
- `crates/processplatform_assemble_designer/src/lib.rs` 中已有的 6 个真实实现
- `crates/orm/src/` 中的 SeaORM entity 模式

**Test scenarios:**
- Happy path: process_id → 返回流程基本信息（AE2）
- Happy path: process_id_enable → x_process_definition.status 更新为 enabled
- Happy path: form_list → 返回该应用下的所有表单
- Happy path: create_flow → creator 为当前登录用户而非 "system"（AE9）
- Edge case: SQL 注入测试 → category = "'; DROP TABLE x_process_definition; --" → 无影响

**Verification:**
- `cargo test -p processplatform_assemble_designer` 通过
- 无 SQL 字符串拼接

---

### U7. BBS 模块 stub 真实化（Tier 3）

**Goal:** 将 bbs_assemble_control 中的 39 个 stub 替换为真实业务逻辑

**Requirements:** R13, R14, R15, R16

**Dependencies:** U2（BBS 表名修复必须先完成）

**Files:**
- Modify: `crates/bbs_assemble_control/src/lib.rs`
- Test: `crates/bbs_assemble_control/src/tests.rs`

**Approach:**
- **用户认证（9 个 stub）**：login 实现真实凭据验证（复用 auth crate 的密码校验逻辑），返回 token + 用户信息
  - R13 中 login 需集成 auth crate 的 SessionManager
  - logout 需调用 SessionManager 移除 session
- **权限检查（3 个 stub）**：permission_* 实现真实 RBAC 校验
  - 复用 shared::middleware::rbac 的权限检查逻辑
- **列表查询（19 个 stub）**：实现真实 DB 查询
  - list_topics_creamed → WHERE is_top = true
  - subject_search → 使用 PostgreSQL 全文检索或 LIKE 查询
  - shutup_list → 查询被禁言用户列表
- **帖子操作（8 个 stub）**：delete_forum/reply/subject 实现真实软删除

**Patterns to follow:**
- `crates/auth/src/lib.rs` 的 login 逻辑（参考密码校验）
- `crates/shared/src/middleware/rbac.rs` 的权限检查

**Test scenarios:**
- Happy path: BBS login 使用有效凭据 → 返回 token + user_id（AE4）
- Happy path: permission_subject_subjectId → 返回基于角色的真实权限
- Happy path: subject_search → 返回匹配的帖子列表
- Happy path: delete_subject → x_bbs_subject.deleted_at 更新

**Verification:**
- `cargo test -p bbs_assemble_control` 通过
- bbs_assemble_control 中不再有 stub 函数

---

### U8. 组织控制模块 stub 真实化（Tier 4）

**Goal:** 将 organization_assemble_control 中的 22 个 stub 替换为真实业务逻辑

**Requirements:** R17, R18

**Dependencies:** U3（组织控制表必须存在）

**Files:**
- Modify: `crates/organization_assemble_control/src/lib.rs`
- Test: `crates/organization_assemble_control/src/tests.rs`

**Approach:**
- **group 成员管理（10 个 stub）**：
  - group_list_flag_sub_direct/nested：递归查询 x_org_group 的父子关系
  - group_flag_add_member/delete_member：操作 x_org_group_member 关联表
- **身份 CRUD（3 个 stub）**：identity_flag_* 实现真实 CRUD
- **人员卡 CRUD（6 个 stub）**：personcard_* 实现真实分页查询和 CRUD
  - personcard_listpaging：使用 LIMIT/OFFSET 分页
- **权限设置 CRUD（3 个 stub）**：permissionsetting_flag_* 实现真实 CRUD
- **单位属性 CRUD（3 个 stub）**：unitattribute_flag_* 实现真实 CRUD

**Patterns to follow:**
- `crates/control/src/lib.rs` 中已有的 person/group/role/unit CRUD（参考实现模式）
- `crates/organization_core_entity/src/lib.rs` 中的 entity 定义

**Test scenarios:**
- Happy path: group_list_flag_sub_nested → 返回真实的子组层级数据（AE5）
- Happy path: personcard_flag（创建）→ x_org_person 写入记录
- Happy path: identity_flag（CRUD）→ 真实数据库操作
- Edge case: 分页查询 → 返回 total_count + data

**Verification:**
- `cargo test -p organization_assemble_control` 通过
- organization_assemble_control 中不再有 stub 函数

---

### U9. 质量/安全修复（Tier 5）

**Goal:** 修复 query 动态 SQL、console 假数据、CMS 权限、SQL 注入等安全质量问题

**Requirements:** R19, R20, R21, R22, R23（部分）

**Dependencies:** None（这些修复独立于其他 unit）

**Files:**
- Modify: `crates/query_core_express/src/lib.rs`
- Modify: `crates/console/src/lib.rs`
- Modify: `crates/cms_express/src/lib.rs`
- Test: `crates/query_core_express/src/tests.rs`
- Test: `crates/console/src/tests.rs`
- Test: `crates/cms_express/src/tests.rs`

**Approach:**
- **R19 execute_query 动态 SQL**：
  - 使用 sqlparser-rs 解析用户输入的 SQL
  - 仅允许 SELECT 语句，拒绝 DML
  - 从 Session 上下文注入 person/identityList/unitList 权限过滤
  - 设置 500 行结果上限和 5 秒超时
- **R20 console get_system_info**：
  - 使用 sysinfo crate 读取真实系统指标
  - 支持 Windows/Linux/macOS 跨平台
- **R21 console execute_command**：
  - 仅允许白名单命令（uname、df、free、ps、uptime）
  - 使用 std::process::Command 执行
  - RBAC 权限提升至 Admin
  - 输出脱敏
- **R22 CMS 权限**：
  - 添加 CmsPermissionService 等效的权限过滤
  - 默认拒绝策略
  - 区分查看权限和发布权限
- **R23 SQL 注入修复**：已在 U6 中完成

**Patterns to follow:**
- `crates/query_service_processing/src/lib.rs`（已有查询处理逻辑参考）
- `crates/shared/src/middleware/rbac.rs`（权限检查）
- `crates/shared/src/session.rs`（SessionManager）

**Test scenarios:**
- Happy path: execute_query 发送 SELECT → 返回查询结果（AE6）
- Error path: execute_query 发送 INSERT/DELETE → 拒绝
- Happy path: console get_system_info → 返回真实 OS/CPU/内存信息（AE7）
- Happy path: console execute_command 发送白名单命令 → 返回真实输出（AE7）
- Error path: console execute_command 发送非白名单命令 → 返回 403
- Happy path: CMS 普通用户访问 → 仅返回有权限的内容（AE8）

**Verification:**
- `cargo test -p query_core_express` 通过
- `cargo test -p console` 通过
- `cargo test -p cms_express` 通过
- 无 SQL 注入漏洞（渗透测试）

---

## System-Wide Impact

- **Interaction graph：** U4 修改 file_assemble_control，可能影响 file crate 的 upload_file_record 复用；U5/U6 修改 processplatform 模块，影响流程引擎整体；U7 修改 BBS 模块，影响论坛功能；U8 修改组织控制模块，影响组织架构管理；U9 修改 query/console/cms 模块，影响系统管理功能
- **Error propagation：** 所有新增端点通过 AppError 统一错误处理，中间件层自动转换为 HTTP 状态码
- **State lifecycle risks：** 流程引擎的状态变更需确保事务完整性（x_work + x_task + x_record 同步更新）
- **API surface parity：** 新增端点保持 ActionResult<T> 9 字段结构
- **Integration coverage：** U5/U7 的行为对比测试需验证与 Java o2server 的响应一致性
- **Unchanged invariants：** ActionResult<T> 9 字段结构不变；PermissionRegistry 扩展；SessionManager 不变

---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| processplatform 表 schema 与 Java 不完全一致 | High | High | 对照 Java entity 类逐一确认字段，创建 migration 时包含完整字段 |
| BBS 表名修复可能影响已有数据 | Medium | Medium | 使用 ALTER TABLE RENAME 而非 DELETE+CREATE，保留已有数据 |
| 文件下载 base64 BLOB 性能问题 | Medium | Medium | Tier 2 暂用 BLOB，后续迁移到文件系统时通过 migration 迁移数据 |
| 并发工单操作的行级锁竞争 | Medium | Medium | SELECT FOR UPDATE 已足够；高并发场景可考虑乐观锁（version 字段） |
| query execute_query 动态 SQL 的安全边界 | High | High | sqlparser-rs 解析 + DML 拒绝 + 权限注入 + 超时限制，多层防护 |
| CMS 权限模型实现复杂度 | Medium | High | 先实现基础权限过滤（全员/管理员/组织），后续逐步增强 |

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-11-oa4rust-full-gap-closure-comprehensive-requirements.md](docs/brainstorms/2026-08-11-oa4rust-full-gap-closure-comprehensive-requirements.md)
- **Related completed plan:** [docs/plans/2026-08-11-001-feat-oa4rust-full-gap-closure-plan.md](docs/plans/2026-08-11-001-feat-oa4rust-full-gap-closure-plan.md)（status: completed）
- **Related code:** `crates/processplatform_service_processing/`, `crates/processplatform_assemble_designer/`, `crates/file_assemble_control/`, `crates/bbs_assemble_control/`, `crates/organization_assemble_control/`, `crates/query_core_express/`, `crates/console/`, `crates/cms_express/`
- **Java reference:** `oa/o2server/x_processplatform_service_processing/`, `oa/o2server/x_file_assemble_control/`, `oa/o2server/x_bbs_assemble_control/`, `oa/o2server/x_organization_assemble_control/`

---

## 实现情况（2026-08-21 审计）

**审计基准：** 工作树 HEAD 314c7a75；判定状态：completed

### 已验证完成

- U1/U3 流程引擎与组织控制 schema 创建：migrations 目录中对应建表脚本在档（含 024 系列）
- U2 BBS 表名修复、U4 文件管理端点注册与真实化、U5 流程工单/任务操作补全、U6 流程设计器核心管理补全、U7 BBS 模块真实化、U8 组织控制模块真实化：相关 crate（processplatform_service_processing / assemble_designer / file_assemble_control / bbs_assemble_control / organization_assemble_control）均存在且配有 tests_generated.rs
- U9 质量/安全修复：query_core_express、console、cms_express 均在档
- Deferred 完成：多级递归组织导航（由 2026-08-11-004 U3 落地）

### 无法验证

- Deferred「processplatform_assemble_bam 模块补全（27 路由已标记 done，待验证）」：实测 `crates/processplatform_assemble_bam/src/*.rs` 仅 5 处 `.route(` 注册，与"27 路由"标称不符，无法证实

### 未完成 / 遗留 → 待汇入剩余工作汇总计划

- Deferred「LDAP 用户自动同步和增量更新」：ldap crate 仅 lib.rs + tests，未见同步模块
- Deferred「文件实际物理存储后端（文件系统/对象存储）」
- Deferred「Office 文档预览的完整 HTML 渲染引擎」：基础 HTML 预览已落地（preview crate），xlsx/pptx 等格式仍缺
- Deferred「BBS 图片附件的完整文件存储」
- Deferred「SQLx 完全移除」
