---
title: OA4Rust 认证提供方补全与工具链验证
type: feat
status: completed
date: 2026-08-10
origin: docs/brainstorms/2026-08-10-oa4rust-auth-and-toolchain-closure-requirements.md
---

# OA4Rust 认证提供方补全与工具链验证计划

## Summary

创建 7 个实施单元：2 个数据库迁移（sso_client 表 + mpwxopenId 字段）、5 个 OAuth 提供方模块（mpweixin/welink/zhengwudingding/andfx/qiyeweixin）、SSO GET 端点、路由注册与脚本重生成。MCP 工具桥接（~8080 条自动生成路由）和行为对比测试（985 个端点）已完备，本计划仅需在新增路由后重新运行生成脚本即可。

---

## Problem Frame

`docs/brainstorms/2026-08-10-oa4rust-auth-and-toolchain-closure-requirements.md` 中部分数据已过时：MCP 实际已有 ~8080 条自动生成路由（非 ~100），behavior_compare 已有 985 个端点（非 79）。真实缺口集中在认证提供方覆盖（5 个第三方 OAuth 全部缺失）和 SSO GET 端点缺失。工具链脚本（gen_mcp_tools.py、gen_openapi_paths.py）已存在且工作正常，仅需在新增路由后重运行。

---

## Requirements

- R1. 实现微信小程序登录模块（`auth/src/mpweixin.rs`），4 个端点
- R2. 实现 WeLink 登录模块（`auth/src/welink.rs`），1 个端点
- R3. 实现政务钉钉登录模块（`auth/src/zhengwudingding.rs`），2 个端点
- R4. 实现移动办公 SSO 模块（`auth/src/andfx.rs`），1 个端点
- R5. 实现企业微信点单登录模块（`auth/src/qiyeweixin.rs`），3 个端点
- R6. 所有 OAuth 提供方遵循现有 `auth/oauth.rs` 模式，返回完整用户信息 + token
- R7. 每个提供方模块添加单元测试
- R8. 创建 `migrations/013_add_oauth_fields.sql`，新增 `auth_person.mpwxopenId` 字段
- R9. 在 `auth/src/sso.rs` 新增 SSO GET 端点
- R10. 创建 `migrations/014_add_sso_client.sql`，新增 `sso_client` 表
- R11. 重运行 `scripts/gen_mcp_tools.py` 和 `scripts/gen_openapi_paths.py`
- R12. `cargo test --workspace --lib` 全部通过
- R13. `cargo check --workspace` 无新增 error

**Origin actors:** A1（开发者）, A2（前端 o2web）, A3（AI Agent / MCP 客户端）
**Origin flows:** F1（第三方 OAuth 登录流程）, F2（SSO 浏览器重定向登录流程）, F3（工具链扩展流程）
**Origin acceptance examples:** AE1（OAuth 端点行为）, AE2（SSO GET 行为）, AE3（数据库迁移验证）, AE4（MCP 工具覆盖）, AE5（OpenAPI 文档）

---

## Scope Boundaries

- **包含：** 5 个 OAuth 提供方模块、SSO GET 端点、2 个数据库迁移、生成脚本重运行
- **排除在外：** 电子签名管理（`personal/signature/`，deferred）、console 模块完整实现、AI 模块 stub 补全

### Deferred for later

- 电子签名管理（`personal/signature/`）—— 低频使用场景
- console 模块完整实现 —— 全部桩代码，低频使用
- AI 模块 MCP/文件/索引端点补全 —— AI 功能框架本身未完成
- SQLx 完全移除 —— 低优先级

### Outside this product's identity

- 前端 o2web 的重写或现代化改造
- 独立的 OAuth 提供商 SDK 发布
- Java 服务的永久下线

---

## Context & Research

### Relevant Code and Patterns

