# 全面优化第二轮 · 跨子项目功能与性能改进计划

- **登记日**：2026-10-24
- **范围**：fakeToys monorepo 全子项目（oa4rust / laiyipao / augmentor / auto-checkin / scripts / CI / docs）
- **目标**：在 2026-10-23 第一轮（见 `2026-10-23-cross-subproject-optimization-backlog.md` §7 台账）之上，修复新发现的**真实缺陷**（含 2 个生产环境 404 的功能断层、1 个 CI 全仓 clippy 失败、1 个口令模式下面板完全不可用的 bug），落地已验证的低风险性能改进，并把工程卫生（CI 门禁、死脚本、文档漂移）收口。
- **方法**：四路并行只读勘察（oa4rust / laiyipao / augmentor / 其余）+ 关键证据一手复核。本文件为**执行计划**：每项含证据、改法、风险、验收判据；§9 为延期台账（含不做理由）。

---

## 0. 与既有计划的关系

| 既有计划 / 台账 | 本文档承接 |
|---|---|
| `2026-10-23-cross-subproject-optimization-backlog.md` | W2 遗留可验证项（LY-5a 竞态、LY-8）、W2 延期项复审（AU-P1/P2 有结论）、§7 台账抽查 |
| `2026-10-21` / `2026-10-23-laiyipao` 两轮 | 不重复；新发现 GAP-1（销账不符）登记延期 |
| `tech-debt.md` | TD-1~3 状态不变；本轮不新增技术债 |

## 1. 环境限制（先说清，避免把「没验」当成「验过」）

- **无本地活库**：本机 PG 15 仅装了客户端（无数据目录），`initdb` 被本机安全策略拦截；Docker Desktop 未运行（不主动启动 GUI）。**DB 依赖用例本机全部无法运行**——这是预期限制，非回归。处置：DB 触达类改动只做「编译/vet/纯逻辑单测 + SQL 人工复核」，并在台账标注「待 PG 验证」；非 DB 类改动全额验证。
- **cool 无 commons-codec jar**：Java 门禁本机跑不了（沿用前轮结论）。
- **验证基线**（本机实跑通过，作为回归参照）：augmentor `C:/Python314/python.exe -m pytest tests/unit/test_package_exports.py tests/unit/test_narrow_excepts_auq2.py` = 32 passed；auto-checkin `npm test` = 35 passed + typecheck 0 错。

## 2. 批次总览

| 批次 | 主题 | 项数 | 风险 | 提交策略 |
|---|---|---|---|---|
| B1 | 止损修复（P0 真实缺陷） | 4 | 低 | 1 提交 |
| B2 | oa4rust 功能复活（生产 404） | 4 | 低-中 | 1-2 提交 |
| B3 | laiyipao 竞态 + 性能 | 3 | 低-中 | 1 提交 |
| B4 | oa4rust 性能 | 4 | 低-中 | 1 提交 |
| B5 | augmentor 性能/可观测 | 4 | 低 | 1 提交 |
| B6 | CI 与工程卫生 | 6 | 低 | 1-2 提交 |

每批完成后跑该批门禁 → commit → push（fast-forward 直推 main，沿用前轮模式；推送后在台账记录 commit hash）。

---

## 3. B1 — 止损修复（P0）

### B1-1 oa4rust：clippy 全仓失败（10-23 轮引入）

- **证据**：`oa4rust/backend/crates/shared/src/db/mod.rs:36` 文档列表续行 `///                         仅 SeaORM 池生效…` 过度缩进 → `clippy::doc_overindented_list_items`。本机实跑 `cargo clippy -p shared`：`error: could not compile shared due to 1 previous error`。根 `Cargo.toml` `[workspace.lints.rust] warnings = "deny"` → 所有依赖 shared 的 crate clippy 全挂；CI `oa4rust-ci.yml` 的 clippy job 当前为红。由提交 `d140c7331`（10-23 连接池统一）引入。
- **改法**：续行缩进对齐为列表项规范缩进（4 空格）。
- **风险**：无（纯注释）。
- **验收**：`cargo clippy -p shared` 0 错误；修复后跑 `cargo clippy --workspace --all-targets`（若本机资源允许，否则标注 CI 验）确认无被挡住的存量告警。

### B1-2 auto-checkin：口令模式下面板完全不可用（AC-4 半截实现）

- **证据**：`auto-checkin/src/server.ts:70-79`，口令中间件只放行 `req.path === "/"` 与 `/assets*`；但实际静态文件在根路径（`/style.css`、`/app.js`，`public/` 下仅 3 个文件）与 `/screenshots/*`（失败截图）。设了 `PANEL_TOKEN` 后 `/` 返回 200 但 CSS/JS/截图全 401——**面板不可用**。现有 35 个测试只覆盖 `/api/stats`，未覆盖静态路径，故全绿漏过。
- **改法**：改为「只守 API」：非 `/api/` 前缀一律放行（静态资源由 `express.static` 只服务 `public/` 与 `screenshots/` 实际存在的文件，无敏感数据；截图为签到失败证据，面板本身就是 opt-in 口令保护）。代码注释写明取舍。
- **风险**：低。行为边界从「首页+assets」放宽到「全部非 API」——API 面（唯一敏感面）保护不变。
- **验收**：新增单测：带 token 时 `/style.css`、`/app.js`、`/screenshots/x.png` → 200；`/api/stats` 无 token → 401、带头 → 200；不存在的路径 → 404（static 落空，非 401，证明没有新暴露面）。`npm test` 全绿 + `tsc --noEmit` 0 错。

### B1-3 auto-checkin：`/api/records` 负 limit 静默返回错误数据集

- **证据**：`server.ts:104` `Math.min(Number(req.query.limit) || 100, 500)` 无下限；`?limit=-5` → `all.slice(5)` 悄悄丢掉前 5 条。同文件 `:93` 的 `/api/trend` 是双侧夹取的，同文件内不一致。
- **改法**：`Math.min(Math.max(Number(req.query.limit) || 100, 1), 500)` + 单测。
- **风险**：低。

### B1-4 scripts/seed_java.py：迁移路径断裂，CI 播种步静默失败

- **证据**：`scripts/seed_java.py:126,134` 读 `REPO_ROOT / "oa4rust/migrations/091_…"`，但迁移目录在 `oa4rust/backend/migrations/`（`063c5d4e4` 三端归一 git mv 时未同步本脚本）。`--process-form` 模式必抛 `FileNotFoundError`；`.github/workflows/oa4rust-ci.yml:361-367` 正以该模式调用，step 每次失败，被 job 级 `continue-on-error`（`:235`）掩盖。
- **改法**：路径补 `backend/`；加载函数加存在性断言（fail-loud，报清晰错误）。
- **风险**：低。**本机可验证**：loader 只读 SQL 文件、不连 DB，直接 import 调用复现/验证。

---

## 4. B2 — oa4rust 功能复活（生产 404，P0）

### B2-1 挂载 `organization_assemble_personal` router

- **证据**：`oa4rust/backend/src/lib.rs:82-173` 的 `.merge()` 链无此 crate；其 `crates/organization_assemble_personal/src/routes.rs:8` `pub fn router(pool: Pool)` 已实现 3 条路由（`/api/organization/assemble/personal/{id}/setting`、`/{id}/role/list`、`custom/{id}/mockputtopost`）。前端实际调用：`frontend/apps/desktop/src/views/OrgViewer.vue:341`（role/list）、`Personal.vue:659`（setting）——**当前生产全部 404**。OpenAPI 文档（`crates/openapi/src/lib.rs:30697`）宣传了这些路径（文档说有、运行时没有）。契约 fixture 从 crate 的 `.route()` 字面量生成，不校验 create_app 是否 merge，所以测试全绿（盲区，见 B2-4）。
- **改法**：create_app 补 `.merge(organization_assemble_personal::router(pool.clone()))`。
- **风险**：低（纯增量；3 条路径与已挂载路由无重叠——实施时用路由冲突测试复核）。

### B2-2 挂载 `empower` router（授权委托 CRUD 14 条）

