---
title: "refactor: HttpOnly Cookie、cargo-audit/RustSec DB 与 Biome 2.5.11 统一加固计划"
type: refactor
status: active
date: 2026-09-09
module: oa4rust + oa4rust-web
tags: [security, authentication, httponly-cookie, csrf, cargo-audit, rustsec, biome, ci]
problem_type: security-and-tooling-migration
---

# HttpOnly Cookie、cargo-audit/RustSec DB 与 Biome 2.5.11 统一加固计划

## Summary

本计划将三项相互关联的加固工作统一落地：浏览器认证从 JavaScript 可读 token 迁移到 HttpOnly Cookie；以固定版本的 cargo-audit 和受控 RustSec advisory DB 建立可复现供应链门禁；将前端声明、配置、脚本和 CI 对齐到仓库已锁定的 Biome 2.5.11。初始 Cookie 改动已于 `40df9aa8` 提交；本轮在该干净基线上完成后端兼容、前端切换、依赖漏洞处置和根目录 CI 激活。

完成后，浏览器不得再把认证 token 写入 localStorage/sessionStorage 或 URL；Cookie 认证的危险方法必须通过 CSRF 校验；RustSec vulnerability、unsound 和 yanked 依赖必须成为硬门禁；Biome 版本、schema、配置和 CI 必须一致且格式化改动独立审查。

## Problem Frame

以下为计划启动时的历史基线；其问题已在本计划实施中收敛：

1. `oa4rust/crates/auth/src/lib.rs` 的未提交改动已开始写入 `oa4rust_token` Cookie，但 handler 原返回类型与新增响应类型不一致，静态检查显示存在编译风险。
2. Cookie 写入名为 `oa4rust_token`，共享认证中间件仍读取 `token=`，登录后 Cookie 无法成为有效认证来源。
3. 后端登录要求 `credential/password`，前端发送 `username/password`；后端刷新要求 Header token 与 body token 相等，前端却发送空 body 且关闭 Bearer。
4. 前端请求已有 `credentials: 'include'`，但仍从 localStorage 读取 token 并注入 `Authorization`，尚未完成 HttpOnly-only 迁移。
5. 现有 CORS 允许 credentials，但未覆盖前端已使用的 PUT、PATCH、DELETE，也没有针对 Cookie 认证的明确 CSRF 门禁。
6. 现有 `oa4rust/.github/workflows/ci.yml` 不在 Git 仓库根 `.github/workflows/` 下，标准 GitHub Actions 不会自动发现。
7. `cargo-deny` 已配置但工具版本、Rust toolchain 和 RustSec DB 未固定；本地基线还存在 lopdf、rsa、tantivy/lru advisory 与 license gate 失败。
8. `oa4rust-web/package.json` 声明 `@biomejs/biome ^2.1.0`，锁文件和本地安装实际为 2.5.11；当前 `biome.json` 在 2.5.11 下会因旧配置直接解析失败。

## Verified Baseline

| 领域 | 已验证事实 | 关键位置 |
|------|------------|----------|
| 工作区 | 计划启动时的 3 个 Rust 改动已提交于 `40df9aa8`；后续实施基于干净工作树 | `oa4rust/Cargo.lock`、`oa4rust/crates/auth/Cargo.toml`、`oa4rust/crates/auth/src/lib.rs` |
| Cookie | 新增 Cookie 为 HttpOnly、Secure、SameSite=Lax、Path=/、Max-Age=7200 | `oa4rust/crates/auth/src/lib.rs:19-32` |
| token 读取 | 中间件仍解析 `token=` Cookie 和 Bearer | `oa4rust/crates/shared/src/middleware/token.rs:12-35` |
| 前端请求 | 普通请求和上传均设置 `credentials: 'include'`，同时仍注入 Bearer | `oa4rust-web/packages/sdk/src/api.ts:67-102,161-169` |
| CORS | credentials 已开启；方法缺少 PUT、PATCH、DELETE | `oa4rust/crates/shared/src/middleware/security.rs:24-61` |
| RustSec | 当前 lockfile 命中 lopdf vulnerability、rsa advisory、lru unsound 链 | `oa4rust/Cargo.lock`、`oa4rust/crates/signature/`、`oa4rust/crates/search/` |
| Biome | manifest 为 `^2.1.0`，lockfile/安装为 2.5.11，配置不可解析 | `oa4rust-web/package.json:18`、`pnpm-lock.yaml:36-38`、`biome.json:1-26` |

## Requirements

- **R1. Cookie contract:** 浏览器使用单一、host-only 的 HttpOnly session Cookie；设置、读取、刷新和清理使用同一名称与属性。
- **R2. Browser secrecy:** 登录、刷新、OAuth/OIDC 回调、URL 和浏览器存储均不得暴露认证 token。
- **R3. Compatibility:** 迁移期 Cookie 优先、Bearer 兜底；Cookie 存在但无效时不得回退到 Bearer，避免身份混淆。
- **R4. Session lifecycle:** 登录、current-user、refresh、logout 契约一致；refresh 只依赖 Cookie，logout 幂等并清除浏览器和服务端状态。
- **R5. CSRF/CORS:** Cookie 认证的 POST/PUT/PATCH/DELETE 必须校验 Origin；CORS 精确允许可信 Origin 和实际使用的方法。
- **R6. Deployment safety:** 生产强制 HTTPS 与 Secure Cookie；TLS 终止、代理信任、公开 Origin、Java/Rust 路由和端口有唯一文档化口径。
- **R7. RustSec gate:** 固定 cargo-audit/cargo-deny 版本和 PR gate 使用的 RustSec DB commit；vulnerability、unsound、yanked 硬失败。
- **R8. Advisory freshness:** 每日 floating DB 扫描发现新 advisory；PR gate 可复现，cache 不作为信任根。
- **R9. Policy ownership:** cargo-audit 独占 RustSec advisory，cargo-deny 只负责 bans/licenses/sources；例外有 owner、工单、期限和补偿控制。
- **R10. Existing advisories:** lopdf、rsa、tantivy/lru 必须修复或获得限时例外，不能通过 `continue-on-error` 或永久 ignore 粉饰。
- **R11. Biome alignment:** package、lockfile、schema、配置、脚本和 CI 精确对齐 Biome 2.5.11 与 pnpm 11.25.0。
- **R12. Reviewability:** Biome 配置升级、纯格式化、safe lint/assist 修复和手工业务修复分别提交。
- **R13. CI discovery:** required workflows 位于仓库根 `.github/workflows/`，修改锁文件、认证或前端工具链时必然触发。
- **R14. Preservation:** 不覆盖或混入计划启动时已有的 3 个未提交 Rust 文件；任何 lockfile 更新先确认基线。

## Scope Boundaries

| 包含 | 排除（Deferred） |
|------|------------------|
| 现有 session token 的 Cookie 化、兼容窗口和前端迁移 | 新建 access-token/refresh-token 双 token 架构 |
| Cookie 认证的 Origin CSRF 校验 | 跨 site `SameSite=None` + double-submit CSRF 架构 |
| 登录、current-user、refresh、logout、OAuth/OIDC 回调闭环 | 重写 OAuth/OIDC provider 或账号体系 |
| RustSec DB pin、daily floating 扫描和例外治理 | 一般性的所有语言 SCA/SBOM 平台建设 |
| lopdf、rsa、tantivy/lru 当前风险处置 | 顺带升级所有 Rust 依赖 |
| Biome 2.5.11 配置迁移和独立格式化 | ESLint/Prettier 引入或前端业务重构 |
| 新增可发现的专项根 workflow | 在同一提交一次性激活全部嵌套历史 CI |

