---
title: OA4Rust 全面差距补全
type: feat
status: completed
date: 2026-08-10
origin: docs/brainstorms/2026-08-10-oa4rust-comprehensive-gap-audit-requirements.md
---

# OA4Rust 全面差距补全计划

## Summary

通过 8 个实施单元补全 oa4rust 与 Java OA 的差距：认证安全模块扩展（双因素登录、SSO、安全注销、Token 校验、用户切换）、用户注册与个人功能补全、MCP 工具桥接全量扩展（~883 个 Rust 端点）、OpenAPI 文档完善、行为对比测试全覆盖，以及 todo crate 标记清理。

---

## Problem Frame

oa4rust 已完成 83 个 crate 的真实化和 SeaORM 迁移，2458 个 handler，但"完全代替 oa"仍有四个维度的缺口：认证安全模块缺失 10+ 个关键端点（双因素登录、SSO、安全注销等），MCP 工具桥接覆盖率仅 1.3%（~100/883），OpenAPI 文档仅 14 个占位路径，行为对比测试覆盖率不足 1%（~79/883）。这些缺口使 oa4rust 无法在生产环境中完全替代 Java OA。

---

## Requirements

- R1. 实现双因素登录端点，支持短信验证码的第二因子验证
- R2. 实现 SSO 单点登录端点，支持 GET/POST 两种方式的 token 登录和 3DES 加密辅助
- R3. 实现安全注销端点，使当前用户所有 session 全部过期
- R4. 实现 Token 校验端点，允许外部系统验证 OA token 有效性
- R5. 实现用户切换端点，需要系统管理员权限
- R6. 将 MCP 工具桥接从 ~100 个扩展至覆盖全部 ~883 个 Rust 端点（以 Rust 实际路由数为准）
- R7. 每个 MCP 工具包含工具名称、HTTP 方法、路径、描述、路径参数列表、body 参数列表、requires_auth 字段
- R8. 区分需认证的端点和公开端点，在工具元数据中标注 requires_auth 字段
- R9. 将 OpenAPI 占位路径从 14 个扩展至覆盖全部 ~883 个已实现 Rust 端点
- R10. 每个 OpenAPI path item 包含 tag、summary、parameters、responses
- R11. 保持 utoipa derive 宏编译通过，/openapi.json 端点可正常生成规范
- R12. 将行为对比测试端点清单扩展至覆盖全部 ~883 个 Rust 端点（含新增认证安全端点）
- R13. 每个端点包含 crate_name、method、rust_path、java_war、java_action、body、requires_auth 字段
- R14. 验证 Java 服务不可用时全部端点标记为 SKIP 而非 FAIL
- R15. 补全用户注册功能，包含验证码发送、用户名/手机/邮箱唯一性校验、密码设置
- R16. 补全电子签名管理功能，包含签名上传、列表查询、删除
- R17. 补全个人头像上传功能，支持图片格式校验和大小限制
- R18. 调查 calendar/process_express/process_surface/mcp_server/openapi/shared 的 todo 标记原因
- R19. 确保 cargo test --workspace --lib 全部通过

**Origin actors:** A1 (开发者), A2 (前端 o2web), A3 (AI Agent/MCP 客户端), A4 (下游规划 Agent)
**Origin flows:** F1 (认证安全补全流程), F2 (MCP 工具桥接扩展流), F3 (行为对比测试扩展流)
**Origin acceptance examples:** AE1 (认证安全), AE2 (MCP 工具), AE3 (OpenAPI), AE4 (行为对比测试), AE5 (业务功能), AE6 (todo 清理)

> 注：R18 原为"补全微信小程序登录和 WeLink 登录端点"，已移至 Deferred to Follow-Up Work（需第三方 API 密钥配置）。当前 R18 为"todo crate 标记调查"，R19 为"全量测试通过"。

---

## Scope Boundaries

- **包含：** 认证安全模块补全（双因素登录、SSO、安全注销、Token 校验、用户切换）；用户注册、电子签名、头像上传；MCP 工具桥接全量扩展；OpenAPI 文档完善；行为对比测试全覆盖；todo crate 标记清理
- **排除在外：** 前端 o2web 代码修改；Java 后端代码修改；文件存储层迁移；定时任务/批处理框架迁移；数据库连接池性能优化；微服务拆分

