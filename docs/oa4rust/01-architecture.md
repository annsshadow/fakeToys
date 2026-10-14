# 01 · 项目定位与整体架构

## 1. 项目定位

OA4Rust 是 **O2OA 开源办公平台（Java 版 o2server）的 Rust 重写项目**，同时配套重写桌面 Web 端与移动端。核心定位：

1. **后端替换**：以 Rust（axum + tokio）逐模块重写 o2server 的各个 Java web 应用（war），最终用单个 Rust 二进制替换整个 Java 集群。
2. **行为对齐**：API 路径、请求/响应信封与 o2server v9（Gson 序列化）保持一致，使存量前端（o2 官方 Web / 三方集成）**无感切换**。对齐结论来自行为对比实跑（基准 o2server v9 Gson 实测，见 `oa4rust/backend/crates/shared/src/response.rs` 注释）。
3. **灰度迁移**：通过 nginx 影子流量 + 分模块切流（影子 → 10% → 50% → 90% → 100%）安全迁移，RTO 目标 5 分钟（见 `oa4rust/backend/docs/gray-release-playbook.md`）。

**现状（2026-10）**：90 个业务 crate 中 88 个已完成真实化（无 stub 桩、无 `Value::Null` 桩），真实 handler 共 2645 个；待迁移仅剩 2 个（`sms`、`captcha_store` 等基础设施级 crate，见端点清单汇总）。

## 2. 总体架构

```
                    ┌──────────────────────────────────────────────┐
                    │              浏览器 / 移动端 / MCP 客户端       │
                    └───────┬──────────────┬───────────────┬───────┘
                            │ Cookie 会话   │ WebSocket     │ JSON-RPC
                            ▼              ▼               ▼
   ┌─────────────────────────────────────────────────────────────────────┐
   │                   oa4rust 单二进制（axum Router，:3000）              │
   │                                                                     │
   │  静态托管: frontend/dist/web（SPA 回退, OA4RUST_WEB_DIST 可覆盖）      │
   │  /openapi.json   /health   /ws/realtime[*]   /mcp（--http 时挂载）    │
   │                                                                     │
   │  中间件洋葱（外→内）:                                                  │
   │    trace → security_headers → cors → rate_limit                     │
   │      → csrf → auth(会话识别) → authorize(RBAC) → 业务 handler         │
   │                                                                     │
   │  lib.rs create_app(): 约 80 个 crate 路由 .merge() 成一棵 Router      │
   │  （构造需 64MB 栈，见 .cargo/config.toml /STACK 说明）                 │
   └──────────┬──────────────────────┬───────────────────┬───────────────┘
              │ deadpool-postgres     │ Redis（可选）      │ BlobStorage 抽象
              ▼                       ▼                   ▼
   ┌──────────────────┐   ┌──────────────────┐   ┌──────────────────────┐
   │ PostgreSQL 14+   │   │ Redis 7          │   │ Fs / Pg / Db 三实现   │
   │ （主库；MySQL 8   │   │ 会话存储 / 分布式 │   │ （shared/storage.rs， │
   │  经方言抽象兼容） │   │ 限流 / PubSub 总线│   │  storage_from_env）  │
   └──────────────────┘   └──────────────────┘   └──────────────────────┘
              ▲
              │ 影子流量 / 灰度切流（nginx，迁移期并存）
   ┌──────────┴─────────┐
   │ o2server (Java)     │  legacy 服务 :20020 / docker :18080
   └────────────────────┘
```

## 3. 三端 monorepo 组织

`oa4rust/` 是 pnpm workspace 根（`pnpm-workspace.yaml`：`frontend/apps/*`、`frontend/packages/*`、`mobile`），前后端共用一个目录但工具链独立：

| 端 | 技术 | 位置 | 职责 |
| --- | --- | --- | --- |
| 后端 | Rust 1.75+ / axum / tokio / deadpool-postgres / utoipa | `oa4rust/backend/` | 全部业务 API、WebSocket、MCP、静态托管 |
| 桌面 Web | Vue 3.5 + Vite 8 + naive-ui + pinia + vue-query + unocss | `oa4rust/frontend/apps/desktop/` | 完整 OA 桌面端（98 个视图文件） |
| 共享包 | TypeScript | `oa4rust/frontend/packages/{apis,sdk,ui,locales}/` | API 转发层 / 传输与状态 SDK / 组件与主题 / 多语言 |
| 移动端 | uni-app（Vue3 + Vite + TS） | `oa4rust/mobile/` | H5 / 微信小程序 / 原生 App，16 页面 |
| 契约守卫 | vitest | `oa4rust/tests/contracts/` | 三端端点 × 后端注册路由对账 |
| E2E | Playwright | `oa4rust/e2e/` | 端到端流程（需 live 后端） |

