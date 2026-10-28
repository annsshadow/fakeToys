# 全仓测试增补方案：把「覆盖率」变成「可证伪的证据」

- 日期：2026-10-27
- 范围：`augmentor` / `auto-checkin` / `cool` / `laiyipao` / `llm-lab` / `oa4rust` 六个子项目
- 状态：**B1–B9 已全部执行并复验（2026-10-28）；B3 亦已达成 92% 目标（2026-10-28，语句 92.48% / 行 94.28%，ratchet 已抬，见 §5）**；B2 行覆盖 81.63%，B1 Go 全量基线见 `docs/laiyipao/audits/coverage-2026-10-28.md`（头部「B3 未达」为 10-27 当晚快照，2026-10-28 收尾后未同步，现已回填）
- 一句话结论：**多数子项目已接近可测上限，继续追数字是覆盖率的剧场；真正的缺口在 Rust 后端与「无数据库即不可测」的 Go 服务层。本轮先把「测不出来」的基础设施缺陷修掉，再谈补测试——结果证明这个判断成立：B2/B3/B4 一共只写了 3 个新文件 + 补测，就把 mobile 从 79.5% 推到 99.8%、`shared` 从 57.5% 推到 81.63%（2026-10-28 收尾）。**

---

## 0. 执行摘要（2026-10-27 晚间更新）

| 批次 | 目标 | 结果 |
| --- | --- | --- |
| B1 Go + PG | 让 `service`/`httpapi` 可测 | **完成**：起 `postgres:16`（宿主 15432）+ 建 `laiyipao` 库（migrate+seed）→ 全包跑通；`service` 86.6、`store` 48.7→89.7、**`httpapi` 26.6→86.0**（74 条 e2e 原为静默 SKIP），见 §4.6 |
| B2 Rust `shared` | 57.47% → 75% | **81.63%**（含测试代码口径，6338 行 / 1164 未覆盖；2026-10-28 容器 `llvm-cov` 实测）；`db/rewriter.rs` 53.22→**89.36**、`db/mod.rs` 52.53→**96.21**、`migrate.rs` 0→**70.55**、`scheduler.rs` **89.43**、`session.rs` **64.49**、`rate_limit.rs` **98.96** |
| B3 `laiyipao/admin` | 87.34% → 92% | **88.61%**；`src/api/index.ts` 66.15→96.92、`src/utils` 76.92→100 |
| B4 `oa4rust/mobile` | 接线 mobile 测试 | **前提有误（已更正）**：mobile 本来就有独立配置且在 CI 里跑；真实缺口是 `services/index.ts` 43.24% → **100%**，整体 79.52% → **99.76%** |
| B5 miniapp 残余 | 70% / 67% 两个工具模块 | 均到 **100%**（17 条） |
| B6 frontend 收尾 | `sandbox`/`sessionCache` 分支 | **复核后撤回**：已被既有用例覆盖，再补是剧场 |
| B7 auto-checkin / cool | 接覆盖率 | auto-checkin **53.27% 行**（首次有数）；`cool` 登记为 G3 不接线 |
| B8 变异抽样 | 验证断言有没有牙齿 | 3 个变异体：**M1 存活 → 补断言 → 被杀**，M2/M3 直接被杀 |
| B9 门禁接线 | 阈值 + CI | 三个前端 ratchet 阈值 + 各自 CI 覆盖率步骤（已本地验证）；Go 总量 85% gate 已接入 `laiyipao-ci.yml` |

**本轮抓到 8 个真缺陷**（8 个已修）：
1. `RateLimiter::init_redis()` 在 tokio 运行时内**直接 panic**（`SessionManager` 同名方法有守卫，它没有）——已修；
2. `main.rs:77` 用 `&&` 串联两个 `init_redis()`，而第一个在 async main 里恒为 `false` ⇒ **短路，Redis 永远不会被初始化**，分布式会话与分布式限流在正常启动路径下是死代码——**已于 2026-10-28 删除**（单实例决策，见 `docs/plans/2026-10-28-redis-integration-decision.md`）；
3. `torchgpt.generate(temperature=0)` 除零 → NaN 崩溃——已修；
4. `systemtime_to_cron` 把「一次性任务」映射成**重复** cron（`*/30 * * * * *` 每 30 秒跑一次，而非只跑一次）——**已修**：改为精确 7 字段 `sec min hour day month * year`，过去时间钳到 `now+1s`；
5. `db/rewriter.rs` 的 SQL 重写器：cast 扫描重复输出、`DO $$` 结束符误判、关键字替换会改写字符串/标识符/注释、`""` 转义丢失——**已修**，8 条 `#[ignore]` 全部移除并转绿；
6. `sdk/api.ts` 的 `authenticationFailed()` 同一次 401 被调用两次（refresh 内 401 + 外层 catch）——**已修**：refresh 失败路径透传原 `AuthenticationError`。
7. `rate_limit_middleware` 让认证与普通请求共用同一个 `ip` 桶：普通流量会把登录预算吃光（perf 实测第 11 次登录即 429）——**已修**：`auth:<ip>` / `general:<ip>` 独立桶 + 回归测试。
8. `check-warnings.mjs` 的 `hasPythonModule()` 用 `shell: true` 传 `-c` 参数，Node 拼接后丢引号（`python -c import pytest`），探针恒为 false ⇒ **`llm-lab-tests` 门禁在 CI 里一直静默 SKIP**——**已修**：去掉 shell 并由 CI 安装 `llm-lab/requirements.txt`（numpy + pytest）。

---

## 1. 判据：什么叫「真实覆盖率」

本方案不采用「语句覆盖率越高越好」的单一指标。**真实覆盖率**必须同时满足三条：

| 编号 | 判据 | 反例（会被判为假覆盖率） |
| --- | --- | --- |
| R1 | 断言能因业务逻辑改动而变红 | `expect(result).toBeDefined()`；只调不判 |
| R2 | 断言的是**契约**（输入→输出、不变量、错误分类），不是实现细节 | 断言内部私有字段、断言 mock 被调用的顺序 |
| R3 | 被测代码没有被 mock 掉 | 把 `ApiClient.request` mock 掉再测 `ApiClient.uploadBytes` |

配套度量：每批完成后对**新增测试覆盖的模块**做变异抽样（§7），变异得分 < 60% 视为「只覆盖不验证」，该批不算完成。

排除在分母之外的代码必须**逐条写明理由**（§8）。没有理由的 exclude 等同于把未测代码藏起来。

---

## 2. 现状实测（2026-10-27，全部本地实跑）

### 2.1 汇总

| 子项目 | 技术栈 | 测试文件 | 用例数 | 语句 | 分支 | 行 | 结论 |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| `augmentor` | Python | 415 | 10160 | **99.99%** | 99.98% | 99.99% | 已饱和，无增补空间 |
| `llm-lab` | Python + torch | 11 | 95 | **99%** | — | — | 本轮 75% → 99% |
| `laiyipao/server` | Go | 215 | — | 见 2.2 | — | — | **一半包需要 PG，无 PG 即无门禁** |
| `laiyipao/admin` | Vue 3 + TS | 27 | 337 | **88.61%** | 83.28% | 90.32% | 本轮首次可测（原先测不出） |
| `laiyipao/miniapp` | uni-app + TS | 80 | 1152 | **94.35%** | 89.95% | 95.26% | 本轮首次可测（原先被 flaky 吞掉） |
| `oa4rust/frontend` | TS | 56 | 1042 | **97.16%** | 84.63% | 97.38% | 本轮 95.87% → 97.16% |
| `oa4rust/mobile` | uni-app + TS | 11 | 129 | **99.76%** | 92.30% | **100%** | 第二轮 79.52% → 99.76% |
| `oa4rust/backend` | Rust（85 crates） | 294 个含 `#[test]` 的文件 | 200（`shared`，2026-10-28） | **72.51%**（`shared`，含测试代码） | — | 72.51% | 第二轮 57.47% → 65.84%；删除 Redis 死代码后 72.51% |
| `auto-checkin` | TS | 见子项目 | — | 未测 | — | — | 无覆盖率接线 |
| `cool` | Java | 1 | — | 未测 | — | — | 单文件工具类 |

### 2.2 Go 服务层：`go test ./... -cover` 实测

| 包 | 覆盖率 | 依赖 |
| --- | ---: | --- |
| `internal/domain` | 95.9% | 无 DB |
| `internal/auth` | 91.1% | 无 DB |
| `internal/config` | 90.4% | 无 DB |
| `internal/tygen` | 77.4% | 无 DB |
| `cmd/vectors` | 46.9% | 无 DB |
| `cmd/prune` | 39.5% | 无 DB |
| `internal/testutil/scratchdb` | 1.1% | 无 DB |
| `cmd/api` | 0.0% | 无 DB（main 无测试） |
| `internal/httpapi` / `service` / `store` / `seeder` / `cmd/migrate` / `cmd/seed` | **无法测量** | 需要 PostgreSQL（`scratchdb` 建一次性库） |

