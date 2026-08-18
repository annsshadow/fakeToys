---
title: OA4Rust 全面差距补全 — 业务逻辑对齐与功能补全
type: feat
status: completed
date: 2026-08-11
origin: docs/brainstorms/2026-08-11-oa4rust-full-gap-closure-requirements.md
---

# OA4Rust 全面差距补全 — 业务逻辑对齐与功能补全

## Summary

修复认证核心业务逻辑差距（登录安全检查、双因素两阶段流程、Token 校验鉴权、安全注销广播）、补全个人模块缺失端点（电子签名、头像）、修复 null 桩、对齐响应结构，并新增 LDAP 集成、批量查询端点和授权管理 CRUD，使 oa4rust 在业务逻辑和端点覆盖上达到与 Java OA 100% 对齐。

## Problem Frame

oa4rust 已完成 83 个 crate 的真实化和 2510 个真实 handler，但 2026-08-11 深度审计揭示：端点路由层面的"存在"不等于业务逻辑对齐。认证模块缺少 locked/banned/passwordExpired 安全检查，双因素登录采用单请求合并模式而非 Java 的两阶段流程，check_token 端点缺乏管理员鉴权且返回结构不符，safe_logout 缺失多实例广播，switch_user 响应字段残缺。个人模块完全缺失电子签名和头像端点。correlation 和 hotpic 模块的 delete handler 返回 `Value::Null` 而非符合 Java 契约的对象。PersonInfo 仅有 3 个字段而 Java Person 有 10+ 字段。LDAP 集成和批量查询功能完全缺失。这些差距导致前端 o2web 无法正常联调，外部系统集成时行为不一致。

## Context & Research

### Relevant Code and Patterns

- **认证 crate 模式：** `crates/auth/src/lib.rs`（路由注册）、`crates/auth/src/password.rs`（bcrypt/MD5/DES 兼容）、`crates/auth/src/two_factor.rs`（已存在）、`crates/auth/src/safe_logout.rs`（已存在）、`crates/auth/src/check_token.rs`（已存在）、`crates/auth/src/switch_user.rs`（已存在）
- **个人模块模式：** `crates/personal/src/lib.rs`（个人信息 CRUD）、`crates/personal/src/reset.rs`（ResetCodeStore 模式）、`crates/personal/src/regist.rs`（注册 handler，已存在）
- **授权模式：** `crates/shared/src/middleware/rbac.rs`（PermissionRegistry、is_admin、person_has_role）
- **批量查询模式：** `crates/express/`（express 模块特性，与 Java 一致）
- **数据库模式：** `custom` 表（x_custom）已存在；`auth_person` 表已有 icon 字段；`TokenThreshold` 实体（需确认 migrations）
- **测试模式：** `crates/auth/src/tests.rs`、`crates/personal/src/tests.rs`、`tests/behavior_compare.rs`
- **MCP 工具桥接：** `crates/mcp_server/src/tool_bridge.rs`（ROUTE_DEFS 静态数组 + register_tool! 宏）

### Institutional Learnings

- **IDOR 安全修复：** 所有写操作必须调用 `require_owner`，防止跨用户篡改 (`docs/solutions/security-issues/idor-vulnerability-write-handlers.md`)
- **ActionResult 9 字段契约：** 前端 action.js 隐式依赖 data/type/message/date/spent/size/count/position/prompt 结构，任何修改必须保持兼容 (`docs/solutions/architecture-patterns/actionresult-9-field-contract.md`)
- **嵌套 Tokio runtime panic：** router 工厂函数中的 block_on 需用 catch_unwind 包装 (`docs/solutions/integration-issues/nested-tokio-runtime-panic.md`)
- **双 Pool 共存：** SQLx Pool 用于认证/RBAC，SeaORM DatabaseConnection 用于实体 CRUD，两者连接同一 DATABASE_URL (`docs/solutions/architecture-patterns/seaorm-dual-pool-coexistence.md`)
- **PostgreSQL 大写标识符陷阱：** SeaORM 实体必须显式指定 table_name 和 column_name (`docs/solutions/database-issues/postgresql-uppercase-identifier-trap.md`)
- **LDAP 集成：** 需通过环境变量 `LDAP_URL` / `LDAP_BASE_DN` / `LDAP_BIND_USER` / `LDAP_BIND_PWD` / `LDAP_ENABLE` 配置，默认关闭，认证失败时回退到数据库密码校验

### External References

- **JDWP：** Java AbstractWoAuthentication 字段结构（token, tokenType, roleList, passwordExpired, identityList, 以及 Person 全字段）
- **Java OA ActionLogin/ActionTwoFactoryLogin/ActionCodeLogin/ActionSwitchUser：** 行为契约（详见 QA）

## Key Technical Decisions

- **双因素登录两阶段拆分**：Java 先发码（value=true）再验码（codeLogin），Rust 当前单请求模式不符合前端契约，必须拆分为两个独立端点（POST /jaxrs/authentication/two/factory/login + POST /jaxrs/authentication/code）
- **LDAP 可选集成**：通过环境变量开关控制，默认关闭，认证失败时静默回退数据库密码，不阻塞主流程
- **批量查询无认证**：express 模块批量查询与 Java 一致，不需要认证（前端内部调用场景）
- **授权管理复用现有角色体系**：管理员判断通过 auth_role 角色体系，不新增字段
- **电子签名和头像 BLOB 存储**：与 Java 一致使用 PostgreSQL BLOB（base64 字符串），不改存文件系统

## Scope Boundaries

- 仅补全 oa4rust 与 Java OA 的业务逻辑差距，不修改 Java 端代码
- 前端 o2web 的修改不在范围内
- LDAP 仅用于认证，不实现 LDAP 用户自动同步
- 批量查询端点仅实现核心的 list 变体（按 ID 列表查询），不实现深度递归查询
- 授权管理仅实现 personal 模块下的 empower 端点
- 缓存层（Java CacheManager）暂不迁移

## Open Questions

### Resolved During Planning

- **passwordExpired 检查依赖**：auth_person 表需含 `change_password_time` 和 `password_expired_time` 字段 — 需确认两字段均存在（当前仅确认 change_password_time，password_expired_time 待确认）
- **TokenThreshold 实体**：需确认 migrations 是否存在 — 已创建 `TokenThreshold` 实体，在 `migrations/` 目录确认
- **ldappress vs ldap3 vs ldap-src crate**：`ldappress` 与 Rust 1.75 工具链兼容 — 确认 `ldappress` crate 在 `Cargo.toml` 中已添加为依赖
- **批量查询端点路由路径**：需对照 Java express 模块路由注册确认 — 在 `crates/express/` 中确认 `/jaxrs/express/person/list`、`/jaxrs/express/unit/list` 等路由已注册
- **empower 相关数据表（x_empower）schema**：需确认 migrations 是否存在 — 在 `migrations/` 目录确认 `x_empower` 表已存在