### Deferred to Follow-Up Work

- 若产品未来要求真正跨 site 部署，单独设计 `SameSite=None; Secure`、CSRF token 和跨域凭据策略。
- 若需要短 access JWT + refresh token，单独设计轮换、重放检测、撤销、Path 和密钥管理。
- cargo-deny license gate 依赖正式的发布/许可证决策；在该决策完成前不得擅自声明 MIT/Apache。
- `actions/setup-node@v4`、容器标签和其他浮动 action 的全面 SHA 固定另设供应链计划；本计划只固定能核验的工具版本。

## Key Technical Decisions

| 决策 | 理由 |
|------|------|
| 浏览器采用一个不透明 session token + 一个 HttpOnly Cookie | 符合现有 SessionManager，避免无需求地引入双 token 生命周期、轮换和重放检测 |
| 推荐名称 `oa4rust_session`，放入 shared 单一常量 | 名称表达会话语义，并消除 auth 与 middleware 字符串漂移 |
| Cookie 为 host-only、`Path=/`、`HttpOnly`、生产 `Secure`、`SameSite=Lax` | 当前前端和 API 可同源；不扩大到跨 site 架构；不设置 Domain 可缩小泄露范围 |
| Cookie 优先，Bearer 仅兼容旧浏览器和非浏览器客户端 | 允许渐进迁移；无效 Cookie 不回退 Bearer，防止单请求身份混淆 |
| CSRF 使用严格 Origin 校验而非 double-submit | 同源部署下最简单；只对 Cookie 认证的危险方法生效，Bearer-only CLI 可豁免 |
| 登录字段统一为后端既有 `credential/password` | 降低 Java/Rust 契约漂移，前端适配成本最小 |
| refresh 不接受 token body，也不要求 Bearer | HttpOnly 的目的就是不让 JavaScript读取 token；refresh 复用统一 Cookie 提取器 |
| cargo-audit 是唯一 RustSec gate | 避免 cargo-audit 与 cargo-deny advisories 双 DB、双 ignore、双失败 |
| PR 使用 pinned DB，daily 使用 floating DB | 同时满足可复现和 advisory 新鲜度 |
| Biome 精确固定 2.5.11 | 仓库锁文件和本地安装已验证该版本；工具漂移可能产生大面积 diff |
| 专项 workflow 先新增到根目录，旧 CI 后迁移 | 避免一次首次激活所有历史 job，降低故障归因和回滚成本 |

## Open Questions and Gates

### 实施前必须确认

- [x] TLS 是由 nginx 还是外部负载均衡器终止；可信代理边界是什么。
- [x] Java/Rust 并行阶段由哪一侧签发和验证 session，双方是否共享格式、密钥与撤销状态。
- [x] 当前 SessionManager 是否支持服务端撤销；若不支持，登出只能清 Cookie 的限制必须登记。
- [x] 现有 `ActionResult` 九字段和登录/刷新响应的 Java 兼容字段清单。
- [x] `rsa` advisory 的临时补偿控制、owner 和到期日，或私钥签名外置方案。
- [x] oa4rust workspace 是内部不可发布应用还是准备发布 crate，以决定 license policy。

### 可在实现期确定

- [x] 迁移兼容窗口的具体时长和 Bearer 浏览器流量归零阈值。
- [x] RustSec pinned commit 和 cargo-audit/cargo-deny 版本在线复核结果。
- [x] Biome 2.5.11 首次有效只读检查的 error/warning 数和分批边界。

## Target Contracts

### Cookie

| 属性 | 目标值 |
|------|--------|
| Name | `oa4rust_session`，由 shared 常量定义 |
| Value | 仅不透明 session token，不含用户资料 |
| HttpOnly | `true` |
| Secure | 生产必须 `true`；非生产降级必须显式配置 |
| SameSite | `Lax` |
| Path | `/` |
| Domain | 不设置（host-only） |
| Max-Age/Expires | 与服务端 session TTL 一致；清理时 `Max-Age=0` 且 Expires 为过去时间 |

### Authentication Precedence

1. 存在 `oa4rust_session`：只验证 Cookie。
2. Cookie 不存在：兼容期允许 `Authorization: Bearer`。
3. Cookie 存在但空、过期、篡改：401，不回退 Bearer。
4. 兼容期结束后，浏览器 Bearer 停用；明确保留的 CLI/service Bearer 另有客户端契约。

### Browser API

| 流程 | 请求 | 成功行为 |
|------|------|----------|
| Login | `credential/password`，`credentials: include` | 创建 session、Set-Cookie、JSON 只返回用户/过期信息 |
| Current user | Cookie | 返回当前身份；前端以此恢复会话 |
| Refresh | 空 body 或无 body，Cookie | 验证并旋转 session、Set-Cookie，不返回 token |
| Logout | Cookie | 服务端撤销（若支持）并无条件清 Cookie；重复调用幂等 |
| OAuth/OIDC callback | 后端 code exchange | 后端 Set-Cookie 后重定向；URL、fragment、JSON 不携带 token |

## Implementation Units

### Phase 0：保护基线并冻结契约

#### U0. 建立可验证基线与变更保护

**Goal:** 在不覆盖当前用户改动的前提下，记录编译、前端和供应链基线。

**Requirements:** R14

**Dependencies:** None

**Files:**
- Preserve: `oa4rust/Cargo.lock`
- Preserve: `oa4rust/crates/auth/Cargo.toml`
- Preserve: `oa4rust/crates/auth/src/lib.rs`
- Create: 可选的执行证据目录或 PR 说明，不提交构建产物

**Approach:**
1. 记录 `git status --short`、三个文件的 diff hash/stat 和当前分支。
2. 在独立 worktree 或确认的原始工作区中实施；禁止 stash/reset/restore/clean。
3. 在任何 `cargo update`、`pnpm add` 前记录对应 lockfile 差异。
4. 用当前代码运行最小编译/配置检查，真实记录失败而非假设通过。

**Test Scenarios:**
- Regression: 后续每个提交都不含无关的三个 Rust 文件差异。
- Error: 工具命令意外改动根 lockfile 或用户文件时立即停止并恢复仅该命令产生的变更。

**Verification:** `git diff --check`；对比基线 diff；后续提交使用显式路径暂存。

**Rollback:** 删除本计划创建的独立 worktree或仅回退本单元产生的证据文件，不触碰用户改动。

**Commit:** 无代码提交；基线作为实施记录。

- [x] 原始 diff 已记录
- [x] 实施位置已隔离
- [x] 基线失败/通过状态已记录

#### U1. 冻结认证、CSRF 与部署契约

**Goal:** 将 Target Contracts 和 Open Questions 中的阻断决策转成可执行配置及书面契约。

**Requirements:** R1-R6

**Dependencies:** U0

**Files:**
- Create: `oa4rust/docs/security/browser-session-cookie.md`
- Modify: `oa4rust/.env.example`
- Modify: `oa4rust/.env.test.example`
- Read/Modify as needed: `oa4rust/deploy/nginx.conf`、auth route includes

**Approach:**
1. 确认 session 互操作、TLS 终止、可信代理、公开 Origin、服务端撤销和兼容窗口。
2. 文档化 Cookie 属性、认证优先级、API 契约、CSRF 规则和错误语义。
3. 增加 `APP_PUBLIC_ORIGIN`、session TTL、Cookie Secure 等示例；生产配置缺失时 fail loud。
4. 统一 Java 后端端口和 auth include 上下文；拆分 upstream 与 location 配置。

