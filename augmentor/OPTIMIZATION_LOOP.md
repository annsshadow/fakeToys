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
- [x] **L10** `perf(dedup)`: A17 可落地的一半 —— 稠密编码矩阵改 float32 + 全程原地归一化（峰值 2429 MB → 622 MB）
- [x] **L11** `feat(quality-gate)`: B4 `QualityGate` + `DatasetHealthScore` 接线 —— 库函数 `gate_dataset_health()` + `/api/quality/health-gate` + CLI `health-gate`（判负退出 1）
- [x] **L12** `perf(api)`: A20 服务配置文件路径的 `Path.resolve()` 收成缓存 —— 200 次查询 29.9 ms → 0.15 ms（安静态）
- [x] **L12** `perf(statistics)`: A15 的算法侧 —— 三处重复扫描合成单趟 + token 分批入 `Counter`，真实 6902 条 `calculate()` **50.94 → 43.87 ms**、词汇统计峰值 **8.15 MB → 2.22 MB**，`to_dict()` 逐字节不变
- [x] **L13** `perf(search)`: A7 的「重建」那一半 —— 倒排索引改成按需构建，真实 6902 条默认 contains 查询 **90.30 → 7.96 ms（11.34×）**、regex **19.41×**、单次查询峰值 **6.30 MB → 0.13 MB**
- [x] **L14** `perf(quality)`: A10 —— 回退多样性的参照 n-gram 集合整批只切一次（与向量分支同构），真实 1500 条 `batch_score` **339.5 → 61.8 ms（5.49×，9/9 轮）**、`_ngram_similarity` **46500 → 1500 次**，五项分数逐条相同；**代价：峰值内存 +80 KB**
- [x] **L15** `perf(vector)`: A22（新发现）—— `FAISSDB.add_vectors` 的逐条写入从 O(n²) 收成 O(n)：几何扩容 + ID 镜像集合，真实维度 384 逐条写 3000 条 **1671.3 → 31.7 ms（52.72×，3/3 轮）**，缩放从平方变线性
- [x] **L16** `feat(converter)`: B3 前半 —— 补上 6 条反向转换边（`alpaca/belle/llama_factory/sharegpt/vicuna/chatml → json`），并给 CLI `--input-format` / API `source_format` 接线；导出→再增强的回路打通，新增 42 例
- [x] **L17** `perf(validation) + fix`: A12① —— 禁止模式预编译并把 `import re` 提到模块级，真实 6902 条 strict 整档校验 **28.99 → 19.49 ms（1.49×，9/9 轮）**、该段 **1.69–1.88×**；**顺带修掉一个新发现的缺陷**：`DatasetValidator(preset=...)` 直接把类级预设字典挂到实例上，任一实例改规则即污染全部预设
- [x] **L18** `perf(indexer)`: A11（A7/L13 的同构缺陷）—— `DatasetIndexer` 的 4 份默认索引改成**按需、按份**构建，`DatasetView` 的切片/过滤/采样从「每次白建 19 MB 索引」变成零构建：真实 6902 条 `filter()` **132.7 → 0.44 ms**、冷启动 contains **79.9 → 7.1 ms**、构造视图（n=4000）内存 **峰值 11.7 MB → 驻留 1.2 KB**，新增 12 例
- [x] **L19** `feat(indexer)`: A23 —— `search_exact` / `search_ngram` 不再被默认清单限死：清单外字段、任意 `n` 第一次被查询时按需建自己的索引。真实 6902 条 `search_ngram("output", …)` **0 → 8 命中**（朴素全表扫描同为 8），`search(method="ngram")` 的命中数从「少报一半」变成与 contains 逐条相同（租房 **419 → 823**）；**代价也实测了**：冷 ngram 查询 **55–65 → 225–240 ms**（要多建一份 `output_2`，单份 170.9 ms），另立 **A24**；新增 8 例、改写 3 例

## Backlog A — 性能（含 file:line 与实测线索）

