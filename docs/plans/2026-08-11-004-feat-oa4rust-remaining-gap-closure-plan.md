---
title: OA4Rust 剩余缺口补全 — 递归导航、AI 上下文、SQLx 清理、Office 预览
type: feat
status: active
date: 2026-08-11
origin: docs/brainstorms/2026-08-11-oa4rust-remaining-gap-closure-requirements.md
---

# OA4Rust 剩余缺口补全 — 递归导航、AI 上下文、SQLx 清理、Office 预览

## Summary

在已完成核心模块 stub 补全的基础上，填补最后四个 Deferred 缺口：unit 层级全量递归导航端点、AI 对话多轮上下文管理与 LLM 框架预留、SQLx 直接依赖批量清理（59 个 crate），以及 Office 文档纯 Rust 库 HTML 预览。完成后 o2web 组织树、AI 聊天、文件预览等核心业务流程不再有任何阻断性缺口。

---

## Problem Frame

oa4rust 已完成 83 个 crate 的真实化（2593 handlers），但 o2web 前端在以下场景仍无法工作：组织管理页面展开单位树时调用 `/jaxrs/unit/list/{id}/sub/nested` 返回 404；AI 助手面板只显示单条固定回复无对话历史；Office 文档附件点击预览只显示 URL 字符串而非内容。这些缺口的根本原因是三个遗留问题：migration 遗漏了 FILE_FILE.content 和 x_ai_chat.creator 列、SQLx 直接依赖与 SeaORM 为默认路径的架构决策矛盾、递归导航只覆盖了 group 层级未覆盖 unit 层级。

---

## Requirements

- R1. 实现 `unit_list_flag_sub_nested` 端点，递归查询 x_org_unit 完整子树
- R2. 实现 `unit_list_flag_sup_nested` 端点，递归查询 x_org_unit 祖先链
- R3. 实现 `unit_list_flag_sup_nested_type_type` 端点，路由兼容 OpenAPI 规范（type 参数保留用于路径匹配，因 x_org_unit 无 type 字段，实现时忽略该参数返回完整祖先链）
- R4. 所有递归端点返回 `{count, data: [{id, name, parentId, level, sort, creator, createTime}]}` 格式
- R5. 递归查询过滤 deleted_at IS NULL
- R6. chat_completion 加载 conversation_id 历史消息，按 create_time 排序取最近 N 条
- R7. 请求体支持可选 context_window 字段（默认 20，最大 100）
- R8. LLM 框架通过 AI_API_KEY 环境变量开关，无 key 时返回模拟回复
- R9. 响应新增 messages 字段包含对话历史 + 最新回复
- R10. 从所有 crate Cargo.toml 移除 sqlx.workspace = true，从 workspace Cargo.toml 移除 sqlx 声明
- R11. 新增 migration 补全 FILE_FILE.content 列（TEXT）和 x_ai_chat.creator 列（VARCHAR(255)）
- R12. 实现 attachment2_id_office_preview 真实 HTML 渲染，读取 content 列并转换
- R13. 使用纯 Rust 库（zip + 简单 XML 解析）处理 .docx，不支持格式降级返回 base64。注：Brainstorm 阶段曾考虑 minidocx crate，最终选用 zip + 手写 XML 解析以降低依赖复杂度，复杂格式（表格、图片）降级返回 base64
- R14. 注册 office_preview 路由到 file_assemble_control，端点需校验文件所有权

**Origin actors:** A1 (o2web 前端), A2 (oa4rust 后端), A3 (集成测试)
**Origin flows:** F1 (组织树展开), F2 (AI 多轮对话), F3 (Office 预览)
**Origin acceptance examples:** AE1 (sub/sup nested), AE2 (sup nested type), AE3 (context window), AE4 (LLM framework), AE5 (messages response), AE6 (office HTML)

---

## Scope Boundaries