### Deferred to Follow-Up Work

- 政务钉钉登录（zhengwudingding）— 特定客户场景，非通用需求
- 批量操作端点（批量删除、批量导入）— 低频使用场景
- 流程平台深度功能（processplatform 的复杂编排端点）— 需要单独评估
- SQLx 完全移除（ORM 为默认路径，复杂查询可保留 SQLx 并存）
- 微信小程序/WeLink 登录（需第三方 API 密钥配置，归入后续迭代）

---

## Context & Research

### Relevant Code and Patterns

- **认证扩展参考：** `crates/auth/src/lib.rs`（登录/登出/OAuth/扫码绑定模式）、`crates/auth/src/password.rs`（bcrypt/MD5/DES 三算法兼容）、`crates/auth/src/bind.rs`（in-memory store + Mutex 模式）
- **个人模块参考：** `crates/personal/src/lib.rs`（个人信息 CRUD 模式）、`crates/personal/src/reset.rs`（ResetCodeStore 模式）、`crates/personal_extend/src/`（头像/密码子模块）
- **RBAC 权限：** `crates/shared/src/middleware/rbac.rs`（PermissionRegistry、is_admin、person_has_role、require_owner）
- **MCP 工具桥接：** `crates/mcp_server/src/tool_bridge.rs`（ROUTE_DEFS 静态数组 + register_tool! 宏）
- **OpenAPI：** `crates/openapi/src/lib.rs`（utoipa derive 宏 + 占位函数模式）、`crates/base/src/lib.rs`（#[utoipa::path] 内联注解模式）
- **测试模式：** `crates/auth/src/tests.rs`（单元测）、`crates/personal/src/tests.rs`（异步单元测）、`tests/behavior_compare.rs`（行为对比）、`tests/integration_tests/`（集成测）
- **脚本工具：** `scripts/gen_inventory.py`（端点清单生成）、`scripts/extract_endpoints.py`（行为对比端点提取）

### Institutional Learnings

- **IDOR 安全修复教训：** 所有写操作必须调用 `require_owner`，防止跨用户篡改 (`docs/solutions/security-issues/idor-vulnerability-write-handlers.md`)
- **ActionResult 9 字段契约：** 前端 action.js 隐式依赖 data/type/message/date/spent/size/count/position/prompt 结构，任何修改必须保持兼容 (`docs/solutions/architecture-patterns/actionresult-9-field-contract.md`)
- **嵌套 Tokio runtime panic：** router 工厂函数中的 block_on 需用 catch_unwind 包装 (`docs/solutions/integration-issues/nested-tokio-runtime-panic.md`)
- **双 Pool 共存：** SQLx Pool 用于认证/RBAC，SeaORM DatabaseConnection 用于实体 CRUD，两者连接同一 DATABASE_URL (`docs/solutions/architecture-patterns/seaorm-dual-pool-coexistence.md`)
- **PostgreSQL 大写标识符陷阱：** SeaORM 实体必须显式指定 table_name 和 column_name (`docs/solutions/database-issues/postgresql-uppercase-identifier-trap.md`)

---

## Key Technical Decisions

- **认证端点扩展 auth crate 而非新建：** 双因素登录、SSO、安全注销、Token 校验、用户切换均属于认证域，扩展现有 auth crate 的子模块（two_factor.rs、sso.rs、safe_logout.rs、check_token.rs、switch_user.rs）遵循现有模式，避免新增 crate 的注册和维护成本
- **用户注册/签名/头像扩展 personal crate：** 这些功能属于个人管理域，扩展 personal crate 的子模块（regist.rs、signature.rs、icon.rs）遵循现有 lib.rs + 子模块模式
- **MCP/OpenAPI 通过脚本自动生成：** 手动维护 ~883 条 MCP 工具注册和 ~883 个 OpenAPI 路径不现实，编写 Python 脚本从端点清单自动生成，放置在 scripts/ 目录
- **SSO token 兼容 Java 3DES 格式：** Rust 侧实现 3DES EDE 加解密逻辑（非现有单 DES），token 格式为 `加密(credential#timestamp)`，与 Java 端互通。需新增 `des3_encrypt`/`des3_decrypt` 函数（16/24 字节 key），现有 `des_encrypt` 为单 DES 仅用于密码存储。
- **用户切换复用现有角色体系：** 通过 `person_has_role(pool, person_unique, "admin").await` 判断管理员权限，不新增数据库字段
- **todo crate 标记通过重新运行清单脚本解决：** 先运行 `scripts/gen_inventory.py` 重新扫描，确认是统计口径问题还是真实缺失

