---
title: feat: OA4Rust full realization (80 crates to production)
type: feat
status: active
date: 2026-08-06
origin: docs/brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md
---

# OA4Rust 全量真实化计划

## Summary

以 Deep 分层推进全部 80 个 crate 的真实业务逻辑落地：先完成基础层升级与中间件加固（Axum 0.8 统一、迁移文件清理、认证/授权/限流/安全头中间件硬化），再按业务优先级分 4 波完成 crate 真实化（组织控制与个人信息 → 文件/日历/考勤/通用管控 → 流程/消息/会议/门户/查询/CMS → 其余基础设施模块），同步补全端点契约对齐、行为对比测试、回滚特性开关与 nginx 灰度路由配置，使 Rust 后端达到可完全替代 Java 运行的完整度。

---

## Problem Frame

O2OA 后端当前 100% 基于 Java（Maven 55+ 模块），oa4rust 已将全部 55 个 Java 模块映射为 80 个 Rust crate，编译通过且全部已挂载到 `main.rs`。但 71 个 crate 仍为桩代码（返回 `ActionResult::success(Value::Null)` 或硬编码 mock 数据），仅 4 个 crate（auth、personal、personal_extend、control）具备完整真实业务逻辑，4 个 crate（attendance、calendar、file、general_assemble_control）含部分真实 PostgreSQL 查询。团队无法在 Rust 后端上推进任何实际工作，迁移停滞的代价是持续维护 Java 运行时和技术栈锁定。本次计划要求全部 80 个 crate 的每一个端点实现真实业务逻辑，无任何桩代码残留，并通过 Strangler Fig 策略与 Java 双轨运行、灰度切换。

---

## Requirements

**路由与框架基础**
- R1. 全部 80 个 workspace crate 的路由已注册到 `main.rs`，需验证所有路由正确暴露且无冲突。对于尚未完全接入的 crate，完成中间件配置（认证、CORS、限流等）
- R2. 桩代码端点必须在代码中标记 `TODO: [module] - real implementation needed`，所有 TODO 标记在 sprint 规划中 review 并分配优先级
- R3. 已存在真实实现的 crate（control、personal_extend 等）优先接入，桩代码端点不得阻塞已实现端点的暴露

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

**端点清单与优先级**
- R34. 实施前必须建立端点清单文档，明确每个 crate 对应的 Java JAX-RS 端点列表、业务优先级和实现顺序
- R35. 优先实现高业务价值、低复杂度的 crate（建议首批 20 个 crate 覆盖 80% 核心用户工作流），验证通过后再推进其余 crate

**认证模块完善**
- R36. 验证码端点返回真正的验证码图片（本地生成，集成 captcha 库）
- R37. OAuth 端点实现第三方登录对接（微信、钉钉）
- R38. 认证流程完整可用（登录 → 会话 → 登出 → 刷新令牌）

**安全需求**
- R39. 除健康检查及认证入口端点（登录、验证码、OAuth 授权、令牌刷新等）外，所有端点强制认证（登录用户方可访问）
- R40. 除 R39 exempted 的未认证端点外，每个端点必须实施基于用户角色、用户组和资源所有者的访问控制，明确每个模块的权限边界（谁能读取/更新哪些资源）
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
- R53. 迁移前必须对已实现真实业务逻辑的 4 个 crate（auth、personal、personal_extend、control）进行行为测试，确认与 Java 后端一致后再作为其余 76 个 crate 的参考基准

**Origin actors:** A1（开发者，单人）、A2（现有 Java 后端）、A3（前端 o2web）
**Origin flows:** F1（模块梳理与优先级排序）、F2（Rust 服务独立开发与测试）、F3（数据迁移与流量切换）
**Origin acceptance examples:** AE1（Covers R4, R5, R6 — CRUD 端点返回真实数据库数据）、AE2（Covers R36 — 验证码端点返回生成的验证码图片）、AE3（Covers R37 — OAuth 端点返回有效的第三方授权 URL）、AE4（Covers R40 — 无权限返回 403）、AE5（Covers R31, R32 — 集成测试覆盖率 ≥ 80%）、AE6（Covers R34 — 端点清单文档驱动实施）、AE7（Covers R51 — 回滚程序在 RTO 内切回 Java）、AE8（Covers R5 — Rust 修复 Java bug 保持前端契约）

---

## Scope Boundaries

- 原则上不修改前端 `o2web` 的代码，仅通过 URL 前缀路由适配后端切换。若后端响应格式调整导致前端展示异常，允许在文档化并经前端团队确认后实施最小化前端适配，但核心业务逻辑和页面代码不得迁移
- 不在改写期间实现 Java ↔ Rust 的实时数据同步，仅依赖一次性迁移窗口
- 不拆分为微服务，Rust 侧始终以单一进程单体服务运行
- 不包含 Rust 性能压测或与 Java 的基准对比
- 不迁移 `o2web` 前端核心代码，该部分保持现状
- Java 服务的永久下线脚本属于后续阶段，但必须制定迁移期间的 rollback 程序和特性开关，确保出现问题时可快速切回 Java
- 不进行数据库 schema 变更或迁移脚本编写（沿用现有计划中的 schema；**允许将重复/废弃的迁移文件归档到 `migrations/archive/` 目录**）
- 全部 80 个 crate 的桩代码必须在真实化阶段全部清除，不允许任何 crate 以桩代码状态进入生产

