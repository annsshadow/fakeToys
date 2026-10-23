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
| B1-1 oa4rust clippy | | | |
| B1-2 auto-checkin 口令面板 | | | |
| B1-3 records limit 夹取 | | | |
| B1-4 seed_java.py 路径 | | | |
| B2-1 挂载 organization_assemble_personal | | | |
| B2-2 挂载 empower | | | |
| B2-3 挂载冒烟（401/404 判别） | | | |
| B2-4 merge 守卫测试 | | | |
| B3-1 竞态修复 | | | |
| B3-2 AdminUserDetail 并发化 | | | |
| B3-3 battles 相关子查询 | | | |
| B4-1 bcrypt spawn_blocking | | | |
| B4-2 program_init 事务+批量 | | | |
| B4-3 core_entity 空池移除 | | | |
| B4-4 attendance/cms 批量化 | | | |
| B5-1 requests 惰性导入 | | | |
| B5-2 generate 耗时统计 | | | |
| B5-3 diagnostics 缓存 | | | |
| B5-4 并行/串行对比例测试 | | | |
| B6-1 auto-checkin CI | | | |
| B6-2 govulncheck 钉版 | | | |
| B6-3 删死 codemod | | | |
| B6-4 check-warnings 修正扩充 | | | |
| B6-5 小额修复组 | | | |
| B6-6 behavior-compare 治理 | | | |

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