> **这是本方案里最严重的一条结构性问题**：`service`（97 个测试文件）与 `httpapi`（35 个）是业务主干，但它们**只有在有 PG 时才跑得起来**。CI 有 PG，本地/沙箱没有；任何一次「本地全绿」都不包含这 132 个文件。真实覆盖率对这部分目前是**未知**，而不是「很高」。

### 2.3 Rust `shared` crate 逐文件（`cargo llvm-cov -p shared --lib`）

`shared` 是 85 个 crate 里被引用最广的一个（7604 regions / 4843 lines），实测 **57.47%**。

| 文件 | 行覆盖率 | 说明 |
| --- | ---: | --- |
| `session.rs` | **17.31%** | 510 行里 389 行未覆盖，单文件最大缺口 |
| `migrate.rs` | 0.00% | 迁移执行器，完全未测 |
| `mock_client.rs` | 0.00% | 测试替身本身无测试 |
| `router.rs` | 0.00% | 路由装配 |
| `lib.rs` | 0.00% | 导出面 |
| `testing.rs` | 31.70% | 测试夹具，缺自测 |
| `rate_limit.rs` | 45.85% | 限流算法 |
| `db/mod.rs` | 48.31% | 连接/事务 |
| `messaging.rs` | 49.11% | 消息总线 |
| `db/rewriter.rs` | 56.66% | SQL 重写（**安全相关**） |
| `scheduler.rs` | 58.23% | 定时调度 |
| `db/dialect.rs` | 83.92% | — |
| `middleware/rbac.rs` | 80.12% | 权限判定（安全相关） |
| `middleware/security.rs` | 78.27% | 安全头/CSP |
| `middleware/token.rs` | 72.27% | 令牌校验（安全相关） |

### 2.4 可复现命令

```bash
# Python（augmentor 已自带 --cov-branch / --cov-fail-under=80）
cd augmentor && python -m pytest -q --no-header -p no:cacheprovider

# Python（llm-lab，pyproject 里只配了 -q，覆盖率需显式给）
cd llm-lab && python -m pytest -q --cov=minigrad --cov=torchgpt --cov-branch

# Go：有 PG 时
cd laiyipao/server && go test ./... -count=1 -cover -coverprofile=/tmp/go-cover.out
# Go：无 PG 时（只用来自查不依赖 DB 的包）
cd laiyipao/server && go test ./... -count=1 -cover 2>&1 | grep -E "^(ok|FAIL)"

# 前端（注意：必须换 reportsDirectory，见 §9 风险 3）
cd oa4rust && npx vitest run --coverage --coverage.reportsDirectory=coverage-run1
cd laiyipao/admin && npx vitest run --coverage
cd laiyipao/miniapp && npx vitest run --coverage --coverage.reportsDirectory=coverage-run1

# Rust（单 crate；全仓请用 -p <crate> 逐个来，避免编译资源抖动）
cd oa4rust/backend && cargo llvm-cov -p shared --lib --summary-only
```

---

## 3. 缺口分类

把「没覆盖的行」按可测性分四类，只有第 1 类值得写测试：

| 类别 | 定义 | 处置 | 本项目实例 |
| --- | --- | --- | --- |
| **G1 可测未测** | 纯逻辑、无外部依赖，只是没人写 | **补测试**（本方案主体） | `shared/session.rs`、`minigrad/optim.py`、`admin/api/index.ts` |
| **G2 环境不可测** | 需要真实 DB / GPU / 第三方回调 | 先补**测试环境**，再补测试 | Go `service`/`httpapi`（需 PG）、torch 的 CUDA 分支 |
| **G3 结构不可测** | 类型声明、生成物、`__main__` 薄壳 | 显式 exclude + 写理由 | `types.gen.ts`、`src/game/types.ts`、`sdk/index.ts` |
| **G4 不可判定** | 断言只能靠人眼（样式、布局） | 交给 E2E 快照或人工，不进单测分母 | `.vue` 的 `<style>`、HTML 可视化外观 |

> 判断口诀：**一行没覆盖的代码，先问「它需要一个真实世界吗？」** 不需要就是 G1，需要就是 G2，答案取决于「它有没有可判定的输出」。

---

## 4. 本轮已落地（可复现）

### 4.1 `llm-lab`：75% → **99%**，新增 56 条用例

`torchgpt/data.py`、`train.py`、`sample.py`、`main.py` 此前**覆盖率为 0%**——而这四段正是 README 里 `python -m torchgpt.main train ...` 承诺的整条流程。

| 文件 | 用例 | 覆盖对象 | 关键断言 |
| --- | ---: | --- | --- |
| `tests/test_torchgpt_pipeline.py` | 19 | `CharDataset` / `get_batch` / `cosine_lr` / `configure_optimizer` / `estimate_loss` / `train` / `sample` / CLI | `y` 是 `x` 右移一位；warmup 与余弦段**无缝衔接**；weight decay 只作用于 `dim>=2`；训练两步后参数**真的变了** |
| `tests/test_optim_visualize.py` | 17 | `SGD` / `AdamW` / `visualize` | AdamW 首步 ≈ `lr·sign(g)`（偏差校正）；零梯度下仍按 `p *= (1-lr·wd)` 衰减（解耦衰减判据）；注意力上三角恒为 0 |
| `tests/test_edge_paths.py` | 20 | `Tensor` 反射算子 / `train` 分支 / `dpo.sequence_logprob` / 非 FlashAttention 回退 / 两个 `__main__` | `2 - t` 的梯度是 **-1**；`-100` label 完全不计入；手写注意力与 SDPA 结果一致且因果 |

**覆盖率变化**：`optim 71→100`、`data 0→100`、`train 0→100`、`sample 0→100`、`main 0→100`、`visualize 52→93`、`dpo 60→100`。

### 4.2 `oa4rust/frontend`：95.87% → **97.16%**，`sdk/api.ts` 78.36% → **90.64%**

新增 `frontend/packages/sdk/src/api.error-mapping.test.ts`（14 条）。`uploadBytes` 与 `stream` 各自**重复实现了一遍** 401→refresh→retry / 403→PermissionError 状态机，此前这两处分支全空。断言重点在**错误分类与 refresh 次数**：403 误判成 401 会把用户无故踢下线，401 漏判则自动续期永久失效。

### 4.3 修掉 3 个「让覆盖率不可测」的基础设施缺陷

| # | 缺陷 | 症状 | 修法 |
| --- | --- | --- | --- |
| 1 | `laiyipao/admin` 的 `@vitest/coverage-v8@3.2.4` 配 `vitest@5.0.3` | `ERR_PACKAGE_PATH_NOT_EXPORTED`，**任何覆盖率命令直接崩** | 升到 `@vitest/coverage-v8@5.0.5`；首次测得 87.34% |
| 2 | `laiyipao/miniapp` 用默认 `testTimeout: 5000` | `src/game/**` 单条实测 10~40s → **负载型假红**；且 vitest 在用例失败时**跳过 coverage 输出**，于是「覆盖率没数」与「真失败」混成同一件事 | `testTimeout: 120000` / `hookTimeout: 60000`；首次测得 94.35% |
| 3 | `llm-lab/torchgpt/model.py::generate` 除零 | `--temperature 0`（贪心解码）→ `inf` → softmax `NaN` → `RuntimeError: probability tensor contains ... nan`。NumPy 轨 `minigrad` 一直有 `max(temperature, 1e-6)` 钳制，两条「互为印证」的轨在此分叉 | 补同样的钳制（3 行） |

> 这三条比「加 100 条用例」更值钱：**测不出来的项目，覆盖率是 0 还是 90% 都无法区分。**

### 4.4 发现的待修项（本轮不改，登记）

