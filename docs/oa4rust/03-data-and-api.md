# 03 · 关键数据结构与接口说明

## 1. 统一响应信封 `ActionResult<T>`

定义：`oa4rust/backend/crates/shared/src/response.rs`。**所有 API 响应的统一 JSON 结构**，字段与 o2server v9（Gson）实测对齐，`None` 字段跳过序列化：

```jsonc
// 成功（非分页，ActionResponse::success）
{
  "data": { /* 业务数据 */ },
  "type": "success",
  "message": "",            // 成功时为空串（与 o2server 一致）
  "date": "2026-10-14 12:00:00",  // 服务器时间 yyyy-MM-dd HH:mm:ss
  "spent": 0,               // 耗时 ms（此处恒 0；精确耗时尚未实现，保持对齐 o2server v9 的信封行为）
  "size": 0,
  "count": 0,
  "position": 0
}
// 分页端点使用 legacy_success(count, size) 填真实计数；非分页传 (0, -1)

// 错误：无 data 字段；prompt 承载 o2server 异常类名风格字符串
{
  "type": "error",
  "message": "具体错误描述",
  "date": "2026-10-14 12:00:00",
  "spent": 0, "size": -1, "count": 0, "position": 0,
  "prompt": "ExceptionEntityNotExist"
}
```

**前端判定规则**：以 `type` 字段区分成功/错误（`"success"` / `"error"`），不要依赖 HTTP 状态码以外字段做分支。

## 2. 错误类型 `AppError`

定义：`shared/src/error.rs`，实现 `IntoResponse`：

| 变体 | 语义 | 典型 HTTP |
| --- | --- | --- |
| `Database(String)` | 数据库错误 | 500 |
| `Redis(String)` | Redis 错误 | 500 |
| `Internal(String)` | 内部错误（带消息） | 500 |
| `InternalServerError` | 内部错误（不带细节，防泄漏） | 500 |
| `BadRequest(String)` | 请求参数错误 | 400 |
| `Unauthorized` | 未认证 | 401 |
| `NotFound` | 资源不存在 | 404 |
| `Forbidden` | 无权限（RBAC/CSRF） | 403 |
| `NotImplemented` | 未实现 | 501 |

## 3. 认证会话模型

- **形态**：HttpOnly Cookie 会话（浏览器安全基线）。会话体由 `SessionManager` 管理：**Redis 优先**，不可达时**自动降级内存+DB**（启动日志会说明实际模式）。
- **配置**（`shared/src/config.rs` `AuthConfig::from_env()`）：
  - `APP_PUBLIC_ORIGIN`：浏览器请求 API 的公开源，**必须与实际访问 origin 一致**，否则 CSRF 层以 403 拒绝写请求；
  - `AUTH_COOKIE_SECURE`：生产必须 `true`（HTTPS）；
  - `SESSION_TTL_SECONDS`：会话 TTL（默认 7200）；
  - `RUST_ENV=production` 时上述项显式必填。
- **迁移期兼容**：`AUTH_LEGACY_COMPAT=true` 接受旧 MD5/DES 密码哈希与 3DES SSO 令牌（一次性，启动打 warn）。

### 核心 auth API（`crates/auth/src/lib.rs`，节选）

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| POST | `/api/authentication` 或 `/api/authentication/login` | 登录（限流精确命中这两个路径） |
| DELETE | `/api/authentication`；POST `/api/authentication/logout` | 登出 / 安全登出 |
| GET | `/api/authentication` 或 `/api/authentication/who` | 当前用户（whoami） |
| POST | `/api/authentication/refresh` | 会话刷新（前端 401 自动重试依赖它） |
| POST | `/api/authentication/code` | 验证码 |
| POST | `/api/authentication/sso`；`/sso/encrypt` | SSO 登录 / 加密票据 |
| GET | `/api/authentication/oauth/list` | OAuth 提供方列表 |
| GET | `/api/authentication/unit/list` / `role/list` / `group/list` | 单位/角色/群组列表 |

另有 oauth / oidc / ldap / mpweixin / qiyeweixin / welink / zhengwudingding / sms / two_factor / switch_user / bind 等族路由（同文件 980 行起）。

### 中间件豁免与限流目标（`shared/src/middleware/constants.rs`）

- `AUTH_EXEMPT_PATHS` / `AUTH_EXEMPT_PREFIXES`：无需会话即可访问的路径（登录、验证码、`/health` 等）。
- `AUTH_RATE_LIMIT_EXACT = ["/api/authentication", "/api/authentication/login"]` + `AUTH_RATE_LIMIT_PREFIXES`：认证类端点单独限流；`RATE_LIMIT_WINDOW_MINUTES = 1`。
- `SECRET_INIT_PATHS = ["/api/secret/check", "/api/secret/set"]`。
- `ADMIN_WRITE_PREFIXES` / `ADMIN_WRITE_DEPRECATED_PREFIXES`：管理写族门禁（越权收口模式，见审计文档）。

## 4. 路由与端点总览

