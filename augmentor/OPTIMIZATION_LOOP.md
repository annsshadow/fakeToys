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
- [x] **L7** `perf(api)`: A13 `dataset_tools` / `system_ops` 的 11 处读取 + stats 的分析全部离开事件循环
- [x] **L8** `feat(api)`: B6 新增 `/api/dataset/impact` + `/api/dataset/evaluate` 两条只读端点（SDK 里「有、路上没有」的能力）
- [x] **L9** `perf(dedup)`: A6 前半 —— 相似度行块只算 `[start, n)` 的窄列，不再白算整个下三角

## Backlog A — 性能（含 file:line 与实测线索）

| # | 位置 | 问题 | 量级 |
|---|------|------|------|
| ~~A1~~ | ~~`augmentor/analytics.py:368`~~ | ~~均值在生成器里重算 → O(n²)；实测 n=20000 时 **1043 ms vs 2 ms**~~ 已修（L3） | S |
| ~~A2~~ | ~~`api/deps.py` `allowed_data_roots()`~~ | ~~每个路径参数都 `load_config()` 重新解析 YAML，实测 **6.8 ms/次**~~ 已修（L4，含环境变量分支 0.29 ms/次） | S |
| ~~A3~~ | ~~`augmentor/analytics.py:403-412`~~ | ~~`get_duplicate_candidates()` 纯 O(n²) 且每对重建字符集合~~ **L9 实测排除**：生产路径零调用者（只有测试与自身），优化它不会改变任何用户可见耗时；真要做属于 B 类「接线」而非 A 类 | M |
| ~~A4~~ | ~~`api/routes/quality.py:141,185,217,253,285,309,370,403` + `audit.py:31,32` `leakage.py:46,47` `privacy.py:52` `export.py:132`~~ | ~~`async def` 里同步 `load_items()` 阻塞事件循环，应走 `run_in_thread`~~ 已修（L5；`export.py:109` 本就在线程内，非缺陷） | M |
| A5 | `augmentor/pipeline.py:205-210` + `quality.py:133,153` | 逐条 `encode()`/`predict()` 打分，已有 `batch_score()` 却没用 | M |
| ~~A6~~ | ~~`augmentor/dedup.py:242-290`~~ | ~~贪心归组只消费 `j > i`，行块却每次算满 `block_rows × n` → 整个下三角白算。真实 6902 条实测分组 **30.0 s → 20.7 s**~~ 已修（L9）；剩下的稠密编码另立 **A17**，块行数另立 **A18** | L |
| A7 | `augmentor/search_enhanced.py:448,82` | 每次查询重建索引（n=3000 重建 8 ms vs 查询 2 ms），且 contains/fuzzy 不查索引 | M |
| ~~A8~~ | ~~`api/routes/data.py:146-164`~~ | ~~整文件解析 + 全量过滤后再切片，无早停~~ **L9 实测排除**：响应里的 `total` 按定义要求「过滤后的总数」，早停会把它算错；能省的只有物化切片，量级不值得 | M |
| ~~A9~~ | ~~`augmentor/tracker.py:114` → `:87-99`~~ | ~~每个指标点整文件重写 → O(points²) 字节~~ **L9 实测排除**：`log_metric`/`start_experiment` 除测试外无调用者（`ExperimentTracker` 本身在 `pipeline.py:118` 有构造，别误判成死模块）。等 B7 把实验回路接上再优化才有意义 | S |
| A10 | `augmentor/quality.py:175-179` | 多样性回退里参照文本 n-gram 集合反复重建 | S |
| A11 | `augmentor/indexer.py:117-123,345,352,372-399` | 每次 `DatasetView` 操作重建全部索引（filter 26 ms @ n=3000） | M |
| A12 | `augmentor/validation.py:217-222`、`sampler.py:296-299` | 循环内未缓存的正则、`list.index`/`in` 线性扫描 | S |
| ~~A13~~ | ~~`api/routes/dataset_tools.py:311,360,386,387,416,436,566,598` + `system_ops.py:320,348,514`~~ | ~~**A4 的同构族**：`read_items()`（同步版）在 11 个 `async def` 路由体里直接调用，同样占着事件循环；`dataset_tools.py:566` 还是「多个文件在循环里串行读」；`/api/dataset/stats` 连分析都留在循环上（43.4 ms）~~ 已修（L7） | M |
| ~~A14~~ | ~~`augmentor/impact.py:66`~~ | ~~`duplicate_rate` 里 `texts.count(t)` 写在推导式中 → O(n²)~~ 已修（L6） | S |
| A15 | `augmentor/statistics.py` `calculate_statistics`、`A3`/`A5` 那类纯 Python 分析 | **线程池对 CPU 型分析不产生并行**（GIL）：3 并发 stats 实测离线后请求方 173.7 → 192.7 ms（+11%），换来的只是循环停顿 170.1 → 60.0 ms。要么上 `ProcessPoolExecutor`，要么回到算法侧把 43.4 ms 这个数本身降下来 | M |
| A16 | `api/routes/system_ops.py:514` `dependency_register` | 为了拿「条数」这一个整数把整个数据集解析一遍（3.4 MB / 9 ms 读 + 全量 list 物化）。登记动作本身只需要 count，可流式计数或延后到首次访问再回填 | S |
| A17 | `augmentor/dedup.py:111`、`:241-242` | A6 剩下的一半：fallback 编码 `np.zeros((n, vocab))` 是**稠密 float64**，真实 6902 条 × 23033 词表 = **1.27 GB**（nnz 仅 194293，稀疏度 0.9988 → 99.88% 是零）；调用方随后 `np.asarray(..., dtype=np.float32)` 又整份复制一份 0.64 GB，并对**已归一化**的向量再 `_normalize` 一遍。稀疏 CSR 需要 scipy、`range_search` 需要 faiss，**本环境两者都没装**，所以可落地的只是 dtype 与重复归一化那部分 | L |
| A18 | `augmentor/dedup.py:132-147,172-174` | L9 的归因后续：`_row_block_size` 用**最大列数 n** 定块行数，窄块化之后每一块的列数在递减，块行数却固定在 `4M // n`。实测（n=6902/dim=8192 合成）固定 579 行只到 68.9 GFLOP/s，而**按剩余列数自适应**能回到 91.5 GFLOP/s（满宽是 108.3），即再快 **1.28×**；峰值内存上限不变（每块仍 ≤ 4M 单元） | S |

