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
- [x] **L20** `feat(converter)`: B3② —— 读写两侧终于分清「文件容器」与「行内 schema」（新导出 `CONTAINER_FORMATS`）：补齐 `json ↔ tsv` 两条边（公开清单 9 → 10，与 `DataFormat` 同集合），声明了 `source_format="alpaca"` 却因落盘是 `.jsonl` 而 `json.load` 崩掉的读侧改成按内容嗅探，写侧容器改由输出扩展名决定（`--format alpaca` 写 `.jsonl` 现在真是一行一条、能原样读回）。真实 6902 条往返 **49.7 / 59.9 ms**、tsv **34.5 / 55.5 ms**；另删 `_json_to_csv` 的死代码；新增 20 例、删 2 例、改写 1 例，另立 **A25**
- [x] **L21** `fix(converter)`: A25 —— 「源格式压根没声明」时的静默坏数据变成响亮报错：护栏只有**一处**，挂在 `convert()` 的分发口（`converter.py:212-213`，L22 后行号）而不是六条 `_json_to_*` 写边里，判据取窄（记录**既取不到 `instruction`/`output`**、**又带着对话数组** `conversations`/`messages` 且必须 `isinstance(list)`）才抛 `DataFormatError`，文案含条目下标 + 「该声明哪个 `source_format`」的可执行建议，且发生在写盘**之前**。真实 6902 条这一路以前产出 **6902 条全空问答**（96.9 ms、退出码 0、HTTP 200），现在 **16.7 ms 内失败、零半截产物**；干净数据的转换**耗时增量 0.0 ms**（3.8 vs 3.8 ms）。新增 19 例、改写 2 例（把钉住「静默空数据集」的旧例改成钉住「响亮失败」），另立 **A26**
- [x] **L22** `fix(converter)`: A26① —— 「记录不是 JSON 对象」在八条写边入口变成 400：`[null]` / 字符串 / 数字 / 嵌套数组以前把整条链路砸成 `AttributeError: 'NoneType' object has no attribute 'get'`（CLI 文案就是这句解释器内部措辞、API **500**），写 `csv`/`tsv` 更坏——`DictWriter` **先把半截文件落盘再崩**（实测产物 `j,u,s,t, ,a,r,i,n,g`，字符串被按字符展开成表头）。护栏同样只挂**一处**（`convert()` 分发口 `converter.py:203-207`，新导出 `PASSTHROUGH_TARGETS` 写明 `json`/`jsonl` 两类豁免原因），文案与 L16 读边对齐（「第 N 条记录必须是 JSON 对象，当前是X」）。真实 6902 条干净数据、同进程 back-to-back 实测代价：`alpaca` **+0.23 ms**（×1.18）、`chatml` **+0.49 ms**（×1.13）、`csv` **+0.14 ms**（×2.34，基数只有 0.11 ms）、豁免路径 `jsonl` **×1.00** 作噪声对照；n/2n/3n 缩放仍线性。新增 27 例、改写 1 例，A26 只剩 ②（既无问答也无数组 → 静默空产物）
- [x] **L23** `fix(converter)`: A26② —— 「整档一条问答都取不到」从静默空产物变成响亮报错：`{"question":…,"answer":…}`、`{"text":…}` 这类字段名认不出的语料走 `json → 六个训练格式`，以前每条都读到 `""`，**退出码 0 / HTTP 200**、产物结构合法却零问答（比崩溃更坏，下游直接拿去训练）。判据仍只挂**一处**分发口（`converter.py:214`，`_reject_all_empty_qa`），且**只看整档不看单条**（个别记录缺问答是常态）。问答键**按目标算**（`_qa_keys()` + `_HISTORY_AWARE_TARGETS`）：六条写边只有 sharegpt/chatml 读 `history`，因此「全是 history」转 chatml 放行、转 alpaca 照样拦。实测四份真实语料共 **20706 条**，可用问答缺失 **0 条** → 判定不误伤；干净数据单次调用 **0.4~0.7 µs**（命中第一条即返回），报错路径整档扫描 **1.32 ms / 6902 条**（一次性代价）。新增 31 例、改写 1 例（L21 那条钉「字符串 `messages` 列转出来仍是空问答」的豁免例，本轮把它改成钉「由 L23 接管、且不得给出声明源格式的建议」），**A26 两条口子全部关闭**
- [x] **L24** `perf(search_enhanced)`: A21 的 ngram 那一半 —— `_search_ngram` 不再**为每条文档建一整个 gram 集合**：查询串先切成 k 个 n-gram，判「文档命中某 gram」只需 `gram in 文档`（子串关系与建集合完全等价），于是每文档的工作量从 O(文档长度) 次 set 插入降到 O(k) 次子串查找。真实 6902 条单字段 **22.2 → 3.2 ms（中位 6.03×，20 字长查询最不利 2.35×）**，默认三字段端到端 **104–113 → 12–16 ms（中位 7.57×）**，ngram 从「比 contains 慢一个数量级的可用方法」（旧 **104–113 ms** vs contains ~8 ms）变成同量级（新 **12–16 ms** vs contains 8.2 ms，约 1.6×）；缩放对照 n=2000/4000/6000 的新旧比 **6.60× / 6.58× / 6.85×**（比值不随规模漂移）；确定性预言机：`_ngrams` 的调用次数 **6902 次（逐文档）→ 1 次（只有查询串）**。新增 20 例（12 例是与旧实现逐条等价的参数化对照），A21 只剩 contains/fuzzy/regex，另立 **A27**（fuzzy 在中文语料上恒 0 命中）
- [x] **L25** `fix(search_enhanced)`: A27 —— `method="fuzzy"` 从「按整段中文 token 算 Jaccard、恒 0 命中」换成**等长滑窗 + 阈值门 + 鸽笼预筛**：分数 = `max(1 - 错配数/查询长度)`（只替换、不增删，是 Hamming 不是编辑距离），且**必须凑不出任何一片**才淘汰候选（片数 = 错配预算 + 1），所以预筛是纯优化、不改语义。真实 6902 条：「公租屋」 **0 → 63 命中**、带错字的「腿租押金」在 CLI 里 contains **0** 而 fuzzy **1**（这就是它存在的理由）；「租房」 **823 命中 = contains 逐条相同**（子集不变式：含即满分）。**代价同时实测**（min-of-7，三字段端到端）：「租房」 21.23 ms（contains 8.06 / ngram 13.05）、「公租屋」 174.41 ms、「租房合同」 167.02 ms/159 命中（旧 fuzzy 48.45 ms 却**返回 0 条**）—— 单字段扫窗是主开销，鸽笼预筛在 `output` 上省 **2.94–22.47×**（60.78 vs 1365.64 ms 等三档，命中集合逐档 SAME），`input` 全空档 0.98–1.00× 作无开销对照。等价性：真实语料随机 **70 组**（3 字段 × 5 阈值 × 含错字查询）与朴素全扫 oracle **0 不一致**，另 42 组参数化用例常驻。新增 51 例（unit 单文件 72 → 122），改写 2 处**钉住旧缺陷**的期望（API 的「fuzzy 公租屋 期望 0 命中」、CLI 的模糊「找到」），另立 **A28**（fuzzy 现在是真全表扫，最贵的方法）
- [x] **L26** `feat(search_enhanced)`: A28④ —— fuzzy 的阈值与 ngram 的 gram 长度从**私有方法的默认参数**提到三端公开入口（`search()` / `search_dataset()` 的 `fuzzy_threshold` / `ngram_n`、CLI `--fuzzy-threshold` / `--ngram-n`、API `SearchRequest` 同名字段）。**为什么值得做**（只能绕私有方法量的真实 6902 条）：同一句「租房合同」阈值 0.5 → **1272** 命中、0.6 → 159、0.8 → 38；「公租屋」0.6 → 63、**0.7 → 0**（错 1 字 / 3 字 = 0.667 正好掉出门槛）；ngram 同查询 n=1 → 4565、n=2 → 1271、n=3 → 42 —— 命中数**差到 33 倍**的旋钮，此前用户一个都摸不到，只能改库源码。**症状的另一半**：API 的 pydantic 对未知字段**默认吞掉**（注入态实测 HTTP **200** + 默认阈值的结果，而不是 422），所以「传了没生效」在 HTTP 层是完全静默的。校验只挂 `search()` **一处**（承 L21/L22 的共同咽喉），越界抛 `DataValidationError`（双继承 `ValueError` → API 照旧 400、CLI 照旧 `错误: …` + 退出码 1）；默认值 = 原行为，另用 5 例参数化钉「不传 ≡ 显式传 0.6/2」。计数预言机：坏参数时五种 `_search_*` 被调 **0 次**（校验发生在扫表之前），同一探针再以合法参数跑一遍证明探针是响的。**自家结构护栏抓了我一次**：第一版写 `raise ValueError`，全量红在 `test_no_bare_builtin_raises_left_in_package` → 改领域异常。新增 33 例（unit 122→144 / CLI 73→79 / API 207→212），注入 **33 红 / 406 绿**（红的恰是全部新例、绿的恰是全部既有例），全量 **4106 passed / 2 skipped**（98.98%），`search_enhanced.py` 211 语句 0 missed；另立 **A29**（生效的旋钮不回显）
- [x] **L27** `feat(search_enhanced)`: A29 —— `SearchResult` 回显**本次真正生效**的松紧旋钮（新增 `fuzzy_threshold` / `ngram_n` 两字段，`to_dict()` 一并导出），口径取「**方法没消费它就是 `None`**」而不是一律回显默认值：`contains` 旁边印个 0.6 会让人以为阈值管得到它，那与 A27/A28④ 是同一类静默。落点：**两条「找到 0 条」终于可分辨**——「公租屋」阈值 0.667 → 0 条（最优窗口分数正好 0.667，差一点就过）vs contains 同查询 → 0 条（语料里真没有）。CLI 把生效值印在**第二行末尾**（`搜索方法: fuzzy (生效阈值 0.6)`）而不是新起一行，因为下游按 `out[out.index("["):]` 取条目，「两行摘要 + JSON」是既有契约；`--output` 落盘的字典自然带两键。API 侧必须**同时**在 `SearchResponse` 声明这两键——FastAPI 按 `response_model` 过滤返回值，模型少写一键就**静默**从响应里消失（本轮由既有的 `test_api_openapi_contract.py` 抓到，红信息直接写出「response_model 漏声明字段…会被静默裁掉」）。代价实测（同进程 back-to-back min-of-7，真实 6902 条）：默认路径四组比值 **0.9834 / 0.9994 / 0.9873 / 1.0000**（命中集合逐条相同 823/823/38/0），`to_dict()` 单次 0.00014 → 0.00017 ms（**+30 ns** 换两个键）。新增 26 例（unit 144→155 / CLI 79→87 / API 212→219）+ 更新 1 处既有契约用例（该用例在缺陷态驱动 **2** 条红），注入 **24 红 / 590 绿**（其中 4 绿是白名单：3 条钉「不消费旋钮的方法不许印数字」+ 1 条钉 stdout 形状，缺陷态必然绿），全量 **4132 passed / 2 skipped**（98.98%，4106 + 26 = 4132 精确对上）；另立 **A30**（`SearchFilter` 的九种算子只在 SDK 里摸得到）
- [x] **L28** `feat(search_enhanced)`: A30 —— `filters` 从「只有 import 库类才用得到」接到三端（`search_dataset(filters=)` / CLI `--filter FIELD OP VALUE`（可重复）/ API `SearchRequest.filters`），并在 `search()` 的**同一处咽喉**新增公开 `normalize_filters()`，把四种旧行为里**静默或崩溃**的坏形状变成 `DataValidationError`：未知算子（旧：**0 命中**，与「语料里没有」同形）、`in`/`not_in` 配标量（旧：**0 命中** / **全留**，即「没过滤」被伪装成「过滤通过」）、值类型不符（旧：`TypeError` 冒到 API 成 **500**）。顺手修掉同族的第五条：`filters` 传**字典**在 HEAD 里是 `AttributeError: 'dict' object has no attribute 'field'`（实测，崩在扫描第 445 行深处），现在字典是与 `SearchFilter` 对象等价的合法形态。**一轮只改一类口径**：判据只看**用户给的参数**，文档字段类型不对仍是「不匹配」而不是报错（那是数据）；`_evaluate_filter` 的 11 条私有方法既有用例因此**一条都不必改写**。CLI 的算子清单与咽喉 `FILTER_OPERATORS` 是跨模块两份手写表（`--filter` 的 `nargs=3` 用不了 `choices=`，实测 argparse 会把 FIELD 也比成算子 → `invalid choice: 'output'` + 退出码 2），于是照 `EXPORT_FORMATS` 的先例由新用例 `TestSearchFilterSurface` 双向钉住（集合并**顺序**相等 + 九个算子逐个真跑通）。实测收益（真实 6902 条）：contains「租房」`instruction` **419 → 叠加 `instruction contains 申请` = 10 条**（收窄 42 倍）。代价（同进程 back-to-back min-of-7）：五组比值 **0.976 / 0.971 / 0.991 / 0.983 / 1.003**（不传 `filters` 的热路径零回归，`normalize_filters(None)` 立即返回），命中集合新旧逐条相同。新增 **59 例**（unit 155→188 / CLI 87→94→**104** / API 219→228），**既有例 0 改写**，注入 **58 红 / 615 绿**（1 绿是白名单 CONTROL：空 `filters` ≡ 不过滤，缺陷态必然绿，守的是「新校验没改动无过滤路径」），全量 **4191 passed / 2 skipped**（98.98%，4132 + 59 = 4191 精确对上），`search_enhanced.py` 213→**243** 语句 0 missed、`cli/parser.py` **199 语句 100%**；另立 **A31**（生效的过滤器不回显）、**A32**（`normalize_filters` / `FILTER_OPERATORS` 不在包级导出 + API 请求侧 schema 不表达算子域）
- [x] **L29** `feat(search_enhanced)`: A31 —— 承 A29 的回显口径补齐第三种「找到 0 条」成因：`SearchResult` 新增 **`applied_filters`**（**规范化之后**的 `{field, operator, value}` 字典清单，不外泄调用方对象）与 **`matches_before_filters`**（过滤**之前**的候选数），三端各自落到能读的地方——CLI 第二行末尾追加「`(N 个过滤器: 检索 X → 保留 Y)`」（继续不另起一行，`out[out.index("["):]` 是既有契约）、`--output` 与 `/api/dataset/search` 各带两键（`SearchResponse` **同步声明**，否则 FastAPI 静默裁剪，L27 已实测过一次）。口径承 L27：没过滤是 `None` 而不是 `[]` / `0`，否则「没过滤」与「过滤后剩 0 条」重新同形；`matches_before_filters` 与 `total_matches` 都是**分页前**的全集口径。顺带一条规范化：成员档收到 `set` 时落成 `list`，因为回显取自规范化产物而集合进不了 JSON（`--output` / 响应都会 `TypeError`）。实测收益（真实 6902 条）：contains「租房」419 条候选 → `instruction contains 申请` 后 **10** 条，且「419 → 10」这句话现在三端都印得出来；「检索 2 → 保留 0」与「检索 0 → 保留 0」在 stdout 与 JSON 上首次可分辨。代价（同进程 back-to-back min-of-7，四组比值 **0.990 / 0.997 / 1.007 / 0.983**，绝对值 1.82/1.80 · 1.79/1.78 · 1.75/1.76 ms + fuzzy 17.47/17.16 ms）——回显在 `if normalized_filters` 分支内、逐条判定外，所以**真正的护栏是计数式**：`SearchFilter.to_dict` 每次查询调用数 == 过滤器条数（不是候选数），带 1 个过滤器时非 0 以自证探针是响的。新增 **38 例**（unit 188→207 / CLI 104→114 / API 228→237），**改写 2 处既有用例期望**（L27 的 `to_dict` 键集合 8→10、旧构造用例扩两字段）+ **1 处契约期望 8→10**（后者驱动 2 条红），注入 **40 红 / 667 绿**（2 绿在白名单：`test_unfiltered_search_prints_no_filter_section` 钉「没传参数不许印数字」、`test_stdout_shape_is_still_two_summary_lines_then_json` 钉下游形状，两者缺陷态必然绿），全量 **4229 passed / 2 skipped**（109.62 s，98.98%，4191 + 38 = 4229 精确对上），`search_enhanced.py` 243→**251** 语句 0 missed；另立 **A33**（多条过滤器只有合计数，说不出是哪一条把候选清零）、**A34**（集合值回显顺序跨进程不稳定，实测）
- [x] **L30** `perf(dedup)`: A17 剩下的那一半（稀疏化）—— fallback 去重**不再物化 n×vocab 矩阵**，改由「倒排 postings 散射累加」产出同形状相似度块，两条实测账选路。实测（真实 6902 条 `instruction`，min-of-3 同进程 back-to-back）端到端 **969 ms vs 10484 ms（比值 0.092，≈10.8 倍）**、峰值 **149.1 MiB vs 621.8 MiB（4.17 倍）**、`duplicate_groups` / `kept_indices` **逐组完全相同**（369 组 / 移除 377 条）；逐块最大绝对差 9.537e-7，阈值 0.9 上**判定不同的单元 0 个**、离阈值最近的一对还有 2.644e-4。根因**不是** A19 那条 Python 双循环，而是 **BLAS 在 0.12% 密度的矩阵上乘零**：nnz 只有 194,293 / 6902×23033=1.59 亿格。选路判据两条独立账：时间 `pair_adds×3000 < n²/2×vocab`、内存 `n×vocab×4×100 > pair_adds×block_rows×96×125`（单价 96 B 由七档实测峰值拟合，比值 0.97～1.13）。**本轮内自我推翻一次**：内存账最初写成「稠密矩阵超过 64 MiB 就换」，随后逐档实测随机语料 n=1200…7200，发现倒排峰值**一直是稠密的 1.4～2.4 倍**（n=4800：171.3 vs 85.7 MiB 且慢 7%），固定 MiB 线会在其中三档判错方向 → 换成按「最大那个块的 pair 级临时数组」估算。收益边界实测：`output`（V=17450、pair_adds 1.466 亿）与 `input`（V=0）两条账都判负、保持旧行为；全重复语料（V=9）倒排慢 69 倍同样被挡。新增 **42 例**（`test_dedup.py` 52→94），**既有断言 0 改写**，注入 **42 红 / 0 绿**（本轮无白名单绿例）、既有 52 例在缺陷态全绿，全量 **4271 passed / 2 skipped**（48.94 s，总计 98.98%，**计数 4229 + 42 = 4271 精确对上**），`dedup.py` 222→**266 语句 0 missed**、只余 `171->169`（`_fallback_encode` 里 `if ngram in vocab` 恒真的死分支，先于本轮，并入 A19）；另立 **A35**（`find_similar_pairs` 仍构造 n×vocab）、**A36**（阈值恰好压在「数学上精确相等」的相似度上时判定由 float32 最后位决定，实测 9 对 / 18 单元）、**A37**（倒排峰值仍随 `pair_adds×block_rows/n` 增长，149.1 MiB 远高于「一个块」的 16 MiB 上限）

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
| ~~A25~~ | ~~`augmentor/converter.py`（`_json_to_*` 六条写边）~~ | ~~**L20 新记（静默坏数据，与 B3② 同族但方向不同）**：源格式**未声明**时按扩展名当 json 读，而 `json → chatml/sharegpt/…` 只认 `instruction` / `output` / `history` 三个键。一份实际是 sharegpt 的 `.json` 于是被逐条读成「没有问答」，产物是**空问答的合法文件**、退出码 0、HTTP 200。L20 修的是「声明对了却绊在容器上」，这一条是「压根没声明」，嗅探帮不上~~ **已修（L21）**：护栏挂在 `convert()` 的分发口而不是六条边里，判据取窄——记录**取不到 `instruction`/`output`（任一非空）却又带着对话数组**（`conversations` / `messages`，且必须 `isinstance(list)`）才抛 `DataFormatError`，报错带条目下标 + 「该声明哪个 `source_format`」的可执行建议，且发生在写盘之前。**真实 6902 条实测**：这一路以前产出 **6902 条全空问答**（96.9 ms、退出码 0），现在 16.7 ms 内失败、零半截产物；干净数据转换的耗时**增量为 0.0 ms**（3.8 vs 3.8 ms，与写边同趟）。三端各有用例（SDK 17 例 / CLI 2 例 / API 1 例），六条写边由参数化逐个覆盖 | S |
| ~~A26~~ | ~~`augmentor/converter.py`（同一批 `_json_to_*` 写边）~~ | **L21 新记（护栏刻意没管的两个口子）**：①~~列表里躺着**非对象条目**（`[null]`、`["x"]`）或 `data` 根本不是列表时，写边直接 `item.get(...)` → `AttributeError: 'NoneType' object has no attribute 'get'`，经 API 就是 **500**——而同族 shape 问题（顶层必须是数组、记录缺字段）早已是 400。修法：写边入口统一做一次「条目必须是对象」，与 `read_conversation_turns` 同口径~~ **已修（L22）**：一处判据挂在 `convert()` 分发口，八类要取字段的目标全覆盖（六 schema + csv/tsv），实测代价 `alpaca` +0.23 ms / `chatml` +0.49 ms（6902 条，同进程对照），且 csv/tsv 不再留下按字符展开的半截产物；`json`/`jsonl` 目标是原样序列化、明确豁免。②~~**仍未修**：记录既无问答也无对话数组（`{"text": …}`、`{"question":…,"answer":…}`）时静默产出空问答；L21 不收是因为给不出「该声明什么」的建议，②的合理判据是「整档零可用问答 → 失败」，需要实测确认没有合法数据集恰好全空~~ **已修（L23）**：`_reject_all_empty_qa` 同一咽喉下整档判定，一条可用问答都取不到 → `DataFormatError`（带条数 + 所认的键）。**误伤前置实测**：train_data / train_data_final / final01 / final02 / up 共 **20706 条**，缺可用问答 **0 条**；问答键**按目标算**（只有 sharegpt/chatml 读 `history`），容器目标与 `[]` 明确豁免。干净数据单次 **0.4~0.7 µs**（早退），报错路径 1.32 ms / 6902 条 | S |
| ~~A27~~ | ~~`augmentor/search_enhanced.py:129-140` × `:308-336`~~ | ~~**L24 新记（结构性死方法，A23/A26 同族：静默返回空）**：`method="fuzzy"` 在中文语料上**任何阈值都取不到命中**。根因不是阈值，是**分词粒度**：`_tokenize` 的 `re.findall(r'[\u4e00-\u9fff]+\|...')` 把**一整段连续中文当成一个 token**，于是查询 `{'租房'}` 与文档 `{'如何申请租房', ...}` 的交集恒空 → Jaccard 恒 0。真实 6902 条实测：「租房」三字段 fuzzy **命中 0 / 48.45 ms**（contains 同查询 823 命中、8.20 ms），把 threshold 从 0.6 一路降到 0.02 命中数**仍是 0**，output 侧相似度 top3 = `0.0, 0.0, 0.0` —— 即「扫了全表、付了最贵的钱（fuzzy 是五种方法里唯一比 contains 慢的）、返回空」。它是 CLI 与 `/api/dataset/search` 的公开可选项，用户点得到。修法方向：fuzzy 改在**字符 n-gram / CJK 逐字**集合上算相似度（`_ngrams` 已在 L24 变便宜），并逐条与朴素 oracle 对照；**必须先实测英文侧会不会被改掉**（现在 `[a-zA-Z]+` 是按词的，逐字符化会让 `cat` 与 `cart` 突然相似），必要时按脚本分流。同一处 `:122` 的 `_build_indexes` 也吃这个 token，所以 `exact` 对中文整段才算一条键（「租房」0 命中属定义内，但同样值得在案）。~~**本轮没动**（L24 的边界是 ngram 的性能口径，不顺手改语义）~~ **已修（L25）**：不再碰 `_tokenize`，改成**与分词无关**的等长滑窗 Hamming（`_search_fuzzy`），`_tokenize` 一行没改 → `exact` / `_build_indexes` 的中文整段键**保持原语义**（A27 里那半条「顺带值得在案」的怀疑没有被顺手改掉）。**当初提的「必须先实测英文侧会不会被改掉」有了答案：会，而且改的方向正是这个方法的存在理由**——实测 `"buy a cart today"` 上「cat」旧实现 `{}`（按词 token，`{'cat'}` ∩ `{'cart','buy',…}` 恒空）→ 新实现 **0.667**、「cars」 **0.75**，即错一字母的拉丁词现在能互相命中；反过来 `AAAa` vs `aaaa` 两侧都是 **1.0**（`lower()` 在前，大小写不参与错配）。所以**没有按脚本分流**：分流会把「错字容忍」人为切成两种语义，而旧实现真正的缺陷是「任何语言的错字都判不相似」。真实语料侧的不变式也复核了：「2024」fuzzy **9 = contains 9**（数字串同样守子集不变式）、「rent」两侧 **0**（该词不在语料里，fuzzy 不该凭空造命中）。真实语料随机 70 组（3 字段 × 5 阈值 × 含错字查询）与朴素全扫 oracle 对照 **0 不一致**。子集不变式（contains 命中必是 fuzzy 满分命中）在真实数据与用例两侧都成立。代价另立 **A28** | M |
| A28 | `augmentor/search_enhanced.py:308-360`（`_search_fuzzy`） | **L25 新记（成本，不是缺陷；A21 同族）**：修好语义之后 fuzzy 成为**五种方法里最贵的一档**，因为每文档要扫 `len(文档)-len(查询)+1` 个窗口 × `len(查询)` 次字符比较。真实 6902 条 min-of-7 实测：三字段端到端「租房」 21.23 ms / 「公租屋」 174.41 ms / 「租房合同」 167.02 ms（同一查询 contains 只有 7–12 ms）；单字段拆开更清楚——`output` 长字段 60.78 / 478.50 / 430.37 ms，`instruction` 5.86 / 18.90 / 15.84 ms，`input`（全空）0.82 ms。鸽笼预筛已经把「无筛」的 1365–1612 ms 砍到 430–480 ms（2.94–22.47×），**剩下的钱在窗口本身**：短查询 + 高预算（`allowed` 大 → 筛片少而文档多）时退化最明显。可修方向按性价比排序：①**按长度预筛**（`len(文档)` 与 `len(查询)` 差距过大时窗口注定不达标？——不行，等长窗口允许任意偏移，长度不设上界，需要先确认语义是否允许「窗口必须在文档内」这条本身收紧）；②把 `zip` 逐位比较换成 `sum(a != b for ...)` 的向等价（如 `os.path.commonprefix` 式的分块比较）；③建 **trigram 倒排**做候选集，再对候选精算（与 A21/A24 的「n-gram 倒排」同一套基础设施，应合并决策）；④~~公开 `--threshold` / API 字段（CLI parser 现在**没有** `--threshold`，只有 `search_dataset(..., threshold=)` 的默认 0.6，用户想放宽只能改代码——属于 B 类接线，缺的是入口不是算法）~~ **已做（L26）**，顺带纠正 L25 那句话里的错：`search_dataset()` 当时**根本没有** `threshold` 形参（我记错了），唯一能调的地方是私有方法 `_search_fuzzy` 的默认值；L26 把两个旋钮（阈值 + ngram 的 `n`）一起提到 `search()` / `search_dataset()` / CLI / API 四处，默认值不变。**A28 剩下的仍是 ①②③（窗口本身的成本）** | M |
| A29 | `augmentor/search_enhanced.py`（`SearchResult` / `search()`）+ `augmentor/cli/commands/data_ops.py`（`run_search` 摘要） | **L26 新记（回显缺失，A27/A28④ 同族的「用户无从判断参数起了作用」）**：旋钮能调了，但**结果不告诉用户当时生效的是哪一档**。`SearchResult` 只带 `query` / `method`，CLI 摘要只印 `搜索方法: fuzzy`，于是「找到 0 条」仍然分不清是「语料里没有」还是「阈值拧太紧了」——而 L26 实测过这两者的差别有多大：「公租屋」 0.6 → 63 条、0.7 → **0** 条。修法：`SearchResult` 加 `fuzzy_threshold` / `ngram_n` 两个字段（`to_dict()` 一并导出，API 响应自然带上），CLI 摘要行跟着印实际生效值；**要一并决定**：非 fuzzy 方法时这两个字段是 `None`（诚实）还是重复默认值（会让用户以为阈值管到了 contains）。**已做（L27）**：取 `None` 那一支，并且**没消费的方法连印都不印**（CLI 第二行只在有值时加括号）；两键都进 `to_dict()`，`SearchResponse` 同步声明（否则 FastAPI 静默裁掉，本轮由既有契约用例抓到）。实测代价：默认路径四组比值 0.9834–1.0000、`to_dict()` +30 ns | S |
| A30 | `augmentor/search_enhanced.py:202`（`search(filters=)`）× `:529`（`search_dataset`）**没有** `filters` 形参 × CLI `search_parser` × `SearchRequest` | **L27 新记（与 A28④ 同族：能力在库里，入口摸不到）**：`SearchFilter` 支持 9 种算子（`eq/ne/contains/gt/lt/gte/lte/in/not_in`，`_evaluate_filter`），`EnhancedSearcher.search()` 也接 `filters`，但**共享入口 `search_dataset()` 不收**，CLI 的 `search` 子命令与 `/api/dataset/search` 更没有任何过滤参数（`grep -n filter augmentor/cli/parser.py api/routes/dataset_tools.py` 零命中）。于是「先按字段检索、再按元数据收窄」这条路径只有直接 import 库类才用得到。真实 6902 条实测它确实会改变答案：contains「租房」单字段 `instruction` **419** 条，叠加 `instruction contains 申请` 过滤 → **10** 条（收窄 42 倍）。**做之前要先定的两件事**：①过滤与分页的先后次序**已经定了**——`search()` 里是「打分汇总 →
逐条过滤器收窄候选集 → 排序 → 切页」（`:268-273`），所以接 CLI 时不许顺手改成「先切页再过滤」，
那会让 `--offset` 的语义随过滤变化；②`input` 字段在真实语料里**整档为空**，所以 `input ne ""` 这类示范过滤会得到 **0 条**（实测），文档与用例不要拿它当正例。另需注意 `filters` 传进来的是对象而不是字典，CLI 只能接 JSON 字符串再构造 → 报错口径要走同一处咽喉（A27/A28④ 形状）。**已做（L28）**：两条约束都照办（顺序未动；示范过滤用 `instruction`，`input ne ""` 只在对照里出现并写明会得到 0 条）；额外发现并修掉「字典形态在 HEAD 里直接 `AttributeError`」，并把 CLI 帮助与咽喉算子表的漂移用 `TestSearchFilterSurface` 钉住（`choices=` 那条路实测走不通：argparse 逐槽校验，FIELD 也被当算子比） | M |
| A31 | `augmentor/search_enhanced.py`（`SearchResult`）+ CLI 摘要 + `SearchResponse` | **L28 新记（A29 同族：回显不全）**：L28 之后 `filters` 能收窄结果，但 `SearchResult` 只回显两个松紧旋钮，**不回显本次真正应用了哪些过滤器**。于是「找到 0 条」新增第三种成因（检索没命中 / 旋钮太紧 / **过滤器把候选全筛掉了**）里最隐蔽的那种仍然读不出来——尤其 `not_in` 配错值形状时旧行为是「全留」，用户看到 419 条会以为过滤器没起作用，而新行为虽然改成报错，**成功路径上仍然没有「过滤器生效了、收窄到 10 条」的正向证据**。修法：`SearchResult.filters`（或 `applied_filters`）回显**规范化后**的过滤器列表（`{field, operator, value}` 字典，不回显原始对象，避免把调用方的可变对象泄进结果），CLI 摘要第二行跟着印 `过滤 N 条`，`SearchResponse` 必须**同步声明**否则 FastAPI 静默裁剪（L27 已实测过一次）。要先决定：空过滤器回显 `[]` 还是 `None`（L27 的口径是「方法没消费就不印」，这里对应「没过滤就是 `None`」）。**已做（L29）**：口径取「没过滤 = `None`」，CLI 印在第二行末尾「`(N 个过滤器: 检索 X → 保留 Y)`」，`SearchResponse` 与契约用例同步改到 10 键；额外踩到一条本行没预见的坑——成员档的 `set` 值进不了 JSON，所以在 `normalize_filters` 里归一成 `list`（其副作用另立 **A34**），逐条过滤器是谁筛光的仍没有（另立 **A33**） | ~~S~~ |
| A33 | `augmentor/search_enhanced.py:385-391`（`search()` 的过滤分支）+ CLI 第二行 | **L29 新记（本轮产物自己的残留）**：多条 `--filter` 现在只回显**合计数**（「3 个过滤器: 检索 419 → 保留 10」），说不出**是哪一条**把候选清零的——而这正是三条以上时唯一真正想知道的事。修法是在逐条应用处记 `len(filtered_indices)` 快照并回显成 `filters_funnel: [419, 12, 10, 0]` 这类并列清单。要先定：①它进不进 `to_dict()`（进则 `SearchResponse` 必须同步声明，见 L27 那条静默裁剪）；②CLI 第二行已经很长，印不印逐条数还是只印「最后为 0 的那条」；③与 A31 同口径：没过滤时是 `None` 而不是 `[]` | S |
| A34 | `augmentor/search_enhanced.py:160-166`（`normalize_filters` 的 set→list） | **L29 新记（实测，不是猜想）**：成员档收到 `set` 时归一成 `list`，而 `list({...})` 的顺序由字符串哈希决定 → 同一份代码三个进程里量到 `['alpha','beta','gamma']` / `['gamma','beta','alpha']` / `['alpha','beta','gamma']`（`PYTHONHASHSEED` 随机化）。于是 SDK 调用方传集合时 `to_dict()` 的回显**跨进程不是字节稳定**的（CLI 与 API 走 JSON 数组，不受影响）。修法两个方向：同质（全 str 或全数字）集合排序、或在 `SearchResult` 文档里明写「回显不保证顺序」并停止归一。选前先量排序对混合类型的 `TypeError` 面 | S |
| A35 | `augmentor/dedup.py`（`find_similar_pairs`） | **L30 新记（L30 的边界之外，如实记下）**：A17 的倒排化只覆盖了「归组」这条主线（`_find_duplicate_groups_chunked` → `_similarity_blocks`），**`find_similar_pairs` 仍然走 `_batch_encode` 物化整份 `n×vocab` 稠密矩阵**——它是公开 API，真实 6902 条上被调用就是 606 MB 峰值 + 那 5486 亿次乘加，L30 的 10.8× 一分钱都拿不到。修法直接：让它也过 `_similarity_blocks`（块形状契约已经和稠密一致，逐块取上三角即可），并补一条「两条公开入口共用同一个生产者」的用例。**没在本轮做**是因为 L30 的边界是「换掉归组的生产者 + 判据」，一处口径一轮改。先实测 `find_similar_pairs` 在真实数据上的端到端耗时，否则改了也不知道值不值 | M |
| A36 | `augmentor/dedup.py`（阈值判定）+ `test_dedup.py`（`test_an_exact_tie_is_not_a_producer_bug`） | **L30 新记（浮点意义上的诚实残留）**：两条生产者在**恰好等于阈值**的配对上会给不同答案——真实小语料上 0.5 这一档有 9 对精确 0.5，稠密路径（`_normalize` 除两次、结果恰好落在 0.5）判进，倒排路径（`float32` 散射累加一次归一）量到 `0.4999999701976776` 判出，**18 个单元翻转**。这不是 bug 而是两种算法各自的舍入方向，L30 已把它钉成用例（而不是把阈值挪开糊过去），并要求「任意翻转单元离阈值的距离 ≤ 2e-6」。残留风险：调用方若**恰好**用「相似度等于某个二进制可精确表示的值」当阈值（0.5、0.25、0.75…）做去重，换生产者会改变分组结果。**该做的是**在 `Deduplicator` 文档里写明「阈值比较是 `>=`，落在浮点边界上的配折不保证跨实现一致」，或干脆内部留 1e-7 容差——两条路都要用户拍口径，故未当轮顺手选 | S |
| A37 | `augmentor/dedup.py`（`_iter_inverted_similarity_row_blocks` 的峰值） | **L30 新记（L30 产物自己的成本残留）**：倒排路径确实不再物化 `n×vocab`，但**峰值仍随 `pair_adds × block_rows ÷ n` 增长**，真实 6902 条端到端峰值 **149.1 MiB**，离「一个块的上界 16 MiB」还差一个量级——因为 postings 视图（`terms/docs/values` + 排序后的副本）是**整份常驻**的，块内又各自有 `flat/picked/weights` 三份临时。随机 60 字符语料逐档实测更说明问题：n=1200…7200 时倒排峰值**一直是稠密的 1.4～2.4 倍**（54.2/22.2、104.0/44.4、153.9/66.6、171.3/85.7、173.9/103.3、175.9/120.9 MiB），**直到 n≥6000 才在时间上反超稠密**。内存账（`pair_adds × block_rows × 96 B` 拟合 + 125% 裕度）就是为此才存在，它现在能挡住这些档位，但**拟合是在 n≤7200 上做的**，更大语料未外推实测。修法候选：postings 按词项分段流式消费（不整体 `argsort`）、或把 `_INVERTED_PAIR_BYTES` 在更大 n 上重拟合 | M |
| A32 | `augmentor/__init__.py:60,220` × `api/routes/dataset_tools.py`（`SearchRequest.filters`） | **L28 新记（两处「同源但没接上」，都不算缺陷）**：①包级 `__init__` 导出了 `SearchFilter` / `SearchResult` / `search_dataset`，却没有导出 `normalize_filters` 与 `FILTER_OPERATORS`——想在自己代码里复用「算子白名单」或提前校验配置的调用方只能 `from augmentor.search_enhanced import ...`，绕过门面；要不要进来需要一并决定 `__all__` 的口径（`tests/unit/test_package_exports.py` 显示这份清单是**刻意策展**的，不是越全越好）。②API 的 `SearchRequest.filters` 是 `Optional[List[Dict[str, Any]]]`，OpenAPI schema 里**算子域与值形状完全不表达**，唯一的契约是咽喉那句 400 文案。修法是把 `filters` 声明成 pydantic 子模型（`Literal` 算子 + 自定义校验），但**代价要先算清**：pydantic 一旦判下来就是 **422**，会把本轮刻意做成 400 的那批语义（与 SDK/CLI 同一口径的领域错误）换掉，而且 `normalize_filters` 仍是 SDK 侧唯一的判据——两层校验谁是第一道要写明，不能靠「反正都会拒」 | S |
| ~~A12~~ | ~~`augmentor/validation.py:217-222`~~、`sampler.py:296-299` | **L13 实测拆成两半**：①`_validate_item` 里 `import re` + 每条每模式一次 `re.search`；②`sampler.generate_report()` 的 `items.index(seed)` **实测不是缺陷**（真实数据只推荐 4 个种子、反查 0.0 ms，单趟 id 映射要 1.1 ms，改了反而更慢；且它还会改变「值相等但不同对象」时的下标语义，真实数据里正好有 367 条重复 dict）—— 这一半作废。**①已修（L17）**：真实 6902 条 strict **28.99 → 19.49 ms（1.49×）**，禁止模式段占整档 53–55%、该段自身 **1.69–1.88×**（L13 预估的「~5 ms / 1.2×」偏保守）；`re.search` 逐条调用 **27608 → 0**、`re.compile` 与条数无关恒为 2 | S |
| ~~A13~~ | ~~`api/routes/dataset_tools.py:311,360,386,387,416,436,566,598` + `system_ops.py:320,348,514`~~ | ~~**A4 的同构族**：`read_items()`（同步版）在 11 个 `async def` 路由体里直接调用，同样占着事件循环；`dataset_tools.py:566` 还是「多个文件在循环里串行读」；`/api/dataset/stats` 连分析都留在循环上（43.4 ms）~~ 已修（L7） | M |
| ~~A14~~ | ~~`augmentor/impact.py:66`~~ | ~~`duplicate_rate` 里 `texts.count(t)` 写在推导式中 → O(n²)~~ 已修（L6） | S |
| ~~A15~~ | ~~`augmentor/statistics.py` `calculate_statistics`、`A3`/`A5` 那类纯 Python 分析~~ | ~~**线程池对 CPU 型分析不产生并行**（GIL）：3 并发 stats 实测离线后请求方 173.7 → 192.7 ms（+11%），换来的只是循环停顿 170.1 → 60.0 ms。要么上 `ProcessPoolExecutor`，要么回到算法侧把 43.4 ms 这个数本身降下来~~ **算法侧已修（L12）**：真实 6902 条 `calculate()` 同进程交替中位 **50.94 → 43.87 ms（1.16×）**、词汇统计峰值 **8.15 MB → 2.22 MB**。**GIL 那半仍然成立**——进程池本轮不做（跨进程要序列化整份数据集，代价未实测），所以「3 并发总耗时」这个数不会因为 L12 变成并行 | M |
| ~~A16~~ | ~~`api/routes/system_ops.py:514` `dependency_register`~~ | ~~为了拿「条数」这一个整数把整个数据集解析一遍（3.6 MB / 6902 条），可流式计数或延后到首次访问再回填~~ **L17 实测排除，未改**：同一份真实数据 back-to-back 量得 `await read_json_file` **14.93 ms**、同步 `json.load` **12.89 ms**、「读字节 + `loads` 只取长度」**9.89 ms**，而手写纯 Python 顶层元素扫描器 **77.84 ms（慢 8 倍）且把 6902 条数成了 13804**（内层数组的花括号它分不清）。结论：C 级解析就是拿这个整数最便宜的路子，「流式计数」在此不但更慢还会放松掉「登记时就拒绝坏 JSON」的语义。 | S |
| ~~A17~~ | ~~`augmentor/dedup.py:111`、`:241-242`~~ | ~~A6 剩下的一半：fallback 编码 `np.zeros((n, vocab))` 是**稠密 float64**（真实 6902 条 × 23033 词表 = 1.27 GB），调用方再 `np.asarray(..., dtype=np.float32)` 整份复制、再 `_normalize` 另起一份~~ **已修（L10）**：float32 + `_row_norms`/`np.divide(out=)` 全程原地，真实数据峰值 **2429 MB → 622 MB**。剩下的只有「稀疏 CSR 表示」（nnz 194293、稀疏度 0.9988 → 理论 2 MB），但本环境**没装 scipy 与 faiss**，手写稀疏结构体属于另起一套索引子系统，不在性能轮范围内 → 本轮不做，装依赖后再议。**「装依赖后再议」那一半已由 L30 做完，而且没装任何依赖**：用 numpy 原语（单趟收集 + `np.unique` 压缩 + `np.bincount` 散射累加）手写倒排 postings，把「先物化 `n×vocab` 稠密矩阵、再让 BLAS 去乘 99.88% 的零」换成「只对真正共现的词项做累加」，产出**与原稠密路径同形状**的相似度行块 → 真实 6902 条端到端 **969 ms vs 10484 ms（10.8×）**、峰值 **149.1 MiB vs 621.8 MiB（4.17×）**，`duplicate_groups` 与 HEAD **逐组完全相同**。换不换路由由**两条独立实测的成本账**决定（时间账 + 内存账，见 L30 日志与 A35/A37），退化语料（全重复、零词表、小矩阵）仍走稠密 | ~~L~~ |
| ~~A18~~ | ~~`augmentor/dedup.py:132-147,172-174`~~ | ~~块行数按剩余列数自适应放大，把 L9 归因微基准里的 68.9 → 91.5 GFLOP/s 捡回来~~ **L10 实测证伪，未采纳**：合成基准（dim=8192）预测 1.28×，真实数据（dim=vocab=23033）同一进程内 back-to-back 实测 **0.95×（更慢）**。归因假设（窄块让 BLAS 变笨）**不随 K 维迁移**，代码已回退 | S |
| A19 | `augmentor/dedup.py:104-116`（HEAD）→ `:155-172`（L30 后工作行号） | 词表构建与 TF 填充是**两遍** Python 双循环（`for text: for i:` 再 `for i, text: for j:`），每条文本的每个字符都进解释器一次。实测真实 6902 条走完 0.36 s（L10 后），**这还称不上瓶颈**；只有在 n 到 10⁵ 量级时才可能翻盘——该外推**未实测**，先记着别当依据。**L30 两处更新**：①**「什么时候翻盘」有了实测答案，而且方向和外推相反**——真实 6902 条稠密路径 10.5 s 里 99% 是 BLAS 在乘零（86 G MAC/s × 5486 亿次乘加），那两遍循环的 0.36 s 连 4% 都不到；换句话说在**本量级**优化双循环是找错了地方，L30 换掉的是乘零那一半。②**倒排路径已经顺手做成了单趟**：`_ngram_entries` 一次扫完收集 + `np.unique` 压缩 + `np.bincount` 计数，同一份真实 6902 条 **73 ms vs `_fallback_encode` 的 330 ms（4.5×）**——这是「单趟确实便宜」的直接证据，但**稠密那条路没改**（`_fallback_encode` 仍是两遍，它的既有用例与语义独立于 L30）。顺带坐实一条 coverage 里长期读出来的 partial 分支：`171->169` 是**永不到达的死分支**——第二遍里的 `if ngram in vocab` 恒为真，因为 `vocab` 就是同一个 `texts` 在 `:155-159` 建出来的，任何出现的 n-gram 必在其中。所以那遍循环里每次字符都白付一次哈希查表 + 分支判定。剩余修法（合并成单趟 + 删死分支）见 **A35** 相邻，未在本轮做 | S |
| ~~A20~~ | ~~`api/deps.py` `config_file_path()`~~ | ~~A2 的同构缺陷第三处：L4 把白名单里的 YAML 重解析缓存掉了，但「服务自身的配置文件路径」这一步仍**每次调用**做一次 `Path.resolve()`（Windows 上要问长路径句柄并归一大小写）。任何带路径参数的请求都至少过一次白名单，等于每个请求白加一份系统调用。安静态同进程交替实测：200 次 **29.9 ms（缺陷）→ 0.15 ms（缓存）**；同一台机器带负载时同一份工作量到 187 ms~~ **已修（L12）**：缓存按 `(环境变量原值, os.getcwd())` 建键、单条目、只在未命中时 resolve；`allowed_data_roots()` 200 次 **218.0 ms → 1.87 ms** | S |
| A21 | `augmentor/search_enhanced.py:158-360` | L13 剩下的那一半：~~contains / ngram / fuzzy / regex~~ **ngram 已修（L24）**，contains / fuzzy / regex 仍逐条扫全表（真实 6902 条、L24 后同进程 5 次中位：三字段端到端 ngram **13.34 ms**、contains 8.20、regex 4.98、fuzzy 48.45（**命中 0**，见 A27）；单字段 ngram instruction 3.87 / output 6.66 ms）。倒排索引里其实已经存了这些词项，但按「子串」「Jaccard 阈值」查需要**不同的索引结构**（n-gram 倒排 / token 集合按文档存），不是把现有索引接上去就行——属于「另起一套」，与 A17 的稀疏化同一档。**注意 L24 走的是另一条路**：没建索引，只把「为文档建集合」换成「用子串判等」，所以 ngram 仍是全表扫，只是每行便宜了 6–7 倍。**L25 更新**：fuzzy 那一格的旧数字（48.45 ms / 命中 0）**作废**——它是「A27 死方法」的副作用而不是成本参照；fuzzy 现在真的返回命中，其成本单独立在 **A28**，本行只剩 contains / regex 两格待办 | M |
| ~~A22~~ | ~~`augmentor/vector/faiss.py:100,106`~~ | ~~**L15 新发现**：`add_vectors` 每次都 `np.vstack` 整份矩阵 + 每次都 `set(self._ids)` 重建镜像 → 「一条一批」的写入是 O(n²)。真实维度 384 逐条写入实测 750/1500/3000 条 **58.2 / 386.1 / 1671.3 ms**（输入翻倍时间 ×6.6、×4.3）~~ **已修（L15）**：写缓冲几何扩容 + `_vectors` 改前缀视图 + `_id_set` 增量维护（带长度自愈判据），同一实验 **6.5 / 13.1 / 31.7 ms（×2.0、×2.4 即线性，8.94×/29.55×/52.72×）**；稳态驻留多 ≤1× 数据量的空槽，峰值不变 | M |

