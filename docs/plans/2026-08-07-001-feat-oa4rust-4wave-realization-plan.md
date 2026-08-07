---
title: feat: OA4Rust 4-wave full realization (80 crates to production)
type: feat
status: active
date: 2026-08-07
origin: docs/brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md
---

# OA4Rust 4 波次全量真实化计划

## Summary

以 4 个波次推进全部 80 个 crate 的真实业务逻辑落地：Wave 0 完成迁移安全准备与测试框架，Wave 1 完成安全加固与 6 个完整 crate 加固，Wave 2 补全 attendance/calendar/file/general_assemble_control 等 11 个 crate，Wave 3 实现 meeting/portal/process/query/cms 等核心工作流，Wave 4 完成剩余边缘模块。每波次完成后更新 `docs/brainstorms/oa4rust-migration-status.md` 和 `docs/brainstorms/oa4rust-endpoint-inventory.md` 作为单一信息源追踪进度。

---

## Problem Frame

O2OA 后端当前 100% 基于 Java（Maven 55+ 模块），oa4rust 已将全部 55 个 Java 模块映射为 80 个 Rust crate，编译通过且全部已挂载到 `main.rs`。当前实际状态：6 个 crate 具备完整真实业务逻辑（auth/control/personal/personal_extend/message/program_init），20 个 crate 含部分真实 PostgreSQL 查询，54 个 crate 仍为桩代码或 mock 数据。团队无法在 Rust 后端上推进任何实际工作，迁移停滞的代价是持续维护 Java 运行时和技术栈锁定。本次计划要求全部 80 个 crate 的每一个端点实现真实业务逻辑，无任何桩代码残留。

---

## Requirements

**全模块全功能全路由真实化**
- R4. 全部 80 个 crate 的每一个端点都必须接入 PostgreSQL 真实业务逻辑，无任何永久性桩代码残留。实施时按业务关键性和依赖关系分优先级推进
- R5. 每个 crate 的全量路由按对应 Java 模块的 JAX-RS 控制器端点逐一实现，功能上与 Java 端等效（字段名、类型、非空约束、分页/游标/排序语义保持一致），允许 Rust 实现在不破坏前端契约的前提下修复错误、简化逻辑和改进性能
- R6. 组织控制模块（人员、单位、角色、用户组）的 CRUD 端点返回真实数据库数据
- R7. 个人信息模块（个人信息查询/更新、密码管理、头像管理）的端点返回真实数据
- R8. 文件管理模块（上传/下载/目录管理/权限）的端点返回真实数据
- R9. 消息服务模块（消息消费/创建/已读/未读）的端点返回真实数据
- R10. 会议管理模块（会议创建/查询/参与人/日程）的端点返回真实数据
- R11. 考勤模块（打卡/统计/排班/请假）的端点返回真实数据
- R12. CMS 模块（栏目/文章/字典/索引）的端点返回真实数据
- R13. 查询/报表模块（视图/统计/导入/导出）的端点返回真实数据
- R14. 门户模块（页面/部件/脚本/字典）的端点返回真实数据
- R15. 流程引擎模块（应用/表单/流程/任务/工作/快照/签名）的端点返回真实数据
- R16. BBS/论坛模块（分类/文章/版主/搜索）的端点返回真实数据
- R17. 日历模块（日程/事件/共享）的端点返回真实数据
- R18. 组件管理模块（应用中心/应用市场/部署）的端点返回真实数据
- R19. AI 引擎模块（模型/推理/训练）的端点返回真实数据
- R20. 思维导图模块（创建/编辑/共享）的端点返回真实数据
- R21. 推送服务模块（消息推送/设备管理）的端点返回真实数据
- R22. 热点图片模块（轮播/推荐）的端点返回真实数据
- R23. 程序中心模块（应用管理/配置）的端点返回真实数据
- R24. 控制台模块（命令行/日志/监控）的端点返回真实数据
- R25. 通用服务模块（序列号/权限/配置）的端点返回真实数据
- R26. 关联关系模块（数据关联/引用）的端点返回真实数据
- R27. 表达式引擎模块（组织/流程/CMS/查询脚本）的端点返回真实数据
- R28. BAM 监控模块（业务活动监控）的端点返回真实数据
- R29. 系统初始化模块（密码设置、初始化检查）的端点返回真实数据
- R30. 快递查询模块（物流追踪）的端点返回真实数据

**测试要求**
- R31. 每个 crate 必须包含单元测试，覆盖核心业务逻辑
- R32. 核心业务流程必须包含集成测试，核心业务流程集成测试覆盖率 ≥ 80%
- R33. 必须建立 Rust 端点与 Java 端点的行为对比测试机制，确保功能等效性

**认证模块完善**
- R36. 验证码端点返回真正的验证码图片（本地生成，集成 captcha 库）
- R37. OAuth 端点实现第三方登录对接（微信、钉钉）
- R38. 认证流程完整可用（登录 → 会话 → 登出 → 刷新令牌）