## Backlog B — 功能增强（价值 ÷ 工作量）

| # | 内容 | 证据 | 量级 |
|---|------|------|------|
| B1 | 把 `/api/system/*`(13) 与 `/api/dataset/*`(12) 接入 UI | `web/src/services/api.ts` 对二者零引用；`System.tsx:3` 只用了 status | M |
| B2 | Excel/CSV 摄取端到端（上传 + `convert --input-format xlsx`） | SDK `csv_excel_import.py:61,152` 可达性为零；`data.py:236-243` 只认 JSON | M |
| B3 | 反向转换边 `alpaca/sharegpt/chatml/vicuna/belle/tsv → json` | `converter.py:60-66` 只读 json/jsonl/csv，导出侧 13 种 → 不可回环 | M |
| B4 | `QualityGate` / `DatasetHealthScore` 暴露为 CLI 子命令 + 端点 | 两者在 `augmentor/__init__.py:38+` 导出但无人可达 | S |
| B5 | 死配置项：读取或显式废弃（`config.py:75-167` 的 sampler/expander/tracker/visualization/multilingual/evaluation/vector/active_learning 等） | 无代码路径读取 | M |
| ~~B6~~ | ~~/api/dataset/evaluate（BLEU/ROUGE）+ /api/dataset/impact（前后对比）~~ | ~~`evaluation.py:74,155,211`、`impact.py:49,96` 纯函数已就绪~~ 已做（L8） | S |
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
- **L6** `92729896b` `perf(impact)` A14 —— `ImpactEvaluator.measure()` 的 `duplicate_rate` 写成
  `[texts.count(t) for t in texts]`：每条扫一遍全表 → O(n²)。改成 `Counter(texts)` 一次计数后
  取 `sum(c for c in counts.values() if c > 1)`。n=1000/2000/4000 实测 **6.0 / 24.3 / 99.7 ms**
  （每翻倍一次 ×4，是干净的二次曲线）→ n=16000 **1630.2 ms → 4.38 ms**。
  新增 `tests/unit/test_impact.py` 2 例：一条锁语义（`a,a,a,b,c,c` 的重复率是 **5/6** 而不是
  「有几条撞了车」的 4/6 —— 整组都算重复，这是 `duplicate_rate` 的口径），一条锁量级
  （n=16000 预算 200 ms，带 trace）。红→绿：把推导式塞回去 → 用例红在 1630.2 ms。
  全仓扫了同一族（推导式里 `list.count`），只剩 `api/vector/chromadb.py:140` 的
  `self._collection.count() == 0`，那是数据库计数、不是列表扫描，族到此为止。