- o2web 前端代码修改不在范围内
- Java o2server 代码修改不在范围内
- LibreOffice/pandoc 外部进程方案不在范围内（已确认为纯 Rust 库）
- SQLx 传递依赖（via sea-orm）保留，仅移除显式声明
- Office 预览仅支持 .docx 格式，其他格式降级返回 base64
- AI 对话不接入具体 LLM provider，仅预留 OpenAI-compatible API 框架
- BBS 图片附件完整文件存储后端不在范围内
- 多级递归导航的 sub/nested/type 过滤变体：unit 表无 type 字段，端点保持路由兼容但 type 参数被忽略（返回完整祖先链）

### Deferred to Follow-Up Work

- Office 文档预览的 .xlsx/.pptx 格式支持
- AI 对话的流式响应（SSE）
- SQLx 完全替代（含 sea-orm 底层依赖）
- 递归导航的 sub/nested/type 变体

---

## Context & Research

### Relevant Code and Patterns

- **递归导航**: `crates/organization_assemble_control/src/lib.rs` lines 457-550（group_list_flag_sub/sup_nested 的 WITH RECURSIVE CTE 模式）；`x_org_unit` 表有 parent_id 自引用、level、sort 字段
- **AI 对话**: `crates/ai_assemble_control/src/lib.rs` lines 1061-1101（当前 chat_completion 实现）；`migrations/024_create_gap_closure_tables.sql`（x_ai_chat 表，缺少 creator 列）
- **SQLx 清理**: 59 个 crate 的 Cargo.toml 有 `sqlx.workspace = true`，但源码零 `sqlx::` 引用
- **Office 预览**: `crates/file_assemble_control/src/lib.rs` lines 1298-1316（stub 实现，只返回 URL）；`FILE_FILE` 表缺少 content 列（migration 008 遗漏）
- **Base64 模式**: `crates/file_assemble_control/src/lib.rs` lines 538-539（base64 decode 统一模式）
- **路由注册**: 各 crate 的 `src/routes.rs` 使用 `Router::new().route(...).layer(Extension(pool))` 模式
- **HTTP 客户端**: `crates/auth/src/welink.rs`（OnceLock<reqwest::Client> 静态单例模式）

### Institutional Learnings

- **ActionResult 9 字段契约**: `docs/solutions/architecture-patterns/actionresult-9-field-contract.md`
- **PostgreSQL 大写标识符陷阱**: `docs/solutions/database-issues/postgresql-uppercase-identifier-trap.md`
- **嵌套 Tokio runtime panic**: `docs/solutions/integration-issues/nested-tokio-runtime-panic.md`

---

## Key Technical Decisions

- **递归导航复用 WITH RECURSIVE CTE 模式**：unit 是自引用树（比 group 简单），不需要 JOIN 其他表，直接对 x_org_unit 自身递归
- **SQLx 清理用脚本批量处理**：59 个 Cargo.toml 文件手动编辑不可行，编写 Python 脚本批量删除 `sqlx.workspace = true` 行
- **AI 上下文框架先存后查**：chat_completion 先保存消息到 x_ai_chat，下次请求加载历史作为上下文，LLM 调用走环境变量开关
- **Office 预览用 zip + minidom 模式**：.docx 本质是 ZIP 包，解压后解析 word/document.xml 提取文本并转换为简单 HTML，不支持的格式降级返回 base64

---

## Open Questions

### Resolved During Planning

- **FILE_FILE.content 列缺失**：migration 008 未声明 content 列但代码中大量使用 → 新增 migration 025 ALTER TABLE 补全
- **x_ai_chat.creator 列缺失**：migration 024 未声明 creator 列但 chat_completion INSERT 使用了 → 同 migration 025 补全
- **unit 无 type 字段**：OpenAPI 文档中有 sup/nested/type 端点但 x_org_unit 无 type 列 → 实现时忽略 type 参数或改为 name ILIKE 模糊匹配

### Deferred to Implementation