## Backlog B — 功能增强（价值 ÷ 工作量）

| # | 内容 | 证据 | 量级 |
|---|------|------|------|
| B1 | 把 `/api/system/*`(13) 与 `/api/dataset/*`(12) 接入 UI | `web/src/services/api.ts` 对二者零引用；`System.tsx:3` 只用了 status | M |
| B2 | Excel/CSV 摄取端到端（上传 + `convert --input-format xlsx`） | SDK `csv_excel_import.py:61,152` 可达性为零；`data.py:236-243` 只认 JSON | M |
| B3① | ~~反向转换边 `alpaca/sharegpt/chatml/vicuna/belle → json`~~ | ~~`converter.py:60-66` 只读 json/jsonl/csv，导出侧不可回环~~ 已做（L16，6 条反向边 + CLI/API 接线；原记「导出侧 13 种」不准，实测 `json →` 只有 8 条边）。**剩余另立 B3②** | M |
| ~~B3②~~ | ~~`tsv ↔ json` 两条边都缺；`convert_file(source_format="alpaca")` 读 `.jsonl` 容器文件会走 `json.load` 而失败；`_json_to_csv` 里 `all_keys` 算了不用~~ | ~~转换图实测：`json →` 8 条、`→ json` 8 条，`tsv` 两侧皆无~~ **已做（L20）**：三条一起，且查明它们是同一个根因的三个症状——`DataFormat` 十个成员里只有 `json/jsonl/csv/tsv` 是**文件容器**，其余六个是**行内 schema**，读写两侧都曾把两者混为一谈。现在图 `json →` 9 条、`→ json` 9 条，`CONTAINER_FORMATS` 写明边界，读侧嗅探、写侧看扩展名；`all_keys` 死代码已删 | S |
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
- **L20** `feat(converter)` B3② —— 把 L16 刻意留下的「剩余」清掉，顺手挖出这一族
  真正的根因：**`DataFormat` 十个成员里有四个是「文件容器」，六个只是「行内 schema」，
  而读写两侧都把它们当同一件事**（`converter.py:35-45` 现在写明这一点，并导出
  `CONTAINER_FORMATS`）。三处症状：
  ①**tsv 两侧皆无边**：`get_supported_formats()` 由 `_converters` 反推所以不虚报，
  但 CLI 的 `--format`/`--input-format` 清单里也就没有 tsv，而 `_infer_format` 认
  `.tsv` 扩展名、`export_enhanced._export_tsv` 早就在用制表符写盘 —— 同一件事三个模块
  三种说法。补 `json ↔ tsv` 两条边后清单自动长到 10 个，与枚举同集合。
  ②**读侧把 schema 当容器**：`_read_file` 只特判 jsonl/csv，其余一律 `json.load`
  整份文件，于是同一批 alpaca 记录写成 `.jsonl` 就抛 `JSONDecodeError: Extra data:
  line 2 column 1`（真实 6902 条实测复现）——调用方已经**正确声明了源格式**，却绊在
  从没声明过的容器上。改成按内容嗅探：整份 JSON 数组 → 单条对象 → 逐行 → 全空白=零条，
  两条路都不通才抛带**文件名与行号**的 `DataFormatError`（不再是解析器黑话）。
  ③**写侧不对称**：`target="alpaca"` 配 `.jsonl` 输出一律 `json.dump` 成一整个数组，
  产物连自己那一侧都读不回来。改成容器由**输出扩展名**决定，schema 只管行内形状。
  **真实语料（6902 条）实测**：`json → alpaca .jsonl` 49.7 ms / 读回 59.9 ms，
  往返 6902 行对 6902 条；`json → tsv` 34.5 ms 落 3.04 MB、`tsv → json` 55.5 ms
  全量回来。顺带删掉 `_json_to_csv` 里那份算了又用的 `all_keys` 死循环。
  **用例**：新增 **20 例**（`TestSchemaIsNotContainer` 15 + `TestConverterExtended2`
  的 tsv 镜像 2 + CLI 清单防漂移 2 + CLI tsv 双向 1）、删除 2 例（`test_unsupported_tsv_is_not_claimed_by_either_side`
  与 `test_tsv_is_not_accepted_as_input_format` 钉的正是被修掉的行为）、改写 1 例
  （API 的「未知源格式」例子从 `tsv` 换成 `parquet`，否则下一轮它就不再是未知格式）；
  `test_every_supported_target_really_converts` 是参数化用例，清单长到 10 个后自动多一条。
  **一条值得记的教训**：`test_alpaca_jsonl_round_trips` 第一次写完在缺陷态是**绿的**——
  老实现把读写两侧都错成同一个形状（`.jsonl` 里落一整个数组，再整份 load 回来），
  往返自然对得上。补了一句「中间产物必须真是 6902 行」才红。往返型用例单独存在时
  测不出对称的错误。
  **红→绿（逐条）**：`converter.py` + `parser.py` 整份退回 HEAD → 新用例 **18 红**、
  白名单 2 绿（`test_schema_target_to_json_keeps_writing_an_array` 钉住保留下来的老
  行为、`test_format_outside_the_graph_is_rejected_by_cli` 是防漂移，两侧本就该同绿），
  对照 325 例（test_converter 全量 + CLI 数据集工具 + API 数据集/系统工具）全程绿，
  恢复后全量全绿。`converter.py` 语句覆盖 100%（唯一残留分支 `317->324` 是 L20 之前就有
  的 `_json_to_chatml` 空 system 分支）。
  **L21 待办另立 A25**：源格式**未声明**时的静默坏数据（`.json` 里的 sharegpt 记录按
  json 读，`json → chatml` 只认 `instruction`/`output`，产物是空问答且退出码 0）。
  —— A25 已于 **L21 修掉**，见下一条。