- **L7** `0d5040f24` `perf(api)` A13 —— A4 的同构族第二处：`dataset_tools.py`（8 处）+
  `system_ops.py`（3 处）在 `async def` 函数体里直接调同步 `read_items()`，全部改为
  `await read_json_file(path)`（它就是 `read_items` 的异步外壳）；`/api/dataset/aggregate`
  的「N 个文件在循环里串行读」改成逐个 await（保留按 `datasets` 顺序、第一个坏文件先报错的
  语义）；`/api/dataset/stats` 连 `calculate_statistics` 都留在循环上，实测 6902 条 **43.4 ms**
  （比读同一份文件的 9.2 ms 还贵 4 倍），一并离线。
  守卫用例 24 → **45** 条：`_record_reads` 现在**同时**打在路由模块自己的
  `read_items`/`load_items` 和收口处的 `deps.read_items` 上——只打前者，修好之后那个名字
  已经不存在，什么都测不到；只打后者，缺陷态（各模块自己 import 的绑定）看不见。新加的
  `test_stats_analysis_runs_off_the_event_loop` 打在**库模块** `augmentor.statistics` 上，
  因为路由是在函数体里 `from ... import`，两个状态都拦得住。
  红→绿：整族退回缺陷态 → 10 条 dataset/system 的 off-loop 用例全红（消息是
  「在事件循环线程上同步读取了 [...]，会阻塞整个服务」），而 20 条 `status==200` 用例
  **全绿**——旧代码功能本来就没坏，只有线程身份这个预言机看得见；单独把 stats 的分析塞回
  循环 → 该用例红（`assert 27848 != 27848`）。
  **实测边界（诚实记录）**：线程池买到的不是「这条请求更快」。3 并发 `/api/dataset/stats`
  （真实 6902 条文件，预热后）——缺陷态总耗时 173.7 ms、循环最大停顿 **170.1 ms**、空转 6 次；
  修复态总耗时 192.7 ms（请求方 +11%）、循环最大停顿 **60.0 ms**、空转 12 次。原因是 GIL：
  纯 Python 分析在工作线程里照样攥着锁。据此新立 **A15**（要并行得换进程池，或回到算法侧把
  43.4 ms 这个数本身降下来）与 **A16**（`dependency_register` 只为一个整数解析整份数据集）。
  顺带**实测排掉**三处「看着像缺陷其实不是」：`check_dependencies()` 0.7 ms（用的是
  `find_spec` 不是 import）、首次 `get_pipeline()` 6.7 ms（无密钥时不加载权重）、
  `dataset_tools._dump` 的两个写盘点本就在线程内。
