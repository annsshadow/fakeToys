# oa4rust × laiyipao 1000 轮实质优化路线图

> 状态：进行中（2026-11-02 启动）
> 口径：只有「改变运行时行为/新增能力」或「可测量的性能提升」且拿到证据才算一轮。
> 编译通过、lint/warning、单纯补测试、纯文档/注释、升级依赖、重命名/格式化、删死代码
> 都不占轮次编号，只能作为某轮的前置动作。
> 断点续跑：直接看 §4 台账，`done` 行有证据，`pending` 行按批次顺序继续。

## 1. 环境复用手册（探测于 2026-11-02）

不再每轮重新摸索，按下面入口直接跑。

| 项 | 值 / 入口 |
| --- | --- |
| Docker | Docker Desktop 29.9.2，`docker info` 可用 |
| PostgreSQL | `postgres:17` 容器，实测 `PostgreSQL 17.11`，映射 `0.0.0.0:5432->5432` |
| 认证 | `POSTGRES_HOST_AUTH_METHOD=trust`，用户 `postgres`（口令占位，不走网络明文） |
| 已有库 | `laiyipao`、`oa4rust`、`oa4rust_cov`、`oa4rust_test`、`oa4rust_test_iso_0..3`、`oa4rust_idx_verify` |
| 规模 | `oa4rust` 385 张表；`laiyipao` 51 张表（均已应用迁移） |
| laiyipao DSN | `TEST_DATABASE_URL` 优先，其次 `DATABASE_URL`；测试默认 `postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable` |
| oa4rust DSN | `DATABASE_URL=postgres://o2server:<password>@localhost:5432/oa4rust_test`，`RUST_ENV=test`（trust 下口令被忽略） |
| laiyipao 真库用例 | `go test ./internal/store -run TestOpen -count=1`；`go test ./internal/service -run TestAdminUserDetail -count=1` |
| laiyipao 一次性库 | `internal/testutil/scratchdb`：建库→迁移→种子→DROP，各包 `TestMain` 自动清理残留 |
| oa4rust 真库用例 | `cargo test -p shared --lib session::tests::pg_validate_session_falls_back_to_db_and_rehydrates -- --exact` |
| 工具链 | Go 1.26.10；cargo/rustc 1.99.0 |
| 门禁 | `node scripts/check-warnings.mjs`（批次结束跑一次，不计轮） |

约束：容器/数据卷只启动不重建；测试优先用一次性 scratch 库，避免污染开发库。

## 2. 审计结论（落到文件:行）

### P0（数据正确性 / 崩溃安全）

| # | 问题 | 位置 |
| --- | --- | --- |
| P0-1 | 10 处管理导出硬编码 `LIMIT 100000`，超限**静默截断**却返回成功 CSV | `laiyipao/server/internal/service/adminapi.go:605,767,908,1475,1609,1744`；`stats.go:589,1167`；`adminuserdetail.go:392`；`recompute_gate.go:762` |
| P0-2 | oa4rust 无优雅停机，SIGTERM 直接断连、在途写请求可能被腰斩 | `oa4rust/backend/src/main.rs`（`axum::serve(...).await?`） |
| P0-3 | 无全局请求超时层，慢 handler 可无限占用连接/池 | `oa4rust/backend/Cargo.toml`（tower-http 未启 `timeout`）+ `create_app` |

### P1（可靠性 / 资源上限）

| # | 问题 | 位置 |
| --- | --- | --- |
| P1-1 | `store.Open` 未设 `statement_timeout` / `MaxConnIdleTime` / `HealthCheckPeriod` | `laiyipao/server/internal/store/store.go` |
| P1-2 | deadpool 未显式 acquire/wait timeout，池耗尽时请求可能长时间挂起 | `oa4rust/backend/crates/shared/src/db/mod.rs` |
| P1-3 | 访问日志只有 5xx 的 warn，无耗时、请求 ID、结构化字段 | `oa4rust/backend/crates/shared/src/middleware/security.rs`（`trace_middleware`） |
| P1-4 | 导出全量 `bytes.Buffer` 物化，10 万行内存峰值高 | laiyipao 10 个导出函数 |
| P1-5 | `ControlPool::acquire` 把池错误统一映射为 `Internal`，丢失可分类性 | `oa4rust/backend/crates/shared/src/lib.rs` |

### P2（可维护性，可作轮次前置）

