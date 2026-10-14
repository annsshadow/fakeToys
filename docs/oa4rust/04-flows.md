# 04 · 主要业务流程

## 1. HTTP 请求全链路（中间件洋葱）

`create_app()`（`backend/src/lib.rs`）挂载顺序自外向内，**全部业务路由共用这一条链**：

```
请求 → trace（请求日志/追踪）
     → security_headers（安全响应头）
     → cors（按 APP_PUBLIC_ORIGIN 白名单）
     → rate_limit（全局限流；认证端点另有精确限流表）
     → csrf（写请求 CSRF 校验；APP_PUBLIC_ORIGIN 不匹配 → 403）
     → auth（会话识别：Cookie → SessionManager → 身份注入；豁免路径放行）
     → authorize（RBAC：PermissionRegistry 判定）
     → 业务 handler（crate 内 SQL 直查 tokio-postgres）
```

要点：

- 中间件只在顶层挂一次（`SecurityState` 单例），各 crate router 不自带中间件，避免重复或遗漏。
- `/health`、登录族等在 `AUTH_EXEMPT_PATHS/PREFIXES` 中豁免认证。
- MCP `/mcp` 子应用独立挂同一套中间件（`main.rs::mcp_app`）。

## 2. 密码登录流程

```
浏览器                          后端
  │ POST /api/authentication    │
  │ {credential, password}      │
  │────────────────────────────▶│ 1. rate_limit（精确表命中，1 分钟窗口）
  │                             │ 2. csrf 检查（login 在豁免/首登路径处理）
  │                             │ 3. auth::password 校验
  │                             │    （AUTH_LEGACY_COMPAT=true 时兼容旧 MD5/DES）
  │                             │ 4. 可选：captcha 校验 / two_factor
  │◀────────────────────────────│ 5. SessionManager 签发会话 → Set-Cookie（HttpOnly）
  │ {type:"success", data:{...}}│
  │ 后续请求自动携带 Cookie       │
```

会话存储：Redis 优先；Redis 不可达自动降级内存+DB（启动日志 `Redis unreachable; ... in-memory fallback`）。

**多通道登录**（同一信封出口）：SSO 加密票据（`/sso`、`/sso/encrypt`）、OAuth、OIDC、LDAP、企业微信、微信小程序、welink、政务钉钉、短信、双因素、切换用户——均位于 `crates/auth/src/`，各自独立文件实现。

## 3. 会话保活与登出

1. 业务请求收到 401 → 前端 SDK（`@oa4rust/sdk` `ApiClient`）自动 `POST /api/authentication/refresh` 重试一次。
2. 登出：`DELETE /api/authentication` 或 `POST /api/authentication/logout`；`safe_logout` 做服务端会话吊销 + Cookie 清理。
3. TTL 由 `SESSION_TTL_SECONDS` 控制（默认 7200 秒）。

## 4. 流程审批（processplatform）核心流

移动端/桌面端审批操作对齐后端**已注册路由**（与 crate `routes.rs`/`u2_router.rs` 一致，E2E 实跑通过）：

```
待办列表 → GET 任务列表（processplatform_assemble_surface 族，487 handler）
   ↓
任务详情 → 表单数据（xform 契约：frontend/src/contracts/xform.ts）
   ↓
审批动作 → POST /api/task/{id}/complete（通过）｜/api/task/{id}/reject（驳回）
   ↓
引擎推进 → processplatform_service_processing（113 handler）路由流转
   ↓
 BAM 埋点 → processplatform_assemble_bam（监控指标）
```

我发起 / 已办同列表族查询；设计器（表单/路由/脚本）走 `processplatform_assemble_designer`（96 handler）。

## 5. IM 实时消息流

```
发送端                         后端                          接收端
  │ POST 消息接口               │                             │
  │（message_assemble_communicate，58 handler，IM 会话收发族） │
  │───────────────────────────▶│ 持久化 + RealtimeManager 分发 │
  │                            │ ═══ /ws/realtime ══════════▶│ 实时收到消息
  │                            │                             │
  │ 会话列表/历史/已读          │  ← message crate 同族接口     │
```

- WebSocket 通道：`/ws/realtime`（全局）、`/ws/realtime/room/{room_id}`（房间）、`.../stats`（统计）。
- 移动端页面映射：`message`（会话列表）+ `chat`（历史 + 发消息 + 标记已读）。