### Deferred to Follow-Up Work

- Java 服务的下线和完全切换（后续阶段）
- 性能压测与基准对比
- 文件存储（本地/NAS/对象存储）的迁移方案
- 定时任务/批处理框架的 Rust 迁移

---

## Context & Research

### Relevant Code and Patterns

- **main.rs 路由注册**：`oa4rust/src/main.rs` 已 merge 全部 80 个 crate 的 Router，但 71 个 crate 的 handler 返回 stub 或硬编码 mock 数据
- **共享基础设施**：`oa4rust/crates/shared/src/` 提供 `SecurityState`（SessionManager + RateLimiter + Pool）、`auth_middleware`、`authorize_middleware`、`rate_limit_middleware`、`security_headers_middleware`、`ActionResult<T>` 响应包装、`AppError` 统一错误处理
- **数据库连接**：`deadpool_postgres::Pool` 通过 `DATABASE_URL` 环境变量初始化，所有 crate 共享同一 Pool
- **已实现 crate**：`auth`（登录/登出/验证码/OAuth/刷新令牌/组织查询）、`control`（人员/单位/角色/用户组完整 CRUD）、`personal`（个人信息/密码修改/重置）、`personal_extend`（头像上传/个人详情）
- **部分实现 crate**：`attendance`（3 个 handler 有真实 DB 查询）、`calendar`（3 个 handler）、`file`（3 个 handler）、`general_assemble_control`（大量 handler 含真实 DB 查询）、`meeting_core_entity`、`meeting_assemble_control`、`attendance_core_entity`、`attendance_assemble_control`
- **桩代码模式**：返回 `ActionResult::success(Value::Null)` 或硬编码 mock JSON；函数命名约定 `stub_{crate}_{handler}`
- **迁移文件**：`oa4rust/migrations/` 下 001-007，其中 001 与 003 重复建表，005 为最新增强版（含 `auth_group`、`deleted_at`、`avatar`、`icon` 列）
- **测试模式**：每个 crate 有 `src/tests.rs`，auth/control/personal 有较完整测试；大量使用惰性 Pool builder 避免测试依赖真实 DB

### Institutional Learnings

- **单一信息源原则**：`docs/brainstorms/oa4rust-migration-status.md` 是迁移进度的权威来源，每完成一个模块后必须立即更新（see origin: `docs/brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md`）
- **Axum 0.8 升级**：约 19 个 crate 仍使用 `:param` 旧语法，升级后路由会静默 404，必须在全量接入前统一转换为 `{param}`
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

- **分层分波推进**：80 个 crate 按业务优先级分为 4 波（组织控制/个人信息 → 文件/日历/考勤/通用管控 → 流程/消息/会议/门户/查询/CMS → 基础设施/AI/组件/热点），每波内 crate 可并行，波间有依赖
- **Axum 0.8 作为 U1 前置**：统一升级并转换 `:param` → `{param}`，避免后续 crate 接入时路由静默 404
- **会话互认延后到 U3**：双轨共存期间会话 token 互认策略（共享 JWT 密钥 / 共享会话存储 / 登录保留在 Java 侧）在 U3 前确定，U1-U2 期间不宣称互通
- **桩代码保留标记 TODO**：不删除 stub 函数，标记 `TODO: [module] - real implementation needed` 作为后续实现的明确占位，避免破坏前端契约。**TODO 生命周期规则**：(1) 所有 TODO 必须关联到具体的 wave 和 crate；(2) CI 中增加检查，生产分支不得包含 TODO 注释；(3) 每个波次完成时，该波次 crate 的 TODO 必须全部清除；(4) 若 crate 无法在当前波次完成，其 stub 端点必须通过 feature flag 禁用
- **_core_entity vs _assemble_control 边界**：`_core_entity` 负责纯 CRUD，`_assemble_control` 负责业务编排调用 `_core_entity` + 其他服务；职责重叠的 crate（如 meeting）在真实化时明确拆分
- **迁移文件清理**：将重复的 003/004 迁移归档到 `migrations/archive/`，保留 001 + 005 + 006 + 007 作为单一权威 schema
- **CORS 中间件在 U1 添加**：当前无 CORS 支持，前端跨域调用会失败，必须在首个 crate 暴露前启用

---

## Open Questions

### Resolved During Planning

- **80 crate 的当前状态**：已通过 repo research 确认（4 完整真实化、4 部分真实化、71 桩代码）
- **路由冲突清单**：已识别（control/auth 重复注册、person 路径冲突等）
- **Axum 版本**：当前 0.7.9，需升级至 0.8
- **共享基础设施能力**：auth/rate_limit/security_headers 中间件已存在，需加固（CORS、session 持久化）

### Deferred to Implementation