| # | 问题 | 位置 |
| --- | --- | --- |
| P2-1 | `rate_limit::record_failure` 单 key 无上限（生产未调用） | `oa4rust/backend/crates/shared/src/rate_limit.rs` |
| P2-2 | 生成 crate 的 `row.get::<_, Option<String>>(...).unwrap_or_default()` 把 NULL 与类型错误都变空串 | oa4rust 生成 crate（按模块逐轮核实） |

已确认**不需要动**（避免重复劳动）：SettleBattle 事务与 token 条件抢占、ClaimTask 并发、
grantWallet/grantToken 负余额可读错误、AdminUserDetail 并发、ttlCache singleflight、
Wechat 8s 超时 + 1MB 响应上限、Fiber 读写超时与 body limit、laiyipao 优雅停机、SIGHUP 热加载。

## 3. 批次主题（每批 50 轮，共 20 批）

| 批次 | 轮次 | 主题 | 主要证据形态 |
| --- | --- | --- | --- |
| B1 | 1–50 | 数据正确性与查询/导出边界 | 真库失败路径用例、CSV 行数断言 |
| B2 | 51–100 | 事务与一致性 | 事务回滚用例、并发用例 |
| B3 | 101–150 | 并发与幂等 | `-race` 并发用例、条件更新行数断言 |
| B4 | 151–200 | 超时/重试/背压/资源上限 | `pg_sleep` 超时用例、池耗尽用例 |
| B5 | 201–250 | 可观测性 | 日志字段断言、`/metrics` 前后对比 |
| B6 | 251–300 | 优雅停机与生命周期 | 信号用例、在途请求完成断言 |
| B7 | 301–350 | oa4rust 请求层可靠性 | 超时/池/限流失败路径用例 |
| B8 | 351–400 | oa4rust 数据访问与 N+1 | 查询次数断言、基准数字 |
| B9 | 401–450 | 安全与权限 | 越权用例、注入/穿越用例 |
| B10 | 451–500 | 契约与兼容性 | OpenAPI 快照、错误码断言 |
| B11 | 501–550 | 迁移与数据完整性 | 迁移幂等用例、约束用例 |
| B12 | 551–600 | 性能热点：laiyipao | 改前/改后 p50/p95、分配/内存 |
| B13 | 601–650 | 性能热点：oa4rust | 基准组数字、bundle/首屏 |
| B14 | 651–700 | 错误分类与恢复 | 错误码矩阵用例 |
| B15 | 701–750 | 配置校验与热加载 | 非法配置启动/重载用例 |
| B16 | 751–800 | 缓存正确性 | 命中/失效/竞态用例 |
| B17 | 801–850 | 运维与迁移 | 健康检查、备份演练脚本 |
| B18 | 851–900 | 故障恢复 | 断连/慢查询/重启恢复用例 |
| B19 | 901–950 | 管理端交互可靠性 | E2E 用例 |
| B20 | 951–1000 | 收口与回归 | 全量门禁 + 基准复测 |

### 每批轮次槽位（执行时按此细化，不改变编号）

> 编号以 §4 台账为准：实际执行时按「先闭环可验证项」调整顺序，下列槽位是候选池，
> 具体轮号由台账在开轮时认领。R3 已用于 statement_timeout、R4 用于优雅停机（原属 B4/B6），
> 因此 B1 的剩余候选（分页夹取、total 口径、关键词上限、CSV 注入、空切片、时间范围反转、
> 导出内存上限、取消传播）从 R5 起继续认领。

- **B1**：导出截断 fail-loud（R1 done）；候选池：分页参数夹取、列表 total 口径、
  后台关键词长度上限、导出 CSV 注入防护、空结果切片归一、时间范围反转校验、
  导出内存上限、导出取消传播；及同主题其他查询（逐条落到文件:行）。
