# 08 · 常见问题与注意事项

## 1. 编译 / 启动期

### Q1：`create_app` 阶段栈溢出（Stack overflow）

**根因**：`create_app` 将约 80 个 crate 的路由 `.merge()` 为巨型 axum Router，构造需约 **64MB 栈**（已确认是栈大小需求，非无限递归）。
**对策**：Windows-MSVC 已由 `backend/.cargo/config.toml` 的 `/STACK:67108864` 解决（仅主线程）。若自行改启动结构（如把 `create_app` 挪到 worker 线程），必须自行提供大栈线程——参考 `lib.rs` 中 `cors_guard` 测试的 64MB 显式栈写法（Linux/CI 上链接参数不生效，测试必须用显式线程）。

### Q2：Windows GNU 目标链接失败 / `0xC0000139`（STATUS_ENTRYPOINT_NOT_FOUND）

**根因**：GNU 目标 proc-macro 原生链接需要 dlltool/objdump/as/ld/ar；或 mingw 运行时 DLL 版本不匹配（混用 rustup 旧工具链 DLL）。
**对策**：按 `oa4rust/vendor/README.md` 重建工具链；DLL 用 Git for Windows 自带（MSYS2 代际）版本；调用必须走原生 cmd 批处理（Git Bash 会篡改 MSYS 风格 PATH）。

### Q3：启动时报 "invalid browser authentication configuration"

**根因**：`RUST_ENV=production` 下 `APP_PUBLIC_ORIGIN` / `AUTH_COOKIE_SECURE` 未正确配置。
**对策**：生产设 `APP_PUBLIC_ORIGIN=https://<域名>`、`AUTH_COOKIE_SECURE=true`；本地开发用 `RUST_ENV=development` + 默认值。

## 2. 认证 / 会话

### Q4：登录后任何**写请求**都 403（GET 正常）

**根因**：CSRF 层按 `APP_PUBLIC_ORIGIN` 校验请求来源——环境变量与浏览器实际访问 origin 不一致（典型：`.env` 留默认 `http://localhost:3000`，但前端走 `:5173` 的 Vite 代理）。
**对策**：Vite 代理开发时设 `APP_PUBLIC_ORIGIN=http://localhost:5173`；生产设为公开域名（`nginx` 场景）。`backend/.env.example` 注释有完整说明。

### Q5：启动日志出现 "Redis unreachable; session store and rate limiter using in-memory fallback"

**定性**：这不是错误——**降级是设计行为**（Phase B-U-B2）。单机开发可无视；生产应修复 Redis 连接（`REDIS_URL`），否则会话不跨实例、限流退化为单机。

### Q6：旧数据密码登录失败

**根因**：存量 o2server 数据的 MD5/DES 哈希默认不被接受。
**对策**：迁移期设 `AUTH_LEGACY_COMPAT=true`（启动会打 warn 日志留痕）；**数据迁移完成后必须关闭**（安全审计明确要求）。

### Q7：移动端（原生/小程序）Cookie 不生效

**根因**：HttpOnly Cookie 在原生容器/微信小程序中不会自动跨域携带（`mobile/README.md` 鉴权模型节）。
**对策**：H5 用同源部署或 dev 代理；原生/小程序目前需 `setApiBase` 指向后端绝对地址，并等待后端配套 token 鉴权通道后注入 `Authorization` 头（现状已知限制，非 bug）。

## 3. 数据库 / 迁移

### Q8：新增了迁移文件但没生效

**检查序**：① 文件名必须形如 `NNN_description.sql`（三位编号前缀）；② `*_rollback.sql` 与 `archive/` 目录**会被忽略**；③ 迁移在**启动时**执行——改完要重启进程；④ 看启动日志 `migrations: N applied, M already applied`。

### Q9：想回滚某次迁移

启动逻辑**不做自动回滚**。手工执行对应的 `NNN_description_rollback.sql`（所有历史迁移均成对提供）。生产回滚属高危操作，先在测试环境演练。

### Q10：MySQL 兼容报 SQL 语法错