---

## Open Questions

### Resolved During Planning

- **双因素登录实现方式：** 使用短信验证码（复用现有 CodeStore），无需引入 TOTP 依赖
- **SSO token 加密格式：** 需实现真正的 3DES EDE 加解密（16/24 字节 key），与 Java 端互操作。现有 `des_encrypt` 为单 DES（8 字节 key），不可复用。需新增 `des3_encrypt`/`des3_decrypt` 函数
- **用户切换权限模型：** 统一使用 `is_admin` 函数检查管理员权限
- **des crate 依赖：** auth crate 已依赖 `des = "0.8"`，3DES 实现需扩展 `cipher` crate 的 `BlockDecryptMut` trait
- **新增认证端点权限：** 在 `PermissionRegistry::with_defaults()` 中为 two_factor/safe_logout/check_token/switchuser/sso 端点注册精确权限覆盖
- **认证端点限流：** 在 `AUTH_RATE_LIMIT_PREFIXES` 中添加 two_factor/safe_logout/switchuser 路径
- **check_token 端点权限：** 从 Public 改为 Authenticated，防止会话枚举
- **SSO 加密辅助端点：** 仅保留 POST 变体，避免 GET 端点在 URL 中暴露敏感参数

### Deferred to Implementation

- **SSO 端点的完整请求/响应结构：** 需对照 Java SsoAction 的实际 JSON 契约确定
- **用户注册的唯一性冲突处理细节：** 需确认 Java 端在用户名/手机/邮箱冲突时的具体错误消息
- **电子签名的存储方式：** 本地文件系统 vs 对象存储，需评估现有文件存储模块能力
- **MCP 脚本自动生成的路径参数解析规则：** 需确定如何处理带路径参数的端点（如 `/jaxrs/person/{id}`）
- **OpenAPI 生成的 tag 分配规则：** 需确定按 crate 名还是按业务域分配 tag
- **SSO 3DES key 的分发方式：** 加密辅助端点返回的加密 token 中，key 如何安全分发（预共享 vs API 动态传递）
- **safe_logout 并发注销的 SessionManager 锁粒度：** RwLock 是否足够，大量 session 批量删除的性能风险

---

## Implementation Units

### U1. 认证安全基础扩展（双因素登录、安全注销、Token 校验）

**Goal:** 为 auth crate 补全双因素登录、安全注销、Token 校验三个缺失端点

**Requirements:** R1, R3, R4, R19, R20

**Dependencies:** 无（在 auth crate 内新增子模块）

**Files:**
- Modify: `crates/auth/src/lib.rs`（新增路由注册）
- Create: `crates/auth/src/two_factor.rs`（双因素登录 handler）
- Create: `crates/auth/src/safe_logout.rs`（安全注销 handler）
- Create: `crates/auth/src/check_token.rs`（Token 校验 handler）
- Test: `crates/auth/src/tests.rs`（新增测试用例）

**Approach:**
- 双因素登录：在已有短信验证码流程基础上，第一因子为 credential+password，第二因子为短信验证码。handler 接收 credential、password、code 三个字段，先验证第一因子（复用现有 login 逻辑），再验证验证码（复用 CodeStore）
- 安全注销：遍历 SessionManager 中所有属于当前用户的 session token，批量移除。U1 需先在 `crates/shared/src/session.rs` 中新增 `remove_sessions_by_person(person_unique: &str)` 方法
- Token 校验：接收 token 字段，查询 SessionManager 验证有效性，返回 authenticated + person 信息。权限级别为 Authenticated（非 Public），防止未认证用户枚举有效 token