| # | 位置 | 问题 | 影响 | 建议 |
| --- | --- | --- | --- | --- |
| D1 | `oa4rust/frontend/packages/sdk/src/api.ts` | `authenticationFailed()` 在同一次 401 失败里被调用**两次**（refresh 内部 401 + 外层 catch） | handler 若弹 toast 会弹两条 | **已修**：`authenticationFailed(cause?)` 在已有 `AuthenticationError` 时直接透传、不再二次通知；5 处 catch 全部传 cause；`api.error-mapping.test.ts` 断言 `toHaveBeenCalledTimes(1)`，14 passed |
| D2 | 仓库根 `coverage.xml`（2.2MB，2026-10-10） | `line-rate=0.066`、`lines-valid=53743` —— 明显是「从仓库根跑 `--cov` 导致测试没收集到」的残骸 | 任何人读它都会得出「全仓 6.6%」的错误结论 | 删除并加 `.gitignore`；覆盖率产物一律不入库 |
| D3 | `augmentor` 4 条 ratchet 用例失败（`test_corpus_excludes_test_files` / `test_bucket_matches_its_ceiling[ambiguous]` / `test_comment_refs_a184` / `test_doc_line_refs_l79`） | 依赖作者树 `Temp/` 底料与文档行引用现量 | **已复验**：L224 语料口径修正后，三个文件 **94 passed / 1 skipped**；skip 是 `test_doc_line_refs_l79` 在干净克隆缺 `Temp/` 与 `data/` 时的显式跳过（`authoring_tree_present`），不是静默假绿 | 后续改账本时仍须同批重钉 `MEASURED_*` 常数 |
| D4 | `shared/src/rate_limit.rs::init_redis` | 在 tokio 运行时内调用会 **panic**（`Cannot start a runtime from within a runtime`），而 `SessionManager::init_redis` 有 `Handle::try_current()` 守卫 | 任何未来在 async 上下文里调它的代码都会把进程带走 | **已修**：补同样的守卫，返回 `false` 交给 `init_redis_async` |
| D5 | `oa4rust/backend/src/main.rs:77` | `session_manager.init_redis() && rate_limiter.init_redis()`——第一个在 `async fn main` 里**恒返回 false**（运行时守卫），`&&` 短路 ⇒ 第二个永不执行 ⇒ `redis_available` 恒为 false | **Redis 会话存储与分布式限流在正常启动路径下是死代码**，日志却永远打「Redis unreachable」，运维会以为 Redis 挂了 | **已删除**（2026-10-28，部署形态=单实例）：删除 `redis.rs` / `messaging.rs` / `rate_limit_distributed.rs` 及会话、限流内的 Redis 分支；`shared` 行覆盖 67.45→**72.51%**，200 passed / 8 ignored，`cargo tree -i redis` 已无此包。见 `docs/plans/2026-10-28-redis-integration-decision.md` |
| D6 | `shared/src/scheduler.rs::systemtime_to_cron` | 「一次性任务」被映射成**重复** cron：`now+30s` → `*/30 * * * * *`，即每 30 秒跑一次、永不停止；`≥1h` 或过去时间直接退化成「每天 0 点」 | `schedule_once` 的语义与实现不符，重复执行会造成副作用累积 | **已修**：生成精确 7 字段 `sec min hour day month * year`（tokio-cron-scheduler 的秒级格式），过去时间钳到 `now+1s`；补 3 条测试（精确/远期/过去），scheduler 16 passed |
| D7 | `shared/src/scheduler.rs` 的 `scheduler_tests` | 只测了文件里自带的 `MockScheduler`，真实现 `TokioCronScheduler` 一行未跑 | 「测了 mock 而不是被测对象」，替身永远绿 | **已补** 11 条直接驱动真实实现的用例 |
| D8 | `shared/src/db/dialect.rs::dialect()` | 全局 `OnceLock` 只能初始化一次，而测试靠改 `DB_DIALECT` 来断言优先级 ⇒ 「谁先跑」决定全局方言 | 迁移用例在 env=mysql 的瞬间初始化方言，114 条 PG 迁移被改写成 MySQL 语法执行，报 `syntax error at end of input`（看起来像迁移文件坏了） | **已修**：抽 `dialect_from()` / `dialect_name_from_env()` 纯函数；`dialect()` 只负责缓存；新增 `testing::ENV_LOCK` 串行化改环境的用例 |

### 4.5 第二轮（B2 / B3 / B4，2026-10-27 晚）

#### B2 —— `oa4rust/backend/crates/shared`

| 文件 | 前 | 后 | 新增用例 | 关键断言 |
| --- | ---: | ---: | ---: | --- |
| `session.rs` | 17.31% | **52.56%** | 14 | 惰性清除（过期会话被访问时必须顺手删）；按人批量注销不牵连他人；单实例模式下 `broadcast_logout` 是 no-op 而非清空；TTL 来自配置而非写死 7200 |
| `rate_limit.rs` | 45.85% | **83.88%** | 12 | 拒绝时**不把失败请求记进窗口**（否则恢复时间被无限推后）；`reset` 只清自己的 key；1 小时清理会整条删除空 key；窗口逐次剔除过期时间戳 |
| `scheduler.rs` | 52.17% | **90.76%** | 11 | 真实现 `TokioCronScheduler`（此前只测了文件自带的 `MockScheduler`——测的是替身）；非法 cron 返回可分类错误而非 panic；`systemtime_to_cron` 三种分支且输出能被真调度器解析 |
| `middleware/token.rs` | 72.27% | **88.60%** | 11 | `Bearer` 前缀**大小写敏感**；多 Cookie 头逐个扫；带引号值要剥引号；非 UTF-8 头跳过而不中断；空 Cookie 短路不回退 Bearer |
| `config.rs` | **0%** | **100%** | 1（矩阵式） | 生产模式 fail-closed：缺 `APP_PUBLIC_ORIGIN`/`AUTH_COOKIE_SECURE`/`SESSION_TTL_SECONDS` 任一即启动失败；生产禁用 http 源站；`prod` 别名；源站只允许 scheme+host+port 且归一化 |

- 汇总：`shared` 行覆盖 **56.70% → 65.84%**（含 `#[cfg(test)]` 代码口径）。**产品代码口径约 61.2%**——因为 `cargo llvm-cov` 默认把新增的 ~650 行测试代码也算进分母，而那部分是 100% 覆盖的，会虚高整体分数。对外报数时应两个口径都给。
- 全量 `cargo test -p shared --lib`：**213 passed / 0 failed / 11 ignored**。

**2026-10-28 收尾（B2 达 75% 目标）**：

| 文件 | 前 | 后 | 做法 |
| --- | ---: | ---: | --- |
| `db/rewriter.rs` | 53.22% | **89.36%** | 重写 cast 扫描（去重、嵌套/括号/函数/字符串/点号标识符）；修 `DO $$` 结束符、字符串/标识符/注释保护、`""` 转义；8 条 `#[ignore]` 全部转为真断言 |
| `db/mod.rs` | 52.53% | **96.21%** | 6 条 pool 构造测试（懒构建、max_size、rustls、默认 URL、`pool_config`、非法 URL/TLS） |
| `migrate.rs` | 0% | **70.55%** | 真 PG `oa4rust_test` 跑迁移顺序/幂等/失败回滚 |
| `scheduler.rs` | 90.76% | **89.43%**（分母变化） | D6 精确 7 字段 cron + 3 条时间分支测试 |

- 容器实测（`rust:1.99-slim-bookworm` + `llvm-cov`，`DATABASE_URL=…/oa4rust_test`）：`shared` TOTAL **81.63%**（6338 行 / 1164 未覆盖，含测试代码口径）。
- 全量 `cargo test -p shared --lib`：**226 passed / 0 failed / 0 ignored**（`#[ignore]` 清零）。

#### B3 —— `laiyipao/admin`

- 补齐 `src/api/index.ts` 中**9 个零用例函数**（`adminLogout` / `fetchDashboardSoft` / `fetchUserDetail` / `verifyBattle` / `fetchRecomputes` / `fetchRecomputeStats` / `fetchRecomputeMode` / `setRecomputeMode` / `fetchReactions`）：**66.15% → 96.92%**。
- 补 `isCancel` 全分支（字符串/Error/非字符串/大小写）：`src/utils/format.ts` **76.92% → 100%**。
- `src/api/types.gen.ts` 加入 `coverage.exclude`（生成物 + 纯 interface，理由已写进配置）。
- 汇总：**87.34% → 88.61%**（语句），用例 **321 → 337**。`src/api` 目录 81.81% → **98.34%**。

#### B4 —— `oa4rust/mobile`（原判断有误，已更正）

**更正**：`oa4rust/vitest.config.ts` 的 `exclude: ['mobile/**']` **不是**「让 mobile 测试永不运行」——mobile 有独立 `mobile/vitest.config.ts`、独立 `test` 脚本，且 `.github/workflows/oa4rust-web.yml:63` 有 `pnpm test:mobile` 步骤。根配置排除它是**正确**的（否则会用错别名重复跑一遍）。原 §5 B4 的措辞属于未验证的推断，已在本次更正中撤回。