- **L21** `fix(converter)` A25 —— L20 把「声明对了却绊在容器上」修了，这一轮修另一半：
  「压根没声明」。读侧按扩展名兜底是**必要**的（合法 `.json` 数组是最常见形态），但
  兜底的结果没人复核：`json → chatml/sharegpt/alpaca/…` 六条写边只认
  `instruction`/`output`/`history` 三个键，一份实际是 sharegpt 的 `.json` 于是被逐条读成
  「没有问答」，产出**空问答的合法文件**、退出码 0、HTTP 200 —— 用户拿到一份训练不出东西
  的数据集，且没有任何信号。L20 当时嗅探帮不上（文件确实是一份合法 JSON 数组）。
  **判据为什么取这么窄**：只有「**取不到 `instruction`/`output`（任一非空）**」**且**
  「**又带着 `conversations` 或 `messages` 列表**」两条同时成立才报。这样：
  ①csv 里恰好有一列字符串叫 `messages` 不误伤（要求 `isinstance(list)`）；
  ②`{instruction: q, output: a, conversations: [...]}` 这种「问答与数组并存」不误伤
  （既然取得到问答，写边就不是空的）；③`{"text": ...}` 这类「既无问答也无数组」的
  **不收**——护栏给不出「该声明什么」的建议，只能泛泛报「不像问答数据集」，误报面立刻
  放大，另立 A26 单独实测。
  **为什么挂在一处而不是六处**：`convert()` 是六条 schema 写边的**共同咽喉**
  （`converter.py:211-213`，L22 后行号；`source_format == "json"` 且目标不是容器时才查），
  一处覆盖六条边、参数化用例逐个验证边界，且不用在每条边里重复一遍。
  **真实语料（6902 条）实测**：`.json` 里的 sharegpt 记录按 json 读→转 chatml，
  以前 **96.9 ms 产出 6902 条全空问答、退出码 0**，现在 **16.7 ms 抛 `DataFormatError`**、
  目标文件**根本没被创建**（`test_no_partial_output_is_written` 钉这一点）；
  正确声明 `--input-format sharegpt` 的端到端路径 121.3 ms 不受影响；
  **干净数据（真有 `instruction`/`output`）转换耗时 3.8 → 3.8 ms，增量 0.0 ms**
  ——护栏是每条一次 `dict.get`，与写边同趟。
  **三端用例**：SDK `TestUndeclaredConversationGuard` **17 例**（六条边参数化、
  报错文案的下标与可执行建议、四类「不误伤」反例、容器目标不拦、声明后放行、
  零半截产物、非记录形状）+ CLI **2 例**（省略 `--input-format` 时退出码 1 且 stderr
  带 `` `conversations` `` 与 `--input-format` 建议；纯 json 仍按扩展名兜底）+
  API **1 例**（同一份数据是 **400** 不是 200，且落盘文件不存在）。**改写 2 例**：
  `test_sharegpt_file_without_declaration_keeps_the_old_behaviour` →
  `..._fails_loud`、CLI 的 `test_omitting_the_flag_keeps_extension_inference` →
  `test_omitting_the_flag_fails_loud_instead_of_emptying_the_dataset`——这两例原本钉的
  **正是本次修掉的静默行为**，删掉等于丢掉护栏，改写成钉住新的响亮失败。
  **红→绿（逐条）**：`converter.py` 整份退回 L20 提交态（`08b9c9537`，26336 B）→
  新/改写用例 **12 红**（guard 类 9：6 个参数化边 + 文案建议 + 下标 + 零半截产物，
  加 reverse-edge 改写 1、CLI 1、API 1），**白名单 9 绿**且逐条有理由：
  `test_a_container_key_next_to_real_qa_is_not_flagged`、
  `test_a_string_column_named_messages_is_not_a_conversation`、
  `test_container_targets_are_not_guarded[json/csv/tsv]`、
  `test_jsonl_target_keeps_the_conversation_array_intact`、
  `test_declaring_the_source_format_clears_the_guard`、
  `test_the_guard_stays_out_of_non_record_shapes`、CLI
  `test_plain_json_still_infers_from_the_extension`——这 9 例钉的是「护栏**不该**管的地方」，
  缺陷态本来就没有护栏、自然绿，它们防的是下一轮把判据放宽。对照 351 例
  （test_converter 全量 + CLI 数据集工具 + API 数据集/系统工具）全程绿，恢复后全量全绿。
  `converter.py` 语句覆盖 **100%**，分支 0.9915（唯一残留 `355->362`，就是 L20 记的
  `317->324` 插入护栏后行号平移，仍是 L20 之前就存在的 `_json_to_chatml` 空 system 分支）。
  **A26 新记**：护栏刻意留的两个口子（非对象条目 → `AttributeError` → API **500**；
  无问答无数组 → 仍静默空产物），本轮不收、另轮实测。