| # | 位置 | 问题 | 量级 |
|---|------|------|------|
| ~~A1~~ | ~~`augmentor/analytics.py:368`~~ | ~~均值在生成器里重算 → O(n²)；实测 n=20000 时 **1043 ms vs 2 ms**~~ 已修（L3） | S |
| ~~A2~~ | ~~`api/deps.py` `allowed_data_roots()`~~ | ~~每个路径参数都 `load_config()` 重新解析 YAML，实测 **6.8 ms/次**~~ 已修（L4，含环境变量分支 0.29 ms/次） | S |
| ~~A3~~ | ~~`augmentor/analytics.py:403-412`~~ | ~~`get_duplicate_candidates()` 纯 O(n²) 且每对重建字符集合~~ **L9 实测排除**：生产路径零调用者（只有测试与自身），优化它不会改变任何用户可见耗时；真要做属于 B 类「接线」而非 A 类 | M |
| ~~A4~~ | ~~`api/routes/quality.py:141,185,217,253,285,309,370,403` + `audit.py:31,32` `leakage.py:46,47` `privacy.py:52` `export.py:132`~~ | ~~`async def` 里同步 `load_items()` 阻塞事件循环，应走 `run_in_thread`~~ 已修（L5；`export.py:109` 本就在线程内，非缺陷） | M |
| ~~A5~~ | ~~`augmentor/pipeline.py:205-210` + `quality.py:133,153`~~ | ~~逐条 `encode()`/`predict()` 打分，已有 `batch_score()` 却没用~~ **L19 读代码后判定：不能直接换（未实测收益，因为换了就不是同一件事）**。`pipeline.py` 那里有一条**写明在代码注释里的刻意语义**（「不使用已有生成文本，避免锁竞争」），而 `batch_score()` 的多样性是**渐进**的：它把已通过门槛的样本逐个追加进参照集（`quality.py:417-433`），所以「同一批 8 个变体各自独立打分」与「一批丢进 `batch_score`」的多样性维度**按定义不同**，`passed` 阈值判定会跟着变，结果是**静默丢掉原本会被采纳的样本**。真要动它属于改质量口径（B 类决策），不属于性能轮顺手能改；且本环境缺 sentence-transformers，`encode()` 走回退路径，收益量级也无法真实测量。 | M |
| ~~A6~~ | ~~`augmentor/dedup.py:242-290`~~ | ~~贪心归组只消费 `j > i`，行块却每次算满 `block_rows × n` → 整个下三角白算。真实 6902 条实测分组 **30.0 s → 20.7 s**~~ 已修（L9）；剩下的稠密编码另立 **A17**，块行数另立 **A18** | L |
| ~~A7~~ | ~~`augmentor/search_enhanced.py:448,82`~~ | ~~每次查询重建索引（n=3000 重建 8 ms vs 查询 2 ms），且 contains/fuzzy 不查索引~~ **前半已修（L13）**：`search_dataset()` 每次都新建搜索器，而构造即无条件建索引（真实 6902 条 64.5 ms + 6.17 MB），五种方法里只有 exact 读它 → 改成按需构建。**「不查索引」那一半另立 A21** | M |
| ~~A8~~ | ~~`api/routes/data.py:146-164`~~ | ~~整文件解析 + 全量过滤后再切片，无早停~~ **L9 实测排除**：响应里的 `total` 按定义要求「过滤后的总数」，早停会把它算错；能省的只有物化切片，量级不值得 | M |
| ~~A9~~ | ~~`augmentor/tracker.py:114` → `:87-99`~~ | ~~每个指标点整文件重写 → O(points²) 字节~~ **L9 实测排除**：`log_metric`/`start_experiment` 除测试外无调用者（`ExperimentTracker` 本身在 `pipeline.py:118` 有构造，别误判成死模块）。等 B7 把实验回路接上再优化才有意义 | S |
| ~~A10~~ | ~~`augmentor/quality.py:175-179`~~ | ~~多样性回退里参照文本 n-gram 集合反复重建~~ **已修（L14）**：真实 1500 条 `batch_score` 同进程交替中位 **339.5 → 61.8 ms（5.49×，9/9 轮）**、`_ngram_similarity` **46500 → 1500 次**、集合分配约 **37200 → 9930 个**（N=300 单测），五项分数逐条相同；**峰值内存反向 +79.6 KB**（缓存常驻 30 个 frozenset），本轮只买 CPU | S |
| ~~A11~~ | ~~`augmentor/indexer.py:117-123,345,352,372-399`~~ | ~~每次 `DatasetView` 操作重建全部索引（filter 26 ms @ n=3000）~~ **已修（L18）**：这是 A7/L13 的同构缺陷——`DatasetView.__init__` 无条件 `DatasetIndexer(items)`，而构造即建齐 3 份倒排 + 1 份 n-gram（真实 6902 条 **70 ms / 峰值 19.2 MB**），可是 `search()` 的默认方法 contains 和 `filter/head/tail/sample/切片` **一份都不读**。改成「按份、按需」构建：`filter()` **132.7 → 0.44 ms**、链式三级 **195 → 0.9 ms**、冷启动 contains **79.9 → 7.1 ms（10.9×）**、exact **68.8 → 8.4 ms**、ngram **65.0 → 56.8 ms（只省 3 份倒排）**；构造视图（n=4000）内存 **峰值 11.7 MB → 驻留 1.2 KB**；旧版构造耗时随 n 线性（15.65/34.08/54.79 ms @ 2000/4000/6000），新版与 n 无关 | M |
| ~~A23~~ | ~~`augmentor/indexer.py:127-141,163-178`~~ | ~~**L18 新发现（口径缺陷，不是性能）**：`search_exact` / `search_ngram` 只对默认清单（`instruction/output/input` + `instruction_2`）生效，清单外的字段**静默返回空列表**~~ **已修（L19）**：判据换成「要么欠着默认清单的账，要么手上真有数据可建」，字段与 `n` 都不再受限。**真实数据上的症状比预想的重**：`search(query, method="ngram")` 的默认字段是 `instruction`+`output`，而 `output_2` 从来没人建过，于是这条查询一直在**静默少报 output 侧的全部命中**——真实 6902 条「租房」 **419 → 823**（与 contains 逐条相同）、「如何」 **2019 → 2035**；`search_ngram("output","，晨")` **0 → 8**（朴素全表扫描 8）。代价同轮实测：冷 ngram 查询 **55–65 → 225–240 ms**（多出的一份 `output_2` 单份 170.9 ms），临时倒排一份只 4.3 ms。`search_enhanced.py` 里**没有同构缺陷**（`_build_indexes` 遍历 `item.items()` 不收清单，`_search_ngram` 走扫表） | S |
| A24 | `augmentor/indexer.py:213-242` | **L19 新记（成本，不是缺陷）**：一次性的 n-gram 查询要付**整份索引**的钱——真实 6902 条首查 `output_2` 170.9 ms、`instruction_2` 58.7 ms，而查询串本身只有 1～k 个 n-gram。对「只查一次」的调用方，直接扫全表算 bigram 交集比建索引便宜得多（contains 扫全表实测 7.3 ms 就是上界参照）。修法与 A21 同族：给 `search_ngram` 加「首查扫表、累计第 N 次才建索引」的策略，或者把 n-gram 倒排改成按需局部化。**本轮没动**（L19 的边界是把语义修对并如实记下代价，不顺手改性能口径） | M |
| ~~A12~~ | ~~`augmentor/validation.py:217-222`~~、`sampler.py:296-299` | **L13 实测拆成两半**：①`_validate_item` 里 `import re` + 每条每模式一次 `re.search`；②`sampler.generate_report()` 的 `items.index(seed)` **实测不是缺陷**（真实数据只推荐 4 个种子、反查 0.0 ms，单趟 id 映射要 1.1 ms，改了反而更慢；且它还会改变「值相等但不同对象」时的下标语义，真实数据里正好有 367 条重复 dict）—— 这一半作废。**①已修（L17）**：真实 6902 条 strict **28.99 → 19.49 ms（1.49×）**，禁止模式段占整档 53–55%、该段自身 **1.69–1.88×**（L13 预估的「~5 ms / 1.2×」偏保守）；`re.search` 逐条调用 **27608 → 0**、`re.compile` 与条数无关恒为 2 | S |
| ~~A13~~ | ~~`api/routes/dataset_tools.py:311,360,386,387,416,436,566,598` + `system_ops.py:320,348,514`~~ | ~~**A4 的同构族**：`read_items()`（同步版）在 11 个 `async def` 路由体里直接调用，同样占着事件循环；`dataset_tools.py:566` 还是「多个文件在循环里串行读」；`/api/dataset/stats` 连分析都留在循环上（43.4 ms）~~ 已修（L7） | M |
| ~~A14~~ | ~~`augmentor/impact.py:66`~~ | ~~`duplicate_rate` 里 `texts.count(t)` 写在推导式中 → O(n²)~~ 已修（L6） | S |
| ~~A15~~ | ~~`augmentor/statistics.py` `calculate_statistics`、`A3`/`A5` 那类纯 Python 分析~~ | ~~**线程池对 CPU 型分析不产生并行**（GIL）：3 并发 stats 实测离线后请求方 173.7 → 192.7 ms（+11%），换来的只是循环停顿 170.1 → 60.0 ms。要么上 `ProcessPoolExecutor`，要么回到算法侧把 43.4 ms 这个数本身降下来~~ **算法侧已修（L12）**：真实 6902 条 `calculate()` 同进程交替中位 **50.94 → 43.87 ms（1.16×）**、词汇统计峰值 **8.15 MB → 2.22 MB**。**GIL 那半仍然成立**——进程池本轮不做（跨进程要序列化整份数据集，代价未实测），所以「3 并发总耗时」这个数不会因为 L12 变成并行 | M |
| ~~A16~~ | ~~`api/routes/system_ops.py:514` `dependency_register`~~ | ~~为了拿「条数」这一个整数把整个数据集解析一遍（3.6 MB / 6902 条），可流式计数或延后到首次访问再回填~~ **L17 实测排除，未改**：同一份真实数据 back-to-back 量得 `await read_json_file` **14.93 ms**、同步 `json.load` **12.89 ms**、「读字节 + `loads` 只取长度」**9.89 ms**，而手写纯 Python 顶层元素扫描器 **77.84 ms（慢 8 倍）且把 6902 条数成了 13804**（内层数组的花括号它分不清）。结论：C 级解析就是拿这个整数最便宜的路子，「流式计数」在此不但更慢还会放松掉「登记时就拒绝坏 JSON」的语义。 | S |
| ~~A17~~ | ~~`augmentor/dedup.py:111`、`:241-242`~~ | ~~A6 剩下的一半：fallback 编码 `np.zeros((n, vocab))` 是**稠密 float64**（真实 6902 条 × 23033 词表 = 1.27 GB），调用方再 `np.asarray(..., dtype=np.float32)` 整份复制、再 `_normalize` 另起一份~~ **已修（L10）**：float32 + `_row_norms`/`np.divide(out=)` 全程原地，真实数据峰值 **2429 MB → 622 MB**。剩下的只有「稀疏 CSR 表示」（nnz 194293、稀疏度 0.9988 → 理论 2 MB），但本环境**没装 scipy 与 faiss**，手写稀疏结构体属于另起一套索引子系统，不在性能轮范围内 → 本轮不做，装依赖后再议 | L |
| ~~A18~~ | ~~`augmentor/dedup.py:132-147,172-174`~~ | ~~块行数按剩余列数自适应放大，把 L9 归因微基准里的 68.9 → 91.5 GFLOP/s 捡回来~~ **L10 实测证伪，未采纳**：合成基准（dim=8192）预测 1.28×，真实数据（dim=vocab=23033）同一进程内 back-to-back 实测 **0.95×（更慢）**。归因假设（窄块让 BLAS 变笨）**不随 K 维迁移**，代码已回退 | S |
| A19 | `augmentor/dedup.py:104-116` | 词表构建与 TF 填充是**两遍** Python 双循环（`for text: for i:` 再 `for i, text: for j:`），每条文本的每个字符都进解释器一次。实测真实 6902 条走完 0.36 s（L10 后），**这还称不上瓶颈**；只有在 n 到 10⁵ 量级时才可能翻盘——该外推**未实测**，先记着别当依据 | S |
| ~~A20~~ | ~~`api/deps.py` `config_file_path()`~~ | ~~A2 的同构缺陷第三处：L4 把白名单里的 YAML 重解析缓存掉了，但「服务自身的配置文件路径」这一步仍**每次调用**做一次 `Path.resolve()`（Windows 上要问长路径句柄并归一大小写）。任何带路径参数的请求都至少过一次白名单，等于每个请求白加一份系统调用。安静态同进程交替实测：200 次 **29.9 ms（缺陷）→ 0.15 ms（缓存）**；同一台机器带负载时同一份工作量到 187 ms~~ **已修（L12）**：缓存按 `(环境变量原值, os.getcwd())` 建键、单条目、只在未命中时 resolve；`allowed_data_roots()` 200 次 **218.0 ms → 1.87 ms** | S |
| A21 | `augmentor/search_enhanced.py:248-320` | L13 剩下的那一半：contains / ngram / fuzzy / regex **仍逐条扫全表**（真实 6902 条：ngram 116.8 ms、fuzzy 56.7 ms、contains 8.0 ms）。倒排索引里其实已经存了这些词项，但按「子串」「Jaccard 阈值」「bigram 覆盖率」查需要**不同的索引结构**（n-gram 倒排 / token 集合按文档存），不是把现有索引接上去就行——属于「另起一套」，与 A17 的稀疏化同一档 | M |
| ~~A22~~ | ~~`augmentor/vector/faiss.py:100,106`~~ | ~~**L15 新发现**：`add_vectors` 每次都 `np.vstack` 整份矩阵 + 每次都 `set(self._ids)` 重建镜像 → 「一条一批」的写入是 O(n²)。真实维度 384 逐条写入实测 750/1500/3000 条 **58.2 / 386.1 / 1671.3 ms**（输入翻倍时间 ×6.6、×4.3）~~ **已修（L15）**：写缓冲几何扩容 + `_vectors` 改前缀视图 + `_id_set` 增量维护（带长度自愈判据），同一实验 **6.5 / 13.1 / 31.7 ms（×2.0、×2.4 即线性，8.94×/29.55×/52.72×）**；稳态驻留多 ≤1× 数据量的空槽，峰值不变 | M |