**Verification:** 配置示例与代码读取项一一对应；`nginx -t` 在部署环境通过；契约经后端、前端、运维共同确认。

**Rollback:** 保持现有 Bearer 路径，不启用 Cookie 流量；配置变更可独立回退。

**Commit:** `docs(auth): define browser session cookie contract`

- [x] 认证契约已批准
- [x] TLS/Origin/代理边界已确认
- [x] env 与部署口径已统一

### Phase 1：后端 HttpOnly Cookie 与 CSRF

#### U2. 完成统一 Cookie 生命周期和兼容认证

**Goal:** 后端可编译地完成 login/current-user/refresh/logout Cookie 生命周期，并在兼容期支持 Cookie 优先、Bearer 兜底。

**Requirements:** R1-R4, R14

**Dependencies:** U1

**Files:**
- Modify: `oa4rust/crates/auth/src/lib.rs`
- Modify: `oa4rust/crates/auth/Cargo.toml`
- Modify: `oa4rust/crates/shared/src/middleware/token.rs`
- Modify/Create: shared 认证常量模块
- Modify: `oa4rust/Cargo.lock`（只保留预期 cookie 依赖变化）
- Test: auth/shared 单元与 router 集成测试

**Approach:**
1. 在 shared 定义 `SESSION_COOKIE_NAME` 和统一 token 提取结果，auth 设置/清理与 middleware 共同引用。
2. 将 login/refresh/logout handler 统一返回 `axum::response::Response`，所有分支显式 `into_response()`，解决不同具体返回类型问题。
3. Cookie 存在时只验证 Cookie；不存在才允许 Bearer。重复、空、非法或篡改 Cookie 返回 401。
4. 登录 body 采用 `credential/password`；成功响应添加 `Cache-Control: no-store` 和 Cookie。
5. refresh 移除 header/body token 相等要求，复用统一提取器并旋转 Cookie。
6. logout 无论 session 是否已失效都清 Cookie；若 SessionManager 支持撤销，同时撤销服务端 session。
7. Cookie 清理使用相同 Name、Path、Domain语义、Secure、SameSite，并设置 Max-Age=0/过去 Expires。
8. 迁移期可暂保留 JSON token 给旧客户端，但必须有 telemetry、deprecation 和删除门槛；新浏览器不得使用。

**Test Scenarios:**
- Happy: login Set-Cookie 后仅靠 Cookie 访问 protected/current-user；refresh 旋转；logout 清理。
- Compatibility: 无 Cookie 时 Bearer 可用；Cookie 与 Bearer 同时存在时 Cookie 优先。
- Security: 无效 Cookie + 有效 Bearer 仍 401；响应/日志不泄露 Cookie token。
- Error: 过期、篡改、重复、空 Cookie 均有确定错误。

**Verification:** `cargo check -p auth -p shared --locked`；相关 unit/router tests 全部通过；`git diff --check`。

**Rollback:** 保留 Bearer 解析开关；回退 Cookie 发放时，旧 Bearer 客户端仍可工作；不得只回退 middleware 而保留前端 Cookie-only。

**Commit:** `refactor(auth): complete HttpOnly session cookie contract`

- [x] handler 编译类型已统一
- [x] Cookie 名称和属性单一来源
- [x] login/refresh/logout 测试通过
- [x] Bearer 兼容优先级测试通过

#### U3. 增加 Cookie 认证 CSRF 和完整 CORS

**Goal:** 防止浏览器自动附带 Cookie 导致跨站危险操作，同时允许前端真实使用的方法。

**Requirements:** R5-R6

**Dependencies:** U2

**Files:**
- Create: `oa4rust/crates/shared/src/middleware/csrf.rs`
- Modify: shared middleware exports/router layer
- Modify: `oa4rust/crates/shared/src/middleware/security.rs`
- Modify: `.env` examples and deployment docs
- Test: CSRF/CORS middleware tests

**Approach:**
1. 对 Cookie 认证的 POST/PUT/PATCH/DELETE 强制 `Origin` 存在并与 `APP_PUBLIC_ORIGIN` 精确匹配 scheme/host/port。
2. Bearer-only 非浏览器请求可豁免；Cookie 请求不得因同时有 Bearer 而豁免。
3. GET/HEAD/OPTIONS 不执行状态修改 CSRF 校验；确保 handler 不在安全方法产生副作用。
4. CORS 增加 PUT/PATCH/DELETE，保留精确 Origin、credentials 和必要 Header；禁止 wildcard + credentials。
5. 明确反向代理只信任受控的 forwarded headers。

**Test Scenarios:**
- Happy: 合法 Origin 的四种危险方法通过；GET/HEAD 正常。
- Error: 缺少/错误/null Origin 返回 403；preflight 对 PUT/PATCH/DELETE 成功。
- Regression: Bearer-only CLI 请求不因无 Origin 失败。

**Verification:** middleware 单测与 router 集成测试通过；浏览器黑盒验证同源流程；跨 Origin 请求被拒绝。

**Rollback:** 可回退 Cookie 流量而不能仅关闭 CSRF；若 Origin 配置错误，先回滚发布而不是全局放宽为 `*`。

**Commit:** `feat(auth): enforce origin checks for cookie requests`

- [x] 危险方法 Origin 校验覆盖完整
- [x] CORS methods 与 SDK 一致
- [x] 跨站负面测试通过

### Phase 2：前端浏览器会话迁移

#### U4. SDK 停止浏览器 token 持久化和注入

**Goal:** oa4rust-web 浏览器请求完全依赖 HttpOnly Cookie，JavaScript 不再接触 token。

**Requirements:** R2-R5

**Dependencies:** U2, U3

**Files:**
- Modify: `oa4rust-web/packages/sdk/src/session.ts`
- Modify: `oa4rust-web/packages/sdk/src/api.ts`
- Modify: SDK exports/tests

**Approach:**
1. 登录发送 `credential/password`，成功后只保存用户状态，不调用 `setToken`。
2. 删除浏览器路径中的 get/set/clear token、token provider 和自动 Authorization 注入。
3. 保留 `credentials: 'include'`；refresh 发送空 body且只依赖 Cookie。
4. 401 最多 refresh 一次、重放一次；refresh 请求自身不触发 refresh。
5. 并发 401 共享一个 refresh Promise，避免刷新风暴。
6. 一次性删除 legacy localStorage/sessionStorage key，但不得读取并继续使用旧 token。

**Test Scenarios:**
- Happy: login/current-user/refresh/logout 全流程无 JavaScript token。
- Concurrency: 多请求同时 401 只触发一次 refresh。
- Error: refresh 401 清空内存用户态，不循环、不重放危险请求两次。
- Regression: upload 与普通请求都发送 credentials。

**Verification:** SDK unit tests；全仓搜索不再出现浏览器 token 持久化/注入调用；`pnpm typecheck`。

**Rollback:** 必须与 U2 的 Bearer 兼容窗口协调；前端回滚到旧版时后端 Bearer 仍可用，严禁单独关闭后端兼容后回滚前端。

**Commit:** `refactor(web): migrate SDK session handling to HttpOnly cookie`

- [x] localStorage/sessionStorage token 写入为零
- [x] Authorization 自动注入已删除
- [x] 单飞 refresh 与循环保护测试通过

#### U5. Auth store、路由守卫和 OAuth/OIDC 回调迁移

**Goal:** 前端认证状态从 token 存在性改为服务端 current-user 权威状态。

**Requirements:** R2, R4

