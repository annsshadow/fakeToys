# 单测覆盖率审计与豁免声明（2026-09-18）

目标：oa4rust 三端（Rust 后端 / 桌面 web / mobile）单测「尽可能覆盖」。
本文记录：①三轮补测增量；②逐 crate 纯逻辑复核结论；③前端显式豁免及理由；
④「剩余未覆盖语句」逐文件闭环清单（每项 = 已测 或 已豁免+理由）。

## 1. 基线与增量（三轮后终态）

| 端 | 补测前 | 补测后 | 说明 |
|---|---|---|---|
| 桌面 vitest | 901 | **953（38 文件）** | 三轮：sdk app/toast/session/api/websocket/widget + contracts 三文件可执行分支 + apis 尾段转发器形状测试 |
| mobile vitest | 66 | **102（6 文件）** | 三轮：form.ts 全分支 + services 请求体/URL + http.ts 尾部分支 + services 全方法表驱动形状测试 |
| Rust lib 测试 | 7,612 | 7,620 | orm 0→2、signature 5→9、query_service 4→6 |

语句覆盖（全量 `vitest run --coverage` 实测，三轮后）：
- 桌面 **99.92%** stmts / **100%** functions（85.54% 起步）；
- mobile **99.66%** stmts / **100%** functions（排除结构豁免的 main.ts 后，86% 起步）。

## 2. Rust：26 个低测试 crate 逐个复核方法

方法：对每个 crate 的 `crates/<name>/src/*.rs` grep 全部函数签名，筛选
「无 `pool: Extension<Pool>` / 无 `Extension` 提取器 / 非 handler」的纯函数。

**找到真实纯逻辑并已补测（本仓库内可无 DB 单测的函数）：**
- `orm`（0→2）：`count_active`/`cursor_list` 文档化占位契约（不触库、哨兵值），
  用 `DatabaseConnection::Disconnected` 做守卫防未来误实现。
- `signature`（5→9）：证书 subject 提取、有效期校验、PEM 链块解析、单自签链。
- `query_service`（4→6）：`validate_query` SQL 注入关键词闸（大小写/子串/中文放行）。
- `sms`（已有 6 例）：`is_valid_phone`/Mock gateway/限流已被既有 lib 测试覆盖。
- `preview`（已有 4 例）：`detect_mime_type`/`detect_target_format` 已被既有测试覆盖。

**复核后确认为 DB 强依赖、本轮豁免（不垫数）：**
其余 21 个 crate（ai_core_entity、bbs、calendar、calendar_core_entity、cms_control、
cms_core_express、cms_express、component_core_entity、correlation、
correlation_core_express、empower、general、general_assemble_control、hotpic、
message_core_entity、mind、openapi、organization_core_express、process_express、
process_surface、processplatform_core_express、search 中的剩余部分）的全部
`pub fn` 均为 `async fn ... pool: Extension<Pool>` 形式的 handler 或
`DeriveEntityModel` 生成实体（无手工逻辑）：
- 路由可达性已由各 crate `tests_generated.rs` 覆盖（生成器保证每条注册路由有探针测试）；
- handler 内部逻辑（SQL、参数校验分支）需 live DB 才有效，沿用仓库既有
  `db_available` 守卫模式（见 shared/crud.rs 的 off-by-one 回归测试）；
- `openapi` 为 utoipa 生成路由 stub，无业务逻辑。

豁免依据：对 DB handler 用 mock 垫覆盖率会产生「测试不能因业务变化而失败」
的伪覆盖（AGENTS.md Rule 9），故显式不补；有效覆盖待 live-DB 集成门禁。

## 3. 前端：显式豁免（配置内记录）

- `oa4rust-web/packages/apis/src/index.ts`：生成的一行 HTTP 包装器（62 模块、
  ~4000 路由），无可测分支；`api-coverage-generated.test.ts` 实跑大部分方法，
  尾段（editor/uuid/export/import/appConfig/processplatformSurface）由
  `api-forwarders-shape.test.ts` 钉死。vitest floor 90% 即豁免线。