**安全需求**
- R39. 除健康检查及认证入口端点（登录、验证码、OAuth 授权、令牌刷新等）外，所有端点强制认证（登录用户方可访问）
- R40. 除 R39 exempted 的未认证端点外，每个端点必须实施基于用户角色、用户组和资源所有者的访问控制，明确每个模块的权限边界
- R41. 所有输入端点进行参数验证（类型、长度、格式），拒绝无效输入
- R42. 认证接口速率限制（10次/分钟/IP），普通接口速率限制（100次/分钟/IP）
- R43. 所有响应强制 HTTPS（TLS 1.2+），生产环境返回安全的响应头（HSTS, X-Content-Type-Options）
- R44. 敏感数据（PII、凭证、组织数据）必须加密存储，传输使用 TLS，日志中禁止记录敏感字段
- R45. OAuth 客户端密钥和 API Key 必须存储在环境特定的密钥管理器中，禁止硬编码，支持密钥轮换和环境隔离
- R46. OAuth 集成必须验证 state 参数、使用 PKCE（若提供者支持）、验证提供者签名，并具备备用认证方案
- R47. 验证码库集成必须验证来源完整性，优雅处理生成失败，不在错误信息中泄露实现细节
- R48. 会话令牌必须加密签名、设置短期过期时间；刷新令牌必须安全存储（HttpOnly、Secure cookies）；令牌在刷新时必须轮换；登出时立即失效；禁止使用 localStorage 存储令牌

**迁移策略**
- R49. 沿用 Strangler Fig 渐进式迁移策略：Rust 与 Java 并行运行，通过 nginx 反向代理按 URL 前缀路由，逐步切换流量
- R50. 迁移进度通过 `docs/brainstorms/oa4rust-migration-status.md` 模块跟踪清单持续反映，每个模块标记为待迁移 / 迁移中 / 已完成
- R51. 必须制定回滚计划：定义触发回滚的条件（数据损坏、性能下降超过阈值等）、回滚流程（切回 Java）、以及用于即时切换的特性开关
- R52. 双轨运行期间必须进行数据库访问模式分析：记录事务隔离级别、识别并发写入风险、对正在迁移的表实施数据校验或禁用 Java 写入
- R53. 迁移前必须对已实现真实业务逻辑的 6 个 crate（auth、control、personal、personal_extend、message、program_init）进行行为测试，确认与 Java 后端一致后再作为其余 74 个 crate 的参考基准

**Origin actors:** A1（开发者，单人）、A2（现有 Java 后端）、A3（前端 o2web）
**Origin flows:** F1（模块梳理与优先级排序）、F2（Rust 服务独立开发与测试）、F3（数据迁移与流量切换）
**Origin acceptance examples:** AE1（Covers R4, R5, R6 — CRUD 端点返回真实数据库数据）、AE2（Covers R36 — 验证码端点返回生成的验证码图片）、AE3（Covers R37 — OAuth 端点返回有效的第三方授权 URL）、AE4（Covers R40 — 无权限返回 403）、AE5（Covers R31, R32 — 集成测试覆盖率 ≥ 80%）、AE6（Covers R5 — 端点清单文档驱动实施）、AE7（Covers R51 — 回滚程序在 RTO 内切回 Java）、AE8（Covers R5 — Rust 修复 Java bug 保持前端契约）

---

## Scope Boundaries

- 原则上不修改前端 `o2web` 的代码，仅通过 URL 前缀路由适配后端切换。若后端响应格式调整导致前端展示异常，允许在文档化并经前端团队确认后实施最小化前端适配，但核心业务逻辑和页面代码不得迁移
- 不在改写期间实现 Java ↔ Rust 的实时数据同步，仅依赖一次性迁移窗口
- 不拆分为微服务，Rust 侧始终以单一进程单体服务运行
- 不包含 Rust 性能压测或与 Java 的基准对比
- 不迁移 `o2web` 前端核心代码，该部分保持现状
- Java 服务的永久下线脚本属于后续阶段，但必须制定迁移期间的 rollback 程序和特性开关，确保出现问题时可快速切回 Java
- 不进行数据库 schema 变更或迁移脚本编写（沿用现有计划中的 schema；允许将重复/废弃的迁移文件归档到 `migrations/archive/` 目录）
- 全部 80 个 crate 的桩代码必须在真实化阶段全部清除，不允许任何 crate 以桩代码状态进入生产
- 会话持久化使用现有 `auth_session` 表，不创建新表（通过复用现有表结构实现）

### Deferred to Follow-Up Work

- Java 服务的下线和完全切换（后续阶段）
- 性能压测与基准对比
- 文件存储（本地/NAS/对象存储）的迁移方案
- 定时任务/批处理框架的 Rust 迁移

---

## Context & Research

### Relevant Code and Patterns

- **main.rs 路由注册**：`oa4rust/src/main.rs` 已 merge 全部 80 个 crate 的 Router
- **共享基础设施**：`oa4rust/crates/shared/src/` 提供 `SecurityState`（SessionManager + RateLimiter + Pool）、`auth_middleware`、`authorize_middleware`、`rate_limit_middleware`、`security_headers_middleware`、`ActionResult<T>` 响应包装、`AppError` 统一错误处理
- **数据库连接**：`deadpool_postgres::Pool` 通过 `DATABASE_URL` 环境变量初始化，所有 crate 共享同一 Pool
- **已完成真实化 crate**：`auth`（登录/登出/验证码/OAuth/刷新令牌/组织查询）、`control`（人员/单位/角色/用户组完整 CRUD）、`personal`（个人信息/密码修改/重置）、`personal_extend`（头像上传/个人详情）、`message`（消息消费/创建/已读/未读/计数）、`program_init`（系统初始化检查/设置/取消）
- **部分真实化 crate**：`attendance`（3 个 handler 有真实 DB 查询）、`calendar`（3 个 handler）、`file`（3 个 handler）、`general_assemble_control`（大量 handler 含真实 DB 查询）、`meeting_core_entity`、`meeting_assemble_control`、`bbs`、`correlation`、`general`、`hotpic`、`ai`、`component`、`organization_*`、`portal_*`、`process_*`、`query_*`、`cms_*` 等共 20 个 crate 含部分真实 PostgreSQL 查询
- **桩代码模式**：返回 `ActionResult::success(Value::Null)` 或硬编码 mock JSON；函数命名约定 `stub_{crate}_{handler}`
- **迁移文件**：`oa4rust/migrations/` 下 001-007，其中 001 与 003 重复建表，005 为最新增强版（含 `auth_group`、`deleted_at`、`avatar`、`icon` 列）
- **测试模式**：每个 crate 有 `src/tests.rs`，auth/control/personal/message 有较完整测试；大量使用惰性 Pool builder 避免测试依赖真实 DB