## Backlog B — 功能增强（价值 ÷ 工作量）

| # | 内容 | 证据 | 量级 |
|---|------|------|------|
| B1 | 把 `/api/system/*`(13) 与 `/api/dataset/*`(12) 接入 UI | `web/src/services/api.ts` 对二者零引用；`System.tsx:3` 只用了 status | M |
| B2 | Excel/CSV 摄取端到端（上传 + `convert --input-format xlsx`） | SDK `csv_excel_import.py:61,152` 可达性为零；`data.py:236-243` 只认 JSON | M |
| B3① | ~~反向转换边 `alpaca/sharegpt/chatml/vicuna/belle → json`~~ | ~~`converter.py:60-66` 只读 json/jsonl/csv，导出侧不可回环~~ 已做（L16，6 条反向边 + CLI/API 接线；原记「导出侧 13 种」不准，实测 `json →` 只有 8 条边）。**剩余另立 B3②** | M |
| B3② | `tsv ↔ json` 两条边都缺；`convert_file(source_format="alpaca")` 读 `.jsonl` 容器文件会走 `json.load` 而失败；`_json_to_csv` 里 `all_keys` 算了不用 | 转换图实测：`json →` 8 条、`→ json` 8 条，`tsv` 两侧皆无 | S |
| ~~B4~~ | ~~`QualityGate` / `DatasetHealthScore` 暴露为 CLI 子命令 + 端点~~ | ~~两者在 `augmentor/__init__.py:38+` 导出但无人可达~~ 已做（L11：库函数 `gate_dataset_health()` + `/api/quality/health-gate` + `health-gate` 子命令，并已加入包级导出） | S |
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
  **（L10 补注：这条归因不迁移到真实数据——那里 K = vocab = 23033，自适应行数实测
  0.95× 反而更慢，A18 已证伪并回退。于是 L9 的 1.37× 与理论 2× 之间的差额至今未归因。）**
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
- **L10** `perf(dedup)` A17 + A18 证伪 —— 先去重路径上「最大的一笔分配」：fallback 编码
  的稠密 n×vocab 矩阵从 **float64 改 float32**，并把归一化全程改成**原地**
  （新增 `_row_norms()` 用 `np.einsum("ij,ij->i")` 取代 `np.linalg.norm(axis=1)`——
  后者内部会先物化一份**同尺寸**的平方和矩阵，`vectors / norms` 又是一份）。
  真实 6902 条数据、同一进程 back-to-back 对照（只读用户数据集）：
  峰值内存 **2428.7 MB → 621.8 MB（3.9×）**，整条分组路径 **27.24 s → 19.45 s（1.40×）**，
  编码一步 **2.72 s → 0.36 s（7.6×）**，组数 369 且 `结果完全相同=True`。
  TF 值是文档内 n-gram 出现次数（个位整数），float32 在 2^24 内精确表示整数，
  实测两种归一化的逐元素差为 0.0。新增 3 例：dtype 断言、编码步峰值、整条路径峰值
  （预言机都是独立算出的 `n × vocab × 4`，不取自被测库）。
  **两条负结果，都写进账**：
  ① **A18 证伪**：L9 的微基准（n=6902、dim=8192 合成）显示窄块让 BLAS 从 108.3 掉到
  68.9 GFLOP/s，据此推断「行数按剩余列数自适应」能捡回 1.28×；实现完在真实数据
  （dim=vocab=23033）上量到的是 **0.95×，反而更慢**——归因假设不随 K 维迁移，
  代码已整份回退（`/tmp/l10-attempt-*.py` 存档），L9 那条归因在本文件里补了不迁移的注。
  ② **纠正一条自己的错误假设**：此前认为「tracemalloc 看不见 numpy 缓冲区」，
  因此 `test_peak_memory_is_far_below_full_matrix` 疑似假测试。三次注入实测：
  交出 n×n 的**窄切片视图**（形状契约不变、峰值真的持有 95MB）→ 红在
  `峰值 191.1MB`；删掉 `block = None`（同时持有两块）→ 红在 `28.3MB`；
  所以 **tracemalloc 能看见这些分配，该用例可失败**。（第一次探针作废——注入成
  满宽 n×n 块后 `_greedy_group_from_row` 直接 IndexError，那是崩溃造成的红，
  与内存断言无关，不能当证据。）
  **红→绿**：dtype 与编码峰值两例是先写后红（`float64` / 峰值 5.1MB vs 预算 1.97MB），
  改完转绿；路径峰值那例**第一次注入没红**（预算 1.6×矩阵 + 整个 16MB 块上限太松，
  注入态 39.8MB 仍低于 45.6MB），据此把预算改成「一个编码矩阵 + 按公式精确算出的
  一个相似度块」再留 25%，同一个注入立刻红在 `39.8MB > 27.7MB`。