| 端点 | 说明 |
| --- | --- |
| `GET /health` | 健康检查，返回 `"ok"`（`shared/src/router.rs`，无认证） |
| `GET /openapi.json` | utoipa `ApiDoc` 全量 OpenAPI JSON（`main.rs` 挂载） |
| `/ws/realtime` | WebSocket 实时协议入口（`realtime::ws_route()`） |
| `/ws/realtime/room/{room_id}` | 房间通道 |
| `/ws/realtime/room/{room_id}/stats` | 房间统计 |
| `POST /mcp` | MCP JSON-RPC（仅 `--http` 启动参数时挂载，复用会话认证） |
| `/*`（其余） | SPA 静态托管：命中静态文件原样返回，未命中回 `index.html`；`/api/**`、`/ws/**` 未知路径保持 404 |

业务 API 全量清单：以 `/openapi.json` 与 `backend/docs/brainstorms/oa4rust-endpoint-inventory.md` 为准（2645 handler，88/90 crate done）。

## 5. 核心横切接口（shared）

| 接口 | 签名要点 | 用途 |
| --- | --- | --- |
| `create_pool()` | `db::create_pool() -> Pool`（deadpool-postgres） | 建连接池，读 `DATABASE_URL` |
| `run_migrations(&Pool)` | `-> MigrationReport { applied, skipped }` | 幂等执行 `migrations/`（忽略 `*_rollback.sql` 与 `archive/`，按文件名序） |
| `SessionManager` | `with_pool_and_config(pool, auth_config)`、`init_redis() -> bool` | 会话存取 |
| `RateLimiter` | `new()`、`init_redis() -> bool` | 限流（内存/Redis 双模） |
| `SqlDialect`（trait） | `quote_ident / param / json_type / cast_text / ilike_op / format_sql` … | PG/MySQL 方言切换（`db/dialect.rs`） |
| `rewrite_pg_to_mysql` | `db/rewriter.rs` | PG SQL → MySQL 兼容重写 |
| `BlobStorage`（trait） | 实现：`FsBlobStorage` / `PgBlobStorage` / `DbBlobStorage`；`storage_from_env()` | 文件二进制存储抽象 |
| `MessageBus`（trait） | `InMemoryBus` / `RedisPubSubBus`；`TokenThresholdEvent` | 进程内/跨进程事件总线 |
| `CrudSpec` + `crud_create/save/delete` | `crud.rs` | 通用真实化 CRUD handler 生成 |
| `row_to_json(&Row)` | `response.rs` | 未知列通用行转 JSON（核心标量类型；时间等复杂类型建议 handler 显式映射） |
| `PermissionLevel / PermissionRegistry` | `middleware/rbac.rs` | RBAC 授权模型（authorize 中间件使用） |

## 6. WebSocket 实时协议（`realtime` crate）

- `RealtimeMessage`：消息体结构；`RealtimeManager`：房间/连接管理（自带 state，不依赖 pool）。
- 端点见上表；`/ws/**` 未知路径 404，不回退 HTML。
- 前端对应：桌面 `@oa4rust/sdk` 的 `websocket.ts`；移动端 H5 走同一协议。

## 7. MCP 工具接口（`mcp_server` crate）

- 传输：HTTP `POST /mcp`，JSON-RPC 2.0；启动需 `--http` 标志。
- 方法：`initialize`（协议版本 `2024-11-05`）、`tools/list`、`tools/call`；未知 method 返回 `-32601`。
- 鉴权：透传调用方 `Authorization` 头，复用现有会话认证，无重复逻辑（`main.rs::mcp_app`）。
- 工具桥：`tool_bridge::ToolBridge`——把后端已注册业务路由封装为 MCP 工具（清单见 `generated_routes.rs`，可用 `backend/scripts/gen_mcp_tools.py` 生成）。

## 8. 数据库迁移约定

- 目录：`oa4rust/backend/migrations/`（顶层 213 个 `.sql` 文件，其中 114 正向 + 99 `*_rollback.sql`；正向编号至 112；历史归档在 `archive/`）。
- 命名：正向 `NNN_description.sql`；回滚 `NNN_description_rollback.sql`（启动时**忽略**）；历史归档 `archive/`（忽略）。
- 执行：启动自动幂等执行（`run_migrations`），无需手工 SQL；`--migrate-only` 仅跑迁移后退出（CI/部署流水线用）。
- **成对约定**：新增正向迁移必须同时提供 `*_rollback.sql`（001–021 等 15 个早期迁移无回滚文件，其余历史迁移均为成对文件）。

## 9. 前端契约类型（TypeScript）

- SDK 类型源：`frontend/packages/sdk/src/types.ts`（`O2User`、`OrgGroup`、`PagedResponse`、`ApiResponse` 等）。
- 消费规则：**移动端只 `import type`**（编译期擦除，不引运行时）；桌面端正常引运行时。
- 契约守卫：`tests/contracts/schema.test.ts` + `schemas.json` / `fixtures.json` 校验三端形状一致；`production-serialization.test.ts` 防止生产信封被本地化变形。