- **L22** `fix(converter)` A26① —— 收 L21 刻意留下的第一个口子。症状不是「坏数据被
  静默接受」而是**坏数据被当成服务器故障**：`json → 八类要取字段的目标` 里躺一条
  `null`，六条 schema 写边直接 `item.get(...)` → `AttributeError: 'NoneType' object
  has no attribute 'get'`；`to_http_error()` 只把 `ValueError` 映射成 400，于是
  实测 **API 返回 500**、CLI 退出码 1 且把这句解释器内部措辞原样打给用户。
  **csv/tsv 更坏**：写边本身是直通（`_json_to_csv` 原样返回），崩点在 `_write_file`
  的 `DictWriter` 里——**先把文件建出来再崩**，实测产物 `'j,u,s,t, ,a,r,i,n,g\n'`
  （字符串记录被按字符展开成表头）。调用方看到「文件存在 + 退出码 1」，很容易以为
  是自己哪里没配好。
  **判据只挂一处**（`converter.py:203-207`），与 L21 同一咽喉；豁免用新导出的
  `PASSTHROUGH_TARGETS = ("json", "jsonl")` 写明理由：这两类是把输入**原样序列化**，
  `null` 记录能落盘也能读回，拦它们没有意义。
  **一轮里踩到的弯路（值得记）**：判据最初写成「只看目标不看源」，想顺手把读边
  （`sharegpt → json` 之类）也接管，结果 `tests/unit/test_converter.py` 立刻
  **4 红**——六个 schema 读边**早就自带**带格式名的文案（「第 1 条 **alpaca** 记录
  必须是 JSON 对象，当前是str」，L16 立的），而 `jsonl` 在内存里本来就是**行字符串
  列表**（每行各自待 `loads`），预检会把合法输入拦下来。退回「只查源是通用 json 的
  写边」，并把读边那套更好的措辞用 `test_schema_sources_keep_their_own_better_message`
  钉住；新文案也改成与读边同形（「第 N 条记录必须是 JSON 对象，当前是X」），
  两端措辞不再各写一份。
  **真实语料（6902 条干净数据）代价**：同进程 back-to-back 三趟对照（护栏开→关→开），
  `alpaca` 1.27 → 1.50 ms（**+0.23 ms，×1.18**）、`chatml` 3.71 → 4.19 ms
  （**+0.49 ms，×1.13**）、`csv` 0.11 → 0.25 ms（**×2.34**，但绝对值 +0.14 ms——
  csv 写边直通、基数极小，倍率好看不代表要紧）；**豁免路径 `jsonl` ×1.00**
  （18.33 → 18.39 ms）当噪声对照，证明测量本身不偏。n/2n/3n 缩放仍线性
  （×1.88 / ×1.62），没有引入平方项。
  **用例**：新增 **27 例**（`TestRecordShapeGuard` 23：八类目标参数化、实际类型入文案、
  条目下标、四类「不是列表」入参、`json`/`jsonl` 豁免、读边措辞不被抢、jsonl 内存
  直通、csv/tsv 不留半截、干净数据照过、**计数护栏只跑一次**；CLI 2 例：非 0 退出 +
  文案不再泄漏解释器措辞、csv 不留半截；API 2 例：`chatml`/`csv` 目标都是 **400** 且
  无半截产物）+ 改写 1 例（L21 的 `test_the_guard_stays_out_of_non_record_shapes`
  原本钉的正是「新护栏不该管」，本轮那条豁免被修掉，改写为
  `test_the_two_guards_do_not_mix`：形状错的记录必须由形状护栏接管，且**不得**被误报成
  「疑似误标源格式」）。
  **红→绿（逐条）**：`converter.py` 整份退回 L21 提交态（`f7f428134`，29342 B）→
  新/改写用例 **24 红**（unit 20 + CLI 2 + API 2），**白名单 4 绿**且逐条有理由：
  `test_passthrough_targets_accept_any_json_value`、
  `test_schema_sources_keep_their_own_better_message`、
  `test_a_string_column_needing_jsonl_still_round_trips`、
  `test_clean_records_still_convert`——四条钉的全是「本轮**没**改的行为」，缺陷态
  本来就这样，它们防的是下一轮把判据放宽到 `jsonl` 源或读边。对照 366 例全程绿，
  恢复后全量 **3951 passed / 3 skipped**。`converter.py` 语句覆盖 **100%**
  （0 missed），分支残留仍是那条 L20 之前就有的 `_json_to_chatml` 空 system（`400->407`）。
  **A26 只剩 ②**（既无问答也无数组 → 静默空产物），继续另轮实测。
