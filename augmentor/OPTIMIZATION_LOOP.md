# augmentor 优化循环进度追踪（2026-09-23 启动，目标 100 轮）

约定（沿用 `.backups/opt100_progress.md` 那次的做法）：每轮 = 一次「实测定位 → 整改 →
全量测试 → 提交」，红→绿必须先验（把改动退回缺陷态，新用例必须失败），全绿才提交。

- 基线（L0 起点）：**3668 passed / 3 skipped，覆盖率 98.55%**，`pytest -q` 约 50–95 s
- 测试解释器：`C:/Users/Administrator/.workbuddy-ai/binaries/python/envs/aug/Scripts/python.exe`，
  cwd = `augmentor/`，`PYTHONIOENCODING=utf-8`
- 硬约束：不往仓库工作目录写产物；不擅自搬动用户数据；破坏性默认值变更需逐项确认

## 已完成

- [x] **L1** `fix(api)`: F-09 依赖注册表缺省目录跟白名单走 + 测试不再污染版本树（`d5601369b`）
- [x] **L2** `fix(api)`: F-10 备份目录缺省值、F-11 服务配置文件路径收敛到 `deps.config_file_path()`（`13ec11eaa`）
- [x] **L3** `perf(analytics)`: A1 多样性均值提出生成器 → 长度方差 O(n²)→O(n)
- [x] **L4** `perf(api)`: A2 白名单两条来源都按「值会变的东西」建缓存 → 7.06 ms/次 → 0.19 ms/次
- [x] **L5** `perf(api)+fix`: A4 12 条端点的数据集读取离开事件循环（14 处读取点）
- [x] **L6** `perf(impact)`: A14 `duplicate_rate` 的 `list.count` 每条扫全表 → `Counter` 一次计数，O(n²)→O(n)

## Backlog A — 性能（含 file:line 与实测线索）

| # | 位置 | 问题 | 量级 |
|---|------|------|------|
| ~~A1~~ | ~~`augmentor/analytics.py:368`~~ | ~~均值在生成器里重算 → O(n²)；实测 n=20000 时 **1043 ms vs 2 ms**~~ 已修（L3） | S |
| ~~A2~~ | ~~`api/deps.py` `allowed_data_roots()`~~ | ~~每个路径参数都 `load_config()` 重新解析 YAML，实测 **6.8 ms/次**~~ 已修（L4，含环境变量分支 0.29 ms/次） | S |
| A3 | `augmentor/analytics.py:403-412` | `get_duplicate_candidates()` 纯 O(n²) 且每对重建字符集合 | M |
| ~~A4~~ | ~~`api/routes/quality.py:141,185,217,253,285,309,370,403` + `audit.py:31,32` `leakage.py:46,47` `privacy.py:52` `export.py:132`~~ | ~~`async def` 里同步 `load_items()` 阻塞事件循环，应走 `run_in_thread`~~ 已修（L5；`export.py:109` 本就在线程内，非缺陷） | M |
| A5 | `augmentor/pipeline.py:205-210` + `quality.py:133,153` | 逐条 `encode()`/`predict()` 打分，已有 `batch_score()` 却没用 | M |
| A6 | `augmentor/dedup.py:94-123,242-290` | 回退路径构造 **稠密 float64** n×vocab 矩阵（大词表 OOM）；应用稀疏 CSR / faiss range_search | L |
| A7 | `augmentor/search_enhanced.py:448,82` | 每次查询重建索引（n=3000 重建 8 ms vs 查询 2 ms），且 contains/fuzzy 不查索引 | M |
| A8 | `api/routes/data.py:146-164` | 整文件解析 + 全量过滤后再切片，无早停 | M |
| A9 | `augmentor/tracker.py:114` → `:87-99` | 每个指标点整文件重写 → O(points²) 字节 | S |
| A10 | `augmentor/quality.py:175-179` | 多样性回退里参照文本 n-gram 集合反复重建 | S |
| A11 | `augmentor/indexer.py:117-123,345,352,372-399` | 每次 `DatasetView` 操作重建全部索引（filter 26 ms @ n=3000） | M |
| A12 | `augmentor/validation.py:217-222`、`sampler.py:296-299` | 循环内未缓存的正则、`list.index`/`in` 线性扫描 | S |
| A13 | `api/routes/dataset_tools.py:311,360,386,387,416,436,566,598` + `system_ops.py:320,348,514` | **A4 的同构族**：`read_items()`（同步版）在 11 个 `async def` 路由体里直接调用，同样占着事件循环；`dataset_tools.py:566` 还是「多个文件在循环里串行读」 | M |
| ~~A14~~ | ~~`augmentor/impact.py:66`~~ | ~~`duplicate_rate` 里 `texts.count(t)` 写在推导式中 → O(n²)~~ 已修（L6） | S |

## Backlog B — 功能增强（价值 ÷ 工作量）