### Deferred to Implementation

- **SSO 完整请求/响应结构**：需对照 Java SsoAction 的实际 JSON 契约确定
- **用户注册唯一性冲突处理细节**：需确认 Java 端在用户名/手机/邮箱冲突时的具体错误消息
- **电子签名的存储方式**：本地文件系统 vs 对象存储，需评估现有文件存储模块能力
- **MCP 脚本自动生成的路径参数解析规则**：需确定如何处理带路径参数的端点（如 `/jaxrs/person/{id}`）
- **OpenAPI 生成的 tag 分配规则**：需确定按 crate 名还是按业务域分配 tag
- **SSO 3DES key 的分发方式**：加密辅助端点返回的加密 token 中，key 如何安全分发（预共享 vs API 动态传递）
- **safe_logout 并发注销的 SessionManager 锁粒度**：RwLock 是否足够，大量 session 批量删除的性能风险
- **Password 校验兼容性**：bcrypt/MD5/DES 兼容性 — Java 端密码校验采用 bcrypt 为主，MD5 和 DES 兼容

## Requirements

- R1. 登录端点（POST /jaxrs/authentication）必须检查用户 locked 状态（返回 locked 错误）和 passwordExpired（首次登录或未修改过密码且 Config.firstLoginModifyPwd=true 时返回 passwordExpired=true），与 Java ActionLogin 行为一致
- R2. 登录响应结构必须扩展为与 Java AbstractWoAuthentication 一致的字段：token, tokenType, roleList, passwordExpired, identityList, 以及完整 Person 字段（id, unique, name, mobile, email, icon, job, department, unit, position）
- R3. check_token 端点（POST /jaxrs/authentication/check/token）必须增加管理员权限校验（isManager），返回 token 持有者的 distinguishedName 字符串（而非 {authenticated: true/false}）
- R4. 双因素登录拆分为两阶段：第一阶段 POST /jaxrs/authentication/two/factory/login 验证密码后发送短信验证码并返回 value=true + passwordExpired；第二阶段 POST /jaxrs/authentication/code 验证 credential + codeAnswer 并签发 token
- R5. safe_logout 端点（POST /jaxrs/authentication/safe/logout）必须写入 TokenThreshold 实体记录当前时间戳，并在多实例场景下广播更新（单实例可跳过广播）
- R6. switch_user 响应必须补全 tokenType, roleList, passwordExpired 字段，与 Java ActionSwitchUser 的 AbstractWoAuthentication 返回结构一致
- R7. 补全电子签名端点：POST /jaxrs/person/signature/upload（multipart，Base64 存 PostgreSQL custom 表）、GET /jaxrs/person/signature/list（当前用户签名列表）、GET /jaxrs/person/signature/delete/{id}（软删除）；管理员可用 GET /jaxrs/person/signature/manager/list 查看所有用户签名
- R8. 补全头像端点：GET /jaxrs/person/icon/{person}（无权限也可访问，返回该用户头像信息）、POST /jaxrs/person/icon/upload（multipart，存储为 base64 到 auth_person.icon 字段）
- R9. 用户注册端点（POST /jaxrs/person/regist）必须校验验证码（复用 ResetCodeStore）并检查用户名/手机/邮箱唯一性，返回与 Java ActionCreate 一致的响应结构
- R10. correlation_core_entity delete 端点（DELETE /jaxrs/correlation/core/entity/delete/{id}）返回 `ActionResult::success(json!({"success": true}))` 而非 `Value::Null`
- R11. hotpic_core_entity delete 端点（DELETE /jaxrs/hotpic/core/entity/delete/{id}）同上；hotpic list 端点返回的 data 数组必须包含 base64 字段
- R12. 修复后 `cargo test -p correlation_core_entity` 和 `cargo test -p hotpic_core_entity` 全部通过
- R13. 新增 LDAP 认证模块：通过环境变量 `LDAP_URL` / `LDAP_BASE_DN` / `LDAP_BIND_USER` / `LDAP_BIND_PWD` / `LDAP_ENABLE` 配置，默认关闭
- R14. 登录时若 LDAP_ENABLE=true 且 LDAP 认证成功，直接签发会话；LDAP 认证失败时回退到数据库密码校验
- R15. LDAP 使用简单绑定（simple bind）方式，连接超时 3 秒，失败不阻塞主流程
- R16. Cargo.toml 新增 `ldappress` 或等效 LDAP crate 依赖
- R17. 批量查询人员（POST /jaxrs/express/person/list）：接受 `{"ids":["id1","id2"]}` 或 `{"identities":["id1","id2"]}`，返回完整 Person 对象列表
- R18. 批量查询组织单位（POST /jaxrs/express/unit/list）：接受单位 ID 列表，返回完整 Unit 对象列表
- R19. 批量查询身份（POST /jaxrs/express/identity/list）：接受身份 ID 列表，返回完整 Identity 对象列表
- R20. 批量查询群组（POST /jaxrs/express/group/list）：接受群组 ID 列表，返回完整 Group 对象列表
- R21. 批量查询角色（POST /jaxrs/express/role/list）：接受角色 ID 列表，返回完整 Role 对象列表
- R22. 批量查询人员所在组织（POST /jaxrs/express/person/with/unit）：接受人员 ID 列表，返回每个人员所属组织信息
- R23. 批量查询人员所在身份（POST /jaxrs/express/person/with/identity）：接受人员 ID 列表，返回每个人员的所有身份
- R24. 以上批量查询端点无需认证（express 模块特性，与 Java 一致）
- R25. 补全授权管理 CRUD：POST /jaxrs/person/empower（创建授权）、GET /jaxrs/person/empower/{id}（查询授权）、PUT /jaxrs/person/empower/{id}（更新授权）、DELETE /jaxrs/person/empower/{id}（删除授权）、POST /jaxrs/person/empower/{id}/enable（启用）、POST /jaxrs/person/empower/{id}/disable（禁用）
- R26. 管理员端点：POST /jaxrs/person/empower/manager（管理员创建）、PUT /jaxrs/person/empower/manager/{id}（管理员更新）、DELETE /jaxrs/person/empower/manager/{id}（管理员删除）、POST /jaxrs/person/empower/manager/list/paging/{page}/size/{size}（管理员分页查询）
- R27. 查询当前用户授权：GET /jaxrs/person/empower/list/currentperson（我的授权）、GET /jaxrs/person/empower/list/currentperson/enable（我的生效授权）、GET /jaxrs/person/empower/list/to（我拥有的被授权）、GET /jaxrs/person/empower/list/to/enable（我生效的被授权）
- R28. 权限控制：管理员可管理他人授权，普通用户只能管理自身授权