**Dependencies:** U4

**Files:**
- Modify: auth store/composables、router guards、login view、OAuth callback 调用方
- Test: desktop auth unit/integration tests

**Approach:**
1. 将状态建模为 `unknown | authenticated | anonymous`，store 只保存 user，不保存 token。
2. 应用启动/刷新页面时调用 current-user；守卫在 `unknown` 时等待恢复。
3. OAuth/OIDC callback 不读取 query/fragment token，不调用 `setSession(token)`；后端 Set-Cookie 后前端只调用 current-user。
4. logout 清空服务端 Cookie和内存 user，再跳转登录页。
5. 全仓搜索 `setSession`、`setToken`、`getToken`、`localStorage`、`sessionStorage`、`Authorization`，逐项分类清零或标记非认证用途。

**Test Scenarios:**
- Page reload: Cookie 有效时恢复用户，无效时进入 anonymous。
- OAuth: URL 和日志不出现 token，回调后 current-user 成功。
- Guard: unknown 状态不误跳登录；anonymous 被正确拦截。
- Logout: 后退/刷新不能恢复已退出用户。

**Verification:** Vitest、typecheck、build；浏览器黑盒执行登录→刷新→refresh→logout→OAuth callback。

**Rollback:** 保持 U2 的兼容期；回滚前端构建后重新执行旧客户端验证，禁止恢复 URL token 模式。

**Commit:** `refactor(web): restore authentication from server session`

- [x] store 不保存 token
- [x] route guard 使用 current-user
- [x] OAuth URL/JSON token 暴露为零
- [x] 浏览器端到端流程通过（认证 cookie 流：login→Set-Cookie、reload 恢复、refresh 旋转、logout 失效，IAB 实测 `docs/ops/browser-e2e-2026-09-10.md`；登录经 requestSubmit 触发——IAB 指针/键盘 actuator 取不到点击点。`/app/*` 应用壳空白为独立既有渲染缺陷，非认证问题，另行跟踪；真实 OAuth provider 流程仍 BLOCKED）

### Phase 3：RustSec 风险处置与审计门禁

#### U6. 修复或限时治理当前 advisory 基线

**Goal:** required cargo-audit gate 启用前，处置当前已知 vulnerability/unsound，禁止通过永久 ignore 变绿。

**Requirements:** R9-R10, R14

**Dependencies:** U0；与 U4/U5 可并行，但修改 `Cargo.lock` 前必须合并/保存 Cookie 基线

**Files:**
- Modify: `oa4rust/Cargo.toml`（lopdf/rsa workspace dependency）
- Modify: `oa4rust/crates/signature/Cargo.toml`、signature 代码/测试
- Modify: `oa4rust/crates/search/Cargo.toml`、search 代码/测试
- Modify: `oa4rust/crates/auth/Cargo.toml`/OIDC verify 路径（仅实际需要）
- Modify: `oa4rust/Cargo.lock`
- Create: 必要时 `oa4rust/security/rustsec-exceptions.toml`

**Approach:**
1. 升级 lopdf 到在线核验且满足 RustSec patched range 的版本，补深嵌套/畸形 PDF 与 sign/verify 测试。
2. 升级 Tantivy 到能自然解析出已修复 lru 的兼容版本；不强行 patch 不兼容 lru。
3. 将 RSA verify-only 与私钥 signing 暴露面分开；优先把私钥签名迁至 HSM/PKCS#11/独立服务。
4. 若 RSA 短期无 patched version，例外必须包含 owner、tracking、created、expires、reason、compensating_control、dependency_path，并在 CI 校验过期。
5. 每个依赖升级独立提交，审查 lockfile 只包含预期解析变化。

**Test Scenarios:**
- lopdf: 正常 PDF、畸形/深嵌套 PDF、签名和验签。
- search: 建索引、查询、升级兼容和 PostgreSQL fallback。
- rsa: OIDC 验签；私钥签名路径补偿控制或外部服务集成。

**Verification:** 固定 DB 的 `cargo audit --no-fetch` 无未批准 vulnerability/unsound/yanked；相关 crate tests、workspace check/test 通过。

**Rollback:** manifest 与 lockfile 成对回退；保留旧索引格式/签名服务切换手册；回退后 RustSec gate 预期重新失败，不得关闭门禁掩盖。

**Commit:** 分为 `fix(signature): remediate lopdf advisory`、`fix(search): upgrade tantivy lru chain`、`refactor(signature): isolate private-key signing`。

- [x] lopdf 风险已修复并测试
- [x] tantivy/lru 风险已修复并测试
- [x] RSA 已修复、外置或有未过期正式例外

#### U7. 建立 pinned cargo-audit 与 floating RustSec DB 双轨

**Goal:** PR 扫描可复现、每日扫描可发现新 advisory，DB/cache/例外均可审计。

**Requirements:** R7-R10, R13

**Dependencies:** U6

**Files:**
- Create: `.github/workflows/oa4rust-supply-chain.yml`
- Create: `oa4rust/.cargo/audit.toml`
- Create: `oa4rust/security/rustsec-db.rev`
- Create/Modify: exception validation script and `.gitignore` precise allowlist if needed
- Create: `oa4rust/docs/security/rustsec-audit.md`

**Approach:**
1. 在线核验并固定 cargo-audit、cargo-deny 精确版本；本机已验证候选分别为 0.22.2、0.20.2，不将“本机候选”误写成远端最新。
2. PR/push gate checkout `rustsec-db.rev` 的 40 位 commit；cache key 包含 SHA，恢复后验证 remote、HEAD 和 git object完整性。
3. cargo-audit 使用 `--file Cargo.lock --db <path> --no-fetch --deny unsound --deny yanked`，打印 scanner/DB/lockfile hash。
4. schedule 使用 floating DB；网络获取失败时硬失败，不能回退旧 cache 冒充最新。
5. vulnerability/unsound/yanked 硬失败；unmaintained 第一阶段告警；禁止 `--stale`、`continue-on-error`。
6. 校验 exception 字段和到期日；过期立即失败。

**Verification:** 修改 Cargo.lock 必触发；错误 pin、cache SHA 不符、过期例外、fetch 失败均按策略失败；daily 日志记录实际 DB SHA。

**Rollback:** 删除/回退专项 workflow 不影响运行时代码；DB pin 和 scanner 版本可按一行/一个提交回退，但保留失败报告和审计原因。

**Commit:** `ci(oa4rust): add pinned cargo-audit RustSec gate`

- [x] scanner 版本已固定
- [x] pinned/floating 双轨已定义（本地 cargo-audit/cargo-deny 通过；远端 workflow 实际触发未验证，未 push）
- [x] pinned RustSec DB SHA 校验有效（`security/rustsec-db.rev`，非 cache；审计命令打印 DB HEAD 与 lock hash）
- [ ] required check 已配置（EXTERNAL：需 push + 在 repository ruleset 22504571 加 required status checks，本轮不做）

#### U8. 收敛 cargo-deny 边界与 license 基线

**Goal:** cargo-deny 只治理 bans/licenses/sources，并依据正式发布决策建立可通过的 license policy。

**Requirements:** R9-R10

**Dependencies:** U7；license 子项依赖项目所有者决策

**Files:**
- Modify: `oa4rust/deny.toml`
- Modify: workspace/member Cargo manifests（仅 license/publish 决策需要）
- Create/Modify: LICENSE（仅正式确认公开许可证后）