- **B2**：结算、领奖、钱包、道具、订单、兑换、公告发布的事务边界与回滚证据（R51–R100）。
- **B3**：token 抢占、领取并发、限流并发、缓存回填并发、管理端并发写（R101–R150）。
- **B4**：PG statement_timeout、池空闲/健康检查、body 上限、导出上限、重试预算（R151–R200）。
- **B5**：访问日志、请求 ID、慢请求、指标、错误分类计数（R201–R250）。
- **B6**：oa4rust graceful shutdown、后台任务取消、laiyipao 关闭顺序（R251–R300）。
- **B7**：请求超时层、acquire 超时、CORS/安全头、限流粒度（R301–R350）。
- **B8**：生成 crate 的 N+1、分页、软删过滤、索引命中（R351–R400）。
- **B9**：admin 越权、CSV 注入、路径穿越、XFF 信任、CSP（R401–R450）。
- **B10**：OpenAPI 契约、错误码、字段兼容、版本化（R451–R500）。
- **B11**：迁移幂等、约束补齐、回填、可空性收紧（R501–R550）。
- **B12**：看板/榜单/导出/引擎性能（R551–R600）。
- **B13**：SPA/OpenAPI/session/路由性能（R601–R650）。
- **B14**：错误分类、降级、重试分类（R651–R700）。
- **B15**：env 校验、热加载一致性、配置漂移（R701–R750）。
- **B16**：缓存 TTL、失效、负缓存、键归一（R751–R800）。
- **B17**：健康检查、备份演练、巡检脚本（R801–R850）。
- **B18**：DB 断连、慢查询、磁盘故障、重启恢复（R851–R900）。
- **B19**：管理端 E2E 与操作反馈（R901–R950）。
- **B20**：全量门禁、基准复测、遗留收口（R951–R1000）。

## 4. 逐轮台账

> `done` 必须带证据；`blocked` 必须写缺口；`pending` 可直接续跑。