- **证据**：`crates/empower/src/router.rs:23-59` 注册 14 条 `/api/person/empower[/{id}][/enable|/disable|/manager…]`；create_app 未 merge。`Personal.vue:482-531,593-615` 调用 empower 全族（create/detail/update/delete/manager/list 变体）——**生产 404**。已挂载的 personal crate 只覆盖 empower 的 list 家族（`personal/src/lib.rs:301-313` 手工转接 4 个 handler）。
- **改法**：create_app 补 `.merge(empower::router(pool.clone(), session_manager.clone()))`，**必须放在 `personal_extend::personal_extend_router(pool.clone(), session_manager)`（消耗 session_manager）之前**或提前 clone。
- **风险**：低-中。empower handler 自带 owner 校验（router.rs 注释「含 IDOR 防护」）；与 personal 已注册的 mockputtopost 变体路径不重叠（实施时跑路由冲突测试 + 安全门禁测试复核）。

### B2-3 挂载冒烟验证（无需 DB 的判别法）

- **改法**：起服务（deadpool 惰性建连，不需要真库）后 `curl` 两条路径：**挂载后返回 401（路由存在、缺会话），未挂载返回 404**——用状态码判别挂载是否生效，把「生产 404」变成本机可证伪。
- **验收**：`/api/organization/assemble/personal/1/setting` 与 `/api/person/empower`（POST）从 404 变 401/405 类响应。

### B2-4 merge 守卫测试（堵住「已注册 ≠ 已挂载」盲区）

- **证据**：`tests/contracts/backend-registered-routes.fixture.ts` 由 crate 的 `.route()` 字面量生成，不校验 create_app merge；B2-1/B2-2 正因该盲区漏网。
- **改法**：新增 Rust 集成测试：解析 `src/lib.rs` 的 `.merge(X::router` 标识符集合，与 `crates/*/src/**` 中公开 `pub fn router(` 的 crate 集合做差集；差集非空即失败（显式豁免名单除外，豁免须写理由）。实施时先跑一次差集，对暴露出的每个未挂载 crate 逐一决策（挂载 / 豁免并记账）。
- **风险**：低（测试增量；可能暴露新的未挂载 crate → 逐个决策，不默认挂载）。

---

## 5. B3 — laiyipao 竞态修复与性能

### B3-1 `SetRecomputeMode` 数据竞争修复（真实竞态，无用例覆盖）

- **证据**：`internal/service/recompute_mode_setter.go:37` 运行期写 `s.Cfg.SettleRecomputeMode`；同文件 `:93` `RecomputeModeState`（`GET /admin/recomputes/mode`）并发读；`recompute_gate.go:177-189` `initRecomputeGate` 也读。PUT 与 GET 并发即 race（`go test -race` 可抓，现无用例）。代码注释（`:30-32`）自述「没有并发切档的需求…得改成 atomic.Pointer…那时再说」——本轮就是「那时」。
- **改法**：档位从 `Cfg` 值字段移入 `RecomputeGate`（gate 已经 `atomic.Pointer` 整体替换，`recompute_gate.go:69/177`）：`NewRecomputeGate(mode, rate)` 构造时把 mode 存为 gate 的不可变字段；`SetRecomputeMode` 构造新 gate 并 `Store`；`RecomputeModeState` 读 `s.gate.Load().mode`。`Cfg.SettleRecomputeMode` 降级为「初始配置」（注释写明）。`initRecomputeGate` 的基准档位改为当前 gate 档位（保持运行时切档语义，不被 Cfg 重置）。
- **风险**：低。约 30 行；`s.Cfg` 测试写法（约 20 处直写 `ts.Cfg.X`）如涉及档位需同批调整（实施时核）。
- **验收**：新增并发测试（`-race`）：并发 `SetRecomputeMode` + `RecomputeModeState` + settle 路径读档位，无 race 报告且终态一致；`go test ./internal/service/ -run Recompute -race -count=1` 通过；`go build/vet/gofmt` clean。

### B3-2 `AdminUserDetail` 串行查询并发化（12 → ~7 往返）

- **证据**：`internal/service/adminuserdetail.go:61`：12 次串行往返（用户行 1 + LoadWallet 3 + LoadMastery 2 + Loadout 1 + equipment 1 + AdminListBattles 2 + verifications 1 + flow count 1）。7 个段互不依赖。
- **改法**：`errgroup` 并发 7 段（pgxpool 并发安全）。**错误语义保持**：用户行段的 `ErrNotFound`→404 必须稳定——先单独取用户行（或 Wait 后优先判定用户行段错误），再并发其余段；各段已有错误前缀原样保留。
- **风险**：低-中。纯调度变更，SQL 不变。DB 集成用例本机无法跑（§1），标注待 PG；Go 层可编译 + DB-free 单测回归。
- **验收**：`go build/vet` clean；`adminuserdetail_test.go` 现有 DB-free 部分通过；新增「错误优先返回用户行段错误」的纯逻辑测试（如用 fake/接口或既有 renameTable 手法的 DB-free 子集——实施时按现有测试基建可行性判定，不可行则仅人工复核错误路径）。

### B3-3 `AdminListBattles` 验真子查询改相关形式

- **证据**：`internal/service/stats.go:608-613`：每次分页都对**全表** `replay_verifications` 做 `GROUP BY battle_id`，即使已按 `user_id` 过滤。
- **改法**：`userID≠0` 时子查询改相关形式（`WHERE battle_id IN (SELECT id FROM battle_records WHERE user_id = $1)`）；`userID=0` 保持原样（运营全量视图，语义不变）。
- **风险**：低（SQL 重写，两种形态等價；人工复核 + 现有分页用例待 PG 跑）。
- **验收**：SQL 双侧人工复核；`go build/vet` clean；DB 集成用例标注待 PG。

---


## 6. B4 — oa4rust 性能

### B4-1 bcrypt 移出 async worker（登录热路径）

- **证据**：`crates/auth/src/password.rs:101` `bcrypt::verify`（cost 10 ≈ 60-100ms 纯 CPU）在 login handler 内联执行；`crates/program_center/src/lib.rs:10691,10780,10785` hash/verify 同样内联。tokio worker 数 = CPU 核数，无自定义 runtime → bcrypt 占满 worker、连接池排队。这是 `docs/performance-baseline.md` 登录场景 P50 7913ms 的**根因假设**（基线库无种子用户，100% 错误是另一回事；8 秒 P50 与该模型吻合）。
- **改法**：`verify_password`/hash 调用包进 `tokio::task::spawn_blocking`（bcrypt crate 是同步 API；闭包持 owned String，Send 无碍）。缓存逻辑（`set_cached`）保持在闭包外/后，语义不变。
- **风险**：低（纯调度变更）。**验收**：`cargo test -p auth -p program_center`（或最小受影响集）通过；登录行为单测不回归。

### B4-2 `program_init` 补事务 + 批量插入（正确性 + 性能，顺带解决 `crud_create_many` 零调用方）

- **证据**：`crates/program_init/src/lib.rs:379` 逐数据源 INSERT **无事务**——中途失败留下部分配置，且入口刚校验过「已配置则拒绝」，重试即坏（正确性风险）。`crates/shared/src/crud.rs:289-319` `crud_create_many`（10-23 新增）全仓零生产调用方（自带 6 单测质量良好，弃之可惜）。
- **改法**：该循环包进事务；批量步骤改用 `crud_create_many`（按 65535 参数上限自动分片，恰是其设计场景）。若列类型不全为 String 不适用，则用多行 VALUES 达到同效果，并在 crud.rs 文档登记预留能力。
- **风险**：中（SQL 重写；实施时先读全函数确认循环体形状与错误路径，回归其现有单测）。
- **验收**：`cargo test -p program_init` 通过；事务性加一条「中途失败无残留」测试（用既有测试基建，不可行则人工复核）。

### B4-3 `program_center_core_entity` 移除空转 SeaORM 池（review-gated）