**前端共享包分层**（移动端刻意只 `import type` 复用 SDK 类型，避免拖入桌面运行时与双份 Vue）：

- `@oa4rust/apis`：API 转发层（`create-request`、forwarders、覆盖率测试）。
- `@oa4rust/sdk`：`ApiClient`（401 自动 refresh 重试）、`app`、`auth`、`session`、`router`、`theme`、`i18n`、`websocket`、`widget`。
- `@oa4rust/ui`：共享组件 / 视图 / 主题（unocss）。
- `@oa4rust/locales`：多语言文案。

## 4. 后端 crate 体系

后端是 **96 个 crate 的 cargo workspace**，分四类（命名延续 o2server 的 assemble/core 惯例）：

| 类别 | 命名模式 | 例子 | 说明 |
| --- | --- | --- | --- |
| 业务入口 crate | 模块名或 `*_assemble_control` / `*_service_processing` 等 | `auth`、`bbs_assemble_control`、`processplatform_assemble_surface` | 持 axum Router + handler，直接写 SQL（tokio-postgres） |
| 实体/表达 crate | `*_core_entity` / `*_core_express` | `organization_core_entity` | 领域模型与 DTO |
| 基础设施 | `shared`、`orm`、`auth`（部分）、`mcp_server`、`openapi`、`realtime`、`search`、`preview`、`signature`、`parity` | — | 跨模块复用，不计入迁移统计 |
| 适配器 | `ldap`、`sms`、`captcha_store`、`jpush` | — | 外部系统对接 |

每个业务 crate 的标准内部结构（以 `bbs_assemble_control` 为例）：

```
crates/bbs_assemble_control/src/
├── lib.rs              # pub fn router(pool) -> Router，路由注册
├── routes.rs / u2.rs   # 路由表与 U2 波次实现
├── tests.rs            # 手写单测
└── tests_generated.rs  # 脚本生成的 handler 测试（tests_generated.rs）
```

**单一入口组装**：`backend/src/lib.rs` 的 `create_app()` 将约 80 个 crate 的 Router 链式 `.merge()`；`backend/src/main.rs` 负责：环境加载 → 迁移执行 → Redis 初始化（失败降级）→ LDAP 同步 worker（可选）→ `create_app` → 挂载 `/openapi.json` / `/mcp`（`--http`）→ 静态 SPA 托管 → `:3000` 监听。

## 5. 关键架构决策（已定案，勿反复）

| 决策 | 内容 | 依据位置 |
| --- | --- | --- |
| 单二进制单 Router | 全部 crate 路由合并为一个 axum Router；中间件只在顶层挂一次，避免重复/遗漏 | `backend/src/lib.rs`、`shared/src/router.rs` |
| 64MB 主线程栈 | `create_app` 巨型 Router 构造需 ~64MB 栈；Windows-MSVC 用 `/STACK:67108864`，测试用显式大栈线程 | `backend/.cargo/config.toml`、`lib.rs` cors_guard 测试 |
| 会话 = HttpOnly Cookie | 浏览器安全基线；Redis 优先存储，不可达降级内存+DB | `backend/docs/security/browser-session-cookie.md` |
| 响应信封对齐 o2server v9 | `ActionResult<T>`：`data/type/message/date/spent/size/count/position(/prompt)`，`None` 字段跳过序列化（对齐 Gson） | `shared/src/response.rs` |
| SQL 方言最小抽象 | 只抽象占位符/类型转换/引号/ILIKE 等实际用到的差异；PG 为主，MySQL 经 rewriter 兼容 | `shared/src/db/dialect.rs`、`2026-08-18-sql-dialect-abstraction.md` |
| 迁移内嵌启动 | 启动时自动跑幂等迁移（`NNN_desc.sql`，忽略 `*_rollback.sql` 与 `archive/`），支持 `--migrate-only` | `shared/src/migrate.rs` |
| 桩零容忍 | 迁移完成度用脚本静态扫描（stub_/Null 桩计数），产物即"单一信息源"文档 | `backend/docs/brainstorms/oa4rust-endpoint-inventory.md` |

## 6. 与 legacy o2server 的关系

- **迁移期并存**：nginx 按模块把流量在 Rust 与 Java 之间切换（`backend/deploy/nginx.conf`、`toggle_module.sh`）。
- **对齐验证**：`backend/tests/parity/` + `parity_suite.rs` 与 Java 基准做行为比对（本地 PG 全量迁移 379 表后 parity 4143/4143 通过）。
- **一次性迁移开关**：`AUTH_LEGACY_COMPAT=true` 时接受旧 MD5/DES 密码哈希与 3DES SSO 令牌（启动时打 warn 日志，数据迁移完成后必须关闭）。