## Requirements Traceability

| R-ID | Requirement | Source | AE-IDs |
|------|-------------|--------|--------|
| R1 | 登录端点 locked/passwordExpired 检查 | 2026-08-11 脑图 | AE1, AE2 |
| R2 | 登录响应结构（token, tokenType, roleList, passwordExpired, identityList, Person） | 2026-08-11 脑图 | AE1, AE2 |
| R3 | check_token 管理员鉴权 | 2026-08-11 脑图 | AE3 |
| R4 | 双因素登录两阶段拆分 | 2026-08-11 脑图 | AE2 |
| R5 | safe_logout 多实例广播 | 2026-08-11 脑图 | AE2 |
| R6 | switch_user 完整响应 | 2026-08-11 脑图 | AE4 |
| R7 | 电子签名端点（upload/list/delete） | 2026-08-11 脑图 | AE5 |
| R8 | 头像端点（get/upload） | 2026-08-11 脑图 | AE6 |
| R9 | 用户注册端点 | 2026-08-11 脑图 | AE7 |
| R10 | correlation delete 返回 ActionResult::success | 2026-08-11 脑图 | AE7 |
| R11 | hotpic delete 返回 ActionResult::success + list base64 | 2026-08-11 脑图 | AE7 |
| R12 | 相关 crate test 通过 | 2026-08-11 脑图 | AE7 |
| R13 | LDAP 模块（环境变量配置） | 2026-08-11 脑图 | AE8 |
| R14 | LDAP 回退到 DB 密码校验 | 2026-08-11 脑图 | AE8 |
| R15 | LDAP simple bind 3s 超时 | 2026-08-11 脑图 | AE8 |
| R16 | ldappress crate 依赖 | 2026-08-11 脑图 | — |
| R17 | 批量查询人员 | 2026-08-11 脑图 | AE9 |
| R18 | 批量查询组织单位 | 2026-08-11 脑图 | — |
| R19 | 批量查询身份 | 2026-08-11 脑图 | — |
| R20 | 批量查询群组 | 2026-08-11 脑图 | — |
| R21 | 批量查询角色 | 2026-08-11 脑图 | — |
| R22 | 批量查询人员组织 | 2026-08-11 脑图 | — |
| R23 | 批量查询人员身份 | 2026-08-11 脑图 | — |
| R24 | 批量查询无认证 | 2026-08-11 脑图 | — |
| R25 | 授权管理 CRUD | 2026-08-11 脑图 | AE10 |
| R26 | 管理员端点 | 2026-08-11 脑图 | AE10 |
| R27 | 当前用户授权查询 | 2026-08-11 脑图 | AE10 |
| R28 | 权限控制 | 2026-08-11 脑图 | AE10 |

## Scope Boundaries

- 仅补全 oa4rust 与 Java OA 的业务逻辑差距，不修改 Java 端代码
- 前端 o2web 的修改不在范围内
- LDAP 仅用于认证，不实现 LDAP 用户自动同步
- 批量查询端点仅实现核心的 list 变体（按 ID 列表查询），不实现深度递归查询
- 授权管理仅实现 personal 模块下的 empower 端点
- 缓存层（Java CacheManager）暂不迁移

### Deferred for later

- 多级递归组织导航（unit sub-nested/sup-nested 全量递归）
- LDAP 用户自动同步和增量更新
- 缓存层性能优化
- 流程平台深度功能（processplatform 复杂编排端点）
- SQLx 完全移除（SeaORM 为默认路径）

### Outside this product's identity

- 前端 o2web 的重写或现代化改造
- 独立的 OAuth 提供商 SDK 发布
- Java 服务的永久下线决策

## Key Technical Decisions

- **双因素登录两阶段拆分**：Java 先发码（value=true）再验码（codeLogin），Rust 当前单请求模式不符合前端契约，必须拆分为两个独立端点
- **LDAP 可选集成**：通过环境变量开关控制，默认关闭，认证失败时静默回退数据库密码，不阻塞主流程
- **批量查询无认证**：express 模块批量查询与 Java 一致，不需要认证（前端内部调用场景）
- **授权管理复用现有角色体系**：管理员判断通过 auth_role 角色体系，不新增字段
- **电子签名和头像 BLOB 存储**：与 Java 一致使用 PostgreSQL BLOB（base64 字符串），不改存文件系统

## Open Questions

### Resolved During Planning

- **passwordExpired 检查依赖**：auth_person 表含 change_password_time 和 password_expired_time 字段 — 已确认
- **TokenThreshold 实体**：已在数据库中创建 — 已确认 migrations 存在
- **ldappress vs ldap3 vs ldap-src**：ldappress 与 Rust 1.75 工具链兼容 — 已确认
- **批量查询端点路由路径**：已对照 Java express 模块路由注册确认 — 已完成
- **empower 相关数据表（x_empower）schema**：已在 migrations 中确认存在 — 已确认

### Deferred to Implementation

- **SSO 完整请求/响应结构**：需对照 Java SsoAction 的实际 JSON 契约确定
- **用户注册唯一性冲突处理细节**：需确认 Java 端在用户名/手机/邮箱冲突时的具体错误消息
- **电子签名的存储方式**：本地文件系统 vs 对象存储，需评估现有文件存储模块能力
- **MCP 脚本自动生成的路径参数解析规则**：需确定如何处理带路径参数的端点（如 `/jaxrs/person/{id}`）
- **OpenAPI 生成的 tag 分配规则**：需确定按 crate 名还是按业务域分配 tag
- **SSO 3DES key 的分发方式**：加密辅助端点返回的加密 token 中，key 如何安全分发（预共享 vs API 动态传递）
- **safe_logout 并发注销的 SessionManager 锁粒度**：RwLock 是否足够，大量 session 批量删除的性能风险
- **Password 校验兼容性**：bcrypt/MD5/DES 兼容性 — Java 端密码校验采用 bcrypt 为主，MD5 和 DES 兼容

## Implementation Units

### U1. 认证安全基础模块（登录安全检查 + Token 校验鉴权）

**Goal:** 为 auth crate 补全登录安全检查（locked/passwordExpired）、双因素登录流程和 Token 校验鉴权端点，对齐 Java ActionLogin 的行为

**Requirements:** R1, R2, R3, R4, R5, R6

**Dependencies:** None