- **L23** `fix(converter)` A26② —— 收 A26 最后一个口子，症状回到 L21 那一族「静默
  坏数据」，但命中面更宽：字段名认不出（`question`/`answer`、`text`、`prompt`/
  `completion`……）的记录既没有问答也没有对话数组，L21 的窄判据抓不到，六条写边于是
  逐条读到 `""`，实测 **CLI 退出码 0、API 200**，产物是 N 条结构合法的空问答数据集。
  **不收的两个理由本轮逐一实测掉**：①「给不出该声明什么的建议」——改成给「认哪几个
  键 + 多少条」；②「万一有合法数据集恰好全空」——四份真实语料（6902 / 6902 / 1124 /
  5778，加 up.json 共 **20706 条**）扫下来缺可用问答的是 **0 条**，故整档判定不会
  误伤现有数据；`[]` 空数据集是既有合法行为（`test_an_empty_dataset_stays_legal`），
  不在拦的范围。
  **判据只挂一处**（`converter.py:214`），与 L21/L22 同一咽喉；顺序在 L21 之后 ——
  误标的对话数组那条给得出「声明 `--input-format`」，比「改字段名」更准，先报它
  （`test_a_mislabeled_conversation_still_gets_the_better_advice` 钉住两条护栏不打架）。
  **本轮真正的收获是问答键必须逐边算**：第一版把 `history` 当六个目标通用的素材键，
  写完用例才发现六条写边里**只有 sharegpt 与 chatml** 会把 `history` 展开进产物，
  `alpaca`/`llama_factory`/`vicuna`/`belle` 读都不读——「整档只有 history」转这四条边
  仍然是一份空问答，放过去等于 A26② 原地复活。于是提出 `_qa_keys()` +
  `_HISTORY_AWARE_TARGETS`，报错文案跟着目标变（alpaca 只列两键、chatml 列三键），
  两侧各有用例（`test_targets_that_ignore_history_do_not_count_it_as_qa` /
  `test_history_aware_targets_accept_history_only_rows`）。
  **代价**：护栏对干净语料命中第一条即返回，单次 **0.4~0.7 µs**；端到端 6902 条
  7 组配对交替实测 `alpaca` 差值中位数 **+0.021 ms**、`chatml` **+0.345 ms**
  （基数 4.5 ms，且隔离测得的护栏本身只有 0.0007 ms → 该差值是测量噪声）；
  真要说代价只有**报错路径**要扫完整档再抛，实测 **1.32 ms / 6902 条**，一次性。
  **用例**：新增 **31 例**（`TestAllEmptyQaRejected` 25：六目标参数化、文案带条数与
  所认的键、history 的两侧、system 不算问答、整档宽松只需一条可用、空格不 strip、
  `[]` 仍合法、容器目标与 jsonl 直通豁免、两条护栏不打架、写盘前失败、**分发口只跑一次**、
  **早退用中毒列表子类证明**；CLI 3 例：非 0 + 文案 + stdout 不报成功、csv 例外侧仍成功、
  单条可用即整档放行；API 3 例：`chatml`/`vicuna` 是 **400** 且不留产物、history-only
  仍 **200**）+ 改写 1 例（L21 的 `test_a_string_column_named_messages_is_not_a_conversation`
  原本钉「转出来仍是空问答」，即本轮要修的行为，改成钉「由 L23 接管且不得误报成声明源格式」）。
  **红→绿（逐条）**：`converter.py` 整份退回 L22 提交态（`38d4879af`，31597 B，
  CRLF 还原后 sha256 `4fe3fff419ec…` 与 L22 注入时一致）→ 新/改写用例 **19 红**
  （unit 16：`TestAllEmptyQaRejected` 15 + 改写那条 L21 豁免例；CLI 1；API 2），
  **白名单 13 绿**且逐条有理由：钉的全是本轮**没**改的
  豁免与宽松侧（容器/`[]`/空格/history-only/单条可用/csv 例外/CLI 成功路径），
  缺陷态本来就这样，它们防的是下一轮把判据放宽到容器目标或逐条报错。
  对照：缺陷态全量 **19 红 / 3963 绿 / 3 跳**，其中绿 = 既有 **3950** 例全过（= 没有
  任何既有用例依赖「静默空产物」）+ 上述 13 条白名单，恢复后
  全量 **3982 passed / 3 skipped**（56.6 s，总计 98.54%）。`converter.py` 语句覆盖
  **100%**（0 missed），分支残留仍是 L20 之前那条 `_json_to_chatml` 空 system
  （`454->461`，护栏新增的 `_qa_keys` 分支两侧都覆盖）。**A26 两条口子全部关闭。**
- **L24** `perf(search_enhanced)` A21(ngram) —— 换一个角度看 L19 留下的「一次性 ngram
  查询付整份索引的钱」（A24）：不改索引结构，先把**评分本身**的粒度对错。旧实现为
  每条文档 `set(_ngrams(文档))` 再与查询 gram 集合求交，而「覆盖率」的定义只是
  「查询的哪个 gram 是文档的子串」——子串关系不需要为文档建集合。改成 k 次
  `gram in 文档.lower()` 后，每文档代价从 O(文档长度) 次哈希插入变成 O(k) 次 C 级子串查找。
  - **等价性先于速度**：新写一份「逐文档建集合」的朴素 oracle，与库实现做 **12 例参数化
    逐条对照**（含空文本、长度不足 n、大小写、非字符串字段、重复 gram、纯查询 gram
    全落空的文档），命中集合与分数必须逐条相同；分数用 `hit/total`，重复 gram 在
    `set` 里只算一次，这一条被单独钉住（`test_a_repeated_gram_in_one_document_scores_once`）。
  - **同进程 back-to-back 实测**（真实 `train_data.json` 6902 条、5 次中位）：单字段
    `_search_ngram("instruction", "租房")` **22.2 → 3.2 ms（6.03×）**，`output` 侧 6.66 ms；
    三字段端到端 `search(method="ngram")` **104–113 → 12–16 ms（中位 7.57×）**，
    本轮收尾复测 13.34 ms（contains 8.20 / regex 4.98 / fuzzy 48.45）。
  - **缩放形态**（对机器负载不敏感，是本条结论的主证据）：n=2000/4000/6000 的新旧比
    **6.60× / 6.58× / 6.85×** —— 加速比不随规模漂移，说明省掉的是与文档长度成正比的那部分。
  - **确定性预言机**：`_ngrams` 被喂进去的文本数 —— 旧实现 **6902 次为文档建集合** +
    1 次查询；新实现**只剩查询那 1 次**（用例里 monkeypatch 成 `*args` 计数的转发
    探针，签名不引新符号，承 L14 教训）。这条断言不依赖墙钟，是本轮红→绿的凭据。
  - **注入对照**（最后一次改动测试**之后**重跑，承 L21 教训）：把 `search_enhanced.py`
    退回 HEAD 版本（LF blob → CRLF 还原，注入前后各核 sha256，恢复后
    `e5f5ed044a9f…`（按当时代码字节核，那一份是 LF 行尾）与修复态一致，最后一次注入发生在**所有测试改动之后**，含下面那条
    恒真断言的改写）→ 该文件 **1 红 / 71 绿**。新类 20 例 = **1 红 + 19 绿**：红的那条
    是计数预言机，报 `AssertionError: 文档也被建了 gram 集合：['如何申请租房？', …]`
    （**是断言红，不是崩溃红**，承 L10 教训）；19 条绿 = **12 条** oracle 逐条等价对照
    （6 个查询 × n=2/3）+ **7 条**大小写 / 重复 gram / 短查询 / 非字符串 / 排序 /
    公开入口累加口径的护栏，两侧本来就等价。同一文件既有的 **52** 例在缺陷态全过
    = 本轮没有改动任何既有语义。
  - **卫生门禁抓了我一次**：`test_the_public_entry_point_accumulates_fields_identically`
    原本写 `assert expected == {0: 2.0, 1: 1.0}`——`expected` 是测试内 oracle 算的，
    右边是字面量，属于 `scripts/triage_tests.py` 定义的**恒真断言**（被测库不参与判定），
    于是 `test_no_vacuous_asserts` 红。改成断言**库输出**（`result.total_matches == len(expected)`
    + 排序必须是 doc0 在前），手算写进 docstring；顺带发现旧语料**区分不了口径**——doc0 两字段
    各 1.0、doc1 一字段 1.0，「取最大」也会因为打平而走稳定排序白拿绿。于是把语料换成
    「doc0 三字段各 0.5 合计 1.5 / doc1 单字段满覆盖 1.0」（实测 `max` 口径为
    `{0:0.5, 1:1.0}`，排序反过来），排序断言才真的钉住「按字段求和」。
    **教训：oracle 型用例的断言必须落在库的返回值上，且语料要能区分被钉的那条口径**，
    否则要么钉的是自己的算术、要么钉的是一个判别不了的排序。
  - 全量：**4022 passed / 2 skipped**（带覆盖率 43.2 s，总计 **98.97%**）；
    `search_enhanced.py` **191 语句 0 missed**，残留分支 `101->exit`、`324->322`（均先于本轮）。
  - **计数漂移如实记**：L23 记录 3985 收集 + 本轮 +20 = 4005，实测收集 **4024**，
    多出 **19 例且 skip 少 1**；本分支 `git diff fa9733321..HEAD -- .`（augmentor 目录）**为空**，
    两次连续运行稳定在 4024 → 归因未坐实，判定为共享工作树上的外部/环境相关集合变化，
    **不计入本轮成果**；今后每轮改用**单文件级**用例差值（72 vs 既有 52）做归属证据。
  - **新记 A27**：fuzzy 在中文语料上恒 0 命中（分词把整段中文当一个 token），
    本轮只记不修 —— L24 的边界是性能口径。
  - **行尾插曲，顺手把 L20/L22 的幻影 ` M` 之谜往前推了一步**：本轮改 `_search_ngram`
    之后，`search_enhanced.py` 整份从工作树的 CRLF 变成 **0 条 CRLF / 502 条纯 LF**，
    而 `git diff --stat` 仍然只报 12+/2− 的**真实**改动（`text=auto` 在比对前归一化了行尾），
    只有 git 的警告泄露了痕迹：「in the working copy of …, LF will be replaced by CRLF the
    next time Git touches it」。收尾把工作树那份统一回 CRLF（最终 `ce69f40c16c3…`，与
    `e5f5ed044a9f…` 是同一内容的两种行尾），避免下一次 git 触碰整档时显示成全量改动。
    **推论：Edit 类工具写文件可能整份换成 LF**，L20/L22 那条「内容与 HEAD 相同却显示 ` M`」
    最可能就是这个机制——以后注入核对除了 sha256，还要看 `CRLF 计数`。