- **首批 20 个高价值 crate 的精确清单**：U4 实施时对照 Java 模块使用率数据确定
- **每个模块的具体权限边界**：U4-U7 各模块实施时根据 Java 源码逐模块定义
- **行为对比测试套件的具体实现方式**：U8 确定快照对比还是语义对比
- **回滚程序的精确 RTO**：U9 与运维团队对齐后定义
- **Java 写入禁用机制**：U9 根据双轨运行情况设计特性开关或 DB 触发器
- **OAuth 密钥管理器选型**：U3 根据团队基础设施选择（环境变量 / Vault / 云服务）
- **会话 token 格式**：U3 确定 JWT vs opaque token（当前为 UUID opaque token）

---

## Implementation Units

### U1. Foundation hardening (Axum 0.8 + middleware + migrations)

**Goal:** 将基础层升级到生产就绪状态：Axum 0.8 统一、CORS 中间件、迁移文件清理、安全头加固、路由冲突扫描工具

**Requirements:** R1, R39, R42, R43, R49

**Dependencies:** None

**Files:**
- Modify: `oa4rust/Cargo.toml`, `oa4rust/Cargo.lock`
- Modify: `oa4rust/src/main.rs`
- Modify: `oa4rust/crates/shared/src/middleware.rs`
- Modify: `oa4rust/crates/shared/src/db.rs`
- Create: `oa4rust/migrations/008_cleanup_duplicates.sql`
- Modify: `oa4rust/migrations/` (remove or archive 003/004)
- Create: `oa4rust/scripts/detect_route_conflicts.rs`
- Test: `oa4rust/crates/shared/src/tests.rs`

**Approach:**
- 升级 Axum 至 0.8，全仓搜索替换 `:param` → `{param}`（约 19 个 crate）
- 在 `shared/middleware.rs` 添加 CORS 中间件（允许 `o2web` 源、`Authorization`/`Content-Type` 头、`OPTIONS` 预检）
- 加固 `security_headers_middleware`：确保 `FORCE_HTTPS=true` 时执行 307 跳转；添加 `Referrer-Policy: strict-origin-when-cross-origin`
- 创建 `detect_route_conflicts.rs` 脚本：启动 Router 前扫描同 path+method 重复注册，CI 中运行
- 清理迁移文件：**将 003/004 移动到 `migrations/archive/` 目录**（而非删除），创建 008 记录清理操作；保留 001 + 005 + 006 + 007 作为权威 schema
- 修复 `auth` crate 中 `POST /jaxrs/authentication/bind` 认证绕过漏洞：**立即移除该路由**，确保漏洞在 U1 即被消除；U3 实现完整扫码流程后重新启用
- 修复 RateLimiter 内存泄漏：为滑动窗口添加定期清理机制（如 TTL 过期条目自动移除），防止长期运行内存线性增长

**Technical design:**
```
CORS layer order (outer → inner):
trace → security_headers → cors → rate_limit → auth → authorize → handler
```

**Patterns to follow:**
- 现有 `security_headers_middleware` 的 tower-layer 模式
- `rate_limit_middleware` 的滑动窗口实现
- Axum 0.8 `Router::route` 新语法

**Test scenarios:**
- Happy: CORS preflight (`OPTIONS`) 返回 204 + `Access-Control-Allow-Origin`
- Happy: 升级后所有参数化路由（`/jaxrs/person/{flag}` 等）正常匹配
- Error: 重复路由注册被 `detect_route_conflicts` 脚本捕获并报告
- Error: `FORCE_HTTPS=true` 时 HTTP 请求返回 307 到 HTTPS
- Integration: 中间件栈按正确顺序执行（outer → inner）

**Verification:**
- `cargo build` 无警告，`cargo test` 全部通过
- `detect_route_conflicts` 脚本运行无冲突报告
- 所有 `:param` 语法已清除

---

### U2. Route conflict resolution and main.rs cleanup

**Goal:** 修复 main.rs 中已知路由冲突，统一 crate 间重复注册的路径，确保服务启动不 panic

**Requirements:** R1, R3

**Dependencies:** U1

**Files:**
- Modify: `oa4rust/src/main.rs`
- Modify: `oa4rust/crates/auth/src/lib.rs`
- Modify: `oa4rust/crates/control/src/lib.rs`
- Modify: `oa4rust/crates/cms_control/src/lib.rs`
- Modify: `oa4rust/crates/cms_express/src/lib.rs`
- Test: `oa4rust/src/tests.rs` (integration smoke test)

**Approach:**
- `control` 与 `auth` 重复注册的路径：保留 `control` 的 CRUD 实现（更完整），`auth` 中移除重复的 list/get 路由；`auth` 保留登录/组织查询等专属路径
- `GET /jaxrs/person/{flag}`（auth）与 `GET /jaxrs/person/{id}`（control）路径冲突：统一为 `GET /jaxrs/person/{flag}` 由 control 实现，auth 移除重复
- `cms_control` 与 `cms_express` 重复注册 `GET /jaxrs/cms/view/list/all`：保留 `cms_express` 实现，`cms_control` 移除
- `control` 自身重复注册的 `/health`：移除 control 中的 health 路由（由 shared 提供）
- 添加集成冒烟测试：启动 Router，验证所有路由可正常匹配且无 panic