**定性**：PG 是主库，MySQL 走 `SqlDialect` 最小抽象 + `rewrite_pg_to_mysql` 重写，只覆盖"当前查询实际用到的差异点"（占位符/类型转换/引号/ILIKE 等）。新写 SQL 若用了未抽象的 PG 特有语法，MySQL 兼容属预期外情况——回到 `db/dialect.rs` 扩抽象，不要在业务代码里写方言分支。

## 4. 前端 / 三端

### Q11：`pnpm test` 中 `tests/contracts` 失败

**定性**：三端契约守卫命中——desktop/mobile 端点与后端注册路由不对账（新增/删除/改名）。
**对策**：按 [04-flows.md](04-flows.md) 第 9 节流程，同步更新 fixture（`backend-registered-routes.fixture.ts` 等）与对应端的前端定义。**这是强制机制，不允许跳过或豁免单条**。

### Q12：`pnpm install` 后构建报 uni-app 相关告警 / 版本差异

**定性**：`pnpm-workspace.yaml` 用 overrides 抬版本（安全）+ `minimumReleaseAgeExclude` 白名单 uni-app alpha 包，属预期配置，勿随意删除 override（会放回已知漏洞版本）。

### Q13：直接访问 `:3000` 的未知 `/api/xxx` 返回 404 JSON 而不是首页

**定性**：设计行为——`/api/**`、`/ws/**` 未知路径保持 404（API 客户端不应收到 HTML）；其余路径 SPA 回退 `index.html`。

## 5. 安全与合规

### Q14：`cargo audit` 命中但不想升版本

**对策**：确认可达性后（是否进入运行时构建图、是否仅 dev-dependency），在 `security/rustsec-exceptions.toml` 登记例外并**必须带到期日**；同时抬 `security/rustsec-db.rev` 固定版本保证审计可复现。流程见 `backend/docs/security/rustsec-audit.md` 与最新审计报告（`docs/audits/2026-10-14-security-audit.md`）。

### Q15：为何部分响应 `spent` 恒为 0、字段像"默认值"

**定性**：信封对齐 o2server v9 Gson 的刻意决策（`response.rs` 详注）——`None` 跳过序列化、元数据恒填，保证 O2OA 生态存量前端无感接入。精确耗时尚未实现（保持信封行为一致），勿在 handler 里自行造元数据。

### Q16：`AUTH_EXEMPT_PATHS` 能否随便加路径

**不能**。豁免路径 = 无会话即可访问，属安全敏感面。新增必须过安全审计（管理写族另有 `ADMIN_WRITE_PREFIXES` 门禁，越权收口模式见 `docs/audits/` 的管理写族越权门禁档案）。

## 6. 测试

### Q17：本地没起数据库，`cargo test` 要不要全绿

**基线**：`cargo test --workspace --lib --locked` 无 DB 应通过（4563+ 用例）。需要 DB 的 parity 族（4143 用例）与集成测试需本地 PG 并完成迁移（`--migrate-only`，约 379 表）。

### Q18：E2E 起不来

**前提**：Playwright 需要 live 后端（`:3000`）且前端已构建/代理可达；先用 `pnpm test:e2e:list` 确认用例装载。全局初始化见 `e2e/global-setup.ts`。

## 7. 高频误判提醒

| 症状 | 常见误判 | 实际 |
| --- | --- | --- |
| 端口 81/8001 混淆 | — | 与本项目无关（那是 BMP 门禁链路的坑）；oa4rust 固定 **:3000**，o2server 基准容器（开发期 parity 用）docker :18080 |
| `migrations: 0 applied, 108 skipped` | 以为迁移坏了 | 幂等：已应用即跳过，属正常 |
| `/openapi.json` 打不开 Swagger UI | 以为有网页 | 当前仅暴露 JSON 端点（utoipa ApiDoc）；Swagger UI zip 已在 backend 目录备用但未挂载 |
| 移动端接口 401 | 以为 token 通道坏了 | 现状 Cookie 模型，原生/小程序通道未补齐（见 Q7） |