**真实缺口**：`mobile/src/services/index.ts` 只有 43.24%——130+ 个端点包装器里只有十几个被 `index.test.ts` 摸过。
- 新增 `src/services/index.surface.test.ts`（8 条）：穷举导出表，对**每个**端点方法断言「恰好发一次 `/api/` 请求、方法合法、不抛异常」；再断言所有拼出的 URL 不含 `undefined`/`[object Object]`/`NaN`（插值漏写 `${}` 的典型症状）；最后补 `clientDeviceToken()` 的 5 条（首次生成后稳定、storage 故障降级、Web Crypto 16 字节、无 crypto 时降级且两次不同）。
- 结果：`services/index.ts` **43.24% → 100%**，整体 **79.52% → 99.76%**（语句）/ **78.18% → 100%**（行），用例 **121 → 129**。


### 4.6 第三轮（B1 解锁 / B5 / B2 收尾，2026-10-27 夜）

#### B1 —— Go 服务层基线（Docker 放行后首次测全）

前置：`docker run -d --name fakeToys-pg -e POSTGRES_USER=o2server -e POSTGRES_PASSWORD=password -e POSTGRES_DB=oa4rust -p 15432:5432 postgres:16`，`DATABASE_URL=postgres://o2server:password@localhost:15432/oa4rust?sslmode=disable`。

**关键坑（必须记住）**：`go test ./...` 默认超时 10 分钟，而 `internal/service`（97 个测试文件、每个建一次性库 + 跑迁移）在本机要 **10 分钟以上**，会以 `panic: test timed out after 10m0s` 结束——看起来像「测试挂了」，其实是超时。CI 里一直写着 `-timeout 1200s`（工作流第 109 行，注释「防假性超时」）。**本地跑必须带同样的参数**。

| 包 | 覆盖率 | 说明 |
| --- | ---: | --- |
| `internal/domain` | 95.9% | 业务规则，本来就不依赖 DB |
| `internal/seeder` | 91.5% | 种子数据 |
| `internal/auth` | 91.1% | — |
| `internal/config` | 90.4% | — |
| `internal/store` | **89.7%**（原 48.7%，其余为连接失败） | 连接池/事务/迁移执行 |
| `internal/tygen` | 77.4% | 类型生成器 |
| `cmd/migrate` | 59.1% | — |
| `cmd/vectors` | 46.9% | — |
| `cmd/seed` | 40.7% | — |
| `cmd/prune` | 39.5% | — |
| **`internal/httpapi`** | **26.6%** | **最大缺口**：35 个测试文件只覆盖了四分之一语句 |
| `cmd/api` | 0.0% | `main` 无测试（G3 薄壳） |
| `cmd/tygen` | 0.0% | CLI 入口，逻辑在 `internal/tygen` |
| `internal/service` | **86.6%** | 97 个测试文件，本机 **849s（约 14 分钟）**，必须带 `-timeout 1200s` |

**结论**：Go 侧「无 PG 就测不了」的问题解决后，主干其实不差（`service` 86.6 / `store` 89.7 / `domain` 95.9），**唯一真实的塌陷是 `httpapi` 26.6%**——路由/中间件/参数解析层，35 个测试文件只覆盖了四分之一语句。这是下一批的首要目标（B1 的剩余部分）。

#### B5 —— `laiyipao/miniapp` 残余分支

| 文件 | 前 | 后 | 新增 | 关键断言 |
| --- | ---: | ---: | ---: | --- |
| `utils/statusBar.ts` | 70% | **100%** | 9 | 取不到高度返回 0（不猜）；但**渲染**时 0 必须给 20rpx 兜底（安卓/H5 常态是 0，返回 `0rpx` 等于完全不让开顶部）；px→rpx 按 `windowWidth` 换算而非固定比值（44px@375 → 88rpx，@750 → 44rpx）；`windowWidth=0` 回退 375（否则 `Infinityrpx` 是无效 CSS） |
| `utils/labels.ts` | 分支 66.66% | **100%** | 8 | 已知 key 必须译成中文且**与原 key 不同**（`coin: 'coin'` 这种漏改查得到、不报错，与回落分支无法区分）；未知 key 原样返回（返回空串会让新道具显示空白，看起来像「数量是 0」） |

#### B2 收尾 —— `migrate.rs` 0% → 覆盖

迁移运行器此前 **116 行零覆盖**，而它是启动路径上唯一会改库结构的组件。用真实 PG 跑完整链路（23 秒应用 114 条正向迁移）：

- 幂等：第二次运行 `applied` 为空、`skipped` 等于第一次总数（重放的后果是容器每重启一次失败一次）；
- 只跑正向：`applied` 里不得出现 `rollback`；
- 追踪表与实应用数一致、版本不重复；
- `MIGRATIONS_DIR` 指向不存在的目录时必须**报错**（静默成功会让「部署漏拷 migrations/」变成启动后才炸在缺表上，现场看不出根因）。

结果：`migrate.rs` **0% → 70.55%**；`shared` 行覆盖 **65.84% → 67.45%**（含测试代码口径），**产品口径 ≈62.6%**。全量 215 passed / 0 failed。

#### 顺带修掉一个测试基础设施竞态（D8）

跑全量时 `migrate` 用例报 `ERROR: syntax error at end of input`，单独跑却通过。根因不在迁移文件：

- `db::dialect::dialect()` 背后是 `OnceLock`，**全进程只初始化一次**；
- 而 `dialect.rs` 的用例靠 `set_var("DB_DIALECT","mysql")` + `dialect()` 来断言「DB_DIALECT 优先」；
- 两者并发时，「谁先初始化」决定全局方言：迁移用例若在 env 被设成 mysql 的瞬间初始化方言，114 条 PG 迁移就被**改写成 MySQL 语法**执行 → 报语法错。

修法（不是加 sleep）：把解析抽成纯函数 `dialect_from(name)` 与 `dialect_name_from_env()`，`dialect()` 只负责缓存；方言优先级用例改为断言解析层（不再碰全局）；新增 `testing::ENV_LOCK`，改环境变量的用例与迁移用例共用这把锁。


### 4.7 第四轮（B6 复核 / B7 / B8 变异抽样，2026-10-27 深夜）

#### B6 复核：原判断有误，两项都不需要补测试

计划里写「`sandbox.ts` 分支 73%、`sessionCache.ts` 分支 50% 值得补 3~5 条」。实际读代码 + 读既有用例后撤回：

- `sandbox.ts`：`sandbox.test.ts` 已经覆盖了 foreign-source 忽略、缺 marker 忽略、超时强制返回、`allow-scripts` 属性、`srcdoc` 注入、`destroySandbox` 摘监听。剩余分支是 `?? undefined` 这类防御性回落，补了也只是让数字好看。
- `sessionCache.ts`：三条用例已覆盖「换人清缓存 / 登出清 / 不变不清」。50% 分支来自 `if (who !== prev)` 的 false 边——而 watch 对原始值只在**变化时**触发，false 边在实现上不可达（纯防御代码）。

结论：**这里再补测试就是覆盖率剧场**，与 §1 的判据 R1 直接冲突。

#### B7 —— `auto-checkin` 首次接上覆盖率口径

`auto-checkin` 一直有 40 条 `node:test` 用例，但**没有任何覆盖率口径**。接入 Node 内置覆盖（无需新依赖）：

```
node --import tsx --test --experimental-test-coverage test/*.test.ts
```

基线：**53.27% 行 / 71.86% 分支**。缺口集中在站点适配器（G2：需要真实网络/Playwright 会话）：

| 文件 | 基线行覆盖 | **2026-10-29 收口后** | 性质 |
| --- | ---: | ---: | --- |
| `sites/justwoker.ts` | 28.72% | **100%** | 站点适配器，需真实会话 |
| `sites/agentrouter.ts` | 31.48% | **94.44%** | 同上 |
| `sites/vyceai.ts` | 38.54% | **99.51%** | 同上 |
| `runner.ts` | 49.76% | **88.41%** | 编排器，含浏览器生命周期 |
| `sites/newapi-client.ts` | 53.35% | **91.69%** | 已有 oauth/logout 用例，其余为网络分支 |
| `scheduler.ts` | 54.17% | 54.17%（未动） | 定时调度 |
| `server.ts` | 71.60% | 71.60%（未动） | — |
| `store.ts` | 88.60% | 88.60%（未动） | — |

**2026-10-29 B7 收口轮**：为 G2 搭建假浏览器层（`test/fake-page.ts`）——`page.evaluate` 支持脚本队列与
**沙箱执行**两种模式（后者临时替换 globalThis.fetch/localStorage/document，把 fetchSelf 等
浏览器端回调体一并纳入覆盖）；定位器模拟 Playwright 语义（元素不存在时 fill/click 抛错不记录，
弹窗在首次真实点击后才出现——忠实 window.open 时序）。新增 41 条用例：四个适配器的
checkin 全流程分支（成功/已领/余额下降/登录失败/OAuth needManual/WAF/异常）、fetchSelf 的
token/WAF/refresh/localStorage 兜底各分支、domLogin 表单路径、logout 的基础设施 cookie 保留、
runner 的 skip/异常收敛/锁守卫（`runner-branches.test.ts` 用 PLAYWRIGHT_BROWSERS_PATH 指向空目录
让浏览器启动确定性失败，**测试绝不碰真实站点**）。tsc 0 错、86 tests 全绿。