**Technical design:**
```
路由归属决策：
- /jaxrs/person/* (CRUD) → control
- /jaxrs/unit/* (CRUD) → control
- /jaxrs/role/* (CRUD) → control
- /jaxrs/group/* (CRUD) → control
- /jaxrs/authentication/* (登录/登出/验证码/OAuth) → auth
- /jaxrs/person (当前用户信息) → personal
- /jaxrs/personal/* → personal_extend
```

**Patterns to follow:**
- 现有 `main.rs` 的 merge 模式
- `auth::router()` 和 `control::router()` 的返回值约定

**Test scenarios:**
- Happy: `cargo test` 集成冒烟测试通过，Router 构建不 panic
- Happy: 所有已知重复路由仅有一个 handler 注册
- Edge: 空数据库时 Router 仍可正常构建
- Error: 若未来 crate 引入新冲突，`detect_route_conflicts` 在 CI 中捕获

**Verification:**
- `cargo build` 成功，服务启动日志无路由冲突 panic
- 集成冒烟测试覆盖所有已注册路由的基本可达性

---

### U3. Auth security hardening and session persistence

**Goal:** 完成认证模块生产级加固：修复已知安全漏洞、实现会话持久化、完善 OAuth/验证码安全、统一密码哈希 rehash 机制

**Requirements:** R36, R37, R38, R39, R42, R44, R45, R46, R47, R48, R53

**Dependencies:** U1, U2

**Files:**
- Modify: `oa4rust/crates/auth/src/session.rs`
- Modify: `oa4rust/crates/auth/src/handlers/login.rs`
- Modify: `oa4rust/crates/auth/src/handlers/captcha.rs`
- Modify: `oa4rust/crates/auth/src/handlers/oauth.rs`
- Modify: `oa4rust/crates/auth/src/handlers/bind.rs`
- Modify: `oa4rust/crates/auth/src/password.rs`
- Modify: `oa4rust/crates/shared/src/middleware.rs`
- Create: `oa4rust/crates/auth/src/session_store.rs`
- Test: `oa4rust/crates/auth/src/tests.rs`

**Approach:**
- 会话持久化：将 `SessionManager` 从纯内存 HashMap 迁移到 PostgreSQL `auth_session` 表（已建未用），支持多实例部署和重启恢复；保留内存缓存层做热读
- 修复认证绕过：`POST /jaxrs/authentication/bind` 在 U1 已移除，U3 实现完整扫码流程后重新启用（`GET /jaxrs/authentication/bind` 返回二维码 + `GET/POST /jaxrs/authentication/bind/meta/{meta}` 轮询确认）
- 密码哈希 rehash：登录成功后检测旧算法（MD5/DES），自动 rehash 为 bcrypt；写入新用户统一使用 bcrypt
- OAuth 安全加固：验证 `state` 参数、支持 PKCE（若提供者支持）、验证提供者签名、添加备用认证方案（短信验证码）
- 验证码安全加固：一次性使用（验证后立即删除）、验证失败不泄露是用户名不存在还是验证码错误
- 令牌格式：会话 token 使用加密签名（HMAC-SHA256），设置 2 小时过期；**刷新令牌存储在 `auth_session` 表**，刷新时轮换；登出时立即失效
- 速率限制：认证端点 10 次/分钟/IP（已有），普通端点 100 次/分钟/IP（已有），验证失败额外记录失败次数

**Technical design:**
```
auth_session 表结构：
- token (PK, encrypted)
- person_unique (FK)
- created_at
- expires_at
- refreshed_at (for refresh token rotation)
- user_agent / ip (audit)
- revoked (boolean, for logout)
```

**Patterns to follow:**
- 现有 `password.rs` 的双算法兼容模式
- 现有 `SessionManager` 的接口设计（create/validate/remove）
- `auth_person_role` + `auth_role` 的 admin 查询模式

**Test scenarios:**
- Happy: 登录后重启服务，session 仍有效（持久化验证）
- Happy: 密码为 MD5 的用户登录后，hash 升级为 bcrypt
- Happy: OAuth 缺少 state 参数返回 400
- Happy: 验证码使用一次后第二次使用返回 400
- Edge: 并发登录同一用户，旧 session 失效（单点登录）
- Error: 会话过期后访问返回 401
- Error: 无权限访问 admin 端点返回 403
- Error: 速率超限返回 429

**Verification:**
- `cargo test` auth 模块全部通过
- 手动验证：登录 → 重启服务 → 访问接口仍正常
- OAuth flow 在测试环境完整走通（微信/钉钉 mock 服务器）
- 行为对比测试：auth 端点响应与 Java 后端一致

---

### U4. Priority wave 1: Organization control & personal info (control, personal, personal_extend, program_init)

**Goal:** 完成组织控制、个人信息、系统初始化模块的真实化，这些模块已有较完整实现，作为后续波次的模板

**Requirements:** R4, R5, R6, R7, R29, R31, R32, R53

**Dependencies:** U1, U2, U3