### Institutional Learnings

- **单一信息源原则**：`docs/brainstorms/oa4rust-migration-status.md` 是迁移进度的权威来源，每完成一个模块后必须立即更新
- **Axum 0.8 升级**：已完成，所有 crate 已使用 `{param}` 语法
- **会话持久化缺失**：当前 `SessionManager` 为纯内存 HashMap，重启即失效，生产环境需持久化（Redis 或 DB 表），多实例部署必现问题
- **路由冲突**：`control` 与 `auth` 重复注册 `/jaxrs/person/list`、`/jaxrs/unit/list` 等；`GET /jaxrs/person/{flag}` 与 `GET /jaxrs/person/{id}` 路径规范化后冲突
- **认证绕过漏洞**：现有 `POST /jaxrs/authentication/bind` 直接按 `unique_id` 查询并签发会话，无密码/授权校验，必须在完整流程实现前从路由中移除
- **密码哈希双算法兼容**：新写入使用 bcrypt（带方案前缀），校验路径同时支持 bcrypt 与既有 MD5/DES，登录成功后自动 rehash
- **响应格式硬约束**：前端 `action.js` 依赖 `ActionResult<T>` 的 9 字段 JSON 结构（`data, type, message, date, spent, size, count, position, prompt`），业务错误返回 HTTP 200 + `type=error`，HTTP 状态码仅用于传输层错误（401/403/429）
- **幂等迁移模式**：数据迁移使用 `INSERT ON CONFLICT` 支持幂等重跑；四步切换流程（数据迁移 → 部署 → 切流 → 观察），每步之间允许回滚

### External References

- Axum 0.8 migration guide: https://docs.rs/axum/0.8.0/axum
- SQLx 0.7 patterns: https://docs.rs/sqlx/0.7/sqlx
- Deadpool-postgres 0.12: https://docs.rs/deadpool-postgres/0.12

---

## Key Technical Decisions

- **分层分波推进**：80 个 crate 按业务优先级分为 4 波（Wave 0: 迁移安全准备 → Wave 1: 6 个完整 crate 加固 → Wave 2: attendance/calendar/file/general → Wave 3: meeting/portal/process/query/cms → Wave 4: 边缘模块），每波内 crate 可并行，波间有依赖
- **_core_entity vs _assemble_control 边界**：`_core_entity` 负责纯 CRUD，`_assemble_control` 负责业务编排调用 `_core_entity` + 其他服务；职责重叠的 crate（如 meeting）在真实化时明确拆分
- **响应格式硬约束**：前端 `action.js` 依赖 `ActionResult<T>` 的 9 字段 JSON 结构，任何 Rust 端点修改响应格式必须保持字段兼容
- **端点清单驱动实施**：`docs/brainstorms/oa4rust-endpoint-inventory.md` 是实施的权威依据，每个 crate 实现前必须对照端点清单逐条实现
- **渐进式迁移**：沿用 Strangler Fig 策略，Rust 与 Java 并行运行，通过 nginx 反向代理按 URL 前缀路由，逐步切换流量
- **权限默认拒绝**：所有端点默认拒绝访问，通过白名单机制开放特定角色/用户组的访问权限
- **会话持久化复用现有表**：使用现有 `auth_session` 表实现会话持久化，不创建新表

---

## Open Questions

### Resolved During Planning

- **80 crate 的当前状态**：已通过 repo research 确认（6 完整真实化、20 部分真实化、54 桩代码）
- **Axum 版本**：已升级至 0.8，所有 crate 已使用 `{param}` 语法
- **CORS 中间件**：已在 `shared/middleware.rs` 实现
- **MODULE_ROUTING feature flag**：已在 `shared/middleware.rs` 实现
- **bind 认证绕过**：已从 AUTH_EXEMPT_PATHS 移除
- **消息模块**：5 个端点已全部实现真实 DB 查询
- **U1 文件路径**：已修正为实际存在的文件路径
- **R30 归属**：已确认由 U9 的 express crate 实现

### Deferred to Implementation

- **各 crate 的具体 SQL 查询细节**：需在实现时对照 Java 源码确认字段映射和业务逻辑
- **复杂流程引擎的简化策略**：U7 允许先实现只读查询，再实现写操作，具体哪些端点简化需在实施时决定
- **AI 模块的外部服务对接**：允许对接外部服务或返回模拟结果，具体方案需在实施时决定
- **部分 crate 的 Java 源码不可获取**：允许功能等效而非逐字节一致，优先保证前端契约
- **具体密钥管理器选型**：需在 U1 实施时根据基础设施情况选择（Vault、云 KMS 或环境变量加密）
- **授权规则集中管理方式**：需在 U1 实施时确定（数据库表、配置文件或代码注解）

---

## Implementation Units

### 前置单元：迁移安全准备

#### U0. Migration safety preparation and testing framework

**Goal:** 在 Wave 1 开始前完成迁移安全准备和行为对比测试框架搭建