- **OAuth 提供方模式：** `crates/auth/src/oauth.rs` — 环境变量配置、`unique_id` 前缀绑定、`login_or_create_user()` 统一创建会话
- **SSO 解密逻辑：** `crates/auth/src/sso.rs` — `decrypt_sso_token()` 共享函数（3DES EDE2 + base64url）
- **密码哈希：** `crates/auth/src/password.rs` — `hash_password()` / `verify_password()`
- **Session 管理：** `shared/src/session.rs` — `SessionManager::create_session()`
- **路由注册：** `crates/auth/src/lib.rs:router()` — axum Router `.route()` 模式
- **Migration 模式：** `migrations/{NNN}_{description}.sql` + 配套 rollback 文件
- **MCP 生成脚本：** `scripts/gen_mcp_tools.py` → `crates/mcp_server/src/generated_routes.rs`
- **OpenAPI 生成脚本：** `scripts/gen_openapi_paths.py` → `crates/openapi/src/lib.rs`
- **Auth crate 测试模式：** `crates/auth/src/tests.rs`

### Institutional Learnings

- `docs/solutions/security-issues/idor-vulnerability-write-handlers.md` — IDOR 安全修复模式
- `docs/solutions/architecture-patterns/actionresult-9-field-contract.md` — `ActionResult<T>` 9 字段契约
- `docs/solutions/database-issues/postgresql-uppercase-identifier-trap.md` — PostgreSQL 标识符大小写陷阱

### External References

- 微信小程序 OAuth2.0 文档：`code` → `access_token` → `openid` 交换流程
- 华为 WeLink OAuth：`https://open.welink.huaweicloud.com/api/auth/v2/userid?code={code}`
- 政务钉钉 API：两步映射（code → dingUserId → userId）
- 企业微信 JSSDK 签名：SHA1(jsapi_ticket+noncestr+timestamp+url)

---

## Key Technical Decisions

- **OAuth 模块拆分：** 每个提供方独立为 `auth/src/{provider}.rs`，与 `oauth.rs`（qywx/dingding）并行，通过 `lib.rs` 的 `pub mod` 导出
- **SSO key 存储：** 新建 `sso_client` 表存储 client→key 映射（用户决策，与 Java 配置方式不同但更灵活）
- **mpwxopenId 字段：** 直接在 `auth_person` 新增列，不新增独立表（与 Java 实体对齐）
- **MCP/behavior_compare：** 无需新增实现——脚本已存在且生成结果已加载，仅需重运行
- **OpenAPI：** 758 条路径已生成，重运行脚本后自动扩展至新增端点

---

## Open Questions

### Resolved During Planning

- **SSO client→key 存储方式：** Java 从 Config 读取，Rust 侧新建 `sso_client` 数据库表 — 用户已确认
- **OAuth 登录返回字段：** 返回完整用户信息（name/mobile/email/roles）+ token，与 Java 对齐 — 用户已确认

### Deferred to Implementation

- **sso_client 表的初始数据：** 需要多少条 SSO client 记录、key 的初始值 — 实现时根据实际部署配置填充
- **mpweixin 模板消息发送：** `testSendTempMessage` 端点的具体消息格式 — 实现时参考 Java `ActionTestSendTempMessage`
- **andfx token 解析格式：** 移动办公 SSO token 的具体格式（明文/加密）— 实现时参考 Java `ActionMoaLogin`

---

## Implementation Units

### U1. 数据库迁移 — sso_client 表与 mpwxopenId 字段

**Goal：** 创建 `sso_client` 表存储 SSO client→key 映射，为 `auth_person` 表新增 `mpwxopenId` 字段支持微信小程序 openid 绑定。

**Requirements：** R8, R10

**Dependencies：** 无

**Files:**
- Create: `migrations/014_add_sso_client.sql`
- Create: `migrations/014_add_sso_client_rollback.sql`
- Create: `migrations/013_add_oauth_fields.sql`
- Create: `migrations/013_add_oauth_fields_rollback.sql`