**Files:**
- Modify: `oa4rust/crates/control/src/` (person.rs, group.rs, role.rs, unit.rs, pagination.rs)
- Modify: `oa4rust/crates/personal/src/` (person.rs, password.rs, reset.rs)
- Modify: `oa4rust/crates/personal_extend/src/` (personal.rs, avatar.rs, password.rs)
- Modify: `oa4rust/crates/program_init/src/` (secret.rs)
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- `control`：已有完整 CRUD，补充边界情况处理（软删除过滤、空结果分页、非法 flag 参数校验）；确保 `deleted_at IS NULL` 过滤在所有查询中生效
- `personal`：已有个人信息查询/更新/密码修改/重置，补充密码规则校验（长度 6-64、复杂度）、头像上传 MIME 白名单加固
- `personal_extend`：已有头像上传/个人详情，补充头像大小限制（5MB）、格式验证
- `program_init`：`/jaxrs/secret/*` 端点从内存状态迁移到 `secret_config` 表持久化，添加系统初始化状态检查（`auth_person` 表是否为空）
- 统一分页行为：确保所有 list 端点返回 `count`（总数）、`size`（当前页大小）、`position`（`next`/`prev`）字段
- 为每个 crate 补充至少 2 个集成测试（happy path + error path）
- **Java 写入禁用策略**：本波次开始前，对 wave 1 涉及的 crate（control, personal, personal_extend, program_init）对应的数据库表，通过 DB 触发器或应用层特性开关禁用 Java 写入，U9 完善监控和回滚

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
- Integration: 登录后修改个人信息，再次登录验证变更生效

**Verification:**
- `cargo test` 本波次 crate 全部通过
- 集成测试覆盖率 ≥ 80%（core business flows）
- 端点清单文档中 wave 1 的 4 个 crate 标记为"已完成"

---

### U5. Priority wave 2: File, calendar, attendance, general_assemble_control

**Goal:** 完成文件管理、日历、考勤、综合管控模块的真实化，这些模块已有部分 PostgreSQL 查询，需补充完整业务逻辑

**Requirements:** R4, R5, R8, R11, R12, R17, R25, R31, R32, R53

**Dependencies:** U1, U2, U3, U4

**Files:**
- Modify: `oa4rust/crates/file/src/lib.rs`
- Modify: `oa4rust/crates/file_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/file_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/calendar/src/lib.rs`
- Modify: `oa4rust/crates/calendar_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/calendar_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/attendance/src/lib.rs`
- Modify: `oa4rust/crates/attendance_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/attendance_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/general_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/general_core_entity/src/lib.rs`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- `file` / `file_assemble_control` / `file_core_entity`：文件上传（multipart、MIME 白名单、5MB 限制）、文件夹 CRUD、文件权限、文件下载；参考 Java `x_file_assemble_control`
- `calendar` / `calendar_assemble_control` / `calendar_core_entity`：日历 CRUD、事件 CRUD、共享权限；已有 `calendar_list_public`、`calendar_list_my`、`calendar_get` 真实查询，补充其余端点
- `attendance` / `attendance_assemble_control` / `attendance_core_entity`：打卡记录、排班规则、统计周期、申诉流程；已有 `list_admins`、`list_employee_configs`、`list_statistical_cycles` 真实查询，补充其余端点
- `general_assemble_control` / `general_core_entity`：综合管控配置、参会范围、区域管理、发票、二维码、安全 clearance；已有大量真实查询，补充剩余端点
- 明确 `_core_entity`（纯 CRUD）与 `_assemble_control`（业务编排）边界，避免职责重叠
- 所有新增端点遵循 `ActionResult` 包装和分页约定

**Patterns to follow:**
- 现有 `file/src/lib.rs` 的 `folder_list_top` 查询模式
- 现有 `attendance/src/lib.rs` 的 `list_admins` 查询模式
- 现有 `general_assemble_control/src/lib.rs` 的 `attendscope_*` CRUD 模式

**Test scenarios:**
- Happy: 创建文件夹 → 上传文件 → 查询文件列表 → 下载文件
- Happy: 创建日历事件 → 查询我的日历 → 更新事件 → 删除事件
- Happy: 提交打卡 → 查询打卡记录 → 申诉 → 审批通过
- Edge: 上传超大文件返回 400
- Edge: 查询不存在的日历返回空
- Error: 无权限访问他人私有日历返回 403

**Verification:**
- `cargo test` 本波次 crate 全部通过
- 文件上传/下载在测试环境手动验证
- 端点清单文档中 wave 2 的 11 个 crate 标记状态

---

### U6. Priority wave 3: Process, message, meeting, portal, query, cms

**Goal:** 完成流程引擎、消息、会议、门户、查询报表、CMS 模块的真实化，覆盖核心用户工作流

**Requirements:** R4, R5, R9, R10, R12, R13, R14, R15, R31, R32, R53

**Dependencies:** U1, U2, U3, U4, U5