**Requirements:** R33, R51, R52, R53

**Dependencies:** None

**Files:**
- Create: `oa4rust/tests/behavior_comparison/`（Rust vs Java 行为对比测试框架）
- Create: `docs/ops/rollback-plan.md`（回滚计划文档）
- Create: `docs/ops/db-access-analysis.md`（数据库访问模式分析文档）
- Modify: `oa4rust/crates/shared/src/middleware.rs`（添加行为对比测试中间件）

**Approach:**
- 行为对比测试框架：建立 Rust 端点与 Java 端点的并行调用机制，记录响应差异并生成对比报告
- 回滚计划：定义触发回滚的条件（数据损坏、性能下降超过阈值、错误率突增）、回滚流程（切回 Java）、以及用于即时切换的特性开关
- 数据库访问模式分析：记录事务隔离级别、识别并发写入风险、对正在迁移的表实施数据校验或禁用 Java 写入
- 6 个已真实化 crate 的行为测试：对照 Java 后端验证功能等效性，建立参考基准

**Test scenarios:**
- Happy: 行为对比框架能同时调用 Rust 和 Java 端点并生成差异报告
- Happy: 6 个基准 crate 的行为测试通过，确认与 Java 等效
- Edge: Java 端点不可用时，对比框架能优雅降级
- Error: 回滚开关触发后，流量在 5 分钟内切回 Java

**Verification:**
- 行为对比测试框架搭建完成
- 回滚计划文档通过评审
- 数据库访问分析报告完成
- 6 个基准 crate 的行为测试通过

---

### Wave 1：基础设施收尾 + 6 个完整 crate 加固

#### U1. Security hardening, RBAC framework, and auth refinement

**Goal:** 完成生产级安全加固：建立 RBAC 框架、输入验证框架、完善 OAuth 安全实现、优化会话管理

**Requirements:** R36, R37, R38, R39, R40, R41, R42, R44, R45, R46, R47, R48

**Dependencies:** U0

**Files:**
- Modify: `oa4rust/crates/shared/src/session.rs`
- Create: `oa4rust/crates/auth/src/session_store.rs`
- Modify: `oa4rust/crates/shared/src/rate_limit.rs`
- Modify: `oa4rust/crates/auth/src/captcha.rs`
- Modify: `oa4rust/crates/auth/src/oauth.rs`
- Modify: `oa4rust/crates/auth/src/password.rs`
- Create: `oa4rust/crates/shared/src/input_validation.rs`（统一输入验证框架）
- Modify: `oa4rust/crates/shared/src/middleware.rs`（RBAC 中间件增强）
- Test: `oa4rust/crates/auth/src/tests.rs`

**Approach:**
- RBAC 框架：在 `authorize_middleware` 中实现基于角色、用户组和资源所有者的访问控制，定义默认拒绝策略
- 输入验证框架：建立统一的参数验证机制（类型、长度、格式），所有端点强制执行
- OAuth 安全完善：实现 PKCE、提供商签名验证、备用认证方案
- 会话管理优化：保留现有 DB 持久化，增加刷新令牌轮换、会话审计、单点登录支持
- 验证安全加固生效：OAuth state 参数验证、验证码一次性使用、密码哈希 rehash、速率限制

**Patterns to follow:**
- 现有 `password.rs` 的双算法兼容模式
- 现有 `SessionManager` 的接口设计（create/validate/remove）
- `auth_person_role` + `auth_role` 的 admin 查询模式

**Test scenarios:**
- Happy: 登录后重启服务，session 仍有效（持久化验证）
- Happy: 密码为 MD5 的用户登录后，hash 升级为 bcrypt
- Happy: OAuth 缺少 state 参数返回 400
- Happy: OAuth PKCE 验证失败返回 400
- Happy: 验证码使用一次后第二次使用返回 400
- Edge: 并发登录同一用户，旧 session 失效（单点登录）
- Error: 会话过期后访问返回 401
- Error: 无权限访问 admin 端点返回 403
- Error: 速率超限返回 429
- Error: 输入参数不合法返回 400

**Verification:**
- `cargo test` auth 模块全部通过
- RBAC 中间件通过所有权限测试场景
- 输入验证框架覆盖所有端点的 DTO
- OAuth flow 在测试环境完整走通（微信/钉钉 mock 服务器）
- 密钥管理策略文档化

---

#### U2. Wave 1 crates boundary case hardening

**Goal:** 为 6 个已完整真实化的 crate（control/personal/personal_extend/message/program_init/auth）补充边界情况处理和测试覆盖

**Requirements:** R4, R5, R6, R7, R29, R31, R32

**Dependencies:** U1

**Files:**
- Modify: `oa4rust/crates/control/src/person.rs`
- Modify: `oa4rust/crates/control/src/group.rs`
- Modify: `oa4rust/crates/control/src/role.rs`
- Modify: `oa4rust/crates/control/src/unit.rs`
- Modify: `oa4rust/crates/personal/src/password.rs`
- Modify: `oa4rust/crates/personal_extend/src/avatar.rs`
- Modify: `oa4rust/crates/message/src/lib.rs`
- Modify: `oa4rust/crates/program_init/src/lib.rs`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- `control`：确保 `deleted_at IS NULL` 过滤在所有查询中生效；非法 flag 参数校验；空结果分页
- `personal`：密码规则校验（长度 6-64、复杂度）
- `personal_extend`：头像大小限制（5MB）、MIME 白名单验证
- `message`：补充边界情况测试（空 consumer、不存在 id 等）
- `program_init`：系统初始化状态检查边界情况
- 统一分页行为：确保所有 list 端点返回 `count`、`size`、`position` 字段

