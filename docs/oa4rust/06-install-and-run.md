# 06 · 安装与运行方式

## 1. 后端：从零到运行

### 1.1 准备依赖

```bash
# 方式 A：docker-compose 一键起 PostgreSQL + Redis（+ 可选 MySQL/o2server 基准）
cd oa4rust/backend
docker compose up -d postgres redis      # 最小集；全量则不带服务名

# 方式 B：本机已有 PostgreSQL 14+/Redis 7，直接用
```

### 1.2 配置

```bash
cd oa4rust/backend
cp .env.example .env
# 编辑 .env：至少确认 DATABASE_URL；
# 本机用 Vite 代理开发前端时把 APP_PUBLIC_ORIGIN 改为 http://localhost:5173
```

### 1.3 编译与启动

```bash
cargo run                # 默认：执行迁移 + 启动 API（:3000）
cargo run -- --migrate-only   # 仅执行数据库迁移后退出（流水线用）
cargo run -- --http           # 额外挂载 MCP 端点 /mcp
```

启动日志关键行（验证用）：

```
migrations: N applied, M already applied (skipped)
Redis backend initialized for session store and rate limiter   # 或 in-memory fallback 降级行
listening on 0.0.0.0:3000
```

Windows 一键编译（MSVC 工具链）：`oa4rust/backend/compile.bat`（检查 Rust → `cargo clean` → `cargo build --workspace`，产物 `target\debug\oa4rust.exe`）。

GNU 目标本机编译：见 `oa4rust/vendor/README.md`（需 vendor/mingw 工具链，批处理设置 PATH 与 `RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu`）。

### 1.4 验证

```bash
curl http://localhost:3000/health          # → ok
curl http://localhost:3000/openapi.json    # → OpenAPI JSON
```

## 2. 前端（桌面 Web）：开发与构建

> 在 **workspace 根 `oa4rust/`** 执行（不是 backend/）。

```bash
pnpm install                       # 首次安装全部 workspace 依赖

pnpm dev                           # Vite 开发服（desktop，:5173，代理 /api → :3000）
pnpm build                         # 构建桌面端 → frontend/dist/web
pnpm typecheck                     # 全 workspace tsc --noEmit
pnpm lint / pnpm lint:check        # biome 检查（--error-on-warnings）
pnpm test                          # vitest：desktop + packages + 三端契约守卫
```

**后端托管前端产物**：`pnpm build` 后直接访问 `http://localhost:3000`（Rust 静态托管 + SPA 回退）。开发期推荐 Vite 代理模式（热更新），此时 `APP_PUBLIC_ORIGIN=http://localhost:5173`。

## 3. 移动端（uni-app）

```bash
# workspace 根 oa4rust/
pnpm --filter @oa4rust/mobile dev:h5          # H5 开发服（:5174，代理 /api → :3000）
pnpm --filter @oa4rust/mobile build:h5        # H5 构建
pnpm --filter @oa4rust/mobile dev:mp-weixin   # 微信小程序（需配置 appid；产物导入微信开发者工具）
pnpm --filter @oa4rust/mobile build:app       # 原生 App（HBuilderX / 云打包）
pnpm --filter @oa4rust/mobile typecheck       # tsc（SDK 经 tsconfig paths 走源码）
pnpm test:mobile                              # 移动端单测
```

产物输出 `mobile/unpackage/dist/`。鉴权差异：H5 用 Cookie 即可；原生/小程序需 `VITE_OA_API_BASE` 指向后端绝对地址（详见 `mobile/README.md`）。

## 4. 质量门禁（提交前全量）

### 后端

```bash
cd oa4rust/backend
cargo check --workspace --locked        # 编译检查（锁版本）
cargo test --workspace --lib --locked   # 单测（无 DB 也应通过；parity 族需本地库）
# parity 全量（需本地 PG 且已完成迁移，约 379 表）：
cargo test -p parity --lib --locked     # 基准：4143/4143
```

### 前端

```bash
# workspace 根 oa4rust/
pnpm test && pnpm typecheck && pnpm lint:check && pnpm build
```

### E2E（需 live 后端）

```bash
pnpm test:e2e:list     # 列出用例
pnpm test:e2e          # 运行（app 覆盖 / 设计器往返 / workflow 运行时 / live 全流程）
```

### 供应链

```bash
cd oa4rust/backend
cargo audit                       # 对 security/rustsec-db.rev 固定的 advisory-db 全量比对
# 例外管理见 security/rustsec-exceptions.toml（带到期日）
```

## 5. 部署形态（生产）

1. `cargo build --release` 产出单二进制；`pnpm build` 产出 `frontend/dist/web`。
2. 同机部署时二进制相对路径 `../../../frontend/dist/web` 可直接命中；否则设 `OA4RUST_WEB_DIST` 绝对路径。
3. 生产环境变量硬性要求：`RUST_ENV=production`、`APP_PUBLIC_ORIGIN=https://<公开域名>`、`AUTH_COOKIE_SECURE=true`、Redis 可达。
4. nginx 反代模板：`backend/deploy/nginx.conf`；迁移期灰度按 `gray-release-playbook.md` 分模块切流（影子 → 10% → 50% → 90% → 100%）。
5. 数据库变更走内嵌迁移（启动自动）或预执行 `--migrate-only`。

## 6. 常用启动组合速查

| 场景 | 命令 |
| --- | --- |
| 纯后端开发 | `cargo run`（:3000，托管已构建的前端产物） |
| 前后端联调 | 终端 1 `cargo run`；终端 2 `pnpm dev`（:5173 代理）+ `.env` 的 origin 指向 5173 |
| 带 MCP | `cargo run -- --http` |
| 只跑迁移 | `cargo run -- --migrate-only` |
| 移动端 H5 联调 | 终端 1 `cargo run`；终端 2 `pnpm --filter @oa4rust/mobile dev:h5` |