**Files:**
- Modify: `oa4rust/crates/process_surface/src/`
- Modify: `oa4rust/crates/process_designer/src/`
- Modify: `oa4rust/crates/process_express/src/`
- Modify: `oa4rust/crates/processplatform_service_processing/src/`
- Modify: `oa4rust/crates/processplatform_core_entity/src/`
- Modify: `oa4rust/crates/message/src/`
- Modify: `oa4rust/crates/message_assemble_communicate/src/`
- Modify: `oa4rust/crates/message_core_entity/src/`
- Modify: `oa4rust/crates/meeting/src/`
- Modify: `oa4rust/crates/meeting_assemble_control/src/`
- Modify: `oa4rust/crates/meeting_core_entity/src/`
- Modify: `oa4rust/crates/portal/src/`
- Modify: `oa4rust/crates/portal_assemble_designer/src/`
- Modify: `oa4rust/crates/portal_assemble_surface/src/`
- Modify: `oa4rust/crates/portal_core_entity/src/`
- Modify: `oa4rust/crates/query_service/src/`
- Modify: `oa4rust/crates/query_express/src/`
- Modify: `oa4rust/crates/query_core_entity/src/`
- Modify: `oa4rust/crates/query_core_express/src/`
- Modify: `oa4rust/crates/query_assemble_designer/src/`
- Modify: `oa4rust/crates/query_assemble_surface/src/`
- Modify: `oa4rust/crates/query_service_processing/src/`
- Modify: `oa4rust/crates/cms_control/src/`
- Modify: `oa4rust/crates/cms_express/src/`
- Modify: `oa4rust/crates/cms_assemble_control/src/`
- Modify: `oa4rust/crates/cms_core_entity/src/`
- Modify: `oa4rust/crates/cms_core_express/src/`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- 按 Java 模块的 JAX-RS 控制器逐一映射端点，优先实现高频工作流（流程发起/审批、消息收发、会议预约、门户页面渲染、查询视图、CMS 发布）
- `process_*` 系列：表单定义、流程实例、任务列表、工作项操作、快照、签名；流程引擎为最复杂模块，允许分阶段（先工作流 CRUD，再引擎逻辑）
- `message_*` 系列：消息消费/创建/已读/未读/删除
- `meeting_*` 系列：会议室管理、会议 CRUD、参与人管理、日程关联
- `portal_*` 系列：页面 CRUD、部件管理、脚本管理、字典管理
- `query_*` 系列：视图定义、查询执行、导入/导出
- `cms_*` 系列：栏目管理、文章 CRUD、发布/撤销、索引管理
- 每个 crate 至少补充 2 个集成测试
- 复杂度高的模块（process）允许先实现只读查询，再实现写操作

**Patterns to follow:**
- 现有 `control` crate 的 CRUD + 分页模式
- 现有 `general_assemble_control` 的复杂查询模式（多表 JOIN）
- 现有 `attendance_core_entity` 的统计查询模式

**Test scenarios:**
- Happy: 创建流程实例 → 查询任务列表 → 审批通过 → 查看工作流状态
- Happy: 发送消息 → 查询未读列表 → 标记已读
- Happy: 预约会议室 → 查询会议列表 → 取消会议
- Happy: 发布 CMS 文章 → 查询文章列表 → 撤回发布
- Edge: 查询无权限的流程实例返回 403
- Error: 审批不存在的任务返回 404

**Verification:**
- `cargo test` 本波次 crate 全部通过
- 核心业务流程（发起流程 → 审批 → 通知）端到端可走通
- 端点清单文档中 wave 3 的 31 个 crate 标记状态

---

### U7. Priority wave 4: Infrastructure and edge modules

**Goal:** 完成剩余基础设施模块（AI、组件、热点、推送、思维导图、BBS、快递、控制台、表达式、关联关系、序列号等）的真实化

**Requirements:** R4, R5, R16, R18, R19, R20, R21, R22, R23, R24, R25, R26, R27, R28, R30, R31, R32, R53

**Dependencies:** U1, U2, U3, U4, U5, U6

**Files:**
- Modify: `oa4rust/crates/ai/src/`
- Modify: `oa4rust/crates/ai_assemble_control/src/`
- Modify: `oa4rust/crates/ai_core_entity/src/`
- Modify: `oa4rust/crates/component/src/`
- Modify: `oa4rust/crates/component_assemble_control/src/`
- Modify: `oa4rust/crates/component_core_entity/src/`
- Modify: `oa4rust/crates/hotpic/src/`
- Modify: `oa4rust/crates/hotpic_assemble_control/src/`
- Modify: `oa4rust/crates/hotpic_core_entity/src/`
- Modify: `oa4rust/crates/jpush/src/`
- Modify: `oa4rust/crates/jpush_assemble_control/src/`
- Modify: `oa4rust/crates/jpush_core_entity/src/`
- Modify: `oa4rust/crates/mind/src/`
- Modify: `oa4rust/crates/mind_assemble_control/src/`
- Modify: `oa4rust/crates/mind_core_entity/src/`
- Modify: `oa4rust/crates/bbs/src/`
- Modify: `oa4rust/crates/bbs_assemble_control/src/`
- Modify: `oa4rust/crates/bbs_core_entity/src/`
- Modify: `oa4rust/crates/console/src/`
- Modify: `oa4rust/crates/express/src/`
- Modify: `oa4rust/crates/correlation/src/`
- Modify: `oa4rust/crates/correlation_service_processing/src/`
- Modify: `oa4rust/crates/correlation_core_entity/src/`
- Modify: `oa4rust/crates/correlation_core_express/src/`
- Modify: `oa4rust/crates/organization_assemble_express/src/`
- Modify: `oa4rust/crates/organization_assemble_control/src/`
- Modify: `oa4rust/crates/organization_core_entity/src/`
- Modify: `oa4rust/crates/organization_core_express/src/`
- Modify: `oa4rust/crates/program_center/src/`
- Modify: `oa4rust/crates/program_center_core_entity/src/`
- Modify: `oa4rust/crates/base/src/`
- Modify: `oa4rust/crates/general/src/`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- 按 Java 模块逐一映射，优先实现 CRUD 和查询接口，复杂业务逻辑（如 AI 推理、推送通知）允许简化或保留接口 stub 但返回真实数据
- AI 模块：模型管理、推理调用（允许对接外部服务或返回模拟结果，但接口契约对齐）
- 组件管理：应用中心 CRUD、部署状态
- 热点图片：轮播图 CRUD、推荐列表
- 推送服务：设备管理、消息推送（允许对接第三方推送服务或记录推送日志）
- 思维导图：创建/编辑/共享（允许使用 JSON 存储简化版）
- BBS：分类/文章/版主/搜索
- 快递查询：物流追踪（允许对接第三方 API 或返回模拟结果）
- 控制台：命令行/日志/监控（允许只读或简化）
- 表达式引擎：组织/流程/CMS/查询脚本的解析和执行
- 关联关系：数据关联/引用 CRUD
- 序列号/权限/配置：通用服务
- 每个 crate 至少 1 个集成测试

