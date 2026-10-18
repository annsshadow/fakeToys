# 05 · 环境依赖与配置

## 1. 环境依赖

### 后端（Rust）

| 依赖 | 版本要求 | 说明 |
| --- | --- | --- |
| Rust 工具链 | **1.85+**（`[workspace.package] rust-version = "1.85"`） | CI 供应链扫描固定用 Rust 1.88.0（与应用最低版本是两个独立约束） |
| PostgreSQL | **14+**（docker-compose 用 postgres:14） | 主库；生产另有 PostgreSQL 17 实测记录 |
| MySQL 8（可选） | 8.x | 多数据库兼容验证用（`SqlDialect` + rewriter） |
| Redis（可选但推荐） | 7.x（redis:7-alpine） | 会话存储与分布式限流的**默认后端**；不可达自动降级内存+DB |

### 前端 / 移动端

| 依赖 | 版本 | 说明 |
| --- | --- | --- |
| Node.js | 22 LTS（workspace 管理版 22.22.2 亦可） | Vite 8 要求 |
| pnpm | **11.25.0**（`packageManager` 字段固定） | workspace 根执行 `pnpm install` |
| 浏览器（E2E） | Playwright 1.62.1 | `pnpm test:e2e` 需先装浏览器 |

### Windows 本机 GNU 编译（可选）

`oa4rust/vendor/`（mingw-bin / mingw-gcc，~450MB，**不入 git**）支撑 `stable-x86_64-pc-windows-gnu` 目标本机 `cargo check/test`。复现方法见 `oa4rust/vendor/README.md`（MSYS2 包清单 + DLL 版本匹配要点）。MSVC 目标则只需 Visual Studio Build Tools。

## 2. 后端环境变量

参考模板：`oa4rust/backend/.env.example`（复制为 `.env`；`dotenvy` 启动自动加载）。

| 变量 | 必填 | 默认/示例 | 说明 |
| --- | --- | --- | --- |
| `DATABASE_URL` | ✅ | `postgres://o2server:password@localhost:5432/oa4rust` | PostgreSQL 连接串 |
| `DB_DIALECT` | 否 | `postgres` | SQL 方言选择：`postgres`（默认）/ `mysql`（经 `SqlDialect` + PG→MySQL rewriter 兼容，见 `shared/src/db/dialect.rs`） |
| `REDIS_URL` | 推荐 | `redis://127.0.0.1:6379` | 会话/限流/PubSub；不可达自动降级 |
| `RUST_ENV` | ✅（生产） | `development` | `production` 时强制校验认证配置 |
| `APP_PUBLIC_ORIGIN` | ✅ | `http://localhost:3000` | **必须等于浏览器请求 API 的 origin**；不一致 → CSRF 403。Vite 代理开发时为 `http://localhost:5173` |
| `AUTH_COOKIE_SECURE` | 生产 ✅ | `false`（开发） | 生产 HTTPS 必须 `true` |
| `SESSION_TTL_SECONDS` | 否 | `7200` | 会话有效期（秒） |
| `OA4RUST_WEB_DIST` | 否 | `../../../frontend/dist/web` | 前端静态产物目录（相对二进制位置），可绝对路径覆盖 |
| `AUTH_LEGACY_COMPAT` | 否 | 未设 | `true` 时接受旧 MD5/DES 哈希与 3DES SSO 令牌（一次性迁移；开启会打 warn 日志，迁完即关） |
| `LDAP_SYNC_ENABLE` | 否 | 未设 | `true` 启动 LDAP 用户自动同步 worker（幂等） |
| `RUST_LOG`（EnvFilter） | 否 | — | tracing 过滤；代码默认加 `o2server=debug` 指令 |
| `VITE_OA_API_BASE`（移动端） | 原生/小程序 | — | `setApiBase` 指向后端绝对地址（H5 走代理不需要） |

## 3. docker-compose 一键依赖（`oa4rust/backend/docker-compose.yml`）

| 服务 | 镜像 | 端口 | 用途 |
| --- | --- | --- | --- |
| postgres | postgres:14 | 5432 | 主库（o2server/password，库 oa4rust，带健康检查） |
| mysql | mysql:8 | 3306 | 多数据库兼容测试 |
| redis | redis:7-alpine | 6379 | 会话/限流 |
| o2server | o2oa/o2server:latest | 18080→80 | O2OA v9 行为对齐基准容器（开发期 parity 测试可选依赖；启动期最长 300s） |

```bash
cd oa4rust/backend && docker compose up -d
```

## 4. 数据库迁移

- 目录：`oa4rust/backend/migrations/`（顶层 205 个 `.sql`，编号 001–108 + `archive/` 归档）。
- **启动自动幂等执行**，无需手工 SQL；`--migrate-only` 供流水线单独跑迁移。
- 全量迁移后约 **379 张表**（parity 全量测试的实测基准）。

## 5. 前端门禁配置（workspace 根）

| 文件 | 作用 |
| --- | --- |
| `pnpm-workspace.yaml` | workspace 成员 + 安全 override（Dependabot 告警抬版本，如 vue 3.4.21→3.5.42 家族、esbuild/qs/vite 范围覆盖）+ `allowBuilds` + uni-app alpha 白名单 |
| `biome.json` | Lint/format（`biome check --error-on-warnings`） |
| `tsconfig.base.json` | TS 基线 |
| `vitest.config.ts` | 单测 + 覆盖率（desktop + packages + contracts） |
| `playwright.config.ts` | E2E（需 live 后端） |
| `unocss.config.ts` | 原子 CSS |

## 6. Rust 侧特殊配置

| 文件 | 内容 |
| --- | --- |
| `backend/.cargo/config.toml` | Windows-MSVC 主线程栈 `/STACK:67108864`（64MB）——`create_app` 巨型 Router 构造所需；仅影响主线程，不影响运行时 worker |
| `backend/deny.toml` | cargo-deny 配置（许可证/依赖审计） |
| `backend/security/rustsec-exceptions.toml` | RustSec 例外（带到期日，如 lopdf 2026-10-28、rsa 2026-11-08） |
| `backend/security/rustsec-db.rev` | advisory-db 固定版本（审计可复现） |
| `oa4rust/.npmrc` | pnpm registry 配置 |

## 7. 部署相关（`backend/deploy/`）

| 文件 | 用途 |
| --- | --- |
| `nginx.conf` | nginx 反代模板（按需自改） |
| `nginx-auth-routes.conf` / `nginx-auth-routes-rust-drill.conf` | 早期灰度方案遗留（历史归档） |
| `toggle_module.sh` | 早期模块切流脚本（无旧系统可切，历史归档） |
| `auth-rollback-drill.sh` | auth Cookie 功能矩阵演练脚本（功能回归用途，与旧系统无关） |