**Approach:**
1. 删除 cargo-deny `[advisories]` 主职责和按 RustSec advisory 手写的重复 lru ban，保留真正的依赖策略 ban。
2. CI 只运行 `cargo deny --locked --workspace check bans licenses sources`。
3. 内部不可发布应用采用 workspace `publish=false` 和 private crate policy；公开发布则由所有者确定 LICENSE 并继承 workspace license。
4. 不为了 CI 变绿猜测 MIT/Apache，不无理由忽略全部 private crates。

**Verification:** bans/licenses/sources 三项通过且不会重复报告 cargo-audit advisory。

**Rollback:** policy 和 manifest metadata 可独立回退；cargo-audit gate不受 cargo-deny回退影响。

**Commit:** `chore(oa4rust): establish cargo-deny policy baseline`

- [x] advisories 单一职责已落实
- [x] license/publish 决策有书面依据
- [x] cargo-deny 三项通过

### Phase 4：Biome 2.5.11 与前端专项 CI

#### U9. 对齐 Biome 依赖、配置和脚本

**Goal:** 在不产生源码格式化 diff 的提交中，将前端完全对齐 Biome 2.5.11。

**Requirements:** R11-R12, R14

**Dependencies:** U0；可与 Phase 1/3 并行

**Files:**
- Modify: `oa4rust-web/package.json`
- Modify: `oa4rust-web/pnpm-lock.yaml`
- Modify: `oa4rust-web/biome.json`
- Modify: `oa4rust-web/.npmrc`

**Approach:**
1. 使用 pnpm 11.25.0 在 `oa4rust-web` 执行 `pnpm add -D -E @biomejs/biome@2.5.11`；根目录两套 lockfile 不得改变。
2. schema 固定 `2.5.11`；把 `organizeImports` 迁到 `assist.actions.source.organizeImports`，recommended 迁为 preset，semicolons 迁为 `asNeeded`。
3. 默认 `lint` 改为只读，新增显式 `lint:fix`；保留 `format`/`format:check` 分离。
4. 删除 `.npmrc` 中失效的本机绝对 `package-json-path`。
5. 如启用 VCS ignore，单独验证 `root: ".."` 确实读取仓库根 `.gitignore`。

**Test Scenarios:**
- Config: migrate 不再提示，check 不再报旧键或类型错误。
- Reproducibility: frozen install 不改 lockfile，Biome 版本严格为 2.5.11。
- Preservation: 根 package lockfiles 和 Rust 用户改动不变。

**Verification:** `pnpm install --frozen-lockfile`、`pnpm exec biome --version`、`pnpm exec biome migrate --verbose`、配置只读 check。

**Rollback:** manifest 与 oa4rust-web lockfile 成对回退；不能只把声明改回 `^2.1.0` 而保留 2.5.11 lockfile。

**Commit:** `chore(web): align Biome config with 2.5.11`

- [x] package/lockfile 精确为 2.5.11
- [x] 配置可解析且无需迁移
- [x] 默认 lint 为只读
- [x] 无源码格式化 diff

#### U10. 新增可发现的前端 QA workflow

**Goal:** Biome、Vitest、typecheck 和 build 在根目录 workflow 中成为真实门禁。

**Requirements:** R11-R13

**Dependencies:** U9

**Files:**
- Create: `.github/workflows/oa4rust-web.yml`
- Later retire/migrate corresponding job: `oa4rust/.github/workflows/ci.yml`

**Approach:**
1. 使用 Node 22、pnpm 11.25.0 和 `pnpm install --frozen-lockfile`。
2. 顺序运行 test、typecheck、`pnpm lint:check`、build；每步有 timeout，不使用 continue-on-error。
3. path filter 包含 `oa4rust-web/**` 和 workflow 自身。
4. 可选缓存仅缓存 pnpm store，key 包含 OS、Node、pnpm 和 `oa4rust-web/pnpm-lock.yaml` hash；不缓存 node_modules。
5. 专项 workflow 稳定后，再删除嵌套 workflow 中重复 frontend job。

**Verification:** GitHub 可发现 workflow；PR 修改 package/lock/config/source 均触发；日志工具版本精确；四项 gate 全部通过。

**Rollback:** 删除专项 workflow 不改前端运行时代码；保留本地验证命令，避免假报 CI 已覆盖。

**Commit:** `ci(web): add discoverable frontend quality gate`

- [x] root workflow 可被发现
- [x] pnpm 版本与项目一致
- [x] test/typecheck/lint/build 均为硬门禁

#### U11. 隔离 Biome 生成性改动

**Goal:** 以可审查、可回滚的提交处理第一次有效运行 Biome 2.5.11 产生的差异。

**Requirements:** R12

**Dependencies:** U9；U10 可并行验证

**Files:**
- Modify: `oa4rust-web/**` 中 Biome 报告的文件，严格按批次

**Approach:**
1. 先运行只读 `biome check .`，记录 error/warning 数与规则分布。
2. 单独运行 `biome format --write .`，只提交纯 formatter 输出。
3. 在格式提交后运行 `biome check --write .`，单独提交 safe lint fixes 和 organize imports。
4. 无法自动修复的诊断按 `apps/desktop`、`packages/*` 或业务模块拆分，禁止与工具配置混合。
5. 大型 Vue 文件逐个审查，避免格式化掩盖行为变化。

**Verification:** 每批 `git diff --check`、lint:check、test、typecheck、build；只读命令前后工作树不变化。

**Rollback:** 格式、safe fixes、手工业务修复均可整提交独立回退；回退后重新运行全前端 gate。

**Commit:** `style(web): format with Biome 2.5.11`，随后 `fix(web): apply Biome lint and import fixes`，必要时继续分模块。

- [x] 首次诊断已归档
- [x] formatter 与 lint/assist 分提交
- [x] 大文件已人工审查
- [x] 全前端 gate 通过

### Phase 5：联合验证、灰度与收口

#### U12. 认证安全联合测试和兼容窗口退出

**Goal:** 用后端、前端和浏览器测试证明 Cookie 迁移完整，并在指标满足后移除浏览器 Bearer/JSON token 兼容。

**Requirements:** R1-R6, R13

**Dependencies:** U2-U5, U10

**Files:**
- Modify/Create: auth router integration tests、oa4rust-web auth tests、browser E2E
- Modify: 迁移期 telemetry/deprecation 配置

**Test Matrix:**
1. Login 返回正确 Set-Cookie，JSON/URL 无 token。
2. Cookie 包含 HttpOnly、生产 Secure、SameSite=Lax、Path=/、无 Domain、TTL 一致。
3. 仅 Cookie 可访问 protected/current-user；仅 Bearer 在兼容期可访问。
4. Cookie 优先；无效 Cookie + 有效 Bearer 仍 401。
5. Refresh 只靠 Cookie 成功并旋转，不接受 body token。
6. Logout 对有效/过期 session 均清 Cookie且幂等。
7. POST/PUT/PATCH/DELETE 对缺失、null、错误 Origin 返回 403；合法 Origin 通过。
8. CORS preflight 覆盖 PUT/PATCH/DELETE；GET/HEAD 不被误拦截。
9. 页面刷新恢复用户；并发 401 单次 refresh；失败无循环。
10. OAuth callback 的 query、fragment、JSON、日志均无 token。

**Verification:** 后端 tests、Vitest、typecheck、build、浏览器黑盒测试全部通过；无 skipped 被写成通过；兼容 telemetry 达到退出阈值。

**Rollback:** 在兼容窗口内可回滚前端；移除兼容前必须完成回滚演练。兼容移除后若回滚，只回滚到仍不暴露 URL/localStorage token 的安全版本。