- **证据**：`crates/program_center_core_entity/src/lib.rs:17-25` 启动时 `create_sea_orm_pool().await.ok()` 建最多 20 连接的第二个池，但 5 个子 router 全返回 `Router::new()`（`handlers/application.rs:222-227` 等，注释称路由归 program_center crate 防 merge panic）；`src/lib.rs:142` 以 `.await` 挂载。每次启动白占 PG 连接额度，`.ok()` 吞初始化失败。
- **改法**（实施时先核实全 crate 无其他池使用点）：删池创建，router 改同步返回 `Router::new()`，create_app 去掉 `.await`；crate 顶部注释说明「路由由 program_center 提供，本 crate 不再建池」。
- **风险**：低-中（触碰 create_app；若核实发现池有其他消费者则整项转延期并记账）。
- **验收**：`cargo check -p oa4rust` clean；B2-3 冒烟（服务起得来）+ 路由集合不变（除 B2 新增外）。

### B4-4 attendance/cms 循环内逐条 execute 批量化（review-gated）

- **证据**：`attendance_assemble_control/src/lib.rs:7825`（考勤重算 N 参与人 = 2N 次往返，有事务）；`cms_assemble_control/src/lib.rs:10301,10357`（correlation 建/改逐对 DELETE+INSERT，有事务）。
- **改法**：读全函数后改多行 VALUES / `unnest`。**任一处复核后判定「改动面超过纯批量化」即转延期**（无本地 PG，SQL 重写盲改风险 > 收益）。
- **风险**：中。**验收**：`cargo check` + 现有单测；DB 行为标注待 PG。

---

## 7. B5 — augmentor 性能与可观测

> 验证解释器：`C:/Python314/python.exe`（3.14.4，依赖齐全；managed 3.13.14 的 site-packages 被沙箱最小化，缺 numpy）。命令统一带 `--no-cov -p no:cacheprovider`。

### B5-1 `retry.py` 的 `requests` 导入惰性化（每次 CLI 启动省 ~210-250ms）

- **证据**：`augmentor/retry.py:21-24` 全包唯一顶层 `import requests`（try/except 降级），仅 `classify_error` 两行 isinstance 使用；它在 `__init__.py` 第一条 import 链上。实测 `import cli` ≈ 820ms，其中 requests ≈ 211ms（含 charset_normalizer 87ms）；本地命令（profile/clean/convert/validate…）永远不需要它。
- **改法**：import 移入 `classify_error` 函数体（保留 try/except ImportError 降级支）。`models/base.py:227` 已有函数内 import 先例。不碰 `__all__`/`__getattr__` 契约（`test_package_exports.py` 钉死 PEP 562 禁止）。
- **风险**：S。**验收**：`-X importtime` 对比（820ms → ~600ms 量级）；`pytest tests/unit/test_retry.py tests/unit/test_model_backend.py` 通过。

### B5-2 `generate()` 耗时统计出口（LLM 延迟是唯一真瓶颈，当前不可观测）

- **证据**：`augmentor/models/base.py:336-425` 已有 `request_count/error_count/retry_count/retry_wait_seconds` 四出口（A66），无任何耗时统计；`/api/status` 已暴露 retry 两键（`api/routes/status.py:33-36`）。
- **改法**：加 `total_generate_seconds`（锁内累加，`reset_stats` 归零）+ 每次真实调用一条 debug 日志（prompt 长度、耗时、缓存命中路径）；`StatusResponse` 加可选键。
- **风险**：S（纯增量；不断言旧值）。**验收**：`pytest tests/unit/test_model_backend.py tests/integration/test_api_status.py` 通过。

### B5-3 `check_dependencies()` 结果缓存

- **证据**：`augmentor/diagnostics.py:59-104` 每次 `/api/status` 对 8 个可选依赖逐个 `importlib.import_module`，缺席模块失败导入要爬依赖树（sentence_transformers 缺席时最贵）。
- **改法**：`functools.lru_cache` 缓存报告（进程内依赖不变）。**先读 `test_diagnostics.py`**：若有 monkeypatch 模块缺席的用例，提供缓存清除口或让测试走原函数。
- **风险**：S。**验收**：`pytest tests/unit/test_diagnostics.py tests/integration/test_api_status.py` 通过。

### B5-4 并行/串行管线口径对比例测试（只加测试，不改行为）

- **证据**：`pipeline.py:240-263`（并行路径 `_process_single_item`）只做质量阈值过滤；`pipeline.py:601-629`（串行 `augment_seed`→`filter_by_quality`）含对已生成集合的多样性过滤。同配置下 `use_parallel=True/False` 产物口径不同，两侧均有测试钉住、无文档说明是有意分歧。
- **改法**：新增对比例测试（mock 后端）：同一 fixture 跑两个开关，**钉住当前差异并注释说明」，把隐性分歧变显性契约；行为对齐是产品决策，转 §9 台账。
- **风险**：S（测试增量）。**验收**：新测试通过；若 mock 基建无法干净隔离两侧，转延期并记账（fail-loud）。

---


## 8. B6 — CI 与工程卫生

### B6-1 新增 auto-checkin CI（最活跃子项目此前无任何门禁）

- **证据**：auto-checkin 的 35 个测试与 typecheck 从未在 CI 跑过——B1-2 的口令 bug 正因无门禁而漏出（有测试也只测了 API）。llm-lab/cool 同样无 CI。
- **改法**：新增 `.github/workflows/auto-checkin-ci.yml`：ubuntu + Node 22 + `npm ci` + `npx tsc --noEmit` + `npm test`。**不带 paths 过滤**（带 paths 的 workflow 永不可设为 required check——仓库既有教训，见 laiyipao-ci.yml:8-20 自述）。
- **风险**：低（新增文件，不影响现有 workflow）。**验收**：YAML 语法校验（python yaml.safe_load 或 actionlint 如有）；本地模拟 job 步骤全通过。

### B6-2 `govulncheck@latest` 钉版本

- **证据**：`.github/workflows/laiyipao-ci.yml:309` `go install govulncheck@latest` 是全仓 SHA-pin 供应链纪律里的例外。
- **改法**：钉到具体版本（与 Go 1.26 兼容的最新稳定版，实施时查证后钉 semver）。
- **风险**：低。

### B6-3 删除 9 个死 codemod 脚本

- **证据**：`scripts/` 下 `dedup_apis.js`、`add_all_fallbacks.js`、`add_request_fallbacks.js`、`analyze_coverage.js`、`coverage_report.js`、`final_coverage.js`、`final_coverage_report.js`、`generate_missing_apis.js`、`lazy_views.js` 全部读写 `oa4rust-web/...`（该目录已在 `063c5d4e4` 改名 `oa4rust/frontend/`），现在跑任何一个是 ENOENT。2026-09-19 一次性脚本，产物早已落库，无 workflow/文档引用。
- **改法**：删除（实施时对每个脚本再确认零引用：workflow / package.json scripts / 文档）。
- **风险**：低。

### B6-4 `check-warnings.mjs` 修正与扩充

- **证据**：`scripts/check-warnings.mjs:99` label 硬编码「97 crates」，实测 96；未接入任何 CI；缺 auto-checkin `npm test`（只有 tsc）与 llm-lab `pytest`（只有 compileall）两个廉价项。
- **改法**：crate 计数改动态；补 `auto-checkin: npm test`、`llm-lab: pytest` 两项。**不接 CI**（required-check 策略需用户拍板，转 §9）。
- **风险**：低。**验收**：`node scripts/check-warnings.mjs --list` 正常；扩充项本机实跑（auto-checkin test 可跑；llm-lab pytest 用 managed py 跑通的项目内测试——实施时确认测试集不依赖 torch，依赖则只挂 compileall 现状并记账）。

### B6-5 小额修复一组

| 项 | 证据 | 改法 | 风险 |
|---|---|---|---|
| cron 估算回退显示 | `auto-checkin/src/scheduler.ts:14-27` 只认字面量，`*/30` 等匹配失败返回 null → 面板显示「空闲待命」而非下次时间 | 不支持时回退显示原始 cron 表达式 | 低（纯展示） |
| rust-env.sh 存在性检查 | `scripts/rust-env.sh:9-10` 硬编码 MSVC 14.44.35207 / SDK 10.0.26100.0，升级后静默失效 | 加存在性检查，fail-loud | 低 |
| README PANEL_TOKEN 文档 | `auto-checkin/README.md:124` 提了 PANEL_TOKEN 但未说明前端从 URL `?token=` 取值转 `X-Token` 头（`public/app.js:7,14`），token 仍进浏览器历史 | 随 B1-2 一并更新文档：说明仅头传输 + 静态资源开放 + 建议首次弹窗输入（评估后只做文档，不做交互改造） | 低 |
| 根 MEMORY.md 死链 | 链接的两个 sept-2026 文档不存在 | 删死链或改注 | 低 |
| 根 README 用例数漂移 | 称 llm-lab「32 个验证用例」，实测 38 | 改数字 | 低 |

