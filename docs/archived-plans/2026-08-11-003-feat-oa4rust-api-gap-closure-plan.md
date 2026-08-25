---
title: OA4Rust API Gap Closure — 附件下载流、查询视图执行、工作流操作、组织模块补全
type: feat
status: completed
date: 2026-08-11
origin: docs/brainstorms/2026-08-11-oa4rust-full-api-gap-closure-comprehensive-requirements.md
---

# OA4Rust API 接口对账与补全

## Summary

基于 o2web 前端调用路径对账，在已完成的 core modules stub 补全基础上，填补导致前端核心业务流程阻断的最后一批接口缺口：附件下载流路由注册、工作流处理/列表/复杂信息接口、查询视图执行路由完善、新建组织认证与个人模块、以及必要的数据库 schema 扩展。

---

## Problem Frame

oa4rust 核心模块（流程引擎、文件管理、BBS、组织控制、CMS、BAM）的 stub 已全部补全，但 o2web 前端在实际运行时仍因三类问题崩溃：一是关键下载流接口已实现但未注册到路由；二是工作流处理状态变更接口未注册；三是新模块（organization_assemble_authentication、organization_assemble_personal）完全缺失。这些问题阻断了表单打开、附件下载、任务处理等核心流程。

---

---

## Requirements

- R1. 实现 `view_flag_flag_query_queryFlag_execute` 路由注册，支持按视图标记和执行参数返回分页数据
- R2. 实现 `view_flag_flag_query_queryFlag_execute_v2_page_page_size_size` 路由注册，支持带分页参数的查询执行
- R3. 实现 `importmodel_id_execute` 路由注册，支持导入模型执行
- R4. 注册 `/jaxrs/file/{id}/download/stream` 路由，返回文件二进制流，Content-Type 正确
- R5. 注册 `attachment_id_download_stream`、`file_id_download_stream` 等下载流接口到路由
- R6. 实现批量附件下载接口 `/jaxrs/attachment/batch/download/work/{workId}/site/{site}/stream`
- R7. 实现工作流列表分页接口，支持按应用标记过滤、分页、排序
- R8. 注册工作处理状态设置接口（`work_id_processing`）到路由
- R9. 实现工作复杂信息接口（`process_id_complex`），返回关联的任务、审批、快照聚合数据
- R10. 实现查询视图表定义接口，返回指定查询视图的字段结构
- R11. 实现文档阅读计数接口，累加指定文档的阅读次数
- R12. 实现评论列表分页接口，支持分页和排序
- R13. 创建 `organization_assemble_authentication` 模块，实现人员头像、身份信息接口
- R14. 创建 `organization_assemble_personal` 模块，实现个人设置、个人角色列表接口
- R15. 实现 AI 对话补全接口
- R16. 实现门户详情接口
- R17. 实现应用详情接口
- R18. 实现身份详情接口

**Origin actors:** A1 (o2web 前端), A2 (oa4rust 后端), A3 (集成测试)
**Origin flows:** F1 (流程表单打开), F2 (CMS 文档阅读), F3 (工作流任务处理)
**Origin acceptance examples:** AE1 (查询视图执行), AE2 (文件下载流), AE3 (工作处理状态), AE4 (文档阅读计数), AE5 (头像/身份信息), AE6 (AI 对话)

---

## Scope Boundaries

- o2web 前端代码修改不在范围内
- Java o2server 代码修改不在范围内
- 安装初始化接口（secret、server、database、h2、restore）不在本次范围内
- SSO 单点登录入口不在本次范围内
- AI 对话实际接入 LLM 服务不在本次范围内（先实现接口框架）
- 文件物理存储后端迁移不在范围内

### Deferred to Follow-Up Work

- 多级递归组织导航（unit sub-nested/sup-nested 全量递归）
- Office 文档预览完整 HTML 渲染引擎
- BBS 图片附件完整文件存储后端
- AI 对话多轮上下文管理
- 程序中心分发组装接口