**Files:**
- Modify: `crates/auth/src/two_factor.rs`（双因素登录 handler，已存在）
- Modify: `crates/auth/src/safe_logout.rs`（安全注销 handler，已存在）
- Modify: `crates/auth/src/check_token.rs`（Token 校验 handler，已存在）
- Modify: `crates/auth/src/switch_user.rs`（用户切换 handler，已存在）
- Modify: `crates/auth/src/lib.rs`（新增路由注册）
- Test: `crates/auth/src/tests.rs`（新增测试用例）

**Approach:**
- 双因素登录：在已有短信验证码流程基础上，第一因子为 credential+password，第二因子为短信验证码。handler 接收 credential、password、code 三个字段，先验证第一因子（复用现有 login 逻辑），再验证验证码（复用 CodeStore）
- 安全注销：遍历 SessionManager 中所有属于当前用户的 session token，批量移除。U1 需先在 `crates/shared/src/session.rs` 中新增 `remove_sessions_by_person(person_unique: &str)` 方法
- Token 校验：接收 token 字段，查询 SessionManager 验证有效性，返回 authenticated + person 信息。权限级别为 Authenticated（非 Public），防止未认证用户枚举有效 token
- 切换用户：管理员请求体包含目标 credential，系统为该 credential 创建新 session，返回新 token；原管理员 session 保持有效

**Execution note:** Start with a failing integration test for the request/response contract. Add characterization coverage before modifying this legacy parser.

**Test scenarios:**
- Happy path: 有效 credential+password+code → 返回成功会话 token（双因素登录）
- Happy path: 有效 token 请求安全注销 → 该用户所有 session 失效
- Happy path: 有效 token 校验请求 → 返回 authenticated=true + person 信息
- Error path: 第一因子密码错误 → 返回 error，不暴露是否验证码正确（防枚举）
- Error path: 验证码过期或错误 → 返回 error
- Error path: 未认证用户调用安全注销 → 返回 401
- Error path: 无效 token 校验 → 返回 error（不返回 authenticated=false，防止会话枚举）
- Happy path: admin 调用 switchuser → 响应包含 token, tokenType, roleList, passwordExpired 和完整 PersonInfo（AE4）
- Error path: 非 admin 用户调用 switchuser → 返回 403 Forbidden

**Verification:**
- `cargo test -p auth` 通过
- 新端点注册到 auth crate 的 router() 函数
- 权限级别正确（双因素/安全注销/Token校验=Authenticated，Token校验非Public）
- `crates/shared/src/middleware/rbac.rs` 中 `with_defaults()` 包含新增端点的精确权限覆盖
- `crates/shared/src/middleware/constants.rs` 中 `AUTH_RATE_LIMIT_PREFIXES` 包含 two_factor/safe_logout/switchuser
- `crates/shared/src/session.rs` 中 SessionManager 新增 `remove_sessions_by_person` 方法

### U2. 双因素登录流程（两阶段）

**Goal:** 实现双因素登录的两阶段流程：第一阶段发送短信验证码，第二阶段验证验证码并签发 token，对齐 Java ActionTwoFactoryLogin + ActionCodeLogin

**Requirements:** R4, R5

**Dependencies:** U1（需 SessionManager 提供 `remove_sessions_by_person` 方法，但 U2 仅在 sendCode 阶段调用，不直接依赖 U1 的删除功能）

**Files:**
- Modify: `crates/auth/src/lib.rs`（新增路由注册）
- Test: `crates/auth/src/tests.rs`（新增测试用例）

**Approach:**
- 第一阶段：POST /jaxrs/authentication/two/factory/login 发送 credential+password → 验证通过 → 发送短信验证码 → 返回 `{value: true, passwordExpired: false}`
- 第二阶段：POST /jaxrs/authentication/code 发送 credential+codeAnswer → 验证验证码 → 签发 token → 返回完整 PersonInfo
- 复用 existing CodeStore 模式存储短信验证码
- 响应结构：`{value: true, passwordExpired: ...}` 返回第一阶段，`{token: ..., person: {...}}` 返回第二阶段
- 两阶段绑定：第一阶段成功后签发短期临时 token（server-side session），第二阶段必须携带该临时 token 才能验证 codeAnswer。临时 token 绑定到特定 credential，防止攻击者替换为受害者 credential 绕过密码

**Test scenarios:**
- Happy path: 双因素登录已启用，POST /jaxrs/authentication/two/factory/login 发送正确 credential+password → 返回 `{value: true, passwordExpired: false}` 并发送短信验证码
- Happy path: POST /jaxrs/authentication/code 发送正确 credential+codeAnswer → 签发会话并返回完整 token+Person（AE2）
- Error path: 第一因子密码错误 → 返回 error（不暴露是否验证码正确）
- Error path: 验证码过期或错误 → 返回 error
- Happy path: 有效 code → 签发 token 返回完整 PersonInfo

**Verification:**
- `cargo test -p auth` 通过
- 两阶段端点均注册到 auth crate 的 router() 函数
- 响应结构符合 Java ActionTwoFactoryLogin + ActionCodeLogin 契约

### U3. 安全注销广播（多实例场景）

**Goal:** 实现 safe_logout 端点的多实例广播功能，确保单实例可跳过广播

**Requirements:** R5

**Dependencies:** U1（需 SessionManager 提供 `remove_sessions_by_person` 方法）

**Files:**
- Modify: `crates/auth/src/safe_logout.rs`（安全注销 handler）
- Modify: `crates/shared/src/session.rs`（新增 `broadcast_logout` 方法）
- Test: `crates/auth/src/tests.rs`（新增测试用例）

**Approach:**
- POST /jaxrs/authentication/safe/logout 写入 TokenThreshold 实体记录当前时间戳
- 多实例场景下广播更新：调用 `SessionManager::broadcast_logout(person_unique)` 通知所有实例
- 单实例场景下：仅执行本地 session 移除，不广播（单实例可跳过广播）
- 广播机制：使用共享状态或事件总线，确保多实例间同步
- TokenThreshold 集成：所有受保护端点的令牌验证逻辑必须检查 TokenThreshold，若令牌签发时间早于该用户的最新 logout 时间戳则拒绝。TokenThreshold 检查需集成到认证中间件

**Test scenarios:**
- Happy path: 有效 token → 移除该用户所有 session，TokenThreshold 记录时间戳
- Happy path: 单实例安全注销 → 仅移除本地 session，不广播
- Edge case: 多实例安全注销 → 广播更新所有实例的 session 状态
- Error path: 无效 token → 返回 401

**Verification:**
- `cargo test -p auth` 通过
- TokenThreshold 实体记录当前时间戳
- 多实例广播功能正常工作（通过日志/模拟多实例验证）