**Approach：**
- `013_add_oauth_fields.sql`：`ALTER TABLE auth_person ADD COLUMN IF NOT EXISTS mpwxopenId VARCHAR(255)` — 与 Java `Person.mpweixinOpenId_FIELDNAME` 对齐
- `014_add_sso_client.sql`：创建 `sso_client` 表，包含 `id VARCHAR(255) PRIMARY KEY`、`client_name VARCHAR(255) UNIQUE NOT NULL`、`key VARCHAR(255) NOT NULL`、`created_at TIMESTAMP DEFAULT NOW()`。使用 `IF NOT EXISTS` 确保幂等
- Rollback 文件对应删除表和列

**Patterns to follow：**
- `migrations/012_add_creator_person.sql` — 最简单的 ALTER TABLE 模式
- `migrations/001_create_auth_tables.sql` — 表创建模式（IF NOT EXISTS）

**Test scenarios:**
- Happy path: 运行 migration 后 `auth_person` 表存在 `mpwxopenId` 列
- Happy path: 运行 migration 后 `sso_client` 表存在且包含 `id`、`client_name`、`key` 列
- Edge case: 重复运行 migration（幂等性）— `IF NOT EXISTS` 确保不报错
- Error path: rollback 后列和表被正确移除

**Verification:**
- `psql` 连接后 `\d auth_person` 显示 `mpwxopenId` 列
- `\d sso_client` 显示表结构正确
- `cargo test --workspace --lib` 通过

---

### U2. 微信小程序登录模块（mpweixin）

**Goal：** 实现 微信小程序 OAuth 登录全流程：code 登录、绑定 openid、测试消息推送。

**Requirements：** R1, R6, R7

**Dependencies：** U1

**Files:**
- Create: `crates/auth/src/mpweixin.rs`
- Modify: `crates/auth/src/lib.rs`（添加 `pub mod mpweixin` 和路由注册）
- Create: `crates/auth/src/mpweixin_tests.rs`（或内联 `#[cfg(test)] mod tests`）

**Approach：**
- 参考 `oauth.rs` 中 qywx/dingding 的模式
- 环境变量：`MPWEXIN_APP_ID`、`MPWEXIN_APP_SECRET`
- `unique_id` 前缀：`mpwx_{openid}`
- 4 个端点：
  - `GET /jaxrs/mpweixin/login/code/{code}` — code→openid→查/建用户→返回 token+用户信息（未绑定返回 `unbind=true`）
  - `GET /jaxrs/mpweixin/bind/code/{code}` — code→openid→绑定到当前登录用户（需认证）
  - `GET /jaxrs/mpweixin/bind/openid/{openid}` — 直接绑定 openid（需认证）
  - `POST /jaxrs/mpweixin/menu/test/send/to/{person}` — 管理员发送模板消息（admin only）
- 微信小程序 access_token 缓存（内存 HashMap，5 分钟 TTL）
- 返回完整用户信息：`token` + `person`（unique、name、mobile、email、icon）+ `unbind` 标识

**Technical design（directional）：**
```
login/code/{code}:
  1. 用 code 换取 access_token（POST https://api.weixin.qq.com/sns/oauth2/access_token）
  2. 提取 openid
  3. 查询 auth_person WHERE unique_id = 'mpwx_{openid}'
  4. 存在 → 创建 session，返回 token + 用户信息
  5. 不存在 → 返回 { unbind: true, mpwxopenId: openid }

bind/code/{code}:
  1. 用 code 换取 openid（同上）
  2. 更新 auth_person SET mpwxopenId = openid WHERE unique_id = 当前用户

bind/openid/{openid}:
  1. 直接更新 auth_person SET mpwxopenId = openid WHERE unique_id = 当前用户
```

**Patterns to follow：**
- `crates/auth/src/oauth.rs` — OAuth 提供方通用模式
- `crates/auth/src/sso.rs` — 会话创建模式

**Test scenarios:**
- Happy path: 有效 code → 返回 token + person（用户已绑定）
- Happy path: 有效 code → 返回 `unbind=true` + `mpwxopenId`（用户未绑定）
- Edge case: 无 MPWEXIN_APP_ID 环境变量 → 返回配置错误
- Error path: 无效 code → 返回 400
- Error path: code 过期 → 返回 400
- Integration: bind/code → 后续 login 可直接登录（openid 已绑定）