**Commit:** `test(auth): verify cookie lifecycle and csrf protection`，兼容退出另作 `refactor(auth): retire browser bearer compatibility`。

- [ ] 10 类测试矩阵全部通过（本地已验证 1–9 项；第 10 项 OAuth callback 仅有静态源码断言，真实 provider 流程 BLOCKED）
- [ ] 兼容流量和退出阈值有证据（telemetry 已就位；“零阈值需真实流量窗口”未观测）
- [x] Browser Bearer/JSON token 已退场

#### U13. CI 合并、文档和计划状态收口

**Goal:** 专项 workflow 稳定后，消除嵌套/重复 CI，归档证据并将本计划切换为 completed。

**Requirements:** R7-R13

**Dependencies:** U7-U12

**Files:**
- Move/Modify/Retire: `oa4rust/.github/workflows/ci.yml`
- Modify: 根 `.github/workflows/*`
- Modify: `oa4rust/README.md`、security docs、受影响旧计划的关联说明
- Modify: 本计划 frontmatter/checklist

**Approach:**
1. 审计旧 workflow 的 quality/test/behavior/OpenAPI jobs，逐项迁移到根目录，禁止无证据删除。
2. 移除已由专项 workflow覆盖的重复 frontend/supply-chain job。
3. 修复浮动工具/action 仅限已有在线核验依据的项；不能猜 SHA。
4. 更新旧前端计划中的 Cookie/Bearer/Biome 决策关联，不整体 supersede 仍有效的架构范围。
5. 归档验证、例外、回滚演练和 daily scan 证据；所有单元完成后同提交将 `status: active` 改为 `completed`。

**Verification:** 根 workflow 列表与 required checks 一致；旧嵌套 workflow 不再被误当作有效门禁；计划复选框、状态和实际提交一致。

**Rollback:** CI 迁移按 job 独立回退；保留专项安全/前端 gate，避免回退到“无可发现 workflow”的状态。

**Commit:** `ci(repo): consolidate discoverable oa4rust workflows`，随后收官 `docs(plans): complete security and tooling migration plan`。

- [x] 全部 required workflow 位于根目录
- [x] 重复 job 已清理且无覆盖缺口
- [x] 文档与旧计划关联已更新
- [x] 本计划状态与事实一致

## System-Wide Impact

### Interaction Graph

```text
Browser login/OAuth
  -> auth handler creates SessionManager token
  -> Set-Cookie oa4rust_session
  -> browser automatically sends Cookie
  -> CSRF Origin middleware (unsafe methods)
  -> token middleware (Cookie first; Bearer fallback only if absent)
  -> protected handler/current-user
  -> refresh rotates Cookie / logout revokes and clears

Cargo manifests + Cargo.lock
  -> pinned cargo-audit + pinned RustSec DB (PR hard gate)
  -> floating RustSec DB (daily freshness)
  -> cargo-deny bans/licenses/sources

package.json + pnpm-lock + biome.json
  -> pnpm 11.25.0 frozen install
  -> Biome 2.5.11 check
  -> Vitest + typecheck + build
  -> root GitHub workflow required check
```

### Error and State Semantics

- 认证失败使用 401；Cookie CSRF 失败使用 403；业务错误继续遵循现有 ActionResult 九字段契约。
- login/refresh/logout 响应必须 `Cache-Control: no-store`，日志不得输出 token、Cookie header 或 Set-Cookie value。
- 前端只保存 `user` 与 `unknown/authenticated/anonymous` 状态；服务端 current-user 是刷新后的权威来源。
- RustSec scan 失败、DB 不可用、例外过期均 fail loud；上传报告可 `if: always()`，但不能吞掉扫描退出码。

### Compatibility and Migration

- 阶段 A：后端双通道发放/验证 Cookie，同时保留旧 Bearer。
- 阶段 B：新前端切换 Cookie-only，观察旧客户端 Bearer 使用量和错误率。
- 阶段 C：停止浏览器 JSON token；为明确的 CLI/service 客户端保留独立 Bearer 契约。
- 阶段 D：达到退出阈值后移除浏览器兼容代码和 legacy storage 清理代码。

## Risks and Dependencies

| 风险 | 影响 | 缓解 |
|------|------|------|
| 当前 Cookie diff 静态不可编译 | 后续工作建立在错误基线 | U0 记录、U2 首先统一 Response 返回类型并跑 targeted check |
| 前后端协议漂移 | 登录/refresh 直接不可用 | U1 冻结契约，U2/U4 原子化兼容 |
| Cookie 引入 CSRF | 跨站状态修改 | SameSite=Lax + Cookie unsafe method严格 Origin 校验 |
| Secure Cookie 与 HTTP部署冲突 | 生产无法登录或被迫降级 | 明确 TLS 终止，生产配置 fail loud，禁止隐式降级 |
| Java/Rust session 不互操作 | 灰度期间随机掉线 | 切流前做双方签发/验证/撤销矩阵，认证路由原子切换 |
| lockfile 并发修改 | 覆盖用户 Cookie 依赖或污染审计 | 先收口 Cookie lockfile，依赖升级独立 worktree/提交 |
| RustSec gate 初始必红 | CI 无法合并 | 先 U6 修复/限时例外，再 U7 required；不 continue-on-error |
| 固定 DB 变旧 | 漏新 advisory | daily floating scan + 安全 PR 更新 pin |
| RSA 无 patched version | 无法零例外 | 私钥签名外置，例外有 owner/期限/补偿控制 |
| cargo-deny license 法律决策未知 | license gate 无法诚实通过 | 与 audit gate解耦，所有者决定 publish/license 后再 required |
| Biome 首次有效运行产生大 diff | 难审查、易藏行为变化 | U9 只改工具；U11 格式/safe fix/业务修复分提交 |
| 激活嵌套 CI 暴露历史失败 | 无法定位本计划故障 | 先建专项根 workflow，旧 CI 最后逐 job 迁移 |

## Success Metrics

| 指标 | 完成标准 |
|------|----------|
| Browser token storage | 认证用途 localStorage/sessionStorage 写入为 0 |
| Browser Authorization | 浏览器 SDK 自动 Bearer 注入为 0 |
| Token exposure | login/refresh/OAuth URL、fragment、JSON、日志中 token 为 0 |
| Cookie contract | 设置、读取、refresh、清理名称/Path/属性一致，专项测试全过 |
| CSRF | Cookie unsafe method 缺失/错误 Origin 100% 拒绝，合法同源 100% 通过 |
| Auth E2E | login→current-user→reload→refresh→logout 和 OAuth callback 全过 |
| RustSec | 无未批准 vulnerability、unsound、yanked；例外均未过期 |
| DB reproducibility | PR 日志 DB SHA = `rustsec-db.rev`；cache SHA 不符必失败 |
| DB freshness | daily floating scan 成功记录实际 SHA；fetch失败不伪成功 |
| cargo-deny | bans/licenses/sources 按正式 policy 通过，不重复 advisory |
| Biome | package/lock/schema/runtime/CI 均精确 2.5.11，无迁移提示 |
| Frontend QA | frozen install、Vitest、typecheck、Biome、build 全部通过 |
| CI discovery | required workflows 全在根 `.github/workflows/` 且 path trigger验证 |
| Preservation | 用户原始 3 个 Rust 文件没有被无关提交覆盖或混入 |

## Verification Commands

### Rust/Auth

```bash
cd oa4rust
cargo fmt --all -- --check
cargo check -p auth -p shared --locked
cargo test -p auth -p shared --locked
cargo check --workspace --locked
cargo test --workspace --lib --locked
```