- **minidom vs 简单正则**：document.xml 解析用 minidom crate 还是手写简单解析 → 实现时根据复杂度决定
- **LLM API 端点**：OpenAI-compatible API 的具体 base URL 和模型名 → 由环境变量 AI_API_BASE 和 AI_MODEL 控制
- **递归深度限制**：x_org_unit 树的最大深度是否需要 protection → 实现时根据实际数据验证

---

## Implementation Units

### U1. Migration 025 — 补全缺失列

**Goal:** 修复 migration 024 遗漏的 x_ai_chat.creator 列和 FILE_FILE.content 列

**Requirements:** R11

**Dependencies:** None（独立 migration）

**Files:**
- Create: `migrations/025_add_missing_columns.sql`
- Create: `migrations/025_add_missing_columns_rollback.sql`

**Approach:**
1. `migrations/025_add_missing_columns.sql`：
   - `ALTER TABLE x_ai_chat ADD COLUMN IF NOT EXISTS creator VARCHAR(255)`
   - `ALTER TABLE "FILE_FILE" ADD COLUMN IF NOT EXISTS content TEXT`（注意 FILE_FILE 是双引号，PostgreSQL 区分大小写）
2. Rollback 对应 DROP COLUMN

**Patterns to follow:**
- `migrations/024_create_gap_closure_tables.sql`（最近参考）
- 所有 DDL 使用 `IF NOT EXISTS` 确保幂等

**Test scenarios:**
- Happy path: 执行 migration 后 `x_ai_chat` 表有 `creator` 列
- Happy path: 执行 migration 后 `FILE_FILE` 表有 `content` 列（TEXT 类型）
- Edge case: 重复执行 migration → 幂等，不报错

**Verification:**
- migration SQL 语法正确
- rollback SQL 可逆
- `cargo check --workspace` 通过（migration 本身不编译，仅验证 SQL 语法）

---

### U2. SQLx 直接依赖清理

**Goal:** 从 59 个 crate 的 Cargo.toml 和 workspace Cargo.toml 中移除 sqlx 直接依赖声明

**Requirements:** R10, R11

**Dependencies:** None（纯配置变更）

**Files:**
- Modify: `oa4rust/Cargo.toml`（移除 workspace sqlx 声明）
- Modify: ~59 个 crate 的 `Cargo.toml`（移除 `sqlx.workspace = true` 行）

**Approach:**
1. 编写 Python 脚本 `scripts/remove_sqlx_deps.py`：
   - 遍历 `crates/*/Cargo.toml`
   - 删除包含 `sqlx` 的行
   - 备份原始文件（可选）
2. 从 workspace `Cargo.toml` 的 `[workspace.dependencies]` 中删除 `sqlx = { ... }` 行
3. 运行 `cargo check --workspace` 验证编译通过

**Patterns to follow:**
- 已有 Cargo.toml 格式：`sqlx.workspace = true` 单独一行

**Test scenarios:**
- Happy path: 脚本执行后所有 crate Cargo.toml 不含 sqlx 声明
- Happy path: `cargo check --workspace` 通过（sea-orm 仍通过 sqlx-postgres feature 引入 sqlx）
- Edge case: 无 crate 因移除非 direct 依赖而编译失败

**Verification:**
- `grep -r "sqlx" crates/*/Cargo.toml` 无结果
- `cargo check --workspace` 通过，无新增 error
- 运行次数统计：处理 59 个文件

---

### U3. 多级递归组织导航

**Goal:** 新增 3 个 unit 层级递归导航端点，实现组织树完整展开能力

**Requirements:** R1, R2, R3, R4, R5

**Dependencies:** None（独立单元，x_org_unit 表结构已存在）

**Files:**
- Modify: `crates/organization_assemble_control/src/lib.rs`（新增 3 个函数）
- Modify: `crates/organization_assemble_control/src/routes.rs`（注册 3 条路由）