**Verification:**
- `cargo test -p auth` 通过
- `cargo check --workspace` 通过

---

### U3. WeLink 登录模块（welink）

**Goal：** 实现华为 WeLink 扫码登录。

**Requirements：** R2, R6, R7

**Dependencies：** U1

**Files:**
- Create: `crates/auth/src/welink.rs`
- Modify: `crates/auth/src/lib.rs`（路由注册）

**Approach：**
- 环境变量：`WELINK_APP_KEY`、`WELINK_APP_SECRET`
- `unique_id` 前缀：`welink_{userId}`
- 1 个端点：`GET /jaxrs/welink/code/{code}`
- API：`https://open.welink.huaweicloud.com/api/auth/v2/userid?code={code}` + Header `WeLink-Auth-Key: {accessToken}`
- accessToken 通过 `app_key` + `app_secret` 换取
- 用户不存在时返回错误（WeLink 不需要 unbind 流程，用户需预先在 OA 中创建）

**Patterns to follow：**
- `crates/auth/src/oauth.rs` — dingding 实现模式（类似的两步 code→userid 流程）

**Test scenarios:**
- Happy path: 有效 code → 返回 token + 用户信息
- Error path: 无效 code → 返回 400
- Error path: 用户未绑定 → 返回 404

**Verification:**
- `cargo test -p auth` 通过

---

### U4. 政务钉钉登录模块（zhengwudingding）

**Goal：** 实现政务钉钉两步映射登录。

**Requirements：** R3, R6, R7

**Dependencies：** U1

**Files:**
- Create: `crates/auth/src/zhengwudingding.rs`
- Modify: `crates/auth/src/lib.rs`（路由注册）

**Approach：**
- 环境变量：`ZWDINGDING_API_BASE`、`ZWDINGDING_CORP_ACCESS_TOKEN`、`ZWDINGDING_APP_ACCESS_TOKEN`
- `unique_id` 前缀：`zwding_{userId}`
- 2 个端点：
  - `GET /jaxrs/zhengwudingding/code/{code}` — code→dingUserId→userId 两步映射→登录
  - `GET /jaxrs/zhengwudingding/info` — 获取配置状态（enable + client 列表）
- 两步映射：
  1. `GET {api_base}/user/getuserinfo?access_token={corp_token}&code={code}` → dingUserId
  2. `POST {api_base}/user/singleGetUserIdByDingId?access_token={app_token}&dingUserId={dingUserId}` → userId
- 用户通过 `zhengwudingdingId` 字段匹配（需确认 `auth_person` 是否有此字段，如无则用 `unique_id` 前缀）

**Technical note：** 需确认 `auth_person` 表是否有 `zhengwudingdingId` 字段。如不存在，使用 `unique_id = 'zwding_{userId}'` 前缀约定。

**Test scenarios:**
- Happy path: 有效 code → 返回 token + 用户信息
- Error path: 第一步映射失败（无效 code）→ 返回 400
- Error path: 第二步映射失败（dingUserId 无对应 userId）→ 返回 404
- Happy path: `/info` 端点返回配置状态

**Verification:**
- `cargo test -p auth` 通过

---

### U5. 移动办公 SSO 模块（andfx）

**Goal：** 实现 Android/iOS MOA APP SSO 登录。

**Requirements：** R4, R6, R7

**Dependencies：** U1

**Files:**
- Create: `crates/auth/src/andfx.rs`
- Modify: `crates/auth/src/lib.rs`（路由注册）

**Approach：**
- `unique_id` 前缀：`andfx_{token}`
- 1 个端点：`GET /jaxrs/andfx/moa/sso/token/{token}/enter/{enterId}`
- token 解析：参考 Java `ActionMoaLogin`，token 为加密格式，enterId 为企业 ID 校验
- 实现时参考 Java 源码确定 token 解密逻辑

**Patterns to follow：**
- `crates/auth/src/sso.rs` — SSO token 解密模式

**Test scenarios:**
- Happy path: 有效 token + enterId → 返回 token + 用户信息
- Error path: 无效 token → 返回 400
- Error path: enterId 不匹配 → 返回 403