- **L11** `feat(quality-gate)` B4 —— 把 `QualityGate` / `DatasetHealthScore` 从
  「在 `augmentor/__init__.py` 导出但无人可达」接成一条真实判定链：新增库函数
  `augmentor/quality_gate.py:gate_dataset_health()`（**唯一**入口，CLI 与 API 共用它，
  门禁口径一旦有两份实现，两边就会给出不同的放行结论）、端点
  `POST /api/quality/health-gate`（`api/routes/quality.py`，只读 → 进 `OPEN_ALLOWLIST`）、
  CLI 子命令 `health-gate`（判负 = 退出码 1）。命令数 36→37。
  三条设计都是**被缺陷逼出来的**，不是风格选择：
  ① `GateRule.evaluate()` 对缺失的指标键返回 False（既有 `test_missing_metric_returns_false`
  早已固化），所以调用方没传 `pass_rate` 时必须**摘掉该规则并记进 `skipped_rules`**，
  否则「没人算过通过率」会被表达成「通过率不及格」——对 CI 这两种结论完全相反；
  ② 空数据集上每项指标都是 0.0，`duplicate_rate <= 上限` 以「0 ≤ 0.3」恒成立，
  门禁会给一份空文件放行，故 `gate_dataset_health([])` 抛 `DataValidationError`
  （API 400 / CLI 退出 1）；
  ③ 权重非法在**读文件之前**判掉（`_validate_health_gate`），否则客户端要用 500 或 404
  去读回一个「参数写错」。
  新增 30 例：单元 12（`TestGateDatasetHealth`：手算四项健康分与加权总分
  `0.7444365314734798`、`text_field` 两口径对照、pass_rate 三态、warning 两态、空集、
  非法权重、包级导出）；API 9（`TestHealthGateEndpoint`）；CLI 9
  （`tests/integration/test_cli_health_gate.py`，断言重心是退出码而非 stdout）。
  三条 API 守门同步登记：`OPEN_ALLOWLIST` +1、`BLOCKING_SITES` +1（读取 1 次且离线）、
  OpenAPI `CALLS` 70→71（`health` / `gate` 两层 nested 契约）。文档同步：
  `docs/API.md` 端点表 + 详细节、`docs/README.md` §5.3、`cli.py` 与 `docs/ARCHITECTURE.md`
  的命令数。
  **红→绿**：7 个注入全部让对应用例变红且源文件逐一还原（摘掉跳过逻辑 / 写死
  `text_field` / 去掉空集防线 / 去掉门口校验 / 去掉退出码 / 忽略 `block_on_warning` /
  不喂 `duplicate_rate`）。
  **一处诚实附注**：注入「去掉 `_validate_health_gate`」时「权重之和不为 1」那条用例
  仍然绿——库层 `DataValidationError` 是 `ValueError` 子类，被路由的
  `except ValueError → 400` 兜住了。所以门口校验的**独有**证据只有「权重个数」与
  「`pass_rate` 越界」两条（它们在缺陷态确实变红），而「校验早于读取」由
  `test_bad_weights_rejected_before_reading_file`（缺文件 + 非法权重 → 400 而非 404/500）
  单独承担。本轮为功能轮，无性能主张。
- **L12a** `perf(api)` A20 —— `api/deps.py:config_file_path()` 每次调用都 `Path.resolve()`，
  而它是白名单查表的必经一步（L4 只缓存了 YAML 重解析，漏了这一处）。改成按
  `(AUGMENTOR_CONFIG_PATH 原值, os.getcwd())` 建键的单条目缓存，**只在未命中时 resolve**。
  实测：200 次 `config_file_path()` 同进程交替 **29.9 / 31.7 / 29.8 ms → 0.30 / 0.15 / 0.15 ms**，
  带并行负载时同一份工作曾到 187 ms；`allowed_data_roots()` 200 次 **218.0 ms → 1.87 ms**。
  新增 `TestConfigPathResolveCache` 4 例，预言机是 **resolve 次数**而不是时间
  （冷/热调用计数差、换 cwd 计数、换环境变量计数、白名单命中不 resolve）。
  **红→绿**：把函数退回 HEAD 版 → 4 例全红（`4 failed, 59 deselected`），5 个对照组
  （含「改配置立刻生效」「坏配置不缓存」）**全绿**，`已还原=True`。
  **一条负结果，必须写清**：既有的时间预算用例
  `test_whitelist_lookup_stays_sub_millisecond` 在缺陷态**安静时也能通过**（本轮注入实测 GREEN），
  它当初在全量套件里红在 1049.9 ms > 200 ms 是「覆盖率 trace + 并行 agent 抢 CPU」叠出来的。
  也就是说**它不是这处缺陷的可靠预言机**，真正拦住回归的是那 4 条计数用例；
  本轮**没有**新增任何墙钟预算断言。
- **L12b** `perf(statistics)` A15 的算法侧 —— `augmentor/statistics.py` 三处重复扫描：
  ①`_calculate_field_statistics` 先拼一遍字符串再 `Counter` → 每个值 `str()` 三次改成一次
  + 只扫一趟；②`_calculate_content_statistics` 的词汇统计原本一次性物化「全数据集所有 token」
  的大列表，改成攒够 1000 个 token 整块 `update()`；③`_calculate_quality_metrics` 三个独立
  循环合成一趟（每条文本 6 次 `item.get` → 2 次）。真实 6902 条、同进程交替中位：
  字段统计 **7.56 → 5.41 ms（1.40×）**、内容统计 **42.02 → 37.05 ms（1.13×）**、
  质量指标 **2.08 → 1.24 ms（1.68×）**、`calculate()` **50.94 → 43.87 ms（1.16×）**；
  tracemalloc 真实峰值 **8.15 MB → 2.22 MB**。**`to_dict()` 输出逐字节不变。**
  **一条自己踩出来的负结果**：中途我把「一次性大列表」换成「逐条 `word_freq.update(tokens)`」，
  先量到 1.20× 的说法**没复现**（同进程交替 0.93×，即更慢）——原因是 `Counter(列表)` 走 C 实现的
  `_count_elements`，逐条 update 要为 13804 条文本各付一次 Python 调用开销。据此才改成
  分批（`_TOKEN_CHUNK = 1000`）这版：既拿回 C 路径，又把峰值从 7.91 MB 压到 1.97 MB（合成集）。
  **（L7 记录的 43.4 ms 与本轮「缺陷态 50.94 ms」不是同一批次，绝对值不可跨批比，只认同进程交替比值。）**
  新增 8 例，全部**确定性预言机**：`__str__` 调用计数、`dict.get` 调用计数、
  monkeypatch `re.findall` 证明走的是预编译模式、tracemalloc 峰值预算（缺陷态 1292.0 KB vs
  预算 400 KB，修复后 66.6 KB）、以及 4 例**手算对照**（含标准差 `sqrt(1.1875)`、
  `avg/min/max/total`、三项质量指标 0.75 / 2÷3 / 0.25）。
  **红→绿**：9 个注入变体逐个验（去掉单趟复用 / 去掉分批 / 去掉合并循环 / 去掉 `if output:`
  / 把正则塞回循环内 / 按字母序打乱并列次序 等），全部符合预期且文件还原。
  两条设计教训：①并列次序用例最初选的 token 让「首次出现序」恰好等于字母序，
  换成故意逆序的 `m`/`k` 才能真正让「按字母排序」的注入变红；
  ②`test_top_word_order_survives_chunk_boundaries` **没有 HEAD 基线**（缺陷版无
  `_TOKEN_CHUNK` 概念），它锁的是新实现的边界行为，不属于红→绿证据链。