## 6. 文件上传/下载与存储抽象

```
handler（file_assemble_control，94 handler）
   ↓ BlobStorage trait（shared/storage.rs）
   ├── FsBlobStorage   本地文件系统（root 可配）
   ├── PgBlobStorage   大对象入库（迁移 099 建 blob_storage 表）
   └── DbBlobStorage   DB 托管
选择：storage_from_env() 按环境变量决定实现
```

下载/预览配套：`preview` crate（文档预览）、`signature` crate（PDF 签名，lopdf 0.42）。移动端 `doc` 页面走 `uni.uploadFile` 上传 + 下载预览。

## 7. 数据库迁移流（启动内嵌）

```
进程启动
  ├─ dotenvy 加载 .env
  ├─ create_pool()（DATABASE_URL）
  ├─ run_migrations(&pool)
  │    ① 确保 schema_migration 版本表存在
  │    ② 收集 migrations/：排除 *_rollback.sql 与 archive/
  │    ③ 按文件名序逐个执行未应用脚本（幂等）
  │    → MigrationReport { applied: N, skipped: M }（打 info 日志）
  ├─（--migrate-only 则在此退出）
  └─ 继续正常启动
```

回滚：手工执行对应 `*_rollback.sql`（设计上启动不自动回滚）。

## 8. 部署与回滚流（Rust 版本内）

**口径（2026-10-14 定案）**：本项目从未上线、无 Java 存量系统——不存在"从 o2server 切流 / 回切 Java"的运维动作。上线判据见 `backend/docs/ops/REALIZE_RUNBOOK.md` §4.2：全部静态门禁 + 仓内 live E2E 通过即上线。

- **部署**：`cargo build --release` 单二进制 + `pnpm build` 前端产物；`backend/deploy/nginx.conf` 为反代模板（按需自改）。`deploy/` 下其余切流脚本为早期灰度方案遗留，仅作归档参考。
- **回滚**：回退到**上一 Rust 版本**的二进制 + 对应迁移回滚脚本（手工执行 `*_rollback.sql`），无跨系统回滚。
- 历史灰度手册 `backend/docs/gray-release-playbook.md` 与演练记录 `backend/docs/ops/auth-rollback-drill-2026-09-10.md` 属早期方案档案，不再作为部署流程依据。

## 9. 三端契约对齐流

```
后端 crate 路由注册（routes.rs / u2_router.rs）
        ↓ 脚本抽取
tests/contracts/backend-registered-routes.fixture.ts
backend-route-methods.fixture.ts
        ↓ vitest 对账（pnpm test 时强制执行）
desktop-endpoints.test.ts   ←→ frontend/apps/desktop/src/contracts/*.ts
mobile-endpoints.test.ts    ←→ mobile/src/services/index.ts
schema.test.ts + schemas.json / fixtures.json（形状校验）
production-serialization.test.ts（生产信封防变形）
empty-catch-guard.test.ts（空 catch 防回归）
```

任何一端新增/删除端点而未对齐三端，`pnpm test` 即红——**这是三端不漂移的强制机制**。

## 10. MCP 工具调用流

```
MCP 客户端 ── POST /mcp（JSON-RPC 2.0，Authorization 头透传）──▶ mcp_app
   initialize → 返回协议版本 2024-11-05 与 serverInfo（oa4rust-mcp 0.1.0）
   tools/list → ToolBridge.list_tools()（generated_routes.rs 清单）
   tools/call → ToolBridge.call_tool() 复用业务 handler 执行（会话鉴权同 HTTP）
```

启动：`cargo run -- --http`；不启用时 `/mcp` 不存在。

## 11. 依赖安全维护流（常态化）

```
advisory-db 固定版本（security/rustsec-db.rev）
   ↓
cargo audit 对 Cargo.lock 全量比对（每日浮动门；--deny unsound/vulnerable）
   ↓ 命中
① 无补丁且不可达 → security/rustsec-exceptions.toml 登记例外（带到期日）
② 有补丁 → cargo update -p <pkg> 抬版本（如 2026-10-14：quinn-proto → 0.11.19）
   ↓
前端/移动端 pnpm audit → pnpm-workspace.yaml overrides 抬版本
   ↓
审计报告落 docs/audits/YYYY-MM-DD-security-audit.md
```