**Patterns to follow:**
- 现有 `control::person.rs` 的 `person_flag_clause` 多字段匹配
- 现有 `personal::reset.rs` 的 `ResetCodeStore` TTL 模式
- 现有 `personal_extend::avatar.rs` 的 multipart 解析

**Test scenarios:**
- Happy: 创建人员 → 查询 → 更新 → 删除 → 确认软删除后不可见
- Happy: 游标分页 `list_next` / `list_prev` 正确返回 `count`、`size`、`position`
- Edge: `GET /jaxrs/person/{flag}` 传入不存在的 flag 返回空
- Error: 删除不存在的记录返回 404
- Error: 密码长度不足返回 400
- Error: 上传非图片文件返回 400
- Integration: 登录后修改个人信息，再次登录验证变更生效

**Verification:**
- `cargo test` 本波次 crate 全部通过
- 集成测试覆盖率 ≥ 80%（core business flows）
- `docs/brainstorms/oa4rust-migration-status.md` 更新
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新

---

### Wave 2：attendance/calendar/file/general_assemble_control 补全

#### U3. Wave 2 part A: attendance and calendar

**Goal:** 补全 attendance 和 calendar 系列 crate 的真实业务逻辑

**Requirements:** R4, R5, R8, R11, R17, R31, R32

**Dependencies:** U1, U2

**Files:**
- Modify: `oa4rust/crates/attendance/src/lib.rs`
- Modify: `oa4rust/crates/attendance_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/attendance_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/calendar/src/lib.rs`
- Modify: `oa4rust/crates/calendar_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/calendar_core_entity/src/lib.rs`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- `attendance`：补全打卡记录查询、排班规则 CRUD、申诉流程（submit/audit/archive）
- `attendance_assemble_control`：补全考勤规则管理、排班管理、申诉审批流程
- `attendance_core_entity`：补全打卡记录、排班规则、申诉记录的 CRUD
- `calendar`：补全日历 CRUD（create/update/remove）、事件 CRUD（create/update/remove/list）
- `calendar_assemble_control`：补全 config/calendars 查询
- `calendar_core_entity`：补全 calendar CRUD
- 明确 `_core_entity`（纯 CRUD）与 `_assemble_control`（业务编排）边界

**Patterns to follow:**
- 现有 `attendance/src/lib.rs` 的 `list_admins` 查询模式
- 现有 `general_assemble_control/src/lib.rs` 的复杂查询模式（多表 JOIN）

**Test scenarios:**
- Happy: 创建日历事件 → 查询我的日历 → 更新事件 → 删除事件
- Happy: 提交打卡 → 查询打卡记录 → 申诉 → 审批通过
- Edge: 查询不存在的日历返回空
- Error: 无权限访问他人私有日历返回 403
- Error: 打卡时间超出允许范围返回 400

**Verification:**
- `cargo test` 本波次 crate 全部通过
- 日历/考勤核心流程端到端可走通
- `docs/brainstorms/oa4rust-migration-status.md` 更新
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新

---

#### U4. Wave 2 part B: file and general_assemble_control

**Goal:** 补全 file 和 general_assemble_control 系列 crate 的真实业务逻辑

**Requirements:** R4, R5, R8, R25, R31, R32

**Dependencies:** U1, U2, U3

**Files:**
- Modify: `oa4rust/crates/file/src/lib.rs`
- Modify: `oa4rust/crates/file_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/file_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/general_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/general_core_entity/src/lib.rs`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- `file`：文件上传（multipart、MIME 白名单、5MB 限制）、文件夹 CRUD、文件权限、文件下载
- `file_assemble_control`：config/storage/categories 查询
- `file_core_entity`：folder/file CRUD
- `general_assemble_control`：补全区域管理、二维码管理、安全 clearance、发票管理等剩余端点
- `general_core_entity`：补全通用配置、序列号、权限等端点

**Patterns to follow:**
- 现有 `file/src/lib.rs` 的 `folder_list_top` 查询模式
- 现有 `general_assemble_control/src/lib.rs` 的 `attendscope_*` CRUD 模式

**Test scenarios:**
- Happy: 创建文件夹 → 上传文件 → 查询文件列表 → 下载文件
- Happy: 创建区域 → 查询区域列表 → 更新区域 → 删除区域
- Edge: 上传超大文件返回 400
- Error: 删除不存在的文件返回 404
- Error: 无权限访问文件返回 403

**Verification:**
- `cargo test` 本波次 crate 全部通过
- 文件上传/下载在测试环境手动验证
- `docs/brainstorms/oa4rust-migration-status.md` 更新
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新

---

### Wave 3：核心工作流（meeting/portal/process/query/cms）

#### U5. Wave 3 part A: meeting and message

**Goal:** 补全 meeting 和 message 系列 crate 的真实业务逻辑

**Requirements:** R4, R5, R9, R10, R31, R32

**Dependencies:** U1, U2

**Files:**
- Modify: `oa4rust/crates/meeting/src/lib.rs`
- Modify: `oa4rust/crates/meeting_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/meeting_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/message_assemble_communicate/src/lib.rs`
- Modify: `oa4rust/crates/message_core_entity/src/lib.rs`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- `meeting`：补全 room_list/building_list/openmeeting_list_room 为真实 DB 查询
- `meeting_assemble_control`：补全会议 CRUD、参与人管理、日程关联
- `meeting_core_entity`：补全 room/meeting CRUD
- `message_assemble_communicate`：补全消息发送/接收/已读/未读/删除等端点
- `message_core_entity`：补全消息 CRUD