- **L8** `c0c3089f7` `feat(api)` B6 —— 把 SDK 里「有、路上没有」的两个能力接成只读端点：
  `POST /api/dataset/impact`（`ImpactEvaluator` 的四项增益 + `is_beneficial` 判定）与
  `POST /api/dataset/evaluate`（`ModelEvaluator.evaluate_batch` 的 BLEU / ROUGE-L / 相似度均值）。
  两条都在门口做了**静默降级**拦截，因为库的取文本方式是 `item.get(field, "")`：
  字段名整体拼错会得到「唯一指令数 1、重复率 100%」这种**根本没读过数据**的结论（→ 400 并回
  首条实际字段），逐条缺字段则会凭空造一条 0 分样本稀释均值（→ 400 并带**下标**）；
  空基线 → 400（四项增益分母都是基线，报 0.0 会被读成「增强毫无效果」），
  空 after → **200**（规模增益 -1.0、`beneficial` False，那是一次清光数据的合法结论）；
  指标白名单由 `METRIC_FUNCTIONS` **反推**，不手抄第二份枚举（F-04 那族教训）。
  新增 13 例（`TestDatasetImpact` 5 + `TestDatasetEvaluate` 8），预言机全部**手算**、不复用被测库：
  `a,a,a,b,c,c → after 3 唯一文本 → diversity_gain = (3-1)/1 = 2.0`；
  `how do i sort a list` vs `how do i filter a list` → rouge_l = LCS 5/6 的 F1、
  similarity = 5/7、bleu = 0（4 元文法零重叠），两侧均值 0.5 / 11÷12 / 6÷7。
  另加 2 条矩阵行进 `test_api_event_loop_blocking.py`（两条新端点各有读取点，同 L7 口径）。
  **红→绿**：定向反向 patch 逐个撤守卫 → 6 条用例全红（撤空基线/拼错字段/空评估侧 → `200 == 400`；
  撤逐条缺字段 → `500 == 400`，KeyError 逃出；撤长度不匹配 → 报错文案换成库里的英文那句）。
  其中「未知指标」那条**第一次注入时没红**——库的 `ModelEvaluator.__init__` 也抛
  `DataValidationError` → 400，只断言状态码等于把守卫和兜底混为一谈。据此把用例改成断言
  「参数校验**先于 I/O**」：传一个不存在的文件仍得 400 且文案是守卫那句（撤守卫后变 404），
  这条守卫的真正契约这才可 falsifiable。
  **两道既有闸门各自抓到一件事**：`TestWriteRouteAuthCoverage` 要求新路由显式归类（已加
  `OPEN_ALLOWLIST`：两条都是只读）；`test_api_openapi_contract.py` 的
  `test_all_modelled_endpoints_have_a_no_drop_case` 直接变红，要求为两个新 `response_model`
  补「不裁字段」用例——补了 2×2 条（含递归钉 `before/after` 的 6 个键与 `gains` 的 4 个键，
  它们是 `Dict[str, Any]`，顶层模型漏字段不会在顶层显形），并反向验证：从 `ImpactResponse`
  删掉 `beneficial` → 行为层与 OpenAPI 文档层**双双变红**。