**Patterns to follow:**
- handler 签名：`async fn handler(pool: Extension<Pool>, session_manager: Extension<SessionManager>, ...) -> Result<Json<ActionResult<T>>, AppError>`
- 响应格式：`ActionResult::success(data)` / `ActionResult::error(msg)`
- 权限：双因素登录和安全注销为 Authenticated，Token 校验为 Authenticated（非 Public，防止会话枚举）

**Test scenarios:**
- Happy path: 有效 credential+password+code → 返回成功会话 token（双因素登录）
- Happy path: 有效 token 请求安全注销 → 该用户所有 session 失效
- Happy path: 有效 token 校验请求 → 返回 authenticated=true + person 信息
- Error path: 第一因子密码错误 → 返回 error，不暴露是否验证码正确（防枚举）
- Error path: 验证码过期或错误 → 返回 error
- Error path: 未认证用户调用安全注销 → 返回 401
- Error path: 无效 token 校验 → 返回 error（不返回 authenticated=false，防止会话枚举）

**Verification:**
- `cargo test -p auth` 通过
- 新端点注册到 auth crate 的 router() 函数
- 权限级别正确（双因素/安全注销/Token校验=Authenticated，Token校验非Public）
- `crates/shared/src/middleware/rbac.rs` 中 `with_defaults()` 包含新增端点的精确权限覆盖
- `crates/shared/src/middleware/constants.rs` 中 `AUTH_RATE_LIMIT_PREFIXES` 包含 two_factor/safe_logout/switchuser
- `crates/shared/src/session.rs` 中 SessionManager 新增 `remove_sessions_by_person` 方法

---

### U2. SSO 单点登录

**Goal:** 为 auth crate 补全 SSO 单点登录端点和 3DES 加密辅助端点

**Requirements:** R2

**Dependencies:** 无（安全注销模式可参考 U1，但无功能依赖，可并行执行）

**Files:**
- Modify: `crates/auth/src/lib.rs`（新增路由注册）
- Create: `crates/auth/src/sso.rs`（SSO handler + 3DES 加密辅助）
- Test: `crates/auth/src/tests.rs`（新增 SSO 测试用例）

**Approach:**
- SSO token 格式：`加密(credential#timestamp)`，使用 auth crate 已有的 `des_encrypt` 函数（3DES ECB 模式，8 字节 key）
- GET 端点：`/jaxrs/authentication/sso/client/{client}/token/{token}` — 解密 token，验证时间戳有效期（如 5 分钟内），签发会话
- POST 端点：`/jaxrs/authentication/sso` — 从请求体解密 token，逻辑同 GET
- 加密辅助端点：`POST /jaxrs/authentication/sso/encrypt`（请求体包含 key 和 credential）— 返回加密后的 token（供前端或第三方使用）；避免 GET 端点在 URL 中暴露敏感参数
- 时间戳校验：防止重放攻击，token 超过 5 分钟有效期拒绝

**Patterns to follow:**
- 复用 `auth/src/password.rs` 的 `des_encrypt` 函数（已存在）
- 解密逻辑：`des_encrypt` 是单 DES 加密函数，SSO 需要实现真正的 3DES EDE 解密函数 `des3_decrypt`（使用 `des::cipher::BlockDecryptMut`，16 或 24 字节 key）
- 与 Java 端互操作验证：用 Java 端加密的已知 token 在 Rust 端解密，确认结果一致
- 路径参数提取：`Path((client, token)): Path<(String, String)>`

**Test scenarios:**
- Happy path: 有效加密 token → 解密成功，签发会话，返回 token
- Happy path: 加密辅助端点 → 返回正确格式的加密 token
- Edge case: token 时间戳超过 5 分钟 → 返回 error（过期）
- Edge case: 无效 3DES key（长度不足 8 字节）→ 返回 error
- Error path: 解密失败（token 格式错误）→ 返回 error
- Error path: credential 不存在 → 返回 error（不暴露 credential 是否存在）

**Verification:**
- `cargo test -p auth` 通过
- 3DES 加解密与 Java 端互操作（可手动验证：用 Java 加密的 token 在 Rust 端解密成功）

---

### U3. 用户切换功能

**Goal:** 实现管理员切换用户身份的端点

**Requirements:** R5

**Dependencies:** U1（需 SessionManager 提供 `remove_sessions_by_person` 方法，由 U1 新增）