**Approach:**
1. `unit_list_flag_sub_nested`：
   - SQL: `SET cte_max_recursion_depth = 100; WITH RECURSIVE sub AS (SELECT id FROM x_org_unit WHERE id = $1 AND deleted_at IS NULL UNION ALL SELECT u.id FROM x_org_unit u JOIN sub s ON u.parent_id = s.id WHERE u.deleted_at IS NULL) SELECT id, name, parent_id, level, sort, creator, create_time FROM x_org_unit WHERE id IN (SELECT id FROM sub) ORDER BY sort ASC, create_time DESC`
   - 返回 `{count, data: [{id, name, parentId, level, sort, creator, createTime}]}`

2. `unit_list_flag_sup_nested`：
   - SQL: `WITH RECURSIVE sup AS (SELECT id FROM x_org_unit WHERE id = $1 AND deleted_at IS NULL UNION ALL SELECT u.id FROM x_org_unit u JOIN sup s ON u.id = s.parent_id WHERE u.deleted_at IS NULL AND u.id IS NOT NULL) SELECT id, name, parent_id, level, sort, creator, create_time FROM x_org_unit WHERE id IN (SELECT id FROM sup) ORDER BY level ASC`
   - 按 level 升序排列（根单位先返回）

3. `unit_list_flag_sup_nested_type_type`：
   - 由于 x_org_unit 无 type 字段，此端点实现为忽略 type 参数（返回完整祖先链），或在备注中标注此端点暂不支持
   - 实际实现：同 sup_nested，但记录 type 参数被忽略的日志

4. 路由注册：
   - `GET /jaxrs/organization/assemble/control/unit/list/{flag}/sub/nested`
   - `GET /jaxrs/organization/assemble/control/unit/list/{flag}/sup/nested`
   - `GET /jaxrs/organization/assemble/control/unit/list/{flag}/sup/nested/type/{type}`

**Patterns to follow:**
- `crates/organization_assemble_control/src/lib.rs` lines 457-550（group sub/sup nested 实现）
- 递归 CTE 模式：`WITH RECURSIVE` + `UNION ALL`
- 响应格式：与 `organization_assemble_control_unit_list_flag_next_count` 保持一致

**Test scenarios:**
- Happy path: 调用 sub/nested 带有效 unit ID → 返回所有子孙单位（含多层级）
- Happy path: 调用 sup/nested 带有效 unit ID → 返回所有祖先单位（从根到当前）
- Happy path: 叶子节点 sup/nested → 返回包含自身在内的完整祖先链
- Edge case: 不存在的 unit ID → 返回空数组
- Edge case: root 节点（parent_id IS NULL）sub/nested → 返回整棵树
- Edge case: 已删除的单位（deleted_at IS NOT NULL）→ 递归查询排除已删除节点
- Error path: 无限递归保护（x_org_unit 应无循环引用，CTE 有内置深度限制）
- **Bonus:** 顺手修复既有 group sup_direct bug（lib.rs:498 SQL 与 sup_nested 完全相同，应改为非递归查询仅返回直接上级）

**Verification:**
- 3 个新函数有 pool 参数，无 NotImplemented
- 路由正确注册到 routes.rs
- `cargo check --workspace` 通过
- 递归查询在测试数据下正确返回预期结果

---

### U4. AI 对话多轮上下文管理

**Goal:** 改造 chat_completion 端点，支持历史消息加载、上下文窗口管理、LLM 框架预留

**Requirements:** R6, R7, R8, R9

**Dependencies:** U1（x_ai_chat.creator 列补全）

**Files:**
- Modify: `crates/ai_assemble_control/src/lib.rs`（改造 chat_completion 函数）
- Modify: `crates/ai_assemble_control/src/routes.rs`（无需改动，路由已注册）
- Test: `crates/ai_assemble_control/src/tests.rs`（新增测试）

**Approach:**
1. 扩展 `ChatCompletionRequest` 结构体：
   - 新增 `context_window: Option<i32>` 字段（默认 20，最大 100）