- **L25** `fix(search_enhanced)` A27 —— 把 L24 记下的「静默返回空的死方法」修成活的。旧
  `_search_fuzzy` 是 `_tokenize` 上的 Jaccard，而 `_tokenize` 用
  `re.findall(r'[一-鿿]+|[a-zA-Z]+|\d+')` 把**一整段连续中文算作一个 token**：查询 `{'租房'}`
  与文档 `{'如何申请租房'}` 交集恒空 → 分数恒 0，阈值从 0.6 降到 0.02 命中数仍是 **0**。
  新实现完全不碰 `_tokenize`（`exact` / `_build_indexes` 的中文整段键语义一行未动）：
  **等长滑窗 + Hamming 计数**，分数 = 各窗口 `1 - 错配数/len(查询)` 的最大值，过阈值才收；
  窗口只能表达「替换」，插一个字母会整体平移，所以它是 **Hamming 不是编辑距离**——这句话
  写进 docstring 和用例名，不借「fuzzy」这个词假装 Levenshtein。外面套一层**鸽笼预筛**：
  允许 `allowed` 个错配时把查询切成 `allowed+1` 片，命中分数 ≥ 阈值的文档至少要整含一片，
  一片都没有就跳过（`any(piece in lowered)`），所以预筛是**纯优化**、命中集合与无筛版逐档相同。
  - **不变式，两条都实测**：① contains 的命中必是 fuzzy 的**满分**命中（真实 6902 条「租房」
    两侧都是 **823 条且逐条相同**；数字串「2024」两侧都是 9）；② 阈值 ∉ (0,1]、空查询、
    查询比字段长 → 一律空（旧实现只在 `threshold=1.0` 时**碰巧**也空，因为它谁都到不了 1）。
  - **成本（同一新实现内对照，min-of-7，真实 6902 条）**：三字段端到端「租房」 fuzzy
    **21.23 ms** / 823 命中（contains 8.06、ngram 13.05）、「公租屋」 **174.41 ms** / 63 命中
    （contains 7.24 / **0 命中**）、「租房合同」 **167.02 ms** / 159 命中（contains 7.32 / 38，
    ngram 16.40 / 1271）。单字段拆开看钱花在哪：`output`（长字段）带筛 60.78 / 478.50 /
    430.37 ms，`instruction` 5.86 / 18.90 / 15.84 ms，`input`（整档为空）0.82 ms。
    **鸽笼预筛的净收益**同进程 back-to-back：`output` 侧 1365.64 → 60.78 ms（**22.47×**）、
    1407.14 → 478.50（**2.94×**）、1612.08 → 430.37（**3.75×**），`instruction` 侧
    18.78× / 6.94× / 8.64×，三档**命中集合全部 SAME**（695 / 32 / 84），空字段侧 0.98–1.00×
    作「预筛本身不额外收费」的对照。修好后 fuzzy 成为五种方法里最贵的一档 → **另立 A28**。
  - **等价性先于一切**：库实现 vs 「去掉预筛的朴素全扫」oracle，在真实语料上随机 **70 组**
    （3 字段 × 阈值 0.6/1.0/0.5/0.75/0.34 × 含错字与截断查询，种子固定 20260924）
    **命中集合与分数 0 不一致**；单测里常驻 **42 组**参数化逐条对照（7 查询 × 3 阈值 × 2 字段）。
  - **拉丁侧的行为改动是修好而不是改坏，实测过**：`"buy a cart today"` 上「cat」旧实现
    `{}`（按词 token，交集恒空）→ 新实现 **0.667**，「cars」 **0.75**；`AAAa` vs `aaaa`
    两侧都是 **1.0**（`lower()` 在前，大小写不计错配）。因此**没有**按脚本分流——分流会把
    「错字容忍」切成两种语义，而旧实现真正的毛病是「任何语言的错字都判不相似」。
  - **波及面：3 处既有断言钉的是缺陷本身**，本轮一起改（不改就会拿着 A27 的症状当规格）：
    API `test_each_method_has_its_own_semantics` 里 `("fuzzy", "公租屋", 0)  # 编辑距离不足`
    → 期望 **2**（旧 0 是恒零 Jaccard，注释里的归因也是错的）；CLI `test_fuzzy_method` 原本
    只 `assert "找到" in out`（任何命中数都过）→ 改成 `找到 2 条匹配结果`；并新增一条
    CLI 用例把「腿租押金」这种带错字查询钉成 **contains 0 条 / fuzzy 1 条**——这才是
    fuzzy 作为公开可选项的意义，之前三端没有任何一条用例证明它比 contains 多找回过东西。
    新增 51 例（unit 单文件 72 → 122，CLI +1），SDK / CLI / API 三端都有覆盖。
  - **注入对照（最后一次测试改动之后重跑，承 L21 教训）**：`search_enhanced.py` 退回 HEAD
    版本（LF blob → CRLF 还原，注入前后核 sha256）→ unit 文件 **24 红 / 98 绿**（新类 50 例
    = 24 红 + 26 条白名单绿，即阈值/空查询/长度/非字符串这些**两侧同构**的护栏；既有 **72**
    例全绿 = 本轮没动既有语义），integration **3 红 / 281 绿**（CLI 2 + API 1，正是上面那 3 处）。
    红因是**行为断言**（含计数预言机报「fuzzy 又去整档切 token 了」、空集 vs 命中集的差集），
    不是崩溃。恢复后实现字节一致（sha256 `8a034b445ab3…`，536 行全 CRLF）。
  - **记账口径升级（把 L24 那条没坐实的漂移往前推了一步）**：本机的解释器有**两个**——PATH 上的
    `python` 是 3.14.4（装了 chromadb / fastapi / pytest），项目 venv `…/envs/aug` 是 3.13.14
    （**没有** chromadb）。可选依赖的条件 skip 与收集条目随解释器不同，跨解释器的总数本来就不可比；
    L24 记的「+19 例 / −1 跳」外部漂移，最可能就是这个来源。**今后固定为：PATH `python` +
    分目录收集快照**，本轮 `--collect-only` = unit **3110** / integration **941** / e2e **24**
    = 4075，而 L24 的 4024 + 本轮 51 = **4075 精确对上**（本轮没有未解释漂移）。
  - 全量：**4073 passed / 2 skipped**（带覆盖率 152.46 s，总计 **98.98%**；两条 skip 是
    chromadb 缺失降级路径，属环境）。`search_enhanced.py` **206 语句 0 missed**，98 分支只余
    `101->exit`（先于本轮）。本轮墙钟含并行 agent 竞争（L24 同规模 43.2 s），只作快照不作跨轮比较。
- **L26** （哈希由下一轮提交补） `feat(search_enhanced)` A28④ —— 把 fuzzy 的阈值与
  ngram 的 gram 长度从**私有方法的默认参数**提到三端公开入口。
  - **症状**：L25 修好 `_search_fuzzy` 的打分后，它的阈值与 `_search_ngram` 的 `n` 仍然只有
    直接调私有方法才碰得到——`EnhancedSearcher.search()` 与 `search_dataset()` **一个字都不接**，
    CLI/API 于是也没有任何入口。用户读到的「fuzzy 更宽松/更严格」这句文档，实际上没法执行。
    顺带纠正 L25 日志里我记错的那句：`search_dataset()` 当时**根本没有** `threshold` 形参。
  - **一处校验、三端接**：判据只写在 `search()` 咽喉（`fuzzy_threshold ∈ (0, 1]`、
    `ngram_n` 是 `int` 且 `>= 1`、显式拒 `bool`），**不判方法**——越界值在任何方法下都给不出
    文档承诺的结果，静默忽略等于骗人（同 A27 的形状）。`search_dataset()`、CLI
    `--fuzzy-threshold` / `--ngram-n`、API `SearchRequest` 三端各自转发，不各自判。
  - **踩到自己家的结构护栏**：第一版写的是 `raise ValueError`，全量红在
    `tests/unit/test_exceptions.py::test_no_bare_builtin_raises_left_in_package`（本仓库禁止
    包内裸抛内置异常）。改成 `DataValidationError`（它同时是 `ValueError`，所以 `to_http_error`
    照旧映射 400、CLI 照旧「错误: …」+ exit 1），并把用例收紧成「领域类型 **且** isinstance
    ValueError」两条都钉。**这是一次有价值的红**：护栏替我把仓库约定教了一遍。
  - **pydantic 会静默吞掉未知字段**：给 API 加字段前我先试了「请求体里塞 `fuzzy_threshold`
    而模型不接」——返回 **HTTP 200 + 默认阈值的结果**，不是 422。所以「客户端传了参数却
    没生效」在这个栈上是**无声**的，API 侧的用例必须真的比对命中数变化，不能只看 200。
  - **旋钮真的动了结果（真实 6902 条，三字段）**：fuzzy「租房合同」阈值 0.5 → **1272** 命中 /
    0.6 → **159** / 0.8 → **38**（最松到最紧 **33×**），且 0.8 的那 38 条正好等于 contains 的
    命中数——与 L25 的「contains 命中必是 fuzzy 满分命中」不变式对上；「公租屋」0.6 → **63**、
    **0.7 → 0**。ngram「租房合同」n=1 → **4565** / n=2 → **1271** / n=3 → **42**。
    阈值 0.7 直接把结果清空这件事，是 A29（生效旋钮不回显）存在的理由：用户看不出
    「参数太紧」和「语料里没有」。
  - **默认口径不变，实测**：同进程 back-to-back、7 组交替取 min，HEAD 与当前在
    contains「租房」/ ngram「租房」/ exact「租房」/ contains「租房合同」四组上
    **7.09 / 12.11 / 0.00 / 6.57 ms，比值全部 1.000，命中集合逐条相同**（823 / 823 / 0 / 38）。
    咽喉校验的成本：越界被拒一次 **0.0005 ms**（比一次合法搜索 3.11 ms 便宜约 6000 倍，
    因为它在建索引之前就返回）——与「校验先于碰语料」那条用例同源。
  - 新增 **33 例**（unit `test_search_enhanced.py` 122 → **144**，CLI 73 → **79**，
    API 207 → **212**）：阈值/长度移动命中集合、边界闭开（0.75 含 / 0.8 不含）、默认值等于
    L25 语义（5 参）、旋钮不串台（4 参）、越界报错（阈值 4 参 + 长度 4 参含 `True`）、
    校验先于语料（`Counting` 子类覆写五种 `_search_*`，先断言**调用列表为空**再空跑一次证明探针响）、
    `search_dataset` 双旋钮转发。
  - **注入对照（最后一次测试改动之后重跑两次，结果一致）**：4 份实现（`search_enhanced.py` /
    `cli/parser.py` / `cli/commands/data_ops.py` / `api/routes/dataset_tools.py`）整体退回 HEAD
    → **33 红 / 406 绿**（unit 22 + CLI 6 + API 5 = 33，恰好是全部 33 条新用例；
    既有 406 例含卫生护栏 4 例**全绿**）。本轮**没有一处既有断言需要改写**——不像 L25
    动了 3 处钉缺陷的期望，这次是纯增量。恢复后实现字节一致（`af899524b644`，562 行全 CRLF）。
  - **另立 A29**：`SearchResult` 不回显**生效的**阈值/ gram 长度（`method` 有、旋钮没有），
    三端打印/返回的都是用户输入的那份，跨端默认值一旦漂移就查不出来。本轮不做（一轮只改一类口径）。
  - 全量：**4106 passed / 2 skipped**（最后一次改动后复跑 113.45 s，首跑 140.65 s 含并行竞争；
    总计 **98.98%**）。
    `search_enhanced.py` **211 语句 0 missed**、无 partial branch；`parser.py` 196 / `data_ops.py` 66
    语句 0 missed，`dataset_tools.py` 只余 4 行既有未覆盖（599/600/671/672，别的路由的
    `except HTTPException: raise`）。计数 **4075 + 33 = 4108** 与 L25 口径精确衔接
    （4108 − 2 skip = 4106 passed）。