**Files:**
- Modify: `crates/auth/src/lib.rs`（新增路由注册）
- Create: `crates/auth/src/switch_user.rs`（用户切换 handler）
- Test: `crates/auth/src/tests.rs`（新增用户切换测试）

**Approach:**
- 端点：`PUT /jaxrs/authentication/switchuser` 和 `POST /jaxrs/authentication/switchuser/mockputtopost`
- 权限：仅 admin 可调用，通过 `is_admin(pool, &session.person_unique).await` 检查
- 逻辑：管理员请求体包含目标 credential，系统为该 credential 创建新 session，返回新 token
- 原管理员 session 保持有效（切换是"以他人身份操作"而非"替换身份"）
- 记录切换日志（可选：写入 CONSOLE_LOG 或独立 audit 表）

**Patterns to follow:**
- **User switching:** admin-only endpoint (`PUT /jaxrs/authentication/switchuser`) — verify admin role via `is_admin()`, create new session for target credential, log the switch action
- 错误处理：非管理员调用返回 `AppError::Forbidden`
- Session 创建：复用 `session_manager.create_session(person_unique, token)` 模式

**Test scenarios:**
- Happy path: admin 用户切换为普通用户 → 返回新 token，新 token 可用作目标用户
- Happy path: mock PUT to POST 变体 → 行为与 PUT 一致
- Error path: 非 admin 用户调用 → 返回 403 Forbidden
- Error path: 目标 credential 不存在 → 返回 error
- Edge case: 切换到自身 → 返回新 token（允许）

**Verification:**
- `cargo test -p auth` 通过
- 切换后的 token 可正确解析为目标用户身份（通过 whoami 端点验证）

---

### U4. 用户注册与个人扩展

**Goal:** 补全用户注册、电子签名管理、头像上传功能

**Requirements:** R15, R16, R17, R18

**Dependencies:** 无（扩展 personal 和 personal_extend crate）

**Files:**
- Create: `crates/personal/src/regist.rs`（用户注册 handler）
- Modify: `crates/personal/src/lib.rs`（新增路由注册）
- Create: `crates/personal_extend/src/signature.rs`（电子签名 handler）
- Modify: `crates/personal_extend/src/lib.rs`（导出 signature 模块）
- Modify: `crates/personal_extend/src/avatar.rs`（确认现有实现，无需重写）
- Test: `crates/personal/src/tests.rs`、`crates/personal_extend/src/tests.rs`

**Approach:**
- 用户注册：参考 Java `x_organization_assemble_personal/regist/` 的 ActionCreate、ActionCheckName、ActionCheckMobile、ActionCheckPassword 等端点。Rust 侧实现：验证码发送、用户名唯一性校验、手机/邮箱唯一性校验、密码设置
- 电子签名：参考 Java `x_organization_assemble_personal/signature/` 的 ActionUpload、ActionList、ActionDelete。Rust 侧实现：签名图片上传（存储到文件模块）、签名列表查询、签名删除
- 头像上传：扩展 personal_extend 的 avatar 模块，支持图片格式校验（jpg/png/gif）和大小限制（如 2MB）
- 唯一性校验：SQL UNIQUE 约束 + 应用层预检查，冲突时返回明确错误消息

**Patterns to follow:**
- 复用 personal crate 的 `resolve_current_person_unique` 模式（仅注册端点不需要认证）
- 复用 personal crate 的 `ResetCodeStore` 模式（注册验证码存储）
- 文件上传：复用 file_assemble_control crate 的上传逻辑（如有）
- 输入验证：复用 `shared/src/input_validation.rs` 的校验助手

**Test scenarios:**
- Happy path: 有效注册请求（唯一用户名/手机/邮箱）→ 创建用户，返回 success
- Happy path: 用户名已存在 → 返回明确错误 "username already exists"
- Happy path: 手机号已存在 → 返回明确错误 "mobile already exists"
- Happy path: 邮箱已存在 → 返回明确错误 "email already exists"
- Happy path: 有效密码强度 → 创建成功
- Happy path: 弱密码（如 123456）→ 返回 error（密码强度校验）
- Happy path: 有效签名上传 → 返回签名 ID 和 URL
- Happy path: 有效头像上传（jpg, <2MB）→ 返回头像 URL
- Error path: 图片格式不支持（如 exe）→ 返回 error
- Error path: 图片大小超限（>2MB）→ 返回 error
- Error path: 未登录用户访问签名/头像端点 → 返回 401