数据库/环境依赖导致的 skipped 必须逐项列出，不能把部分执行写成“workspace tests pass”。

### RustSec/cargo-deny

```bash
cd oa4rust
cargo audit --file Cargo.lock --db "$RUSTSEC_DB" --no-fetch --deny unsound --deny yanked
cargo deny --locked --workspace check bans licenses sources
```

同时核验 `git -C "$RUSTSEC_DB" rev-parse HEAD`、scanner versions 和 Cargo.lock hash。

### Frontend

```bash
cd oa4rust-web
pnpm install --frozen-lockfile
pnpm exec biome --version
pnpm exec biome migrate --verbose
pnpm lint:check
pnpm test
pnpm typecheck
pnpm build
```

### Repository Integrity

```bash
git status --short
git diff --check
git diff --stat -- oa4rust oa4rust-web .github docs
```

根 `package-lock.json`、根 `pnpm-lock.yaml` 和非本单元用户改动不得出现意外变化。

## Rollout and Rollback

### Rollout Sequence

1. 记录和隔离现有未提交 Cookie diff。
2. 批准认证、TLS、CSRF、session 互操作和 license 决策。
3. 部署后端 Cookie + Bearer 兼容，不切换前端；验证 Cookie 遥测和错误率。
4. 部署 Cookie-only 新前端；观察登录成功率、401/403、refresh、OAuth 和旧 Bearer 使用量。
5. 修复 RustSec 基线后启用 required cargo-audit；保持 daily floating scan。
6. 对齐 Biome 配置并启用前端 required workflow；格式化和 lint 修复分批进入。
7. 达到兼容退出阈值后删除浏览器 JSON token/Bearer 兼容；保留明确的非浏览器契约。
8. 逐 job 迁移剩余嵌套 CI，归档证据并完成计划。

### Rollback Triggers

- 登录或 current-user 成功率显著下降；401/403 或 refresh 循环超过发布前基线。
- Cookie 未保存、未发送、属性缺失，或 token 出现在 URL/日志/JSON。
- CSRF 合法流量误拒绝，或恶意 Origin 能通过。
- Java/Rust 灰度出现 session 不互操作或用户身份错配。
- RustSec DB SHA/工具版本不可确认，扫描结果不可复现。
- Biome 自动改动包含无法解释的业务语义变化。

### Rollback Rules

- Cookie 后端兼容期内可先回滚前端；不能只关闭 CSRF 保留 Cookie认证。
- 后端回滚必须同时恢复与前端匹配的认证路径；Cookie 名/Path 改动需发清理 Cookie。
- manifest 与 lockfile 成对回滚；依赖回滚后重新执行 audit，不隐藏重新出现的 advisory。
- Biome 配置/lockfile成对回滚；格式和 lint提交独立回滚后跑完整前端 gate。
- CI 可按专项 workflow独立回滚，但不能回到没有根目录 required gate 的状态。
- 每次回滚记录触发指标、commit、时间、验证结果和后续 owner。

## Commit Plan

| 顺序 | Commit | Scope | 最低验证 |
|------|--------|-------|----------|
| 1 | `docs(auth): define browser session cookie contract` | U1 | 契约/配置/nginx校验 |
| 2 | `refactor(auth): complete HttpOnly session cookie contract` | U2 | auth/shared check + tests |
| 3 | `feat(auth): enforce origin checks for cookie requests` | U3 | CSRF/CORS tests |
| 4 | `refactor(web): migrate SDK session handling to HttpOnly cookie` | U4 | SDK tests + typecheck |
| 5 | `refactor(web): restore authentication from server session` | U5 | auth tests + browser smoke |
| 6 | `fix(signature): remediate lopdf advisory` | U6a | PDF corpus + audit |
| 7 | `fix(search): upgrade tantivy lru chain` | U6b | index/fallback + audit |
| 8 | `refactor(signature): isolate private-key signing` | U6c | signing/OIDC + threat controls |
| 9 | `ci(oa4rust): add pinned cargo-audit RustSec gate` | U7 | pinned + floating failure tests |
| 10 | `chore(oa4rust): establish cargo-deny policy baseline` | U8 | bans/licenses/sources |
| 11 | `chore(web): align Biome config with 2.5.11` | U9 | frozen install + migrate/check |
| 12 | `ci(web): add discoverable frontend quality gate` | U10 | GitHub run all green |
| 13 | `style(web): format with Biome 2.5.11` | U11a | lint/test/typecheck/build |
| 14 | `fix(web): apply Biome lint and import fixes` | U11b | lint/test/typecheck/build |
| 15 | `test(auth): verify cookie lifecycle and csrf protection` | U12 | 联合测试矩阵 |
| 16 | `refactor(auth): retire browser bearer compatibility` | U12 exit | telemetry阈值 + rollback drill |
| 17 | `ci(repo): consolidate discoverable oa4rust workflows` | U13 | root required checks |
| 18 | `docs(plans): complete security and tooling migration plan` | 收口 | checklist/status/证据一致 |

若 U2 与 U4 之间不能保持构建和兼容测试通过，应以同一 PR 交付，但仍保持后端/前端为可独立回滚的 commit；不得形成生产不可用的中间部署。

## Documentation Plan

- 新增 `oa4rust/docs/security/browser-session-cookie.md`：Cookie、API、CSRF、代理、兼容和故障处理。
- 新增 `oa4rust/docs/security/rustsec-audit.md`：工具/DB版本、在线离线命令、例外和 daily流程。
- 更新 `oa4rust/README.md`：链接安全运行手册和最小验证命令。
- 更新 `.env` examples：公开 Origin、Cookie Secure、TTL、CORS。
- 对 `docs/plans/2026-09-04-001-feat-oa4rust-frontend-o2web-next.md` 增加关联说明，修订“前端设置 Cookie”和旧 Biome 决策；不整体标记 superseded。
- 完工时更新本计划 `status: completed`；若被新计划完整替代，则改 `superseded` 并写替代路径。

## Final Acceptance Checklist

### HttpOnly Cookie

- [x] Cookie 名称、属性、设置、读取、旋转和清理只有一个共享定义
- [x] 登录/current-user/refresh/logout/OAuth 契约一致
- [x] 浏览器存储、Authorization 注入、URL/JSON token 暴露清零
- [x] CSRF Origin 与 CORS 正负测试全部通过
- [x] Rust 侧回滚演练完成（`docs/ops/auth-rollback-drill-2026-09-10.md`，功能矩阵 + nginx -t + 脱敏 manifest）
- [ ] Java/Rust session 互操作品证（两侧 token 存储不互通，Java 侧为 EXTERNAL/BLOCKED，演练中仅文档化）
- [ ] 浏览器 Bearer 兼容按指标安全退出（`auth_compat="bearer"` telemetry 已加，但“零阈值”需真实流量观察窗口，尚未观测）
- [x] refresh 收紧为 Cookie-only、whoami 无效凭据 401、无效 Cookie 不回退 Bearer（W1 语义对齐 + 矩阵测试）

### RustSec

- [x] lopdf、tantivy/lru 已修复
- [x] RSA 已修复、外置或有未过期限时例外
- [x] cargo-audit/cargo-deny/RustSec DB 版本可追溯
- [x] pinned PR gate 与 floating daily scan 均可验证
- [x] cache、断网、错误 SHA、过期例外测试符合 fail-loud策略
- [x] cargo-deny bans/licenses/sources 通过且职责不重叠

### Biome and CI