- **L27** （哈希由下一轮提交补） `feat(search_enhanced)` A29 —— 让结果回显**本次真正生效**的松紧旋钮。
  - **症状**：L26 把两个旋钮接到三端之后，`SearchResult` 仍然只带 `query` / `method`，
    CLI 摘要只有 `搜索方法: fuzzy`。于是「找到 0 条」这两种成因读起来一模一样：
    语料里确实没有 / 旋钮拧得太紧。而 L26 已经量过这两者的差别有多大——真实 6902 条
    「公租屋」阈值 0.6 → **63** 条、0.7 → **0** 条，两条结果的 `method` 都是 `fuzzy`。
  - **口径（本轮唯一的设计决定）**：`fuzzy_threshold` / `ngram_n` 只在**本次方法真的消费它**时
    非空，否则 `None`。反面选择是「一律回显」，那会在 contains 旁边印一个 0.6，
    让用户以为阈值管得到 contains——正是 A27/A28④ 那一类「以为参数起了作用」的静默，
    所以宁可不印。同时**没传参也要回显**（回显 0.6 而不是 `None`）：回显的是生效值，
    不是「用户是否显式传过」，否则默认路径上仍然什么都看不出来。
  - **落点用例（不是「有字段」而是「能分辨」）**：`test_two_zero_hit_results_now_tell_apart_a_tight_knob`
    与 API 的 `test_a_starved_result_names_the_threshold_that_starved_it` 都拿**两条 0 命中**
    做对照（fuzzy 0.667 饿死 vs contains 语料没有），缺陷态两者除 `method` 外完全同形。
  - **CLI 的 stdout 契约先查再改**：既有下游按 `out[out.index("["):]` 取条目（本轮新增的
    `test_the_echo_stays_inside_the_two_summary_lines` 把「两行摘要 + JSON」钉死），
    所以生效值印在**第二行末尾**（`搜索方法: fuzzy (生效阈值 0.6)`）而不是新起一行——
    新起一行会把提示语混进 JSON 前面。这条用例在缺陷态**必然是绿的**（HEAD 本来就是两行），
    它钉的是「本轮不许撑坏既有形状」，进白名单而不是混在红里。
  - **FastAPI 静默裁剪被仓库自己的契约用例抓了一次**：给 `to_dict()` 加两键后，
    `test_api_openapi_contract.py::TestResponseModelDoesNotDropFields::test_response_matches_domain_contract[route_key40]`
    立刻红，红信息就是它自己写的「缺项：['fuzzy_threshold', 'ngram_n'] …
    response_model 漏声明字段…会被静默裁掉」。所以本轮的 API 侧证据链是两层的：
    ①`SearchResponse` 声明两键（否则响应里根本没有这两键，**且不报错**）；
    ②新增 `test_the_http_shape_matches_the_sdk_result_dict` 拿 **SDK 的 `to_dict()` 键集合**
    当预言机比对 HTTP 载荷——不能用 `SearchResponse.model_fields`，那与被测对象同源、恒真。
    未消费时的两键还得**存在且为 `null`**（`test_the_knob_keys_are_present_even_when_unused`
    断言 `"fuzzy_threshold" in payload`，因为 `.get(...) is None` 在键被裁掉时照样通过）。
  - **代价实测（同进程 back-to-back min-of-7，真实 6902 条）**：默认路径四组
    contains「租房」/ ngram「租房」/ contains「租房合同」/ exact「租房」比值
    **0.9834 / 0.9994 / 0.9873 / 1.0000**（7.31→7.19 / 12.31→12.30 / 6.53→6.45 / 0.00→0.00 ms，
    两个 <1.0 是噪声不是加速），命中集合逐条相同（823 / 823 / 38 / 0）；
    隔离出来看 `to_dict()` 单次 **0.00014 → 0.00017 ms（+30 ns）** 换两个键。
    结论：回显是**免费的**，不进 Backlog。
  - 新增 **26 例**（unit 144 → **155**、CLI 79 → **87**、API 212 → **219**）+ **更新 1 处既有契约用例**
    （`/api/dataset/search` 那格键集合从 6 加到 8，它在缺陷态驱动 **2** 条红：域契约比对 + OpenAPI 声明）。
  - **注入对照（最后一次测试改动之后跑）**：3 份实现（`search_enhanced.py` /
    `cli/commands/data_ops.py` / `api/routes/dataset_tools.py`）退回 HEAD → **24 红 / 590 绿**。
    红的构成：新用例里 22 条真行为红（unit 11 全红、CLI 4、API 7）+ 更新的契约用例 2 条；
    其余 **4 绿是白名单**（3 条 `不消费旋钮的方法不许印数字` + 1 条 stdout 形状，
    理由逐条写在注入脚本里）。红因全部是「字段不存在 / 键集合缺项」这类**行为断言**
    （`AttributeError: 'SearchResult' object has no attribute 'fuzzy_threshold'` × 8、
    `KeyError: 'fuzzy_threshold'` × 4、`缺项: ['fuzzy_threshold','ngram_n']` 等），
    没有崩溃式红。恢复后三份实现字节一致（`e7b41d52b3fe` / `e1cf45f7df50` / `7eec9cd84b21`，
    573 / 154 / 858 行全 CRLF、0 纯 LF）。
  - **另立 A30**：`SearchFilter` 九种算子只在 SDK 里摸得到——`search_dataset()` 不收 `filters`，
    CLI/API 零命中（`grep -n filter`）。真实 6902 条实测它会改变答案：contains「租房」
    `instruction` **419** 条，叠加 `instruction contains 申请` 过滤 → **10** 条（收窄 42 倍）。
    顺序约束（「打分汇总 → 过滤 → 排序 → 切页」，`:268-273`）与「`input` 整档为空所以
    `input ne ""` 会得到 0 条」两件事都写进行，避免下一轮拿它当正例示范。
  - 全量：**4132 passed / 2 skipped**（最后一次改动后复跑 113.31 s，首跑 126.00 s 含并行竞争；
    总计 **98.98%**）。计数 **4106 + 26 = 4132 精确对上**，无外部漂移。
    `search_enhanced.py` **213 语句 0 missed**、只余 `112->exit`（双重检查锁的 `with` 出口，
    先于本轮，即 L25 记的 `101->exit` 换了行号）；`data_ops.py` 71 语句 0 missed、
    只余 `61->68`（`run_validate` 的 `if args.output:` 假分支，本轮没碰那个命令）。
- **L28** （哈希由下一轮提交补） `feat(search_enhanced)` A30 —— 把九种过滤算子从「只有 import 库类才摸得到」接到三端，并给坏形状一个咽喉。
  - **症状**：`EnhancedSearcher.search(filters=)` 早就支持 9 种算子，但共享入口
    `search_dataset()` 不收 `filters`，CLI 与 `/api/dataset/search` 也没有任何过滤参数
    （L27 `grep -n filter` 零命中）。它不是理论需求：真实 6902 条 contains「租房」
    单字段 `instruction` **419** 条，叠加 `instruction contains 申请` → **10** 条（**42 倍**收窄），
    也就是说「检索 + 按元数据收窄」这条路径此前只有改代码才走得到。
  - **旧行为的五种坏法（这才是本轮的主体，接线只是让它们可见）**：逐条在 HEAD 上实测过——
    未知算子（`equals` / `EQ` / 空串 / 尾巴多一个空格）**0 命中**，与「语料里没有」完全同形；
    `in` 配标量 **0 命中**；`not_in` 配标量**全留**，即「什么都没筛掉」被读成「过滤后还剩这些」；
    值类型不符（`gte` 配字符串）**`TypeError`**，在 API 那头是 **500**；
    `filters` 传字典（最自然的一种写法）**`AttributeError: 'dict' object has no attribute 'field'`**，
    栈顶在扫描函数第 445 行，离用户的调用点隔了三层。
    本轮统一成 `DataValidationError`（领域异常，双继承 `ValueError` → API **400** / CLI **退出码 1**，
    承 L26 的口径），并且字典升格为与 `SearchFilter` 对象**等价**的合法输入。
  - **判据只认参数，不认数据**（本轮唯一的设计决定）：`normalize_filters()` 判的是
    **用户传进来的过滤器**（`filters` 不是列表、算子不在白名单、值形状与算子不配、
    元素缺键、字段名不是字符串）→ 抛；而**文档里**那个字段的值类型不对
    （`views = "几千"` 却按 `gte` 比）→ 是数据，判「不匹配」继续跑，不抛。
    这条界线是刻意的：`_evaluate_filter` 的 11 条私有方法既有用例（含「未知操作符应返回 `False`」
    「`in` 配字符串返回 `False`」「`not_in` 配字符串返回 `True`」）**一条都没改写**，
    因为校验只挂在 `search()` 入口，私有方法照旧是「不确定就不匹配」的宽容语义。
    代价是 `_evaluate_filter` 仍然能被直接调用而绕过校验——它不是公开入口，本轮不动（A31/A32 也不涉及）。
  - **CLI 的帮助文本是第二份算子清单，会漂**：`--filter` 用 `nargs=3 + action="append"`，
    三个槽位分别是字段/算子/值，所以**用不了 argparse 的 `choices=`**——实测它逐槽校验，
    把 FIELD 也当算子比，`--filter output contains 登录` 直接
    `invalid choice: 'output'` + 退出码 2。既然不能构造性同源，就照本仓库
    `EXPORT_FORMATS` 的先例（同样是手写清单 + 注释指明事实来源 + 用例钉住）办：
    `parser.py` 里显式列 `SEARCH_FILTER_OPERATORS`、帮助文本由它拼出来，
    新增 `TestSearchFilterSurface` 双向钉——**集合与顺序**都要等于 `FILTER_OPERATORS`，
    并且九个算子**逐个真跑通**一次 CLI（每个算子配一个它接受的值形状，
    数字档给 `0`、成员档给 JSON 列表），防止「照抄进帮助却后端不认」。
  - **两条 A30 事先约定的约束都守住了**：`search()` 内次序仍是「打分汇总 → 过滤 → 排序 → 切页」
    （接入口没有顺手改成「先切页再过滤」，那会让 `--offset` 语义随过滤漂移）；
    示范过滤一律用 `instruction`，真实语料里 `input` **整档为空**，`input ne ""` 会得到 **0 条**，
    只在「多次过滤取交集」的对照里出现并写明预期是 0。
  - **代价实测（同进程 back-to-back min-of-7，真实 6902 条）**：contains 无过滤
    **7.246 → 7.068 ms（0.976）**、1 条过滤 **0.971**、3 条过滤 **0.991**、ngram 无过滤
    **0.983**、fuzzy 无过滤 **95.70 → 95.96 ms（1.003）**；命中集合新旧**逐条相同**
    （823 / 10 / 0 / 2808 / 0），两个 <1.0 是噪声不是加速。结论：不传 `filters` 时
    `normalize_filters(None)` 立即返回，热路径零回归，不进 Backlog。
  - **顺带排掉一个假缺陷**：代价探针里 fuzzy「如何申请公租方」三字段 **0 命中**，
    看着像 L25 又坏了。手算全语料等长滑窗的最高相似度 = **0.5714**（4/7，最佳窗口出现在
    `instruction`）——0.6 门槛之下，**是数据不是 bug**（同一探针「租房合通」41 命中、
    「公租房申请」8 命中，fuzzy 本身是活的）。这正是 L27 回显存在的意义：现在这条
    0 命中会自己印出「生效阈值 0.6」。
  - 新增 **59 例**（unit 155 → **188**、CLI 87 → 94 → **104**、API 219 → **228**），
    **既有用例 0 改写**（承 L25/L26 两次「改写钉住旧缺陷的期望」之后，本轮第一次不用改任何老例）。
  - **注入对照（最后一次测试改动之后跑）**：4 份实现（`search_enhanced.py` /
    `cli/parser.py` / `cli/commands/data_ops.py` / `api/routes/dataset_tools.py`）退回 HEAD →
    **58 红 / 615 绿**（673 = 58 + 615，与恢复后的 673 全绿一一对上）。
    红的构成：unit 32 + CLI 7 + API 9 + 新增的 `TestSearchFilterSurface` 10；
    **1 绿是白名单**：`test_an_empty_filter_list_is_the_same_as_no_filter`——空 `filters`
    在修复前后都必须等价于「不过滤」，缺陷态必然绿，它守的是「新校验没有改变无过滤路径」，
    是 CONTROL 而不是漏网。原打算一并放进白名单的
    `test_a_document_field_of_the_wrong_type_is_data_not_an_error` 实测**是红的**
    （它用字典形态的过滤器，而旧实现连字典都不接：`AttributeError` 在读文档值之前就炸了），
    所以白名单最终只有 1 条。
    红因抽样：`AttributeError: 'dict' object has no attribute 'field'`、`TypeError` 比较串与整数、
    CLI `assert 2 is None`（argparse 根本不认 `--filter`）、`ImportError: cannot import name
    'SEARCH_FILTER_OPERATORS'`（这条是新用例的构造性依赖，故该类的两个 parser 名字
    **刻意不在文件顶部 import**——否则缺陷态是整份文件收集失败，连带 104 条既有例一起变红）。
    恢复后四份实现字节一致：`e0457184e0`(667 CRLF) / `49b70b3801`(443) / `45b32e651d`(179) /
    `d3659f6251`(864)，纯 LF 均为 0；HEAD 态对应 `95ac0a1aa3`(574) / `94ef1729f8`(426) /
    `e1cf45f7df`(154) / `7eec9cd84b`(858)。
  - 全量：**4191 passed / 2 skipped**（133.96 s，总计 **98.98%**）。计数
    **4132 + 59 = 4191 精确对上**，无外部漂移。`search_enhanced.py` **243 语句 0 missed**
    （L27 是 213），只余 `196->exit`（双重检查锁的 `with` 出口，先于本轮，即 L27 记的
    `112->exit` 因新增 `normalize_filters` 换了行号）；`cli/parser.py` **199 语句 100%**；
    `data_ops.py` 82 语句 0 missed、只余 `61->68`（先于本轮）；`api/routes/dataset_tools.py`
    311 语句只缺 `612-613 / 684-685`（**另两条路由**的 `except Exception → to_http_error`，先于本轮）。
  - **另立 A31 / A32**：A31 = 生效的过滤器不回显（A29 同族，`SearchResult` 新增第三种
    「找到 0 条」成因读不出来）；A32 = `normalize_filters` / `FILTER_OPERATORS` 不在包级导出，
    且 API 的 `filters` 是裸 `List[Dict[str, Any]]`，OpenAPI schema 不表达算子域——
    改成 pydantic 子模型会把 400 换成 422，要先算清两层校验谁是第一道，故不当轮顺手做。