---

## Context & Research

### Relevant Code and Patterns

- **查询视图执行**: `crates/query_assemble_surface/src/lib.rs` 中已有 `view_flag_flag_query_queryFlag_execute` 等函数实现，需要在 `routes.rs` 中注册路由
- **文件下载流**: `crates/file_assemble_control/src/lib.rs` 中已有 `file_id_download_stream`（line 1844）返回 `Response`，需要在 routes.rs 中注册 `/download/stream` 路径
- **工作流处理**: `crates/processplatform_service_processing/src/lib.rs` 中已有 `work_id_processing`（line 223），需要在 routes.rs 中注册
- **组织模块模板**: `crates/organization_assemble_express/src/lib.rs` 可作为新模块的结构模板
- **路由注册模式**: `crates/*/src/routes.rs` 中使用 `Router::new().route(...).route(...).layer(Extension(pool))` 模式
- **Migration 模式**: `migrations/023_create_cms_assemble_tables.sql` 为最新参考

### Institutional Learnings

- 已有 solution: `docs/solutions/` 中无直接相关记录，但本次工作延续之前 core modules gap closure 的模式
- 所有新接口使用 `deadpool_postgres::Pool` + `Extension<Pool>` 模式
- 响应格式统一使用 `ActionResult<Value>` 9 字段结构

---

## Key Technical Decisions

- **路由注册优先于新增函数**: 大部分功能函数已存在但未注册到 routes.rs，优先完成注册工作
- **新模块遵循既有命名**: `organization_assemble_authentication` 和 `organization_assemble_personal` 与 `organization_assemble_express` 保持相同结构
- **AI 对话先实现框架**: 接收消息返回模拟回复，为后续接入真实 LLM 预留接口
- **Migration 延续现有风格**: 新建 migration 024，CREATE TABLE IF NOT EXISTS，包含 rollback 脚本

---

## Open Questions

### Resolved During Planning

- **前端路径 vs 后端路由对齐**: 用户确认后端适配前端，所有新路由使用 o2web 实际调用的路径
- **查询视图执行实现**: 函数已存在（query_assemble_surface/lib.rs:1414-1508），只需完善路由注册
- **文件下载流实现**: `file_id_download_stream` 已存在（file_assemble_control/lib.rs:1844），只需注册路由

### Deferred to Implementation

- 查询视图的 content 字段存储的实际 JSON 结构（需运行时验证 o2web 解析需求）
- AI 对话接口的具体请求/响应格式（先实现通用框架）

---

## Output Structure

```
oa4rust/
├── crates/
│   ├── organization_assemble_authentication/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── routes.rs
│   ├── organization_assemble_personal/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── routes.rs
│   ├── query_assemble_surface/
│   │   └── src/routes.rs          (modify — add execute routes)
│   ├── file_assemble_control/
│   │   └── src/routes.rs          (modify — add download/stream routes)
│   └── processplatform_service_processing/
│       └── src/routes.rs          (modify — add work processing routes)
├── migrations/
│   ├── 024_create_gap_closure_tables.sql
│   └── 024_create_gap_closure_tables_rollback.sql
└── src/
    └── main.rs                    (modify — register new crates)
```

---

## Implementation Units

### U1. 路由注册修复（查询视图、文件下载流、工作流处理）

**Goal:** 将已实现但未注册到路由的关键函数注册到对应 routes.rs，确保 o2web 前端能正常调用

**Requirements:** R1, R2, R3, R4, R5, R8

**Dependencies:** None（函数已存在，只需注册）

**Files:**
- Modify: `crates/query_assemble_surface/src/routes.rs`
- Modify: `crates/file_assemble_control/src/routes.rs`
- Modify: `crates/processplatform_service_processing/src/routes.rs`