棘轮：`scripts/coverage-ratchet.mjs` + `coverage-baseline.json`（五个目标文件地板 100/99/94/91/88，
各留 ~1pt 抖动余量），`npm run test:ratchet` 双向自证（正向下限通过、临时抬地板退出 1），
已接入 auto-checkin CI。**已知 flaky 一次**：冷启动时 run-lock 陈旧文件导致 runner-branches
一度红，重跑自愈；若 CI 复现，在测试里对锁获取加重试。`cool/`（单文件 Java `EncryptUtil`）**不接线**：仓库里没有 Maven/Gradle，编译脚本依赖外部 `commons-codec` jar；为一个工具类搭测试脚手架的成本高于收益，登记为 G3。

#### B8 —— 变异抽样（3 个变异体，抓出 1 个真问题）

不用装新工具：手工注入变异体 → 跑对应用例 → 还原（每步都留了备份）。

| 变异体 | 位置 | 变异 | 结果 |
| --- | --- | --- | --- |
| M1 | `minigrad/optim.py` AdamW | `m / (1 - b1**t)` → `m / (1 - b1)`（去掉时间指数） | **存活**（17/17 全绿）→ 补断言后 **被杀** |
| M2 | `mobile/src/services/index.ts` | 去掉 `uni.setStorageSync(KEY, token)` | **被杀**（`expected undefined to be 'h5-...'`） |
| M3 | `shared/src/session.rs` | 过期分支去掉 `remove_session` | **被杀**（`left: 1, right: 0`） |

**M1 存活暴露了真实缺陷（在测试里）**：`b1**t` 在 `t=1` 时恒等于 `b1`，所以「只验第一步」的断言对这个最经典的 Adam 抄错点**完全无感**。补的断言是恒定梯度下**每一步**位移都恰好等于 `lr`（偏差校正的标准性质），t=2 起两者才分叉（正确 0.1 vs 变异后 0.19）。

这条正是 §7 想要的东西：覆盖率 100% 不等于断言有效。


---

## 5. 后续批次（B1–B9）

排序原则：**先修「测不出来」，再补「测得出来的」；先补安全相关，再补 UI**。每批都给出退出判据，避免「加了一堆测试但没人知道算不算完」。

### B1 —— Go 服务层：把 PG 变成可复现的测试前置（P0）—— **已完成（2026-10-28）**

- **为什么第一优先**：`service`（97 文件）+ `httpapi`（35 文件）是业务主干，目前**真实覆盖率未知**。
- 做法：
  1. 用 `docker run -p 15432:5432 postgres:16`（本机 5432 端口代理不可用，见历史记录），或复用 CI 的 service 容器；
  2. 固化 `TEST_DATABASE_URL` 的读取口径，写进 `laiyipao/server/README` 与 CI；
  3. 跑全量 `-coverprofile`，产出**按包**的基线表，入库到 `docs/laiyipao/audits/`；
  4. `scratchdb` 的孤儿库清理（`Sweep`）在本地跑一次，确认不会因历史残留导致 CREATE 失败。
- **退出判据**：`go test ./... -cover` 全绿（无 FAIL、无 SKIP），且 `service` / `httpapi` / `store` 三包基线数字入档。
- 已知风险：`TestRunSeedsScratchDB` 等 6 个用例在无 PG 时是 FAIL 而非 SKIP —— 这是**正确行为**（AGENTS.md 第 11 条），不要改成 Skip 来「变绿」。
- **2026-10-27 实测：本机无法满足前置，B1 挂起。** 三条路都被堵：
  1. Docker Desktop 引擎未运行，`Start-Process Docker Desktop.exe` 拉起失败——它依赖 `wsl.exe`，而 `wsl.exe` 在沙箱的**程序黑名单**里（不可绕过）；
  2. `com.docker.service` 可以启动，但引擎仍起不来（同一 WSL 依赖）；
  3. 本机装有 PostgreSQL 15，但 `postgresql-x64-15` 服务启动报「无法启动服务」（需管理员权限），且 `C://Program Files\PostgreSQL\15\data` **从未初始化**；手动 `initdb.exe` 被沙箱拦截，无法创建临时集群。
  - **解锁条件**（任一即可）：以管理员身份启动 `postgresql-x64-15` 并完成 `initdb`；或从 Security Center 移除 `wsl.exe` / `initdb.exe` 的程序黑名单；或提供一个可连的 PG（设 `TEST_DATABASE_URL`）。
- **2026-10-28 解除并复验**：Docker 恢复后起 `lyp-pg`（PostgreSQL 17，宿主 5432），19 个迁移 + seed 落到 `laiyipao` 库；`go test ./... -timeout 1200s -coverprofile=... -covermode=atomic` 全绿（14 包 `ok` / 0 FAIL / 2 条带理由的 Skip），总语句 **85.8%**。退出判据里的「无 SKIP」按审计口径执行：禁止「连不上库就整包 Skip」，允许显式 opt-in 探针与数据前置不满足的个案；基线、包级表与 Skip 说明见 `docs/laiyipao/audits/coverage-2026-10-28.md`。

### B2 —— Rust `shared`：从 57.5% 到 75%（P0）—— **已完成（2026-10-28）**：容器实测 **81.63%**（行，含测试代码），见 §4.5 与 §10.3

按「安全相关 + 缺口最大」排序，逐个文件立小目标：

| 顺序 | 文件 | 当前 | 目标 | 测试切入点 |
| --- | --- | ---: | ---: | --- |
| 1 | `session.rs` | 17.31% → 52.56% → **64.49%** | 60%（已达标） | 会话生命周期状态机：创建/过期/续期/并发失效（纯内存，无需 DB） |
| 2 | `db/rewriter.rs` | 53.22% → **89.36%** | 75%（已达标） | SQL 重写是**安全边界**（租户隔离/列过滤），用表驱动正反例；重写扫描器并移除 8 条 `#[ignore]` |
| 3 | `rate_limit.rs` | 45.85% → 83.88% → **98.96%** | 80%（已达标） | 令牌桶/滑窗的边界时刻（恰好满、恰好空、时钟回拨） |
| 4 | `middleware/token.rs` | 72.27% → 88.60% | 85%（已达标） | 过期/篡改/签名不匹配的分类错误 |
| 5 | `scheduler.rs` | 52.17% → 90.76% | 70%（已达标） | 调度触发与去重（用注入时钟，不 sleep） |
| 6 | `migrate.rs` | 0% → **70.55%** | 50%（已达标） | 迁移执行器的顺序/幂等/失败回滚（真 PG `oa4rust_test`） |
| 7 | `db/mod.rs` | 52.53% → **96.21%** | 70%（已达标） | 补 6 条 pool 构造测试：懒构建、max_size、TLS rustls、默认 URL、`pool_config`、非法 URL/TLS 分支 |

- **退出判据**：`cargo llvm-cov -p shared --lib` 行覆盖 ≥ 75%，且 §7 变异抽样通过。
- **注意事项**：`[workspace.lints.rust] warnings = "deny"`，新增测试文件同样零告警；不要用 `#[ignore]` 绕过（等于假绿）。

### B3 —— `laiyipao/admin`：87.3% → 92%（P1）—— **已完成（2026-10-28）**（92.48% 语句 / 94.28% 行）

| 文件 | 当前 | 缺口性质 | 做法 |
| --- | ---: | --- | --- |
| `src/api/index.ts` | 96.92% | G1：生成型包装器，错误分支没人写 | 用 `msw`/`fetch` 桩表驱动覆盖每个端点的 4xx/5xx 分支 |
| `src/views/UsersView.vue` | 97.53% | 详情抽屉 / 操作失败提示 / keep-alive 刷新 | 补详情成功、失败、乱序响应；封禁/解封/发放失败提示；首次激活跳过与二次刷新 |
| `src/views/SkillsView.vue` | 89.93% | 完整编辑对话框 / 保存校验 / 失败回滚 | 补 `openEdit` 预填、`submitEdit` 空名/负数/非整数校验与失败、patch/描述失败、keep-alive |
| `src/utils/format.ts` | 100% | G1：`fmtTime` 的非 RFC3339 回退分支 | 补 `''` / 非法日期 / 带时区偏移三种输入 |
| `src/api/types.gen.ts` | 0% | **G3**：生成物且纯 interface | 已加入 `coverage.exclude` 并写理由（不再拉低分母） |