**Patterns to follow:**
- 现有 `control` 的 CRUD 模式
- 现有 `general_assemble_control` 的复杂查询模式

**Test scenarios:**
- Happy: CRUD 接口正常返回真实数据
- Edge: 空列表返回正确分页结构
- Error: 无权限访问返回 403

**Verification:**
- `cargo test` 本波次 crate 全部通过
- 端点清单文档中 wave 4 的 31 个 crate 标记为"已完成"或"部分完成"
- 无 `ActionResult::success(Value::Null)` 残留

---

### U8. Testing infrastructure and behavior validation

**Goal:** 建立完整的测试基础设施：单元测试、集成测试、行为对比测试套件，确保 Rust 端点与 Java 端点功能等效

**Requirements:** R31, R32, R33, R34, R35, R53

**Dependencies:** U4, U5, U6, U7

**Files:**
- Create: `oa4rust/tests/integration_runner.rs`
- Modify: `oa4rust/crates/*/src/tests.rs` (全仓)
- Create: `oa4rust/tests/behavior_compare.rs`
- Create: `docs/brainstorms/oa4rust-endpoint-inventory.md`
- Modify: `docs/brainstorms/oa4rust-migration-status.md`

**Approach:**
- 为所有 80 个 crate 补充单元测试（核心业务逻辑覆盖）和集成测试（带真实 DB）
- 建立行为对比测试套件：启动 Rust 和 Java 服务，对相同输入对比响应结构（字段名、类型、非空约束），不要求逐字节一致但要求前端可解析
- 创建端点清单文档 `oa4rust-endpoint-inventory.md`：列出每个 crate 对应的 Java JAX-RS 端点、Rust 实现状态、业务优先级、实现顺序
- 在 CI 中集成：`cargo test` + `detect_route_conflicts` + 集成测试覆盖率报告
- 更新迁移状态跟踪文档

**Patterns to follow:**
- 现有 `shared/src/tests.rs` 的中间件集成测试模式
- 现有 `auth/src/tests.rs` 的密码验证测试模式
- 现有 `control/src/tests.rs` 的 fail-closed 测试模式

**Test scenarios:**
- Happy: `cargo test` 全仓通过
- Happy: 集成测试覆盖率报告显示 core business flows ≥ 80%
- Happy: 行为对比测试发现 Rust 与 Java 响应结构不一致时自动报告
- Edge: 测试数据库不可用时集成测试跳过而非 panic

**Verification:**
- `cargo test --all` 通过
- 覆盖率报告生成并存档
- 端点清单文档完整覆盖 80 个 crate

---

### U9. Migration readiness: rollback, feature flags, nginx config

**Goal:** 完成双轨运行所需的回滚程序、特性开关、nginx 灰度路由配置，确保可按模块逐步切换流量

**Requirements:** R49, R50, R51, R52

**Dependencies:** U1, U2, U3, U4, U5, U6, U7, U8

**Files:**
- Create: `oa4rust/deploy/nginx.conf`
- Modify: `oa4rust/deploy/nginx-auth-routes.conf`
- Create: `oa4rust/deploy/rollback-playbook.md`
- Modify: `oa4rust/crates/shared/src/middleware.rs` (feature flag)
- Create: `oa4rust/scripts/toggle_module.sh`

**Approach:**
- nginx 配置：按 URL 前缀路由，`/jaxrs/attendance/*` → Rust，`/jaxrs/message/*` → Java（通过 `map` 或 `upstream` 切换）
- 特性开关：环境变量 `MODULE_ROUTING=attendance:rust,calendar:java,...` 控制每个模块路由到 Rust 还是 Java
- 回滚程序：定义触发条件（数据损坏、性能下降、错误率飙升）、回滚流程（nginx 切回 Java）、RTO 目标（5 分钟）
- 双轨运行数据校验：对正在迁移的表定期对比 Rust 与 Java 的查询结果一致性
- 记录数据库访问模式：事务隔离级别（已为 READ COMMITTED）、并发写入风险、Java 写入禁用机制（特性开关或 DB 触发器）