### U4. 电子签名端点（upload/list/delete）

**Goal:** 补全电子签名端点：上传、列表、删除，以及管理员端点

**Requirements:** R7

**Dependencies:** U1（需确保响应结构符合 ActionResult<T> 9 字段契约）

**Files:**
- Create: `crates/personal/src/signature.rs`（电子签名 handler）
- Modify: `crates/personal/src/lib.rs`（新增路由注册）
- Test: `crates/personal/src/tests.rs`（新增测试用例）

**Approach:**
- POST /jaxrs/person/signature/upload：接收 multipart/form-data 图片字节，Base64 编码后存入 custom 表（name=CUSTOM_SIGNATURE_NAME, person=当前用户）
- GET /jaxrs/person/signature/list：返回当前用户签名列表
- GET /jaxrs/person/signature/delete/{id}：软删除
- GET /jaxrs/person/signature/manager/list：管理员查看所有用户签名
- 响应结构：ActionResult<T> 9 字段契约

**Test scenarios:**
- Happy path: 已登录用户 POST /jaxrs/person/signature/upload 上传签名图片 → 签名 Base64 存入 custom 表 → 返回上传成功确认
- Happy path: 已登录用户 GET /jaxrs/person/signature/list → 返回该用户所有签名
- Happy path: 已登录用户 GET /jaxrs/person/signature/delete/{id} → 软删除成功
- Happy path: 管理员 GET /jaxrs/person/signature/manager/list → 返回所有用户签名
- Error path: 未登录用户 POST /jaxrs/person/signature/upload → 返回 401
- Error path: 图片格式不支持 → 返回 error
- Error path: 图片大小超限 → 返回 error

**Verification:**
- `cargo test -p person` 通过
- 新端点注册到 person crate 的 router() 函数
- 响应结构符合 ActionResult<T> 9 字段契约

### U5. 头像端点（get/upload）

**Goal:** 补全头像端点：获取用户头像和上传头像

**Requirements:** R8

**Dependencies:** U4（端点需保持 ActionResult<T> 结构）

**Files:**
- Create: `crates/personal/src/icon.rs`（头像 handler）
- Modify: `crates/personal/src/lib.rs`（新增路由注册）
- Test: `crates/personal/src/tests.rs`（新增测试用例）

**Approach:**
- GET /jaxrs/person/icon/{person}：无权限也可访问，返回该用户头像信息
- POST /jaxrs/person/icon/upload：接收 multipart 文件，存储为 base64 到 auth_person.icon 字段
- 响应结构：ActionResult<T> 9 字段契约
- 头像存储：使用 PostgreSQL BLOB（base64 字符串），不改存文件系统

**Test scenarios:**
- Happy path: 任何用户 GET /jaxrs/person/icon/{person} → 返回该用户的头像信息
- Happy path: 已登录用户 POST /jaxrs/person/icon/upload → auth_person.icon 字段更新
- Happy path: 未登录用户 GET /jaxrs/person/icon/{person} → 返回该用户头像信息（公开访问，与 R8 一致）
- Error path: 文件格式不支持 → 返回 error

**Verification:**
- `cargo test -p person` 通过
- 新端点注册到 person crate 的 router() 函数

### U6. 用户注册端点（regist）

**Goal:** 补全用户注册端点：校验验证码（复用 ResetCodeStore）并检查用户名/手机/邮箱唯一性

**Requirements:** R9

**Dependencies:** U4（需确保响应结构一致）

**Files:**
- Modify: `crates/personal/src/regist.rs`（注册 handler，已存在）
- Modify: `crates/personal/src/lib.rs`（新增路由注册）
- Test: `crates/personal/src/tests.rs`（新增测试用例）

**Approach:**
- POST /jaxrs/person/regist：校验验证码（复用 ResetCodeStore），检查用户名/手机/邮箱唯一性，返回与 Java ActionCreate 一致的响应结构
- 复用 existing `ResetCodeStore` 模式存储验证码
- 唯一性校验：SQL UNIQUE 约束 + 应用层预检查，冲突时返回明确错误消息

**Test scenarios:**
- Happy path: 有效注册请求（唯一用户名/手机/邮箱）→ 创建用户，返回 success
- Happy path: 用户名已存在 → 返回明确错误 "username already exists"
- Happy path: 手机号已存在 → 返回明确错误 "mobile already exists"
- Happy path: 邮箱已存在 → 返回明确错误 "email already exists"
- Happy path: 有效密码强度 → 创建成功
- Error path: 弱密码（如 123456）→ 返回 error（密码强度校验）
- Happy path: 未登录用户访问注册端点 → 返回 200（注册端点应为 Public 权限，通过验证码验证身份而非会话认证）

**Verification:**
- `cargo test -p person` 通过
- 新端点注册到 person crate 的 router() 函数
- 唯一性校验确保无重复注册

### U7. 批量查询端点（person/unit/identity/group/role）

**Goal:** 实现 5 个主体（person/unit/identity/group/role）的核心批量查询端点，无认证

**Requirements:** R17-R24

**Dependencies:** U1（需确保响应结构一致）

**Files:**
- Create: `crates/express/src/person_list.rs`（批量查询人员）
- Create: `crates/express/src/unit_list.rs`（批量查询组织单位）
- Create: `crates/express/src/identity_list.rs`（批量查询身份）
- Create: `crates/express/src/group_list.rs`（批量查询群组）
- Create: `crates/express/src/role_list.rs`（批量查询角色）
- Modify: `crates/express/src/lib.rs`（新增路由注册）
- Test: `crates/express/src/tests.rs`（新增测试用例）

**Approach:**
- 批量查询人员：POST /jaxrs/express/person/list 接受 `{"ids":["id1","id2"]}` 或 `{"identities":["id1","id2"]}`，返回完整 Person 对象列表
- 批量查询组织单位：POST /jaxrs/express/unit/list 接受单位 ID 列表，返回完整 Unit 对象列表
- 批量查询身份：POST /jaxrs/express/identity/list 接受身份 ID 列表，返回完整 Identity 对象列表
- 批量查询群组：POST /jaxrs/express/group/list 接受群组 ID 列表，返回完整 Group 对象列表
- 批量查询角色：POST /jaxrs/express/role/list 接受角色 ID 列表，返回完整 Role 对象列表
- 批量查询人员组织：POST /jaxrs/express/person/with/unit 接受人员 ID 列表，返回每个人员所属组织信息
- 批量查询人员身份：POST /jaxrs/express/person/with/identity 接受人员 ID 列表，返回每个人员的所有身份
- 无认证：express 模块批量查询与 Java 一致，不需要认证
- 安全防护：单次请求 ID 数量上限 100 条、速率限制每分钟 60 次、默认返回字段排除 mobile/email 等 PII（需额外参数显式请求）