2. chat_completion 改造逻辑：
    - 生成/获取 conversation_id
    - 从 Session 提取当前用户（`session.person_unique`），校验 conversation 所有权（仅允许读取 own conversations 或 shared conversations）
    - 查询 x_ai_chat WHERE conversation_id = $1 AND deleted_at IS NULL ORDER BY create_time ASC LIMIT context_window
   - 将历史消息拼接到当前 messages 前面（作为 system context）
   - 保存新 user 消息到 x_ai_chat
    - 检测 `AI_API_KEY` 环境变量：
      - 有值：构造 OpenAI-compatible 请求（`https://api.openai.com/v1/chat/completions`），发送 messages，解析 response.choices[0].message.content
      - 无值：返回模拟回复（当前行为）
    - 响应新增 `messages` 字段：[{role, content}, ...]（历史 + 最新回复）
    - 边界校验：context_window 超出 [1, 100] 范围时 clamp 到边界值
    - 安全：API key 不写入日志；LLM 错误统一返回 AppError::Internal，不泄露 provider 详情
3. HTTP 客户端：使用 `reqwest`（workspace 已有），参考 auth crate 的 `OnceLock` 模式

**Technical design:**
```
// 上下文加载伪代码
let history: Vec<(String, String)> = client.query(...)
  -> [(role, content), ...]

// 拼接上下文
let full_messages = [history..., ...req.messages]

// 边界校验
let context_window = req.context_window.unwrap_or(20).clamp(1, 100);

// LLM 调用（伪代码）
if let Some(api_key) = std::env::var("AI_API_KEY").ok() {
    let response = reqwest_client.post(API_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&{"model": MODEL, "messages": full_messages})
        .send().await?;
    let reply = response.json::<Value>().await?.["choices"][0]["message"]["content"].as_str();
} else {
    let reply = simulate_reply(&last_user_message);
}
```

**Patterns to follow:**
- `crates/auth/src/welink.rs`（reqwest + OnceLock 模式）
- `crates/ai_assemble_control/src/lib.rs`（现有 chat_completion 函数风格）
- `shared::response::ActionResult`（响应格式）

**Test scenarios:**
- Happy path: 调用 chat_completion 带已有 conversation_id → 返回历史消息 + 新回复
- Happy path: context_window=5 → 只加载最近 5 条历史
- Happy path: AI_API_KEY 未设置 → 返回模拟回复（与当前行为一致）
- Edge case: conversation_id 不存在 → 返回空历史 + 新回复
- Integration: 连续两次调用同一 conversation → 第二次包含第一次的回复作为历史
- Error path: AI_API_KEY 设置但 LLM API 返回错误 → 返回 AppError::Internal

**Verification:**
- chat_completion 加载历史消息并返回 messages 数组
- 无 AI_API_KEY 时行为与改前一致（向后兼容）
- `cargo check --workspace` 通过

---

### U5. Office 文档 HTML 预览

**Goal:** 实现 attachment2_id_office_preview 端点的真实 HTML 渲染，支持 .docx 格式

**Requirements:** R12, R13, R14

**Dependencies:** U1（FILE_FILE.content 列补全）

**Files:**
- Modify: `crates/file_assemble_control/src/lib.rs`（实现真实 HTML 渲染逻辑）
- Modify: `crates/file_assemble_control/src/routes.rs`（注册 office_preview 路由）

**Approach:**
1. 读取 FILE_FILE 表获取 content（base64 编码的原始文件字节），校验文件所有权（require_owner 模式）
2. base64 解码得到原始字节
3. 根据 extension/mime_type 判断文件格式：
   - `.docx`：用 `zip` crate 解压，读取 `word/document.xml`，用简单 XML 解析提取文本段落，转换为 HTML（`<p>` 标签）
   - 其他格式：降级返回 `{content: base64, contentType: mime}` （当前行为）
4. 返回 `{html: "<p>文档内容...</p>", contentType: "text/html"}` 或降级响应

**Technical design (.docx 解析):**
```
// docx 本质是 ZIP 包，document.xml 包含段落
1. base64::decode(content) -> Vec<u8>
2. zip::ZipArchive::new(&bytes) -> archive
3. archive.by_name("word/document.xml") -> xml_str
4. 简单解析 <w:p> 段落 → <p> 文本
5. 返回 HTML 字符串
```