### B6-6 `behavior-compare` CI job 空转治理（review-gated）

- **证据**：`oa4rust-ci.yml` 的 `behavior-compare` job 围绕 `tests/behavior_compare.rs`（**文件从来不存在**）建了 build+PG+Java 容器全套，仅靠 `continue-on-error` 兜底；且 `u2_probe` 与 `integration-tests` job 重复跑。
- **改法**：删除 behavior-compare 里的 u2_probe 重复；comparator 落地前把该 job 改为 `workflow_dispatch` 手动触发（省每次全量 build）。
- **风险**：低-中（动 CI 结构；实施时通读 job 全貌确认无其他有效步骤后被拖）。**验收**：YAML 合法；与 oa4rust CI 负责人视角复核（本机只能静态验）。

---

## 9. 延期台账（本轮明确不做，含理由）

| ID | 事项 | 不做理由 |
|---|---|---|
| OA-P0-1 余量 | empower 等分类的**契约路径别名**（57/76 已有实现，剩 19 条为 mockputtopost 方法约定差异） | 挂载既有 router（B2）已复活真实前端调用面；契约别名需与 Java 契约逐条对表，属产品/契约决策 |
| OA-PERF-LIMIT | 737 处列表查询无 LIMIT | 改响应形状影响前端契约，需逐端点对表 + 灌数据实测；数据量未到临界 |
| OA-MOBILE-CONTACTS | 移动端通讯录全量拉取 | 前后端联动 + 分页契约，同族 |
| OA-FI002-CI | FI-002 的 (table,column) 对撞扫描固化进 CI | 完整价值需 live PG；本轮无库，留待有库轮次 |
| LY-5 全量 | 整个 Config 原子化 + SIGHUP 热加载 | 33 处引用点的结构性改动；B3-1 已消真实竞态并打好结构基础 |
| LY-7 | bag.vue 虚拟化 | 列表有界（~114 行）；前端视觉改动本机无法验收 |
| LY-PERF-1 | SettleBattle 复用 build 快照 | 复用后响应 rating 可能不含本局新解锁技能——**语义变化**，需产品确认；实施时若证明两次加载点数据等价则升级执行，否则维持延期 |
| LY-PERF-3/5 | LoadWallet 合并、requireAdmin 三查合一 | 经济/鉴权热路径，无本地 PG 验证，盲改风险 > 收益 |
| LY-GAP-1 | me.vue works 玩家可选 | 销账不符已登记；是否开放选择是产品决策 |
| LY-GAP-2 | 后台详情触发体力 regen 写 | 读端点有写副作用是设计问题，需先定「后台只读钱包」语义 |
| AU-P1 | 异步 LLM 后端 | 勘察证实 `augment_async` **零产品消费方**（只有测试引用），异步化无收益入口；维持延期 |
| AU-P2 | 全包惰性导入 | `test_package_exports.py` 明文禁止 `__getattr__`（PEP 562）；B5-1 已拿走其唯一安全切片 |
| AU-PERF-POOL | 连接池 pool_maxsize 与线程数对齐 | 需压测证明；改大增大上游压力 |
| AU-PERF-ANYIO | anyio 40 token 线程池调整 | 先量后改，无压测基准 |
| WF-REQUIRED | required checks 核对（3 个 bypassed） | GitHub 分支保护设置，需用户在页面操作；本文档 §10 记录待办 |
| COOL-CI | cool 的 javac -Werror CI job | 需先定 jar 来源（Maven Central 下载）；本机无 jar |
| WARN-CI | check-warnings.mjs 接入 CI | required-check 策略需用户拍板（带 paths 不可 required） |

---

## 10. 验证与提交台账（实施中填写）

> 口径：**已完成 = 有改动 + 有构建/测试证据**；跳过/待外部条件逐条写明（AGENTS #11 fail-loud）。

| 项 | 状态 | 证据 | commit |
|---|---|---|---|
| B1-1 oa4rust clippy | 已完成 | shared/doc 缩进修复；另发现并修复 auth crate 编译断裂（sha1 0.11 vs sha2 0.10 digest 不互通，d1fe3ac67 引入、被 clippy 失败掩盖）——main 后端当时根本编不过；cargo test -p auth 99 passed；全仓 clippy --workspace --all-targets 全绿 | 08c85ef2b |
| B1-2 auto-checkin 口令面板 | 已完成 | server.ts 改只守 /api/ 前缀；新增 2 条回归测试；npm test 40/40（37+2+1 组后续）；tsc 0 错；README 补说明 | 08c85ef2b |
| B1-3 records limit 夹取 | 已完成 | Math.max 下限夹取 + 单测（40/40 内） | 08c85ef2b |
| B1-4 seed_java.py 路径 | 已完成 | 补 backend/ + read_migration_091() fail-loud；本机 import 复现验证可读（13 keys / 1135 chars） | 08c85ef2b |
| B2-1 挂载 organization_assemble_personal | 已完成 | create_app 补 merge；cargo test -p oa4rust 构建通过（挂载后路由存在） | dbd706b31 |
| B2-2 挂载 empower | 已完成 | 同上；session_manager.clone() 置于 personal_extend 消费前 | dbd706b31 |
| B2-3 挂载冒烟（401/404 判别） | 已完成 | 挂载差集复核为空（recon 脚本）；路由冲突预演零重叠；cargo test 构建 create_app 即权威判据 | dbd706b31 |
| B2-4 merge 守卫测试 | 已完成 | 新增 tests/mount_coverage_guard.rs（差集非空即红）；当前差集为空 | dbd706b31 |
| B3-1 竞态修复 | 已完成 | initRecomputeGateWith 拆分；SetRecomputeMode 不写 Cfg；RecomputeModeState 单次 Load；新增 8×8 并发 -race 测试通过；原测试不回归 | 88a149d9e |
| B3-2 AdminUserDetail 并发化 | 已完成 | 用户行段顺序执行 + 7 段 WaitGroup 并发；错误按段序收集；go build/vet/gofmt clean；DB 用例无 PG 跳过（待 PG） | 88a149d9e |
| B3-3 battles 相关子查询 | 已完成 | userID≠0 时子查询收窄；SQL 人工复核；DB 用例待 PG | 88a149d9e |
| B4-1 bcrypt spawn_blocking | 已完成 | auth 增 verify_password_async 包装 + 3 调用点；program_center 3 处（hash×2/verify×1）移入 spawn_blocking；语义不变（JoinHandle 出错 fail-closed） | 17afcd4c6 |
| B4-2 program_init 事务+批量 | 已完成 | 逐条 INSERT → 事务 + 多行 VALUES（65535 参数分片）；占位符全局连续单测 2 条；cargo test -p program_init 过 | 17afcd4c6 |
| B4-3 core_entity 空池移除 | 已完成 | 删 create_sea_orm_pool().await.ok()；_router 传 None；router 改同步；create_app 去 .await | 17afcd4c6 |
| B4-4 attendance/cms 批量化 | 已完成（review-gate 通过） | 两处均为纯批量化形状：attendance 2N→2 往返（DELETE=ANY + 多行 VALUES，去重保序、personsRebuilt 按原始长度）；cms 两函数同型（created/updated 同口径）；tx 原子性不变 | 17afcd4c6 |
| B5-1 requests 惰性导入 | 已完成 | import cli 820ms→456ms 实测；requests 离开启动链；test_package_exports 契约不回归 | 6ffdedc79 |
| B5-2 generate 耗时统计 | 已完成 | total_generate_seconds + debug 日志；/api/status 新键；TS 类型/System.tsx/契约键集/渲染守门同步；319 相关测试过 | 6ffdedc79 |
| B5-3 diagnostics 缓存 | 已完成 | lru_cache(maxsize=1)；patch 探测函数的测试同步 cache_clear；49 相关测试过 | 6ffdedc79 |
| B5-4 并行/串行对比例测试 | 已完成 | 新增分歧钉测试（call 级 + 行为级双证据）；test_pipeline.py 68 过 | 6ffdedc79 |
| B6-1 auto-checkin CI | 已完成 | 新 workflow（无 paths 过滤，SHA pin）；YAML 校验过；本地模拟步骤全过 | 915d380a6 |
| B6-2 govulncheck 钉版 | 已完成 | @latest → @v1.9.0（查证当前最新稳定版） | 915d380a6 |
| B6-3 删死 codemod | 已完成 | 9 脚本 git rm（零外部引用最终复核） | 915d380a6 |
| B6-4 check-warnings 修正扩充 | 已完成 | crate 计数动态化；auto-checkin 补 npm test；llm-lab-tests 项 requires 守门（无 pytest SKIP）；--list 与实跑验证过 | 915d380a6 |
| B6-5 小额修复组 | 已完成 | cron 回退显示 + 3 测试；rust-env 存在性检查；MEMORY.md 死链转史实；README 32→38 | 915d380a6 |
| B6-6 behavior-compare 治理 | 已完成（方案调整） | 保留 job 跑 push（双栈播种+启动冒烟是真覆盖，转 manual 会静默丢覆盖——偏离原计划，理由记账）；删重复 u2_probe；加定位说明头 | 915d380a6 |