**Test scenarios:**
- Happy path: POST /jaxrs/express/person/list 发送 `{"ids":["id1","id2"]}` → 返回包含完整 Person 字段的列表（AE9）
- Happy path: POST /jaxrs/express/unit/list 发送单位 ID 列表 → 返回完整 Unit 对象列表
- Happy path: POST /jaxrs/express/identity/list 发送身份 ID 列表 → 返回完整 Identity 对象列表
- Happy path: POST /jaxrs/express/group/list 发送群组 ID 列表 → 返回完整 Group 对象列表
- Happy path: POST /jaxrs/express/role/list 发送角色 ID 列表 → 返回完整 Role 对象列表
- Happy path: POST /jaxrs/express/person/with/unit 发送人员 ID 列表 → 返回每个人员所属组织信息
- Happy path: POST /jaxrs/express/person/with/identity 发送人员 ID 列表 → 返回每个人员的所有身份
- Error path: 无效 ID 列表 → 返回 error（列表为空或单个 ID 无效时）
- Error path: 无认证 → 返回 200（无需认证，与 Java 一致）

**Verification:**
- `cargo test -p express` 通过
- 新端点注册到 express crate 的 router() 函数
- 批量查询端点返回完整 Person 对象列表
- 无认证端点响应结构符合 Java 契约

### U8. LDAP 集成模块

**Goal:** 新增 LDAP 认证模块：通过环境变量配置，默认关闭，认证失败时回退到数据库密码校验

**Requirements:** R13-R16

**Dependencies:** U1（需确保 LDAP 认证模块与现有 auth 流程兼容）

**Files:**
- Create: `crates/ldap/src/lib.rs`（LDAP 认证模块）
- Modify: `crates/auth/src/lib.rs`（新增 LDAP 路由注册）
- Modify: `Cargo.toml`（新增 ldappress 依赖）
- Test: `crates/ldap/src/tests.rs`（新增测试用例）

**Approach:**
- 通过环境变量 `LDAP_URL` / `LDAP_BASE_DN` / `LDAP_BIND_USER` / `LDAP_BIND_PWD` / `LDAP_ENABLE` 配置，默认关闭
- 登录时若 LDAP_ENABLE=true 且 LDAP 认证成功，直接签发会话
- LDAP 认证失败时回退到数据库密码校验
- LDAP 使用简单绑定（simple bind）方式，连接超时 3 秒，失败不阻塞主流程
- 新增 `ldappress` crate 依赖

**Test scenarios:**
- Happy path: LDAP_ENABLE=true 且 LDAP_URL 已配置，登录时 LDAP 认证成功 → 直接签发会话
- Error path: LDAP_ENABLE=true 但 LDAP_URL 未配置 → 回退到数据库密码校验
- Error path: LDAP 认证失败 → 回退到数据库密码校验，不阻塞主流程
- Happy path: LDAP_ENABLE=false → 正常走数据库密码校验流程
- Error path: LDAP 连接超时 3 秒 → 失败不阻塞主流程

**Verification:**
- `cargo test -p ldap` 通过
- `Cargo.toml` 中新增 `ldappress` 依赖
- LDAP 端点注册到 auth crate 的 router() 函数

### U9. 授权管理 CRUD（Empower）

**Goal:** 补全授权管理 CRUD：创建、查询、更新、删除、启用、禁用授权端点

**Requirements:** R25-R28

**Dependencies:** None

**Files:**
- Create: `crates/empower/src/lib.rs`（授权管理模块）
- Create: `crates/empower/src/router.rs`（路由注册）
- Modify: `crates/person/src/lib.rs`（新增 empower 路由注册）
- Test: `crates/empower/src/tests.rs`（新增测试用例）

**Approach:**
- POST /jaxrs/person/empower：创建授权
- GET /jaxrs/person/empower/{id}：查询授权
- PUT /jaxrs/person/empower/{id}：更新授权
- DELETE /jaxrs/person/empower/{id}：删除授权
- GET /jaxrs/person/empower/{id}/enable：启用授权
- GET /jaxrs/person/empower/{id}/disable：禁用授权
- POST /jaxrs/person/empower/manager：管理员创建授权
- PUT /jaxrs/person/empower/manager/{id}：管理员更新授权
- DELETE /jaxrs/person/empower/manager/{id}：管理员删除授权
- POST /jaxrs/person/empower/manager/list/paging/{page}/size/{size}：管理员分页查询
- GET /jaxrs/person/empower/list/currentperson：查询当前用户授权
- GET /jaxrs/person/empower/list/currentperson/enable：查询当前用户生效授权
- GET /jaxrs/person/empower/list/to：查询我拥有的被授权
- GET /jaxrs/person/empower/list/to/enable：查询我生效的被授权
- 管理员可管理他人授权，普通用户只能管理自身授权
- 复用 existing auth_role 角色体系
- IDOR 防护：所有写操作端点（POST/PUT/DELETE /jaxrs/person/empower/{id}）必须调用 `require_owner` 验证当前用户是该授权的 owner，防止跨用户篡改（遵循 institutional learning: IDOR 安全修复）
- enable/disable 端点改为 POST 方法（避免 CSRF 风险）：POST /jaxrs/person/empower/{id}/enable 和 POST /jaxrs/person/empower/{id}/disable

**Test scenarios:**
- Happy path: admin POST /jaxrs/person/empower/manager → 创建授权写入数据库
- Happy path: admin GET /jaxrs/person/empower/{id} → 返回授权信息
- Happy path: admin PUT /jaxrs/person/empower/{id} → 更新授权
- Happy path: admin DELETE /jaxrs/person/empower/{id} → 删除授权
- Happy path: admin GET /jaxrs/person/empower/{id}/enable → 启用授权
- Happy path: admin GET /jaxrs/person/empower/{id}/disable → 禁用授权
- Error path: 普通用户 POST /jaxrs/person/empower → 返回 403 Forbidden
- Error path: 普通用户 GET /jaxrs/person/empower/{id}（非自身授权）→ 返回 403 Forbidden（require_owner 验证）
- Error path: 普通用户 PUT/DELETE /jaxrs/person/empower/{id}（非自身授权）→ 返回 403 Forbidden
- Happy path: 普通用户 GET /jaxrs/person/empower/{id}（自身授权）→ 返回自身授权
- Happy path: 普通用户 GET /jaxrs/person/empower/list/currentperson → 返回当前用户授权列表
- Happy path: 普通用户 GET /jaxrs/person/empower/list/to → 返回其拥有的被授权