**Approach:**
1. 分析 query_assemble_surface 中所有已实现的函数签名，对照 o2web 调用的路径，在 routes.rs 中注册：
   - `view_flag_flag_query_queryFlag_execute` → `/jaxrs/queryview/flag/{view}/application/flag/{app}/execute`
   - `view_flag_flag_query_queryFlag_execute_v2_page_page_size_size` → `/jaxrs/queryview/flag/{view}/application/flag/{app}/execute/page/{page}/size/{size}`
   - `importmodel_id_execute` → `/jaxrs/importmodel/id/{id}/execute`
2. 分析 file_assemble_control 中的下载流函数，注册：
   - `file_id_download_stream` → `/jaxrs/file/{id}/download/stream`
   - `attachment_id_download_stream` → `/jaxrs/attachment/download/{attid}/stream`
   - `anonymous_file_id_download_stream` → `/jaxrs/anonymous/file/{id}/download/stream`
3. 分析 processplatform_service_processing 中的工作流处理函数，注册：
   - `work_id_processing` → `/jaxrs/work/{id}/processing`
   - `work_v2_id_terminate` → `/jaxrs/work/{id}/terminate`
   - `work_v2_id_retract` → `/jaxrs/work/{id}/retract`

**Patterns to follow:**
- `crates/query_assemble_surface/src/routes.rs` 现有路由注册模式
- `crates/processplatform_service_processing/src/routes.rs` 现有模式

**Test scenarios:**
- Happy path: 调用 `/jaxrs/queryview/flag/{view}/application/flag/{app}/execute` → 返回 ActionResult 含执行结果
- Happy path: 调用 `/jaxrs/file/{id}/download/stream` → 返回 200 + 二进制内容 + 正确 Content-Type
- Happy path: 调用 `/jaxrs/work/{id}/processing` → 工作状态从 pending 变为 processing
- Error path: 调用不存在的 work ID → 返回 404 或 error 响应
- Edge case: 下载不存在 ID 的文件 → 返回 404

**Verification:**
- `cargo check --workspace` 通过
- 所有新增路由可被 axum Router 正确匹配
- `cargo test --workspace --lib` 通过

---

### U2. 工作流列表分页与工作复杂信息接口

**Goal:** 新增工作流列表分页接口和工作复杂信息聚合接口，填补流程表单打开时的数据加载缺口

**Requirements:** R7, R9

**Dependencies:** U1（需要工作流基础路由先注册）

**Files:**
- Create: `crates/processplatform_service_processing/src/lib.rs`（新增函数）
- Modify: `crates/processplatform_service_processing/src/routes.rs`（注册新路由）
- Test: `crates/processplatform_service_processing/src/tests.rs`

**Approach:**
1. 实现 `work_list` 函数：查询 x_work 表，支持按 application（应用标记）过滤、分页（page/size）、排序
   - SQL: `SELECT id, title, process, application, work_status, creator, create_time FROM x_work WHERE application = $1 AND deleted_at IS NULL ORDER BY create_time DESC LIMIT $3 OFFSET ($2 - 1) * $3`
   - 返回 `{count, data: [...]}` 格式
2. 实现 `process_id_complex` 函数：聚合 x_work 关联的任务、审批、快照
   - 查询 x_task WHERE work = $1
   - 查询 x_review WHERE work_id = $1
   - 查询 x_snap WHERE work_id = $1
   - 返回聚合后的 JSON 对象

**Technical design:**
```
// work_list 伪代码
client.query("SELECT id, title, process, application, work_status, creator, create_time 
              FROM x_work WHERE application = $1 AND deleted_at IS NULL 
              ORDER BY create_time DESC LIMIT $3 OFFSET ($2-1)*$3", &[&app, &page, &size])
→ 返回 {count, data: [...]}

// process_id_complex 伪代码
SELECT 从 x_work, x_task, x_review, x_snap 
WHERE work_id/work = $1
→ 返回 {work: {...}, tasks: [...], reviews: [...], snaps: [...]}
```

**Patterns to follow:**
- `crates/processplatform_service_processing/src/lib.rs` 中已有的 list_processes 函数
- 使用 `row_to_json` helper（从 shared 模块导入）