**Patterns to follow:**
- 现有 `message/src/lib.rs` 的消息 CRUD 模式
- 现有 `meeting/src/lib.rs` 的 create_meeting 查询模式

**Test scenarios:**
- Happy: 预约会议室 → 创建会议 → 查询会议列表 → 取消会议
- Happy: 发送消息 → 查询未读列表 → 标记已读
- Edge: 查询不存在的会议返回 404
- Error: 无权限访问他人会议返回 403

**Verification:**
- `cargo test` 本波次 crate 全部通过
- 会议预约/查询/取消正常
- 消息收发正常
- `docs/brainstorms/oa4rust-migration-status.md` 更新
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新

---

#### U6. Wave 3 part B: portal

**Goal:** 补全 portal 系列 crate 的真实业务逻辑

**Requirements:** R4, R5, R14, R31, R32

**Dependencies:** U1, U2

**Files:**
- Modify: `oa4rust/crates/portal/src/lib.rs`
- Modify: `oa4rust/crates/portal_assemble_designer/src/lib.rs`
- Modify: `oa4rust/crates/portal_assemble_surface/src/lib.rs`
- Modify: `oa4rust/crates/portal_core_entity/src/lib.rs`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- `portal`：补全 portal 查询（当前仍为 mock）
- `portal_assemble_designer`：补全页面设计器 CRUD、design 相关端点
- `portal_assemble_surface`：补全 surface preview/publish 等端点
- `portal_core_entity`：补全 page CRUD

**Patterns to follow:**
- 现有 `portal_assemble_designer/src/lib.rs` 的 page CRUD 模式
- 现有 `portal_assemble_surface/src/lib.rs` 的 surface CRUD 模式

**Test scenarios:**
- Happy: 创建门户页面 → 查询页面列表 → 更新页面 → 删除页面
- Happy: 发布 surface → 查询 surface 列表 → 预览 surface
- Edge: 查询不存在的页面返回 404
- Error: 无权限编辑页面返回 403

**Verification:**
- `cargo test` 本波次 crate 全部通过
- 门户页面发布/预览正常
- `docs/brainstorms/oa4rust-migration-status.md` 更新
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新

---

#### U7. Wave 3 part C: process engine

**Goal:** 补全 process 系列 crate 的真实业务逻辑，覆盖流程引擎核心工作流

**Requirements:** R4, R5, R15, R31, R32

**Dependencies:** U1, U2

**Files:**
- Modify: `oa4rust/crates/process_designer/src/lib.rs`
- Modify: `oa4rust/crates/processplatform_assemble_designer/src/lib.rs`
- Modify: `oa4rust/crates/processplatform_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/processplatform_service_processing/src/lib.rs`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- `process_designer`：补全流程设计器 CRUD、表单定义、路由配置
- `processplatform_assemble_designer`：补全流程应用预览、发布、删除
- `processplatform_core_entity`：补全 work/task/ticket CRUD、workcompleted 列表
- `processplatform_service_processing`：补全 process CRUD、instance 管理、cancel 等
- 流程引擎分阶段实施：先实现 CRUD 层（本单元），再实现引擎逻辑（U8）

**Patterns to follow:**
- 现有 `control` crate 的 CRUD + 分页模式
- 现有 `general_assemble_control` 的复杂查询模式（多表 JOIN）

**Test scenarios:**
- Happy: 创建流程应用 → 查询应用列表 → 预览流程
- Happy: 创建流程实例 → 查询任务列表
- Edge: 查询无权限的流程实例返回 403
- Error: 审批不存在的任务返回 404

**Verification:**
- `cargo test` 本波次 crate 全部通过
- 流程 CRUD 接口正常
- `docs/brainstorms/oa4rust-migration-status.md` 更新
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新

---

#### U8. Wave 3 part D: process engine workflow and query/cms

**Goal:** 完成流程引擎工作流逻辑和 query/cms 系列 crate 的真实业务逻辑

**Requirements:** R4, R5, R12, R13, R15, R31, R32

**Dependencies:** U1, U2, U5, U6, U7

**Files:**
- Modify: `oa4rust/crates/process_express/src/lib.rs`
- Modify: `oa4rust/crates/process_surface/src/lib.rs`
- Modify: `oa4rust/crates/process_bam/src/lib.rs`
- Modify: `oa4rust/crates/processplatform_assemble_bam/src/lib.rs`
- Modify: `oa4rust/crates/processplatform_assemble_surface/src/lib.rs`
- Modify: `oa4rust/crates/processplatform_core_express/src/lib.rs`
- Modify: `oa4rust/crates/query_assemble_designer/src/lib.rs`
- Modify: `oa4rust/crates/query_assemble_surface/src/lib.rs`
- Modify: `oa4rust/crates/query_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/query_core_express/src/lib.rs`
- Modify: `oa4rust/crates/query_express/src/lib.rs`
- Modify: `oa4rust/crates/query_service/src/lib.rs`
- Modify: `oa4rust/crates/query_service_processing/src/lib.rs`
- Modify: `oa4rust/crates/cms_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/cms_control/src/lib.rs`
- Modify: `oa4rust/crates/cms_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/cms_core_express/src/lib.rs`
- Modify: `oa4rust/crates/cms_express/src/lib.rs`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- `process_express`：补全任务列表、工作项操作、流程状态查询
- `process_surface`：补全流程实例查询、工作流状态
- `process_bam`：从桩代码实现 BAM 监控统计路由
- `processplatform_assemble_bam`：补全 BAM 监控统计路由
- `processplatform_assemble_surface`：补全 surface preview/publish 等端点
- `processplatform_core_express`：补全 work/task 操作（terminate/retract/processing 等）
- `query_assemble_designer`：从桩代码实现查询设计器 CRUD
- `query_assemble_surface`：从桩代码实现查询 surface CRUD
- `query_core_entity`：补全 item/view/import 查询的 CRUD
- `query_core_express`：从桩代码实现查询执行
- `query_express`：从桩代码实现查询列表
- `query_service`：从桩代码实现查询服务
- `query_service_processing`：从桩代码实现查询执行、结果导出
- `cms_assemble_control`：补全 config/sections 查询
- `cms_control`：从桩代码实现 CMS 控制端点
- `cms_core_entity`：补全 category/app/config 查询
- `cms_core_express`：从桩代码实现文章 CRUD
- `cms_express`：从桩代码实现 CMS 辅助端点