**待用户操作**：GitHub 分支保护 required checks 核对（§9 WF-REQUIRED）；CI 全绿确认（每批推送后）。

---

## 11. 审计修订记录（2026-10-24，实施前只读复核）

> 方法：`plan-claim-recon` 技能 + 两个只读侦察脚本（`Temp/recon_20261024.py` 判据复核、`Temp/route_conflict_check.py` 路由冲突预演，均含自检）。**结论：规划主体成立，以下 5 处修订后进入实施。**

### 11.1 修订一（扩大）：B2 挂载范围 2 → 4 个 crate

守卫测试预演（merge 标识符 vs `pub fn router` crate 差集）暴露规划遗漏的两个未挂载 crate，且**均被前端实际调用**：

| crate | 路由数 | 前端调用证据 | 现状 |
|---|---|---|---|
| `base_core_project` | 8 | `ServerApp.vue:82-84,330`（`/api/echo`、`/api/cache/detail`、`/api/openapi`、`POST /api/cache`——服务器状态页）、`CmsIndexApp.vue:780-781`（缓存刷新按钮） | 全 404 |
| `organization_assemble_authentication` | 53 | `OrgDirectoryApp.vue:425-446`（组织目录认证流：authentication CRUD / safe/logout / switchuser / two-factory-login）、`Personal.vue:328-329`（oauth 配置） | 全 404 |

- 两 crate 的 b74 安全门（`security_gate_tests_b74.rs`，OAuth 授权码签发会话门）已就位；git 史无「故意不挂载」记录，属创建后漏接线（同 empower）。
- `organization_assemble_authentication` 的 `router` 内部自建 SessionManager（`routes.rs:9-13`），签名仅需 pool；`base_core_project` 仅需 pool。
- **冲突预演**：4 crate 全部 (method,path) 与已挂载 crate 及互相之间**零冲突**（脚本输出）。且 `program_center_core_entity/src/lib.rs:222-227` 注释明示 axum 对重叠 method route 会 **panic**——权威冲突判据是 `cargo test` 构建 create_app（挂载后必跑）。
- B2-4 守卫测试在本批后差集应为空；豁免名单预计为空。

### 11.2 修订二（设计定稿）：B3-1 竞态修法比原设计更小

读 `recompute_gate.go` 全文后定稿：`RecomputeGate` 本身持有不可变 `mode` 字段（`Mode()` 访问器，mutex 保护），`Service.gate` 是 `atomic.Pointer[RecomputeGate]`。修法：

- `initRecomputeGate()` 拆为 `initRecomputeGate()`（读 Cfg，仅 New 与测试用）+ `initRecomputeGateWith(mode string)`（实际构造 + 原有 fail-loud 日志不变）；
- `SetRecomputeMode`：校验后直接 `initRecomputeGateWith(mode)`，**不再写 `s.Cfg`**（消除 race 源）；
- `RecomputeModeState`：`ConfiguredMode` 改读 `s.gate.Load().Mode()`（nil 兜底同 `recomputeVerdict` 范式）。
- 兼容性：`AdminRecomputeSummary` 的 `ConfiguredMode` 本就来自 `g.Mode()`，两侧口径统一；测试中「写 `ts.Cfg.SettleRecomputeMode` + 调 `initRecomputeGate()`」模式（recompute_live_test.go 5 处）不受影响。14 处测试引用点在实施时逐个跑测试回归。

### 11.3 修订三（证据精确化）：B4-3 生产 router 链

生产链为 `create_app` → `program_center_core_entity::router(pool).await`（`routes.rs` 转调）→ `program_center_core_entity_router(pool)`（lib.rs:17-28，**建池在 :20**）→ 5 个 handler `_router` **全部返回 `Router::new()`**（application.rs:222-227 已验，注释明示路由归 program_center 防 merge panic）。修法精确为：删 :20 建池行，5 个 `_router` 调用传 `None`（`_db` 参数本就下划线标记弃用），`routes::router` 与 `program_center_core_entity_router` 改同步 fn，create_app 去 `.await`。

### 11.4 修订四（工具可信度注记）：路由冲突检查器的边界

`route_conflict_check.py` 扫到 6 条「已挂载 crate 间冲突」，逐条定位**全部为 `#[cfg(test)]` mock router / 常量表误报**（如 `program_center_mock_router` lib.rs:30-42）。结论：该脚本用于「待挂载 crate 零冲突」的**辅助**判据；**权威判据是 cargo test 构建 create_app**（axum 真重叠即 panic）。B2 验收同时含两者。

### 11.5 修订五（精确化）：seed_java.py 失效路径

全文件扫描确认只有**一个** distinct 失效路径 `oa4rust/migrations/091_seed_real_process_definition.sql`（:126、:134 两处引用，补 `backend/` 后存在）。

### 11.6 复核通过的判据（摘）

clippy 失败已实跑复现；auto-checkin public/ 仅 3 文件且无 assets；server.ts:104 负 limit；laiyipao 竞态三读点 + `initRecomputeGate` 生产调用点（service.go:129 / recompute_mode_setter.go:38）；adminuserdetail.go 全段在位；stats.go:612 全表 GROUP BY；password.rs bcrypt 无 spawn_blocking；program_init 0 transaction；retry.py:22 顶层 `import requests as _requests`；base.py 无 total_generate_seconds；diagnostics 无 lru_cache（test_diagnostics.py 有 2 处 monkeypatch，实施时先读）；govulncheck@latest；behavior_compare.rs 不存在；check-warnings label 97；9 个死脚本全部引用 oa4rust-web；rust-env.sh 版本硬编码行。

---

## 12. 最终验证阶段的额外收口（2026-10-24，提交推送后）

CI 复盘发现 laiyipao CI 在**本轮开始前的 main（88681f239）上就已失败**（gh run 39989752210 实证）——10-23 轮留下了若干红守卫。本轮顺手收口 3 个，剩余 2 个 ownership 守卫同批修复：

| 守卫 | 根因 | 修复 | commit |
|---|---|---|---|
| TestOpsDocMatchesReality | 10-23 观测轮加 METRICS_ENABLED/PPROF_ADDR 未进运维手册 | OPERATIONS.md §2 补两行（计数 25→27） | 88a149d9e |
| TestEveryRouteHasClientCaller | 同上：`/metrics` 无客户端调用方且未登记豁免 | KNOWN_UNSERVED 补 `GET /metrics`（理由带 METRICS_ENABLED + OPERATIONS.md §2 锚点） | f2fcf4177 |
| TestOwnershipExemptionKeysAreStillPresent | Leaderboard 重构后旧豁免键失效 | 删失效键（失效豁免比没有豁免更糟） | e7051f065 |
| TestUserScopedQueriesMustFilterByOwner | adminDashboardUncached 3 条 battle_records 聚合无归属过滤 | 按守卫规矩登记豁免（管理面、只吐计数、admin 组门禁后） | e7051f065 |

