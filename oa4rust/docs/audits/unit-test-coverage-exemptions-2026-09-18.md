# 单测覆盖率审计与豁免声明（2026-09-18）

目标：oa4rust 三端（Rust 后端 / 桌面 web / mobile）单测「尽可能覆盖」。
本文记录：①本轮补测清单；②逐 crate 纯逻辑复核结论；③显式豁免及理由。

## 1. 基线与本轮增量

| 端 | 补测前 | 补测后 | 说明 |
|---|---|---|---|
| 桌面 vitest | 901 | 950（37 文件） | sdk app/toast/session/api/websocket/widget + contracts designer/xform(+jsdom DOMParser 分支)/process-definition 可执行分支 |
| mobile vitest | 66 | 88（5 文件） | form.ts 全分支 + services 请求体/URL + http.ts 参数过滤/upload refresh 失败 |
| Rust lib 测试 | 7,612 | 7,620 | orm 0→2、signature 5→9、query_service 4→6 |

桌面语句覆盖 85.54% → 93.86%+（全量 All files，含 950 例后的重测）；
mobile 语句覆盖（排除结构豁免的 main.ts 后）≈ 90.7%。

## 2. Rust：26 个低测试 crate 逐个复核方法

方法：对每个 crate 的 `crates/<name>/src/*.rs` grep 全部函数签名，筛选
「无 `pool: Extension<Pool>` / 无 `Extension` 提取器 / 非 handler」的纯函数。
复核清单（lib 测试 <8 或 0 的 26 crate）：

**找到真实纯逻辑并已补测（本仓库内可无 DB 单测的函数）：**
- `orm`（0→2）：`count_active`/`cursor_list` 文档化占位契约（不触库、哨兵值），
  用 `DatabaseConnection::Disconnected` 做守卫防未来误实现。
- `signature`（5→9）：证书 subject 提取、有效期校验、PEM 链块解析、单自签链。
- `query_service`（4→6）：`validate_query` SQL 注入关键词闸（大小写/子串/中文放行）。

**复核后确认为 DB 强依赖、本轮豁免（不垫数）：**
剩余 23 个 crate（ai_core_entity、bbs、calendar、calendar_core_entity、cms_control、
cms_core_express、cms_express、component_core_entity、correlation、
correlation_core_express、empower、general、general_assemble_control、hotpic、
message_core_entity、mind、openapi、organization_core_express、process_express、
process_surface、processplatform_core_express、search、sms*）的全部 `pub fn`
均为 `async fn ... pool: Extension<Pool>` 形式的 handler 或 derive 生成实体
（`_core_entity` 族的 `DeriveEntityModel` 无手工逻辑）：

- 路由可达性已由各 crate `tests_generated.rs` 覆盖（生成器保证每条注册路由
  有 404/500 探针测试）；
- handler 内部逻辑（SQL、参数校验分支）需 live DB 才有效，沿用仓库既有
  `db_available` 守卫模式（见 shared/crud.rs 的 off-by-one 回归测试）；
- `sms` 的纯逻辑（`is_valid_phone`/Mock gateway/限流）已有 6 个 lib 测试覆盖；
- `openapi` 为 utoipa 生成路由（`health_check` 等 stub），无业务逻辑。

豁免依据：对 DB handler 用 mock 垫覆盖率会产生「测试不能因业务变化而失败」
的伪覆盖（AGENTS.md Rule 9），故显式不补；有效覆盖待 live-DB 集成门禁。

## 3. 前端：显式豁免（配置内记录）

- `oa4rust-web/packages/apis/src/index.ts`：生成的一行 HTTP 包装器（62 模块、
  ~4000 路由），无可测分支；`api-coverage-generated.test.ts` 已实跑每个方法
  触发 V8 覆盖（97% stmts）。余 ~3% 为生成器模板纯转发行，结构不可测。
  vitest floor 90% = 豁免线。
- `oa4rust-web/apps/mobile/src/main.ts`：uni-app 入口，模块顶层 createApp/挂载，
  node 环境不可执行；行为由 H5 E2E 实跑覆盖（记忆 oa4rust-mobile-gui-verified）。
  mobile vitest coverage 显式 exclude。
- `oa4rust-web/packages/sdk/src/session.ts` 1 行（`clearLegacyStorage` 的
  `globalThis[name]` catch-continue）：触发条件为 globalThis 访问抛错，
  node/jsdom 下均不可构造，豁免。
- `contracts/**` 与 `utils/**` 的 70/80% floor 锁住本轮实测覆盖（见
  oa4rust-web/vitest.config.ts 阈值注释）。

## 4. 门禁终态

- vitest 桌面 / mobile 全绿；`pnpm vitest run --coverage` 阈值全过；
- biome 0 error；tsc 6 工程；desktop vite build；
- cargo test（orm/signature/query_service lib）+ fmt 全绿。