- `oa4rust-web/apps/mobile/src/main.ts`：uni-app 入口，模块顶层 createApp/挂载，
  node 环境不可执行；行为由 H5 E2E 实跑覆盖（记忆 oa4rust-mobile-gui-verified）。
  mobile vitest coverage 显式 exclude。
- `contracts/**` 与 `utils/**`/`sdk/**` 阈值已上调（sdk 15→80、contracts 0→70、
  utils 0→80，见 oa4rust-web/vitest.config.ts 阈值注释）锁住实测覆盖防回退。

## 4. 剩余未覆盖语句逐文件清单（闭环：每项 = 已测 或 已豁免+理由）

**桌面端**（三轮后 `coverage/coverage-final.json` 实测：仅剩 1 行未覆盖）

| 文件 :: 行 | 状态 | 理由 |
|---|---|---|
| `packages/sdk/src/session.ts :: 18` | **已豁免** | `clearLegacyStorage` 的 `catch { continue }`：node/jsdom 下 `globalThis['localStorage'\|'sessionStorage']` 属性访问永不抛错，触发条件不可构造；纯防御分支。 |
| `packages/apis/src/index.ts :: 783,803,830,831,843,922-936` | **已测**（三轮） | `api-forwarders-shape.test.ts` 钉死 5 个单方法模块 + processplatformSurfaceApi 全 15 方法的 verb+path+body。该行段现 0 未覆盖。 |
| `apps/desktop/src/utils/toast.ts :: 144` | **已测**（三轮） | 「confirmMsg 二次调用先移除仍打开的旧 overlay」钉死重复打开保护。现 0 未覆盖。 |
| `packages/sdk/src/websocket.ts`、`contracts/*.ts`、`utils/sandbox.ts`、其余 sdk 文件 | **已测**（一/二轮） | 最近一次 coverage 实测 0 未覆盖语句。 |

**移动端**（三轮后 `coverage-mobile/coverage-final.json` 实测：仅剩 1 行未覆盖）

| 文件 :: 行 | 状态 | 理由 |
|---|---|---|
| `src/utils/form.ts :: 242` | **已豁免（死分支）** | `if (g.id === '' && !defaultHasField) continue` 不可达：`''` 组只可能经「无容器字段」进入 groups，此时 `defaultHasField` 必为 true，条件恒假。 |
| `src/services/index.ts :: 22,30-34,72,75,82,105,108,110,113,116,153,155,169,175,199,208,213,236,239,261,263,282,289` | **已测**（三轮） | `shape.test.ts` 表驱动：mock `./http` 后逐方法钉死 verb+path+options（~30 个一行转发器）。与 URL 契约守卫互补（本层防动词写错/请求体漏字段）。该行段现 0 未覆盖。 |
| `src/main.ts`（整文件 0%） | **已豁免（结构）** | uni-app 入口 node 不可执行；E2E 实跑覆盖；mobile coverage 已显式 exclude。 |
| `src/services/http.ts`、`src/store/session.ts`、`src/utils/auth-guard.ts` | **已测**（一/二轮） | 最近一次 coverage 实测 0 未覆盖语句。 |

**Rust 端**：无新增未覆盖；26 低测试 crate 判定见 §2（5 个纯逻辑已测，
21 个 DB 强依赖显式豁免，路由可达性由 tests_generated 探针覆盖）。

**结论**：三端「剩余未覆盖语句」= 桌面 1 行（不可构造 catch）+ mobile 1 行（死分支）
+ mobile main.ts（结构入口），全部显式豁免且理由在案；其余每一行均有测试执行。

## 5. 门禁终态（三轮后实测）

- vitest 桌面 **953/953**、mobile **102/102**；`pnpm vitest run --coverage`
  全阈值通过（exit 0，桌面 stmts 99.92%）；
- biome 0 error；tsc 6 工程全绿；desktop vite build 通过；
- cargo test（orm/signature/query_service lib）+ fmt 全绿。
