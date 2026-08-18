---
date: 2026-08-11
topic: oa4rust-remaining-gap-closure
---

# OA4Rust 剩余缺口补全 — 递归导航、AI 上下文、SQLx 清理、Office 预览

## Summary

填补 oa4rust 最后四个 Deferred 缺口：unit 层级全量递归导航端点、AI 对话多轮上下文管理与 LLM 框架、SQLx 直接依赖清理、以及 Office 文档纯 Rust 库 HTML 预览。完成后 o2web 核心业务流程不再有任何阻断性缺口。

---

## Problem Frame

oa4rust 已完成 83 个 crate 的真实化（2593 handlers），但四个 Deferred 项目仍阻碍 o2web 完整联调：组织导航缺少 unit 层级的全量递归端点导致树形组件无法渲染；AI 对话仅返回固定文本、不加载历史消息；SQLx 作为直接依赖存在于 Cargo.toml 中与"SeaORM 为默认路径"的架构决策不一致；Office 文档预览只返回 URL 字符串、没有实际内容渲染。这些缺口使 o2web 的组织树、AI 聊天、文件预览等模块无法完整运行。

---

## Actors

- A1. **o2web 前端**: 调用 `/jaxrs/unit/list/{id}/sub/nested` 等端点渲染组织树；调用 AI 对话接口展示多轮聊天；预览 Office 文档
- A2. **oa4rust 后端**: 提供与 o2server 兼容的 API 端点
- A3. **集成测试**: 验证新增端点的请求/响应契约

---

## Key Flows

- F1. **组织树展开**
  - **Trigger:** o2web 打开组织管理页面，展开单位树节点
  - **Actors:** A1, A2
  - **Steps:** 前端调用 sub/nested 获取子单位 → 递归展开直到叶子 → 渲染完整树
  - **Outcome:** 用户看到完整的组织树形结构
  - **Covered by:** R1-R4

- F2. **AI 多轮对话**
  - **Trigger:** 用户在 o2web AI 助手面板发送消息
  - **Actors:** A1, A2
  - **Steps:** 前端携带 conversation_id 和 messages 数组 → 后端加载历史 → 拼接上下文 → 调用 LLM（或返回模拟回复）→ 返回新回复
  - **Outcome:** 用户看到连贯的多轮对话
  - **Covered by:** R5-R8

- F3. **Office 文档预览**
  - **Trigger:** 用户在 o2web 中点击 Office 文件附件
  - **Actors:** A1, A2
  - **Steps:** 前端调用 preview 端点 → 后端读取文件 BLOB → 转换为 HTML → 返回 HTML 内容
  - **Outcome:** 用户在浏览器中看到渲染后的文档内容
  - **Covered by:** R9-R11

---

## Requirements

**多级递归组织导航**
- R1. 实现 `GET /jaxrs/unit/list/{id}/sub/nested` 端点，使用 `WITH RECURSIVE` CTE 查询 x_org_unit 表的完整子树（含所有层级），返回全部子孙单位列表
- R2. 实现 `GET /jaxrs/unit/list/{id}/sup/nested` 端点，使用递归 CTE 向上遍历 parent_id 链，返回全部祖先单位列表
- R3. 实现 `GET /jaxrs/unit/list/{flag}/sup/nested/type/{type}` 端点，在 sup_nested 基础上增加 type 过滤（只返回指定类型的祖先单位）
- R4. 所有递归端点返回 `{count, data: [{id, name, parentId, level, ...}]}` 格式，与现有 unit 列表端点保持一致
- R5. 递归查询需处理 deleted_at IS NULL 软删除过滤，避免返回已删除的单位

**AI 对话多轮上下文管理**
- R6. `chat_completion` 端点改为加载 conversation_id 对应的历史消息（x_ai_chat 表），按 create_time 排序，截取最近 N 条（默认 20 条）作为上下文
- R7. 请求体支持可选的 `context_window` 字段（整数），控制返回的历史消息条数；默认值为 20，最大值为 100
- R8. 保留 LLM 调用框架：检测 `AI_API_KEY` 环境变量，有值时调用 OpenAI-compatible API，无值时返回模拟回复（当前行为不变）
- R9. 响应新增 `messages` 字段，包含对话历史 + 最新 AI 回复，格式为 `[{role, content}, ...]`