- **L13** `perf(search)` A7 前半 —— `EnhancedSearcher` 在**构造函数**里无条件建倒排索引，
  而五种方法里只有 `exact` 读它；更要命的是 CLI 与 `/api/dataset/search` 的唯一入口
  `search_dataset()` **每次调用都新建一个搜索器**。真实 6902 条实测：建索引 64.5 ms、
  常驻 6.17 MB，而 contains 查询本身 7.5 ms —— 端到端 80.85 ms 里 80% 是没读者的工作。
  改成 `_ensure_indexes()`（`_index_ready` 标志 + 锁内二次检查 + 建在局部字典上再整体发布，
  这样并发首查不会有人读到半成品索引并把结果误报成「无匹配」），`load()` 换数据时作废。
  **同进程新旧交替 9 轮**（`train_data.json` 只读）：contains **90.30 → 7.96 ms（11.34×，9/9 轮）**、
  regex **19.41×（9/9）**、fuzzy 2.45×、ngram 1.69×、exact 0.97×（**噪声带内，无变化**——它是
  唯一本来就要建这份索引的方法）；contains 单次查询 tracemalloc 峰值 **6.30 MB → 0.13 MB**；
  五种方法的 `total_matches` 与 `items` 逐条相同。
  新增 10 例（`TestIndexBuiltOnDemand`），预言机是**构建次数**（monkeypatch 类方法计数，
  构造期也计得到）而非计时；另有一条与**朴素全表扫描**逐条对照的正确性用例、
  一条「换数据必须作废」用例、一条 `get_statistics()` 统计口径手算对照（6 个词项 / 2 个词项）。
  **红→绿**：整份退回 HEAD → `TestIndexBuiltOnDemand` **6 红 4 绿**（构造即建：
  `build_counter == [5]` ≠ `[]`；contains/ngram/fuzzy/regex 四种各红一次；`search_dataset`
  入口红），被改写的既有 `test_build_indexes_non_string` 因 `_ensure_indexes` 不存在而
  AttributeError 红；4 个既有类 31 例全绿，`已还原=True`。**类里那 4 例两侧都绿**，
  它们是语义护栏（exact 只建一次、朴素扫描对照、load 作废、统计口径），不是这处缺陷的
  证据——缺陷态同样满足它们，写在这里是为了不让它们冒充红→绿。
  **顺带实测排掉 A12 的一半**：`sampler.generate_report()` 的 `items.index(seed)` 看着像
  O(n²)，真实数据只推荐 4 个种子、反查 **0.0 ms**，单趟 id 映射反而要 1.1 ms；
  且真实数据里有 367 条「值相等但不同对象」的 dict，改语义会挪动下标。已写回 A12。
- **L14** `perf(quality)` A10 —— 向量分支早就有「参照集整批算一次 + hash 失效」，
  n-gram 回退分支却没有：`_ngram_similarity(text, ref)` 每对都重新切两侧集合，
  并集还物化成第三个集合。改成模块级 `_char_ngrams` / `_jaccard`（并集走容斥
  `|A|+|B|-|A∩B|`）+ `_existing_ngrams` 缓存，`reset_cache()` 一并清。
  **同进程新旧交替 9 轮**（真实 1500 条 × 参照窗 100，`train_data.json` 只读）：
  `batch_score` **339.5 → 61.8 ms（中位 5.49×，9/9 轮都更快，逐轮 4.94–5.91×）**，
  `_ngram_similarity` 调用 **46500 → 1500 次**（剩下的 1500 次是相关性回退分支的
  每条 1 次，本轮没碰），N=300 单独数集合分配 **约 37200 → 约 9930 个**
  （旧：9300 次配对 × 4 个集合；新：930 个 frozenset + 9000 次交集临时集合，
  并集不再物化）。外推到真实 6902 条约 **1.56 s → 0.28 s**（按候选数线性）。
  语义/相关性/多样性/总分/是否通过**五项逐条相同**。
  **两条必须写下的负面**：① 峰值内存**没有降反而升** —— 真实批次 tracemalloc
  **414.86 KB → 494.46 KB（+79.6 KB）**，因为缓存要常驻最多 30 个 frozenset，
  这轮买的是 CPU 不是内存；② 回退分支里 `_char_ngrams` 我第一版顺手加了 `.lower()`，
  而 HEAD 切的是原文 —— 那会把所有英文数据集的多样性**凭空抬高**，已删掉并用
  `"AB"` vs `"ab"` → 1.0 这条口径守卫钉住。
  真实批次之所以能命中缓存：`existing` 在 `batch_score` 里逐条增长，但 hash 只覆盖
  **被缓存的那一段** `existing[:diversity_sample_size]`，前 30 条一旦坐定就不再变
  （300 条候选只切了 930 个集合），这正是早先那处 hash 口径修正的复利。
  新增 7 例（`TestFallbackDiversityRefSets`），预言机是**调用次数**：`pair_calls`
  数 `_ngram_similarity`（两版都有的名字）、`ngram_builds` 数 `_char_ngrams`，
  再加分数不依赖缓存（与冷实例逐个对照）、手算 Jaccard、换窗失效、`reset_cache`、
  两份缓存互不共用。
  **红→绿**：整份退回 HEAD → `AssertionError: 多样性逐对比较了 3000 次，参照集没被复用`
  （计数红，非符号缺失）+ 1 例 `_existing_ngrams_hash` AttributeError；
  另 4 例因新符号不存在而 setup 报错，它们是**结构护栏不是缺陷证据**，写在这里防冒充；
  5 个既有类 30 例全绿，`已还原=True`。**踩坑记录**：`pair_calls` 夹具第一版写成
  `def spy(self, a, b, n=quality._NGRAM_SIZE)`，缺陷态于是只报 AttributeError ——
  红得很但没有回答「有没有逐对比较」。改成 `*args, **kwargs` 转发后计数断言才真的响，
  这是 L10「红必须为正确的理由红」的具体复现。
  **同构扫描**：`augmentor/dedup.py` 里那份 `_ngram_similarity` 是同名复刻，但
  **非测试调用者为 0**（`grep` 只命中定义与测试），改它不会改变任何用户可见耗时，
  不立条目；`quality.py` 剩余的 `_ngram_similarity` 两条调用点（语义/相关性回退）
  本身就是每条 1 次，无可摊。