**Patterns to follow:**
- 现有 `control` crate 的 CRUD + 分页模式
- 现有 `query_core_entity/src/lib.rs` 的查询模式

**Test scenarios:**
- Happy: 创建流程实例 → 查询任务列表 → 审批通过 → 查看工作流状态
- Happy: 发布流程应用 → 查询应用列表 → 预览流程
- Happy: 创建查询视图 → 执行查询 → 导出结果
- Happy: 发布 CMS 文章 → 查询文章列表 → 撤回发布
- Edge: 查询无权限的流程实例返回 403
- Edge: 查询不存在的视图返回 404
- Error: 审批不存在的任务返回 404
- Error: 无权限访问查询返回 403

**Verification:**
- `cargo test` 本波次 crate 全部通过
- 核心业务流程（发起流程 → 审批 → 通知）端到端可走通
- 查询/报表功能正常
- CMS 文章发布/撤回正常
- `docs/brainstorms/oa4rust-migration-status.md` 更新
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新

---

### Wave 4：边缘模块（AI/BBS/组件/热点/推送/思维导图/快递/控制台/表达式/关联关系/组织/程序中心）

#### U9. Wave 4 part A: organization, program_center, and base

**Goal:** 完成组织、程序中心和基础服务模块的真实化

**Requirements:** R4, R5, R25, R26, R31, R32

**Dependencies:** U1, U2

**Files:**
- Modify: `oa4rust/crates/organization_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/organization_assemble_express/src/lib.rs`
- Modify: `oa4rust/crates/organization_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/organization_core_express/src/lib.rs`
- Modify: `oa4rust/crates/program_center/src/lib.rs`
- Modify: `oa4rust/crates/program_center_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/base/src/lib.rs`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- `organization_assemble_control`：补全组织相关业务编排
- `organization_assemble_express`：补全组织相关表达式
- `organization_core_entity`：补全组织/单位/人员关联 CRUD
- `organization_core_express`：补全组织表达式逻辑
- `program_center`：补全应用管理/配置端点
- `program_center_core_entity`：补全应用 CRUD
- `base`：补全基础服务端点

**Patterns to follow:**
- 现有 `control` 的 CRUD 模式
- 现有 `general_assemble_control` 的复杂查询模式

**Test scenarios:**
- Happy: 组织架构查询正常
- Happy: 程序中心应用列表查询正常
- Edge: 空列表返回正确分页结构
- Error: 无权限访问返回 403

**Verification:**
- `cargo test` 本波次 crate 全部通过
- `docs/brainstorms/oa4rust-migration-status.md` 更新
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新

---

#### U10. Wave 4 part B: AI, component, hotpic, jpush

**Goal:** 完成 AI、组件管理、热点图片、推送服务模块的真实化

**Requirements:** R4, R5, R18, R19, R21, R22, R31, R32

**Dependencies:** U1, U2

**Files:**
- Modify: `oa4rust/crates/ai/src/lib.rs`
- Modify: `oa4rust/crates/ai_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/ai_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/component/src/lib.rs`
- Modify: `oa4rust/crates/component_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/component_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/hotpic/src/lib.rs`
- Modify: `oa4rust/crates/hotpic_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/hotpic_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/jpush/src/lib.rs`
- Modify: `oa4rust/crates/jpush_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/jpush_core_entity/src/lib.rs`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- AI 模块：模型管理、推理调用（允许对接外部服务或返回模拟结果，但接口契约对齐）
- 组件管理：应用中心 CRUD、部署状态
- 热点图片：轮播图 CRUD、推荐列表
- 推送服务：设备管理、消息推送（允许对接第三方推送服务或记录推送日志）

**Patterns to follow:**
- 现有 `control` 的 CRUD 模式
- 现有 `general_assemble_control` 的复杂查询模式

**Test scenarios:**
- Happy: AI 模型列表查询正常
- Happy: 组件应用发布/查询正常
- Happy: 轮播图 CRUD 正常
- Happy: 推送设备注册/消息推送正常
- Edge: 空列表返回正确分页结构
- Error: 无权限访问返回 403

**Verification:**
- `cargo test` 本波次 crate 全部通过
- `docs/brainstorms/oa4rust-migration-status.md` 更新
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新

---

#### U11. Wave 4 part C: BBS, mind, correlation, express, general, console

**Goal:** 完成 BBS、思维导图、关联关系、表达式引擎、通用服务、控制台模块的真实化

**Requirements:** R4, R5, R16, R24, R25, R26, R27, R30, R31, R32

**Dependencies:** U1, U2