存量失败中的 task-claim 家族（TestE2ETasksClaim / TestClaimTaskService / TestConcurrentClaimTaskOnlyOnce / TestLockChallengeStateCountersBeforeWallets / TestTxMidwayFailures）在预轮 CI 失败、本轮两次 CI 均未复现——时序性抖动，代码未动，留档观察。

## 13. 全量验证汇总（本机 + CI）

| 门禁 | 结果 |
|---|---|
| oa4rust `cargo clippy --workspace --all-targets` | **全绿**（修复前 main 因 auth 编译断裂 + shared clippy 双失败） |
| oa4rust `cargo fmt --all -- --check` | 通过 |
| oa4rust 改动 crate 单测 | auth 99 / program_init 46（含 2 新）/ core_entity 20 / attendance 48 / cms 337 / mcp_server lib 19 全过 |
| mount_coverage_guard / designer_route_match | 通过（差集为空；路由形状不回归） |
| laiyipao `go build/vet/gofmt` | clean |
| laiyipao 全量 `go test` | DB-free 全过；DB 用例无 PG 跳过；**CI（有 PG）侧存量红守卫 4/4 收口** |
| auto-checkin `npm test` + `tsc --noEmit` | 40/40 + 0 错；新增 auto-checkin CI 生产环境已跑（success） |
| augmentor 全量单测 | 8632 passed / 1 failed（`test_parametrize_blind_face_l98`——stash 对照证实的存量失败，与本轮无关） |
| check-warnings.mjs | rust-lints / auto-checkin（含 npm test）/ llm-lab-tests（requires 守门 SKIP）等项验证通过 |
| YAML 语法（3 个改动 workflow + 1 新增） | 全部合法 |

**环境限制（fail-loud）**：本机无 PostgreSQL（原生 PG 15 仅客户端无数据目录、initdb 被安全策略拦截、Docker Desktop 未运行），DB 集成用例全部标注「待 PG」；本机 loopback 对关闭端口的拒绝被延迟到 ~2s（全端口现象），放大了 DB 触达测试的耗时。oa4rust 的 arity_probe（4600+ 请求逐一带 DB 触达）因此未能在本机跑完，其 `build_core_router` 已成功构建（无路由冲突的权威判据），请求循环待 CI 验证。

---

## 14. 延期台账清算（2026-10-25，R1–R6）

> 用户指示「继续完成剩余所有台账并验证」。§9 台账 18 项逐项可行性评估后分期实施；
> 本节记录每项的处置结果。口径不变：**已完成 = 有改动 + 有构建/测试证据**。

### R1 — OA-P0-1 余量：契约路径别名补齐（原判「产品/契约决策」，本轮落地）

**结论：契约 76 条中 73 条补齐别名，3 条有意不映射（安全/冲突理由），中间件四张表
镜像补齐且 parity 实测为 0。**

- **旧结论的假象**：routing-completion-plan（2026-09-03）称「NEEDS 73 ROUTES」——
  复核发现路由早已存在于 `/api/person/*`、`/api/reset/*`（personal/empower/
  personal_extend/control 四 crate），缺的只是**契约基路径** `/api/organization/
  assemble/personal/*` 的接线。
- **落地**：三个 crate 的 router 内新增「契约路径别名」段，共 **73 条**路由原样委托
  同一 handler（personal 57 / empower 14 / personal_extend 2）。旧路径行为零变化。
- **3 条有意不映射**：
  1. `GET regist|reset/check/password/{password}`（2 条）——口令入 URL 的旧形态，
     Rust 侧已按「十类功能1 安全重设计」改 POST body；别名回 GET 是安全回退；
  2. `POST custom/{name}/mockputtopost`——与 organization_assemble_personal 既有路由
     `custom/{id}/mockputtopost`（POST）同形，双注册会 axum panic；该契约路径
     已由既有路由服务。
- **契约数据怪癖**：`managerCreate|POST|empower` 的 path 字段是 "empower"（提取器
  取了类级路径），实际在 `/api/person/empower/manager`——按 key 覆写映射
  （不覆写就与 create 双注册同路径，u2_probe 实测抓到 panic）。
- **中间件 parity（本轮最深的一处）**：别名要可用且不能削弱安全，四张表必须镜像——
  `constants.rs` 豁免 +13（**不含** mockdeletetoget 两条：旧径本就要登录，给别名
  加豁免是弱化不是对齐）、限流 +3、`ADMIN_WRITE_PREFIXES` +1；`rbac.rs` 注册表
  +7（6 条 Public 镜像 + 1 条别名兜底 Authenticated）。**兜底条不能省**：
  `check_permission` 对注册表未命中路径早退 Authenticated 且不 consult
  `requires_admin`——少了它，别名下的 empower 等写操作绕过 admin 门禁
  （单测实测抓到：裸 requires_admin 比对发现 `PUT .../personal/reset` 不一致，
  深挖后定位到 None 早退这条控制流）。
- **守卫（两道，表同源 `shared::middleware::contract_alias::CONTRACT_ALIAS_PAIRS`）**：
  1. `shared` crate 单测 `contract_alias_parity_tests`：73 对的豁免/限流/Public/
     **有效 admin**（复刻 check_permission 控制流，非裸 requires_admin）四项逐一相等；
     外加双锁单向不变式（豁免 ⟹ Public，禁 401 死态）；
  2. `oa4rust` 集成测试 `tests/contract_alias_parity.rs`：全栈 oneshot 实测 73 对
     状态码逐对相等 + 别名非 404（匿名臂证明 exempt/Public parity；admin parity
     由单测把守——匿名请求两侧都 401，看不见 admin 差异）。
- **行为变更登记**：既有路由 `POST /api/organization/assemble/personal/custom/{id}/mockputtopost`
  从「仅要会话」变为「要 admin」（别名兜底 Authenticated + ADMIN_WRITE_PREFIXES 的
  连带效果）。理由：其旧径同族 `POST /api/person/custom/{name}` 在契约语义里是
  admin 操作；前端无此调用（grep 实证）；handler 自体仍有 require_session。
- **验证**：`cargo test -p shared` 167 passed（含 2 新）；`contract_alias_parity`
  全栈 73 对全过；`u2_probe_full_app_build`（create_app 全量构建，axum 冲突权威
  判据）过；`mount_coverage_guard`/`designer_route_match` 不回归；fmt/clippy 全净。

### R2 — OA-FI002-CI：静态列扫描固化 + CI 棘轮

**结论：FI-002 的「代码 SQL vs schema 对撞」方法固化为无需活库的静态脚本并接入 CI；
首跑即挖出 48 对真实缺口（7 遗留表 + 41 迁移缺列），全部登记基线并标「待有库轮次」。**

- **落地**：`oa4rust/backend/scripts/fi002_column_scan.py`
  - schema 宇宙：解析 `migrations/*.sql`（209 个正向迁移）→ 379 表 / 5096 列；
  - 代码引用：扫 `crates/*/src` + `src/` 的 Rust 字符串字面量（4345 条含 SQL），
    抽表引用/别名映射/限定列/INSERT 列清单/UPDATE SET/单表 SELECT 裸列；
  - 棘轮：对 `docs/audits/artifacts/fi002_unknown_columns.tsv` 基线，新增未知对
    退出 1，缺口修掉提示收敛。
- **CI**：`oa4rust-ci.yml` 新增 `fi002-column-scan` job（秒级、无需 PG）。
- **开发过程中修掉的 7 类误报（都留了注释，防复发）**：
  1. `ident_of` 只读第一个 IDENT → ALTER 的列名读成表名（auth_person 曾只剩
     13 真列 + 1 个假列）；
  2. 单列 ADD COLUMN 正则认不出**多列 ALTER**（018 一次加 7 列）→ 改语句级扫描；
  3. RENAME 大小写归一后成了「同名」告警 → 视为 no-op；
  4. CTE 名当表（`WITH RECURSIVE chain AS (...)`）→ 解析 CTE 定义 + 全仓 CTE 名集合
     （片段引用外层模板里的 CTE）；
  5. Rust 行续接（`\` + 换行）残留在提取文本里 → 反转义；
  6. SELECT 裸列分支把限定列的别名当列名（`SELECT u2.id` → 列 `u2`）；
  7. `information_schema` / `pg_class` / `schema_migrations` 等系统表；
  另：测试文件与 `#[cfg(test)]` 模块的夹具 SQL（虚构表 users/t/x_probe）不计违规。