- **L15** `perf(vector)` A22（本轮新发现）—— 用 O(n²) 惯用法横扫全仓（`.count(` /
  `.index(` / `in <list>`）命中向量库：`FAISSDB.add_vectors` 每次写入都 `np.vstack`
  整份矩阵、每次都 `set(self._ids)` 重建镜像，于是「一条一批」的写入是平方级。
  本环境**没装 faiss**，走的正是 `faiss.py` 自带的 numpy 兜底分支（`backend == "numpy"`），
  所以这条路径可实测。**同进程新旧交替**（dimension=384，逐条写入）：
  750 条 **58.2 → 6.5 ms（8.94×，5/5 轮）**、1500 条 **386.1 → 13.1 ms（29.55×，5/5 轮）**、
  3000 条 **1671.3 → 31.7 ms（52.72×，3/3 轮）**；关键是**缩放形态**从平方
  （翻倍输入 ×6.6、×4.3）变成线性（×2.0、×2.4）。重复 ID 探测 200 次
  **7.8 → 1.5 ms**。外推真实规模 6902 条约 **8.8 s → 73 ms**（按平方/线性形态推，
  未直接实测该点）。
  改法：`_vectors` 从「普通属性」改成**写缓冲的前缀视图**（属性 + 赋值器），
  `_grow()` 按几何倍率扩容（`max(needed, 2×cap, 8)`），`_id_set` 随增删改增量维护；
  `delete`/`clear`/`load` 三处整份重建镜像集合，`add` 入口再用「长度不符即重建」自愈
  —— 绕过写接口改 `_ids` 的用法本来就同时会打乱 `_vectors` 对齐，不在受支持范围内，
  注释里写明了这条边界。
  **三条必须写下的负面/风险**：① 稳态驻留多了 ≤1× 数据量的空槽（1500 条 ×384 维实测
  **2.59 → 3.50 MB**），**峰值不变**（4.79 MB 两版相同），买的是 CPU；
  ② 逐条写与「写一条查一条」交替的模式**不会变快**——`search` 走的是前缀视图，
  几何缓冲对读路径无额外代价，所以这条连退化都没有，但与 L14 的「缓存要读者配合」
  是同一类前提，实测的加速只属于纯写入；③ **本仓内 `create_vector_db` 没有任何非测试调用者**
  （`grep` 只命中定义、导出与测试）。这与 A3/A9 被「零调用者」排除的判据**表面冲突**，
  取哪一边要讲清楚：A3/A9 是**内部分析函数**，优化它们不改变任何用户可见耗时；
  `FAISSDB` 是 `__all__` 导出的**公开 SDK 类**，向量库的正常用法就是持续增量写入，
  平方级写入是使用者一定会踩的坑——所以本轮落地，但收益归属「SDK 使用者」而非本产品
  的既有请求路径。
  新增 7 例（`TestIncrementalAdditions`）：`numpy.vstack` 计数（两版都有的名字，
  非零即「整份重抄」）+ `_grow` 次数与**复制行数 ≤ 2n** 的摊销上界 + 逐条/一次写满
  逐行一致 + 落盘不得泄漏空槽 + 镜像集合跟住增删与改址 + 删除后行对齐。
  **红→绿**：整份退回 HEAD → `AssertionError: 逐条写入期间整份矩阵被重抄了 30 次`
  是本轮的计数红；另 4 例因 `_grow`/`_buffer`/`_id_set` 不存在而 AttributeError，
  属于新符号的结构护栏不是缺陷证据；**2 例两侧都绿**（逐条=一次写满、删除后对齐），
  它们是语义护栏，写在这里防冒充。5 个既有类 17 例全绿，`已还原=True`。
  **顺带实测排掉两处「看着像缺陷」**：① `search` 缓存命中路径上的
  `r["id"] in self._ids` 逐条线性校验，n=3000 / top_k=10 实测命中 **0.2 ms vs 未命中
  0.5 ms（0.35×）**，兜底并没有让命中比重算更贵，不改；`base.get_metadata()` 的
  `in` + `index` 两次全表扫同样落在这 0.2 ms 里，量级不值得。② `context.py:112-114`
  的「每条候选重算已有问题列表」是 O(n²) 形态，但它每一步都要跑一次 LLM 调用
  —— 这条**没有实测**，只按调用形态判掉，不当作证据。
- **L16** `feat(converter)` B3① —— 转换图此前是**单向**的：`json →` 有 8 条边
  （实测，Backlog 原记「导出侧 13 种」不准），`→ json` 只有 jsonl/csv 两条，
  导出的训练格式再也回不来。补 6 条反向边并接线（CLI `--input-format`、
  API `ConvertRequest.source_format`）。**接线不是可选项，而是这条功能的本体**：
  容器格式落盘也是 `.json`，`_infer_format` 只能看扩展名，于是
  「把 sharegpt 文件转成 chatml 而不声明源格式」在修复前后都是**退出码 0 的静默坏数据**
  —— 走 `json → chatml` 时 `conversations` 整个被忽略，产物是
  `{"role":"user","content":""}` + `{"role":"assistant","content":""}`。
  这条实测结果写成了用例（`test_omitting_the_flag_keeps_extension_inference`、
  `test_sharegpt_file_without_declaration_keeps_the_old_behaviour`），它同时钉住
  「老调用语义不变」和「为什么必须有显式声明」。
  口径决策三条，都写进了代码注释：① 反向**不给 `input` 补空串**（`validation` 里
  `input` 可选，凭空造键会让「源文件有没有这一列」失去可辨性，下游稀疏字段检测漏报）；
  ② 不认识的角色（`tool` / 函数调用）**报错不猜**，猜成 user 会把工具输出静默变成
  「用户说的话」；③ 对话必须以「user → assistant」结尾才还原问答，否则按位置硬切
  会把答案切错。`_to_json` 顺手从 if/elif 改成查表，`sharegpt → chatml` 这类
  「非 json 源 → 非 json 目标」因此自动经规范形中转。
  新增 **42 例**（34 单元 + 5 CLI + 3 API）：反向边的字段保留/复制语义、
  6 条 `json → 格式 → json` 的**逐家有损表**（alpaca/belle 丢 system+history、
  llama_factory 只丢 history、sharegpt 丢 input+system、vicuna 丢 input+system+history、
  chatml 只丢 input）全部用手写字面量钉住，8 类结构错误各带条目下标与字段名。
  **红→绿**：4 个源文件整份退回 HEAD → 新用例 **34 单元 + 4/5 CLI + 3/3 API 全红**，
  唯一绿的是白名单里那条「老行为不变」；对照（既有 61 单元 + 13 CLI + 2 API）全程绿。
  注入脚本本轮起**改为逐条列红/绿**而非只看组级退出码——第一版只看组级，
  `test_tsv_is_not_accepted_as_input_format` 在缺陷态靠「argparse 根本不认识
  `--input-format`」也算退出码 2 而混过去，属于假红依赖；补断言
  `invalid choice` 后才真正锁住「清单里排除了 tsv」这件事。
  同样被逐条模式抓出来的还有 API 的 happy path：第一版用 alpaca 样本，而 alpaca 记录
  本身带 `instruction`/`output`，按 json 读也能出对的结果，**测不出这条边**；换成
  sharegpt（只有 `conversations`）才是真依赖。
  **本轮无性能主张**：反向边是新能力，没有「修复前也能跑」的对照组可测。
  遗留 **B3②**：`tsv ↔ json` 两侧皆无边（CLI/API 的 choices 里刻意不放 tsv，与
  `get_supported_formats()` 同源）；`convert_file(source_format="alpaca")` 读 `.jsonl`
  容器文件会落到 `json.load` 而失败；`_json_to_csv` 里 `all_keys` 算完不用（死代码）。