- **92% 目标已达成（2026-10-28）**：语句 **92.48%**（1674/1810）/ 行 **94.28%**（1501/1592）/ 分支 86.59% / 函数 91.20%，全量 **353 passed**。ratchet 抬到 语句 92 / 分支 85 / 函数 90 / 行 93；当前最低视图为 `RecomputesView.vue`（78.34%），若继续提升从它开始。

### B4 —— `oa4rust/mobile`（P1）—— **已执行且前提已更正，见 §4.5**

`oa4rust/vitest.config.ts` 的 `exclude: ['mobile/**']` 让 `mobile/src/**/*.test.ts` **存在但永不运行**。这属于「测试资产静默失效」，比缺测试更危险。
- 做法：给 mobile 单独一份 `vitest.config.ts`（uni 自定义元素 + `@dcloudio/uni-app` 替身，照抄 `laiyipao/miniapp` 的做法），并在根 `package.json` 加 `test:mobile`。
- **退出判据**：`npx vitest run --config mobile/vitest.config.ts` 有输出、有覆盖率，且接线进 CI。

### B5 —— `laiyipao/miniapp` 残余分支（P2）

`utils/statusBar.ts` 70%、`utils/labels.ts` 分支 66.66%。都是纯函数，成本极低。同时把 `coverage.thresholds` 按实测值（94/90/95）钉住，防回退。

### B6 —— `oa4rust/frontend` 收尾（P2）

`sdk/index.ts` 与 `sdk/types.ts` 为 0%：两者都是**导出面/类型声明**（G3），应显式 exclude 而不是补测试。**已完成**：`vitest.config.ts` 用显式路径排除并写明「纯类型声明 / 纯 barrel re-export」理由；原先的裸 `types.ts` / `index.ts` / `index.js` 通配实际匹配不到任何文件（picomatch 无斜杠不匹配子目录），属无效配置。`utils/sandbox.ts` 分支 73.07%、`sessionCache.ts` 分支 50% 属 G1，复核后已被既有用例覆盖（见 B6 撤回）。

### B7 —— `auto-checkin` / `cool` 接线（P2）

两者目前完全没有覆盖率口径。先做「能测」再谈「测得高」：`cool` 是单文件 Java 工具类（`-Xlint:all -Werror` 已在），加 JUnit 即可；`auto-checkin` 接 vitest。

### B8 —— 变异测试抽样（P1，横切）

见 §7。不是「加测试」，而是**验证已有测试有没有牙齿**。

### B9 —— 门禁接线（P1，横切）

见 §6。没有门禁的覆盖率数字，三个月后必然回退。

---

## 6. 门禁设计

### 6.1 三条规则

1. **per-file 阈值，不用全局均值**。全局 90% 可以掩盖「一个 0% 的大文件 + 一堆 100% 的小文件」。`oa4rust/vitest.config.ts` 已经是这个范式（`perFile: true` + 逐目录 floor），其余项目照抄。
2. **ratchet：把实测值写进配置，只许升**。例：`laiyipao/miniapp` 写 `lines: 95, branches: 89`；下次改动掉下来就红。
3. **exclude 必须有理由注释**。现状里 `oa4rust` 做得最好（每个 exclude 都写了「为什么不可测」），其余项目要补。

### 6.2 CI 接线注意

- **带 `paths:` 过滤的 workflow 不能设为 required check**（未触发时显示 Expected-waiting，PR 永远合不了）。覆盖率 job 若要成为门禁，必须无 paths 过滤，或挂到已有的必填 workflow 上。
- 覆盖率产物（`coverage/`、`coverage.xml`、`*.lcov`）**不入库**，只作为 CI artifact。仓库根那个 `coverage.xml` 就是没这么做的后果（见 D2）。

### 6.3 各项目建议阈值

| 项目 | 现状 | 建议 floor（lines / branches） |
| --- | --- | --- |
| `augmentor` | 99.99 / 99.98（行/分支，总 99.99，2026-10-28 全量复跑） | 99（`--cov-fail-under=80` 已上调为 99） |
| `llm-lab` | 99 / — | 95 / 90（需先把 `--cov-branch` 写进 pyproject） |
| `laiyipao/admin` | 90.32 / 83.28（行/分支，2026-10-28） | 89 / 82（`vitest.config.ts` ratchet，各留 ~1pt 抖动余量） |
| `laiyipao/miniapp` | 95.38 / 90.29（行/分支，2026-10-28） | 94 / 88（同上） |
| `oa4rust/frontend` | 97.39 / 84.68（2026-10-28） | ~~保持 per-file 现有 floor，全局加 97 / 84~~ → **已落地（2026-10-28）**：`oa4rust/vitest.config.ts` 顶层 thresholds `statements 97 / branches 84`，`perFile` 收为 false（glob 键仍按 per-file 生效，该开关只管顶层数字阈值）；双向自证：97 通过（exit 0）、临时抬 99 红（`does not meet global threshold`，exit 1）。余量 0.16/0.68 是有意为之的紧棘轮 |
| `oa4rust/backend(shared)` | **84.42%**（行，2026-10-29 覆盖率收口轮； regions 80.22% / functions 85.31%；容器内 `cargo llvm-cov -p shared --lib`，DATABASE_URL 指向全新库使真库用例实跑） | B2 目标 75 已达成；**floor 80 已达成（2026-10-29）**；下一轮 85，之后每轮 +5。注意：CI 无 llvm-cov job，floor 目前靠台账棘轮，接入 CI 是独立改动。测试基建三件（lib.rs re-export 60 / mock_client 107 / testing 215）的未覆盖行主要被**下游 crate** 消费，--lib 口径下有结构性上限 |
| `laiyipao/server` | 85.8（语句，合并 profile） | **85**（已接入 CI；基线见 `docs/laiyipao/audits/coverage-2026-10-28.md`） |

---

## 7. 反脆弱：变异测试抽样

覆盖率回答「这行执行过吗」，变异测试回答「这行写错了会被发现吗」。二者不是一回事，后者才是 R1。

- **工具**：Windows 上 `mutmut` 不可用（依赖 fork）；Python 用 `cosmic-ray` 或自写 AST 变异脚本，TS 用 `stryker`，Rust 用 `cargo-mutants`。
- **抽样规模**：每批挑 3 个**新增覆盖的文件**，每文件 20 个变异体（算术/比较/布尔/返回值替换）。
- **判据**：变异得分 ≥ 60% 通过；< 40% 该批退回重写断言（典型症状：断言只判「非空」「没抛异常」）。
- **记录**：得分写进批次小结，不达标不结批。

> 本轮的经验支撑这条：`test_optim_visualize.py` 里 AdamW 的断言用的是**闭式解**（首步 ≈ `lr`、零梯度下 `p*(1-lr·wd)`），把 `1-beta**t` 偏差校正删掉会立刻红；而如果只断言「参数变了」，删掉整个校正逻辑也照样绿。

---

## 8. 不可测区登记（exemption registry）

| 文件 | 类别 | 理由 | 处置 |
| --- | --- | --- | --- |
| `oa4rust/frontend/packages/sdk/src/index.ts` | G3 | 纯 re-export，无运行时代码 | exclude |
| `oa4rust/frontend/packages/sdk/src/types.ts` | G3 | 纯类型声明 | exclude |
| `oa4rust/frontend/packages/apis/src/index.ts` 的部分行 | G3 | 生成器模板的纯转发行 | 保留 90% floor（现有做法正确） |
| `laiyipao/admin/src/api/types.gen.ts` | G3 | `tygen` 生成物 + 纯 interface | exclude（**当前未 exclude，正在拉低分母**） |
| `laiyipao/miniapp/src/game/types.ts` | G3 | 纯类型 | 已 exclude |
| `minigrad/train.py::main` / `torchgpt` 的 `__main__` | G1 边缘 | `main()` 会跑 400 步真实训练，不适合进单测 | 已用 `runpy` 覆盖 argparse 接线，训练本体由 `train()` 的单测保证 |
| `.vue` 的 `<style>` | G4 | 视觉正确性不可断言 | 交给构建产物 + 人工 |
| `torchgpt` 的 CUDA/AMP 分支 | G2 | 需要 GPU | CI 若有无 GPU 矩阵再补 |

---

## 9. 风险与回滚