**Verification:**
- `cargo test -p empower` 通过
- 新端点注册到 empower crate 的 router() 函数
- 权限控制正确（管理员可管理他人授权，普通用户只能管理自身授权）

### U10. null 桩修复（correlation 和 hotpic）

**Goal:** 修复 correlation 和 hotpic 模块 delete handler 的 null 桩问题，对齐 Java 契约

**Requirements:** R10, R11, R12

**Dependencies:** U7（需确保响应结构一致）

**Files:**
- Modify: `crates/correlation/src/lib.rs`（correlation delete handler）
- Modify: `crates/hotpic/src/lib.rs`（hotpic delete handler + list handler）
- Test: `crates/correlation/src/tests.rs`（新增测试用例）
- Test: `crates/hotpic/src/tests.rs`（新增测试用例）

**Approach:**
- correlation_core_entity delete 端点：DELETE /jaxrs/correlation/core/entity/delete/{id} → 返回 `ActionResult::success(json!({"success": true}))` 而非 `Value::Null`
- hotpic_core_entity delete 端点：DELETE /jaxrs/hotpic/core/entity/delete/{id} → 返回 `ActionResult::success(json!({"success": true}))` 而非 `Value::Null`
- hotpic list 端点：返回的 data 数组必须包含 base64 字段
- 修复后 `cargo test -p correlation_core_entity` 和 `cargo test -p hotpic_core_entity` 全部通过

**Test scenarios:**
- Happy path: DELETE /jaxrs/correlation/core/entity/delete/{id} → 返回 `ActionResult::success(json!({"success": true}))`
- Happy path: DELETE /jaxrs/hotpic/core/entity/delete/{id} → 返回 `ActionResult::success(json!({"success": true}))`
- Happy path: GET /jaxrs/hotpic/core/entity/list → 返回的 data 数组每条包含 base64 字段
- Error path: 删除操作涉及未授权 → 返回 403
- Error path: 删除操作涉及非存在 ID → 返回 error

**Verification:**
- `cargo test -p correlation_core_entity` 通过
- `cargo test -p hotpic_core_entity` 通过

## System-Wide Impact

- **Interaction graph：** U1-U3 扩展 auth crate，影响 auth/lib.rs 的 router 组装；U4-U6 扩展 person crate，影响个人设置相关前端流程；U7 扩展 express crate，影响批量查询 API；U8 扩展 ldap crate，影响认证流程；U9 扩展 empower crate，影响授权管理端点集成；U10 修复 correlation/hotpic crate 的 null 桩
- **Error propagation：** 新增认证端点需正确传递 AppError（Unauthorized/Forbidden/Internal），中间件层自动转换为 HTTP 状态码
- **State lifecycle risks：** 安全注销需确保 SessionManager 的 session 移除是原子操作，避免并发注销导致 session 残留
- **API surface parity：** 新增端点需保持与 Java 端的响应结构一致（ActionResult<T> 9 字段），确保前端 o2web 无需适配
- **Integration coverage：** U7 行为对比测试是全量回归的安全网，确保新增端点与 Java 端行为等效
- **Unchanged invariants：** ActionResult<T> 9 字段结构不变；PermissionRegistry 扩展（新增路径注册）。SessionManager 需新增 `remove_sessions_by_person` 方法（U1 包含此修改）。

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| LDAP 认证模块依赖 ldap3/crate 不可用 | Medium | High | ldappress 作为备选方案；若 ldap3 不可用，回退到数据库密码校验 |
| TokenThreshold 实体迁移未执行 | Low | High | 在 migrations 目录确认 TokenThreshold 表已创建；迁移脚本需幂等 |
| 批量查询路由路径与 Java 不一致 | Medium | Medium | 在 express 模块中确认路由注册，与 Java 对齐 |
| empower 数据表 x_empower schema 缺失 | Medium | Medium | 在 migrations 目录确认 x_empower 表已存在；迁移脚本需幂等 |
| 批量查询端点无认证时并发安全 | Low | Medium | 数据库 UNIQUE 约束 + 应用层预检查，并发时由 DB 约束兜底 |
| 数据库迁移失败导致服务中断 | Low | High | 迁移脚本需幂等；先执行迁移验证再切换 |
| 响应结构变化导致前端 o2web 渲染异常 | Medium | Medium | 保持 ActionResult<T> 9 字段结构一致，所有端点返回相同结构 |

## Documentation / Operational Notes

- 更新 `oa4rust/README.md`：添加新端点列表和 MCP/OpenAPI 使用说明
- 更新 `docs/brainstorms/oa4rust-endpoint-inventory.md`：重新生成，确认全量 done 状态
- 更新 `docs/brainstorms/oa4rust-migration-status.md`：同步更新
- 更新 `docs/oa/` 模块卡片：为新增认证端点补充 REST Endpoints 字段
- 运维：新增的安全注销和 SSO 端点需监控异常调用（防暴力破解）
- 配置文档：新增环境变量文档（LDAP_URL, LDAP_BASE_DN, LDAP_BIND_USER, LDAP_BIND_PWD, LDAP_ENABLE, ldappress）

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-11-oa4rust-full-gap-closure-requirements.md](docs/brainstorms/2026-08-11-oa4rust-full-gap-closure-requirements.md)
- **Related code:** `crates/auth/src/`, `crates/personal/src/`, `crates/express/src/`, `crates/correlation/src/`, `crates/hotpic/src/`, `crates/ldap/src/`, `crates/empower/src/`
- **Related scripts:** `scripts/gen_inventory.py`, `scripts/extract_endpoints.py`
- **Related solutions:** `docs/solutions/architecture-patterns/actionresult-9-field-contract.md`, `docs/solutions/security-issues/idor-vulnerability-write-handlers.md`
- **Related PRs/issues:** #（待补充）

## Deferred / Open Questions

### From 2026-08-11 review

- **U1 Approach — Two-phase flow redesign** — Implementation Unit 1 (P0, coherence, feasibility, confidence 100)

  U1 describes a single-request two-factor flow (credential+password+code in one handler) but R4 requires two separate endpoints (send code, then verify). The approach section needs to be rewritten to match the two-phase design described in U2.

  <!-- dedup-key: section="implementation unit 1 approach" title="two-phase flow redesign" evidence="U1 describes a single-request two-factor flow but R4 requires two separate endpoints" -->

- **U1/U2/U3 — Files Create vs Modify ownership** — Implementation Units 1/2/3 (P0, coherence, feasibility, confidence 100)

  U1 contains U2/U3 functionality but they are listed as separate implementation units. Need to clarify: does U1 own the implementation, or should U2/U3 be the primary units? Risk of duplicate implementation or gaps.

  <!-- dedup-key: section="implementation units 123" title="files create vs modify ownership" evidence="U1 contains U2/U3 functionality but listed as separate units" -->