- **L17** `perf(validation) + fix` A12① —— `_validate_item` 里每条数据 `import re`、每个字段
  每个模式一次 `re.search(字符串, ...)`。禁止模式预编译成 `re.Pattern` 并缓存，`import re`
  提到模块级。**实测（真实 6902 条 `train_data.json`、strict 预设、同进程交替 9 轮）**：
  整档校验 **28.99 → 19.49 ms（中位 1.49×，9/9 轮新快于旧，逐轮比 1.23–1.59）**；
  单独把「禁止模式那一段」拎出来 **15.44 / 16.37 → 8.71 / 9.14 ms（1.69–1.88×）**，
  它占整档耗时的 **53–55%** —— 这一段就是当初立 A12 的全部理由，也印证了 L13 那句
  预估（「~5 ms / 1.2×」）偏保守。basic 预设没有回退（新/旧 **0.92–0.94**）。
  确定性预言机：500 条数据上 `re.search` **2000 → 0 次**（提交进用例的 300 条即
  **1200 → 0**），`re.compile` **恒为 2**（= strict 的模式条数）与条数无关；
  strict / basic / chat 三套预设的 `(字段, 消息, 级别, 下标)` 元组序列**逐条相同**，
  tracemalloc 峰值 3.2 → 3.4 KB（多出的是一份编译结果，量级可忽略）。
  **顺带修掉一个新发现的缺陷**：`DatasetValidator(preset="strict")` 以前是
  `self.rules = self.PRESET_RULES[preset]`——把**类级字典本身**挂到实例上，于是任一实例
  `validator.rules["min_instruction_length"] = 5` 会污染该预设下**所有**验证器、
  并且跨请求存活（构造时改浅拷贝）。用例 `test_preset_rules_are_not_shared_state`
  用 `monkeypatch.setitem` 保证「泄漏」这一侧真能红。
  **红→绿（逐条）**：整份退回 HEAD 后，8 例新用例里 3 例按缺陷起因红——
  「仍有 1200 次逐条字符串模式搜索」「验证器每条数据 import 一次 re（300 次）」
  「改动污染了类级预设」；另 3 例**设计上两侧都绿**，它们是语义等价护栏
  （报错时机、大小写不敏感、问题清单逐条相同），写进注入脚本的 `ALLOW_GREEN` 白名单
  而不是靠它们冒充证据。
  **方法论补一条**：数 `re.compile` 的调用次数**不能**证明旧代码在重复编译——
  CPython 里 `re.search(字符串, ...)` 走的是 `re._compile`（另一个全局名），
  所以计数必须打在 `re.search` 上。
  **A16 本轮实测排除**（数字已写回 Backlog）：`dependency_register` 那个「只为一个整数
  解析整档」看着可优化，实际纯 Python 顶层扫描器比 `json.load` 慢 8 倍还数错条数。
  新增 **8 例**（`tests/unit/test_validation.py`，该文件现 44 例）。
- **L18** `perf(indexer)` A11 —— A7/L13 那个缺陷的**同构体**，藏在另一套实现里：
  `DatasetView.__init__` 无条件 `DatasetIndexer(items)`，构造即建齐 3 份字段倒排 +
  1 份 `instruction_2` n-gram（真实 6902 条实测 **70 ms / 峰值 19.2 MB**）。可是这些
  索引的读者只有 `search_exact` 与 `search_ngram`，而 `search()` 的**默认方法是
  contains（扫全表、一个都不读）**，`filter/head/tail/sample/切片` 也一律不读。
  改成「按份、按需」构建：`_ensure_field_index(field)` / `_ensure_ngram_index(field, n)`
  各自销账，`list_indexes()` 与 `get_statistics()` 先 `_ensure_default_indexes()`
  （对外「建了哪些索引」的口径一个字节都不许变）。
  **实测（真实 6902 条、同进程交替）**：`DatasetView(items)` 构造 **67.6 → 约 0 ms**；
  `view.filter()` 三轮 **132.73 / 141.12 / 142.60 → 0.44 / 0.44 / 0.67 ms**；
  链式 `filter().filter().head()` **194.98 → 0.90 ms**；`head(10)` / `[100:200]` /
  `sample(50)` 各自 **66–69 → 约 0 ms**；冷启动单次查询 contains
  **78.65 → 7.21 ms（10.9×）**、exact **68.83 → 8.35 ms**、ngram **64.97 → 56.82 ms**
  （ngram 只省掉 3 份倒排，它自己那 23020 个键仍要建 —— 这一路本来就不便宜）。
  内存：n=4000 构造视图后驻留 **3258.6 → 1.2 KB**（旧版峰值 11.7 MB）。
  **缩放形态**：旧版构造耗时随 n 线性（2000/4000/6000 条 = 15.65/34.08/54.79 ms），
  新版恒为 1–2 µs 与 n 无关。
  **确定性预言机（「建了几份索引」，每行都新建对象）**：构造视图 4→0、`view.filter()`
  8→0、链式三级 16→0、`head/slice/sample/to_list` 8→0、冷 contains 4→0、
  冷 exact 4→**2**、冷 ngram 4→**1**、`load()` 后不查询 4→0、`load()` 后一次 exact
  4→**1**；`list_indexes()` 与 `get_statistics()` **4→4**（这是有意的等价，不是漏改）。
  **等价性（真实 6902 条）**：三种 method × {如何 / 租房 / 写一篇} 的命中数与条目序列
  逐条相同；`_field_indexes` / `_ngram_indexes` 两个字典连**键序**都完全相等；
  exact 整条指令、ngram n=2/3、字段 instruction/output/自定义 的边界结果全部相同；
  空数据集的两种旧口径（`__init__([])` 报 0 份、`load([])` 报 4 份空索引）都保住。
  **三条口径决策**：① 清单外的字段**不建也不改语义**（`search_exact("answer", ...)`
  一直返回 `[]`），性能轮不顺手改语义，另立 **A23** 并写用例钉住现状；
  ② `load()` 连空数据也「欠账」，否则 `list_indexes()` 报出的份数会变；
  ③ **销账必须发生在建完之后**（`_pending_fields.discard()` 排在 `_build_*` 之后），
  配双检锁——先销后建会让并发进来的第二个线程以为已就绪、读到没有这个键的字典而
  把结果误报成「无匹配」；这条写成了 8 线程同发首查询的用例（并断言只建 1 份）。
  改成按需后 `_build_default_indexes()` 再无调用者，删掉。
  **顺带排掉一个「看着像 L17 同构」**：`search()` 里的 `import time` 是**每次查询**
  一次，不是每条数据一次，量级不值得动（L17 的 `import re` 之所以算缺陷，是因为它
  在 `_validate_item` 里、每条数据一次）。
  **收益归属**：`DatasetView` / `DatasetIndexer` 都在 `__all__` 里，本产品内部除
  `create_indexer/create_view` 外无调用者 —— 与 L15 的 `FAISSDB` 同一判据，
  这条买的是 **SDK 使用者**的耗时，不是本仓库请求路径的耗时。
  新增 **12 例**（`tests/unit/test_indexer.py::TestLazyIndexConstruction`）。
  **红→绿（逐条）**：`indexer.py` 整份退回 HEAD → 新用例 **10 红 / 2 白名单绿**
  （那两条是 `test_empty_dataset_keeps_its_old_report` 与
  `test_statistics_and_list_indexes_still_report_all_defaults`，钉的正是「对外可见
  口径不变」，设计上两侧都绿）；失败理由全是计数/内存断言
  （「视图操作白建了索引：[4 份]」「构造视图后仍驻留 2410.6 KB」「切片查询一份都没建」），
  不是崩溃。对照 6 组（既有 44 例 + branches 18 + rng_hygiene 10 + round90 4 +
  round100 2 + CLI 合并命令 72）全程绿，恢复后全绿。