**SQLx 依赖清理**
- R10. 从 oa4rust/Cargo.toml 移除 sqlx 的直接依赖声明，保留 sea-orm 通过 sqlx-postgres feature 传递引入
- R11. 确保移除后 `cargo check --workspace` 无新增错误

**Office 文档 HTML 预览**
- R12. 实现 `attachment2_id_office_preview_type_type` 端点的真实 HTML 渲染：读取 FILE_FILE 表中的 base64 编码文件内容，转换为 HTML 返回
- R13. 使用纯 Rust 库完成转换（minidocx 用于 .docx，可选 minipptx/minixlsx 用于其他格式），不引入外部进程依赖
- R14. 转换失败时降级返回原始 base64 数据（当前行为），不中断请求

---

## Acceptance Examples

- AE1. **Covers R1, R2.** Given 一个存在多层级结构的 x_org_unit 树（parent_id 引用链深度 ≥ 3），when 调用 sub/nested 和 sup/nested 端点，then 返回完整层级数据，count 与实际节点数一致。
- AE2. **Covers R3.** Given 一个 type='department' 的单位，when 调用 sup/nested/type/department，then 只返回祖先链中 type 匹配的节点。
- AE3. **Covers R6, R7.** Given 一个已有 15 条消息的 conversation，when 调用 chat_completion 不带 context_window，then 返回最近 20 条上下文 + 新回复；当 context_window=5 时，只加载最近 5 条历史。
- AE4. **Covers R8.** Given 未设置 AI_API_KEY 环境变量的部署，when 调用 chat_completion，then 返回模拟回复（与当前行为一致）。
- AE5. **Covers R9.** Given 一次多轮对话，when 调用 chat_completion，then 响应中包含完整的 messages 数组（历史 + 新回复）。
- AE6. **Covers R12, R14.** Given 一个 .docx 格式的附件记录，when 调用 office_preview 端点，then 返回 HTML 内容；当转换库不支持该格式时，降级返回 base64。

---

## Success Criteria

- o2web 组织树组件可完整渲染（不再因 sub/sup nested 端点缺失而崩溃）
- o2web AI 聊天面板显示多轮对话历史
- `cargo check --workspace` 通过，无新增 error
- 所有新增端点注册到 main.rs 路由
- `cargo test --workspace --lib` 通过

---

## Scope Boundaries

- o2web 前端代码修改不在范围内
- Java o2server 代码修改不在范围内
- LibreOffice/pandoc 外部进程方案不在范围内（已确认为纯 Rust 库）
- SQLx 传递依赖（via sea-orm）不在本次移除范围内
- BBS 图片附件完整文件存储后端不在范围内
- AI 对话接入具体 LLM provider（仅框架预留，不实现具体 provider）不在范围内

### Deferred for later

- 多级递归组织导航的 type 过滤变体（sub/nested/type/{type}）
- Office 文档预览的 .xlsx/.pptx 格式支持（先 .docx 优先）
- AI 对话的流式响应（SSE）
- SQLx 完全替代（含 sea-orm 底层依赖）

---

## Key Decisions

- **递归导航复用 WITH RECURSIVE CTE 模式**：与现有 group sub/sup nested 实现保持一致，降低维护成本
- **AI 上下文窗口默认 20 条**：覆盖典型对话场景，通过 context_window 参数可调
- **LLM 框架通过环境变量开关**：AI_API_KEY 有值时启用真实调用，无值时走模拟回复，不阻塞联调
- **Office 预览优先 .docx 格式**：minidocx 库成熟稳定，其他格式降级处理
- **SQLx 从直接依赖移除**：保留 sea-orm 传递依赖，消除架构一致性警告

---

## Dependencies / Assumptions

- x_org_unit 表存在 parent_id 自引用字段且数据完整性良好（递归查询依赖此结构）
- x_ai_chat 表已有 conversation_id 和 create_time 字段（migration 024 已创建）
- FILE_FILE 表的 content 字段存储的是标准 Office 格式的 base64 编码
- minidocx crate 可解析现有 .docx 文件内容并提取文本/段落结构