**Patterns to follow:**
- 现有 `deploy/nginx-auth-routes.conf` 的 nginx 配置模式
- 现有 `shared/middleware.rs` 的环境变量读取模式

**Test scenarios:**
- Happy: nginx 将 `/jaxrs/control/*` 路由到 Rust，`/jaxrs/message/*` 路由到 Java
- Happy: 切换 `MODULE_ROUTING` 后流量立即切到 Java
- Integration: Rust 和 Java 同时运行，数据库事务隔离级别防止数据竞争
- Drill: 模拟数据损坏，回滚程序在 5 分钟内完成切流

**Verification:**
- nginx 配置 `nginx -t` 通过
- 特性开关切换后 Router 正确响应
- 回滚演练记录存档

---

## System-Wide Impact

- **Interaction graph:** 全部 80 个 crate 共享 `shared::SecurityState`（SessionManager + RateLimiter + Pool），U1-U3 修改中间件栈影响所有 crate 的请求处理顺序；U4-U7 修改各 crate handler 影响前端 `action.js` 的 `ActionResult<T>` 解析
- **Error propagation:** 统一 `AppError` → HTTP 状态码映射（400/401/403/404/429/500），业务错误保持 HTTP 200 + `type=error`；U1 加固中间件后错误路径更严格
- **State lifecycle risks:** SessionManager 从内存迁移到 DB（U3）期间需保证平滑过渡（双写或灰度）；RateLimiter 内存泄漏风险（滑动窗口无定期清理）需在 U1 修复
- **API surface parity:** 前端 `o2web` 的 `action.js` 依赖 9 字段 `ActionResult<T>` 结构，任何 Rust 端点修改响应格式必须保持字段兼容；U4-U7 实施时需对照 Java 响应逐字段验证
- **Integration coverage:** 中间件栈集成（U1）、路由冲突冒烟（U2）、auth 安全加固（U3）为全仓影响；各波次 crate 的真实化需通过集成测试验证与 Java 的行为等效性
- **Unchanged invariants:** `ActionResult<T>` 的 9 字段 JSON 结构不变；`/health` 端点保持公开；`DATABASE_URL` 环境变量配置方式不变；单进程单体部署模型不变

---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Axum 0.8 升级引入 breaking change | Medium | High | U1 前完成升级，CI 集成 `detect_route_conflicts` 脚本，全仓搜索替换 `:param` |
| 路由冲突导致服务启动 panic | Medium | High | U2 修复已知冲突，U1 添加自动化检测脚本 |
| 会话持久化迁移期间用户被强制登出 | Medium | Medium | U3 采用双写过渡（内存 + DB），定义降级策略（DB 不可用时回退到纯内存并监控一致性） |
| 复杂模块（process engine）实现周期过长 | High | Medium | U6 分阶段（先 CRUD 后引擎逻辑），允许简化非核心路径 |
| 前端 `action.js` 对响应格式有隐式假设 | Medium | High | U1 建立 ActionResult 契约测试框架，U4-U7 每个 crate 实现时自动验证响应格式 |
| Java 与 Rust 并发写入导致数据不一致 | Low | High | U4 开始前禁用 Java 写入，U9 完善监控和回滚 |
| 行为对比测试无法覆盖所有边缘情况 | Medium | Medium | U8 优先覆盖核心业务流程，边界情况通过集成测试补充 |
| 部分 crate 的 Java 源码不可获取或逻辑复杂 | Medium | Low | 允许功能等效而非逐字节一致，优先保证前端契约 |
| TODO 标记意外进入生产 | Low | High | CI 检查生产分支无 TODO；feature flag 禁用未完成 stub |
| OAuth 密钥泄露 | Low | High | U3 前明确密钥管理器选型（Vault/云 KMS），禁止硬编码 |

---

## Documentation / Operational Notes

- 每完成一个 crate 立即更新 `docs/brainstorms/oa4rust-migration-status.md`（单一信息源原则）
- 端点清单文档 `docs/brainstorms/oa4rust-endpoint-inventory.md` 在 **U2 完成后、U4 开始前**创建，作为实施依据
- 回滚演练在每波次完成后执行一次，记录存档
- CI 配置增加：`cargo test`、`detect_route_conflicts`、集成测试覆盖率检查、TODO 标记检查（生产分支禁止 TODO）
- U3 开始前完成 OAuth 密钥管理器选型（Vault/云 KMS/环境变量），制定密钥轮换策略

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md](../brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md)
- Migration status: [docs/brainstorms/oa4rust-migration-status.md](../brainstorms/oa4rust-migration-status.md)
- Related code: `oa4rust/src/main.rs`, `oa4rust/crates/shared/src/`
- Related plans: [docs/plans/2026-08-05-001-feat-oa4rust-comprehensive-advancement-plan.md](../plans/2026-08-05-001-feat-oa4rust-comprehensive-advancement-plan.md)
- External docs: Axum 0.8, SQLx 0.7, Deadpool-postgres 0.12