**Test scenarios:**
- Happy path: 调用 work_list 带有效 application → 返回分页数据
- Happy path: 调用 process_id_complex 带有效 work ID → 返回聚合数据包含 tasks/reviews/snaps
- Edge case: application 为空时 → 返回所有工作列表
- Edge case: work ID 不存在 → 返回空数组
- Error path: 无效 page/size 参数 → 返回错误

**Verification:**
- 新增函数有 pool 参数，无 NotImplemented
- 路由正确注册
- 编译通过

---

### U3. CMS 数据接口（文档阅读计数、评论列表、查询视图表定义）

**Goal:** 实现 CMS 文档阅读计数、评论列表分页、查询视图表定义接口

**Requirements:** R10, R11, R12

**Dependencies:** U1（依赖已注册的路由基础设施）

**Files:**
- Modify: `crates/cms_assemble_control/src/lib.rs`（新增函数）
- Modify: `crates/cms_assemble_control/src/routes.rs`（注册路由）
- Test: `crates/cms_assemble_control/src/tests.rs`

**Approach:**
1. 实现 `document_id_view_count` 函数：
   - 对 x_cms_document 表中对应文档的 view_count 字段 +1
   - 返回新的计数值
2. 实现 `commend_list_paging` 函数：
   - 查询 x_cms_commend 表，支持分页（page/size）
   - 返回 {count, data: [...]}
3. 实现 `queryview_flag_definition` 函数：
   - 查询 x_query_view 表，返回视图的字段结构（content JSON 解析）
   - 返回 {fields: [...]}

**Patterns to follow:**
- `crates/cms_assemble_control/src/lib.rs` 中现有的列表函数模式
- 使用参数化查询防止 SQL 注入

**Test scenarios:**
- Happy path: 调用 document_id_view_count → view_count 递增
- Happy path: 调用 commend_list_paging → 返回分页评论列表
- Happy path: 调用 queryview_flag_definition → 返回视图字段结构
- Edge case: 文档不存在 → 返回 error
- Edge case: 评论列表为空 → 返回空数组

**Verification:**
- 新增函数全部有 pool 参数
- 路由注册正确
- 编译通过

---

### U4. 新建 organization_assemble_authentication 模块

**Goal:** 创建组织认证模块，实现人员头像和身份信息接口

**Requirements:** R13

**Dependencies:** None（全新模块）

**Files:**
- Create: `crates/organization_assemble_authentication/Cargo.toml`
- Create: `crates/organization_assemble_authentication/src/lib.rs`
- Create: `crates/organization_assemble_authentication/src/routes.rs`
- Modify: `oa4rust/Cargo.toml`（添加依赖）
- Modify: `oa4rust/src/main.rs`（注册路由）

**Approach:**
1. 参照 `crates/organization_assemble_express/src/lib.rs` 创建新模块
2. 实现 `person_id_icon` 函数：
   - 查询 auth_person 表获取头像 URL
   - 返回 {iconUrl} 或默认头像
3. 实现 `identity_id` 函数：
   - 查询 x_org_identity 表获取身份信息
   - 返回 {id, name, unit, ...}
4. 路由前缀: `/jaxrs/organization/assemble/authentication/*`

**Patterns to follow:**
- `crates/organization_assemble_express/src/lib.rs`（结构模板）
- `crates/organization_core_entity/src/lib.rs`（认证相关实体）

**Test scenarios:**
- Happy path: 调用 person_id_icon 带有效 person ID → 返回头像 URL
- Happy path: 调用 identity_id 带有效 identity ID → 返回身份详情
- Edge case: person ID 不存在 → 返回 404
- Edge case: 头像字段为空 → 返回默认头像

**Verification:**
- 模块编译通过
- 路由注册到 main.rs
- cargo check --workspace 通过

---

### U5. 新建 organization_assemble_personal 模块

**Goal:** 创建组织个人模块，实现个人设置和个人角色列表接口