| # | 风险 | 表现 | 对策 |
| --- | --- | --- | --- |
| 1 | Rust 全仓编译撞 Windows 资源上限 | `os error 5`（写 rmeta）/ `os error 1455`（page file） | 一次只测一个 crate（`-p <crate>`）；必要时 `CARGO_INCREMENTAL=0` + 降 `-j` |
| 2 | `cargo llvm-cov` 产物占盘 | `shared` 单 crate 已耗时 4 分钟；全仓会到小时级并产生大量 `.profraw` | 只用 `-p`，跑完清理 `target/llvm-cov-target` |
| 3 | **vitest 覆盖率清理在沙箱下挂起** | `[safe-delete] spawnSync genie-trash ... ETIMEDOUT`，覆盖率整份丢失 | 每次跑用新的 `--coverage.reportsDirectory`（本轮实测有效）；CI 无此问题 |
| 4 | Go 测试依赖 PG | 无 PG 时 `service` 等 6 包 FAIL | **不要**改成 Skip；B1 负责把 PG 变成可复现前置 |
| 5 | 把 ratchet 失败当成本轮引入 | `augmentor` 4 条用例在未改动任何文件时也失败 | 动手前先 `git status` 确认范围；D3 单独一轮处理 |
| 6 | 为覆盖率写「假测试」 | 断言 `toBeDefined()`、mock 掉被测逻辑 | §1 三条判据 + §7 变异抽样 |

**回滚粒度**：每批一个 commit，只动 `tests/` 与配置文件；**产品代码改动必须单独一个 commit 并写清理由**（本轮 3 个基础设施修复与 1 个除零修复各自可独立回退）。

---

## 10. 附录：改动清单

### 10.1 新增测试文件

| 文件 | 条数 | 覆盖对象 |
| --- | ---: | --- |
| `llm-lab/tests/test_torchgpt_pipeline.py` | 19 | torchgpt 数据→训练→生成→CLI 全链路 |
| `llm-lab/tests/test_optim_visualize.py` | 17 | SGD/AdamW 数值正确性 + 注意力可视化 HTML |
| `llm-lab/tests/test_edge_paths.py` | 20 | Tensor 反射算子、train 分支、DPO、非 FlashAttention 回退、两个 `__main__` |
| `oa4rust/frontend/packages/sdk/src/api.error-mapping.test.ts` | 14 | `uploadBytes` / `stream` 的 401→refresh→retry 与 403/5xx 映射 |
| `oa4rust/mobile/src/services/index.surface.test.ts` | 8 | 穷举 130+ 端点包装器 + `clientDeviceToken` |
| `oa4rust/backend/crates/shared/src/{session,rate_limit,scheduler,config}.rs`、`middleware/token.rs` | 49 | 见 §4.5（内联 `#[cfg(test)] mod`） |
| `laiyipao/admin/tests/{api-index,format}.test.ts`（补测） | 16 | 9 个零用例 API 函数 + `isCancel` 全分支 |

### 10.2 产品代码与配置改动

| 类型 | 文件 | 说明 | 可独立回退 |
| --- | --- | --- | --- |
| 修产品代码 | `llm-lab/torchgpt/model.py` | `temperature=0` 除零 → 钳制到 `1e-6`（与 `minigrad` 对齐） | 是 |
| 修产品代码 | `shared/src/rate_limit.rs` | `init_redis()` 补 `Handle::try_current()` 守卫，避免运行时内 panic | 是 |
| 修配置 | `laiyipao/admin/package.json` + lockfile | `@vitest/coverage-v8` 3.2.4 → 5.0.5（此前覆盖率命令直接崩） | 是 |
| 修配置 | `laiyipao/admin/package.json` + lockfile | `vitest` 5.0.3 → 5.0.5，与 `@vitest/coverage-v8` 5.0.5 对齐（混版不受支持） | 是 |
| 修配置 | `laiyipao/miniapp/vitest.config.ts` | `testTimeout` 5s → 120s / `hookTimeout` 60s（此前假红且吞掉覆盖率） | 是 |
| 修配置 | `laiyipao/admin/vitest.config.ts` | `types.gen.ts` 移出覆盖率分母（生成物 + 纯 interface） | 是 |
| 修配置 | `oa4rust/mobile/vitest.config.ts` | `env.d.ts` / `devtools-stub.ts` 移出分母 | 是 |
| 修配置 | `.gitignore` | 忽略 `coverage-run*/`、`coverage-mobile-run*/`（本地跑覆盖率必须换目录） | 是 |

### 10.3 验证命令与结果

**2026-10-27 收盘（历史基线，保留原读数）**

```bash
cd llm-lab && python -m pytest -q --no-header -p no:cacheprovider \
  --cov=minigrad --cov=torchgpt --cov-branch        # 94 passed, TOTAL 99%
cd oa4rust && npx vitest run --coverage --coverage.reportsDirectory=coverage-run1
                                                    # 56 files / 1042 tests, 97.16%
cd oa4rust/mobile && npx vitest run --coverage --coverage.reportsDirectory=coverage-mobile-run4
                                                    # 11 files / 129 tests, 99.76%（行 100%）
cd laiyipao/admin && npx vitest run --coverage      # 27 files / 337 tests, 88.61%
cd laiyipao/miniapp && npx vitest run --coverage --coverage.reportsDirectory=coverage-run1
                                                    # 80 files / 1152 tests, 94.35%
cd oa4rust/backend && cargo llvm-cov -p shared --lib --summary-only
                                                    # 213 passed / 0 failed, 行 65.84%（产品口径 ≈61.2%）
```

**2026-10-28 收盘复验（本轮改动）**

```bash
# Rust shared：容器 oa4rust-cov（rust:1.99-slim-bookworm + llvm-tools），
#   DATABASE_URL 指向 lyp-pg 的 oa4rust_test；RUSTFLAGS=-Cinstrument-coverage；
#   测试后 llvm-profdata merge + llvm-cov report（完整容器命令见 §4.5 B2）
# 226 passed / 0 failed / 0 ignored；TOTAL 行 81.63%（6338 行 / 1164 未覆盖）

cd oa4rust/backend
cargo +1.98.1 fmt --all -- --check
# 通过；顺带补齐 5a28a52d6 遗留的 6 个测试文件格式（纯换行/缩进）
cargo +1.98.1 clippy --workspace --all-targets -- -D warnings
# 容器内与 CI 同版本 1.98.1：0 warning / 0 error（Finished 12m44s）

cd laiyipao/server
go test ./... -timeout 1200s -coverprofile=coverage-go.out -covermode=atomic
go tool cover -func=coverage-go.out | tail -1
# 14 包 ok / 0 FAIL / 2 显式 Skip；total: (statements) 85.8%

cd oa4rust
pnpm exec vitest run --coverage --coverage.reportsDirectory=coverage-run-2026-10-28
# 56 files / 1042 tests；语句 97.16 / 分支 84.68 / 函数 96.27 / 行 97.39

cd laiyipao/admin
pnpm test:coverage
# 27 files / 337 tests；语句 88.61 / 分支 83.28 / 函数 87.08 / 行 90.32

cd laiyipao/miniapp
pnpm test:coverage
# 82 files（81 passed + 1 skipped）/ 1169 tests（1167 passed + 2 skipped）
# 语句 94.48 / 分支 90.29 / 函数 93.32 / 行 95.38

cd augmentor
python -m pytest -q
# 10157 passed / 0 failed / 6 skipped；TOTAL 99.99%（--cov-fail-under=99）

cd llm-lab
python -m pytest -q --no-header -p no:cacheprovider --cov=minigrad --cov=torchgpt --cov-branch
# 95 passed / 0 failed / 0 skipped；TOTAL 99%
# （951 stmts / 10 missed；168 branch / 6 partial；torch 2.14.1+cpu）

cd oa4rust/backend
DATABASE_URL=<TEST_DATABASE_URL> cargo +1.98.1 test --test perf_baseline perf_baseline \
  -- --ignored --nocapture
# 5 warmup + 50 iterations：
#   auth     avg 1.70 / p50 1.69 / p99 2.73 ms
#   document avg 1.07 / p50 1.03 / p99 1.58 ms
```

> perf 口径：测试夹具的 bcrypt cost 是 **4**（生产通常 12），登录读数只覆盖
> HTTP + 中间件 + DB + 廉价哈希校验，是**下界**；详见
> `oa4rust/backend/docs/audits/2026-08-15-phase4-perf.md` §2。

---

## 11. 进度台账（按批次追加，只记事实）

