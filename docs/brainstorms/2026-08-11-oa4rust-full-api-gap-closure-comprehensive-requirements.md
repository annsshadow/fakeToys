---
date: 2026-08-11
topic: oa4rust-full-api-gap-closure
---

# OA4Rust 全量 API 接口对账与补全

## Summary

系统性地对账 o2web 前端调用的 REST API 与 oa4rust 后端已实现的接口，填补导致核心业务流程阻断的差距，使 oa4rust 在功能覆盖面上接近可替代 o2server 的水平。

---

## Problem Frame

oa4rust 已完成核心模块（流程引擎、文件管理、BBS、组织控制、CMS、BAM 等）的 stub 补全，但 o2web 前端在实际运行时仍因接口缺失或路径不匹配而崩溃或返回空数据。问题集中在三类：一是附件下载流式接口未注册到路由；二是查询视图执行接口（CMS 数据加载的核心）仅占位；三是工作流状态操作接口（工作处理、列表分页）缺失。这些缺口使得前端的核心业务流程——表单打开、附件下载、数据列表加载——无法跑通。

---

---

## Actors

- A1. **o2web 前端页面**: 通过 `/jaxrs/` 前缀的 REST API 发起所有数据请求
- A2. **oa4rust 后端服务**: 提供与 o2server 兼容的 API 端点，承载全部业务逻辑
- A3. **oa4rust 集成测试**: 验证每个新接口的请求/响应契约

---

## Key Flows

- F1. **流程表单打开**
  - **Trigger:** 用户在 o2web 中点击一个待办工作流项
  - **Actors:** A1, A2
  - **Steps:** 前端调用工作详情接口 → 调用附件下载流接口 → 调用查询视图执行接口加载表单数据 → 渲染表单
  - **Outcome:** 用户看到完整的表单和附件
  - **Covered by:** R3, R4, R7

- F2. **CMS 文档阅读**
  - **Trigger:** 用户在 o2web 中打开一篇 CMS 文档
  - **Actors:** A1, A2
  - **Steps:** 前端调用文档详情 → 调用查询视图执行接口加载附件列表 → 调用附件下载流 → 调用文档阅读计数
  - **Outcome:** 用户看到文档内容和可下载的附件
  - **Covered by:** R5, R7, R11

- F3. **工作流任务处理**
  - **Trigger:** 用户点击"处理"一个待办任务
  - **Actors:** A1, A2
  - **Steps:** 前端调用工作处理接口（启动处理） → 调用工作列表获取下一页 → 调用工作详情
  - **Outcome:** 任务状态更新，列表刷新
  - **Covered by:** R8, R9

---

## Requirements

**查询视图执行接口**
- R1. 实现 `view_flag_flag_query_queryFlag_execute` 接口，支持根据查询视图标记和应用标记执行查询，返回分页数据列表
- R2. 实现 `view_flag_flag_query_queryFlag_execute_v2_page_page_size_size` 接口，支持带分页参数的查询执行
- R3. 实现 `importmodel_id_execute` 接口，支持导入模型执行操作

**附件与文件下载流**
- R4. 实现 `/jaxrs/file/{id}/download/stream` 接口，返回文件二进制流，Content-Type 正确设置
- R5. 确保 `attachment_id_download_stream`、`file_id_download_stream` 等下载流接口已注册到路由且能正确返回二进制数据
- R6. 实现批量附件下载接口 `/jaxrs/attachment/batch/download/work/{workId}/site/{site}/stream`

**工作流核心操作**
- R7. 实现工作流列表分页接口，支持按应用标记过滤、分页、排序
- R8. 实现工作处理状态设置接口（`work_id_processing`），将工作从 pending 转为 processing
- R9. 实现工作复杂信息接口（`process_id_complex`），返回工作关联的任务、审批、快照等聚合数据

**CMS 数据接口**
- R10. 实现查询视图表定义接口，返回指定查询视图的字段结构
- R11. 实现文档阅读计数接口（`document_id_view_count`），累加指定文档的阅读次数
- R12. 实现评论列表分页接口（`commend_list_paging`），支持分页和排序

**组织模块补全**
- R13. 创建 `organization_assemble_authentication` 模块，实现人员头像接口（`person_id_icon`）、身份信息接口（`identity_id`）
- R14. 创建 `organization_assemble_personal` 模块，实现个人设置接口（`user_setting`）、个人角色列表接口