**Files:**
- Modify: `oa4rust/crates/bbs/src/lib.rs`
- Modify: `oa4rust/crates/bbs_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/bbs_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/mind/src/lib.rs`
- Modify: `oa4rust/crates/mind_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/mind_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/correlation/src/lib.rs`
- Modify: `oa4rust/crates/correlation_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/correlation_core_express/src/lib.rs`
- Modify: `oa4rust/crates/correlation_service_processing/src/lib.rs`
- Modify: `oa4rust/crates/express/src/lib.rs`
- Modify: `oa4rust/crates/general/src/lib.rs`
- Modify: `oa4rust/crates/general_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/console/src/lib.rs`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- BBS：分类/文章/版主/搜索
- 思维导图：创建/编辑/共享（允许使用 JSON 存储简化版）
- 关联关系：数据关联/引用 CRUD
- 快递查询：物流追踪（允许对接第三方 API 或返回模拟结果）
- 表达式引擎：组织/流程/CMS/查询脚本的解析和执行
- 通用服务：序列号/权限/配置
- 控制台：命令行/日志/监控（允许只读或简化）

**Patterns to follow:**
- 现有 `control` 的 CRUD 模式
- 现有 `general_assemble_control` 的复杂查询模式

**Test scenarios:**
- Happy: BBS 文章发布/查询正常
- Happy: 思维导图创建/编辑正常
- Happy: 快递查询返回正常结果
- Happy: 表达式解析执行正常
- Edge: 空列表返回正确分页结构
- Error: 无权限访问返回 403

**Verification:**
- `cargo test` 本波次 crate 全部通过
- `docs/brainstorms/oa4rust-migration-status.md` 更新
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新

---

## System-Wide Impact

- **Interaction graph:** 全部 80 个 crate 共享 `shared::SecurityState`（SessionManager + RateLimiter + Pool），Wave 1 修改中间件栈影响所有 crate 的请求处理顺序；Wave 2-Wave 4 修改各 crate handler 影响前端 `action.js` 的 `ActionResult<T>` 解析
- **Error propagation:** 统一 `AppError` → HTTP 状态码映射（400/401/403/404/429/500），业务错误保持 HTTP 200 + `type=error`
- **State lifecycle risks:** SessionManager 从内存迁移到 DB（U0）期间需保证平滑过渡（双写或灰度）；RateLimiter 内存泄漏风险需在 U1 修复
- **API surface parity:** 前端 `o2web` 的 `action.js` 依赖 9 字段 `ActionResult<T>` 结构，任何 Rust 端点修改响应格式必须保持字段兼容
- **Integration coverage:** 行为对比测试框架（U0）、中间件栈集成（U1）、Wave 1 边界情况测试（U2）、Wave 2-Wave 4 各 crate 的真实化需通过集成测试验证与 Java 的行为等效性
- **Unchanged invariants:** `ActionResult<T>` 的 9 字段 JSON 结构不变；`/health` 端点保持公开；`DATABASE_URL` 环境变量配置方式不变；单进程单体部署模型不变

---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| 部分 crate 的 Java 源码不可获取或逻辑复杂 | Medium | Low | 允许功能等效而非逐字节一致，优先保证前端契约 |
| 复杂模块（process engine）实现周期过长 | High | Medium | 分阶段（先 CRUD 后引擎逻辑），允许简化非核心路径 |
| 前端 `action.js` 对响应格式有隐式假设 | Medium | High | 建立 ActionResult 契约测试框架，每个 crate 实现时自动验证响应格式 |
| Java 与 Rust 并发写入导致数据不一致 | Low | High | 对正在迁移的表实施数据校验或禁用 Java 写入 |
| 行为对比测试无法覆盖所有边缘情况 | Medium | Medium | 优先覆盖核心业务流程，边界情况通过集成测试补充 |
| 部分 crate 的端点清单与实际代码不符 | Medium | Medium | 每次波次完成后立即更新 endpoint-inventory 文档 |
| 密钥管理策略执行不到位 | Medium | High | 在 U1 中明确具体方案和轮换流程，纳入 CI 检查 |
| OAuth 提供商签名验证实现复杂 | Medium | Medium | 优先实现微信/钉钉主流方案，其他提供商后续补充 |
| 80 个 crate 的权限配置工作量巨大 | High | Medium | 建立权限配置模板和自动化工具，减少手动配置 |

---

## Documentation / Operational Notes

- 每完成一个 crate 立即更新 `docs/brainstorms/oa4rust-migration-status.md`（单一信息源原则）
- 每完成一个 crate 立即更新 `docs/brainstorms/oa4rust-endpoint-inventory.md`（端点清单驱动实施）
- 端点清单文档在实施前作为权威依据，每个 crate 实现前必须对照清单逐条实现
- CI 配置增加：`cargo test`、集成测试覆盖率检查、TODO 标记检查（生产分支禁止 TODO）
- 每波次完成后执行一次回滚演练，记录存档
- 密钥管理策略、授权规则、日志策略等安全相关文档在 U1 完成前必须评审通过

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md](../brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md)
- **Migration status:** [docs/brainstorms/oa4rust-migration-status.md](../brainstorms/oa4rust-migration-status.md)
- **Endpoint inventory:** [docs/brainstorms/oa4rust-endpoint-inventory.md](../brainstorms/oa4rust-endpoint-inventory.md)
- Related code: `oa4rust/src/main.rs`, `oa4rust/crates/shared/src/`
- Related plans: [docs/plans/2026-08-06-001-feat-oa4rust-full-realization-plan.md](../plans/2026-08-06-001-feat-oa4rust-full-realization-plan.md)
- External docs: Axum 0.8, SQLx 0.7, Deadpool-postgres 0.12