- **首跑发现（48 对，全部进基线）**：
  - **遗留表 7 对**：`pp_c_touch_detained|expire|passexpired.xupdatetime`、
    `x_process_transition.{condition,id,to_activity,work_id}`——Java 时代建的表，
    migrations 从不创建（011 只做大小写重命名）。引用它们的 handler 在 fresh DB
    上会 500；修法要先定「这些表归谁建」，转 §9 新台账。
  - **迁移缺列 41 对**（与 FI-002 同类，fresh DB 必 500）：
    `auth_person_identity.person_id`、`x_meeting_checkin.{meeting_id,person,checkin_time}`、
    `x_bbs_section.creator`、`x_cms_index.status`、`x_portal.{config,pages}`、
    `x_express_subscribe.{xcallback,xcompany,xtrackingnumber}`、`x_program_*` 10 对、
    `pp_c_draft.xstatus`、`pp_c_keylock.{xidentity,xtitle}`、`pp_e_file.xcontent`、
    `pp_e_script.xcode` 等。修法按 101 的范式补幂等 ADD COLUMN，但**列类型须对
    live DB 复核**——本机无 PG，盲写类型会让 fresh DB 出现新错，故整批转 §9 新台账
    （「有库轮次」执行），不在此盲改。
- **验证**：基线态退出 0（「48 对未知均在基线内」）；临时把
  `FROM x_cms_index` 改成 `x_cms_index_typo` → 退出 1 并点名 8 对新违规（已还原）；
  YAML 合法（7 个 job）。

#### §9 台账新增（R2 发现，均需活库）

| ID | 事项 | 不做理由 |
|---|---|---|
| FI002-MISSING-COLUMNS | 41 对「迁移缺列」（见 `docs/audits/artifacts/fi002_unknown_columns.tsv`） | 修法是按 101 范式补幂等 ADD COLUMN，但**列类型**须对 live DB 复核；本机无 PG，盲写类型会让 fresh DB 出现新的类型不匹配错误（比现状更糟）。有库轮次按「先查 live information_schema 类型 → 生成迁移 → fresh DB 跑集成测试」执行 |
| FI002-LEGACY-TABLES | 7 对遗留表引用（`pp_c_touch_*`、`x_process_transition`） | 这些表不是 migrations 建的（Java 时代遗留）；补建表需要先定 schema 归属与索引，且 touch/transition 端点的语义要确认，属独立任务 |

### R3 — laiyipao 剩余性能项与读端点副作用

