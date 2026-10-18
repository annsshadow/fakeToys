---
title: "浏览器认证从 JS 可读 token 迁移到 HttpOnly Cookie"
date: 2026-09-09
last_refreshed: 2026-10-14
category: security-issues
module: oa4rust/crates/auth + oa4rust/frontend/packages/sdk
problem_type: security_migration
component: authentication
severity: high
symptoms:
  - "登录后 Cookie 无法成为有效认证来源：auth 写 oa4rust_token，中间件读 token=，名称漂移"
  - "前端从 localStorage 读 token 注入 Authorization，HttpOnly 化之前 XSS 即可窃 credential"
  - "CORS 允许 credentials 但未覆盖 PUT/PATCH/DELETE，Cookie 认证无 CSRF 门禁"
  - "后端登录要求 credential/password，前端发 username/password；refresh 要求 header/body token 相等，前端发空 body"
root_cause: token_based_auth_design
resolution_type: architecture_migration
tags: [httponly-cookie, csrf, cors, session, authentication, xss]
applies_when:
  - "把 Web 应用从 Bearer/localStorage 认证迁到 HttpOnly Cookie"
  - "为 Cookie 认证增加 CSRF 防护"
  - "设计登录/刷新/登出的会话生命周期"
related_components:
  - oa4rust/backend/crates/auth/src/lib.rs
  - oa4rust/backend/crates/shared/src/middleware/token.rs
  - oa4rust/backend/crates/shared/src/middleware/csrf.rs
  - oa4rust/backend/crates/shared/src/middleware/security.rs
  - oa4rust/frontend/packages/sdk/src/session.ts
  - oa4rust/frontend/packages/sdk/src/api.ts
---

# 浏览器认证从 JS 可读 token 迁移到 HttpOnly Cookie

## Problem

浏览器把认证 token 放在 localStorage 并手动注入 `Authorization`，任何 XSS 都能窃取 credential。迁移到 HttpOnly Cookie 涉及全链路改名、生命周期、CSRF、CORS、前后端契约与兼容窗口，任一环断裂即登录不可用。

## Resolution

### Cookie 契约（单一来源常量）

| 属性 | 值 | 理由 |
|------|----|----|
| Name | `oa4rust_session`（shared 常量 `SESSION_COOKIE_NAME`） | 消除 auth 与 middleware 字符串漂移；表达会话语义 |
| HttpOnly / Secure / SameSite | true / 生产 true / Lax | 防 XSS 读取；不扩大到跨站架构 |
| Path / Domain | `/` / 不设置（host-only） | 缩小泄露范围 |
| Max-Age | 与服务端 session TTL 一致；清理用 Max-Age=0 + 过去 Expires | 旋转与清理同属性 |

### 认证优先级（防身份混淆）

1. 存在 Cookie：只验证 Cookie。
2. Cookie 不存在：兼容期允许 Bearer（CLI/旧客户端）。
3. **Cookie 存在但空/过期/篡改：401，不回退 Bearer**——单请求身份混淆是最高危场景。
4. 兼容期结束后浏览器 Bearer 停用；CLI/service Bearer 另有独立契约。

### CSRF / CORS

- Cookie 认证的 POST/PUT/PATCH/DELETE 强制 `Origin` 存在且与 `APP_PUBLIC_ORIGIN` 精确匹配 scheme/host/port；Bearer-only 非浏览器请求豁免，**同时有 Cookie+Bearer 不得豁免**。
- CORS 补齐 PUT/PATCH/DELETE，精确 Origin + credentials，禁止 wildcard+credentials。
- 同源部署下 Origin 校验比 double-submit 更简单；`SameSite=Lax` 作纵深。

### 会话生命周期契约

- 登录 `credential/password`（对齐后端既有字段，勿发明 username）；JSON 只回用户/过期信息，响应 `Cache-Control: no-store`。
- refresh 只依赖 Cookie（空 body），旋转 session 并重新 Set-Cookie；**不接受 body token、不要求 Bearer**（HttpOnly 的目的就是 JS 不可读）。
- logout 幂等：无论 session 是否已失效都清 Cookie，支持撤销则同时撤销服务端。
- 前端 401 最多 refresh 一次、重放一次；**并发 401 共享一个 refresh Promise** 防刷新风暴；refresh 自身 401 清空内存态不循环。

### 迁移阶段（按序，可回滚）

A 后端双通道（Cookie+Bearer）→ B 新前端 Cookie-only 并观测旧 Bearer 用量 → C 停浏览器 JSON token → D 达阈值后删兼容代码。前端回滚必须与后端兼容窗口协调，严禁单独关闭后端兼容后回滚前端。

## 实测踩坑（同类项目直接可用）

- **Pinia 状态解包**：`store.state` 是 ref，模板/JS 中 `session.state.user` 而非 `session.state.value?.user`——对象字面量 getter 快照恒 false 的 bug 只有行为测试能抓到。
- **登录必须经 requestSubmit 触发**：IAB 指针/键盘 actuator 取不到点击点，真实浏览器黑盒登录只有表单提交路径可验证。
- 回滚演练要产出脱敏 manifest（功能矩阵 + `nginx -t` + sha256），证据归档 `docs/ops/`。

## Why This Matters

HttpOnly 迁移的失败模式不是"安全功能没生效"而是"登录全挂"或"随机 401"。名称单一来源、优先级不回退、CSRF 只卡危险方法、refresh 单飞，四条规则使迁移可灰度、可回滚、可观测。

## When to Apply

- 任何 Bearer/localStorage 认证的 Web 应用安全加固
- Cookie 认证的新后端设计（一开始就内建 CSRF Origin 校验）

## Related

- 计划：`docs/archived-plans/2026-09-09-001-refactor-httponly-cookie-cargo-audit-rustsec-db-biome-2-5-11-plan.md`（U2-U5、U12、Local Closure Evidence）
- 回滚演练证据：`oa4rust/backend/docs/ops/`（auth-rollback-drill-2026-09-10）
- 前端 SPA 壳层路由陷阱：`../integration-issues/vue-spa-shell-routing-pitfalls.md`