- **L29** （哈希由下一轮提交补） `feat(search_enhanced)` A31 —— 让「被过滤器筛光」这条成因自己说话。
  - **症状**（L28 产物自己的残留）：接上 `filters` 之后，「找到 0 条」在 CLI 上有三种成因、
    两种读法。实测 HEAD：`--query 租房 --filter output contains 退租`（2 条候选被筛光）与
    `--query 根本没有这个词 --filter input eq ""`（检索就没命中）两份 stdout **逐字相同**，
    两份 `--output` JSON 也只剩 `total_matches: 0` 一个数；API 侧连这点差别都没有。
    这不是理论需求：过滤器是本轮才接到三端的，用户第一次能在 CLI 上把 419 条筛成 10 条，
    而「筛成 0 条」与「没有 419 条」读起来一样时，唯一 debug 手段是删掉 `--filter` 重跑。
  - **口径（本轮唯一设计决定，全部承 L27）**：没过滤 → `None`，**不是** `[]` / `0`，
    否则①「没过滤」与③「过滤后剩 0 条」重新同形，本轮就白做；`matches_before_filters` 与
    `total_matches` 都是**分页前**的全集口径（`limit` / `offset` 只切 `items`）；
    回显取**规范化之后**的字典清单（`SearchFilter.to_dict()`），不泄漏调用方传入的对象，
    且是新建容器——两条用例分别钉「传 `SearchFilter` 对象时回显仍是 dict」与
    「改回显不污染入参、也不影响下一次查询」。
  - **连带踩到的一条**：成员档（`in` / `not_in`）允许集合，而 `SearchFilter.to_dict()`
    原样带着集合值 → `--output` 与 API 响应的 JSON 序列化会 `TypeError`。落在
    `normalize_filters()` 里把 `set` 归一成 `list`（判定与集合等价，语义不动），
    用例 `test_a_set_filter_value_still_leaves_a_json_serializable_result` 同时钉
    「回显可 `json.dumps`」与「命中集合不变」。副作用另立 **A34**：`list({...})` 的顺序
    随 `PYTHONHASHSEED` 变，实测三个进程给出 `['alpha','beta','gamma']` /
    `['gamma','beta','alpha']` / `['alpha','beta','gamma']`，即回显**跨进程不字节稳定**
    （只有 SDK 调用方传集合会碰到，CLI 与 API 的值来自 JSON 数组）。
  - **CLI 侧 `effective` 从赋值改成 `+=`**：旧写法两段互斥（`if/elif`），加了过滤器段之后
    若不累计，`--method fuzzy --filter …` 会**互相覆盖**。整行等值断言钉住顺序：
    `搜索方法: fuzzy (生效阈值 0.6) (1 个过滤器: 检索 2 → 保留 1)`。仍然印在第二行末尾，
    不新起一行（下游按 `out[out.index("["):]` 取条目）。
  - **代价实测（同进程 back-to-back min-of-7，真实 6902 条）**：contains 无过滤
    **1.82 → 1.80 ms（0.990）**、1 条过滤 **1.79 → 1.78（0.997）**、3 条过滤
    **1.75 → 1.76（1.007）**、fuzzy 无过滤 **17.47 → 17.16 ms（0.983）**；命中数两侧相同
    （419 / 10 / 10 / 42），四个比值全在 ±1% 内 → 噪声。**本轮不把墙钟当证据**：
    回显在 `if normalized_filters` 分支内、`for filter_item` 循环外，所以真正的护栏是**计数式**
    （承 L12/L14）——`SearchFilter.to_dict` 每次查询的调用数 == 过滤器条数而不是候选条数，
    同一用例先用「带 1 个过滤器时计数非 0」自证探针是响的，再断言无过滤路径调用数为 0。
    （顺带把该调用点 grep 了一遍确认全库只有 `search_enhanced.py:387` 一处，否则计数无意义。）
  - **改写既有例 2 处 + 契约期望 1 处**（承 L19「放宽判据会让上一轮护栏失效，必须挪进 NEW 侧」）：
    L27 的 `TestTheResultEchoesTheKnobsItUsed` 里 `to_dict()` 键集合 8→10、
    「不带新字段构造仍可用」那条扩两字段；`test_api_openapi_contract.py` 的 search 契约
    `keys=` 8→10（这一处驱动 **2** 条红：`test_response_matches_domain_contract[route_key40]`
    与 `test_openapi_advertises_domain_contract[route_key40]`）。**API 这次没被静默裁剪抓到**
    是因为改结果对象与改 `response_model` 在同一轮，而那两条契约用例正是用来保证这件事的。
  - 新增 **38 例**（unit 188 → **207**、CLI 104 → **114**、API 228 → **237**）。
  - **注入对照（最后一次测试改动之后跑）**：3 份实现（`search_enhanced.py` /
    `cli/commands/data_ops.py` / `api/routes/dataset_tools.py`）退回 HEAD →
    **40 红 / 667 绿**（707 = 40 + 667，与恢复后同一批 4 文件的 707 全绿**精确一一对上**）。
    红的构成：unit 21（新 19 + 更新 2）+ CLI 8 + API 9 + 契约 2。
    **2 绿在白名单**，都是「护栏不该管的地方」：
    ①`test_unfiltered_search_prints_no_filter_section`——没传过滤器时第二行不许出现括号，
    缺陷态什么都不印所以必然绿，守的是下一轮把回显放宽成「一律印 0 个过滤器」；
    ②`test_stdout_shape_is_still_two_summary_lines_then_json`——两行摘要 + JSON 的下游形状，
    本轮刻意不变所以必然绿（承 L27 同一条）。
  - 恢复后三份实现字节一致：`c52e40256f07`(696 CRLF) / `db2292677b3f`(186) / `2a393842b1cb`(869)，
    纯 LF 均为 0；HEAD 态对应 `e0457184e038`(667) / `45b32e651dad`(179) / `d3659f6251d5`(864)。
  - 全量：**4229 passed / 2 skipped**（109.62 s，总计 **98.98%**）。计数
    **4191 + 38 = 4229 精确对上**，无外部漂移。`search_enhanced.py` **243 → 251 语句 0 missed**，
    只余 `214->exit`（双重检查锁的 `with` 出口，先于本轮，即 L28 记的 `196->exit` 因新增
    set→list 归一而换了行号）；`data_ops.py` 82 → **84 语句 0 missed**、只余 `61->68`（先于本轮）；
    `api/routes/dataset_tools.py` 311 → 313 语句只缺 `617-618 / 689-690`（**另两条路由**的
    `except Exception → to_http_error`，先于本轮）。
  - **另立 A33 / A34**：A33 = 多条过滤器只有合计数，说不出**哪一条**把候选清零（本轮回显
    自己的下一步）；A34 = 集合值回显跨进程顺序不稳（上面实测的三个排列）。
- **L30** （哈希由下一轮提交补） `perf(dedup)` A17 剩下的那一半 —— 不装 scipy/faiss，用 numpy 原语把稠密 n-gram 矩阵换成倒排 postings，并让**两条实测成本账**决定换不换。
  - **症状定位（先量再改）**：`_find_duplicate_groups_chunked` 在 fallback 编码下先把
    `n×vocab` 稠密 float32 整份物化（真实 6902 条 × 词表 23033 = **606 MiB**，而非零元
    只有 194,293 个，稀疏度 0.9988），再让 BLAS 去乘。归因实测：那条路上
    `n²/2 × vocab ≈ 5486 亿`次乘加，按本机 86 G MAC/s 折 **≈ 6.4 s**，占端到端 10.5 s 的
    绝大部分；A19 那两遍 Python 双循环只占 0.36 s（4%）——**所以本轮改的是乘零，不是循环**。
  - **改法**：`_ngram_entries`（单趟收集 + `np.unique` 压缩 + `np.bincount` 计数，
    真实数据 **73 ms vs `_fallback_encode` 330 ms**）→ `_inverted_view`（ postings 按词项
    `argsort` 稳定排序、行内 L2 归一在 float64 算完再降 float32、顺带算出
    `pair_adds = Σ df(df+1)/2`）→ `_iter_inverted_similarity_row_blocks`（逐块用
    `searchsorted` 取该块文档命中的词项区间，一次 `np.bincount(flat, weights=...)` 做完整
    散射累加）。**产出形状与原稠密路径逐块一致**（`(end-start) × (n-start)` float32、
    同一列语义），所以 L9 的块契约、L6 的贪心归组、以及 **52 条既有 dedup 用例全部不用改**。
  - **判据是两条独立的账，不是一条**（`_inverted_blocks_win`）：
    ①**时间账** `pair_adds × 3000 < n²/2 × vocab` —— 单价 3000 来自实测：稠密 BLAS
    86 G MAC/s vs 倒排 29.5 M add/s（比值 2914，取 3000 偏保守）；
    ②**内存账** `稠密 n×vocab×4×100 > pair_adds × block_rows × 96 × 125` —— 96 B/pair
    由逐档实测峰值拟合（随机语料 n=1200→93.5、2400→93.5、3600→94.5、4800→95.0、
    6000→96.9、7200→98.8 B/pair，真实 6902 条 85.2 B/pair，取上沿），125% 是裕度。
    **两条都对倒排有利才换路**；判据不看 `_model` 是刻意的——它在 `fallback_encode` 之前
    调用，拿不到编码结果，只能从 `_inverted_view` 顺手算出的三个整数推断。
  - **收益（真实 6902 条 `instruction`，min-of-3，同进程对照 HEAD）**：端到端
    **969 ms（分布 982/960/954）vs 10484 ms（10660/9644/8792），比值 0.092 ≈ 10.8×**；
    `tracemalloc` 峰值 **149.1 MiB vs 621.8 MiB（4.17×）**；
    `duplicate_groups` 与 HEAD **逐组完全相同**（369 组 / 移除 377 条），
    逐块最大绝对差 **9.537e-07**、阈值 0.9 上判定翻转的单元数 **0**。
  - **本轮内自我推翻一次（这是最有价值的部分）**：内存账最初写成「稠密矩阵超过 64 MiB
    就换」，听起来合理，但**没实测**。随后逐档跑随机 60 字符语料 n=1200…7200，发现
    倒排峰值**一直是稠密的 1.4～2.4 倍**（54.2/22.2、104.0/44.4、153.9/66.6、
    171.3/85.7、173.9/103.3、175.9/120.9 MiB），且 n=4800 档倒排还**慢 7%**（620 vs 577 ms）
    ——按 64 MiB 那条线会在**内存反而更贵**的三档上判赢。于是把代理量换成拟合的
    `pair_adds × block_rows × 96 B`，并**给判据加了第 4 个形参 `block_rows`**（同一份语料，
    块越大越贵，判据必须跟着变）。真实语料不受影响（606 MiB vs 估算 176 MiB，裕度充足）。
  - **退化语料必须留在稠密**（判据的第二半，全部实测）：全重复语料 vocab 塌成 9，
    `pair_adds` 2.1440 亿 ≈ 稠密乘加 2.1437 亿（**次数上零优势**，输在单价 ×3000），
    实测倒排 6.23 s vs 稠密 0.09 s（**慢 69 倍**）→ 判 False；`["x"]*6000`（长度<2 无
    二元组 → vocab 0）判 False，这一档同时兜住既有护栏
    `test_no_full_n_by_n_matrix_is_allocated` 不因换路而假红；n=1200 的 fixture 判 False
    （省下的 17.6 MiB 不值 postings 常驻）。真实 `output` 字段 pair_adds 1.466 亿 → 判
    False（**赢是 instruction 那一类多样性带来的，不是普遍属性**），`input` 全空 → False。
  - 新增 **42 例**（`test_dedup.py` 52 → **94**），**既有断言 0 改写**。三组新用例的分工：
    `TestNgramInvertedView`（16）钉表示本身；`TestInvertedBlocksMatchDenseBlocks`（28）
    拿稠密当 oracle 逐元素对照（含 block_rows=1/2/3/5/1000、零语料、同文得 1.0）；
    `TestTheProducerIsChosenByMeasuredCost`（22）把上面**实测表**写进 `MEASURED` 并要求
    判据复现（含时间账/内存账的**精确边界翻转**、单价拟合 0.8–1.3 倍带宽）；
    `TestOnlyOneProducerRuns`（18）用 4 个计数探针证明选定后另一条**一次都没跑**，
    并有第三条独立浮点路径（float64 手工点积）作参照。
  - **等价性用例一度是红的，而且红得有意义**：阈值 0.5 上「逐组相同」断言失败
    （`[0,1,7,…]` vs `[0,1,2,7,…]`）。没有挪阈值糊过去，而是查到底：9 对文本的相似度
    **精确等于 0.5**，稠密（双归一）落在 0.500000，倒排（散射累加）落在
    0.4999999701976776，**18 个单元在阈值线上翻转**。改法是把断言写成
    「误差 ≤ 2e-6 带宽 **且** 任何翻转单元离阈值不超过该带宽」，并专门留一条
    `test_an_exact_tie_is_not_a_producer_bug` 把这个事实钉住；端到端一致性用例则用
    自证前置（`margin > 1e-4`）挑无平局的阈值档。**残留口径另立 A36**。
  - **注入对照（最后一次测试改动之后跑）**：`dedup.py` 退回 HEAD → 新增 42 例
    **42 红 / 0 绿**（无白名单绿例：倒排的表示、块等价、判据、生产者互斥，四条面在
    HEAD 里全都不存在），**52 条既有用例在缺陷态全绿**（= 未改既有断言的正面证据）。
    恢复后实现字节一致 `6bf5bf1564b4`（769 CRLF / 769 LF / 34,719 B）。
  - 全量：**4271 passed / 2 skipped**（48.94 s，总计 **98.98%**）。计数
    **4229 + 42 = 4271 精确对上**，无外部漂移。`dedup.py` **266 语句 0 missed**，
    只余 1 条 partial 分支 `171->169` —— 本轮**查明**它是永不到达的死分支
    （`if ngram in vocab` 恒真，vocab 由同一个 texts 建成），已写进 **A19**。
  - **另立 A35 / A36 / A37**：A35 = `find_similar_pairs` 还在物化整份稠密矩阵，
    L30 的 10.8× 它一分拿不到（本轮边界外，如实记）；A36 = 恰好等于阈值的平局在两条
    生产者下判定不同（已钉成用例，但文档口径待拍）；A37 = 倒排峰值 149.1 MiB 仍随
    `pair_adds × block_rows / n` 增长，离 16 MiB 单块上界差一个量级，且 96 B/pair 的拟合
    只在 n≤7200 上做过、未外推。
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
  L19 后 **3886 passed / 3 skipped**（57.4 s，总计 98.53%），
  L20 后 **3905 passed / 3 skipped**（55.5 s，总计 98.50%），
  L21 后 **3924 passed / 3 skipped**（61.0 s，总计 98.54%），
  L22 后 **3951 passed / 3 skipped**（55.7 s，总计 98.54%），
  L23 后 **3982 passed / 3 skipped**（56.6 s，总计 98.54%，`converter.py` 语句 0 missed），
  L24 后 **4022 passed / 2 skipped**（43.2 s，总计 98.97%，`search_enhanced.py` 语句 0 missed；
  收集总数除本轮 +20 例外另有 +19 例 / −1 跳 的**外部漂移**，归因未坐实，详见 L24 日志），
  L25 后 **4073 passed / 2 skipped**（152.46 s 含并行竞争，总计 **98.98%**，`search_enhanced.py`
  语句 0 missed；**计数 4024 + 51 = 4075 精确对上**，解释器与分目录快照见 L25 日志），
  L26 后 **4106 passed / 2 skipped**（113.45 s，总计 **98.98%**，`search_enhanced.py`
  211 语句 0 missed；**计数 4075 + 33 = 4108 精确对上**，纯增量、0 处既有断言改写），
  L27 后 **4132 passed / 2 skipped**（113.31 s，总计 **98.98%**；**计数 4106 + 26 = 4132 精确对上**，
  本轮更新了 1 处既有契约用例（驱动 2 条红），注入 **24 红 / 590 绿**，4 绿全在白名单），
  L28 后 **4191 passed / 2 skipped**（133.96 s，总计 **98.98%**；**计数 4132 + 59 = 4191 精确对上**，
  **0 处既有断言改写**，注入 **58 红 / 615 绿**，1 绿在白名单，恢复后四份实现字节一致），
  L29 后 **4229 passed / 2 skipped**（109.62 s，总计 **98.98%**；**计数 4191 + 38 = 4229 精确对上**，
  改写既有断言 2 处 + 契约期望 1 处（驱动 2 条红），注入 **40 红 / 667 绿**（707 与恢复后同批
  4 文件的全绿数一一对上），2 绿在白名单，恢复后三份实现字节一致）。
  L30 后 **4271 passed / 2 skipped**（48.94 s，总计 **98.98%**；**计数 4229 + 42 = 4271
  精确对上**，**0 处既有断言改写**，注入 **42 红 / 0 绿**、无白名单绿例，52 条既有用例在
  缺陷态全绿，恢复后实现字节一致 `6bf5bf1564b4`；`dedup.py` 266 语句 0 missed）。
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