- **L19** `feat(indexer)` A23 —— 把 L18 **刻意留下**的那条口径缺陷修成真能力。
  L18 让索引变成「按份、按需」之后，「哪些份」仍由两张默认清单写死：
  `_ensure_field_index` 开头一句 `if field not in DEFAULT_INDEX_FIELDS: return`，
  `_ensure_ngram_index` 同理。后果不是「慢」而是**静默查不到**：调用方拿到 `[]`，
  真相却是「我压根没为这个字段建过索引」。判据换成
  「**要么欠着默认清单的账，要么手上真有数据可建**」（`or self._items` 保住空数据集
  不凭空造空索引，`list_indexes()` 的两种旧口径原样不动），清单从此只决定
  「报告里该有哪几份」，不再决定「能查什么」。
  **真实数据上的症状比 L18 记录的更重**：`search(query, method="ngram")` 的默认字段
  是 `instruction`+`output`，可 `output_2` 从来没被建过 —— 也就是说这条查询**一直在
  静默少报 output 侧的全部命中**，而且它返回的是正常 `QueryResult`、HTTP 侧是 200。
  实测（真实 6902 条）：ngram「租房」**419 → 823**、「如何」**2019 → 2035**（两者修后
  与 contains 的命中数逐条相同，contains 不受影响所以正好可作对照），
  「写一篇」6 → 6 不变；`search_ngram("output", "，晨")` **0 → 8**，同一份数据的朴素
  全表扫描就是 8。新能力：`search_ngram("instruction", …, n=4)` **0 → 5**、
  `search(fields=["answer"], method="exact")` 由空手而归变成真命中。
  **代价同轮实测，没有藏**：冷 ngram 查询 **55–65 → 225–240 ms**，因为要多建一份
  `output_2`（单份 170.9 ms；`instruction_2` 58.7 ms）；临时倒排一份只 4.3 ms
  （`search_exact` 的按需建索引跟文本长度线性，n-gram 跟字符数 × n 线性）。
  这条成本另立 **A24**（「一次性 n-gram 查询付整份索引的钱」），L19 不动它 ——
  一轮只改一类口径，否则红→绿归因就混了。
  **L18 的收益没回退**（同进程交替，真实 6902 条）：构造索引器 0.00 ms、
  `indexer.filter()` 0.17 ms、冷 contains 7.31 ms、冷 exact 7.87 ms、
  `list_indexes()` 61.9 ms 且仍报 4 份、合成 4000 条构造视图后驻留仍 1.2 KB。
  **建索引份数预言机（每行全新对象）**：默认路径全部不变（构造 0、filter 0、
  冷 contains 0、冷 exact 2、`list_indexes()` 4），变化只在本来就不该空手而归的几行 ——
  冷 ngram 查询 **1 → 2 份**（`instruction_2` + `output_2`）、n=3 **0 → 1**、
  清单外字段 exact **0 → 1**、清单外字段 ngram **0 → 1**、`fields=["answer"]` **0 → 1**。
  **等价性**：默认流程下 `_field_indexes` / `_ngram_indexes` 连**键序**都逐项相等；
  contains / exact 九组查询命中序列逐条相同；空数据集三种口径（`DatasetIndexer([])`
  报 0 份、`load([])` 后报 4 份、清单外字段返回 `[]`）两侧全等。
  **一条有意的口径变化**：临时索引建了就会被 `list_indexes()` / `get_statistics()`
  报出来（顺序是先建的字段在前、再补齐默认清单），不再等于「默认清单四项」。
  把它藏起来等于新增一层「报的不是真相」，与本轮方向相反，所以选择暴露 + 用
  `test_ad_hoc_index_appears_in_the_report` 钉住。另一条后果：`_field_indexes` 的上限
  从「清单里的 3 份」变成「被查过的字段种类数」——一个索引器活多久它就长多久，
  量级是每字段一份 O(文本总量) 的倒排。本仓库没有任何请求路径会拿用户输入当字段名
  （`DatasetIndexer.search` 在 `create_indexer/create_view` 之外零调用者，CLI/API 走的是
  `search_enhanced`），所以这不是可被外部放大的增长；SDK 用户若拿不受信的字段名查，
  需要自己复用同一个索引器的生命周期。
  **同构排查**：`search_enhanced.py` 没有这个缺陷 —— `_build_indexes()` 遍历
  `item.items()` 收全部字符串字段（无清单），`_search_ngram` 走扫表（与 `n` 无关）。
  顺手把 **A5** 也判了：`pipeline.py` 那处逐条打分**不能**换成 `batch_score()`，
  因为后者的多样性参照集是渐进累积的（`quality.py:417-433`），换了会静默改变
  `passed` 判定 —— 理由已写进 Backlog，未做任何代码改动。
  **用例**：新增 **8 例**（`tests/unit/test_indexer.py::TestArbitraryFieldQueries`，
  含一条 8 线程同发清单外字段首查询只建 1 份），改写 **3 例**：L18 的
  `test_non_default_field_query_stays_empty_without_building`（钉的正是被修掉的缺陷）
  换成 `test_ad_hoc_field_on_empty_indexer_builds_nothing`（钉住保留下来的豁免），
  `test_ngram_query_builds_only_its_own_ngram_index` 与
  `test_each_index_is_built_once_and_reused` 的期望从 1 份 n-gram 改成 2 份，并给前者
  补了一条「命中数必须等于朴素全表扫描」的交叉校验，防止用「少建一份索引」冒充「省」。
  **红→绿（逐条）**：`indexer.py` 整份退回 HEAD → 新用例 **10 红**（8 + 改写的 2）、
  白名单 1 绿（`test_ad_hoc_field_on_empty_indexer_builds_nothing`，设计上两侧都绿），
  失败理由全是「命中数/建索引份数不符」（`assert 0 == 3`、`assert [] != []`、
  `['ngram:instruction_2']` 少一份），不是崩溃。对照 6 组（TestLazyIndexConstruction
  余下 10 例 + branches 18 + enh 1 + search_enhanced 52 + CLI 合并命令 72 +
  round90 4 / round100 2）全程绿，恢复后全绿。
- 全量：L4 后 **3679 passed / 3 skipped**（89.2 s），L5 后 **3703 passed / 3 skipped**
  （90.2 s），L6 后 **3705 passed / 3 skipped**（91.1 s），L7 后 **3726 passed / 3 skipped**
  （95.1 s），L8 后 **3747 passed / 3 skipped**（98.0 s），L9 后 **3747 passed / 3 skipped**
  （92.4 s）——L9 用例数不变是**有意**的：它**改写**了既有的结构护栏用例而非新增。
  L10 后 **3750 passed / 3 skipped**（55.7 s，coverage.xml line-rate 0.9927），
  L11 后 **3780 passed / 3 skipped**（101.7 s，coverage.xml 总计 98.51%），
  L12 后 **3792 passed / 3 skipped**（64.3 s，coverage.xml 总计 98.51%），
  L13 后 **3802 passed / 3 skipped**（65.8 s，总计 98.50%），
  L14 后 **3809 passed / 3 skipped**（52.0 s，总计 98.51%），
  L15 后 **3816 passed / 3 skipped**（51.3 s，总计 98.51%），
  L16 后 **3858 passed / 3 skipped**（56.4 s，总计 98.52%），
  L17 后 **3866 passed / 3 skipped**（54.0 s，总计 98.52%），
  L18 后 **3878 passed / 3 skipped**（54.8 s，总计 98.52%），
  L19 后 **3886 passed / 3 skipped**（57.4 s，总计 98.53%）。
  **注意**：这些墙钟秒数**彼此不可比**——本工作树与并行 agent 共用一台机器，
  它跑全量时我会慢 40%+（L9 时 92 s、L10 时无竞争 55.7 s）。跨轮只比
  **同一进程内 back-to-back 的对照组**，绝对秒数只作当次快照。
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