**Verification:**
- `cargo test -p auth` 通过

---

### U6. 企业微信点单登录模块（qiyeweixin）

**Goal：** 实现企业微信扫码点单登录 + JSSDK 签名。

**Requirements:** R5, R6, R7

**Dependencies:** U1

**Files:**
- Create: `crates/auth/src/qiyeweixin.rs`
- Modify: `crates/auth/src/lib.rs`（路由注册）

**Approach:**
- 环境变量：`QYWX_CORP_ID`（复用现有企微配置）、`QYWX_AGENT_ID`、`QYWX_APP_SECRET`
- 3 个端点：
  - `GET /jaxrs/qiyeweixin/code/{code}` — 企业微信扫码登录
  - `GET /jaxrs/qiyeweixin/update/person/detail/{code}` — 登录并同步用户详细信息
  - `POST /jar/rs/qiyeweixin/jssdk/sign/info` — JSSDK 签名（SHA1(jsapi_ticket+noncestr+timestamp+url)）
- JSSDK 签名算法：`sha1("jsapi_ticket={ticket}&noncestr={nonce}&timestamp={ts}&url={url}")`
- `jsticket` 类型：支持 `app` 类型（应用 ticket）和普通企业 ticket

**Patterns to follow:**
- `crates/auth/src/oauth.rs` — 企微 OAuth 基础流程
- `crates/auth/src/oauth.rs::verify_wechat_signature()` — 现有签名验证逻辑可复用

**Test scenarios:**
- Happy path: 有效 code → 返回 token + 用户信息
- Happy path: JSSDK 签名 → 返回有效 signature + nonceStr + timestamp
- Edge case: 未配置企微 → 返回配置错误
- Error path: URL 为空 → 返回 400

**Verification:**
- `cargo test -p auth` 通过

---

### U7. SSO GET 端点

**Goal：** 新增 `GET /jaxrs/authentication/sso/client/{client}/token/{token}` 端点，支持浏览器 URL 重定向场景。

**Requirements:** R9

**Dependencies:** U1, U10（sso_client 表）

**Files:**
- Modify: `crates/auth/src/sso.rs`（新增 GET 处理器）
- Modify: `crates/auth/src/lib.rs`（路由注册）

**Approach:**
- 与现有 `sso_post_login()` 共享 `decrypt_sso_token()` 和 `create_sso_session()` 内部函数
- 差异：GET 端点从 path params 提取 `client` 和 `token`（POST 从 body）
- 从 `sso_client` 表查询 client→key 映射（替代 Java 的 Config 方式）
- 解密后流程与 POST 完全一致：提取 credential → 验证时效 → 查用户 → 创建会话 → 返回 token + 用户信息（含 roles）

**Technical design（directional）:**
```
GET /jaxrs/authentication/sso/client/{client}/token/{token}:
  1. 从 sso_client 表查询 client_name = {client} 的记录
  2. 获取 key
  3. 调用 decrypt_sso_token(token, key) → decrypted
  4. parse_sso_payload(decrypted) → (credential, timestamp)
  5. validate_sso_timestamp(timestamp) — 5 分钟 TTL
  6. check is_admin(credential) — 禁止管理员通过 SSO 登录
  7. 查询 auth_person WHERE unique_id = credential
  8. 创建 session，返回 token + 用户信息 + roles
```

**Patterns to follow:**
- `crates/auth/src/sso.rs` — 现有 POST 端点和解密逻辑

**Test scenarios:**
- Happy path: 有效 GET token → 返回 token + 用户信息
- Happy path: 过期 token（>5 分钟）→ 返回 400
- Error path: 未知 client → 返回 400/404
- Error path: 解密失败 → 返回 400
- Error path: 管理员账号 → 返回 403

**Verification:**
- `cargo test -p auth` 通过
- AE2: 向 GET 端点发送有效 3DES 加密 token，返回成功会话

---

### U8. 路由注册与生成脚本重运行

**Goal：** 将所有新端点注册到 `auth::router()`，重运行 MCP/OpenAPI 生成脚本。