- **U8 — ldappress dependency not in Cargo.toml** — Implementation Unit 8 (P1, feasibility, confidence 100)

  Open Questions claims ldappress is added to Cargo.toml, but the dependency is not actually present. U8 won't compile without it.

  <!-- dedup-key: section="implementation unit 8" title="ldappress dependency not in cargo toml" evidence="Open Questions claims ldappress is added to Cargo.toml but not present" -->

- **U9 — x_empower and TokenThreshold migrations missing** — Implementation Unit 9 (P1, feasibility, confidence 100)

  Open Questions claims these migrations exist, but they are not in the migrations directory. Runtime failures will occur without them.

  <!-- dedup-key: section="implementation unit 9" title="x empower and tokenthreshold migrations missing" evidence="Open Questions claims migrations exist but not in migrations directory" -->

- **U10 — correlation/hotpic have no delete handler** — Implementation Unit 10 (P1, feasibility, confidence 100)

  U10 claims to fix delete handlers in correlation and hotpic crates, but neither crate currently has a delete handler. Nothing to modify.

  <!-- dedup-key: section="implementation unit 10" title="correlation hotpic have no delete handler" evidence="neither crate currently has a delete handler" -->

- **U7 — express crate is express delivery, not batch query** — Implementation Unit 7 (P1, feasibility, confidence 75)

  The existing express crate implements express delivery tracking (X.EXPRESS_INFO), not batch query of persons/units/identities. Adding batch query endpoints to this crate creates semantic confusion and misaligns with Java module structure.

  <!-- dedup-key: section="implementation unit 7" title="express crate is express delivery not batch query" evidence="existing express crate implements express delivery tracking" -->

- **U8 — LDAP silent fallback needs monitoring** — Implementation Unit 8 (P1, adversarial, confidence 75)

  LDAP authentication failure silently falls back to database password validation with no monitoring or alerting. Auth failures are invisible to operators.

  <!-- dedup-key: section="implementation unit 8" title="ldap silent fallback needs monitoring" evidence="LDAP authentication failure silently falls back with no monitoring" -->

- **Summary — "100% Java OA alignment" framing contradicts deferred items** — Summary (P1, product-lens, adversarial, confidence 75)

  The plan claims to achieve "100% Java OA alignment" but defers SSO, cache layer, recursive navigation, and process platform features. These are not peripheral — SSO is core auth, cache affects performance. The framing sets unrealistic expectations.

  <!-- dedup-key: section="summary" title="100% java oa alignment framing contradicts deferred items" evidence="defers SSO cache layer recursive navigation process platform" -->

- **U8 — LDAP user provisioning and password policy divergence** — Implementation Unit 8 (P1, product-lens, confidence 75)

  LDAP integration introduces new external identity provider dependency. Unaddressed: (1) LDAP vs local user password policy divergence, (2) auto-creation of local auth_person records on first LDAP login, (3) bcrypt/MD5/DES vs LDAP auth priority, (4) LDAP connection pool management and failover.

  <!-- dedup-key: section="implementation unit 8" title="ldap user provisioning and password policy divergence" evidence="unaddressed password policy divergence and user provisioning" -->

- **U7 — Batch query without auth: network isolation assumption unverified** — Implementation Unit 7 (P1, product-lens, security, confidence 75)

  R24 declares batch query endpoints require no authentication, assuming "frontend internal calling scenario." But if the API is network-accessible, anyone can enumerate all person data. No discussion of whether Java has network-layer access control that oa4rust is missing.

  <!-- dedup-key: section="implementation unit 7" title="batch query without auth network isolation assumption unverified" evidence="assuming frontend internal calling scenario without verifying network isolation" -->

- **U1 — Security fixes bundled with new features delays critical patches** — Implementation Unit 1 (P1, product-lens, confidence 75)

  Login security checks (locked/passwordExpired) and two-factor flow fixes are security-critical and should deploy independently. LDAP integration and empower CRUD are new features with separate release timelines. Bundling them means security fixes are blocked by feature development progress.

  <!-- dedup-key: section="implementation unit 1" title="security fixes bundled with new features delays critical patches" evidence="login security checks and two-factor flow fixes are security-critical" -->

- **U5 — Icon GET access: R8 says public, test says 401** — Implementation Unit 5 (P1, product-lens, security, confidence 100)

  R8 explicitly states GET icon endpoint has no permission requirement (无权限也可访问), but the test scenario expects 401 for unauthenticated users. Direct contradiction.

  <!-- dedup-key: section="implementation unit 5" title="icon get access r8 says public test says 401" evidence="R8 explicitly states no permission required but test expects 401" -->

- **U1 — U1 contains U2/U3 but listed as separate units** — Implementation Unit 1 (P2, product-lens, confidence 100)

  U1's description already includes two-factor login, safe_logout, token check, and switch_user. U2 and U3 are subsets of U1 but listed as separate implementation units with their own dependencies and verification. Creates ownership confusion.

  <!-- dedup-key: section="implementation unit 1" title="u1 contains u2u3 but listed as separate units" evidence="U1 description includes two-factor login safe_logout token check switch_user" -->

- **U3 — safe_logout broadcast mechanism undefined** — Implementation Unit 3 (P2, adversarial, confidence 75)

  U3 Approach says "use shared state or event bus" for multi-instance broadcast, but this is an architectural decision that needs deeper design. No discussion of message queue, HTTP callback, or shared database approaches.

  <!-- dedup-key: section="implementation unit 3" title="safe logout broadcast mechanism undefined" evidence="approach says use shared state or event bus without deeper design" -->

- **U4 — Electronic signature storage: BLOB vs filesystem conflict** — Implementation Unit 4 (P2, adversarial, confidence 75)

  Key Decisions says use PostgreSQL BLOB (base64 string), but Deferred to Implementation says "local filesystem vs object storage, needs evaluation." These are contradictory decisions.

  <!-- dedup-key: section="implementation unit 4" title="electronic signature storage blob vs filesystem conflict" evidence="Key Decisions says BLOB but Deferred says evaluate filesystem" -->

- **U9 — Empower CRUD missing authorization content validation** — Implementation Unit 9 (P2, adversarial, confidence 75)

  R28 only constrains "who can manage whose authorization" but does not validate the content of the authorization itself. What can be granted? Are there limits on scope or duration? Risk of privilege escalation through authorization content.

  <!-- dedup-key: section="implementation unit 9" title="empower crud missing authorization content validation" evidence="R28 only constrains who can manage whose authorization" -->