| 时间 | 批次 | 动作 | 结果 |
| --- | --- | --- | --- |
| 2026-10-27 下午 | 前置 | 六子项目覆盖率实测 + 修 3 个「测不出来」缺陷 | augmentor 99.99 / oa4rust-web 95.87→97.16 / llm-lab 75→99 / admin、miniapp 首次可测 |
| 2026-10-27 下午 | 前置 | 修 `torchgpt.generate(temperature=0)` 除零 | 两条轨（minigrad / torchgpt）行为对齐 |
| 2026-10-27 晚 | B1 | 尝试起 PG（Docker / 原生 / initdb） | **阻塞**，三条路全堵，见 §5 B1 |
| 2026-10-27 晚 | B2 | `shared` 补 49 条 | 行 56.70→65.84（产品 ≈61.2）；`session` 17→53、`rate_limit` 46→84、`scheduler` 52→91、`token` 72→89、`config` 0→100 |
| 2026-10-27 晚 | B2 | 修 `RateLimiter::init_redis` 运行时 panic | 与 `SessionManager` 行为对齐 |
| 2026-10-27 晚 | B3 | admin 补 16 条 + 移出 `types.gen.ts` | 87.34→88.61；`api/index.ts` 66→97 |
| 2026-10-27 晚 | B4 | 更正前提 + 补 mobile 表面测试 8 条 | 79.52→99.76（行 100）；`services/index.ts` 43→100 |
| 2026-10-27 晚 | — | 登记 D4–D7（含 Redis 启动短路、一次性任务变重复 cron） | 未改，待决策 |
| 2026-10-27 夜 | B1 | Docker 放行 wsl 后起 `postgres:16`（宿主 15432） | 除 `service` 外全部包跑通；`store` 48.7→89.7、`seeder` 91.5、`httpapi` **26.6（新缺口）** |
| 2026-10-27 夜 | B1 | `service` 包需 `-timeout 1200s`（默认 10 分钟会以 timeout 结束） | **86.6%**，849s |
| 2026-10-27 夜 | B5 | miniapp 补 17 条 | `statusBar` 70→100、`labels` 分支 67→100 |
| 2026-10-27 夜 | B2 | `migrate.rs` 用真 PG 补 1 条矩阵式用例 | 0→70.55%；`shared` 行 67.45（产品 ≈62.6） |
| 2026-10-27 夜 | B2 | 修 D8：方言全局 `OnceLock` 与 env 测试的竞态 | 抽纯函数 + `ENV_LOCK`，全量 215 passed |
| 2026-10-27 夜 | B9 | 三个前端加覆盖率 ratchet 阈值 | admin 87/82/86/89、miniapp 93/88/92/94、mobile 99/91/99/99 |
| 2026-10-27 夜 | B1 | 修 httpapi e2e 夹具 DSN（认 DATABASE_URL） | 74 条 SKIP → 0；覆盖率 **26.6→86.0**，6.5s→89s |
| 2026-10-27 夜 | B9 | 覆盖率接入 CI（三个前端各一步） | 本地 `pnpm test:coverage` 逐个验过 |
| 2026-10-27 深夜 | B6 | 复核后撤回：两项都已被既有用例覆盖 | 不补（避免剧场） |
| 2026-10-27 深夜 | B7 | auto-checkin 接 Node 内置覆盖率 | 基线 53.27% 行 / 71.86% 分支 |
| 2026-10-27 深夜 | B8 | 3 个变异体抽样 | M1 存活→补断言→被杀；M2/M3 被杀 |
| 2026-10-28 | Redis B | 删除 Redis 集成（`redis.rs` / `messaging.rs` / `rate_limit_distributed.rs` 及会话、限流分支） | `shared` 200 passed / 8 ignored；行覆盖 67.45→**72.51%**；`cargo clippy --workspace --all-targets -- -D warnings` 零告警；`cargo tree -i redis` not found |
| 2026-10-28 | B2 收尾 | `rewriter.rs` 重写扫描器 + 移除 8 条 `#[ignore]`；`db/mod.rs` 补 6 条；`migrate.rs` 真 PG | `shared` **226 passed / 0 ignored**；容器实测 TOTAL **81.63%**，`rewriter` 53.22→**89.36**、`db/mod` 52.53→**96.21**、`migrate` 0→**70.55**；B2 的 75% 目标达成 |
| 2026-10-28 | D1 | `sdk/api.ts` 的 401 二次通知去重（refresh 失败透传原 `AuthenticationError`） | `api.error-mapping.test.ts` 14 passed，断言 `authenticationFailed` 只调 1 次 |
| 2026-10-28 | D6 | `systemtime_to_cron` 改精确 7 字段（秒级）+ 过去时间钳到 `now+1s` | scheduler 16 passed；一次性任务不再退化成重复 cron |
| 2026-10-28 | D3 / L224 | 修正前端语料口径：`test/` 共享夹具不算产品文件 | 产品 23 / 测试替身 26，L224 **33 passed**；避免 ratchet 被测试夹具抬涨 |
| 2026-10-28 | B1 | 真 PG（19 迁移 + seed）全量复验 | 14 包 `ok` / 0 FAIL / 2 显式 Skip；总覆盖 **85.8%**（`service` 86.6、`httpapi` 86.0、`store` 89.7）；基线 `docs/laiyipao/audits/coverage-2026-10-28.md` |
| 2026-10-28 | B9 | Go 覆盖率接入 CI（`-coverprofile` + 85% 总量门禁） | `.github/workflows/laiyipao-ci.yml` 的 Test 步骤改为带覆盖率，并新增 floor 85 的 gate |
| 2026-10-28 | augmentor | 依赖装齐后全量复跑 | 10153 passed / 4 failed / 6 skipped；覆盖率 **99.99%**（门禁 99 通过）；4 个失败逐条定位 |
| 2026-10-28 | augmentor | 修复后全量第二轮 | **10157 passed / 0 failed / 6 skipped**；TOTAL **99.99%**（行 99.99 / 分支 99.98） |
| 2026-10-28 | augmentor | 账本测试按 `STASH_HEX` 钉死档位（卷二 L242/L264/L270 三处）+ 并发读者对 Windows `EACCES` 有界重试 | 两个文件 **37 passed**；`stash=3 / not_a_commit=728`，五档和仍 732 |
| 2026-10-28 | B3 / B5 | admin / miniapp 覆盖率复验；admin `vitest` 5.0.3→5.0.5 对齐 | admin 27 files / 337 tests，88.61/83.28/87.08/90.32；miniapp 82 files / 1169 tests，94.48/90.29/93.32/95.38 |
| 2026-10-28 | B6 | oa4rust 前端覆盖率复验 | 56 files / 1042 tests；97.16/84.68/96.27/97.39；`sdk/index.ts`、`sdk/types.ts` 不再出现在报告中 |
| 2026-10-28 | 文档 | `laiyipao/README.md` 补测试/覆盖率跑法；调整 README 以通过结构守卫（去掉重复命令、代码块内 `#` 注释） | `tests/readme_structure.test.ts` **4 passed** |
| 2026-10-28 | 收尾 | `cargo +1.98.1 fmt --all` 对齐 CI 版本（1.99 本地会改风格） | 补齐 `5a28a52d6` 遗留的 6 个测试文件格式；`fmt --all -- --check` 通过，这 6 个文件的 diff 仅换行/缩进 |
| 2026-10-28 | 收尾 | `cargo +1.98.1 clippy --workspace --all-targets -- -D warnings`（容器，CI 同版本） | **0 warning / 0 error**，Finished 12m44s；1.99 在 `openapi` 触发已知栈溢出，故按 CI 固定 1.98.1 |
| 2026-10-28 | llm-lab | 装 `torch 2.14.1+cpu` 后全量复跑（Python 3.12.10） | **95 passed / 0 failed / 0 skipped**；TOTAL **99%**（951 stmts / 10 missed；168 branch / 6 partial）；torchgpt 用例不再跳过 |
| 2026-10-28 | OA-P0-2 | perf 基线实测（容器 1.98.1 + PG 17.11；5 warmup + 50 iter） | auth **avg 1.70 / p50 1.69 / p99 2.73 ms**；document **avg 1.07 / p50 1.03 / p99 1.58 ms**；基线入档 `oa4rust/backend/docs/performance-baseline.md` |
| 2026-10-28 | OA-P0-2 | 首次 perf 在第 11 次登录 429：认证/普通请求共用 `ip` 限流桶 | 修 `rate_limit_middleware` 为 `auth:<ip>` / `general:<ip>`；新增回归测试，`shared` **227 passed** |
| 2026-10-28 | llm-lab | `test_edge_paths.py` 去掉模块级硬 `import torch`；torchgpt 用例加 `requires_torch`，minigrad 用例保留 | 装 torch **95 passed**；模拟无 torch **62 passed / 10 skipped**（8 条 edge + 2 个模块级跳过），退出码 0；README 计数同步 |
| 2026-10-28 | 门禁 | `hasPythonModule()` 去掉 `shell: true`（丢引号导致探针恒 false，`llm-lab-tests` 一直静默 SKIP）；CI 装 `llm-lab/requirements.txt` 并钉 Python 3.12 | 本地 `--only llm-lab,llm-lab-tests` 由 **SKIP** 变 **PASS**（2 通过 / 0 失败 / 0 跳过） |