**Requirements:** R1-R5（路由注册部分）, R11

**Dependencies:** U2, U3, U4, U5, U6, U7

**Files:**
- Modify: `crates/auth/src/lib.rs`（router 函数，添加新路由）
- Run: `python scripts/gen_mcp_tools.py`
- Run: `python scripts/gen_openapi_paths.py`

**Approach:**
- 在 `auth::router()` 中添加新路由：
  ```
  .merge(mpweixin::router())
  .merge(welink::router())
  .merge(zhengwudingding::router())
  .merge(andfx::router())
  .merge(qiyeweixin::router())
  .route("/jaxrs/authentication/sso/client/{client}/token/{token}", get(sso::sso_get_login))
  ```
- 重运行 `gen_mcp_tools.py`：生成新的 `generated_routes.rs`（新增 ~12 条路由）
- 重运行 `gen_openapi_paths.py`：生成新的 `lib.rs`（新增 ~12 条 utoipa 路径）
- 验证 `cargo check --workspace` 通过

**Patterns to follow:**
- `crates/auth/src/lib.rs:router()` — 现有路由注册模式
- `crates/auth/src/oauth.rs` — 子模块 router 模式（如 `captcha::captcha_router()`）

**Test scenarios:**
- Happy path: `cargo check --workspace` 通过
- Happy path: `cargo test --workspace --lib` 通过
- Integration: 新路由在 `generated_routes.rs` 中出现
- Integration: 新路由在 `openapi/src/lib.rs` 中出现

**Verification:**
- `cargo check --workspace` 无新增 error
- `cargo test --workspace --lib` 全部通过
- `python scripts/gen_mcp_tools.py` 输出显示新生成的路由数
- `python scripts/gen_openapi_paths.py` 输出显示新的路径数

---

## System-Wide Impact

- **Interaction graph：** 新增 OAuth 提供方不改变现有认证中间件链；SSO GET 端点复用现有 SessionManager
- **Error propagation：** 所有新端点使用 `shared::error::AppError` 统一错误处理，返回 `ActionResult<T>` 9 字段结构
- **State lifecycle risks：** `sso_client` 表需确保 key 字段的安全存储（不在日志中输出）
- **API surface parity：** 新增端点与 Java 端点对齐，前端 o2web 可通过新增路由发现新登录方式
- **Unchanged invariants：** `auth_person` 表的 `unique_id` 唯一约束不受影响；现有 OAuth 提供方（qywx/dingding）不受影响

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| 政务钉钉/移动办公 API 格式不明确 | 实现时参考 Java 源码，先写 stub 再逐步完善 |
| sso_client 表 key 字段安全性 | key 不在日志中输出，migration 中不硬编码实际 key 值 |
| utoipa derive 宏处理新增路径编译超时 | 如超时，分批生成 OpenAPI（先核心端点，后边缘端点） |
| 微信小程序 access_token 缓存并发 | 使用 Mutex<HashMap> 缓存，TTL 5 分钟，与现有 oauth.rs 模式一致 |

---

## Documentation / Operational Notes

- 新增环境变量文档：在 `.env.example` 中添加 `MPWEXIN_APP_ID`、`MPWEXIN_APP_SECRET`、`WELINK_APP_KEY`、`WELINK_APP_SECRET`、`ZWDINGDING_API_BASE`、`ZWDINGDING_CORP_ACCESS_TOKEN`、`ZWDINGDING_APP_ACCESS_TOKEN`
- `sso_client` 表初始数据需运维手动插入（key 值从 Java Config 迁移）

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-10-oa4rust-auth-and-toolchain-closure-requirements.md](../brainstorms/2026-08-10-oa4rust-auth-and-toolchain-closure-requirements.md)
- **Related code:** `crates/auth/src/oauth.rs`, `crates/auth/src/sso.rs`, `scripts/gen_mcp_tools.py`, `scripts/gen_openapi_paths.py`
- **Java reference:** `oa/o2server/x_organization_assemble_authentication/` 对应 Action 类