**AI 对话接口**
- R15. 实现 AI 对话补全接口（`ai_assemble_control_chat_completion`），接收消息并返回 AI 回复

**门户与查询接口**
- R16. 实现门户详情接口（`portal_id`），返回指定 ID 的门户页面配置
- R17. 实现应用详情接口（`application_id`），返回指定 ID 的应用信息
- R18. 实现身份详情接口（`identity_id`），返回指定 ID 的身份信息

---

## Acceptance Examples

- AE1. **Covers R1, R2.** Given 一个已注册的查询视图和有效应用标记，when 前端调用查询视图执行接口，then 返回包含分页数据和总条数的 JSON 响应，数据行与数据库记录一致。
- AE2. **Covers R4, R5.** Given 一个已上传的文件记录，when 前端调用 `/jaxrs/file/{id}/download/stream`，then 响应 Content-Type 与文件类型匹配，响应体为文件原始二进制数据。
- AE3. **Covers R7, R8.** Given 一个状态为 pending 的工作，when 前端调用工作处理接口，then 工作状态更新为 processing，响应中包含更新后的工作状态。
- AE4. **Covers R10, R11.** Given 一篇已发布的 CMS 文档，when 前端打开文档详情，then 文档信息完整返回，阅读计数随每次请求递增。
- AE5. **Covers R13, R14.** Given 一个已登录用户，when 前端请求人员头像和身份信息，then 响应返回正确的头像 URL 和身份详情。
- AE6. **Covers R15.** Given 一条用户消息，when 前端调用 AI 对话补全接口，then 响应包含 AI 生成的回复文本。

---

## Success Criteria

- o2web 核心业务流程（流程表单打开、CMS 文档阅读、工作流任务处理）在无 o2server 的情况下可正常运行
- 所有新增接口通过集成测试，覆盖 happy path 和关键边界条件
- oa4rust 编译通过，无新增 warning
- 新增模块统一遵循现有代码模式：Pool 依赖注入、参数化查询、ActionResult 响应格式

---

## Scope Boundaries

- o2web 前端代码修改不在范围内
- Java o2server 代码修改不在范围内
- 安装初始化相关接口（secret、server、database、h2、restore 等）不在本次范围内
- SSO 单点登录入口不在本次范围内
- 文件物理存储后端迁移（已有 base64 BLOB 方案）不在本次范围内
- LDAP 用户自动同步不在本次范围内

### Deferred for later

- 多级递归组织导航（unit sub-nested/sup-nested 全量递归）
- Office 文档预览的完整 HTML 渲染引擎
- BBS 图片附件的完整文件存储后端
- SQLx 完全移除
- AI 对话的多轮上下文管理
- 程序中心分发组装接口

---

## Key Decisions

- **查询视图执行复用 query_service_processing 模式**: 现有的 process_query 接口已具备基础框架，新接口在此基础上扩展，复用相同的参数解析和数据库查询模式
- **附件下载流使用已有的 file_assemble_control 基础设施**: 复用 `file_id_download_stream` 的实现模式，避免重复代码
- **新增模块遵循既有 crate 命名约定**: `organization_assemble_authentication` 和 `organization_assemble_personal` 与 `organization_assemble_express`、`organization_assemble_control` 保持相同的命名结构
- **数据库 schema 跟随新增接口扩展**: 如需新表（如 ai_chat、query_view_field），新建 migration 文件而非修改已有迁移

---

## Dependencies / Assumptions

- 已有的数据库表（x_work, x_task, x_cms_document, x_attachment, x_query 等）存在并可查询
- 流程引擎表已通过 migration 020 创建
- CMS 表已通过 migration 023 创建
- o2web 前端的 API 调用路径可能与 oa4rust 当前的路由路径存在差异，需要在对账后逐一校准

---

## Outstanding Questions

### Resolve Before Planning

- [User decision] o2web 前端调用路径与 oa4rust 路由路径是否需要对齐？如果发现路径不匹配，是直接修改 oa4rust 路由还是在前端做适配？

### Deferred to Planning

- [Needs research] 查询视图（query view）的实际数据结构是什么？需要哪些字段才能满足 o2web 的渲染需求
- [Needs research] AI 对话接口的上游 LLM 服务是什么？是否需要接入外部 API
- [Technical] 附件下载流的大小限制和超时策略是否需要特别处理