| # | 内容 | 证据 | 量级 |
|---|------|------|------|
| B1 | 把 `/api/system/*`(13) 与 `/api/dataset/*`(12) 接入 UI | `web/src/services/api.ts` 对二者零引用；`System.tsx:3` 只用了 status | M |
| B2 | Excel/CSV 摄取端到端（上传 + `convert --input-format xlsx`） | SDK `csv_excel_import.py:61,152` 可达性为零；`data.py:236-243` 只认 JSON | M |
| B3 | 反向转换边 `alpaca/sharegpt/chatml/vicuna/belle/tsv → json` | `converter.py:60-66` 只读 json/jsonl/csv，导出侧 13 种 → 不可回环 | M |
| B4 | `QualityGate` / `DatasetHealthScore` 暴露为 CLI 子命令 + 端点 | 两者在 `augmentor/__init__.py:38+` 导出但无人可达 | S |
| B5 | 死配置项：读取或显式废弃（`config.py:75-167` 的 sampler/expander/tracker/visualization/multilingual/evaluation/vector/active_learning 等） | 无代码路径读取 | M |
| B6 | `/api/dataset/evaluate`（BLEU/ROUGE）+ `/api/dataset/impact`（前后对比） | `evaluation.py:74,155,211`、`impact.py:49,96` 纯函数已就绪 | S |
| B7 | 主动学习 + 实验跟踪回路（`active_learning.py:46`、`tracker.py:29`） | 公开导出但无 CLI/API | L |
| B8 | 前端 5 个未接线服务函数 + 运行时 schema 校验 | `api.ts:141,152,175,180,206` | M |

## 循环日志

（每轮追加一行：`L<n>` 提交哈希 · 做了什么 · 实测数字 · 测试计数）

- **L3** `e7e051279` `perf(analytics)` A1 —— `_calculate_diversity_score()` 长度方差把均值提出生成器，
  O(n²)→O(n)。n=20000 实测 **1147.7 ms → 2 ms**（不带 trace）/ **1130 ms → 143 ms**（带 trace）。
  新增 `tests/unit/test_analytics_performance.py` 2 例：一条锁语义（手算 0.6/0.4 权重对照），
  一条锁量级（预算 500 ms；docstring 说明预算为何按**带覆盖率 trace** 的实测值取，而不是 2 ms）。
  红→绿：把均值塞回生成器 → 预算用例红在 1130.3 ms。
- **L4** `7cca1596a` `perf(api)` A2 —— `allowed_data_roots()` 两条来源都加缓存：config 分支
  `(路径, mtime_ns)` 键、环境变量分支 `(原值, os.getcwd())` 键。实测 7.06 ms/次 → **0.19 ms/次**、
  0.29 ms/次 → **0.0006 ms/次**。新增 `TestConfigDataRootsCache` 4 例 + `TestEnvDataRootsCache` 4 例；
  `docs/ARCHITECTURE.md` §3.12.1 补「缓存键都带值会变的东西」一节。红→绿两处注入：
  ①缓存键去掉 cwd → 相对根目录换目录用例红（`first/data` ≠ `second/data`）；
  ②关掉缓存读 → 「200 次查询解析了 201 次配置」红。
- **L5** `a97d57710` `perf(api) + fix` A4 —— 12 条端点 / 14 个读取点在事件循环上同步 `load_items()`，
  改为 `await run_in_thread(load_items, ...)`。这条**是缺陷不只是优化**：`api/deps.py` 里
  `load_items` 的文档串写着「供线程内使用」（同步版），异步版是 `read_json_file`；
  这些路由把分析离线了、却把第一行读取留在循环上，等于每个请求让全站停 15 ms
  （实测 3.6 MB / 6902 条 `train_data.json` 读一次 15.1 ms）。
  新增 `tests/integration/test_api_event_loop_blocking.py` 24 例：预言机用**线程身份**
  （`httpx.ASGITransport` 在当前线程跑循环，读取落在线程池还是循环上是确定的）而非计时，
  第二条用例锁响应仍 200，防止「干脆不读了」把第一条糊过去。
  并发证据（人为把读取放慢到 50 ms 后 4 条并发）：**59.5 ms** 完成，读取若仍占循环
  至少 200 ms；并发期间循环空转 27349 次。红→绿：修复前 12 条 off-loop 用例全红。
- **L6** `<待填>` `perf(impact)` A14 —— `ImpactEvaluator.measure()` 的 `duplicate_rate` 写成
  `[texts.count(t) for t in texts]`：每条扫一遍全表 → O(n²)。改成 `Counter(texts)` 一次计数后
  取 `sum(c for c in counts.values() if c > 1)`。n=1000/2000/4000 实测 **6.0 / 24.3 / 99.7 ms**
  （每翻倍一次 ×4，是干净的二次曲线）→ n=16000 **1630.2 ms → 4.38 ms**。
  新增 `tests/unit/test_impact.py` 2 例：一条锁语义（`a,a,a,b,c,c` 的重复率是 **5/6** 而不是
  「有几条撞了车」的 4/6 —— 整组都算重复，这是 `duplicate_rate` 的口径），一条锁量级
  （n=16000 预算 200 ms，带 trace）。红→绿：把推导式塞回去 → 用例红在 1630.2 ms。
  全仓扫了同一族（推导式里 `list.count`），只剩 `api/vector/chromadb.py:140` 的
  `self._collection.count() == 0`，那是数据库计数、不是列表扫描，族到此为止。
- 全量：L4 后 **3679 passed / 3 skipped**（89.2 s），L5 后 **3703 passed / 3 skipped**
  （90.2 s），L6 后 **3705 passed / 3 skipped**（91.1 s），覆盖率门禁均通过；基线 3668。

> **操作纪律**（L4 踩过）：验红用的是**定向反向 patch**，绝不用 `git checkout <file>` 撤注入 ——
> 本轮 `api/deps.py` 有未提交工作，一次 `git checkout` 把整段缓存实现清掉了，只能重写。