**路由注册:**
- `GET /jaxrs/file/assemble/control/attachment2/{id}/office/preview/type/{type}` → `attachment2_id_office_preview_type_type`（复用现有函数，type 参数可忽略或用于格式校验）

**Patterns to follow:**
- `crates/file_assemble_control/src/lib.rs` lines 538-539（base64 decode 模式）
- `crates/file_assemble_control/src/lib.rs` lines 720-800（file 内容读取模式）
- `zip` crate 用于 .docx 解压（需在 Cargo.toml 添加 `zip = "2"` 依赖）

**Test scenarios:**
- Happy path: 调用 office_preview 带 .docx 文件 ID → 返回 HTML 内容
- Happy path: 调用 office_preview 带非 .docx 文件 → 降级返回 base64
- Edge case: content 为 NULL → 返回 error
- Edge case: 损坏的 .docx → 降级返回 base64
- Integration: 上传 .docx → 预览返回的 HTML 包含文档文本内容

**Verification:**
- office_preview 端点注册到 routes.rs
- .docx 文件返回 HTML 而非 base64
- 非 .docx 文件保持原有行为（降级）
- `cargo check --workspace` 通过

---

## System-Wide Impact

- **Interaction graph:** U1 migration 影响所有使用 x_ai_chat 和 FILE_FILE 的 crate；U2 清理影响 59 个 crate 的编译依赖；U3 影响 organization_assemble_control 路由；U4 影响 ai_assemble_control 端点行为；U5 影响 file_assemble_control 路由
- **Error propagation:** 所有新增接口使用统一的 AppError::Internal 错误传播模式；Office 预览降级不抛错
- **API surface parity:** 新增路由覆盖 o2web 组织树、AI 聊天、文件预览的核心端点
- **Unchanged invariants:** ActionResult<T> 9 字段结构不变；SQLx 通过 sea-orm 的传递依赖保留不影响功能

---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| FILE_FILE.content 列在已部署数据库已存在 | 高 | 低 | `ALTER TABLE ... ADD COLUMN IF NOT EXISTS` 幂等处理 |
| x_org_unit 存在循环 parent_id 引用 | 低 | 中 | PostgreSQL CTE 有内置深度限制（默认 100），超深时返回空结果 |
| 59 个 Cargo.toml 清理引入编译错误 | 中 | 高 | 逐 crate 验证，保留编译错误记录 |
| minidom crate 与项目其他依赖版本冲突 | 低 | 中 | 使用 `zip = "2"` 和 `quick-xml = "0.31"` 等稳定版本 |
| AI_API_KEY 明文存储在环境变量 | 中 | 低 | 仅开发/联调环境使用，生产环境通过 KMS/Secret Manager 管理 |

---

## Documentation / Operational Notes

- 新增 migration 025 需在生产环境执行（ALTER TABLE，低风险）
- SQLx 清理后 `cargo check` 应无新增 warning
- Office 预览的 HTML 输出为简化格式（纯文本段落），不包含复杂样式
- AI 对话的 LLM 框架需配置 `AI_API_KEY` 和 `AI_API_BASE` 环境变量才能启用真实调用

---

## Sources & References

- **Origin document:** docs/brainstorms/2026-08-11-oa4rust-remaining-gap-closure-requirements.md
- Related code: crates/organization_assemble_control/src/lib.rs (lines 457-550, 3176-3177), crates/ai_assemble_control/src/lib.rs (lines 1061-1101), crates/file_assemble_control/src/lib.rs (lines 1298-1316)
- Related migrations: migrations/008_file_tables.sql, migrations/024_create_gap_closure_tables.sql
- Related solutions: docs/solutions/architecture-patterns/actionresult-9-field-contract.md, docs/solutions/database-issues/postgresql-uppercase-identifier-trap.md