| 轮号 | 标题 | 状态 | 证据摘要 |
| --- | --- | --- | --- |
| R1 | laiyipao 管理导出超限 fail-loud（10 处）+ 413 映射 + 边界用例 | done | 10 处 `LIMIT 100000`→`100001` 探测；超过上限返回 `ErrExportTooLarge`（可 `errors.Is`）；真库：恰好 10 万行成功且 CSV 行数=100001（表头+数据），100001 行被拒；单用户流水导出同样被拒；HTTP 映射 413 `export_too_large` |
| R2 | （并入 R1：边界用例即 R1 的验收证据） | done | — |
| R3 | laiyipao 连接池：statement_timeout 上界 + 空闲回收可配置 | done | 默认 30s `statement_timeout` 随握手下发（RuntimeParams）；`DB_CONN_IDLE_TIME` 接 `MaxConnIdleTime`。真库证据：`SHOW statement_timeout` = 2s；`pg_sleep(3)` 在 2.08s 被取消（57014，耗时 <2.5s）；设为 0 时会话为 `0`（无限制） |
| R4 | oa4rust axum 优雅停机（SIGINT/SIGTERM → 停止接新连接、排队在途、宽限期强制退出） | done | 新增 `shared::graceful`：`serve_router_with_shutdown(listener, app, grace, signal)` 组合 `axum::serve(...).with_graceful_shutdown()` 与 `tokio::time::timeout`；`main.rs` 改用该入口，宽限 10s。用例：`drains_in_flight_request`——handler 进入后触发 shutdown，200ms 在途请求仍返回 `200 OK` 且 body=`done`；`forced_exit_after_grace_period`——5s 卡死请求在 100ms 宽限期后返回（实测 ≥90ms 且 <1s），证明不会无限挂住重启。`cargo clippy -p shared --tests -D warnings` 与 `cargo check --bin oa4rust` 通过 |
| R5 | oa4rust 全局请求超时层（默认 30s，超时 408） | done | 新增 `shared::middleware::timeout`：`REQUEST_TIMEOUT_SECONDS`（未设=30s，`0`=关闭，非法=启动报错），在 `create_app` 路由层内层套 `TimeoutLayer::with_status_code(408)`。用例：500ms handler + 50ms 上界 → 实测 <300ms 返回 408；10ms handler 正常 200；**流式 body 不被截断**（响应头 50ms 内返回、body 200ms 后才吐完仍完整）；`None` 关闭后慢 handler 跑完；`parse_request_timeout` 覆盖默认/`0`/`" 5 "`/`"5s"`/`-1`。`cargo clippy -p shared --tests -D warnings`、`cargo check --bin oa4rust` 通过 |
| R6 | oa4rust 连接池：池满等待与建连超时可配置 + 池超时映射 503 | done | `DbPoolConfig` 新增 `wait_timeout`/`create_timeout`（`DB_POOL_WAIT_TIMEOUT_SECONDS`/`DB_POOL_CREATE_TIMEOUT_SECONDS`，默认各 10s，`0`=关闭，非法回退默认），经 `Pool::builder().timeouts(Timeouts{..})` 落到 deadpool（配 timeouts 需显式 `Runtime::Tokio1`）。`AppError::Unavailable` → 503，`ControlPool::acquire` 把 `PoolError::Timeout` 映射到它。真库证据：max_size=1 占满后第二次 `get()` 在 wait_timeout=1s 后返回 `Timeout`（总耗时 2.01s 含下一用例）；`create_timeout=1s` 对「只接受 TCP 不回应握手」的本地黑洞：`get()` 返回 `Timeout` 且 <4s；`shared` 全量 343 用例通过、clippy 零告警、`cargo check --bin oa4rust` 通过 |
| R7 | oa4rust 结构化访问日志 + 请求 ID（生成/透传/防注入/回写） | done | `trace_middleware` 改为每个请求一条结构化日志（method/uri/status/latency_ms/request_id），5xx 仍 warn、其余 info；`x-request-id` 上游合法值透传、缺失生成 UUID v4、非 `[A-Za-z0-9._-]` 或 >64 字节拒绝并重新生成；响应回写同值。用例：缺省→响应头为合法 UUID；合法上游 id 原样回显；恶意 id（引号/空格）与 65 字节被拒；`set_default` 捕获日志断言含 `request completed`/`status=200`/`latency_ms=`/与响应头一致的 request_id。`shared` 347 用例通过、clippy 零告警 |
| R8 | （并入 R1：413 映射与文案是同一次改动） | done | — |
| R9 | laiyipao 导出 CSV 公式注入防护（10 处导出统一出口，负数保持数值） | done | 新增 `csvSafe`/`writeCSVRow`：`=`/`+`/`@`/TAB/CR 前导一律加 `'`；`-` 开头且非合法数字才加（`-5`/`-1.50` 保持数值，`-1+1`/`-HYPERLINK()` 被中和）。10 处导出（`adminapi.go` 6、`adminuserdetail.go` 1、`recompute_gate.go` 1、`stats.go` 2）全部改为 `writeCSVRow`，service 包内已无裸 `w.Write(rec)`。用例：表驱动覆盖公式/数字/文本；真库审计日志 `target='=HYPERLINK(...)'` 导出后为 `'=HYPERLINK(`；真库钱包流水 `delta=-12` 导出保持 `-12` 未被误加前缀。`go vet ./internal/service` 干净、`go test ./internal/service` 全包 530s 通过 |
| R10 | laiyipao 后台时间范围语义：`date_to` 日期格式含当天整天 + 反转/空范围 400 | done | `queryTimeRange`（middleware.go）：成对解析，左端按日期 00:00、右端日期格式 +1 天（服务层 SQL 一律 `< $to` 半开区间），`from >= to` 返回 `ErrBadInput`。11 个处理器改为成对调用（users/export、battles/export、orders/export、defense-challenges/export、announcements、audit-logs/export；含 last_login 两处）。用例：单元 5 条——同日 `date_from=date_to=2026-11-02` 右端=11-03T00:00Z；RFC3339 右端不位移；反转与相等被拒且文案含「时间范围为空」；单边/空串按未给；非法格式报错。E2E：`/admin/battles?date_from=今天&date_to=今天` total≥1（修前整天被排除为 0）；4 条路径（battles、battles/export、users、users/export）反转均 400 `bad_input`。既有 9 条日期筛选 e2e 回归全绿；`go vet ./internal/httpapi` 干净 |
| R11 | oa4rust handler panic 收敛为 500 信封（不再断连） | done | 新增 `shared::panic_guard`：`CatchPanicLayer::custom` 把 panic 转成与 `AppError::Internal` 完全一致的 500 信封，并在 `create_app` 超时层**内层**接入。用例 3 条：panic 路由返回 500 且 `type=error` / `message="internal server error"` / `prompt=...ExceptionInternal`；panic 之后 `/ok` 仍 200 且 body=`ok`（服务不被带崩）；正常请求响应原样透传。`cargo check --bin oa4rust` 通过、`cargo clippy -p shared --tests -D warnings` 零告警、`shared` 全量 350 用例连续 3 次全绿（含把 `trace_middleware` 日志用例改成进程级 subscriber，消除并行偶发空日志） |
| R12–R50 | 见 §3 B1 槽位 | pending | — |
| R51–R1000 | 见 §3 各批槽位 | pending | — |