**Verification:**
- `cargo test -p personal -p personal_extend` 通过
- 注册端点无需认证（Public 权限级别）
- 签名/头像端点需要认证（Authenticated 权限级别）

---

### U5. MCP 工具桥接全量扩展

**Goal:** 将 MCP 工具桥接从 ~100 个扩展至覆盖全部 ~883 个 Rust 端点

**Requirements:** R6, R7, R8

**Dependencies:** 无（独立扩展 mcp_server crate）

**Files:**
- Create: `scripts/gen_mcp_tools.py`（MCP 工具注册生成脚本）
- Create: `crates/mcp_server/src/generated_routes.rs`（生成产物，由脚本输出）
- Modify: `crates/mcp_server/src/tool_bridge.rs`（导入生成结果）
- Test: `crates/mcp_server/src/tests.rs`（新增 MCP 工具注册测试）

**Approach:**
- 生成脚本：读取 `docs/brainstorms/oa4rust-endpoint-inventory.md` 和源码中的路由注册，自动生成 `ROUTE_DEFS` 数组
- 脚本输出：生成 `crates/mcp_server/src/generated_routes.rs`（或直接嵌入 tool_bridge.rs）
- 工具命名：保持 `jaxrs_{crate}_{action}` 命名约定
- 路径参数提取：从路由路径（如 `/jaxrs/person/{id}`）自动提取 path_params
- body_params 推断：根据 HTTP method 和路径模式推断（POST/PUT/DELETE 有 body，GET 无 body）
- requires_auth 推断：从 PermissionRegistry 获取权限级别，Public → false，其他 → true

**Patterns to follow:**
- 保持 `register_tool!` 宏的命名约定和字段结构
- 生成脚本风格：参考 `scripts/gen_inventory.py` 和 `scripts/extract_endpoints.py`

**Test scenarios:**
- Happy path: 运行生成脚本 → 生成的 ROUTE_DEFS 包含全部 ~883 个端点
- Happy path: MCP tools/list 返回工具数量 ≈ 883
- Happy path: 随机抽取 10 个工具 → 每个包含 name、description、inputSchema（含 path_params 和 body_params）
- Happy path: 需认证端点的工具 → requires_auth=true
- Happy path: 公开端点的工具 → requires_auth=false
- Error path: 生成脚本无法解析的路由 → 跳过并记录警告，不中断生成

**Verification:**
- `cargo test -p mcp_server` 通过
- 启动 oa4rust 并调用 MCP tools/list → 返回工具数量 > 7000
- `/openapi.json` 端点正常（MCP 扩展不影响 OpenAPI）

---

### U6. OpenAPI 文档完善

**Goal:** 将 OpenAPI 规范从 14 个占位路径扩展至覆盖全部 ~883 个已实现 Rust 端点

**Requirements:** R9, R10, R11

**Dependencies:** 无（可独立扩展 mcp_server crate，与 U5 并行执行）

**Files:**
- Create: `scripts/gen_openapi_paths.py`（OpenAPI 路径生成脚本）
- Modify: `crates/openapi/src/lib.rs`（替换占位函数为生成结果或动态生成）
- Test: `tests/integration_tests/scenarios/test_openapi_generation.rs`（集成测试验证 /openapi.json 输出）

**Approach:**
- 生成脚本：读取端点清单，为每个端点生成 `#[utoipa::path]` 注解的占位函数
- 占位函数风格：参考现有 `crates/openapi/src/lib.rs` 的空函数模式
- 自动推断 tag：从 crate 名映射（如 `auth` → "authentication"，`calendar` → "calendar"）
- 自动推断 summary：从路径段生成（如 `/jaxrs/calendar/event/list/{calendarId}` → "List calendar events"）
- 编译约束：utoipa derive 宏对路径数量有限制，如 883 个路径导致编译超时，考虑分 tag 生成或多文件结构
- 备选方案：如果 utoipa derive 无法处理大规模路径，改为运行时生成 OpenAPI JSON（不依赖 derive 宏）