**Requirements:** R14

**Dependencies:** U4（authentication 模块先完成，personal 可能需要访问同一数据库）

**Files:**
- Create: `crates/organization_assemble_personal/Cargo.toml`
- Create: `crates/organization_assemble_personal/src/lib.rs`
- Create: `crates/organization_assemble_personal/src/routes.rs`
- Modify: `oa4rust/Cargo.toml`
- Modify: `oa4rust/src/main.rs`

**Approach:**
1. 参照 `crates/organization_assemble_express/src/lib.rs` 创建新模块
2. 实现 `user_setting` 函数：
   - 查询 auth_person 表获取用户设置
   - 返回 {mobile, email, icon, theme, lang, ...}
3. 实现 `user_role_list` 函数：
   - 查询 x_org_group_member 和 x_org_role 表获取用户角色
   - 返回 {roles: [...]}
4. 路由前缀: `/jaxrs/organization/assemble/personal/*`

**Patterns to follow:**
- `crates/organization_assemble_express/src/lib.rs`（结构模板）
- `crates/organization_assemble_control/src/lib.rs`（个人相关操作）

**Test scenarios:**
- Happy path: 调用 user_setting 带有效 person ID → 返回用户设置
- Happy path: 调用 user_role_list 带有效 person ID → 返回角色列表
- Edge case: person ID 不存在 → 返回 404

**Verification:**
- 模块编译通过
- 路由注册到 main.rs
- cargo check --workspace 通过

---

### U6. AI 对话补全接口

**Goal:** 实现 AI 对话补全接口框架，接收消息返回模拟 AI 回复

**Requirements:** R15

**Dependencies:** None

**Files:**
- Modify: `crates/ai_assemble_control/src/lib.rs`（新增 chat_completion 函数）
- Modify: `crates/ai_assemble_control/src/routes.rs`（注册路由）

**Approach:**
1. 实现 `chat_completion` 函数：
   - 接收 messages 数组
   - 提取最后一条用户消息
   - 返回模拟的 AI 回复（先返回固定模板，后续接入 LLM API）
   - 记录对话到 x_ai_chat 表
2. 路由: `/jaxrs/ai_assemble_control/chat/completion`（POST）

**Patterns to follow:**
- `crates/ai_assemble_control/src/lib.rs` 中已有的 chat/list 等函数
- 复用 shared::response::ActionResult

**Test scenarios:**
- Happy path: 发送用户消息 → 返回 AI 回复文本
- Edge case: 空 messages → 返回错误
- Integration: 对话记录写入数据库

**Verification:**
- 编译通过
- 路由注册正确

---

### U7. 门户与查询接口

**Goal:** 实现门户详情、应用详情、身份详情接口

**Requirements:** R16, R17, R18

**Dependencies:** None

**Files:**
- Modify: `crates/portal/src/lib.rs`（新增 portal_id 函数）
- Modify: `crates/portal/src/routes.rs`（注册路由）
- Modify: `crates/cms_assemble_control/src/lib.rs`（新增 application_id 函数）
- Modify: `crates/organization_assemble_control/src/lib.rs`（新增 identity_id 函数）
- Modify: `crates/portal/src/routes.rs` / `cms_assemble_control/src/routes.rs` / `organization_assemble_control/src/routes.rs`

**Approach:**
1. 实现 `portal_id`：查询 x_portal 或 x_cms_view 表，返回门户页面配置
2. 实现 `application_id`：查询 x_cms_appinfo 表，返回应用信息
3. 实现 `identity_id`：查询 x_org_identity 表，返回身份详情

**Test scenarios:**
- Happy path: portal_id 带有效 ID → 返回门户配置
- Happy path: application_id 带有效 app ID → 返回应用信息
- Happy path: identity_id 带有效 ID → 返回身份详情
- Edge case: ID 不存在 → 返回 404

**Verification:**
- 编译通过
- 路由注册正确

---