- **L9** `perf(dedup)` A6 —— 去重分组只消费 `j > i` 方向的相似度（见
  `_greedy_group_from_row` 的 `row[i + 1 - start:]`），行块却每次算满 `block_rows × n`：
  整个下三角的结果**按定义没有读者**。改成让 `_iter_similarity_row_blocks` 产出
  `(end-start) × (n-start)` 的窄块后，用用户自己的真实 `train_data.json`（6902 条；
  本环境没装 sentence-transformers，走的正是 fallback 编码路径）只读实测：
  **端到端 31.2 s → 22.4 s**，其中相似度分组 **30.0 s → 20.7 s**，组数 369 不变；
  另写一份「新旧实现各跑一遍」的对照脚本，得到 **1.37×** 且 `结果完全相同=True`。
  **没有到达理论的 2×，因此做了归因微基准**（n=6902、dim=8192 的合成归一化矩阵）：
  满宽块 7.20 s / 有效算力 108.3 GFLOP/s，固定 579 行的窄块 6.14 s / **68.9 GFLOP/s**
  ——结果单元数减半而耗时只降 15%，说明**窄块本身让 BLAS 分块效率掉了 36%**。
  同一基准里「块行数按剩余列数自适应」跑出 4.79 s / 91.5 GFLOP/s（相对满宽 **1.50×**），
  这就是 **A18** 的候选，留给 L10；本轮的边界是「只把没读者的列砍掉」。
  **红→绿**：定向反向 patch 恢复 `np.dot(normalized[start:end], normalized.T)` +
  `row[i + 1:]` → 结构护栏用例红在 `第 666 行的块列数是 6000，应为 n - start = 5334`。
  该用例把「不白算」写成了**可失败的断言**（每块 `cols == n - start`、
  总计算单元 ≤ `n²/2 + n·block_rows`），而不是只测墙钟；既有的「与满矩阵朴素参照逐块等价」
  用例（chunk_size 1/2/3/7/25/1000）在注入态**仍然全绿**——这正说明只靠等价性测不出浪费，
  护栏必须断言「算了多少」。
  **顺带实测排掉 4 个 backlog 候选**（都是「看着像缺陷、量了不是」）：
  `A3 get_duplicate_candidates()` 在生产路径零调用者（全仓只搜到定义本身）；
  `A9` 的 O(points²) 重写只在 `log_metric()` 里发生，而 `grep` 显示**除测试外无人调用**
  `log_metric`/`start_experiment`——注意别记成「死模块」：`pipeline.py:118` 确实
  `self.tracker = ExperimentTracker()`，构造是活的，只是这条写入路径没被走；
  `A8` 的切片看似无早停，但响应里的 `total` 按定义要求全量过滤，砍不得；
  `A5` 的语义分支本环境根本没装 sentence-transformers，无法实测。
- 全量：L4 后 **3679 passed / 3 skipped**（89.2 s），L5 后 **3703 passed / 3 skipped**
  （90.2 s），L6 后 **3705 passed / 3 skipped**（91.1 s），L7 后 **3726 passed / 3 skipped**
  （95.1 s），L8 后 **3747 passed / 3 skipped**（98.0 s），L9 后 **3747 passed / 3 skipped**
  （92.4 s）——L9 用例数不变是**有意**的：它**改写**了既有的结构护栏用例而非新增。
  覆盖率门禁均通过（98.52%）；基线 3668。

> **操作纪律**（L4 踩过）：验红用的是**定向反向 patch**，绝不用 `git checkout <file>` 撤注入 ——
> 本轮 `api/deps.py` 有未提交工作，一次 `git checkout` 把整段缓存实现清掉了，只能重写。
>
> **工作树共享事故（L8 提交后）**：`c0c3089f7` 落地 10 秒后被同一工作树里的并行 agent
> 以 `ce9b3c1ba` 整体 `git revert`（只撤我那 7 个 augmentor 路径），随后的历史整理又把那次
> revert 从 `main` 上抹掉了，所以 L8 最终仍在。**但共享 `main` 上的互相撤销与历史重写不是
> 我能控制的**，经用户确认，L9 起改到独立分支 `augmentor-opt100` 上继续（起点 `39e968d47`，
> 含 L1–L8 全部成果）。同一工作树同一时刻只能检出一个分支：切到本分支后，并行 agent 若继续
> 提交，会落在 `augmentor-opt100` 而不是 `main`。
>
> **该预言在 L9 时兑现**：`main` 停在 `39e968d47`（G5 rev347），而 rev348–rev351 四条
> 并行 agent 的提交全部落在了 `augmentor-opt100` 上（`git rev-list --count main..augmentor-opt100`
> = 5，含我自己那条文档整理）。这不是故障而是同一工作树只有一份检出的必然后果，
> 记录在此是为了说明：**本分支的哈希不纯是 augmentor 的线性历史**，两方的工作交错在一起；
> 合并时不能按「只挑 augmentor 提交」来 rebase。并行 agent 的 4 条提交都没碰 `augmentor/`
> 下任何文件（工作树里 `augmentor/` 只有我改的 dedup.py 与 test_dedup.py），所以 L1–L8
> 成果完好，全量 3747 例可证。