**Patterns to follow:**
- 占位函数模式：`#[utoipa::path(get, path = "...", tag = "...")] async fn placeholder() {}`
- 参考 `crates/base/src/lib.rs` 的内联 utoipa 注解模式（用于少量关键端点的手动注解）

**Test scenarios:**
- Happy path: 运行生成脚本 → 生成的 lib.rs 包含 ~883 个占位函数
- Happy path: `cargo build -p openapi` 编译通过
- Happy path: 访问 `/openapi.json` → 返回包含 ~883 个 path item 的 OpenAPI JSON
- Happy path: 随机抽取 5 个 path item → 每个包含 tag、summary、parameters、responses
- Error path: 编译超时（utoipa derive 限制）→ 降级为运行时生成方案

**Verification:**
- `cargo build` 全量通过
- `/openapi.json` 端点返回合法 OpenAPI 3.x JSON
- JSON 可用 utoipa 或独立验证器校验合法性

---

### U7. 行为对比测试全覆盖

**Goal:** 将行为对比测试端点清单扩展至覆盖全部 ~883 个 Rust 端点，确保 Java 不可用时全部标记为 SKIP

**Requirements:** R12, R13, R14

**Dependencies:** U5（MCP 端点清单可作为参考）

**Files:**
- Modify: `tests/behavior_compare_endpoints.rs`（全量端点清单）
- Modify: `tests/behavior_compare.rs`（如有需要调整）
- Modify: `tests/behavior_comparison/allowlist.yaml`（扩展字段命名差异规则）
- Test: `tests/behavior_compare.rs`（全量端点测试）

**Approach:**
- 端点清单生成：编写或扩展现有脚本，从源码路由注册自动提取全部 ~883 个端点
- 每个端点条目：`crate_name`、`method`、`rust_path`、`java_war`、`java_action`、`body`（可选）、`requires_auth`
- java_war 映射：从端点路径前缀推断（如 `/jaxrs/calendar/` → `x_calendar_core_entity`）
- allowlist 扩展：覆盖 Rust camelCase 与 Java snake_case 的所有已知命名差异（当前仅覆盖时间戳字段）
- Java 不可用降级：已有 SKIP 机制，验证全量端点均能正确标记为 SKIP

**Patterns to follow:**
- 端点定义格式：参考现有 `tests/behavior_compare_endpoints.rs` 的 EndpointDef 结构
- 脚本风格：参考 `scripts/extract_endpoints.py`

**Test scenarios:**
- Happy path: 运行生成脚本 → behavior_compare_endpoints.rs 包含 ~883 个端点
- Happy path: Java 服务可用 → 部分端点 Pass/Fail，报告生成
- Happy path: Java 服务不可用 → 全部 ~883 个端点标记为 SKIP，测试通过
- Edge case: 新端点无 Java 映射 → 标记为 SKIP 并记录原因
- Error path: 端点清单与源码路由不一致 → 脚本输出警告

**Verification:**
- `cargo test --test behavior_compare` 通过（Java 不可用时全部 SKIP）
- `target/debug/behavior-report.md` 生成，无未标记的端点

---

### U8. todo crate 清理与全量集成验证

**Goal:** 调查并修复 todo 标记的 crate，确保 cargo test --workspace --lib 全部通过

**Requirements:** R19, R20

**Dependencies:** U1-U7（前序单元完成后）

**Files:**
- Modify: `docs/brainstorms/oa4rust-endpoint-inventory.md`（更新状态）
- Modify: `docs/brainstorms/oa4rust-migration-status.md`（更新状态）
- Modify: 各 todo crate 的 `src/lib.rs` 或 `src/routes.rs`（如有真实缺失）
- Test: `cargo test --workspace --lib`（全量回归）

**Approach:**
- 重新运行 `scripts/gen_inventory.py` 生成最新端点清单
- 对比当前 `oa4rust-endpoint-inventory.md` 与新生成结果，确认 todo 标记原因
- `calendar` crate：已有完整实现（585 行），检查路由是否正确注册到 main.rs
- `process_express` / `process_surface`：已有实现，检查路由注册和 handler 计数
- `mcp_server` / `openapi` / `shared`：这些是基础设施 crate，todo 标记可能为误报
- 确认为扫描口径问题：更新清单文件，标记为 done
- 确认为真实缺失：在前序单元中已覆盖（U1-U7）