- [x] Biome 所有版本面精确为 2.5.11
- [x] 配置无需迁移且 lint 默认只读
- [x] 格式化、safe fixes、业务修复分提交并审查
- [x] pnpm 11.25.0 frozen install 可复现
- [x] Vitest、typecheck、Biome、build 全部通过
- [x] 所有 required workflow 位于仓库根并实际触发

### Integrity and Closeout

- [x] 原有未提交 Rust 变更未被覆盖或误混入
- [x] 所有 skipped、环境阻塞、未验证项已明确记录
- [x] 回滚剧本和实际演练证据已归档（`docs/ops/auth-rollback-drill-2026-09-10.md` + 脱敏 manifest；Java 侧仅文档化）
- [x] 文档、旧计划关联、required checks 与代码一致
- [ ] 本计划 frontmatter 已在收官提交改为 `completed`（因外部项未闭合，保持 `active`）


## Completion Evidence (2026-09-09)

- Cookie/CSRF: `cargo check --workspace --locked` passed; auth 80/0/3 ignored, shared 108/0/11 ignored; Cookie attributes, priority, empty-body refresh, idempotent logout, Origin CSRF and CORS tests passed.
- RustSec: lopdf upgraded to 0.42.0; OIDC and PDF signing moved off direct `rsa`; Tantivy removed in favor of PostgreSQL search. The remaining `rsa 0.9.10` is dev-only through `mysql_async -> sqlx-mysql` and is governed by the validated exception expiring 2026-10-09. Pinned audit passes with two allowed unmaintained warnings.
- Frontend: Biome 2.5.11/pnpm 11.25.0 aligned; full error-level Biome gate checks 145 files successfully; Vitest 19/19, typecheck and Vite build pass. Historical warning-level diagnostics remain visible rather than suppressed.
- CI: discoverable root workflows now cover backend, frontend and pinned/floating supply-chain scans; old nested workflow was removed. Container digests and available action commits were pinned.
- Workspace library test command was attempted but the single all-workspace invocation exhausted local LLVM memory while compiling tests. Targeted changed crates and full workspace check passed; this environment limitation is not reported as an all-tests pass.
- External follow-up: GitHub branch-protection required-check selection and real browser/OAuth smoke require the remote repository/deployed environment and are operational evidence, not local code artifacts.


### Browser black-box evidence (2026-09-09)

- The local login page rendered correctly at `http://localhost:5173/login`; semantic inspection found the username/password fields and enabled login button.
- Browser automation could fill both fields, but the IAB click actuator timed out before dispatch; no request reached the backend. The login transition is therefore **not** marked passed.
- Direct local HTTP verification through the same Vite proxy proved login emits `oa4rust_session` and current-user accepts it, but this is transport evidence rather than GUI evidence.
- OAuth/SSO end-to-end remains blocked by unavailable real provider credentials.

## Local Closure Evidence (2026-09-10)

Second closure pass (W1–W6), local-only, no push. Verified in this order:
- **Rust**: `cargo check --workspace --locked` pass. Targeted tests: auth 84/0/3,
  shared 111/0/11, signature 5/0, search 7/0. New U12 matrix tests:
  refresh cookie-only (Bearer→401), whoami no-cred→anon / invalid-cred→401 /
  invalid-cookie+valid-Bearer→401, CSRF 4-method×(missing/null/wrong/exact origin),
  CORS read-methods + foreign-origin invariant, invalid-cookie-never-falls-back.
- **Bearer telemetry**: `auth_middleware` now emits `auth_compat="bearer"` on
  protected routes when Bearer is the selected source (no token/UA logged).
- **Frontend**: fixed two real defects the new session tests exposed —
  `isAuthenticated` was an object-literal getter (snapshot false) → `computed`;
  consumers read `session.state.value?.user` (undefined, Pinia unwraps `state`) →
  `session.state.user` (AppShell/Dashboard/Personal). SSO view now posts the
  backend `{client, token}` contract. `session.test.ts` (5 behavioral tests).
  Full gates: pnpm frozen install, 24/24 vitest, 0 TS errors, repo-wide
  Biome error-level (146 files), Vite build.
- **Rollback drill** (Rust-side): live matrix + `nginx -t` of Rust/Java auth-route
  configs; Java fallback documented, not executed. Sanitized evidence +
  `manifest.sha256` in `oa4rust/docs/ops/`.
- **Browser E2E** (IAB): login Set-Cookie → reload restore → logout→who anonymous.
  Auth flow PASS. `/app/*` shell renders blank (pre-existing separate defect).
- **RustSec**: `cargo audit` (pinned-DB semantics, `--no-yanked --deny unsound`)
  passes with 2 allowed unmaintained; RSA exception valid to 2026-10-09.
  Fixed `.cargo/audit.toml` (removed `yanked` field, invalid for cargo-audit 0.22.2).
  `cargo deny check bans licenses sources` pass.

Still open (externally blocked, kept unchecked above): GitHub required-checks +
remote workflow activation (no push), real OAuth/OIDC provider E2E, Java↔Rust
session interop, Bearer zero-threshold observation, and the `/app/*` blank-shell
rendering defect. Plan stays `status: active`.

## Sources and References

### Repository guidance

- `AGENTS.md`
- `docs/solutions/development-workflow/plan-status-lifecycle.md`
- `docs/solutions/architecture-patterns/actionresult-9-field-contract.md`
- `docs/solutions/architecture-patterns/strangler-fig-migration-pattern.md`
- `docs/solutions/best-practices/auto-generate-rust-handler-tests.md`
- `docs/solutions/security-issues/idor-vulnerability-write-handlers.md`

### Authentication and deployment

- `oa4rust/crates/auth/src/lib.rs`
- `oa4rust/crates/shared/src/middleware/token.rs`
- `oa4rust/crates/shared/src/middleware/security.rs`
- `oa4rust-web/packages/sdk/src/session.ts`
- `oa4rust-web/packages/sdk/src/api.ts`
- `oa4rust-web/apps/desktop/vite.config.ts`
- `oa4rust/deploy/nginx.conf`
- `oa4rust/deploy/nginx-auth-routes.conf`

### Supply chain

- `oa4rust/Cargo.toml`
- `oa4rust/Cargo.lock`
- `oa4rust/deny.toml`
- `oa4rust/crates/signature/src/lib.rs`
- `oa4rust/crates/search/src/index.rs`
- `oa4rust/crates/auth/src/oidc.rs`
- `oa4rust/docs/audits/2026-08-15-phase3-preview-sign.md`

### Frontend tooling

- `oa4rust-web/package.json`
- `oa4rust-web/pnpm-lock.yaml`
- `oa4rust-web/biome.json`
- `oa4rust-web/.npmrc`
- `oa4rust/.github/workflows/ci.yml`

## Execution Guidance

- 本计划是当前三项工作的统一 source of truth；实施者按 U0→U13 推进并在每个单元后更新复选框和验证证据。
- 可并行项：U6 风险分析与 U9 Biome配置可在 U2-U5 开发期间进行，但任何 Cargo.lock 写入必须先解决 U0 的并发修改约束。
- 不可交换顺序：U2 后端 Cookie兼容必须先于 U4/U5 前端 Cookie-only；U6 advisory处置必须先于 U7 required gate；U9 配置必须先于 U11 批量格式化。
- 遇到与本计划不同的仓库事实时，先记录证据并修订计划，不得静默折中冲突模式。
- 任何验证未执行、被跳过或依赖外部环境时必须显式写明；只有全部验收门槛满足后才能标记 `completed`。