| 项 | 处置 | 依据 |
|---|---|---|
| LY-PERF-1 SettleBattle 复用 build 快照 | **关闭（不做）** | 两处加载形态不同：`replaySkillsSegment` 是 5 字段投影（slot/skill_id/**按等级解析的** base_damage/apply_stacks/heat_cost），`LoadBuildSnapshot` 是给回放哈希用的 JSON map。复用要从另一形状重推 S 段字符串——而 S 段是与客户端上报**逐字节比对**的反作弊判据，格式稍变即把所有正常对局判成伪造。收益仅「结算路径约 13 次往返里省 1 次」。风险/收益不成比例 |
| LY-PERF-3 LoadWallet 合并 | **已做** | 体力恢复 UPDATE 与随后的读钱包 SELECT 合并成 `UPDATE ... RETURNING`：3→2 往返（省的是每次进页面 fetchWallet 最热的一次）。语义逐位保持：行不存在 → UPDATE 命中 0 行 → RETURNING 无行 → 同一个 ErrNotFound 分支；RETURNING 返回更新后的 energy 与 energy_updated_at |
| LY-PERF-5 requireAdmin 三查合一 | **已做** | 每后台请求 3 次往返（admin_tokens → admin_users.status → admin_users.role）合并为一条 JOIN。判定顺序逐位保持：会话存在 → 未吊销 → 未过期 → 未停用（FK ON DELETE CASCADE 保证 token 行的 admin 必存在）。`ResolveAdmin` 保留为「只要身份」的委托入口（17 处测试断言的是会话有效性语义，不必为多一个返回值改写）；零调用的 `AdminRole` 删除 |
| LY-GAP-2 后台详情触发体力 regen 写 | **已做** | 读端点带写副作用的三条具体坏处：readonly 运营账号绕过写门、只读副本下会失败、审计出现无主动触发的写入。新增 `LoadWalletReadOnly`：同一公式**投影**（不结算、不推进 energy_updated_at），显示值与玩家侧一致；顺带省掉 TokenBalance 一次往返。体力参数提到包级常量（两处共用防漂）。新增静态守卫 `TestAdminUserDetailNeverWritesWallet` 钉住「详情页不得走回 LoadWallet」 |

**验证**：`go build ./...` / `go vet ./...` / `gofmt` 全净；静态守卫
（ownership 扫描 / 路由消费 / 文档漂移 / readonly 白名单）全过；
新守卫 `TestAdminUserDetailNeverWritesWallet` 通过。DB 集成用例无 PG 跳过（待 CI）。
剩余两处 `LoadWallet` 调用都在**写**端点（签到、结算），副作用属预期。

### R4 — CI 与工程台账收口

| 项 | 处置 | 依据 |
|---|---|---|
| WARN-CI（check-warnings 接入 CI） | **已做（改方案）** | 原顾虑是「全量 check-warnings 含 Rust clippy（30min），与 oa4rust CI 重复」。改为只跑**廉价项**：新增 `.github/workflows/warnings-gate.yml` → `--only rust-lints,llm-lab,llm-lab-tests,java`（合计约 1 分钟）。覆盖的是此前**完全没有 CI** 的三块：llm-lab 的字节码编译与 pytest（38 用例）、cool 的 `javac -Xlint:all -Werror`、以及「每个 workspace 成员都继承 `[lints]`」静态守卫（漏写无症状失效，只有这条扫描能发现）。不带 paths 过滤（保留将来可设为 required 的可能） |
| COOL-CI（jar 来源） | **已定并落地** | commons-codec **1.17.1**（与 `cool/README.md` 记录一致），CI 从 Maven Central 下载后按官方 **SHA-1 `973638b7…0579`** 校验。该 SHA-1 由两路互证：Maven Central 的 `.sha1` 文件 + 本机 `~/.m2` 里独立下载的同一版本 jar 实测一致；`javac -Xlint:all -Werror` 用该 jar 编译 `EncryptUtil.java` 通过 |
| WF-REQUIRED（required checks 核对） | **已核对并给出精确清单（不代改）** | 实测：main 无 classic branch protection，走 **rulesets**。`laiyipaoCIRuleset`（active）已要求 3 个 context，且与现行 job 名**逐字一致**：`Server build, vet & test (PostgreSQL)` / `Miniapp type-check, test & build` / `Admin type-check, test & build`。**未纳入 required 的**（按「确定性」排序，建议按此追加）：`quality`、`fi002-column-scan`、`openapi-guard`、`contract-gate`（三端契约）、`test`（auto-checkin）、`Dependency vulnerability gate`。**不建议**纳入：`integration-tests`/`unit-tests`（依赖 PG，本轮实测过 flaky）、`behavior-compare`（含 continue-on-error 段）。改法（用户执行一次即可）：GitHub → Settings → Rules → `laiyipaoCIRuleset` → 勾选上述 context，或用 `gh api --method PUT repos/:owner/:repo/rulesets/24845982` 提交完整 rules 数组。**没有代改的理由**：当前所有直推都走 admin bypass（推送时提示「bypassed N required status checks」），加 required 只增加 bypass 提示、不产生实际拦截；而一旦真正启用，任何 flaky job 都会卡住合并——需要先积累若干次全绿运行再启用 |
| 附带修复：oa4rust 夹具并发 flaky | **已修** | CI 实测（run 40181402605，R2 提交——该提交只动脚本/文档）：`personal::u2_custom_put_get_delete_roundtrip` 偶发 E23505 `auth_person_unique_id_key`。根因：`auth_person` 的 `id` 与 `unique_id` **各有一个唯一索引**，而 `ON CONFLICT` 只能指定一个仲裁索引；两个测试并发插同一行时 PG 可能先在 unique_id 索引上判冲突。修法：`upsert_fixture_person` 重试（每次重跑 DELETE 清残留），3 次仍失败才 fail-loud |
| 附带修复：laiyipao admin size 检查 flaky | **已修** | CI 实测（run 40181786246，R3 提交——只动 Go）：`Admin type-check, test & build` 在 `size:check` 红，报「dist 比源文件旧」。实测 vite 构建仅 0.6s，`dist/index.html` 写于 09:43:41.35、检查在 .44 跑——同秒内检出阶段某源文件 mtime 反超不到 0.1s（8 次运行 1 次）。修法：加 2 秒容差（抓「上次构建」的分钟级陈旧足够，又远小于人「改完再敲命令」的间隔）。两方向实测：源文件 = dist+1s → 退出 0；dist+10s → 退出 2 |

**验证**：warnings-gate 本地实跑 4 项（rust-lints/llm-lab/java PASS、llm-lab-tests 本机无 pytest 故 SKIP，
CI 有 pytest 会实跑）；YAML 合法；size:check 容差两方向实测；夹具重试逻辑编译通过。

---

## 15. 台账终态总表（2026-10-25 收尾）

> §9 的 18 项 + R2 新增 2 项 = 20 项，逐项给出终态。**没有「悬空」项**：
> 每项要么有改动+验证证据，要么有明确的不做理由（含证据，不是「先不做」）。

### 已完成（有改动 + 验证证据）

| ID | 交付 | 批次 / commit |
|---|---|---|
| OA-P0-1 余量 | 契约路径别名 73 条 + 中间件四表镜像 + 双重 parity 守卫 | R1 / `ef48f3c35` |
| OA-FI002-CI | 静态列扫描固化 + CI 棘轮（首跑挖出 48 对缺口并登记） | R2 / `b799858cf` |
| LY-PERF-3 | LoadWallet 合并（UPDATE+RETURNING，3→2 往返） | R3 / `9b2ea89d6` |
| LY-PERF-5 | requireAdmin 三查合一（3→1 往返） | R3 / `9b2ea89d6` |
| LY-GAP-2 | 后台详情改只读投影（读端点不再写库）+ 静态守卫 | R3 / `9b2ea89d6` |
| WARN-CI | warnings-gate workflow（补 llm-lab/cool/rust-lints 三块空白） | R4 / `4a14d6e82` |
| COOL-CI | jar 定源 commons-codec 1.17.1 + SHA-1 双路互证 | R4 / `4a14d6e82` |
| WF-REQUIRED | required checks 实测核对 + 精确建议清单（设置需用户拍板） | R4 / `4a14d6e82` |

### 关闭（不做，含证据）

| ID | 不做理由（证据） |
|---|---|
| LY-PERF-1 | 两处加载形态不同（5 字段投影 vs 哈希用 JSON map）；复用需重推与客户端**逐字节比对**的 S 段 → 反作弊误判风险；收益仅 13 次往返省 1 次 |
| AU-P1 异步 LLM 后端 | 勘察证实 `augment_async` **零产品消费方**（只有测试引用），异步化无收益入口 |
| AU-P2 全包惰性导入 | `test_package_exports.py` 明文禁止 PEP 562 `__getattr__`；B5-1 已拿走唯一安全切片（requests） |
| LY-5 全量 Config 原子化 | B3-1 已消真实竞态（档位入 gate）并打好结构基础；33 处引用点的结构性改动没有对应需求（无热加载场景），属无需求重构 |
| LY-7 bag.vue 虚拟化 | 列表有界（~114 行），虚拟化收益小于引入的复杂度；前端视觉改动本机无法验收 |
| LY-GAP-1 me.vue works 玩家可选 | 是否开放玩家自选是**产品决策**，不是工程缺陷（销账不符已登记） |
| OA-PERF-LIMIT 737 处无 LIMIT | 改响应形状影响前端契约，需逐端点对表 + 灌数据实测；数据量未到临界 |
| OA-MOBILE-CONTACTS | 同族（前后端联动 + 分页契约） |
| AU-PERF-POOL / AU-PERF-ANYIO | 先量后改：无压测基准，改大反而增大上游压力 |
| FI002-MISSING-COLUMNS（41 对） | 列类型须对 live DB 复核；盲写类型会让 fresh DB 出现新的类型不匹配（比现状更糟） |
| FI002-LEGACY-TABLES（7 对） | 遗留表归谁建、索引与端点语义要先定，属独立任务 |

### 本轮验证汇总（本机 + CI）

| 门禁 | 结果 |
|---|---|
| oa4rust CI（clippy / unit / integration / fi002 / behavior / openapi / contract） | **全绿**（`4a14d6e82`） |
| laiyipao CI（server+PG / miniapp / admin） | **全绿**（`4a14d6e82`） |
| auto-checkin CI / warnings-gate（新增） / three-ends / supply-chain | **全绿**（`4a14d6e82`） |
| 本机：shared 单测 167 passed；契约别名全栈 73 对全过；u2_probe 构建通过 | ✓ |
| 本机：laiyipao build/vet/gofmt + 全部静态守卫 | ✓ |
| 本机：FI-002 扫描器两方向（基线 exit 0 / 新增 exit 1） | ✓ |
| 本机：size:check 容差两方向（+1s 过 / +10s 红） | ✓ |

**本机环境限制（fail-loud，与 §13 同）**：无 PostgreSQL，DB 集成用例本机跳过；
但 CI 有真库，上述所有 DB 面已由 CI 验证。

### R6 — 修 main 上的活红：augmentor/web 类型检查（2026-10-25 深夜，排查 Dependabot PR 时发现）

**怎么发现的**：PR #102（Dependabot npm_and_yarn）大部分检查绿，但
`Docker image smoke test` 红。回查该 job 历史：10-23 全绿、10-24 起连续两次红
→ **不是依赖升级引入的，是 main 自己坏了**。

**根因**：`ai-platform-ci` 的 Docker 构建里 `npm run build` = `tsc --noEmit && vite build`，
而 `augmentor/web/src/pages/System.test.tsx` 的两个 `StatusResponse` 夹具缺
`model_generate_seconds`——该字段加进契约时只改了 `types/api.ts` 与 `System.tsx`，
**漏了带类型标注的测试夹具**（夹具文件自己的注释就写着「少一键当场 typecheck 红」，
这次正好是它自己漏了）。TS2741 × 2。

**修法**：
- 两个夹具补字段：`HEALTHY` 用 `12.5`（带小数才能钉住 `toFixed(1)` 没被截断）、
  `DEGRADED` 用 `null`（沿用「管道缺失 → null 而不是 0」的契约语义）；
- 两个用例补断言 `累计生成耗时（秒）`：`12.5` / `无后端`——让新字段真正被测到，
  而不只是「编译过了」；
- 夹具注释里记下这次事故，防止同类漏更再发生。

**顺带补上结构性缺口**：`augmentor/web` 此前**唯一的门禁**是 `ai-platform-ci` 的
Docker smoke，而那个 workflow 带 `paths: augmentor/**` 过滤——
**带 paths 的 workflow 永远不能设为 required check**（本仓 `auto-checkin-ci` 头注释里
就记着这条教训）。所以给它一个**无过滤**的快速门：`warnings-gate` 增加
`augmentor-web` 项（`npm ci` + `build && lint`，约 55s）。

**验证**：`npx tsc --noEmit` 干净；`System.test.tsx` 13 用例全过（含 2 条新断言）；
`check-warnings --only ...augmentor-web` 5 项 4 通过 1 跳过（llm-lab-tests 本机无 pytest）；
顺带把其余 5 个前端/TS 项全跑一遍（desktop 79s / mobile 34s / admin 10s /
miniapp 14s / auto-checkin 6s）**全 PASS**，确认没有第二处藏着的红。