**Patterns to follow:**
- 清单文件格式：保持与现有 `oa4rust-endpoint-inventory.md` 格式一致
- 状态更新：done/doing/todo 三态，每完成一个 crate 重新生成

**Test scenarios:**
- Happy path: 重新运行 gen_inventory.py → 所有 crate 标记为 done（除明确规划后续的新功能）
- Happy path: `cargo test --workspace --lib` 全部通过
- Happy path: `cargo build` 全量通过
- Error path: 发现真实缺失的 crate → 记录为 Deferred to Follow-Up Work

**Verification:**
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 中无 todo 标记的 crate
- `cargo test --workspace --lib` 100% 通过
- `cargo build` 无 warning（或仅有预期 warning）

---

## System-Wide Impact

- **Interaction graph：** U1-U3 扩展 auth crate，影响 `src/lib.rs` 的 router 组装；U4 扩展 personal/personal_extend crate，影响个人设置相关前端流程；U5-U6 扩展 mcp_server/openapi crate，影响 AI Agent 集成和 API 文档；U7 扩展测试套件，影响 CI 流水线
- **Error propagation：** 新增认证端点需正确传递 AppError（Unauthorized/Forbidden/Internal），中间件层自动转换为 HTTP 状态码
- **State lifecycle risks：** 安全注销需确保 SessionManager 的 session 移除是原子操作，避免并发注销导致 session 残留
- **API surface parity：** 新增端点需保持与 Java 端的响应结构一致（ActionResult<T> 9 字段），确保前端 o2web 无需适配
- **Integration coverage：** U7 行为对比测试是全量回归的安全网，确保新增端点与 Java 端行为等效
- **Unchanged invariants：** ActionResult<T> 9 字段结构不变；PermissionRegistry 扩展（新增路径注册）。SessionManager 需新增 `remove_sessions_by_person` 方法（U1 包含此修改）。

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| utoipa derive 宏处理 ~883 个路径（编译超时） | 备选方案：运行时生成 OpenAPI JSON，不依赖 derive 宏 |
| SSO 3DES 加解密与 Java 端不兼容 | 实现后手动验证：用 Java 加密的 token 在 Rust 端解密 |
| 用户注册唯一性校验并发竞态 | 数据库 UNIQUE 约束 + 应用层预检查，并发时由 DB 约束兜底 |
| MCP 工具桥接端点数量爆炸导致响应超时 | 分页返回工具列表，或按需加载（先全量注册，如有性能问题再优化） |
| 行为对比测试 ~883 个端点运行时间过长 | 并行执行端点对比，Java 不可用时 SKIP 不消耗网络时间 |
| personal_extend crate 的 avatar.rs | 已完整实现（MIME 校验、5MB 限制、magic bytes 验证），无需新增开发 |

---

## Documentation / Operational Notes

- 更新 `oa4rust/README.md`：添加新端点列表和 MCP/OpenAPI 使用说明
- 更新 `docs/brainstorms/oa4rust-endpoint-inventory.md`：重新生成，确认全量 done 状态
- 更新 `docs/brainstorms/oa4rust-migration-status.md`：同步更新
- 更新 `docs/oa/` 模块卡片：为新增认证端点补充 REST Endpoints 字段
- 运维：新增的安全注销和 SSO 端点需监控异常调用（防暴力破解）

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-10-oa4rust-comprehensive-gap-audit-requirements.md](docs/brainstorms/2026-08-10-oa4rust-comprehensive-gap-audit-requirements.md)
- **Related code:** `crates/auth/src/`、`crates/personal/src/`、`crates/personal_extend/src/`、`crates/mcp_server/src/tool_bridge.rs`、`crates/openapi/src/lib.rs`
- **Related scripts:** `scripts/gen_inventory.py`、`scripts/extract_endpoints.py`
- **Related solutions:** `docs/solutions/security-issues/idor-vulnerability-write-handlers.md`、`docs/solutions/architecture-patterns/actionresult-9-field-contract.md`