### U8. 数据库 schema 扩展（migration 024）

**Goal:** 创建支持新接口所需的数据库表

**Requirements:** R11, R12, R13, R15

**Dependencies:** None

**Files:**
- Create: `migrations/024_create_gap_closure_tables.sql`
- Create: `migrations/024_create_gap_closure_tables_rollback.sql`

**Approach:**
新增以下表（如尚未存在）：
- `x_cms_commend`（评论/推荐表）— 支持 commend_list_paging
- `x_ai_chat`（AI 对话记录表）— 支持 chat_completion 持久化
- `x_cms_document_view_count`（文档阅读计数表）— 支持 document_id_view_count
- `x_org_identity` 已存在（migration 022），验证即可

**Patterns to follow:**
- `migrations/023_create_cms_assemble_tables.sql`（最近参考）
- 所有表使用 `CREATE TABLE IF NOT EXISTS`

**Test scenarios:**
- Happy path: 执行 migration 后，所有新表创建成功
- Edge case: 重复执行 migration → 幂等，不报错

**Verification:**
- migration SQL 语法正确
- rollback SQL 可逆

---

## System-Wide Impact

- **Interaction graph:** 新增路由注册影响 main.rs 的 router 组装；新模块添加到 Cargo.toml workspace
- **Error propagation:** 所有新接口使用统一的 AppError::Internal 错误传播模式
- **API surface parity:** 新增路由覆盖 o2web 核心业务流程所需的全部端点
- **Unchanged invariants:** 已有模块（auth、personal、file、processplatform 等）的核心逻辑不修改，仅扩展路由注册

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| o2web 前端路径与 oa4rust 路由存在细微差异 | 在 U1 中对账后逐一确认，必要时调整路由路径 |
| 新模块添加可能引起编译依赖问题 | 每个模块独立创建，逐步添加到 workspace |
| AI 对话接口后续需要接入真实 LLM | 当前实现通用框架，预留 API key 配置点 |
| 文档阅读计数并发写入冲突 | 使用 SELECT FOR UPDATE 或简单的原子 UPDATE |

---

## Documentation / Operational Notes

- 所有新增路由在 OpenAPI 文档中自动注册（utoipa）
- 新模块遵循既有代码风格：函数注释使用中文，遵循 axum::debug_handler 宏

---

## Sources & References

- **Origin document:** docs/brainstorms/2026-08-11-oa4rust-full-api-gap-closure-comprehensive-requirements.md
- Related code: crates/query_assemble_surface/src/lib.rs, crates/file_assemble_control/src/lib.rs, crates/processplatform_service_processing/src/lib.rs
- Related PR: #12

---

## 实现情况（2026-08-21 审计）

**审计基准：** 工作树 HEAD 314c7a75；判定状态：completed

### 已验证完成

- U1 路由注册修复（查询视图、文件下载流、工作流处理）：query_assemble_surface / file_assemble_control / processplatform_service_processing 均在档
- U2 工作流列表分页与复杂信息接口、U3 CMS 数据接口：随对应 crate 落地
- U4 organization_assemble_authentication 模块：crate 实测存在
- U5 organization_assemble_personal 模块：crate 实测存在
- U6 AI 对话补全接口：ai_assemble_control 在档（后续已叠加 SSE 流式，提交 ae911482）
- U7 门户与查询接口：portal / cms_assemble_control / organization_assemble_control 在档
- U8 migration 024：`migrations/024_create_gap_closure_tables.sql` 实测存在
- Deferred 完成：多级递归组织导航（2026-08-11-004 U3）、AI 对话多轮上下文管理（2026-08-11-004 U4）

### 未完成 / 遗留 → 待汇入剩余工作汇总计划

- Deferred「Office 文档预览完整 HTML 渲染引擎」：基础 HTML 预览已落地，xlsx/pptx 等格式仍缺
- Deferred「BBS 图片附件完整文件存储后端」
- Deferred「程序中心分发组装接口」：未逐项核验
