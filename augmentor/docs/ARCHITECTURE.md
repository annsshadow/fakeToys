# 架构说明

## 1. 总体结构

平台采用**分层架构**：核心引擎（`augmentor`）不依赖任何 Web 框架，API 层（`api`）与 CLI（`cli.py`）
都是核心引擎的薄适配层。因此同一套能力可以通过 Python API、命令行、HTTP 三种方式驱动，行为完全一致。

```
┌──────────────────────────────────────────────────────────────┐
│  表现层                                                       │
│  ├── Web UI   web/src  (React 18 + TS + Ant Design 5 + ECharts)│
│  ├── CLI      cli.py   (argparse 子命令)                       │
│  └── REST API api/     (FastAPI + uvicorn)                     │
├──────────────────────────────────────────────────────────────┤
│  编排层                                                       │
│  └── augmentor/pipeline.py  AugmentorPipeline                  │
│      组装全部组件，对外暴露 augment_dataset / export_dataset /  │
│      analyze_dataset / visualize_dataset                       │
├──────────────────────────────────────────────────────────────┤
│  能力层                                                       │
│  ├── 生成   models/  context.py  expander.py                   │
│  ├── 质量   quality.py  dedup.py  report.py  benchmark.py      │
│  ├── 数据   data/ (cleaner, annotator, image, audio, multimodal)│
│  ├── 转换   export.py  preview.py  rag.py  multilingual.py     │
│  ├── 存储   versioning.py  checkpoint.py  vector/              │
│  └── 评估   evaluation.py  active_learning.py  tracker.py      │
├──────────────────────────────────────────────────────────────┤
│  基础设施层                                                   │
│  ├── config.py       dataclass 配置中心 + YAML/环境变量解析     │
│  ├── model_manager.py 单例，共享 sentence-transformers 实例     │
│  └── visualizer.py   图表生成（可选依赖）                       │
└──────────────────────────────────────────────────────────────┘
```

依赖方向严格单向向下：`api` / `cli` → `pipeline` → 能力层 → 基础设施层。能力层内部不反向依赖 `pipeline`。

## 2. 目录结构

```
augmentor/
├── cli.py                     命令行入口（薄壳：解析参数 → 查分发表 → 调用 handler）
├── config.yaml                默认配置
├── requirements.txt           依赖清单（分组 + 版本上界）
├── pytest.ini                 测试与覆盖率门禁配置
├── .coveragerc                覆盖率报告配置
│
├── augmentor/                 核心引擎
│   ├── __init__.py            对外导出面（版本 3.0.0，全仓唯一版本声明）
│   ├── pipeline.py            主流程编排
│   ├── config.py              dataclass 配置中心
│   ├── model_manager.py       sentence-transformers 单例
│   │
│   ├── models/                模型后端（工厂模式）
│   │   ├── base.py            ModelBackend 抽象基类 + 重试 + 会话池
│   │   ├── factory.py         create_model_backend
│   │   ├── ernie.py          百度 ERNIE
│   │   ├── openai_model.py   OpenAI 兼容接口
│   │   ├── ollama.py         本地 Ollama
│   │   ├── claude.py         Anthropic Messages API
│   │   └── gemini.py         Google generateContent API
│   │
│   ├── quality.py             三维质量评分
│   ├── dedup.py               向量语义去重
│   ├── report.py              质量报告生成
│   ├── benchmark.py           质量基准与回归对比
│   ├── preview.py             导出格式预览
│   ├── export.py              多格式导出 + 批量导出
│   ├── versioning.py          版本快照/对比/回滚/历史
│   ├── checkpoint.py          断点续传
│   ├── evaluation.py          BLEU / ROUGE-L / 相似度
│   ├── active_learning.py     主动学习循环
│   ├── multilingual.py        多语言翻译增强
│   ├── rag.py                 RAG 格式转换
│   ├── context.py             多轮对话上下文增强
│   ├── expander.py            领域主题扩展
│   ├── sampler.py             覆盖度分析与选样推荐
│   ├── tracker.py             实验追踪
│   ├── visualizer.py          数据可视化
│   │
│   ├── data/                  数据加工
│   │   ├── cleaner.py         清洗（去噪/标准化/语言检测）
│   │   ├── annotator.py       自动标注（实体/意图/情感）
│   │   ├── image.py           图像元数据
│   │   ├── audio.py           音频元数据
│   │   └── multimodal.py      多模态融合
│   │
│   ├── vector/                向量数据库（工厂模式）
│   │   ├── base.py            VectorDB 抽象基类 + L2 归一化
│   │   ├── faiss.py           FAISS（缺失时回退 numpy）
│   │   └── chromadb.py        ChromaDB
│   │
│   ├── frameworks/            LLM 框架互转
│   │   ├── langchain.py
│   │   └── llamaindex.py
│   │
│   └── cli/                   CLI 实现（37 个子命令，按领域分 9 组）
│       ├── io.py              4 个共用 I/O helper
│       ├── parser.py          build_parser()（全部 argparse 定义）
│       └── commands/          profiling / pipeline / quality / export /
│                              analysis / data_ops / version / security / ops
│
├── api/                       REST API
│   ├── main.py                FastAPI 应用装配（中间件 + 路由 + 静态文件）
│   ├── deps.py                依赖注入：管道单例、路径白名单、鉴权、异步文件 IO
│   ├── middleware/            限流、请求日志与耗时统计
│   └── routes/                按领域拆分的 11 个 router
│       ├── data.py            数据 CRUD / 上传 / 分析 / 可视化
│       ├── augment.py         增强任务与进度
│       ├── quality.py         评估 / 去重 / 报告 / 清洗 / 标注 / 基准 / 画像
│       ├── export.py          导出 / 批量导出 / 预览 / 格式清单
│       ├── version.py         版本 CRUD / 对比 / 回滚 / 历史
│       ├── config.py          配置与模型清单
│       ├── multimodal.py      多模态处理与扫描
│       ├── status.py          健康与依赖诊断
│       ├── audit.py           数据集审计
│       ├── leakage.py         数据泄漏检测
│       └── privacy.py         隐私风险检测
│
├── web/                       Web UI
│   └── src/
│       ├── App.tsx            路由与菜单（9 个页面）
│       ├── services/api.ts    后端接口封装
│       ├── components/        可复用组件（5 个）
│       └── pages/             页面（9 个）
│
├── tests/
│   ├── conftest.py            共享 fixture
│   ├── unit/                  单元测试
│   ├── integration/           API 与管道集成测试
│   └── e2e/                   前端结构与前后端契约测试
│
└── docs/                      文档
```

## 3. 关键设计决策

### 3.1 可选依赖的降级而非崩溃

所有非核心依赖都采用**惰性加载 + 降级**：

```python
def _load_wordcloud(self):
    if self._wordcloud is None:
        try:
            from wordcloud import WordCloud
            self._wordcloud = WordCloud
        except ImportError:
            logger.warning("wordcloud 未安装，跳过词云生成")
```

调用方通过返回值区分「成功」与「降级」：降级时返回空字符串或空列表，而不是抛异常。
这样一套代码可以同时运行在「装了全部依赖的服务器」和「只有核心依赖的开发机」上。

各模块的降级对照：

| 模块 | 首选实现 | 降级实现 |
|------|---------|---------|
| 语义相似度 | sentence-transformers | n-gram 相似度 |
| 向量检索 | FAISS | numpy 精确检索 |
| ChromaDB | chromadb 包 | 构造时抛 `ImportError`（明确提示，不静默） |
| 图像元数据 | Pillow | 标准库解析 PNG/GIF/BMP/JPEG 文件头 |
| 音频元数据 | soundfile | 标准库 `wave` 解析 WAV |
| 图表 | matplotlib / wordcloud / sklearn | 跳过并返回 `""` |
| LLM 框架 | langchain / llama-index | 返回等价的普通字典 |

### 3.2 模型后端：工厂 + 模板方法

`ModelBackend` 在基类中实现统一的重试、退避、会话复用与响应解析骨架，
子类只需实现 `_call_api(prompt)` 一个方法：

```python
class ClaudeBackend(ModelBackend):
    def _call_api(self, prompt: str) -> str:
        # 只负责请求构造与响应提取
```

`create_model_backend(config)` 按 `config.type` 分发，未知类型给出**列出全部合法取值**的错误信息，
避免排查时反复试错。

### 3.3 管道组件共享

`AugmentorPipeline.__init__` 一次性组装全部组件并共享模型后端实例。
`ContextAugmentor` 与 `DomainExpander` 持有的是同一个 backend 引用，因此重试策略与连接池全局一致。

`model_manager` 是模块级单例，保证 `sentence-transformers` 模型只加载一次，
避免每个 `QualityScorer` / `Deduplicator` 实例各加载一份（单实例约 400MB）。

### 3.4 API 层的异步与阻塞隔离

FastAPI 路由是 `async def`，但核心引擎是同步阻塞的。所有可能耗时的调用都通过
`run_in_thread` 抛到线程池，避免阻塞事件循环：

```python
results = await run_in_thread(p.export_dataset, input_file, output_dir, formats)
```

> **`load_items()` / `save_items()` 的「供线程内使用」是契约，不是建议**
>
> `api/deps.py` 里同时存在两对读写函数：`read_json_file` / `write_json_file` 是
> **异步**的（内部已 `run_in_executor`），而 `load_items` / `save_items` 是**同步**的，
> 文档串里就写着「供线程内使用」——它们是给 `run_in_thread(run)` 里的那个 `run` 用的。
>
> 12 条端点（`/api/quality/*` 8 条 + `/api/audit` + `/api/leakage/check` +
> `/api/privacy/sanitize` + `/api/export/preview`）曾经把**分析**离线了、却把开头那行
> `items = load_items(request.input_file)` 留在 `async def` 函数体里，于是每个请求都在
> 事件循环上同步读盘：实测 3.6 MB / 6902 条的 `train_data.json` 要 **15 ms**，这段时间
> 全站其他请求一起停。现在这 14 处读取点统一写成
> `items = await run_in_thread(load_items, request.input_file)`。
>
> 这条不变量由 `tests/integration/test_api_event_loop_blocking.py` 用**线程身份**守着，
> 而不是靠计时（计时在 CI 上只会误报）：`httpx.ASGITransport` 在当前线程直接跑事件循环，
> 所以「读文件发生在哪个线程」是确定的。同文件的第二条用例锁住响应仍是 200，
> 防止「干脆不读了」把第一条糊过去。
>
> 实测效果（把读盘人为放慢到 50 ms 后打 4 条并发请求）：并发总耗时 **59.5 ms**，
> 而读取仍占着循环时同样是 4 次读取至少要 **200 ms**；并发期间事件循环空转 27349 次。
>
> **同一族还有第二处**：`dataset_tools.py` / `system_ops.py` 用的是收「已 resolve 路径」的
> 同步 `read_items()`，11 处读取点同样躺在 `async def` 函数体里；`/api/dataset/stats` 甚至
> 把 `calculate_statistics()` 也留在循环上——实测 6902 条要 **43.4 ms**，比读同一份文件的
> 9.2 ms 还贵 4 倍。现在这些点位一律写成 `await read_json_file(path)`（就是 `read_items`
> 的异步外壳），守卫用例从 24 条扩到 45 条；`_record_reads` 同时打在路由模块自己的
> `read_items` / `load_items` 和收口处的 `deps.read_items` 上——只打其中一处，就只能看见
> 缺陷态与修复态里的某一种。
>
> **线程池的边界是 GIL**。离线只对「等得起」的东西有效：文件读取是 I/O，工作线程阻塞时
> GIL 是放开的，所以并发能重叠（上面那组 59.5 ms）。纯 Python 的分析不是——3 条并发
> `/api/dataset/stats` 实测：缺陷态（在循环上算）总耗时 173.7 ms、事件循环最大停顿
> **170.1 ms**、期间循环只空转 6 次；离线后总耗时 192.7 ms（线程交接 + GIL 争用，对
> **发请求的人**慢 11%），但循环最大停顿降到 **60.0 ms**、空转 12 次。也就是说离线买到的是
> 「别人的请求还能被服务」，不是「这条请求更快」。要让 CPU 型分析真并行，得换
> `ProcessPoolExecutor` 或者把算法本身变快，线程池不是那个手段。

管道实例由 `get_pipeline()` 惰性创建并缓存，`reset_pipeline()` 供测试注入替身。

### 3.5 断点续传的一致性

这是本项目最容易出错的部分，设计要点：

1. **任务 ID 必须跨进程稳定**。早期实现用 `augment_{int(time.time())}`，导致每次运行都是新任务，
   永远无法恢复。现改为由输入文件绝对路径派生：`augment_{md5(path)[:12]}`。

2. **增量文件累加写入**。`_save_delta()` 先读取已有增量再合并本次待保存索引。
   早期实现用 `'w'` 模式只写当前批次并清空待保存集合，第二次自动保存会把第一次的增量整个覆盖，
   崩溃后只能恢复最后一批。

3. **合并后立即清除增量**。`load_checkpoint` 把增量并入主文件后立刻重写主文件并删除增量文件；
   `save_checkpoint` 写完主文件后同样删除增量。否则下次恢复会把同一份增量重复累加，
   导致 `processed_items` 翻倍、`completed_indices` 出现重复。

4. **恢复时载入上一轮输出**。断点只记录「哪些索引处理过」，不保存生成内容。
   若恢复后直接写输出文件，会把上一轮结果覆盖为空。因此恢复分支会先读入已存在的输出文件，
   与本轮结果合并后再去重落盘。

5. **`quality_scores` 键类型还原**。该字段声明为 `Dict[int, float]`，JSON 往返后会变成字符串键，
   加载时显式转回 `int`，否则 `Dict[int, float]` 的类型契约名存实亡。

### 3.6 版本 ID 的唯一性

版本 ID 早期为 `%Y%m%d_%H%M%S`（秒级）。同一秒内创建两个版本会互相覆盖。
现改为 `%Y%m%d_%H%M%S_%f`（微秒级）并附加存在性去重循环，从根上消除碰撞。

版本操作会追加写入 `history.jsonl`，`get_history()` 倒序返回。
读取时按行 `try/except` 跳过损坏行，单行损坏不会导致整个历史不可用。

### 3.7 质量评分与报告

总分由三个维度加权：

```
total_score = 0.3 × 语义相似度 + 0.4 × 回答相关性 + 0.3 × 多样性
```

权重与阈值均可通过 `config.yaml` 的 `quality.weights` / `quality.threshold` 调整。

「多样性」只与**参照窗前 `diversity_sample_size` 条**（默认 30）已采纳文本比较，参照集未变时
复用缓存。该缓存的失效判据是「上一轮缓存的那一段列表」逐元素相等，**不做内容指纹**：早期用
`md5("|".join(切片[:100]))`，既不单射（`["a", "b"]` 与 `["a|b"]` 同键，实测多样性虚高 0.565）
又带 100 条截断（窗口 >100 时尾部变化看不见，虚高 0.837），且每条候选都要重算整段字节。
批量编码缓存同样以批次内容元组为键——指纹非单射时轻则静默拿到别批的向量，重则条数不匹配抛
`IndexError`。见优化轮 L32。

`ReportGenerator` 基于评分与去重结果生成结构化报告与**可执行的改进建议**，
而不是只输出一堆数字。建议规则见 [API 文档](API.md#post-apiqualityreport)。

### 3.8 质量基准的缺失指标处理

`QualityBenchmark.compare_with_baseline` 对基准中不存在的指标标记 `status: "no_baseline"`
且 `baseline`/`delta` 为 `null`，**不参与** improved/regressed 统计。
早期实现把缺失值当作 `0`，会把「基准里没有这个指标」误报成「指标从 0 提升到 X」的虚假改进。

### 3.9 去重的分块相似度矩阵

`Deduplicator` 对 N 条数据计算 N×N 相似度矩阵，N 较大时内存会爆炸。
实现按 `chunk_size=1000` 分块计算，只保留超过阈值的相似对。

`find_similar_pairs()` 与 `deduplicate()` 现在**共用同一个相似度块生产者** `_similarity_blocks`：
逐块取上三角、边并入边按「相似度降序、同分按 `(i, j)` 升序」剪到 `top_k`，所以保留
集大小至多 `top_k + 单行达标数`，**不随总达标对数增长**。收录门槛是 `threshold × 0.8`
（刻意比去重阈值松一档，让调用方看得见「接近但没到重复」的配对）。

**早期版本只比较前 200 条**：真实 6902 条语料上它报出的最大下标是 `instruction` 158 /
`output` 49，第 250 条之后的重复永远不会被报出，且不报错、不提示截断 —— 该缺陷已在
优化轮 L31 修掉（`find_similar_pairs()` 早期还曾调用不存在的 `_compute_similarity_matrix`，
运行期必然 `AttributeError`，那也是同一方法上的历史缺陷）。

### 3.10 评估指标的数值稳定性

BLEU 使用标准裁剪计数与简短惩罚。当所有 n-gram 的裁剪计数均为 0 时**直接返回 0.0**，
否则平滑项会产出一个非 0 的极小值，让人误以为存在微弱匹配。

中文按**字**切分，英文与数字按**词**切分，避免中文被当成一个整词导致 BLEU 恒为 0。

### 3.11 前端与后端的契约

`tests/e2e/test_web.py` 通过 `app.openapi()["paths"]` 枚举后端全部路由，
再用正则从 `web/src/services/api.ts` 中提取前端发起的请求路径，逐一比对。

该契约测试在 API 层拆分时抓到过一次真实回归：`/api/analyze/{filename}` 与
`/api/visualize/{filename}` 在拆分中丢失，前端调用会 404 导致页面白屏。

> 注意：当前 FastAPI 版本把 `include_router` 包装为不展开的 `_IncludedRouter`
> 容器（`path` 与 `methods` 均为 `None`），因此不能遍历 `app.routes`，
> 必须走 OpenAPI schema。

### 3.12 安全边界

三条防线：**路径白名单**、**写操作鉴权**、**限流**。共同前提是「默认单机开箱即用，
生产部署再收紧」——因此三项都默认宽松，但都会在启动日志或响应中明确暴露当前状态，
不制造「以为已经安全」的错觉。

#### 3.12.1 路径白名单

所有接受客户端传入路径的接口都必须经 `api/deps.py` 规范化，不允许直接把字符串拼进
`open()`：

| 函数 | 语义 | 失败码 |
|------|------|--------|
| `resolve_within_roots(name, label)` | 只做边界校验，不要求存在 | 400 / 403 |
| `resolve_data_path(name, for_write=False)` | 文件路径；读操作要求 `is_file()` | 400 / 403 / 404 |
| `resolve_data_dir(name)` | 目录路径 | 400 / 403 |
| `default_registry_dir()` | 依赖注册表缺省目录：白名单首个根目录下的 `.dependency_registry` | 不失败 |
| `default_backup_dir()` | 备份目录缺省值：白名单首个根目录下的 `.backups` | 不失败 |
| `config_file_path()` | 服务**自身**配置文件路径（`AUGMENTOR_CONFIG_PATH` > 工作目录 `config.yaml`），不过数据白名单 | 不失败 |

校验顺序：

1. 空值或非字符串 → `400`；
2. 路径含 `..` 组件 → `400`（在 `resolve` 之前判断，避免依赖文件系统语义）；
3. 相对路径先生成候选：白名单各根目录在前、进程工作目录兜底，取第一个**存在**的解释
   （裸文件名因此能在 `data/` 内命中，见 `_relative_candidates`）；一个都不存在时落回
   工作目录解释，**不**替写入请求猜目录。`Path.resolve()` 规范化并**展开符号链接**；
4. 逐条与白名单根目录做 `relative_to` 比对，全部不匹配 → `403`。

白名单来源按优先级解析（`allowed_data_roots()`）：

```
AUGMENTOR_DATA_ROOTS（os.pathsep 分隔）
  └─ 未设置 → config.yaml 的 web.data_roots
       └─ 读取失败或解析为空 → 出厂默认 WebConfig.data_roots（["data"]）
```

两级降级都落到**出厂默认**而不是工作目录：空白名单意味着所有请求 403，比放宽更危险，
但降级到 `["."]` 会把出厂默认刚收紧掉的范围又悄悄放开，等于修复失效。
出厂默认在 `WebConfig` 字段、`load_config` 的兜底字典、仓库根 `config.yaml` 三处各写一遍，
由 `tests/integration/test_api_security.py::TestShippedDataRootsDefault` 用字面量分别钉住并交叉比对。

环境变量优先是为了让容器部署与测试无需改配置文件即可收紧或放宽范围；
`tests/conftest.py` 的 autouse fixture 正是通过它把 cwd 与临时目录注入白名单。

> **两条来源都带缓存，缓存键都带「值会变的东西」**
>
> 任何带路径参数的请求至少要过一次白名单，而未缓存时 `_config_data_roots` 每次要
> **重解析一遍 YAML + 重 resolve 一遍根目录**（实测 7.06 ms/次），环境变量分支则把
> 0.29 ms/次 全花在 `Path.resolve()` 的系统调用上。因此：
>
> | 来源 | 缓存键 | 为什么这个必须在键里 |
> |------|--------|----------------------|
> | `config.yaml` | `(路径, mtime_ns)` | 改过配置文件要立刻生效，不能等到重启 |
> | `AUGMENTOR_DATA_ROOTS` | `(原值, os.getcwd())` | 白名单允许相对路径，换目录后同一个原值指向不同目录 |
>
> 缓存值存**已 resolve 的 `Path`**，`allowed_data_roots()` 每次返回新列表，调用方可以
> 放心增删。配置文件 stat 不到（不存在）时**不**写缓存——降级到出厂默认这条路径不该
> 被冻住。由 `TestConfigDataRootsCache` / `TestEnvDataRootsCache` 分别钉住
> 「200 次查询只解析 1 次配置」「改 mtime 立刻换根」「改相对值后换 cwd 立刻换根」。

> **收紧到 `["data"]` 的兼容性代价**
>
> 读一侧的写法没变：`/api/data/load/train_data.json` 这类裸文件名会在白名单根目录内
> 查找，因此前端把 `files[].name` 原样拼回 URL 的 8 处调用照常可用
> （由 `TestBareNameResolvesInsideRoots` 钉住这条闭环）。
> **写**一侧必须显式传 `data/xxx.json` —— 目标不存在时服务端不猜目录，越界就是 403。
> 历史上直接放在工作目录根下的数据集（如本仓库的 `train_data*.json`）需要移入 `data/`，
> 或显式设 `AUGMENTOR_DATA_ROOTS` 覆盖。Docker 镜像的工作目录就是 `/app`、数据在
> `/app/data`，因此新默认与部署方式一致。
> `/api/data/list` 的扫描范围与白名单同源（`allowed_data_roots()`），不会出现「列得出、读不到」。

> **缺省目录参数必须跟着白名单走**
>
> `/api/system/dependency/*` 三条端点有一个可选的 `registry_path`。它的缺省值曾写死为
> `.dependency_registry`（相对工作目录），而工作目录收紧后已不在白名单内 —— 于是
> **不传参的默认调用会被自己的闸拦下**，三条端点全部 403。现在由
> `default_registry_dir()` 取白名单首个根目录（出厂默认即 `data/.dependency_registry`），
> 显式传参时仍走 `resolve_data_dir()` 校验，越界照旧 403。
> SDK / CLI 侧的 `DependencyManager()` 与 `DatasetBackup()` 默认参数依然是相对工作目录，
> 那是离线工具的常识落点，不经过 API 白名单。
>
> 例外是**服务自身的配置文件**：`config.yaml` 不是数据集，因此 `config_file_path()`
> （`AUGMENTOR_CONFIG_PATH` > 工作目录 `config.yaml`）不过数据白名单，`get_pipeline`、
> `allowed_data_roots`、CORS 读取与 `POST /api/config` 的落盘共用这一个入口。客户端
> 显式传进来的配置路径仍然要过闸（F-11）。

> **为什么 `multimodal` 走 `resolve_within_roots` 而不是 `resolve_data_path`**
>
> `MultimodalProcessor` 对缺失或损坏的媒体文件是**降级**语义：在返回结果的 `errors`
> 里记录问题，而不是中断整批处理。若在这里要求文件必须存在，一个坏文件会让整个
> 请求变成 404，反而丢失了其余文件的处理结果。因此这里只做边界校验。

#### 3.12.2 写操作鉴权

写操作统一挂 `Depends(verify_api_key)`，校验请求头 `X-API-Key`：

| 方法 | 路径 |
|------|------|
| PUT | `/api/data/update/{filename}` |
| DELETE | `/api/data/delete/{filename}` |
| POST | `/api/data/upload` |
| POST | `/api/data/export`、`/api/export/batch` |
| POST | `/api/augment/start` |
| POST | `/api/config` |
| POST | `/api/versions/create`、`/api/versions/{version_id}/rollback` |
| DELETE | `/api/versions/{version_id}` |
| POST | `/api/quality/profiling`（`save=true` 时会落盘） |

读接口（列表、加载、分析、可视化、评估、去重、预览、进度、配置查询、版本查询）
**不要求**密钥，保持匿名可读。

认证模型是「**未配置即关闭**」：

```python
expected = os.environ.get("AUGMENTOR_API_KEY", "").strip()
if not expected:
    return          # 未配置 → 放行
if not x_api_key or not secrets.compare_digest(x_api_key, expected):
    raise HTTPException(status_code=401, ...)
```

- 密钥来自环境变量 `AUGMENTOR_API_KEY`，不写入 `config.yaml`，避免随仓库泄漏；
- 比较使用 `secrets.compare_digest`，不泄漏长度与时序信息；
- 未配置时 `api/main.py` 在启动阶段打印 WARNING，明确写出「当前无鉴权」，
  让运维在日志里就能看到，而不是靠读文档才知道。

> 该模型只能防「未授权访问」，不能防重放，也不做用户区分——定位是内网/单机部署的
> 最低门槛。需要多用户或审计时应在前面加反向代理。

#### 3.12.3 限流

`RateLimitMiddleware`（Starlette `BaseHTTPMiddleware`）在 `api/main.py` 装配，
配置项位于 `config.yaml` 的 `web` 段：

| 配置 | 默认值 | 说明 |
|------|--------|------|
| `rate_limit_max_requests` | `300` | 窗口内允许的请求数；`<= 0` 表示关闭 |
| `rate_limit_window_seconds` | `60.0` | 滑动窗口长度（秒） |
| `rate_limit_exempt_paths` | `/api/health`、`/docs`、`/redoc`、`/openapi.json` | 豁免路径（前缀匹配） |

超限返回 `429`，并带 `Retry-After` 与 `X-RateLimit-*` 响应头。

`RateLimiter` 实例在 `api/main.py` 模块级创建后**共享注入**中间件，而不是让中间件
自己 new 一个——这样测试可以用 `rate_limiter.reset()` 逐用例复位，否则用例之间会
互相污染计数。中间件顺序（洋葱模型，后注册的先执行）：

```
RequestTraceMiddleware → RateLimitMiddleware → RequestLoggingMiddleware → CORSMiddleware
```

限流排在日志之前，保证被限流的请求也会被日志记录。

回归测试见 `tests/integration/test_api_security.py`：覆盖「无关答案不得通过质量阈值」
「越界路径 403 / `..` 400」「限流已装配且 429 生效」「密钥开启后写操作 401、读操作仍匿名」，
共 18 例。

### 3.13 「比例 → 条数」的余数归属

五处「按占比切条数」统一走 `augmentor/allocation.py:largest_remainder()`：
`aggregator.aggregate_weighted`（逐源配额）、`dataset_ops._stratified_sample`（逐组配额）、
`dataset_ops.split`（三段）、`data_splitter._random_split`（三段）、
`data_splitter._stratified_split`（L38 起：先在全体上算三段目标，再逐组按「各段还欠多少」
摊派，每组一次调用；L39 起尾部的单条组改为一热批量摊派，不再逐组调用分配器，见 §3.14）。

为什么要收一处：`int(total * 比例)` 逐份下取整会**同时**少交和偏心。以下数字实测自本轮起点
`6b64d3842`：

- 3 源等权各 10 条时，`target_size` 取 1..30 有 **20 个**取值少交条数，`1` 和 `2` 直接交出
  **0 条**，而响应里的 `aggregated_count: 0` 与「三个源本来就是空的」同形；
- 4 组各 5 条要 7 条 → 交出 **4** 条（逐组 `int(1.75)`）；
- 7 条按 0.8/0.1/0.1 分割 → `(5, 0, 2)`：名义 10% 的 val 是 **0 条**，名义 10% 的 test
  实拿 **28.6%**；且 n ≤ 9 时 val 恒为 0。

口径保证：和 == `total`（`caps` 之和不足时交回诚实的短交）；`caps` 不绑定时每段与名义值
差 < 1 条；装不下的段把名额让给还有空间的段；**任何情况下和都不大于 `total`** ——
`minimum_each` 的准入读的是「各份保底**先被各自上限夹过**之后再求和，装不装得下 `total`」，
装不下时整条保底不生效、退回纯按权重摊派，不做「能塞几份塞几份」的尽力塞（L40 修法，
关闭 A49/A55，推导与实测见 §3.15）。旧判据 `total >= count` 在同一条款下会超发：契约合法域
穷举 399,594 例里 **24,889 例** `和 > total`，另有 **261 例**是「保底其实装得下、却被份数判据
误拒」。全仓唯一调用方 `dataset_ops._stratified_sample` 传 `minimum_each=1` 且 `caps` 是组大小
（必然 ≥ 1），那时保底之和恰为 `count × 1` ⇒ 新旧判据同一条式子，产品面答案逐例不变。

代价实测：典型 3–4 段单次 2.7–3.4 µs（被替掉的逐段下取整是 0.2 µs），可整除路径新旧同答。
**但份数才是成本变量**：`_stratified_sample` 的段数 = 分组键基数，而
`SampleConfig.stratify_key` 默认 `"instruction"` ⇒ 真实数据集上每组只有 1 条、段数 == 行数。
L36 换入时按「同值桩」口径（桩返回与真函数逐个相同的配额表）实测过这条自造回归：
6902 组（每键唯一）→ 500 条，端到端中位 4.81 → **7.83 ms（+62.7%，分配器本体 2.90 ms）**，
`aggregate_weighted(5 源 × 2000 → 5000)` 中位 4.33 → 4.36 ms（**+0.62%**，min 那一档出现
4.02 < 4.17 的**倒挂** ⇒ 只能说这一档「未测出代价」，不能说「已证可忽略」）。

**L37 还的就是这条 +62.7%**，改了两处而答案一字不改（差分见下面的表后说明）：一般路径减掉
冗余遍历（`list(caps)` 代替逐段下标、下取整与排序键同趟落下、排序不带 lambda），并给「每份
上限都是 1」这一最常见形状加一条**可证等价**的快路 `_unit_cap_quota()`。推导：上限 1 ⇒ 配额
只能是 0 或 1；一般路径里「本轮取满」⇔ 名义值 `exact >= 1`，这些段必然各占掉 `total` 里的一个
名额，没取满的那批 `exact < 1` 其小数部分就是 `exact` 本身，所以「补名额」的次序 == 按 `exact`
降序、平局按段序 ⇒ 选中集恰是「**按 `exact` 降序、平局按输入顺序**的前 `total` 份」，一次排序
取阈值即可，不必逐段挪名额。

两条口径是实测逼出来的，写下来免得被当成可有可无的细节：

- **排序键必须是 `exact`（权重乘完比例之后的浮点），不是原始权重**。第一版按权重排序，实测
  与一般路径**不等价**：权重取相邻 double 时 `total * 权重 / 权重和` 会把它们舍入成同一个
  `exact`，一般路径按平局处理（先到先得），按权重排序却挑后出现的那个；随机单上限形状里
  **2.3%**（4,673 / 200,000）答案不同。改成 `exact` 键后由用例
  `test_fast_path_compares_the_same_float_as_the_general_path` 钉住。
- **不能用 `sorted(..., reverse=True)[:total]` 取前 N 份**：稳定排序配 `reverse` 会把平局的
  输入次序**颠倒**，与「平局按输入顺序」直接矛盾。所以写成「先取阈值，再按输入顺序把等于
  阈值的那几份补到刚好 `total`」。
- 等权重是「全体 `exact` 相同 ⇒ 全体平局 ⇒ 前 `total` 份」的特例，可以在排序前短路。真实分层
  采样正落在这条支路上：`_stratified_sample` 传的 `weights` 与 `caps` **都是组大小**，上限全 1
  就意味着每组只有 1 条，于是权重必然全相等。

同进程 back-to-back 复测（绝对值落在与并行 agent 争抢的窗口里时只有比值可比；本轮用 L36 记录的
+62.7% 作对照，两个窗口分别重跑出 +64.8% 与 +67.0%，口径吻合）：

| 形状 | L36 版 | L37 版 |
| --- | --- | --- |
| 本体 6902 段（等权）→ 500 条 | 6.81 ms | **0.066 ms（1.0%）** |
| 本体 6902 段全拿 | 1.30 ms | **0.028 ms（2.2%）** |
| 本体 6902 段不等权 → 500 条 | 7.03 ms | 1.89 ms（27%） |
| 本体 2000 段上限参差 → 1000 | 2.08 ms | 1.38 ms（66%） |
| 本体 3–4 段 | 3.3–3.4 µs | 2.5–2.6 µs（77–79%） |
| 本体 3 段 split 形状 / 1e9 名额 | 6.57 / 10.52 µs | 4.20 / 6.70 µs（64%） |
| 端到端 `_stratified_sample` 6902 组 → 500（争抢窗口） | 相对同值桩地板 +64.8% | **+4.9%** |
| 端到端 `_stratified_sample` 6902 组 → 500（无争抢、15 对交替） | +67.0%（7.780 ms vs 地板 4.658） | **+2.5%（4.775 ms，新为旧的 61.8%）** |
| 端到端 `aggregate_weighted(5×2000→5000)` | 与地板同量级 | 与地板同量级（未测出代价） |

等价性不靠推理担保，靠逐例对答案：与 L36 版（即「一般路径 = 定义」的参照实现）在 **40,060 例**
差分矩阵（含 4 万随机例，快路形状 14,917 例）、**相邻 double 平局族 4,020 例**、**相邻浮点权重
2 万例**上全部同答，0 例不一致；端到端另有交替配对 A/B（15 对，剔除机器漂移）确认新旧交回同
一份 500 条。等权短路本身是**纯常数优化**：它与阈值路径可证同答，因此「摘掉短路」不会让任何
用例变红，只能由上面那张差分矩阵和实测数字担保 —— 这一条按 A48 的旧例如实标注，不冒充测试
覆盖。

L36 记录的第二档「1000 条 / 40 类 → 500 条（+11.8%）」这一轮**测不出方向**：两跑分别是旧的
103.6% 与 86.0%，而该档端到端只有 0.44–0.52 ms、地板 0.45–0.50 ms，差值整个埋在噪声带里。
写结论时只能说「40 段这一量级未测出代价」，不能说改善或恶化。

还没还完的部分：6902 组那一档的**地板本身**（争抢窗口 ~10.2 ms / 无争抢 4.66 ms）几乎全是
分组循环 + 逐组 `rng.sample(单条组, 1)`，分配器只占其中 2.5%。要再降得改接线端「配额 1 条时
直接取该组唯一一条」，但那会改掉 rng 的消耗次序 ⇒ 另立 `OPTIMIZATION_LOOP.md` 的 A50。

**分配轮数只与份数同阶**（一段被取满即冻结），所以 `target_size` 是 1e9 还是 1e6 都在微秒级
返回（1e9 名额实测 6.7 µs）——不要写成逐条挪名额，实测那种写法在 1e6 名额 / 偏斜 `caps`
上是 67 ms，1e5 名额 / 100 段上是 327 ms，而 `target_size` 只有下界没有上界，是调用方
可达的路径。

一处刻意保留的偏心，写下来免得被当成 bug：

1. **平局按输入顺序补**，因此等权时最后一段永远拿得最少（`largest_remainder(7, [.25]*4)`
   → `[2, 2, 2, 1]`）。选「可复现」而不是「轮流公平」，因为随机平局会让同一份输入两次
   跑出不同产物。

**L38 补上了第五处，但它的口径不是「把最大余数法在组内套用」**：`_stratified_split` 是
「每个类别内部再切三段」，与另外四处「一份 `total` 切成 N 段」不是同一个问题。组内套用本轮
分配器实测更糟（6 组 × 2 条：HEAD 逐组 `int()` 交出 `(6, 0, 6)`，test 占 50%；组内套最大
余数法交出 `(12, 0, 0)`，val 与 test 双双为空），因为每组的 `total` 只有 2 条，0.1 的名义值
0.2 条在组内永远取不到 1。真正的修法是**跨组对三段做全局对账**：

- 三段目标先算一次 `largest_remainder(len(items), ratios)`，于是分层与不分层**同答**同一组
  三段大小；逐组把「各段还欠多少条」当权重摊派。可证 `cells[k] <= targets[k]`（`exact_k` 只在
  `n == 剩余` 时等于 `targets_k`，其余情况严格更小，而下取整补名额不会越过它），所以列和必然
  逐次对平 —— 4,000 个随机算例（每次 1–12 组 × 五种比例）实测越界 0 次。
- 真实语料 6,902 条按 `instruction` 分层（6,531 组，组大小直方图 `{1: 6168, 2: 359, 3: 1,
  4: 2, 5: 1}` ⇒ **6,168 个单条组 + 363 个多行组**；L38 的日志与本文旧版都写成「371 组 2 条 +
  6,160 组 1 条」，L39 重新实测后订正）：HEAD 交出
  train 371 / val **0** / test 6531（test 名义 10% 实拿 94.6%，与目标差 `(-5151, -690, +5841)`），
  按 `output` 分层交出 `(4925, 142, 1835)`（与目标差 `(-597, -548, +1145)`，val 只有 82/916 个
  类别出过条目）；修后两种字段都交出 `(5522, 690, 690)`，val 覆盖的类别数升到 690（instruction）
  与 609（output），最大单元偏差 0.900。
- **组的处理顺序必须「组大小降序 + 同尺寸随机」**，两个条件各挡一种坏形状。最大单元偏差
  实测自真实语料（12 与 40 个种子同值，且复刻实现与 `DataSplitter` 逐组同答）：
  `instruction` 档现口径 **0.900**、全局随机序 3.000、升序 3.000、按出现顺序 0.900；
  `output` 档现口径 **0.900**、全局随机序 1.000（12 种子）→ 1.200（40 种子）、升序 1.000、
  按出现顺序 0.800。缩例 `[(100,1),(12,2),(1,3),(2,4),(1,5)]`（116 组 / 140 条，单条组在前）
  里现口径在 12/40/200 个种子上恒为 **0.900**，而全局随机序 2.200（12 种子）→ 3.000（40/200
  种子）、升序恒 3.000、出现序恒 3.000（这一档单条组在前，是缩例故意选的形状）。⇒ 决定偏差的
  是「大组先摊」而不是「随机一下」：随机只保证成员不沿组序成块，尺寸序保证零头只由 1 条的组吸收。
  反过来在真实 `instruction` 语料上出现序之所以也是 0.900，只因那份数据的大组恰好靠前，
  是数据的巧合而不是口径的保证。
  另一条纪律挡的是成员偏置：只按出现顺序摊派时，`instruction` 档 690 条验证集里有 **460** 条来自
  语料后 20% 区段（按分布应约 138 条），100 单条组的缩例里验证集恒为组序 71..98 那一批（换种子
  也不变）；现口径下真实语料 4 个种子的尾段落点数是 132–162 / 690，缩例 20 个种子里每个都有
  ≥ 5 / 10 条来自前 80% 位置，且 4 个种子的验证集成员互不相同。
  偏差上限 0.9 与 DP 精确最优在 10 个算例上逐个相同（差 0.0000）。
- **代价 A53 已由 L39 还掉**（见下节）：摊派是每组一次分配器调用（约 2.95 µs / 组），全量
  `instruction` 端到端 5.80 → **26.07 ms（×4.49）**，`output`（916 组）2.90 → 6.12 ms（×2.11），
  比值对 n/2n/3n 稳定 ⇒ 与组数线性；不分层路径 0 代价（同批实测 0.848 ms）。这条路径无 CLI/API
  入口（`DataSplitter` 是 `__all__` 导出的公开 SDK 类），故按 L36→L37 的先例：本轮修对答案，
  代价下一轮还 —— L39 还完后同一落点回到 **10.217 / 5.149 ms**，仍留 +16.6% / +37.7% 的残留。

### 3.14 L39：把 A53 的摊派代价还掉（三处可证等价的加速）

L38 把「逐组 `int()`」换成「逐组一次最大余数法」，代价是每组一次分配器调用。L39 不改变任何
返回值，只把这条路径上**能被证明为多余的计算**拿掉。阶段分解（真实 `instruction` 语料，7 次
独立进程取最小值，复刻实现与真实现逐元素同答）：分组 0.825 / 组序+三段目标 1.400 /
**逐组摊派 15.186 ms（占 70%）** / 切片+清单 3.510 / 末段三次洗牌 0.848，端到端 25.018 ms；
顺带实测 `logger.info` 打印整个分布字典只花 0.123 ms —— 也就是说日志不是瓶颈，摊派才是。

三处加速：

- **C1 `_no_cap_quota()`**（`allocation.py`）：`caps is None and not minimum_each` 时，一般路径
  可证**只需要一轮**：`exact = remaining * share / weight_sum ≤ remaining ≤ total`，所以没有任何
  段会取满而被冻结，也就不存在「解冻后重分」的第二轮；同时 `left > 0 ⇒ 所有 floor < total`、
  `left < count`。因此直接一轮算完，不必进 `while`。派发处的 `if caps is None: / elif
  caps.count(1) == count:` 是嵌套而不是并列——写平会让 `minimum_each=1, caps=None` 在
  `None.count` 上崩，差分矩阵抓到了这个。
- **C2 单条组尾部一热摊派**（`data_splitter.py`）：组大小降序之后，尾部全是 1 条的组，而
  `largest_remainder(1, targets)` 可证等于「在 `targets` 最大处放 1」的下标 one-hot（平局取最小
  下标，与最大余数法的「平局按输入顺序」同一条纪律）。于是 6,168 个单条组不再逐个调用分配器，
  改成一次比较 + 手工维护 `targets`。**前提条件**是 `sum(targets) == 未处理组条数之和` 且
  `targets` 非负；负的 `targets` 会让 one-hot 与分配器答案分岔（实测过，所以这里的不变式由
  前面「多行组先摊」保证，不是默认成立）。
- **C3 单元素组不洗牌**：CPython 3.14 的 `shuffle` 主循环是 `reversed(range(1, len(x)))`，
  `len == 1` 时一次都不迭代 ⇒ 不消耗随机数。两个独立的 `Random(1)` 流实测同答（首个
  `random()` 都是 0.13436424411240122），所以跳过它与调用它逐元素同答，含输出顺序。

三方对比（L37 / L38 / L39，同进程 back-to-back，绝对值只在同窗口内比）：

| 落点 | L37 | L38（欠账） | L39（还账） |
| --- | --- | --- | --- |
| `_stratified_split` instruction 6,902 条 | 8.765 ms | 25.299 ms | **10.217 ms（×0.404）** |
| `_stratified_split` output 916 组 | 3.739 ms | 6.033 ms | **5.149 ms（×0.853）** |
| 全单条组 2000 组 | — | 基准 | ×0.334 |
| 无单条组 2100 组 | — | 基准 | ×0.806 |
| n/2n/3n 缩放比 | — | 线性 | 线性（×0.332 / 0.343 / 0.353） |
| 分配器本体 3 / 20 / 2000 段 | 3.632 / 11.193 / 1166.8 µs | 同 L37 | **1.804 / 5.887 / 593.9 µs** |

两处**没有**被还掉的账，如实记下：一是相对 L38 的干净基线 L37 仍有 **+16.6%（instruction）/
+37.7%（output）** 的残留，那是 L38 语义修正本身的必要成本（多行组确实要调用分配器），不是
可以再省掉的开销；二是**带 `caps` 的路径慢了 ×1.039**（3.709 → 3.854 µs），因为派发处多了一次
`caps is None` 判断——这一档不落在任何真实调用点上，但它是本轮唯一变慢了的地方。

候选里被实测**否掉**的：组序排序改桶排/计数排（`instruction` 16.213 vs 现口径 15.119 ms，更慢；
`output` 4.477 vs 4.806，边缘收益不值得换一份新语义），以及把 `targets` 维护改成 `heapq`
（在 6,168 次单元素比较上被常数项吃掉）。

等价性证据：分配器差分 **120,000 例 0 不一致**；划分器与 L38 已提交 blob 在 **60 个（形状 ×
种子）组合上逐元素同答，含输出顺序**；17 种注入劣化全部可红（除模式⑧是**可证等价变异体**：
`total <= 0` 改成 `< 0` 时 `total == 0` 在两条路径都落下全 0，故与模式⑬「删掉守卫」配对记录）。
另有两处「劣化 0 红 = 测试漏钉」当场补掉：one-hot 平局取最小下标（补
`test_singleton_tie_goes_to_the_lowest_index_segment`）与单元素组不洗牌（补
`test_single_item_groups_are_not_shuffled`）—— 前者暴露了一条通用纪律：**规则被内联复制时，
钉住原语的测试不会为副本变红**，必须为副本单独写定向用例。

### 3.15 L40：`minimum_each` 的准入判据改读「保底之和」（关闭 A49/A55，顺手收 A51）

**缺陷形状**：一般路径的保底预分配写成 `if minimum_each and total >= count:` —— 判据读的是
「名额盖不盖得住**份数**」，而正确口径是「盖得住**保底之和**」。两者之间有一段窗口会超发，
另一段会误拒。穷举契约合法域（份数 1–5、权重 1/2/3、`caps` 为 None 或取自 {0,1,2,5}、
`total` 0–12、`minimum_each` 0–5）共 **399,594 例**：

- 旧超发（`sum(配额) > total`）**24,889 例**，例如 `lr(5, [1,1,1], minimum_each=2)` 交
  `[2,2,2]`（和 6）、`lr(3, [1,1], minimum_each=5)` 交 `[3,3]`（和 6，多出 3 条）；
- 旧误拒（保底其实装得下，却一份都没给）**261 例**，形状都是「份数里含上限 0 的段」：
  `lr(4, [1,1,1,1,2], caps=[0,0,0,2,5], minimum_each=2)` 的保底之和是 4 而不是 5；
- 新旧不同共 **25,192 = 24,889 + 261 + 42**。剩下那 42 例全部是 `minimum_each=1` 且含零上限段
  的**纯舍入次序差**（`[0,0,0,1,3]` → `[0,0,0,2,2]`）：新判据先落下保底再摊剩余，摊派基数变小。
- ⇒ `minimum_each <= 1` 的分歧 253 例**全部含上限 0 的段**（不含零上限的 0 例），而在库唯一调用
  方传的 `caps` 是组大小（必然 ≥ 1），所以产品面不可达；这一点不是推理，是穷举出来的计数。

**修法**：先把每份保底按各自上限夹一遍（`floor = [min(minimum_each, lim)]`）再求和，
`floor_total <= total` 才准入，准入后 `limits` 与 `remaining` 同步扣减；装不下时整条退回
按权重摊派。**两个边界都是实测选定的**：

- `<=` 而不是 `<`：域内有 **213,010 例** `floor_total == total`，准与不准给出不同答案，
  代表算例 `lr(4, [1,2], minimum_each=2)` 给 `[2,2]` 而 `<` 给 `[1,3]`；「恰好装得下」当然
  是装得下，故用例把这两行边界（另加 `caps` 全等于保底的那档）钉成参数化定点。
- 「整条不生效」而不是「尽力塞」：`lr(9, [5,1,1], minimum_each=4)` 交 `[7,1,1]`；尽力塞会交
  `[4,4,1]`，那等于让保底反过来吞掉权重 —— `minimum_each` 是下界，不是第二份权重。

**A51 两条死守卫同时收口**（`_no_cap_quota` 的 `if left:` + 内层 `break` 双保险、
`_unit_cap_quota` 的外层 `if left:`）。可证同答：`cut` 取的是第 `total` 大的 `exact`，故
「严格大于 `cut`」的段数 `m <= total - 1` ⇒ `left >= 1` 恒成立；补发段每份都还有余量 ⇒
「取前 `left` 个键」与「逐段补到 `left` 归零」是同一批段。证据不是推理而是差分：新实现 vs
「HEAD + 只换取保底判据」的参照在 **399,594 例上差 0**，另外注入模式 13（把外层 `if left:`
原样加回来）**0 红** —— 这条 0 红是重言式的证据，不是测试漏钉，与 ⑫ 类「等价劣化」同批记录。

**代价**（min-of-9、同进程 back-to-back，L39 → L40，µs）：被拒的分支现在要多走一遍
`min`-夹求和，所以只在 `minimum_each >= 1` 的多段形状上看得见 —— 3 段 me=1 `0.824 → 0.874`
（×1.061）、20 段 me=1 带 caps `12.047 → 14.006`（×1.163）、2000 段 me=1 `caps=None`
`1369.5 → 1451.8`（×1.060）、2000 段 caps 递增 me=2 `1396.4 → 1479.9`（×1.060）；
真实热形状（2000 段 caps 全 1 me=1，走 `_unit_cap_quota`）**18.182 → 16.560（×0.911）**，
无保底形状 ×0.97–0.94（即未变）。产品面端到端（真实 6,902 条语料，min-of-7）六种形状
（`sample` 的 instruction/output × size 500/2000、`split` 的 instruction/output）**逐条同答**，
比值 0.920 / 1.003 / 1.007 / 0.996 / 1.041 / 1.012 —— 最小那一档含进程 warm-up，不采信为收益；
其余五档都在 ±4% 内，故本轮的结论是「语义修复在产品面上答对且未测出实质代价，代价落在
SDK 直调的 `minimum_each >= 2` 与多段形状上」。

**测试与注入**：该文件收集 **107 → 129**（`def test_` 46 → 52；新类 `TestMinimumEachAdmission`
13 个参数化定点 + 4 条纪律用例，快路准入用例加 4 个 `minimum_each >= 2` 参数行，
外加 1 条接线护栏），全量 **4628 passed / 2 skipped**（74.64 s，coverage 总计 **99.01%**；
`allocation.py` 首次被 `--cov-report=term-missing:skip-covered`（`pytest.ini:12`）摘出 term 表 ——
表里不再出现该文件，而 coverage.xml 给出 line-rate 1.0 / branch-rate 1.0，这就是 A51 关闭的痕迹）。注入 13 项：**12 项可红**（并集
**117 条**节点 id / 去参数化 40 个用例名），**1 项可证等价**（模式 13）；模式 01「整体回退
L39 blob」与模式 02「只退回旧判据」红**逐条同名同 12 条** ⇒ 本轮除准入之外的所有改动
（两处死守卫 + 注释）答案不变，这是差分计数之外的第二条独立证据。
**一处判别力缺口当场补掉**：模式 12（`_unit_cap_quota` 的准入多读一次 `minimum_each`）起初
0 红 —— 它不是等价变异体而是**测试漏钉**，因为真实调用点 `dataset_ops.py:242` 正是
`caps=组大小, minimum_each=1`，多读一次会把全单条组的真实形状从「一次比较」踢回一般路径。
于是补 `test_unit_cap_request_uses_the_fast_path_even_with_a_guarantee`（与 L39 的
`test_uncapped_request_uses_the_fast_path` 同一形状的反向接线护栏），复跑后该模式恰红在它身上。


### 3.16 L41：让 `DatasetOperations.split` 消费 `stratify` / `stratify_key`（关闭 A54）

`SplitConfig` 自 3.0 起就带着 `stratify` 与 `stratify_key` 两个字段，但 `split()` 从头读到尾也不看它们
（全仓非测试代码 0 个消费点）——「参数存在但代码不消费」是 A46 那一族的第三处，也是 L38 把分层划分口径修对之后
**唯一还接不到修复的产品内路径**。本轮接线，接法与三条边界都记在这里。

**接法：委托，不复制**。`split()` 在 `config.stratify` 为真时走 `_stratified()`，后者只调用
`DataSplitter(*config.ratios, seed=config.seed, stratify_field=config.stratify_key).split(items)`。
分层摊派在 §3.13 / §3.14 刚改过口径、刚还完代价，若这里再写一份，下一次口径变更就要改两处（咽喉原则，
同 §3.4 / §3.8 的取舍）。`stratify=False` 的默认路径逐行保持 L40 提交态形状，本轮未动它一个判断。

**三条口径边界（都由实测拍）**：

1. `seed` 管可复现，`shuffle` 只管段内顺序。分层的组序随机是 §3.13 的纪律（尺寸降序 + 同尺寸随机），
   所以「可复现」只能由 `seed` 提供；`shuffle=False` 时三段各按语料原始相对次序回排，成员集合与
   `shuffle=True` 完全相同。把 `shuffle` 理解成「能不能复现」会得出相反结论。
2. `stratify_key` 为空（`""` / `None`）**报错** `DataValidationError("分层分割需要非空的 stratify_key")`，
   不静默退回不分层。静默退化正是本轮修掉的缺陷形状，拿它当兜底等于换个入口把同一个 bug 再引进来。
3. 比例校验沿用被委托方的严口径（`1e-6` 且不许负数），于是与默认路径（±0.01 且不查负数）出现**可接受域分歧**：
   `(1.5, −0.5, 0.0)` 默认路径照跑出 `30/0/0`、分层路径报「比例不能为负数」；`(0.8, 0.1, 0.105)` 默认给
   `24/3/3`、分层报「比例之和必须为 1.0，实际为 1.0050」。两侧现状各钉一条用例；统一方向要碰默认侧的
   破坏性变更，记 **A57** 交用户拍。

**收益与代价（真实 `train_data.json` 6,902 条）**：

| 形状 | 最大单元偏差（不分层 → 分层） | `val` 覆盖类数（最差 → 恒） |
| --- | --- | --- |
| `instruction`（6,531 组） | 2.000 → **0.900** | 685 → **690** |
| `output`（916 组） | 6.500 → **0.900** | 448 → **609** |

比例 `(0.8, 0.1, 0.1)`、种子 0–11 取最大；不分层侧的读数**随种子数变差**（同样形状只跑 3 个种子读到
1.800 / 5.200）⇒ 写「最差情形」必须标种子基数。三段条数两侧都是 `(5522, 690, 690)`，
所以本轮改善的不是总量分配，而是「每个类别内部是否拿到名义份额、验证集是否漏类」。

代价（同进程 back-to-back min-of-9）：默认路径 **×1.006 / ×0.992**（噪声，未变贵）；分层档相对直接调
`DataSplitter` **0.987–1.018**（委托层白付 ≈ 0）；`shuffle=False` 的保序重排 **×1.296–1.595（+3.2 ~ +3.9 ms）**，
记 **A58**（该括注自 L42 起已是假陈述 —— CLI `--stratify --no-shuffle` 与请求体 `stratify=true, shuffle=false` 都会付这笔钱；**L43 就地校正并结案**：阶段级实测 3.175 → 2.708 ms （**×0.853**，min 与中位同向），形态见 §3.18）。

**可达面**：SDK 构造 `SplitConfig(stratify=True, stratify_key=...)`，以及文件级便捷函数
`split_dataset(input_path, output_dir, stratify=True, stratify_key=...)`（`**kwargs` 直通 `SplitConfig`）。
**L42 更新（本段末句已成假陈述，就地校正）**：HTTP `POST /api/dataset/split` 与 CLI `split` 现在
也能传到了 —— 请求体加了 `stratify` / `stratify_key` / `shuffle`，CLI 加了 `--stratify` /
`--stratify-key` / `--no-shuffle`，A56 已关闭，细节与代价见 §3.17。另注意 `augmentor/__init__.py`
导出的 `split_dataset` 是 `data_splitter` 那个内存版，不是本节说的文件版（**A59**）。

**证据**：注入 8 项劣化红数 20/20/4/2/11/10/2/5，红例并集 29 条 = 本轮新增 29 行参数化用例每行至少被一种劣化杀掉；
「①整体回退」与「②只把开关改成恒假」的红逐条同名，据此判定接线之外的结构改动答案不变。
`dataset_ops.py` 216 语句 0 missed、分支 60/60。

### 3.17 L42：把分层分割与分层采样带到 HTTP / CLI 契约上（关闭 A56）

L41 把 `stratify` / `stratify_key` 从死旋钮接成真实现，但只到 SDK 面为止：`SplitRequest`
只有六个字段，路由构造 `SplitConfig` 时不传分层参数，CLI `split` 同样只传 `ratios` 与 `seed`
⇒ 用户在产品面上仍然「传了也没用」。本轮把入口接上，并顺手补上采样侧的同族漏洞。

**接法：只加请求字段，响应形态一字不改**。三条理由都在实测上：
①本组「写盘变换类只回写到哪、写了多少」是成文约定（`docs/API.md` dataset 分组说明）；
②真实 6,902 条按 `instruction` 分层时 `stratify_distribution` 有 **6,531 个键、紧凑 JSON
404,362 字节**（缩进版 567,638 字节），是四计数响应（79 字节）的 **5,119 倍**、相当于整份语料
序列化的 29%；按 `output` 分层是 916 键 / 135,752 字节 —— 回传它等于把一个可选诊断变成每次请求
必付的响应体；
③老客户端的兼容只能靠「新字段全部有默认值」，而默认值必须等于 L42 之前的行为，这条已由
`test_new_knobs_default_to_pre_l42_behavior` 钉成「三段产物逐条同答」（比对解析后的条目与计数，不是文件字节）。

**面**：HTTP `POST /api/dataset/split` 收 `stratify`（默认 `false`）/ `stratify_key`（默认
`"instruction"`）/ `shuffle`（默认 `true`）；`POST /api/dataset/sample` 收 `stratify_key`（默认
`"instruction"`，只在 `method="stratified"` 生效）。CLI 对应 `--stratify`、`--stratify-key`、
`--no-shuffle`。

**采样侧的同族缺陷（本轮一并修）**：`_stratified_sample` 不校验空键，`item.get("", "")` 对每条
都交出 `""` ⇒ 20 条 4 类语料取 8 条，正常键恒为 `2/2/2/2`、空键交出 `5/2/1` 的随机形状（探针
Temp `l42b.py`）—— 与 A54「旋钮不被读就等于没有」同形，且 L42 之前 `stratify_key` 在产品面不可达，
所以这条只在 SDK 面暴露。判据与 §3.16 边界 ② 同一条：`DataValidationError("分层采样需要非空的 stratify_key")`，
400 / CLI exit 1，且**中止在落盘之前**（实测：判据本身 0.0073 ms，整次请求只付一次读盘 10.72 ms vs
成功档 19.307 ms，min-of-9 同进程）。判据只管分层支路，`random` / `systematic` 不读键、空键照旧可用。

**`shuffle` 在两条支路上口径不同（新事实，记 A60）**：分层支路的 `shuffle=False` 只把每段成员按
输入原始相对次序回排，成员与条数都不动、`seed` 仍决定谁进哪段；默认支路是「先整份打乱再按位置切片」，
所以 `False` 会连同成员一起改变，且三段恒为输入的前 50%/25%/25% —— **此时 `seed` 完全空转**
（两个不同 seed 的 `shuffle=False` 产物逐条同答，`test_shuffle_false_on_default_path_freezes_the_seed`
钉住）。这不是本轮引入的行为，但接上旋钮就必须把它钉成事实，免得日后被当成回归；统一口径交 A60。

**代价（同进程 back-to-back min-of-9，真实 6,902 条 × 1 种子 seed 7，比例 0.8/0.1/0.1）**：
请求模型构造 `SplitRequest` 省略新字段 0.0011 ms → 显式默认值 0.0013 ms（**×1.1818**，绝对 +0.2 µs）、
`SampleRequest` **×1.1111** ⇒ 字段本身是微秒级，可忽略。端到端 `split_file`：默认档 43.325 ms、
显式默认值 44.331 ms（**×1.0232**，+1.006 ms，与 L41 同档的 ×1.006 一致，属噪声）、分层档 53.826 ms
（**×1.2424**，+10.501 ms）、分层 + `shuffle=False` 55.292 ms（**×1.2762**，较分层档 +1.466 ms）—— **L43 更新**：这 +1.466 ms 的分母含读写，且保序阶段本身已按 §3.18 改掉一部分（内存级阶段 ×0.853）；A58 已结案。
最后这一档是 **A58 那笔保序代价第一次由产品面触发** —— L41 时它只有 SDK 显式传参才付，现在
CLI `--no-shuffle --stratify` 或请求体 `shuffle=false` 就会付。绝对值跨进程不可比（首轮探针曾读到
分层 + 保序 55.851 ms / 成功档 34.689 ms，与同进程重测差一截，故结论只取比值）。

**证据**：新增 19 个 `def test_`（unit 3 / CLI 7 / API 9）、参数化后 **27** 行；注入 5 项劣化红数
**4/2/4/2/4**，逐条与预定 node-id 同名（每次注入只跑该劣化对应的节点，红数 == 名单长度），
还原后 5 个文件 sha 逐字节一致。全量 **4684 passed / 2 skipped**（75.87 s，覆盖率 **99.02%**），
计数 **4657 + 27 = 4684 精确对上**，0 处既有断言被改写（三个测试文件 `−0` 行）。

### 3.18 L43：还 A58 —— 保序重排从「三段各整份走查」改成「一次位姿表 + 段内状态键排序」

A58 是 L41 接线时自带的一笔代价（`shuffle=False` 时每段各整份走查一遍语料，三段 = 三次 O(n)），L42 又把它从
「只有 SDK 显式传参才付」变成「产品面会付」。本轮还掉一部分，**并在动手前先把账本自己记的降法差分否掉了**。

**账本候选是错的**：「先建 `id → 原索引` 字典、再对段内 `sorted(key=...)`」在**同一对象被引用多次**的语料上不
等价 —— 反例：语料 `A@{0,5} + B@{1}`、段 `[A, A, B]` 的正解是 `[A, B, A]`，用「首次出现位置」当键交出
`[A, A, B]`（实测 **120/120 全分歧**，分母 = 60 组对抗语料 × 2 个分层字段的三段比较数）；把字典换成 set 过
滤同样 120/120 错。可证的只有**带状态**的那把键：某对象的第 k 次出现取它第 k 个位置。落地形态 ——
`_position_table(items)` 一次走查建 `id -> 下标 | 升序下标列表`，`_in_original_order(positions, seg)` 只做段
内排序；真实语料（`json.load` 每条都是独立对象）表值恒为整数。

**三条口径边界（全由实测拍）**：

1. **等价性三方供证**：400 trial 随机对抗语料（重复率 0 / 0.1 / 0.3 / 0.5）里 145 个合法段 **0 分歧**；真实
   6,902 条三段逐条同答、段大小 `(5522, 690, 690)` 不变；`test_duplicate_reference_corpus_keeps_head_semantics[0..3]`
   把旧口径当 oracle 钉进用例（注入 ③④⑦⑨ 都能让它红 ⇒ 不是死用例）。
2. **`id()` 只在对象存活期间唯一**：旧实现拿着 `items` 走查，顺带保证语料全程存活；换成表以后表里只有整数，
   前提落到调用方（`_stratified` 里 `items` 是入参，天然满足）。这条不是理论担忧 —— 写用例时把语料当临时对
   象传进去、放掉之后 `ghost` 复用同一地址会**假命中**这张表（实测）。前提已写进 docstring。
3. **外来成员从「静默少一条」变成 `KeyError`**：不是本轮加的检查，是查表的天然结果 —— 但行为变了就得钉
   （`test_member_not_in_corpus_fails_loud`）。

**被实测否掉的三条路（写下来免得下一轮重走）**：①「语料无重复引用 ⇒ 用无状态键」的快路只比单路径快 **1.52 倍**
（`2.739 / 1.801`，min `×0.6573` / 中位 `×0.6602` 同向），未过开工前立的 ≥2 倍门槛 ⇒ 记 **A61** 不采纳（那
条门槛之所以看着「已过」，是因为我把 `0.6139 / 0.3528 = 1.74` 口算成了 3.5 倍 —— **比值必须写算式**）；
②「整份语料只走一次、按段落袋」的纯落袋最快（**×0.538**）但在重复引用上是**另一套语义**（**A62**，要它得先
口语义轮）；③为复刻 HEAD 语义而写的「按名次发袋」单次走查确实等价（真实 + 对抗逐条同答），但 **2.06 倍于单路
径**（min；中位 3.05 倍）—— 每 id 一份段认领表加一张名次表的常数吃掉了省下的排序，**形状更优不等于更快**。

**代价（真实 6,902 条、内存级 `DatasetOperations.split`、0.8/0.1/0.1、seed 7；同进程交错，min 与中位双口径）**：
保序阶段本身 3.175 → **2.708 ms（×0.8531 min / ×0.8537 中位，两口径同向）**、占端到端 **24.0% → 21.3%**；
端到端 min 13.230 → 12.693 ms（×0.9595）**而中位 13.760 → 14.033 ms（×1.0198，方向相反）**，同批极差 5.4–5.9 ms ⇒ **端到端这一档不可判定**，本轮只采信阶段级，不宣称端到端变快。打乱档 9.836 ms、不分层档 0.886 ms 未触及 ⇒ 剩下的贵处在分层本体（≈ 8.95 ms）而不是保序。

**证据**：注入 9 项 = 8 红 / 1 等价绿（⑧「位置全体 +1」是可证等价变异体，0 红，如实记），预期红名单逐条命中 0 落空，还原后按原始字节核对 sha 一致；新增 4 个 `def test_` = 参数化后 8 行，全量 **4692 passed / 2 skipped**
（**4684 + 8 精确对上**，总计 99.02%），0 处既有断言改写，`dataset_ops.py` 语句与分支 0 missed。

### 3.19 L44：重试的计数旋钮并进 `require_count` 一族（`retry.py` 首次进账）

`augmentor/retry.py` 是这条链路上唯一从没被这本账碰过的模块。本轮不改算法，只把它两个
**整数计数旋钮**的越界读法统一成 §6 已经成文的那一条口径。

**三个实测症状**（Temp 探针 `l44_probe.py`，全部在动手前复现）：

1. `with_retries(max_retries=-1)` —— `range(0, 0)` 为空，**`func` 一次都没被调用**
   （实测 `func_calls=0`），循环结束后 `raise last_exc` 就是 `raise None`，
   抛 `TypeError: exceptions must derive from BaseException`。这条报文与真实原因
   （参数写错）没有任何关系，而且请求从未发出。
2. `compute_delay(attempt=0)` 给基础的 **0.5 倍**、`-1` 给 0.25、`-3` 给 0.0625 ——
   `attempt - 1` 是指数，越界不报错而是**静默把退避算成 2 的负次幂**，于是存在
   「第 0 次重试比第 1 次等得更短」这种没有对应现实的档位。
3. 同一个名字在两层是两种口径：`generate()` 的 `max_retries` 是**总尝试次数**，
   `with_retries()` 的是**额外重试次数**。这本身是有意保留的既有语义（改它属破坏性），
   但 `generate` 用 `max(1, …)` 把负数**静默夹成 1 次**，等于用一层 clamp 掩护口径混淆。

**边界由实测拍，不顺手收紧**：`max_retries` 下界取 **0** 而不是 1 —— 0 在两层的读法
一致（只调用一次、不重试），且是一条真实请求，判掉它属过度收紧；`attempt` 下界取 **1**
（「已失败次数」从 1 起算）。`bool` 与数字串一并拒绝（`True` 会被 `range` 静默读成
「重试 1 次」）。**浮点等待时长（`base_delay` / `factor` / `max_delay` / `Retry-After`）
是另一半口径，本轮不做**：实测 `base_delay=-5` 时 `sleeper` 收到 `[0.0, 0.0]`（
`max(0.0, delay)` 把负退避整条清零）、`factor=-2` 时四档退避是 `[1.0, 0.0, 4.0, 0.0]`
（隔一次完全不等待）—— 它需要一条 `require_seconds` 式的浮点判据，而不是把浮点塞进
按口径拒绝非整数的 `require_count`。（**L45 已补这条判据并接上模型后端，见 §3.20；`retry.py`
内部三参与 `parse_retry_after` 仍未接，账本 A67 残余。**）

**代价（同进程 7 轮交错，min + 中位两口径同向）**：`require_count` 孤立 68.5 ns；
`with_retries` 成功档 476.2 → **556.8 ns（×1.17）**、`compute_delay` 123.3 →
**205.8 ns（×1.67）**。比值在这里没有决策意义 —— 前者每次 `generate()` 只付一次、
包的是网络往返；后者每次重试只付一次、其后紧跟 ≥ `base_delay`（默认 1 s）的 sleep，
**+82 ns 是那一秒的 8.2e-8 倍**。与 §3.18 的「形状更优不等于更快」是同一条纪律的两面：
**不为纳秒级的绝对量牺牲入参校验**。

**顺带挖出的三笔「契约面有旋钮、实现面不消费」**（与 §3.16 的 A54 同构，见账本 A64–A66）：
~~`ModelConfig.max_retries` / `retry_delay`~~ **`AugmentationConfig.max_retries` / `retry_delay`**（L45 校正类名归属：`ModelConfig` 在 `config.py:15` 且没有重试字段，两个旋钮属于 `:28` 的 `AugmentationConfig`）全仓 0 个消费者，而 `models/base.py` 另有一副写死的
`Retry(total=3)` 在~~传输层独立生效（⇒ 一次 `generate()` 最坏请求数上界 `attempts × (1+3)`，
此数按代码推得、未实测）~~ **传输层从不生效（L45 实测校正，见 §3.20）**；`compute_delay` 的 `jitter` / `rng` 没有任何调用路径能传进去
（`with_retries` 不接也不透传）⇒ 真实重试链上抖动恒为 0；`RetryStats` 回到 `generate`
后被丢进 `_stats` ⇒「实际试了几次、总共等了多久」在产品面无出口。

**证据**：注入 3 项（逐个摘掉三处判据）红数 **2 / 1 / 1**，三档越界各配一条
「合法档一字不动」的对照用例 ⇒ 红数既不为 0 也不覆盖全集；还原后两份实现按原始字节
核对 sha 一致。新增 9 条用例（含 1 条 ×3 参数化），全量 **4701 passed / 2 skipped**
（**4692 + 9 精确对上**，总计 99.02%），**0 处既有断言改写**。

### 3.20 L45：`augmentation.max_retries` / `retry_delay` 真正接上模型调用（关闭 A64）

上一节的第三笔「契约面有旋钮、实现面不消费」在本轮清偿。两个旋钮声明在
`config.py:28` 的 `class AugmentationConfig`（字段 `:33` / `:34`，默认值同时进 `:308`
的默认配置字典），L44 之前**全仓非测试代码 0 个消费者** —— 改配置文件里的重试次数与
间隔，行为一个字都不变。

**接线是三层各加一段，没有新抽象**：

```
config.augmentation.max_retries / retry_delay
  → AugmentorPipeline._init_components            （pipeline.py，产品面唯一调用点）
  → create_model_backend(default_attempts=…, default_retry_delay=…)
  → ModelBackend.__init__                          （5 个子类各自加两个形参并 super() 透传）
  → generate() 的缺省档：attempts = max(1, max_retries if max_retries is not None else self._default_attempts)
```

优先级保持显式入参赢过后端默认（`test_explicit_arg_beats_backend_default`），类常量
`_DEFAULT_ATTEMPTS = 3` / `_DEFAULT_RETRY_DELAY = 1.0` 取代原先散在 `generate()` 里的字面量。
**已发布配置的数值行为不变**：`config.yaml` 解析出 3 / 1.0，与旧硬编码同值，只有设过非默认值的
用户会看到变化 —— 而那正是缺陷的症状面。

**重试口径由实测拍定，不是读代码拍的**（这条是本轮最要紧的更正）。L44 曾在账本与本文档写下
「传输层那副 `Retry(total=3)` 独立生效 ⇒ 一次 `generate()` 最坏 HTTP 请求数上界
`attempts × (1+3)` = 默认档 12」，本轮用本地回环上的真实 `HTTPServer` 数服务端收到的请求
（requests 2.33.1 / urllib3 2.6.3）：

| 形状 | 服务端收到的请求数 |
|------|--------------------|
| `generate(max_retries=3)` | **3** |
| `generate(max_retries=0)` / `(1)` | **1** / **1** |
| 同一 URL 换成 GET | **4**（1 + 传输层 3 次） |
| `POST` + 显式 `Retry(allowed_methods={'POST'})` | **4** |

根因：`Retry.DEFAULT_ALLOWED_METHODS` 是 `frozenset({'DELETE','GET','HEAD','OPTIONS','PUT','TRACE'})`，
**不含 POST**，而 5 个后端的全部 6 处 `session.post(...)` 都是 POST ⇒ 那副适配器只带来了连接池，
重试在产品路径上从未起跑过，最坏请求数就是 `attempts` 本身。**结论：全仓重试口径唯一 =
`retry.with_retries` + `classify_error` 这一层**（`models/base.py:_get_session` 的注释里写死了这句话）。
要不要把那副死适配器改成真生效（加 `allowed_methods`）是另一笔决策，风险是与 `with_retries` 叠乘
（`attempts=3` 时最坏 3×4 次真实请求、两副退避各算各的），已单立账本 **A69** 待拍，本轮不动它。

> **L46 就地更新本节**：上表是**传输层 `Retry` 还在时**的实测快照，其中 GET 那一档随 L46 的删除
> 变成 1 次（POST 各档不变）；「本轮不动它」也已被 §3.21 的 A69 关闭取代。本节其余结论仍然成立。

**新判据 `validation.require_seconds(name, value, minimum=0.0)`**：拒 `bool`、拒字符串、拒 NaN、
拒负值，接受 int 与 `+inf`（`+inf` 由 `compute_delay` 尾部的 `min(max_delay, …)` 夹住，语义有界）。
它必须存在的原因是实测：`compute_delay` 对坏值**不抛异常，只换档** —— `base_delay=NaN` → `30.0`
（`30.0 < nan` 为假 ⇒ `min()` 交出第一个参数）、`max_delay=NaN` → `0.0`、`base_delay=-5.0` → `0.0`。
这三档被写成一条参数化反向护栏用例，断言的正是「没有判据时会发生什么」，摘掉判据时红例自带病灶说明。
`retry.py` 侧的 `with_retries(base_delay/factor/max_delay)` 与 `parse_retry_after` 仍未接判据（账本 A67 残余）。

**两处位置纪律，与「判参先于副作用」同族**：① `ModelBackend.__init__` 的两道判据排在
`_build_response_cache()` **之前** —— `DiskCache.__init__` 会 mkdir，坏档位不该留下建好了没人用的目录；
② `pipeline._init_components` 的判据排在既有的降级 `try/except` **外面** —— 那个 `try` 的语义是
「模型没配好就退化为无模型模式」，若让配置笔误从 `try` 内抛出，症状会被读成「后端不可用」而真因是参数越界。

**旋钮可设 ⇒ 必带校验规格，而新规格必须先测误报率**。给 `config_validator` 的 KNOWN_FIELDS 加
`augmentation.max_retries`（`int`，0–20）与 `augmentation.retry_delay`（`float`，0.0–60.0）之后，
`validate-config` 立刻把**合法** YAML `retry_delay: 1` 判成「类型错误: 期望 float, 实际 int」。
修法是把内联类型判定对齐本仓 `_validate_float` 已有的宽口径（`float` 规格接受 int），并补 NaN 拒绝，
使**静态校验面与运行时判据同判** —— 这条同判由 `test_validator_and_runtime_use_the_same_verdict`
用同一批值参数化永久钉住。上界不是形式主义：`max_retries: 1000000` 在最坏情形是 10⁶ × 30 s ≈ 347 天
的单次 `generate()` 等待。顺带查出 `_validators` 表全仓 0 读取点（账本 **A70**）。

**代价（只报绝对量，承 §3.19 那条「比值不得放大绝对值」）**：`require_seconds(name, None)` 27.3 ns、
`require_seconds(name, 1.0)` 109.6 ns，对照 `generate()` 缓存命中档 1,711 ns/op（≈1.6 %）与本地回环
一次 POST 往返 min 0.68 ms（约 4e-5 倍）⇒ 不为此写快路。本轮是能力轮，不做 before/after A/B
（「以前不消耗这两个旋钮」与「现在消耗」之间没有可比的性能口径）。

**证据**：注入 8 项（摘除式反向 patch）红数 **7 / 2 / 2 / 1 / 5 / 5 / 2 / 2**，无 0 红、无全集红；
其中一项初版把 NaN 判据写成恒假，导致所有浮点全被拒（20 红）⇒ 红数异常大同样要分诊，重注后才精确命中。
新增 **61** 条用例（`TestRequireSeconds` 21 + 后端默认档位 8 + 重试次数生效 3 + 流水线接线 7 + 配置校验 22），
全量 **4762 passed / 2 skipped**（**4701 + 61 精确对上**，总计 99.02%，门禁 80% 通过）。
既有断言只改 1 处，且是**故意收紧**：`test_pipeline_forwards_cache_options` 的精确字典 pin 被工厂契约
变宽打红，补两键显式认领而非放宽。16 文件 **+460 / −24**，逐文件行数差与 numstat 净值精确相等。

### 3.21 L46：删掉从不起跑的传输层重试，顺手把降级分支说成真话（关闭 A69 / A71）

§3.20 留下的那笔「死适配器去留」在本轮拍定，取 A69 的建议方向 ①：
`_get_session` 里 `HTTPAdapter(max_retries=Retry(total=3, backoff_factor=0.1, status_forcelist=[429,500,502,503,504]), …)`
的 `max_retries` 参数整个删掉，只留 `HTTPAdapter(pool_connections=10, pool_maxsize=20)`。
删之前先做可达面普查（非测试代码 6 处请求点）：**6/6 全是 `session.post(...)`，零 GET/PUT/DELETE，
`Session()` 构造点 2 处且都在这同一个函数里**，且全仓 `grep -r "urllib3"` 只命中这一处 ⇒ 那副 Retry
唯一可能被读到的方法面（GET/HEAD/OPTIONS/PUT/TRACE/DELETE）在本仓不存在。方向 ②（加
`allowed_methods={'POST'}` 让它真重）实测确认是叠乘风险而不是收益，不采。

**行为不变的证据是同进程交错 A/B，不是「读了一遍代码」**（Temp `l46_ab2.py`，恒 503 / 恒 200 的本地
回环 `ThreadingHTTPServer` 数服务端收到的请求，两档只换适配器、其余路径完全一致）：

| 适配器 | `generate(max_retries=0/1/3/None)` 的服务端请求数 | 恒 200 时单次请求计数 |
|--------|---------------------------------------------------|------------------------|
| 挂 `Retry(total=3, status_forcelist=…)`（改前） | **1 / 1 / 3 / 3** | **1** |
| 只留连接池（改后） | **1 / 1 / 3 / 3** | **1** |

墙钟两档各测 4 组（正反序各两轮，min + 中位）：失败档绝对差在 **−28 ~ +40 µs** 之间跳、成功档在
**−10 ~ −40 µs** 之间跳，**符号跨轮不稳**，量级 ≤ 一次回环往返（≈620–2,300 µs）的 4 %
⇒ **本轮没有性能收益可主张**，删除的理由是契约诚实（少一层不存在的重试 ⇒ 少一个会被抄成数字的乘数，
§3.20 那个「12 次」就是这么来的）。顺带得到一条新纪律：**A/B 至少正反序各一轮，符号不稳就不许写收益**。

**连接池数字按原文核过**（requests 2.33.1 `HTTPAdapter.__init__` 源码与其 docstring）：
`DEFAULT_POOLSIZE = 10`、`DEFAULT_RETRIES = 0`、`DEFAULT_POOLBLOCK = False` ⇒
`pool_connections=10` 与上游默认同值（写了等于没写，但保留显式写法免得依赖上游不变），
`pool_maxsize=20` 是本仓唯一偏离默认的项；`pool_connections` 语义是「缓存多少个 host 池」
（每个后端实例一个 session、一个 host ⇒ 远未触顶），`pool_maxsize` 是单池连接上限。本仓各模块默认并发
`max_workers ∈ {3, 4, 5}`（`expander.py:30` / `export.py:51` / `multilingual.py:43` / `context.py:21` /
`pipeline.py:554`）⇒ 20 有 4 倍余量，且它是 per-host 上限，不构成跨后端瓶颈。原作者注释从未说明这两个数，
本节只是把「实测得到的关系」补成可核对的话，不宣称这就是当初的理由。

**A71：降级分支同时修三处说谎的地方。** 原形是
`except ImportError: logger.warning("requests 未安装，使用基础连接"); import requests; self._session = requests.Session()`，
三条 import 任一失败都进这里，而分支第一句就把已经成功的 `import requests` 再跑一次。本轮改成
「`import requests` 在 `try` 外面先做、只有 `from requests.adapters import HTTPAdapter` 包在可降级的
`try/except ImportError` 里」，于是三种情形各得其所：requests 本体缺失 ⇒ 原样抛 `ImportError`（硬依赖，
fail loud，不再先打一条假告警再抛一个无关异常）；`HTTPAdapter` 缺失 ⇒ 降级为无连接池 session 且日志写明
降级掉的是连接池；正常 ⇒ `logger.info("创建 HTTP 会话（带连接池）")`。同函数里那句
`Returns: requests.Session 或 httpx.Client` 也一并更正 —— 全仓（含 `requirements*.txt`）
`grep httpx` 只命中这一行 docstring，是个从未兑现过的幻影契约。
**该分支不是死代码**：`tests/unit/test_micro_branches_l62.py::test_import_error_falls_back_to_plain_session`
一直在测它（用一个缺 `HTTPAdapter` 属性的假 `requests.adapters` 命中），所以「产品不可达」与「无覆盖」是两件事，
账本 A71 原句偏松，已就地校正。

**新钉的测试**（`tests/unit/test_model_backend.py::TestTransportRetrySingleSource`，7 条定义 = **17 例**）：
① 5 个后端 × http/https 两个 scheme 的适配器都必须 `max_retries.total == 0`（10 例；只钉 http 会留下
「真实 API 全走 https」的盲区）；② 传输层不得持有 `status_forcelist`（状态码可否重试只能由
`classify_error` 裁决，2 例）；③ 删重试不许把 `pool_connections/pool_maxsize` 一起删了；④ 一条**上游事实哨兵**
—— `Retry(3).DEFAULT_ALLOWED_METHODS` 不含 POST，若哪天 urllib3 改了默认，这条会红，提示重新审计而不是
静默放大请求数；⑤ 会话里**每个**已挂载前缀都要 0 重试（不看 `http://`/`https://` 之外的键就漏了）；
⑥ 降级分支的两条断言（日志文案指对根因 + 降级后的 session 同样 0 次传输重试）；
⑦ requests 本体缺失 = 抛 `ImportError` 且 `_session` 仍是 `None`（不留半初始化状态）。

**交叉核对退避封顶时撞出来的新缺陷（账本 A72；L46 只记不修，L47 的裁定与实测见 §3.22）**：`with_retries` 在服务端给出
`Retry-After` 时走的是另一条分支（`delay = min(MAX_RETRY_AFTER, max(0, suggested))`），
**`max_delay` 参不到场** ⇒ `generate()` 传进去的 `ModelBackend._MAX_RETRY_DELAY = 30.0` 对限流指令无效。
实测（Temp `l46_retryafter.py`，注入 sleeper）：429 + `Retry-After: 45 / 300 / 3000` 的等待序列是
`[45, 45] / [300, 300] / [300, 300]` ⇒ 那句「单次重试等待上限」在最坏情形差 10 倍，默认档一次
`generate()` 可以睡 2 × 300 = 600 s 而不是 60 s，L45 给配置旋钮定的 `retry_delay ≤ 60 s` 同样被旁路。
量这一条踩了两个探针坑，都值得记住：`with_retries` 的 `sleeper=time.sleep` 是**定义时绑定的默认参数**，
事后 `mock.patch("augmentor.retry.time.sleep")` 拦不住（v1 因此量出「sleep 序列 = []」差点误报成产品没等）；
`classify_error` 只在 `isinstance(status_code, int)` 时才认定状态码，用 `mock.Mock()` 充当 response 会静默
退化成「无建议等待」（v1 那四条全 `[1.0, 2.0]` 就是这么来的）—— **探针量出「没有症状」时，先证明探针能看见症状。**

**证据**：注入两轮。① 把 `Retry(total=3, status_forcelist=…)` 原样挂回去 ⇒ 定向 17 例中
**13 红 / 4 绿**，那 4 条绿正是**不该红**的四条（连接池参数不受注入影响、上游事实哨兵、
HTTPAdapter 降级路径、必需依赖缺失路径）。② 三条结构注入（Temp `l46_inject2.py`，定向 43 例子集）：
把 `except ImportError` 改成 `raise`（取消降级）⇒ **2 红**（本轮新增的降级文案例 +
`test_micro_branches_l62.py::test_import_error_falls_back_to_plain_session`，证明那条 L62 老例确实在守这条分支）；
把 `import requests` 放回 `try` 内（复原改前形状）⇒ **1 红**（只有本轮新加的 `test_missing_requests_…` 红，
L62 那条例看不见这个隐患 ⇒ 一条既有测试守住的分支不等于守住它的全部隐患形状）；日志文案退回
「requests 未安装」⇒ **1 红**。两轮还原后均按原始字节核对一致，定向子集回到 43 例全绿。
全量 **4779 passed / 2 skipped**（**4762 + 17 精确对上**，39.10 s，总计 99.02%，门禁 80% 通过，
2 skip 仍是既有 chromadb 缺依赖）。`models/base.py` **+24 / −25**（净 −1 行：删掉的是重试装配与一次
冗余 import，加上的是更清楚的控制流与三条诚实注释），测试 **+101 / −1**、本文档 **+82 / −1**（1,156 → 1,237 行）。

### 3.22 L47：两副等待封顶各管一支，浮点退避旋钮补上判据（关闭 A72 与 A67 的浮点残余）

A72 的裁定取 **方向 ①（诚实化，0 行为变更）**，不取 ②（把 `max_delay` 的语义扩到也夹服务端建议）。
理由不是「② 麻烦」而是 ② 会拿到一个更坏的行为：`Retry-After: 45` 被夹成 30 s，等于对刚说过
「45 秒后再来」的服务端提前 15 s 敲门，而限流场景里这正是最不该做的事；这个方向与 §3.21 里
A69 方向 ②（让传输层真的重 POST）同族——**拿到服务端的指示却不完全照办**。
① 的工程量因此全在「把已经在生效的语义说清」：四处文案各修各处（`ModelBackend._MAX_RETRY_DELAY`
的注释、`config.yaml` 里 `retry_delay` 的注释、`retry.MAX_RETRY_AFTER` 与 `with_retries` 的口径说明），
外加一支测试把两副封顶的**互不越界**钉住。方向 ② 里真正有价值的那半件事——「让使用者能把
最坏等待调小」——立为 **A73** 单独一轮做：它要动 `AugmentationConfig`（**不是** `ModelConfig`，本仓两个类名相近、
L44 就把这两个字段的归属记错过一次）+ `config_validator` + 工厂透传 + 文档 + 测试五处，半接一条旋钮比不接更糟（A65 的教训）。

**「最坏等待」现在是可以算的，两个数各归一支**：

| 哪一支 | 触发条件 | 封顶 | 默认档（`attempts=3`）一次 `generate()` 最坏总等待 |
|--------|----------|------|---------------------------------------------------|
| 退避计算 | `classify` 没给建议（无 `Retry-After`，或根本没传 `classify`） | 调用方的 `max_delay`，模型后端传 `_MAX_RETRY_DELAY = 30.0`。**前提是 `jitter=0`**：抖动加在这道夹**之后**，非 0 时这一支的真正上界是 2 × `max_delay`（L48 补的条件，见 §3.23） | 2 × 30 = **60 s**（实测第 1/2 档 = 1.0 / 2.0，远未触顶） |
| 服务端指令 | `classify` 返回的建议非 `None` | `retry.MAX_RETRY_AFTER = 300.0`（**自 L49 起是「上界的上界」**：`augmentation.max_retry_wait` 只能在 0-300 内把它调小，不能放大，见 §3.24） | 2 × 300 = **600 s** |

第二支不是「不设限」：实测 `Retry-After: 3000` 与 `864000` 都拿到 `300.0`（`test_max_retry_after_clamps_it_instead`
与 §3.21 就有的 `test_retry_after_is_capped` 各钉一头）。而 L46 之前它连个名字都没有，只被一句
「单次重试等待上限」顺手概括掉——那句话在限流场景最多差 10 倍（45 s vs 30 s 实测）。

**A67 的浮点残余一并补上判据**：`compute_delay` 与 `with_retries` 的 `base_delay` / `factor` / `max_delay`
三参照 §6 既有口径在入参处一次判掉（`require_seconds` 两参 + 新判据 `require_positive` 判 `factor`）。
`require_positive` 不复用 `require_seconds`：后者的文案是「必须是数值秒数」，拿来判一个无量纲倍率会把
根因说错（§3.21 刚为「文案指错根因」记过一笔）；下界也不做成可配参数，因为这类值只有「每档乘多少」
一种合法读法。修前的坏值形状全部实测过（Temp `l47_probe.py`）：`factor=0` → `[1.0, 0.0, 0.0]`、
`factor=-2` → `[1.0, 0.0, 4.0]`、`factor=NaN` → `[1.0, 30.0, 30.0]`、`base_delay=True` → `[1.0, 2.0]`
（布尔被安静读成 1.0）、`max_delay=-1` → 整条退避清零。而 `base_delay='1'` 最恶劣：第一次真实调用
**照常打出去**，退避时才在幂运算处炸成 `TypeError: can't multiply sequence by non-int of type 'float'`，
`raise last_exc` 那句根本轮不到执行，原始异常整个丢失 ⇒ 这就是「判参必须排在 `func` 之前」的实证理由，
与 L45 给构造函数定的「判参先于 `mkdir`」同一条纪律。两处判据（`with_retries` 入口 / `compute_delay` 本体）
**不是冗余**，注入分得开：只摘 `compute_delay` 那道 ⇒ 2 红且 9 条 `with_retries` 侧的照常绿；
只摘 `with_retries` 入口那三行 ⇒ 9 红（全部红在「`calls == []`」这条顺序断言上）而 `compute_delay` 侧照常绿。
`0 < factor < 1` 与 `+inf` 都放行：实测前者给 `[1.0, 0.5, 0.25]`（非负、单调、有界，没有需要拒的形状），
后者第 2 档起就贴着 `max_delay`。

**`parse_retry_after` 那条残余按实测判定为「不是缺陷」**，写在这里免得下一轮重查：过去的 HTTP-date →
`0.0`、负数 → `0.0`（实测 `parse_retry_after("-3") == 0.0`，同一形状的 `classify` 建议也读成「等 0 s 立刻重发」），
这与 RFC 对「时限已过」的读法一致，判成坏输入反而会替服务端把一个合法信号变成异常；远未来与荒谬大值
（`'1e9'` → `1000000000.0`、日期 +10 天 → `863999.4`）由第二支的 `MAX_RETRY_AFTER` 夹住；非数值（`'soon'` /
空串 / `None` / `["120"]`）→ `None` 即「没有建议」，回落到退避计算。这一组配了正对照（+120 s 的日期量出
119.4 s）才敢说「探针没瞎」。

**`jitter` 的区间判据刻意留给 A65**，不在本轮顺手做：产品里至今没有任何路径能传非默认 `jitter`，
而实测 `jitter=50` 会让结果（61.09 s）越过 `max_delay=30` —— 判据与透传分两轮做，中间那一档就是
「文档说 0-1、代码不判」的半保护状态，正是 A65 立条时点名要避免的形状。**A65 已在 L48 落地**
（透传 + 判据 + `rng` 回落同批），那一档半保护状态至此关闭，见 §3.23。

**代价（本轮不主张收益）**：`compute_delay` 单次调用从 0.1 µs 涨到 0.7 µs（同进程 A/B，Temp `l47_ab.py`，
正反序各一轮、**符号跨序稳定**），走拒绝路径约 1.0 µs（异常构造占大头）。量级对照：一次回环 HTTP 往返
按 §3.21 实测是 620 ~ 2,300 µs ⇒ 每档退避多出的 0.6 µs 不到一次请求的 0.1 %，而 `with_retries` 入口那三行
每次 `generate()` 只跑一遍。诚实的说法是：**这是硬化的成本，不是优化的收益**。

**测试**：37 例新增（`test_count_knob_validation.py::TestRequirePositive` 20、`test_retry.py::TestRetryFloatKnobs` 11、
`test_retry.py::TestTwoDelayCeilings` 6），**另有 1 处既有测试改写**——L45 立的
`test_compute_delay_hides_bad_values_which_is_why_the_guard_exists`（1 条 × 3 参）拆成
`test_compute_delay_keeps_the_good_shape` + `test_compute_delay_rejects_what_it_used_to_hide`（1 + 1 × 2 参，
净 0 例）。改写不是清理：那条例钉的正是本轮搬进函数内部的旧症状，判据进了 `compute_delay` 之后
它**必须**失败，留着「静默换一档」的断言等于把修法倒过来。新增的 `test_both_ceilings_match_what_the_docs_promise`
钉住 `(300.0, 30.0)` 这一对常数：改任一常数都会红，逼着改的人同步本节与 §6，而不是让文档漂走。

**记账**（`git diff --numstat` 实测，不是先写后补）：`augmentor/validation.py` **+45 / −1**（543 → 587 行，新判据
+ 模块 docstring 一行）、`augmentor/retry.py` **+48 / −11**（296 → 333）、`augmentor/models/base.py` **+3 / −1**
（368 → 370，全是注释）、`config.yaml` **+3 / −1**（345 → 347，全是注释）、`tests/unit/test_retry.py`
**+126 / −1**（377 → 502）、`tests/unit/test_count_knob_validation.py` **+61 / −12**（744 → 793）、本文档
~~**+71 / −3**（1,237 → 1,305）~~ **+81 / −3**（1,237 → 1,315 —— 初量的 +71 是 §6 那三行旋钮条目与本段两处改正写完之前的状态），`OPTIMIZATION_LOOP.md` 另计。七个文件的行数差与各自净值逐一相等。
`config.yaml` 改完仍 `load_config` 出 3 / 1，`validate_config_file` 判 `is_valid=True`、5 条 warning
与 HEAD 版逐条同名（都是未设的 API key）。全量 **4779 + 37 = 4816 passed / 2 skipped**，总计 99.02 %，
门禁 80 % 通过。**本节初稿把六个 numstat 里的三个先写成了预测值**（`config.yaml` 写成 +4/−2、
`test_retry.py` 写成 +124/−1、`test_count_knob_validation.py` 写成 +57/−11），落笔时并未跑过 diff
—— 与 L46 那条「记账数字必须先测后写」是同一处漏洞，本轮又踩一次，说明这条纪律还没有真正长进
肌肉里：本轮的做法是「写完再量、量完就地改正」，而正确做法是「先量、数字没到手之前那段不写」。
**同一次还暴露了这条纪律的第二种破坏形状**：本文档自己那一行数**量过，却量的不是最终态** —— 量完之后
又补写了 §6 的条目与本段的两处改正 ⇒ 「测过一次」不等于「测的是最终态」。六个源/测试文件的数字测完即定，
唯独**文档本体与账本**的 numstat 只能在该文件**最后一次编辑之后**量；本轮照此复量，故上面那行是最终值。

### 3.23 L48：把 `jitter` 接到唯一入口上，判据族补第四员「闭区间比例」（关闭 A65，新立 A75）

`compute_delay` 的抖动分支**从建库起就在**，但 `with_retries` 从来不透传它（改前实测函数体里
`jitter` / `rng` 各 0 命中），而全仓产品路径只有 **1 个** `with_retries` 调用点（`models/base.py:309`）
⇒ 那一支对真实调用方是死的。接它的理由不是「多一个旋钮」：本仓有 5 处 `ThreadPoolExecutor`
（`context.py` 5 / `expander.py` 3 / `export.py` 4 / `multilingual.py` 4 / `pipeline.py` `max_workers=4`
与 `num_threads` 一处）会并发打同一个后端，同一档退避让它们**同时醒来**再撞一次 429，
抖开才有意义 —— 这是「立条时假定的前提」第一次被逐处量出来。

**量出来的第一件事改变了 §3.22 的口径**：抖动是加在 `min(max_delay, …)` 那道夹**之后**的
（`delay += rng.uniform(0.0, jitter * delay)`），所以它一旦非 0 就成了退避这一支真正的上限来源。
同一贴顶档位（`base_delay=20`、`attempt=3` ⇒ `min(30, 80)` 夹完正好 30.0）各抽 500 次实测
（Temp `l48_probe2.py`，分母即 500；`jitter=1.0` 那一档另在**同样夹顶**的 `attempt=4`
（`min(30, 160)`）上抽 2 万次复算，最坏 59.999 s，见 `l48_probe3.py` 第 8 项）：

| `jitter` | 抽样区间（s） | 越过 `max_delay=30` 的比例 |
|----------|----------------|-----------------------------|
| 0.0 | 30.000 ~ 30.000 | 0.000 |
| 0.5 | 30.023 ~ 44.935 | 1.000 |
| 1.0 | 30.045 ~ 59.871 | 1.000 |
| 2.0 | 30.091 ~ 89.742 | 1.000 |
| 50.0 | 32.265 ~ **1523.542** | 1.000 |

⇒ §3.22 那句「退避一支 30 s」是**以 `jitter=0` 为前提**的，本轮已把该条件补回那一行的表格里。
把 `jitter` 判在 0-1 闭区间，换来的是这一支最坏等待的可证上界 **2 × `max_delay`**（夹后最多再乘 `(1+1)`）。

**坏 `jitter` 的三种静默形状**是这一轮真正的缺陷面，它们都不报错：`True` 被算术读成 `1.0`
（同一种子五次抽样与 `1.0` **逐字同答** ⇒ 手滑打一个布尔等于直接把抖动拉到**最大档**）；
`-5.0` 与 `NaN` 都过不了 `if jitter > 0` 那道门，交出的档位（实测 `[1.0, 2.0, 4.0]`）与
「根本没传参数」**一字不差** —— 「传了参数等于没传」正是 L33 禁掉 `or` 回落之后反复回来那一族。
所以判据族补第四员 `validation.require_ratio(name, value, minimum=0.0, maximum=1.0)`：
它是唯一**上界也要判**的成员（其余三位只判下界），因为比例旋钮越界的症状是「上限从别处冒出来」；
`None` 表示「没传」直接放行（实测 60 ns，传值才走完整判据 235 ns），闭区间做成可配是为了下一位
比例旋钮（如未来的阈值）复用同一条文案而不必再造判据。

**`rng` 的回落判据从 `or` 改成 `is None`**：实测一个 `__bool__` 为假但完全可用的种子 rng
（Mock 与若干包装类的常见形状）被 `rng or random.Random()` 静默换成全局随机源 —— 同一个「固定种子」
对象连续三次调用抽出三个不同值（6.54 / 7.25 / 5.62），而签名里那句「可注入以便测试」当场失效。
这是本仓第 N 次为 `or` 回落记账，规则不变：**判「没传」只能用 `is None`**。

**抖动只加在退避这一支**（与 A72 拍的「拿到服务端的指示就照办」同一条口径）：`classify` 给出建议
等待时原样采纳、不叠加。这条语义不是注释里的愿望，注入验证钉住了它 —— 把抖动也涂到 `suggested`
那一支 ⇒ 恰 1 条测试红（`test_jitter_never_stretches_a_server_instruction`，桩随机源被调用即红）。

**后端仍然传 0**（`generate()` 不传 `jitter`，实测 `captured` 里根本没有这个键），因此 §3.22 的
30 s 口径在产品路径上依然逐字成立；配置面暴露**另立 A75**，与 A73 / A74 同批 —— 上一轮刚为
「半接一条旋钮比不接更糟」记过一笔，本轮不把 `jitter` 单独立成配置字段。**（L49 状态更新：A75 已关闭，
`augmentation.retry_jitter` 上线并接进 `generate()`，默认 0.0 ⇒ 本节那句 30 s 口径逐字不变，见 §3.24。
本节那句「`captured` 里根本没有这个键」描述的是 L48 当时的形状，L49 起该断言已改成「键在且值为配置读出的
`retry_jitter`」—— 见 `test_models.py::TestGenerateRetryKnobs`。）**

**两处判据依然不冗余**（沿用 L47 的注入分判法）：只摘 `compute_delay` 那道 ⇒ **2 红**（全在
`TestRequireRatio` 直调 `compute_delay` 的两条上）；只摘 `with_retries` 入口那道 ⇒ **8 红**（全在
`test_out_of_range_jitter_is_rejected_before_the_first_request` 的 `calls == []` 上），两组不相交。
另两模式各 1~2 红：摘透传 ⇒ **2 红**（档位落不到抖动上 + 「同种子两次相等」这条**弱断言**在
首次注入时只有 1 红，因为摘掉透传后两次都交 `[1.0, 2.0, 4.0]` 照样相等 ⇒ 已给该条补上
`a == b != [1.0, 2.0, 4.0]` 才分得开，规则：**能摘出 0 红或 1 红的断言要回头看它是不是只测了「不变」**）；
改回 `or` ⇒ 1 红；后端偷传 `jitter=0.5` ⇒ 1 红。六模式全部还原后 sha 与基线逐字节一致。

**代价**（同进程 A/B，Temp `l48_ab.py`，正反序各一轮、min-of-7）：`compute_delay(3)` 0.814 → 0.972 µs
（×1.194，两次顺序同向）、`with_retries` 成功档 1.202 → 1.351 µs（×1.124，同向）。
「重试 1 次」那一档两次 A/B 符号相反（A 5.30 / 7.04，B 5.36 / 4.86）⇒ 按 L43 的规矩**该档不可判定，
不引用**。绝对量级对照：判据多出的 ~0.15 µs 落在其后 ≥ `base_delay`（默认 1 s）的 sleep 之前，
是 1.5e-7 倍 —— 与 L44 定的那条一样，**比值不构成立项依据**，本轮同样不主张性能收益。

**零回归锚点**：不传 `jitter` 时五档退避仍是 `[1.0, 2.0, 4.0, 8.0, 16.0]`，与接参前逐字相同；
`with_retries` 默认路径的 `stats.delays` 仍是 `[1.0, 2.0, 4.0]`（`test_default_jitter_leaves_the_ladder_untouched`）。

**测试**：41 例新增 —— `test_count_knob_validation.py::TestRequireRatio` 23（判据本体）、
`test_retry.py::TestJitterPassthrough` 15（透传、可复现、falsy rng、未注入 rng 的回落支、两支隔离、2 × 上界、入口判参）、
`test_retry.py::TestJitterConfigSurface` 2（钉住「当前默认不抖」这一决策，改它必须同步 A75）、
`test_models.py::TestGenerateRetryKnobs::test_backend_does_not_shake_the_backoff_yet` 1。
按 `--collect-only` 逐类 grep 计数与全集差值相符：全量 **4816 + 41 = 4857 passed / 2 skipped**，
总计 99.02 %，门禁 80 % 通过（提交前复跑 50.20 s）。

**记账**（`git diff --numstat` 实测，六个源/测试文件在本节写完后不再改动，故数字即最终值；
本文档与账本按 L47 的规矩在**最后一次编辑之后**量）：`augmentor/validation.py` **+51 / −1**
（587 → 637，新判据 + 模块 docstring 一行）、`augmentor/retry.py` **+27 / −10**（333 → 350）、
`augmentor/models/base.py` **+2 / −0**（370 → 372，全是注释）、`tests/unit/test_retry.py`
**+151 / −1**（502 → 652）、`tests/unit/test_count_knob_validation.py` **+69 / −1**（793 → 861）、
`tests/unit/test_models.py` **+19 / −0**（485 → 504）、本文档 **+88 / −2**（1,315 → 1,401）。
七个文件的行数差与各自净值逐一相等。`config.yaml` 本轮**未改** —— 抖动不是配置旋钮（A75）。
覆盖读数两条：`retry.py` 120 语句缺 **6**（`287` / `310-311` / `346-350` + 偏支 `180->220`）、
`validation.py` 193 语句缺 **0**（5 条偏支全在 `DataSanitizer` 旧码）。本轮新增语句一度**多出一条未触达**
（`rng = random.Random()` 那行，因所有用例都注入了 rng）⇒ 补 `test_missing_rng_still_shakes_within_the_bound`
把它走掉；A68 原来那六条一个没少，只是行号随本轮又位移了一次。

### 3.24 L49：把两副等待封顶交到配置面，校验器与运行时第一次做到全区间同判（关闭 A73 + A75，新立 A76 / A77 / A78 / A79）

A75 行末预告的「等待预算」一轮落地，做足 A73 + A75 两件事，**A74 刻意留下一轮**：那 6 处
`session.post(..., timeout=…)` 是三个互不相同的常数（60 / 10 / 120 s），而「ernie 换 token 那 10 s 要不要
共用同一根旋钮」是另一条轴上的决定 —— 塞进同一轮会把「等待预算」与「请求超时」两个口径混写成一格。

**两个新旋钮**都进 `AugmentationConfig`（**不是** `ModelConfig`，A64 行记错过一次）：`max_retry_wait: float = 300.0`
（`config.py:37`）与 `retry_jitter: float = 0.0`（`:40`），默认值就是 L47 / L48 定下的那两个常数 ⇒ 已发布配置
的行为一字不变；`config.yaml:88` / `:94` 把两键写出显式值并各带注释。接线面八处，一处不落（避免 A54 型
「契约面有旋钮、实现面不消费」）：`config.py` 字段 + `config_sections` 映射表（`:314`）→ `config_validator`
的 KNOWN_FIELDS 规格（0-300 / 0-1）→ `pipeline._init_components` 两道判据（排在降级 `try/except` **之外**，
L45 定的那条）+ 转发 → `models/factory.py` 签名 → 5 个后端构造透传 → `ModelBackend` 存 → `generate()` 把
`max_retry_wait` / `jitter` 交给 `with_retries`。

**口径（A73 行里预拍的那条，本轮照办）**：`max_retry_wait` **只能把 300 s 夹小、不能放大**。这不是保守而是
承诺本身：判据写成 `require_seconds(..., maximum=MAX_RETRY_AFTER)`。加判据前实测（Temp `l49q/probe_budget.py`，
注入 sleeper、不发真实请求，nonce `1b3af725db17`）—— 摘掉那道判据后 `max_retry_wait=inf` 配
`Retry-After: 3000` 交出 `sleeper=[3000.0, 3000.0]`，默认档一次 `generate()` **合计 6000 s**（传 `3000.0`
同形）；判据在场时同一个入参在**第一次请求之前**就抛 `DataValidationError`，而默认封顶下同一场景是
`[300.0, 300.0]` = 600 s。`0.0` 是合法档，语义「不再尊重服务端指令、失败就立刻重发」，与 `max_delay=0`
在退避一支的读法同一条口径。

**为什么给 `require_seconds` 加一个 `maximum` 参数、而不是造第五判据**：它判的仍然是「秒数」，文案根因不变
—— L47 之所以另立 `require_positive`，是因为把「数值秒数」安到一个无量纲倍率上会说错根因，这里没有这个
错位。既有调用点不传 `maximum` 时判定逐字不变（实测 0.137 → 0.143 µs，见代价段）。

**最坏总等待第一次可以用一行式子写完**（两支各一顶，抖动乘在退避那一支上）：

`(attempts − 1) × max(_MAX_RETRY_DELAY × (1 + retry_jitter), max_retry_wait)`

实测五档（同一探针，`attempts=3`，服务端统一给 45 s）：

| `max_retry_wait` | `retry_jitter` | 服务端指令档 sleeper | 退避档 sleeper（无 `Retry-After`） | 式子给出的上界 |
|---|---|---|---|---|
| 300.0 | 0.0 | `[45.0, 45.0]` = 90 s | `[1.0, 2.0]` = 3.0 s | 600 s |
| 45.0 | 0.0 | `[45.0, 45.0]` = 90 s | `[1.0, 2.0]` = 3.0 s | 90 s |
| 45.0 | 0.5 | `[45.0, 45.0]` = 90 s | `[1.17, 2.25]` = 3.4 s | 90 s |
| 0.0 | 1.0 | `[0.0, 0.0]` = 0 s | `[1.77, 2.40]` = 4.2 s | 120 s |
| 300.0 | 1.0 | `[45.0, 45.0]` = 90 s | `[1.43, 3.47]` = 4.9 s | 600 s |

第一、二行的对比里藏着一个必须写清的误读点：把封顶调到 45 s **不会**让一条 45 s 的建议变短（两行实测都是
90 s），它压住的是**大于** 45 s 的建议 —— 这根旋钮买到的是上界，不是当前等待。第四行是「封顶归零 + 抖动拉满」：
服务端指令那一支归零，等待全部落到退避一支，上界由 `2 × 30 × (1 + 1) = 120 s` 给出，而实测只落到 4.2 s
（退避基数 1 s 时离封顶还很远，与前几轮「上界可证、实测未触顶」的口径一致）。

**守护第一次跑就红，红出来的是真洞（A78）**：新写的映射表覆盖守护 `TestSectionDefaultsMatchTheMappingTable`
首跑报出缺失 `('web', ['cors_credentials', 'cors_origins'])`。那不是测试写错：`WebConfig` 声明了这两个字段
（`config.py:182-183`）、`api/main.py:119-120` 确实在读，而 `load_config` 的 `web` 默认表里没有 ⇒ `_load_section`
按表取键，用户写了也不读。实测复现（写 `cors_origins: ["https://trusted.example"]` + `cors_credentials: false`
读回 `['*']` 与 `True`，**同一文件里的 `web.port: 9999` 生效** ⇒ 不是整节失效，只有那两键被丢）。后果是 CORS
出厂即「全源 + 带凭证」，而唯一的收紧路径恰好被这条洞堵住（`validate-config` 也不报错，因为这两个键在
KNOWN_FIELDS 里是**已知**的 —— 已知于校验器、未知于读取器）。本轮不动它（修法与 A76 同源，要一起拍），
做法是**把守护改成精确棘轮**：`KNOWN_UNMAPPED_FIELDS` 常量列出这两条豁免，断言「实际缺失集必须与之相等」⇒
补一条就红（注入 m5 ⇒ 恰 1 红）、漏一条也红，不许悄悄扩面。

> **（L50 校正）** 该段末尾「本轮不动它」是 L49 的处置，A78 已在 §3.25 关闭，`KNOWN_UNMAPPED_FIELDS`
> 的豁免清单随之清空。同理，本节「测试」段的 **4940 passed / 2 skipped**（99.02 %）与同轮另一处的
> 4920 / 3（98.64 %）也不是环境漂移，而是**两套解释器**：`C:\Python314`（pandas 3.0.2 + chromadb 1.5.9 +
> starlette 1.2.1）与 venv `aug`（pandas、chromadb 皆无，starlette 1.6.0）。L50 的同一份工作树两跑 =
> **4963 passed / 3 skipped**（aug，= 4920 + 43）与 **4983 / 2**（`C:\Python314`，= 4940 + 43），
> 两套各自与本行历史数字严丝合缝 —— 差 **+20** 的构成（实测四份：`--collect-only` 清单 4,964 vs 4,985、
> 两份全量日志，Temp `l50q/col_aug.txt` / `col_py314.txt` / `full_aug2.txt` / `full_py314.txt`）：
> **+21** 两个 pandas 门控文件（`test_csv_excel_export.py` 9 例 + `test_csv_excel_import_real.py` 12 例）
> 在 aug 下**根本不 collect**（模块级 `importorskip` ⇒ 只各留 1 条 skip 占位）、在 `C:\Python314` 下
> collect 且 pass；**+1** `test_micro_branches_l61.py:99` 在 aug 下缺 pandas 而 skip、在 `C:\Python314`
> 下实跑；**−2** `test_chromadb_backend.py:110` 与 `test_vector.py:56` 在 aug 下走「缺依赖」分支而
> pass、在 `C:\Python314` 下 skip。21 + 1 − 2 = 20，Δskipped = 3 → 2 = −1，两式相加 = Δevents = +19
> = 4,985 − 4,966 ✓。结论与纪律：**覆盖率分母同样是解释器的函数**（同一份代码 TOTAL 语句 12,785 vs
> 12,159、覆盖 98.64 % vs 99.02 %），此后全量数字必须连解释器身份一起记。

**同构扫描挖出的 bool 洞（A79，本轮发现并同轮修掉）**：`isinstance(True, int)` 恒真，于是
`_validate_known_fields` 里 7 个数值规格键**全部**把 YAML 里的 `retry_jitter: true` 读成合法，而运行时四道判据
（`require_count` / `require_seconds` / `require_positive` / `require_ratio`）**全部**显式拒 bool ⇒ 症状正是
「`validate-config` 绿灯、一进 `AugmentorPipeline` 就 `DataValidationError`」（参数错误被读成后端不可用那一族）。
修法：内联判定里排 bool（`type: bool` 的开关字段不受影响，实测 3 个 bool 规格照常通过）。**NaN 那一半不在本轮的 diff 里** —— 它是 L45 就补下的守卫，本轮没有重复劳动，只是用注入 m8 把它摘掉 ⇒ **5 红**，坐实它至今仍在守这条契约。三条形状逐守卫实测（Temp `l49q/final_probe.py`，NONCE-9b00030107a0；同一支探针在 0ba6943e13a7 那次跑到守卫段也给出逐字相同的三行）：**现行 0/7 放行 bool、0/7 放行 NaN**；摘掉 bool 守卫 → bool **7/7** 放行而 NaN 仍 0/7；摘掉 NaN 守卫 → NaN **4/7** 放行而 bool 仍 0/7 ⇒ 两条守卫**覆盖面互不重叠**（「重复劳动」的判据在这里同样是「摘一处能不能把另一处分开来」），且 NaN 的实际覆盖面是**完整的** —— 3 个 `type: int` 规格靠 `isinstance(nan, int)` 为假天然拦住，不需要额外判据。
两条洞的覆盖面不同，都量过（Temp `l49q/`，把克隆绑在覆盖 `_validate_known_fields` 的子类上重跑 7 个数值键）：
`true` 在旧形状下 **7/7 个数值键完全无报错**（`isinstance(True, int)` 恒真），NaN 只漏 **4 个 `type: float`
键**（3 个 `int` 键靠 `isinstance` 本来就拦得住）⇒ 排 bool 与 NaN 判据不是重复劳动，各堵一片。
这个洞能活到今天的原因就写在 A70 那笔死账上：**只有测试在调用的那 6 个 `_validate_int` / `_validate_float` /
`_validate_bool` 死助手早就排掉了 bool，却没有 NaN 判据**，而真正跑的内联路径两边都没有 —— 两套口径各拿对
半边，活的那套恰好是错的半边。A70 由此第一次有了实测代价，已在该行补记。

**A76 / A77 是同一次扫描顺手量出的既有债**（只记不修）：A76 = 配置里的未知 / 拼错键（如 `max_retry_wai`）
零反馈，与 A78 同一个根（那张映射表本身就是白名单）；A77 = `retry_delay ≤ 60`、`max_retries ≤ 20` 是
**校验器独有**的上界，运行时照收 999。本轮两个新旋钮特意做成两侧**全区间**一致
（`test_validator_and_runtime_agree_on_every_axis`），而旧旋钮那处不对称留在原地：给 SDK 面补上界会让既有
调用方的合法入参当场变非法，这是要单独拍的一次破坏性变更，不该搭在能力轮的车上。

**代价（本轮不主张任何收益）**：同进程 A/B，正反序各一轮、min-of-7（Temp `l49q/ab2.py`，nonce
`f090dff45108`）—— `with_retries` 一次成功调用 1.261 → 1.390 µs（+0.129 µs，+10.26 %，两序同号）；
`require_seconds` 既有调用点（不传 `maximum`）0.137 → 0.143 µs（+4.18 %，两序同号）；两道新判据本身
0.300 µs / 次构造；`_validate_known_fields` 走查 5 节 12 个规格 7.143 → 7.693 µs（+7.71 %），配套的
**同形克隆自检行顺序翻转 ⇒ 不可判定**，即那 +0.551 µs 不是克隆本身带来的。绝对量级对照：判据多出的
0.13 µs 落在其后 ≥ `base_delay`（默认 1 s）的 sleep 之前，是 1.3e-7 倍。

**`load_config` 那一行被自己推翻了**（本轮最重要的一条方法学）：首跑报「前 6687.9 / 后 6507.1 µs，
−2.70 %，两序同号」—— 一个加了两个键却变快 2.7 % 的结果。补测（Temp `l49q/ab3.py`，nonce
`ffe5cae48c1c`）两档迭代数：60 次×9 报 **序1 +27.20 / 序2 −17.35 ⇒ 顺序翻转**，120 次×5 报 +1.489 %
同号 ⇒ 三档互斥，那一行的「两序同号」骗过了 L43 定的符号判据。根因是载体成本：单跑一次
`yaml.safe_load` = 6,859 µs，几乎等于 `load_config` 的全部耗时（6.4 ~ 6.7 ms），两键的净成本直接测是
**0.088 µs**（占比 1.3e-5 %）⇒ 整函数 A/B 在这一档**没有判定力**。新纪律入「操作纪律」：**被测增量远小于
载体成本时不许用整函数 A/B，必须直接测边际操作**，「两序同号」在这种形状下不构成放行条件。

**注入 8 模式全部在 Temp 沙箱副本里跑**（`Temp/l49q/tree/`，4 MB 源码+测试副本，nonce `54ae1d9bf38b` /
`cfbfe0089e19`）：
基线 569 passed；m1 摘 `with_retries` 的 `max_retry_wait` 判据 ⇒ **7 红**；m2 后端不再传两封顶 ⇒ **3**；
m3 流水线不再转发两键 ⇒ **3**；m4 校验器放回 bool 洞 ⇒ **2**；m5 坐实 A78（把 cors 两键补进映射表）⇒ **1**；
m6 映射表默认值漂出 dataclass（300.0 → 30.0）⇒ **2**；m7 摘掉 `require_seconds` 的整条上界分支 ⇒ **14**；m8 摘掉 `_validate_known_fields` 的 NaN 判据 ⇒ **5**
（3 条 L45 时代立的旧例 + 2 条本轮新例 ⇒ 这条旧判据同样不是无人守的）；
全部还原 ⇒ **569 passed**，六个产品文件与真身逐字节 sha 一致。改在沙箱里做是本轮新加的机制：共享工作树里
做注入有被并行 agent 的中途提交带走坏状态的风险。与上一段会话那轮在 6 文件 / 547 例子集上的记录相比，
m3 由 2 → 3（第 3 条正是 `test_pipeline_forwards_cache_options` 那枚棘轮兑现的时刻）、m7 由 6 → 14
（本轮摘的是 `validation.py` 里整条上界分支，旧记录摘的是 `retry.py` 的传参处），其余各档一致。

**本轮第三次「探针接错口」**（方法学，比结论更值得留）：量 bool 洞覆盖面时第一版把克隆出来的方法当普通函数
调用，而 `_validate_known_fields` 内部靠 `self._validate_known_fields(...)` **递归**下探节内字段 ⇒ 递归全部
调回真身，两列（旧形状 / 本轮形状）打印出**逐字相同**的结果却看不出问题。改成「用子类覆盖该属性再实例化」
才拿到 7/7 与 4/7 这两个真数。纪律：**克隆一个内部有 `self.` 分派的方法，必须把它绑在覆盖同名属性的类上**；
而「两列完全一样」本身就是探针没接对口的症状，不是「修与没修等价」的证据。

**三处既有测试的处置**（如实记账，含一处全量首跑才暴露的红）：① `test_models.py::test_backend_does_not_shake_the_backoff_yet`
（L48 立，断 `"jitter" not in captured`）**被推翻重写**成「后端把两封顶原样交出」—— 它钉的前提随 A75 关闭
一起消失，硬留着就是把「旋钮没接」伪装成契约；② 同文件 `TestJitterConfigSurface` 的类 docstring 里那句
「抖动不是用户可配旋钮」同步改写（例数不变）；③ **全量首跑 1 红**：
`test_model_cache_optimization.py::TestPipelineCacheWiring::test_pipeline_forwards_cache_options` —— 那条按
**精确字典**断言工厂收到的全部键，注释明写「工厂契约每长一个键，这里就必须显式认领一次，不允许静默扩面」
⇒ 本轮正是它要抓的情形，按规矩显式认领两个新键（净 0 例）。另 `test_retry.py` 的 import 行改成包级
`from augmentor import (MAX_RETRY_AFTER, ...)`（1 行）。

**测试**：新增 **28 条测试函数**、参数化展开后全量净 **+83 例** —— 四个新类的 collect 数
`TestServerWaitCeilingKnob` 14、`TestWaitBudgetKnobSurface` 28、`TestRequireSecondsUpperBound` 19、
`TestSectionDefaultsMatchTheMappingTable` 4（合计 65），`test_models.py` 4 条展开 10 例减去被替换的 1 条（净 9），
`tests/integration/test_pipeline.py` 3 条展开 9 例。全量 **4857 + 83 = 4940 passed / 2 skipped**，
总计 99.02 %，门禁 80 % 通过（49.17 s）；定向 7 文件 **569 passed**。

**记账**（`git diff --numstat` 实测，产品与测试文件在本节写完后不再改动）：`augmentor/validation.py`
**+19 / −2**（637 → 654）、`augmentor/retry.py` **+26 / −9**（350 → 367）、`augmentor/config.py`
**+8 / −1**（467 → 474）、`augmentor/config_validator.py` **+17 / −1**（343 → 359）、`augmentor/pipeline.py`
**+11 / −2**（617 → 626）、`augmentor/models/base.py` **+42 / −9**（372 → 405）、5 个后端各
**+7 / −1**（claude 100→106、ernie 145→151、gemini 102→108、ollama 109→115、openai_model 107→113）、
`models/factory.py` **+10 / −1**（73 → 82）、`config.yaml` **+15 / −2**（347 → 360）；测试
`test_retry.py` **+89 / −4**（652 → 737）、`test_models.py` **+59 / −7**（504 → 556）、
`test_count_knob_validation.py` **+30 / −0**（861 → 891）、`test_config.py` **+103 / −0**（343 → 446）、
`test_config_validator.py` **+94 / −0**（779 → 873）、`tests/integration/test_pipeline.py` **+57 / −0**
（787 → 844）、`test_model_cache_optimization.py` **+3 / −0**（423 → 426）。19 个文件的行数差与各自净值
逐一相等；行尾逐文件按字节核对为纯色（LF 单一体：`retry.py` / `pipeline.py` / `models/` 全部 / 四个测试文件 /
本文档；CRLF 单一体：`validation.py` / `config.py` / `config_validator.py` / `config.yaml` /
`test_retry.py` / `test_models.py` / `tests/integration/test_pipeline.py`）。覆盖读数：`validation.py` 195 语句
缺 **0**（5 条偏支全在 `DataSanitizer` 旧码）、`config_validator.py` 152 缺 **0**、`pipeline.py` 245 缺 **0**、
`config.py` 215 缺 3（`449-450` / `452`，本轮 hunk 在 `35-40` 与 `314-315`，不相交）、`models/base.py` 157 缺 5
（`272` / `279-280` / `404-405`，全在既有 `_cache_*` 与 `__del__` 兜底，本轮 10 个 hunk 无一命中）、
`retry.py` 121 缺 6 ⇒ **本轮新增语句 0 条落入未触达**；A68 那六条依旧一个没少，行号随本轮 +17 位移成
`304` / `327-328` / `363-367`（+ 偏支 `197->237`）。

### 3.25 L50：`web` 节第一次整体进配置面 —— CORS 两键接上、跨源默认收紧（关闭 A78，新立 A80 / A81）

**一个缺陷牵出两个方向不同的洞**：L49 的映射表覆盖守护第一次跑红出 `('web', ['cors_credentials',
'cors_origins'])`（A78）—— 症状不是「配置文件没读到」，而是「同一节里只有这两个键没人读」，因为
`_load_section` 按默认表的键取字段（实测：写 `cors_origins: ["https://trusted.example"]` 读回 `['*']`，
同一文件的 `web.port: 9999` 正常生效）。顺着同一条「表 = 白名单」的思路往旁边看，`web` 节另外七个键
在 `ConfigValidator.KNOWN_FIELDS` 里也一条规格都没有。两份实测一起摆（Temp `l50q/probe2.py`，
NONCE-bee0664903ff）：

| 注入 | 结果 |
|------|------|
| `port: eighty`、`cors_origins: "https://…"`（字符串而非列表）、`cors_credentials: maybe`、`data_roots: data`（标量）、`rate_limit_max_requests: -5`、`rate_limit_window_seconds: NaN` —— **六个键同时写坏** | `is_valid=True`、0 error、0 warning |
| 只把 `data_roots` 写成 YAML 标量 `data` | `api/deps._config_data_roots` 按字符 resolve 出 `d/a/t/a` 四个根 ⇒ 数据端点**全部** 403，`config.yaml` 自身也 403 |

第二条才是这条债真正的分量：`validate-config` 绿灯的配置能把服务跑成「处处 403」，症状长得像后端坏了。

**跨源默认为什么敢改（上游形状实测）**：收紧前出厂是 `["*"]` + `credentials=True`。这个组合在 starlette
里**不是**「不开放」—— 实测（Temp `l50q/probe3.py`，NONCE-74891e9630fb / -38dd3b833cfe，同一支探针在
starlette **1.6.0 与 1.2.1 下逐字节相同**）：

| `allow_origins` / `allow_credentials` | 带 `Origin: https://evil.example` 的 GET | 同来源预检 OPTIONS |
|------|------|------|
| `["*"]` / `True`（旧默认） | 200，回显 `allow-origin: https://evil.example` + `allow-credentials: true` | **200**，两行都给 |
| `[]` / `False`（本轮新默认） | 200，**零个** `access-control-*` 头 | **400**，无 `allow-origin`（但带 `allow-methods` 与 `max-age`） |
| `["*"]` / `False` | 200，`allow-origin: *`、无凭据头 | 200 |
| 精确白名单 / `True`（非名单来源） | 200，只有 `allow-credentials: true`，无 `allow-origin` | 400 |

本仓合法消费方全都不需要跨源：随包 UI 与后端同源（前端 `baseURL` 是相对路径 `/api`、vite 开发模式走
proxy），非浏览器客户端不受 CORS 约束，而 API 不用 cookie（`set_cookie` / `request.cookies` 在 `api/`
下 0 命中，鉴权是 `X-API-Key` 头）⇒ `allow_credentials` 对合法用法零收益、纯风险。据此把默认改成
`[]` / `False`，这是 **3.x 的一次破坏性默认变更**，迁移路径写在 `config.yaml` 与 `API.md`「跨源（CORS）」。

**改了哪三处**：① `load_config` 的 `web` 默认表补 `'cors_origins': []` / `'cors_credentials': False`
⇒ A78 关闭，`TestSectionDefaultsMatchTheMappingTable.KNOWN_UNMAPPED_FIELDS` 的豁免清单**清空**（留一项
就等于承认还有一处「配了不生效」）；② `KNOWN_FIELDS` 一次性补 `web` 节 **10 条**规格（节本身 + 九个
字段，含全表第一批 `type: list`）⇒ 六键全错那份配置从「绿灯」变成逐键报错；③ 出厂默认按上表收紧，
`WebConfig` 字段 / 默认表 / `config.yaml` 三处各写一遍、三处各有一条用例用字面量钉死（沿用
`TestShippedDataRootsDefault` 的形状，不拿 `WebConfig()` 当同源预言机）。

**用例从 0 到 43**：本轮之前 `tests/` 里 CORS 断言是 **0 条**（既没测默认值也没测行为）。现在
`TestWebCorsKeysReachTheConfig` 4 例（写了必须读到 + 列表默认不许跨配置共享）、
`TestWebSectionKnobSurface` 31 例（字段/规格双向棘轮 + 16 合法 + 12 非法 + 六键结案 + 标量 `data_roots`
形状）、`TestShippedCorsDefault` 8 例（三处默认一致、中间件参数确实来自配置、跨源读拿不到头、预检 400、
同源与非浏览器客户端不受影响、上游形状锁）。参数化展开后全量净 **+43**。

**注入 8 模式（沙箱副本，NONCE-5a0c6d049d00）**：基线 252 passed / 0 failed，还原后复跑同样 252，
七个文件与真身逐字节 sha 一致。m1 映射表退回 A78（摘掉 cors 两键）⇒ **4 红**（含精确棘轮）；
m2 字段默认漂回 `["*"]`/`True` ⇒ **3**；m3 只摘 `web.data_roots` 一条规格 ⇒ **4**；m4 摘掉 `web` 节
全部 10 条规格 ⇒ **17**（其中 2 红来自 L49 立的数值/布尔计数棘轮 10→7、4→3 被打破 ⇒ 新字段自动进旧
断言）；m5 `api/main.py` 硬编码 `allow_credentials=True`（origins 仍读配置）⇒ **2**；m6 两参数全硬编码
⇒ **3**；m7 对照档：映射表 `data_roots` 默认漂成 `[]` ⇒ **3**（证明「三处一致」这套机制对同表另一个键
同样有牙，不是只对 cors 有效）；m8 三处默认漏改一处（`config.yaml` 摘掉两键）⇒ **1**。

**自纠两处，都比结论更值得留**：
① **版本号等式把测试绑死在解释器上**。本节第一版写的是 `assert starlette.__version__ == "1.6.0"`，
在 `C:\Python314`（starlette 1.2.1）上必红 —— 而 1.2.1 的 CORS 形状与 1.6.0 **逐字节相同**（上表两版
各测一次）。判据应该盯行为，不该盯版本的身份证：改成四条行为断言各自把 `starlette.__version__` 与
`starlette.__file__` 带进失败消息，红的时候自带版本信息，换解释器不再假报警。
② **「环境漂移」是自己造的**。L48 / L49 记过「pandas 与 chromadb 忽然消失、全量数从 4940/2 变 4920/3」，
本轮才定位到根因：这台机器有**两套 Python**（`C:\Python314` 与 venv `aug`），两段会话换了 `python` 而
账本没记解释器。同一份工作树双跑实测：aug（4963 passed / 3 skipped，TOTAL 12,785 语句，98.64 %）与
`C:\Python314`（4983 / 2，12,159 语句，99.02 %），Δpassed +20 = **+21**（两个 pandas 门控文件在 aug 下
不 collect）+ **1**（`test_micro_branches_l61.py:99`）− **2**（两条 chromadb「缺依赖」用例转为 skip），
Δcollect = 21 与两份 `--collect-only` 清单逐条相符。纪律：**全量数字必须连解释器身份一起记**，覆盖率
分母同样是环境的函数。

**代价（不主张任何收益）**：`validate_config` 的走查表从 20 条规格变 30 条（`web` 节 10 条），
同进程 A/B、正反序各一轮、min-of-7（Temp `l50q/ab.py`，NONCE-47fc5c23a400）：现行 4.900 / 4.929 µs vs
摘掉 web 3.331 / 3.161 µs ⇒ **Δ +1.570 / +1.768 µs，两序同号**；载体对照是整次 `validate_config(cfg)`
= 10.5 µs ⇒ 本轮新增判据占一次校验的 ~15 %，但校验只在显式 `validate-config` 调用时发生，相对
`load_config` 单次 6,859 µs 是 0.02 %。默认值改动本身的运行时代码路径 0 变化（`api/main.py` 那两个
参数早就在读配置）。

**新立两债**：A80 = `web` 节运行时零判据（SDK 直构 `WebConfig` 或绕过 `validate-config` 时，字符串
`data_roots`、`None` origins 照样流到白名单与 starlette，本轮只补了校验器这一侧）；A81 = `docs/API.md`
health 示例缺 `version` 键（文档滞后，与本轮无关，只记不修，见 `OPTIMIZATION_LOOP.md` Backlog A）。覆盖读数：`config_validator.py` 156 语句缺
**0**（78 分支 3 条偏支全在既有 `_validate_env_refs` 一侧）、`config.py` 216 语句缺 3（`457-458` / `460`
= L49 记录的 `449-450` / `452` 随本轮 +8 位移，本轮两个 hunk 在 `182-189` 与 `385`，不相交）⇒ **本轮
新增语句 0 条落入未触达**。

### 3.26 L51：判据住进配置对象自己 —— 区间搬家 + `web` 节长出 `__post_init__`（关闭 A77 + A80，新立并同修 A82 / A83）

**这一轮修的不是两个洞，而是造出这两个洞的那个形状**。A77（`max_retries ≤ 20`、`retry_delay ≤ 60`
只有校验器判）与 A80（`web` 节运行时零判据）方向相反，但根因是同一个：**区间住在 `config_validator.py`
的规格表里，配置对象自己一条判据都没有**。于是「两边各抄一遍」是这套结构的必然产物而不是某次疏忽 ——
L49 立 A77 时写的那句「刻意留的不对称」（给 SDK 加下界以外的上界属破坏性变更，要单独拍），到 L51 就是
按用户口径（安全默认值偏松时直接收紧，但破坏与迁移代价必须一并实测）正式拍板的一轮。修完的形状：八个
区间常量（`VARIANTS_PER_SEED_RANGE` / `NUM_THREADS_RANGE` / `AUTO_SAVE_INTERVAL_MIN` /
`MAX_RETRIES_RANGE` / `RETRY_DELAY_RANGE` / `PORT_RANGE` / `RATE_LIMIT_MIN_REQUESTS` /
`RATE_LIMIT_MIN_WINDOW_SECONDS`）住在 `config.py:21-28`，`AugmentationConfig.__post_init__` 与
`WebConfig.__post_init__` 按它们逐个判，`config_validator.KNOWN_FIELDS` 的 **11 条**数值规格 import
同一批常量 ⇒ A77 类分歧在结构上不可表达（改常量的那一刻，两侧同时改完）。

**改前症状全部是量出来的**（Temp `l51q/probe1.py`–`probe5.py`，NONCE-A9C41E77 / B7E42C1D9F3A）：

| 写法 | 改前实测 |
|------|----------|
| `AugmentationConfig(max_retries=10**6, retry_delay=10**6, retry_jitter=50.0)` | 构造成功、零信号；校验器对同一批值报 6 条错。按 §3.24 的等待公式那是 `999999 × 300 s ≈ 83,333 h` 的最坏预算 |
| `data_roots: data`（YAML 标量而非序列） | 按字符拆成 `d/a/t/a` 四个根 ⇒ 数据端点一律 403，症状长得像后端坏了 |
| `cors_origins: "https://api.corp.example"`（标量） | **收紧意图拿到放宽结果**：starlette 的 `origin in allow_origins` 在 `allow_origins` 是字符串时走**子串**匹配 ⇒ `https://api.corp`、`https://api` 两个别的主机被放行。starlette **1.6.0 与 1.2.1 各测一遍，形状相同** |
| `data_roots: [null]` | `api/deps.py` 的 `Path(str(p))` 把它变成一个**名叫 `None` 的白名单根目录** —— 静默可用的坏白名单比静默不可用更坏 |
| `rate_limit_window_seconds: NaN` | 窗口永不滚动 ⇒ 超阈值后**永久 429**，且 `retry_after()` 抛 `ValueError: cannot convert float NaN to integer`；写成负数则是限流静默关闭 |
| `auto_save_interval: 0` / `-1` | 与 `1` 逐字同答（`interval <= 1` 那支）⇒ 20 条样本触发 **20 次**增量存盘，默认档 10 只 2 次。两侧**都**没有判据，也**没有**校验规格 |

最后一行是本轮新立的 **A82**（不是 A77 的重复：A77 是「校验器有上界、运行时无」，A82 是「两边都没有」），
同轮修掉。**A83** 更是本轮探针自己撞出来的**镜像**症状：校验器只判「是不是 list」，不判逐元素，于是
`data_roots: [null]` 与 `host: ""` 在 `validate-config` 绿灯、在 `load_config` 抛 —— 与 A77 方向相反，
同一轮一起封住。它的修法带一条**方向约束**：`items` / `non_empty` 两个形状旗标只写在运行时真判的键上
（`web.host` / `static_dir` / `cors_origins` / `data_roots` / `rate_limit_exempt_paths` 五个），
`test_shape_flags_exist_only_where_runtime_judges` 把这两个集合逐字钉死；多写一处就会造出反向缝隙
「校验器红 / 运行时绿」，那正是本轮要消灭的东西。

**新增两条判据族成员，其中一条故意不像同族**：`require_string` 与 `require_string_list` 都
**不放行 `None`**，而 `require_count` / `require_seconds` / `require_positive` / `require_ratio`
那一族放行 —— 因为「未提供」对函数入参是有意义的（各调用点自己回落默认值），对配置字段却没有意义：
YAML 里写了键没给值就是 `None`，字段没有「没传」这种状态（`_reject_null_fields` 另判这一维，报错文案
「不能是 null（配置里写了这个键却没有给值）」）。`require_string_list` 的报错直接把 YAML 写法教给用户
（「必须是列表（YAML 里要用 `- 项` 写成序列，不能写成标量）」），因为这条判据的触发原因 100 % 是书写形状。
**元素内容一律不判**（URL 合法性、路径存在性、豁免前缀是否以 `/` 开头）—— 那是各消费点的语义，
判在这里会把「配置形状」与「业务语义」焊死，`test_content_semantics_stay_out` 是这条边界的反向护栏。

**破坏性变更的命中面在落笔前量过**：① 出厂 `config.yaml` 在双解释器下 `load_config` **0 error**
（校验器同份配置 0 错，两侧同判据后才敢这么说）；② 全仓产品代码里 `WebConfig(...)` /
`AugmentationConfig(...)` 的直构点只有 **1 处**（`api/deps.py:192`，`grep -rn` 排除 `tests/` 与定义处后
的唯一命中），而且是**无参** `WebConfig()` —— 走的正是出厂默认，新判据对默认值恒收（由
`test_defaults_satisfy_their_own_verdict` 钉住）⇒ 本轮破坏面在仓内为 0 命中，风险只剩仓外调用方；③ 用例侧因
新判据而**必须改写**的既有断言 **3 处**，且三处都是「把上界口径接进断言」的同口径收紧而非放宽
（`TestAugmentationRetryKnobs` 的 `parametrize` 补 60.0/60.1 两值、`require_seconds("retry_delay", …)`
补 `maximum=RETRY_DELAY_RANGE[1]`、数值规格计数棘轮 10 → 11 为新规格 `augmentation.auto_save_interval`
让路）；④ 失败路径的形状说清楚：`api/main.py:116` 的 `load_config` 在 import 期 ⇒ `web` 节写坏是
**进程起不来**，而 `api/deps.py:186` 那道 `except Exception` ⇒ **运行中**改坏配置是退回出厂白名单
`["data"]` 并告警（不是 500、也不是处处 403），这条形状本轮起有两条 API 用例钉住。文案面同步三处：
`config.yaml` 的 augmentation / web 两节抬头各加一句「加载时即判」，`docs/API.md` 增一行错误码口径
「配置越界不走 HTTP」并补白名单回退段。

**用例净 +143，两路独立闭合**：全量差（aug 4963 → 5106、`C:\Python314` 4983 → 5126，两边同为 +143）
与四份被触碰文件的**完整 node-id 多重集差**（HEAD 440 → 工作树 583，新增 143、**消失 0**）逐位对上；
构成 = 新定义 141 行（`TestRuntimeSectionJudgements` 17 + `TestRuntimeValidatorParity` 74 +
`TestRequireCount` 上界 17 + `TestRequireString` 12 + `TestRequireStringList` 19 +
`test_api_security` 2）+ 既有参数化扩列 2 行。`TestRuntimeValidatorParity` 是本轮的常驻对照表：
56 行 `(path, value, 该不该拒)` 同时喂给校验器与运行时构造，**逐行要求同判**，另配一条反空转断言
（表里必须既有拒也有收）与一条「数值区间的 min/max 必须来自 `config` 常量」的引用断言。

**注入 8 模式（沙箱副本 `Temp/l51q/tree/`，NONCE-c8fcb3378570）**：基线 **583 passed / 0 failed**，
八种降级红数 **32 / 4 / 9 / 14 / 5 / 2 / 6 / 2**，全还原后复跑同为 583，八个文件沙箱 ↔ 真身 sha
逐字节一致。m1 摘 `WebConfig.__post_init__`（A80 改前态）⇒ 32 红；m2 让 `_reject_null_fields` 直接返回
⇒ 4；m3 摘 `require_count` 的 `maximum` 一维 ⇒ 9；m4 摘 `require_string_list` 的「是不是列表」判据
⇒ 14（含那条 API 白名单回退用例，标量又被逐字符吃下）；m5 摘校验器 `items` 块 ⇒ 5；m6 只摘
`non_empty` 分支 ⇒ 2；**m7 是本轮最有价值的一档**：把校验器 `retry_delay` 的上界改回私有字面量 30.0
（即人为恢复「两边各抄一遍」的旧结构）⇒ 6 红，红在常量引用断言 + 逐值同判表 ⇒ 「抄两遍」这件事现在
一被抄出来就报警；m8 摘 `cors_credentials` 的布尔判据（`config.py:293-297`）⇒ 2。

**代价只报代价**（Temp `l51q/ab_cost.py`，NONCE-C3F5D18B2，同进程 A/B、正反双序、min-of-5）：
`AugmentationConfig()` 0.190 → 1.452 µs（Δ **+1.262 µs**，7.66×，反序 Δ +1.269 同号）、
`WebConfig()` 0.383 → 1.720 µs（Δ **+1.337 µs**，4.49×，反序 +1.270 同号）。两节合计每次构造
+2.6 µs，而 `load_config` 一次进程只跑一回 ⇒ 相对 7.2 ms 的一次加载约 **0.036 %**。整函数那一档
（`load_config` 7215 → 7316 µs）正反序给出 +100.8 / +76.0 µs，两序相差 25 µs 而效应只有 1 % ⇒
按 L49 立的规矩**判为不可判定、不引用**。⇒ **本轮不主张任何性能收益。**

**过程缺陷两条，第二条是计数机制本身的洞**：① A/B 脚本首版把「秒/次」按 ns 标注，三行全打成 0.0
（只有比值可读），换单位重跑（NONCE C3F5D18B → C3F5D18B2）；② 我一度按**用例名 grep** 数新增行数，
得 144 与 143 两种读数 —— 根因是 `test_omitting_maximum_keeps_the_legacy_verdict` 这个 def 名在同文件
的 `TestRequireSeconds`（HEAD，3 行）与本轮新写的 `TestRequireCount`（6 行）里**各有一个**，名字维度的
差集与 grep 都会把它读偏。新纪律：**收集数差只能按完整 node-id 的多重集算**（`--collect-only -q` →
`Counter` 差），按用例名 grep 在重名 def 下必错，且这次是「两路都对不上」才发现的。

**覆盖读数**（先测后记，双解释器）：venv `aug`（Python 3.13.14，无 pandas / 无 chromadb，starlette 1.6.0）
**5106 passed / 3 skipped**（67.52 s，总覆盖 **98.64 %**，门禁 80 % 达成，TOTAL 12,852 语句缺 88、
3,530 分支缺 110）；`C:\Python314`（Python 3.14.4，pandas 3.0.2 + chromadb 1.5.9，starlette 1.2.1）
**5126 passed / 2 skipped**（46.26 s，**99.03 %**，TOTAL 12,226 缺 41 / 分支缺 104）。三份本轮源文件：
`config.py` 256 语句缺 **3**（`556-557` / `559` = L50 记的 `457-458` / `460` 随 +99 行位移，本轮未触；
**本轮新增 40 条语句 0 条落入未触达**，含 L51 首跑唯一漏掉的 `:294` 那条 `cors_credentials` 判据 ——
它是被探针打红、被 parity 表补上的，见日志）、`config_validator.py` 166 语句缺 **0**（88 分支 3 条偏支
全在既有 `_validate_env_refs`）、`validation.py` 218 语句缺 **0**（122 分支 5 条偏支为既有）；
`C:\Python314` 侧同一批文件的语句数是 255 / 162 / 212，分母同样是环境的函数。

### 3.27 L52：给「写了没人读」装反馈通道 —— 白名单从字段类型推导，不抄第二遍（关闭 A76 + A81，新立并同修 A84–A89）

**A76 的根因与 L51 是同一个，但方向相反**。L51 消灭的是「同一区间两侧各抄一遍」，A76 是「`load_config`
按 defaults 表逐键取，表外的键无声消失」。症状面（沙箱把 `config_validator.py` 换回 **HEAD 态**复现，
Temp `l52q/post_check2.py` NONCE-45A0C1AB9510-POST —— 不再靠改前记忆下笔）：一份塞了 **8 处**错拼的配置
（`augmenation` 错节名、`augmentation.variant_per_seed` / `web.ports` / `web.cors_origin` /
`logging.levl` / `models.ernie.temperatur` 错键名、顶层 `output` / `default_model` 两个幻影段）在 HEAD 是
`is_valid=True, errors=0, warnings=0`，`augmenation.variants_per_seed=999` 与
`augmentation.variant_per_seed=999` 都读回默认 **5**；同一份配置在 L52 后是 `is_valid=True` +
**8 条 unread 警告**，逐条带「是否想写 X？」。**定级 S 的理由一直是静默**：缺陷能级看的不是影响大小，
而是有没有人被告知（A76 原文）。

**白名单必须推导出来，不能抄出来**——这是本轮全部结构决策的落点。「哪些键有人读」这份清单如果手写，
它就成了第三份副本，于是 A76 的修法本身会变成一个 A77 型缺陷（清单与代码漂移）。推导链是两跳，且两跳
都已被既有机制钉住：`load_config` 读的键集 **恰好等于** `_load_section` 收到的 `defaults.keys()`
（`test_the_derived_whitelist_equals_what_load_section_reads` 把 `_load_section` 打桩、逐节记录真实的
`set(defaults)` 再与推导集比对），而 L49/L50 立的映射表棘轮（`TestSectionDefaultsMatchTheMappingTable` +
`KNOWN_UNMAPPED_FIELDS == set()`）已经把 `defaults.keys()` 钉成 **dataclass 字段集**。所以
`ConfigValidator.consumed_section_keys()`（`config_validator.py:193`）只做一件事：遍历
`fields(AppConfig)`，取 `is_dataclass(默认值)` 的那 **20** 节、各取其字段的 **67** 个键名，结果缓存
（`_CONSUMED_SECTIONS`）。⇒ 新增配置节或字段自动进入判据，没有任何地方需要「记得同步一份清单」。

**一律 WARNING，绝不动 `is_valid`**。`is_valid` 是 `POST /api/system/validate-config` 响应的判决位
（`api/routes/system_ops.py:147` 的 `is_valid: bool`，本轮那条端点用例就断在它上面），而多余的键不让服务起不来；
把它判红等于把「配置文件里夹自己的段落」变成破坏性变更 —— `save_config`
**刻意**保留非 `AppConfig` 的顶层段落，正是这条决定让 ERROR 不可选。豁免只有两节（`META_TOP_SECTIONS`，
由 `test_exempt_list_is_exactly_the_two_meta_sections` 逐字钉死）：`app` 是谁都不读的历史元信息、
`models` 的键是**模型名**；但 `models.<名字>` 的**子项**照 `ModelConfig` 的 8 个字段判（`temperatur` 出声），
`models.default` 单独跳过。建议文案只在 `difflib` 有 ≥0.7 近邻时才附，且两个方向都有用例
（`test_suggestion_names_the_real_key` / `test_no_suggestion_when_nothing_is_close`）—— 硬猜一个不存在的
正确名字比不猜更坏。实跑输出与口径写在 `docs/API.md`「配置的「写了没人读」反馈（3.x）」，那里额外标了
「警告条数随本机环境而变」（后 5 条是既有的环境变量告警）。

**顺手拆掉校验器自己造的那个 A76 实例**：`KNOWN_FIELDS` 里躺着 `output` 与 `output.export_dir` 两条规格，
指向一个 `load_config` 从不消费的节（`hasattr(config, 'output') == False`，真实节叫 `export`；导出目录从来
不是配置项，它是 CLI/API 的 `--output` 参数）——即「反馈通道」的表自己就含没人读的键。删掉两条（全表
31 → **29**），并立一条反向棘轮 `test_every_spec_path_points_at_a_real_key`：规格表每条路径必须落在
推导出来的「真实键集」里，⇒ 幽灵规格从「可以存在」变成「一写出来就红」（注入 m6 把 `output` 加回去
⇒ 恰 **2 红**，两条都红在棘轮本身）。

**用例净 +31，两路独立闭合**：双解释器全量各跑 ——venv `aug`（3.13.14，无 pandas / 无 chromadb，
starlette 1.6.0）**5137 passed / 3 skipped**（69.16 s，总覆盖 **98.65 %**，门禁 80 % 达成，TOTAL 12,895
语句缺 88、3,558 分支缺 110）；`C:\Python314`（3.14.4，pandas 3.0.2 + chromadb 1.5.9，starlette 1.2.1）
**5157 passed / 2 skipped**（46.28 s，**99.03 %**，TOTAL 12,269 缺 41 / 分支缺 104）⇒ 两边同为 **+31**
（5106→5137 / 5126→5157），与 L51 立的「收集数差只按完整 node-id 多重集算」一致：两份被触碰测试文件
HEAD 481 → 工作树 512，**新增 31、消失 0**（Temp `l52q/collect_diff.py`，NONCE-45A0C1AB9510-COLLECT）。
skip 组成不变（aug 侧 3 条全 pandas，py314 侧 2 条 chromadb 反向用例）。**本轮新增语句全部落在已覆盖区**：
`config_validator.py` 209 语句缺 **0**、116 分支仅 **3** 条偏支（`460->459` / `466->exit` / `471->exit`，
三行都是既有 `_validate_env_refs` / `_validate_dict` / `_validate_list`，L51 已记过其中一条），新写的
`_warn_unread_keys` / `_warn_unread_model_keys` / `_suggest` / `consumed_section_keys` 分支全覆盖。

**注入 8 模式**（沙箱副本 `Temp/l52q/tree/`，NONCE-45A0C1AB9510-INJ）：基线与全还原后同为
**880 passed / 0 failed**，红数 **21 / 21 / 15 / 3 / 1 / 2 / 26 / 2**，九份文件沙箱 ↔ 真身 sha 逐字节一致。
m1 摘掉 `validate_config` 里那一遍走查（即回到 A76 改前态）⇒ **21 红**；m2 把 WARNING 升成 ERROR
⇒ **21 红**，其中一条是既有例 `test_valid_full_config_passes` ⇒ 定级口径被既有测试一起守住；m3 把推导
换成硬编码只含 `web` 一节（模拟「抄清单抄漏」）⇒ **15 红**；m4 去掉建议文案 ⇒ 3；m5 摘掉 `models` 子键
判据 ⇒ 1；m6 幽灵规格加回 ⇒ 2；**m7 是对照档**：豁免清单少一项（`models` 被当成未知顶层段落）⇒ **26 红**
—— 它比 m1 红得更多，正说明「豁免少一项」比「整遍摘掉」更毒，而它红的是既有那批「合法配置必须绿」的
用例；m8 反向证牙：往出厂 `config.yaml` 塞一个真没人读的键 ⇒ **2 红**（`test_shipped_config_has_no_unread_keys`
+ `test_the_shipped_file_actually_gets_walked`），这条是防「走查逻辑写好了但出厂文件从没被走过查」的空转。

**代价只报代价**（Temp `l52q/ab_cost.py`，NONCE-45A0C1AB9510-AB，同进程 A/B、正反双序、min-of-7，
两解释器各一份报告落盘）：出厂 `config.yaml` 全量校验 aug 侧 66.390 → 78.026 µs（Δ **+11.636 µs** /
1.18×，反序 +10.423 同号）、py314 侧 72.238 → 82.208 µs（Δ **+9.970 µs** / 1.14×，反序 +11.117 同号）；
最小配置 `{models.default}` +1.731 / +2.288 µs。**这条路径不在任何热路径上**：`validate_config` 在生产代码里
只有两个入口（`grep -rn` 坐实：`api/routes/system_ops.py:292` 与 `augmentor/cli/commands/data_ops.py:88`），
都是「用户主动要诊断」，`load_config` 与生成/服务路径一次都不调它 ⇒ 十几微秒落在诊断命令上。
推导若每次重跑会吃掉整次校验的 97 %（aug 首跑 75.7 µs / py314 104.5 µs）⇒ 缓存不是装饰，
命中后 0.0714 / 0.1417 µs 一次。⇒ **本轮不主张任何性能收益。**

**过程缺陷五处 + 一条边界披露（⑥）**：① 测试里钉了**环境相关的警告条数**（`assert len(result.warnings) == 5`）⇒ 在 pytest 环境
（`BAIDU_API_KEY` / `OPENAI_API_KEY` 已 export）只剩 2 条而假红，与 L51 钉 `starlette.__version__ == "1.6.0"`
同一类错，改成「按 marker 过滤后再判」并补一条防空转的 `test_the_shipped_file_actually_gets_walked`；
② **注入脚本的快照会悄悄吃掉 CRLF**：`open(..., encoding="utf-8")` 不带 `newline=""` 把 `\r\n` 读成 `\n`，
还原时按原样写出 ⇒ 沙箱那两份 CRLF 文件（`config_validator.py` / `config.yaml`）sha **假报不一致**，
而其余 7 份纯 LF 文件全一致 —— 症状本身就是根因。修法是快照读 `newline=""` + 补丁锚点随行尾自适应，
复跑后九份全一致；③ **`sed -i '...; e'` 清空了脚本正文**（`e` 命令把每一行当 shell 执行并打乱回写），
一次性验证脚本被毁、须重写 ⇒ 新纪律：改文件一律用 Edit/Write，`sed` 只用于只读的 `grep`/`sed -n`。
④ **本账 numstat 写成预测值，且被本行自己的三个加数否掉**：账本里先写下「7 路径 **+689 / −11**」，
可同一句里当时那三个加数 116 + 140 + 340 相加是 **596**、删行 6 + 1 + 1 = **8**，689 / 11 两个数都对不上。
L47 已为同一件事立过规矩（「文档本体与账本的 numstat 只能在该文件最后一次编辑之后量」）⇒ **第 2 次破**，
且这次是**量过之后凭记忆重写**。更难看的是：我为了写这条缺陷又往本节加了 ④⑤⑥ 三段，于是「文档堆 140/1」
在落笔改数字的这一刻再次作废 —— **本轮亲身把那条纪律的存在理由演了两遍**。修法：只报「除本账外 6 路径」
这个能在账本最后一次编辑前锁死的集合（数字见循环日志 L52「数字核对」条），而本节自己的插行数、以及
「本账写了本账自己的行数」这件事结构上不可能自洽（写对的那一刻它就变了），留给下一轮从 `git show` 量。
新增纪律一句：**凡多个数相加等于第四个数的句子，落笔前把加法做一遍。**
⑤ **一次破坏性 Edit 吃掉相邻一行**（「Edit 锚点吃掉相邻结构」这条链的第 5 次：L45 丢「；A63」→ L46 吞
`## Backlog B` 标题 → L48 装饰器错挂 → L51 吞 `test_api_security` 一行 → 本轮给「累计」段插行时把
「它跑全量时我会慢 40%+…跨轮只比」整行删掉），当场回读账本补回、未流入提交。成因始终一样：
**把「定位锚点」和「替换范围」当成同一件事**，而插一行根本不需要替换整个锚点块。
⑥ **沙箱副本少一份符号链接**：`cp -r` 进 `Temp/l52q/tree/` 时真身的 `data/versions/current -> v_20260921_234911_686071`
在沙箱里不存在（两侧 `ls -la` 对读坐实），与 L51 同类。本轮 8 模式只跑 6 份测试文件、880 条基线无一条读它
⇒ 对红数无影响，但它限定「九文件 sha 一致」这句话的射程：**沙箱 == 真身只在被注入的那 9 份文件上成立**，
不能让一次逐字节核对替整棵树背书。
另有一条纪律补强：A/B 与探针**必须落盘报告**（本轮 `ab_report_aug.txt` / `ab_report_py314.txt` /
`post_check_report.txt`），因为 L49 已经为「引用另一次运行的数字」记过一次账。

**新立 A84–A89，其中 A84 是本轮的天花板**：**A84** = 反馈只住在诊断面 —— `load_config` 与所有产品命令
（`augment` / 服务启动）**零信号**，实测拼错节后 `variants_per_seed` 仍交出默认 5（Temp `post_check2.py` ③）
⇒ 用户默认工作流里 A76 的症状原样存在，只是现在「问一句就答」。**A85** = 节写成标量（`augmentation: abc`）
时校验面报 ERROR，运行时却抛 `AttributeError: 'str' object has no attribute 'get'`（④）—— 与 A83 同族而
方向是崩不是静默。**A86** = `POST /api/config` 静默丢弃未知键仍回 `success: True`（probe4）。
**A87** = 文档与契约漂移无守卫：A81 只是抽样命中，其余端点示例未普查。**A88** = 本轮只覆盖「写了没人读」
这一根轴，类型 / 区间那一根仍有 **16 / 20** 个被消费的节零规格（`export` / `context` / `versioning` /
`sampler` / `expander` / `tracker` / `visualization` / `multilingual` / `rag` / `evaluation` / `vector` /
`multimodal` / `benchmark` / `active_learning` / `frameworks` / `logging`，全表 29 条规格只覆盖 6 节）。
**A89 是收尾复验自己撞出来的**：我原打算在文档里写「`is_valid` 驱动 CLI 退出码」，复跑 CLI 时实测到
**退出码与判决脱钩**（Temp `final_cli.py` / `final_cli_case3.py`，NONCE-45A0C1AB9510-FINAL，两解释器各一遍）——
一份缺 `models` 必需字段的配置打 `错误: 2` 却 **exit 0**，脚本与 CI 拿 `validate-config` 当门禁会永远绿；
机制在 `augmentor/cli/commands/data_ops.py:88` 的 `run_validate_config` 只 `print` 判决、从不返回码。
同一次复跑还量到第二个形状：`cli.py:46` 的 `load_config(args.config)` 在 `try` **外面**，所以配置坏到运行时
判据拒收（`web.port: 99999`）时是**裸 traceback + exit 1、stdout 全空**（`DataValidationError: web.port 必须是
不大于 65535 的整数，当前是 99999`），而 handler 里的异常走 `cli.py:49-51` 才是「`错误: …` + exit 1」——
同一个 CLI 两种失败形状，诊断命令在最需要它说话的那一档恰好不说话。**这句话本来会被写成一句错的**：
本轮四份文档（§3.27 / 本账 / `docs/API.md`）原先都写着「`is_valid` 是 CLI 退出码 + 端点判决」，
CLI 那一半被上面的实测否掉 ⇒ 就地改成只主张 HTTP 响应位，并立 A89。**这就是「收尾复验」存在的理由**，
也是本轮第三次亲身验证「报症状也只能靠 diff / 靠跑」（L49 立）。
**A81 同轮结案**：`GET /api/health` 示例补上 `version`（实跑 `{'status': 'ok', 'version': '3.0.0'}`，
`test_api_openapi_contract.py` 早已把两键钉死 ⇒ 纯文档滞后）。

**行号勘误（L53 关掉 A89 之后补，原文不删）**：上两段引的 `cli.py:46`（`load_config` 在 `try` **外**）
与 `cli.py:49-51`（handler 异常的统一文案）在 L53 之后分别位移为 `cli.py:49`（已挪进 `try` **内**）与
`cli.py:51-53` ⇒ 「同一个 CLI 两种失败形状」从此只剩一种。病历留着是对的，但读者按行号跳过去会对不上号
—— 这一处对不上号正是 **A87**（文档与代码之间没有机械联动）的现场样本，本轮靠人工回读抓到，普查仍未做（§3.28）。

### 3.28 L53：判决要能被脚本读到 —— 三条「校验形」CLI 命令接上退出码（关闭 A89，新立 A90–A92）

L52 立 A89 时的读法是「`validate-config` 只 `print` 判决、从不返回码」。本轮把它接上，顺带把整张 CLI
分发表按**同一根轴**过了一遍，量到的不是三个孤例而是一条口径缺失：仓里已经有 159 处 `assert code is None`
（通过 ⇒ 不抛 `SystemExit`）与 53 处 `assert code == 1`（失败 ⇒ 退出码 1）两套既有约定，缺的从来不是机制，
是**「判决」（verdict）和「异常」（exception）这两件事谁负责翻译成退出码**。

**本轮立的口径：校验形命令判负即 exit 1，报告形命令只出报告。**

| 形状 | 子命令 | L53 前 | L53 后 | 判据 |
|------|--------|--------|--------|------|
| 校验形 | `validate`（数据集） | 恒 0 | 判负 ⇒ 1 | `result.is_valid` |
| 校验形 | `validate-config` | 恒 0 | 报 ERROR ⇒ 1 | 同上（只按 ERROR，L52 的 WARNING 不判负） |
| 校验形 | `dependency --action validate` | 恒 0 | 有问题 ⇒ 1 | `issues` 非空 |
| 报告形（无判决位） | `audit` / `check-leakage` / `doctor` | 恒 0 | **恒 0（刻意）** | handler 内零判决词（实测，见下） |
| 报告形（**有**判决位，本轮仍未接） | `quality-report` / `auto-test` / `migrate` | 恒 0 | **恒 0（欠账 A91）** | `overall_passed` / `failed_tests` / `failed_items` 有判决语义，接不接是产品口径不是机制 |
| 已有先例 | `health-gate` | 判负 ⇒ 1 | 不变 | `augmentor/cli/commands/quality.py:205`（本轮口径即沿用此处） |

普查是脚本量的不是记忆的（Temp `l53/census.py`，NONCE-L53-CENSUS，落盘 `census_report.txt`）：
`augmentor/cli/commands/*.py` 里 **37 个 `run_*` handler**，按「handler 体内是否出现判决词 × 是否有 `sys.exit(1)`」
分格 ⇒ 带判决词且已接退出码的恰好是本轮那三条加 `health-gate`（共 4 条）；`audit` / `check-leakage` / `doctor`
三个 handler 体内**零判决词**（`audit` 报发现数、`check-leakage` 报重叠率、`doctor` 报「可选依赖 N/M 可用」），
把它们接上退出码等于新造一条「有发现即失败」的隐含契约，而审计报告的本分是**有问题时照样把报告出完** ——
所以恒 0 是决定不是遗漏。

**§3.30 的勘误（L55 实测，上面三行原文一字不改留着）**：上表「报告形（无判决位）」那一格把名字写错了
对象。「handler 体内零判决词」这条只对**handler 印出去的那句话**成立；L55 把 SDK 返回值本身量了一遍
（Temp `l55/verify.py`，NONCE-L55-VERIFY，P4 / P7 两段）⇒ `audit` 的 `report.ready`、`check-leakage` 的
`report.is_clean`、`doctor` 的 `report.all_required_present` 三个字段都已经在 JSON 里，且都是判决语义
（本机实测：`ready: false`、`is_clean: false`、`all_required_present: true`）。于是「无判决位」那一格与
下一格的真实区别不是**有没有**判决位，而是**要不要把它翻译成退出码** —— L55 给前者的答案是「默认不译、
`--gate` 才译」，与后者同批。「恒 0 是决定不是遗漏」这句仍然成立，成立的理由换了：不是「没有可判的东西」，
而是「报告的本分是出完」。这一格的名字、以及它顺带给读者的因果，才是本轮真正修掉的东西。

**但 `quality-report` 不在「无判决位」那一格里，这一条是本轮写完首稿之后复核才发现自己写错的**：
`quality.py:76` 打的「总体状态: 通过 / 未通过」读的就是 `report.overall_passed`，且命令带 `--threshold`；
`auto-test` 的 `get_test_report` 打「通过: N / 失败: M」（`auto_test.py:385-386`），`migrate` 打「失败: N 条」
（`result.failed_items`）。⇒ 三条同形状命令有判决语义却不落退出码，本轮仍**刻意不接**：要拍的不是机制而是口径
（「`overall_score` 不达阈值算不算 CI 失败」是产品决定），且 `health-gate` 读的是 `quality` 的 `pass_rate`、
**不是** `overall_score` ⇒ 两者不能互相顶替，想「质量不达标就让 CI 红」的人今天只能自己 grep stdout。
这条欠账记在 **A91 的边界 ①**（该行原来只写了 `quality-report`，本轮按普查扩成三条）。
校验形那一侧则相反 —— 它已经在 stdout 上打印「配置验证结果: 通过 / 失败」，只把这句话给人看、
不把同一个判断给进程看，是同一件事被截了一半。

**两处 `sys.exit(1)` 与一处 `try` 范围，同批不可拆**：`data_ops.py:76-78` / `:112-115` 与
`ops.py:126-127` 各是一行判负；`cli.py:46-53` 把 `load_config(args.config)` 从 `try` 外挪进来。
第二条不是顺手改的：A89 的两半如果只修退出码，那么 `web.port: 99999` 这一档（配置坏到运行时判据拒收）
仍然是**裸 traceback + stdout 全空**，诊断命令恰好在最需要它说话的那一档沉默；只修 `try` 范围则判决照旧
不落退出码。两半一起修才有「CLI 的失败只有一种形状」这句话可主张。实测形状（Temp `l53/probe.py`，
NONCE-L53-A89PROBE，两解释器各一遍、报告落盘 `probe_report_aug.txt` / `probe_report_py314.txt`）：

```
A 仓库配置          exit 0  stdout: 配置验证结果: 通过 / 错误: 0, 警告: 5, 信息: 0
B 只有拼错节         exit 0  stdout: 配置验证结果: 通过 / 错误: 0, 警告: 1     ← L52 的 WARNING 通道不判负
C 报 ERROR 的配置    exit 1  stdout: 配置验证结果: 失败 / 错误: 1, 警告: 0
D 加载器拒收         exit 1  stderr: 错误: web.port 必须是不大于 65535 的整数，当前是 99999（stdout 空、无 traceback）
E 干净数据          exit 0  stdout: 验证结果: 通过 / 总数据: 2, 有效: 2
F 脏数据            exit 1  stdout: 验证结果: 失败 / 总数据: 1, 有效: 0
G 无关命令仍正常分发  exit 0（usage 正常打印，判负分支没有渗到别的子命令）
```

C 与 D 现在**同形**（都是 exit 1），区别只在信息落 stdout 还是 stderr —— 这正是本轮要的形状：
退出码读判决，stderr 读原因。D 那档在 L52 是 traceback，本轮之前一直被当成「exit 1 所以没问题」，
本轮量到的是「脚本能判负，但人读不到话」，两个维度都得对。

**破坏面实测（先量后改）**：① 仓内 CI 零影响 —— `.github/workflows/ai-platform-ci.yml:60` 只有
`python -m compileall -q augmentor api cli.py` 加 pytest，全仓 `*.yml` / `*.bat` / `*.sh` 里
`validate-config` 的命中只在文档与本文件（`grep -rln` 排除 `node_modules`）⇒ 没有一处脚本把它当门禁用，
本轮改动不会让任何既有流水线换颜色；② 测试面只有 **1 处**既有断言与本口径冲突
（`test_cli_dataset_tools.py::TestValidateCommand::test_validate_reports_invalid_data` 钉的是
「脏数据 ⇒ `code is None`」）⇒ 本轮唯一一次既有断言改写，方向是收紧不是放宽；③ 用户侧（仓外脚本）
**未量**，诚实留账 —— 这也是 A89 当初被标成「M 级但带破坏性」的原因。

**注入 5 模式（Temp `l53/inj.py`，NONCE-L53-INJ，沙箱副本 `l53/tree/`）**：基线 **99 passed / 0 failed**
（3 份被触碰测试文件的定向选择）。I1 摘掉 `validate` 的退出码 ⇒ **1 红**、I2 摘 `validate-config` 的 ⇒ **1 红**、
I3 摘 `dependency validate` 的 ⇒ **1 红**、I4 把 `load_config` 挪回 `try` 外 ⇒ **1 红**，
**四个红集两两不相交** ⇒ 一行守卫各自都有人守，不存在「删掉任一行都还有别人兜着」的冗余；
I5 是**反向劣化**（`validate-config` 有 WARNING 就判负）⇒ **3 红**，其中一条是既有例
`TestValidateConfigCommand::test_repo_config_is_valid` ⇒ 「过度收紧」与「没收紧」同样有人报警，
这条与 L52 的 m7（豁免少一项比整遍摘掉更红）是同一族判据的两次兑现。全部还原后复跑 99 passed，
快照与真身逐字节一致。

**用例净 +4（双解释器各自闭合）**：aug 5137 → **5141**（3 skipped，98.65 %，TOTAL 12,900 语句缺 88 /
3,562 分支缺 110）、py314 5157 → **5161**（2 skipped，99.03 %，12,274 缺 41 / 3,562 缺 104）⇒ 两边同为 +4，
skip 组成与 L51/L52 逐字相同。语句总数 +5、缺数与 L52 一字不差（88 / 41）⇒ 新增语句两边全绿；
三份被改源文件单读：`cli.py` 22 语句缺 0、`cli/commands/data_ops.py` 88 缺 0、`cli/commands/ops.py` 102 缺 0。
新增两处 `if` 的**两个方向**各有用例（判负 ⇒ 1 / 通过 ⇒ 不抛），并由 I1–I4 摘任一行即红、I5 反向即红背书。
**性能不主张任何方向的收益**：`try` 范围从 1 行变 4 行，同进程正反双序 A/B 给 +1.7 ns / −1.2 ns（Temp
`l53/ab_try.py`，NONCE-L53-AB）⇒ 两序变号，按 L49 的规矩判为不可判定、不引用。

**新立 A90–A92，三条都是本轮做走查时被量出来的**：**A90** = 判负那条支在**纯 CLI 世界不可达** ——
`dependency` 的四个 action 里没有任何一个能造依赖边（`add_dependency` 只在 `augmentor/dependency.py:186`
与模块级 `:335`，`grep -rn` 坐实 `cli.py` / `augmentor/cli/` / `api/` **0 命中**）⇒ CLI 用户跑
`dependency --action validate` 永远只会看到「没有注册的数据集」或全通过；本轮 I3 那条用例是**绕道 SDK**
（`DependencyManager(...).add_dependency("src", "ghost", ...)`）才造出可判负的输入。
**A91** = 本轮新立的「校验形 ⇒ exit 1」口径**本身没有守卫**：今天没有第 4 条校验形命令，明天加一条就会漏；
守卫的正解是**从 `COMMANDS` 分发表推导**（列出全部子命令 × 各自是否判负），不是手抄一份清单 —— 抄清单正是
L52 刚为配置面白名单消灭的那一族缺陷。**A92** = **空数据集恒判「有效」**（三层同形，实测：
Temp `l53/empty_report.txt` 的 CLI `validate --preset basic/strict/chat` 全部 `exit 0` +
「总数据: 0, 有效: 0」；SDK `validate([])` `is_valid=True`；`POST /api/dataset/validate` 空文件 →
200 `{'is_valid': True, 'total_items': 0, ...}`，Temp `l53/api_empty2_report.txt`）。
根因是 `validation.py:437-442` 的 `is_valid = valid_count == len(items)` —— `[] == []` 成立，
于是 0 条数据**空洞地**通过。它本轮值得记是因为**新退出码把一个既有的空洞判据变成了可见的门禁风险**：
一份被上游写空的文件以前是「静默通过」，现在仍然是「静默 exit 0」，脚本作者会以为 0 代表验过。

**过程缺陷披露四处，其中一处是本轮自己的记账数字**：① 本账 A89 行原本写「测试面钉了 **165 处**
`assert code is None`」—— 那是 `grep -rn "code is None" tests/` 不加 `--include` 的读数（混进 `*.pyc`
与 prose 引用）。复测三种口径：真断言（行首缩进 + `assert code is None`）**159 处**、纯 `grep -F`
命中 160 行（多的那 1 行是本轮新用例 docstring 里的字面引用）、`assert code == 1` 真断言 53 处。
⇒ 文档与账本一律改用 159 / 53 并**连 grep 口径一起写**，这是 L51「名字维度差集会读偏」那一族的第三次命中：
**凡引一个 grep 数，必须同时写下那条命令**。② 一份探针报告是 GBK 乱码（`l53/empty_report.txt` 用 shell
`>` 重定向接 CLI 的控制台输出，而 Windows 控制台码页不是 UTF-8）⇒ 读数（`exit=0` / 「总数据: 0」）仍可用，
但这条又应了 L52 立的「落盘要脚本自己按 UTF-8 写，不要靠重定向」。③ **Edit 工具会把工作树文件行尾归一**：
`test_cli_stream_migrate_commands.py` 原本是 MIXED（3 行孤 LF 头 + 其余 CRLF），本轮插用例后那 3 行被
写成 CRLF ⇒ 工作树变纯 CRLF。`git diff --numstat` 里**看不见**（`core.autocrlf=true`，入库统一成 LF），
但磁盘上确实动了 3 行 ⇒ 不能写「0 处行尾归一化」，只能写「1 份工作树归一化、提交侧无差异」。
④ 探针**猜 API 路径猜错两次**（`/api/validate`、`/api/validation` 都 405，`/api/dataset/validate` 少
`input_file` 字段 422），第三版才用真实临时文件拿到 200 ⇒ 「凭形状猜请求」与「凭形状猜配置输入」
（L52 的 ⑦）是同一个错的两次现形，A87 的普查欠账继续滚。

### 3.29 L54：把「写了没人读」从诊断面接到产品面（`load_config` 出声）+ 空配置文件的档位（关闭 A84，自立并同修 A94，新立 A95–A97）

**先量症状，再动代码**（Temp `l54/probe_a84.py`，NONCE-L54-A84PROBE，两套解释器各一遍）。同一份「拼错一
节 + 拼错一键」的配置：诊断面 `validate_config` 报 **2 条 WARNING**（`键没人读取: augmentation.variant_per_seed…`
与 `… models.ernie.temperatue …`），产品面 `load_config` 交出 `variants_per_seed=5`（写了 999）而
`cli stats` **`exit=0`、stdout 正常、stderr 全空**，`cli validate` 同形 ⇒ L52 立的通道只住在诊断命令里，
用户要主动问一句才答。探针顺手撞出 **A94**：0 字节或只含注释的 YAML 解析成 `None`，`load_config` 里第一处
`'models' in raw_config` 直接抛 `TypeError: argument of type 'NoneType' is not iterable`（aug 3.13）/
`… is not a container or iterable`（3.14.4）⇒ CLI 给出的是**这句无法行动的原文** + `exit 1`，而校验器对同一
份文件干净地报 `is_valid=False, errors=1: 配置必须是字典类型`。

**通道选择是三条否决而不是两条**：① **不写 stdout** —— 每个产品命令的 stdout 是它的输出契约（`stats` 是
JSON、`validate` 是中文判决），夹一行自由文本等于把 L53 刚钉住的「一档 stdout 全空」那类断言全部作废；
② **不进退出码** —— 承 L53 口径「**只有 WARNING 不判负**」，否则「配置里夹了自己的段落」的用法凭空变红
（L52 的 m2/m7 注入就是为这条付过红数的）；③ **不选 `print` 而选 `logging.warning`** —— API 进程在 `api/main.py:42-45` 已 `basicConfig`，走 logging
才与它同一条出口、同一种格式；纯 CLI 进程未配置 logging 时由 `logging.lastResort` 落 stderr，两条路都不碰
stdout。**但「用户能不能用配置把它静音」这一问本轮答不了**：`api/main.py` 那次 `basicConfig` 的
`level=logging.INFO` 与 `format=` 是**硬编码**，`logging` 节（`level` / `file` / `format`）三键在生产代码里
**零应用方** ⇒ 见 A97。出厂 `config.yaml` **零命中**（实测 `cli stats` 的
stderr 为空）⇒ 正常一次运行**一行都不多**，只有拼错才出声。

> **L57 勘误（上面那句「本轮答不了」已被 L57 答完，原文一字未删）**：A97 已结案，三键从
> L57 起有消费方（`augmentor/logging_setup.py`，由 `load_config` 在**节真的写出**时调用）。
> 那一轮留的两个不确定性各自有了实测答案：`api/main.py` 那次硬编码 `basicConfig` **一字未动**
> （装配只 refine 已存在的 handler，且「没写出的键不动」，否则「只想加个日志文件」会顺手把
> API 的全部 INFO 日志压回 WARNING）；`logging.lastResort` 兜底那条路也**没被改掉**，默认值就是
> 按它的形状写的（WARNING + `%(message)s` + 无 handler），所以 14 条命令逐字节不变。
> 静音旋钮从此存在：`logging: {level: ERROR}` ⇒ 107 B / 1 行 变 0 B / 0 行。详见 §3.32。

**同源而不是抄写（A77 那一族的正解）**：新增 `config_validator.unread_key_messages(config)`，内部调的正是
`validate_config` 用的同一个 `ConfigValidator._warn_unread_keys`；比对两条通道时不许各抄一份文案，于是把
那句文案的共同词落成常量 `ConfigValidator.UNREAD_MARKER = "没人读取"`，用例用**同一个常量**过滤
`validate_config(...).warnings` 并与 `unread_key_messages(...)` 逐条相等（`tests/unit/test_config_unread_feedback.py::TestTheTwoChannelsShareOneSource`
⇒ 等式不成立即红）。`config_validator.py` 顶部已 `from .config import …`，所以 `config.py` 里那句
`from .config_validator import unread_key_messages` **必须留在函数体内**，提到模块顶部就是循环 import。
**刻意只出「没人读」这一件事**：环境变量引用那一路（`环境变量未设置: X`）不进加载声 —— 它的条数是本机 shell
的函数（L52 纪律），接进来等于让同一份配置在不同机器上输出不同行数；这一条由
`test_env_reference_findings_stay_out_of_the_load_noise` 反向钉住（对照组先断言诊断面**必须**有那条）。

**A94 的修法是一行 `or {}`**，先例就在同模块 `save_config`；语义依据是「什么都没写」与「路径不存在」应当
同档（后者一直是全默认）。用例**不断言那句 TypeError 原文** —— 两个解释器的原文不同（上面逐字抄了两种），
断言它就是把测试绑死在解释器版本上（L50 立的「环境相关数字/文案不进断言」）。

**代价量到 µs**：加载时多走一遍全树键名走查。同进程 A/B、正反双序各一遍（Temp `l54/verify.py` V7，
NONCE-L54-VERIFY）：`load_config(出厂 config.yaml)` aug 跳过 6,755.2 / 6,913.7 → 含扫描 6,872.7 / 6,945.4 µs
⇒ **+74.6 µs（1.09 %）**；py314 6,660.9 / 6,900.3 → 6,830.2 / 7,024.8 ⇒ **+146.9 µs（2.17 %）**。**两序同号、
两解释器同号 ⇒ 本轮判为可判定**（与 L53 那对 +1.7 / −1.2 ns 的变号读数不同档），并如实写成代价而不是收益。
API 侧不重复付：`deps._config_data_roots` 已按 `(路径, mtime_ns)` 缓存，每请求不再重解析 YAML。

**注入 6 模式 = 3 / 3 / 1 / 4 / 4 / 0 红**（Temp `l54/inj.py`，NONCE-L54-INJ，定向选择新用例文件 13 例，
两解释器**逐条同红集**，还原后 `sha256` 与快照一致）：I1 摘掉 `load_config` 里的调用 ⇒ **3 红**（三条
「出声」用例，含那条子进程真验 stderr 的），I2 把出声改道 `print` ⇒ **同样 3 红**（说明「走 stdout」这条路
已被契约用例守住，且守它的不只一条），I3 把整份校验结果（含环境变量那一路）塞进加载声 ⇒ **1 红且只有那 1 红**
（最外科的一条，证明 env 用例不是顺带红的），I4 退回 A94（去掉 `or {}`）⇒ **4 红**（三条参数化 + 一条空文件
静音），I5 让「有 unread 就 `sys.exit(1)`」⇒ **4 红**（既有例 `test_it_is_only_a_warning_loaded_values_stay_default`
也在守这条口径），**I6 无害对照**（只加一行注释）⇒ **0 红 / 13 passed** ⇒ 前五档的红数全部归因于判据本身。

**用例 +13 两边各自闭合**：aug 5,141 → **5,154**（3 skipped，66.39 s）、py314 5,161 → **5,174**
（2 skipped，49.21 s），覆盖 TOTAL aug 12,900 → **12,913** 语句而缺数 **88** 一字不动、py314 12,274 → **12,287**
而缺数 **41** 一字不动 ⇒ 本轮新增语句两边全绿。分支总数 3,562 → **3,564**（`_log_unread_keys` 那一条 `for`
的两个方向；`or {}` 与函数内 import 不产生行级弧），分支偏支 **110 / 104** 同样一字不动。单读两份被改源文件：
`config.py` 263 语句缺 **3**（`585-586` / `588` —— 就是 L49 起既有的那 3 条，因本轮在文件上方插了 29 行而整体
位移，非本轮新增）、`config_validator.py` 215 语句缺 **0**，116 分支 3 条偏支且三行全是既有代码。6 条 warnings
两边同数且全是第三方与既有收集告警（starlette testclient / anyio / `auto_test.py` 的 dataclass），本轮 **0 新增**。

**新立三条，都是本轮读数直接给的**：**A95** = **`${ENV}` 占位符只有 `models` 节的三个字段真的解析**
（`grep -rn --include=*.py "resolve_env" .`（排除 `Temp/`）命中 **6** 行 = `def _resolve_env` + 内层
`def resolve_env` + `return _resolve_env` + 三处调用 `api_key` / `secret_key` / `base_url`），其余各节一律不解析
⇒ 实测 `augmentation.variants_per_seed: ${NOT_A_REAL_ENV_VAR_L54}` 在产品面是
`错误: augmentation.variants_per_seed 必须是整数，当前是 '${NOT_A_REAL_ENV_VAR_L54}'（str）` + `exit 1`
（文案指向**字段类型**而不是「环境变量没设」，用户看不出根因），而诊断面 `_validate_env_refs` 是全树递归的
（`config_validator.py:449-467`）⇒ 同一个占位符两面的**支持范围不一致**；另一半是同函数在环境变量未设置时
**静默回落 `''`**（`:356` `os.environ.get(env_var, '')`）⇒ models 的 `${UNSET}` 变成空密钥，症状推迟到调用
模型那一刻。`_resolve_env` 在 `tests/` 里 **0 命中**（同一条 grep 的 `tests/` 侧）⇒ 整条通路无守卫。
**A96** = **本轮把 A94 修完之后，「空配置文件」在两面的判决第一次对立**：产品面 = 出厂默认 + `exit 0`，
诊断面 = `is_valid=False` + 「配置必须是字典类型」（P1/P3 实测）⇒ 该统一成哪一边是**产品口径**（空配置是
「合法的默认」还是「无效的配置文件」），不是机制，与 A92 同类 ⇒ 先记后问。
**A97** = **`logging` 节三键（`level` / `file` / `format`）写了没人听**，且它正是本轮这条出声通道的静音旋钮：
`grep -rn --include=*.py "config\.logging\|\.logging\.level\|\.logging\.file\|\.logging\.format\|LoggingConfig" augmentor/ api/ cli.py tests/`
命中 **3** 行，全部住在 `augmentor/config.py` 自己体内（`:311` 类声明、`:342` 字段、`:519` 映射表条目），
`api/` / `cli.py` / `tests/` **0 命中** ⇒ 没有任何一处把这三键应用出去；`api/main.py:42-45` 的
`basicConfig(level=logging.INFO, format="…")` 是硬编码。于是 `logging: {level: ERROR}` 既不静音本轮的
「键没人读取」，也不改 API 的实际级别，而 `logging.file` 对应的「写日志到文件」这件事从来不存在。
**这条是 L52 那套白名单的天花板而不是它的漏网**：`_warn_unread_keys` 的权威是「`AppConfig` 的字段集 ==
`load_config` 读走的键集」，而这三键**确实被读进了对象** ⇒ 判据在定义上看不见「读进对象之后没人用」。
两档修法：① 把 `logging` 节真接上（`basicConfig` 读它，`file` 非空时挂 `FileHandler`）—— 会改默认档的日志
形状，破坏面要先量（现有测试对 stderr 的断言全是内容式、无 `filterwarnings`，实测 0 处断空 stderr）；
② 机制级：把「键有没有生效面」也纳入判据（对每个 `AppConfig` 字段查消费方，零消费方即出声），代价与假阳
都要先普查再拍。定级 M：**本轮把出声通道建在了这块地上**，所以 A84 与 A97 必须同屏读 —— 「有通道」不等于
「用户能管住它」。

**过程缺陷披露五处**：① **注入脚本第一版 6 条锚点里 5 条脱靶**，只有不带换行的那条命中 —— 原因是
`augmentor/config.py` 在工作树里是**纯 CRLF**（610 处 `\r\n`、0 处孤 LF），而我把锚点按 `\n` 拼 ⇒ 任何跨行
锚点都对不上。更糟的是我**先用 `cat -A` 看过同一份文件、读到的是 `$` 而不是 `^M$`**，于是判定它是 LF 就动手
⇒ 管道里的 `grep` 已经把 `\r` 吃掉，`cat -A` 不是行尾的可靠读数。新纪律：**判行尾只按字节计数，注入前先探
`NL = "\r\n" if "\r\n" in original else "\n"` 并按文件自己的行尾拼锚点**（第二版 6/6 命中、还原 sha 一致）。
顺带量齐四份主文档/源文件的行尾实况：`config.py` 610 CRLF、`config_validator.py` 549 CRLF、`docs/API.md`
973 CRLF、`README.md` 389 CRLF、`docs/ARCHITECTURE.md` **纯 LF**（这一项**刻意不钉自己的行数**：本行正写在这份
文档里，再改一次那句数就过期 —— L40 / A93 那族「自指计数会过期」的现场，终态行数交给收尾自查与
`git diff --numstat` 各量一遍）、新用例文件 223 纯 LF（`tests/unit/`
190 份里 150 MIXED / 23 LF / 17 CRLF ⇒ 新建文件两种都有先例）。② **V8 那一档的 `rc=1` 我第一版读成了
「`load_config` 拒收未设置的环境变量」**，据此差点把 A95 写成「静默回落空串」这一条错误结论；复核那次跑的
stderr 原文（`必须是整数，当前是 '${…}'（str）`）才发现根因是**占位符压根没被解析**，两回事 ⇒ 「同一个读数
可以有两种机制」的第四次现形（L49 立、L52 ①、L53 ④、本轮 ①），修法是回到原文与 grep 计数，不是回到推断。
③ **本轮第二次「回读自己刚写下的句子」才没把一句错话留在文档里**：③ 号否决那句我写的是「走 logging 才跟着
**用户的 `logging.level`** 走」，写完复核这句时去 grep 那三键的消费方，才看见 `api/main.py:42-45` 的
`basicConfig` 是硬编码、`logging` 节零应用方 ⇒ 那句「跟着用户的 level 走」当时就改成了「与那次 `basicConfig`
同出口」，并把这块地登记成 **A97**。**如果我没有回头读自己写的那句，它会一直留在 §3.29 与 §6 口径表里**，
而它是那种读起来完全合理的句子 —— 与 L53 的 ⑥（`quality-report` 那条口径写过头）同一族，链条：L52 ⑦ →
L53 ⑥ → 本轮 ③。④ **一次破坏性 Edit 在本文档凭空造出一个标题**（`### 4.0 一句话`）：给 §3.29 定位插入点时
把我要**新写**的内容前错当成了既有结构 ⇒ 当场回读发现、撤销，并复验文件回到改动前的行数与字节数才继续
（「Edit 吃掉或造出相邻结构」链条的第 7 环，也是**凭空造出**而不是吃掉的第一次，同一只坏手的两个方向）。
⑤ **`UNREAD_MARKER` 交付时产品侧读者为 0**：立它时注释写着「不是两处各抄一份字符串」，可三条文案模板全是
字面量、只有测试读它（`grep -rn "UNREAD_MARKER" --include=*.py .` 只命中定义 + 测试两行）⇒ **本轮新立的 A97**
（读进对象却没人消费）与 L45 的「只有测试在调用的死助手」在我自己刚写的代码上第二次现形，只是这次对象是一
个常量而不是一个助手。改掉之后（`:251` / `:263` / `:284` 三条模板全部从常量取词）输出文案逐字节不变：定向两份
测试文件两边同为 **263 passed**、注入 6 模式红集与改前**逐字相同**（3 / 3 / 1 / 4 / 4 / 0，两解释器各一遍）、
`docs/API.md` 那段实跑转录件的 stderr sha 前缀两侧仍一致、双解释器全量各复跑一次仍是 **5154 / 5174**。
新纪律：**新立常量或助手在交付前 grep 一次「产品侧读者数」，为 0 就当场接上或删掉**；本条与前四条的差别在于
它是**收尾自查那次 grep 抓的，不是写代码时抓的** —— 缺陷 ⑤ 与 A97 是同一根的第二面：A97 说的是「键被读进对象
但没人用」，⑤ 说的是「常量被定义出来但只有测试用」，两者都不是白名单能看见的形状。

### 3.30 L55：判决只走一个出口 —— 六条报告形命令长出 `--gate`，顺带修掉两起编码崩（关闭 A91，新立 A98–A100）

A91 的症状写着「口径只活在文档里，零守卫」。本轮开工第一件事不是写守卫，而是把 L53 那张表**重测一遍**
—— L53 自己就在同一张表上错过一次（首稿把四条有判决位的命令一并写成「无判决位」）。量到的三件事，
每一件都改变了本轮要写的代码：

**① §3.28 的「无判决位」那一格确实写错了对象**（已在原表下就地补勘误、原文一字未删）：
「handler 体内零判决词」只对**handler 印出去的那句话**成立，而 SDK 返回值里的 `ready` / `is_clean` /
`all_required_present` 三个字段一直就在 JSON 里，且都是判决语义。⇒ 那一格与下一格的真实区别不是
**有没有**判决位，而是**要不要把它翻译成退出码**。

**② 「报告形命令恒 0」不等于「不接退出码」，它缺的是一档开关**。L53 反对的是「用户没要求就判负」这条
隐含契约（有发现即失败），不是反对判负本身。⇒ 接线做成显式一档：**默认恒 0 一字未改，`--gate` 才把
判决位翻译成 1**。这样既关掉 A91 的边界 ①（「想让质量不达标时 CI 红的人只能自己 grep stdout」），
又不推翻 §3.28 刚立的口径。

| 命令 | 判决位（SDK 字段） | 默认 | 带 `--gate` | 接线位置 |
|------|--------------------|------|-------------|----------|
| `quality-report` | `report.overall_passed` | 0 | 未通过 ⇒ 1 | `cli/commands/quality.py:96` |
| `audit` | `report.ready` | 0 | 未就绪 ⇒ 1 | `cli/commands/security.py:76` |
| `check-leakage` | `report.is_clean` | 0 | 检出泄漏 ⇒ 1 | `cli/commands/security.py:52` |
| `doctor` | `report.all_required_present` | 0 | 缺必需依赖 ⇒ 1 | `cli/commands/ops.py:31` |
| `auto-test` | `suite.failed_tests == 0` | 0 | 有失败用例 ⇒ 1 | `cli/commands/ops.py:54` |
| `migrate` | `result.failed_items == 0` | 0 | 有失败条目 ⇒ 1 | `cli/commands/ops.py:159` |
| 已有四条改走同一出口 | `is_valid` / `issues` / `gate.passed` | — | 判负 ⇒ 1（无 `--gate`，恒判） | `data_ops.py:78` / `:114`、`ops.py:134`、`quality.py:210` |

**唯一出口 = `augmentor/cli/verdict.py:verdict_exit(passed, enforce=True)`**。退出码从此只有三种形状，
彼此不许互相顶替：`0` = 判决通过或这条命令压根不判负；`1` = 判决未通过（只由 `verdict_exit` 产生）**或**
handler 抛异常被 `cli.py:main()` 翻译成 `错误: …` + `1` ⇒ 同样是 1，**靠 stderr 有无「错误:」分「判负」与
「崩溃」**；`2` = argparse 用法错误。这个二义性本轮刻意没动（改异常档要动对外契约，记在 A100 旁边）。

**③ 两起编码崩：一起是既有的、一起是本轮自己造的** —— 这是本轮真正想记下来的一段。

- **既有**：GBK 管道（子进程 stdio 不带 `PYTHONIOENCODING` 时实测就是 `gbk`）下逐条跑 16 条命令 ⇒
  **恰好 2 条崩**：`quality-report` 与 `auto-test`（Temp `l55/gbk.py`，NONCE-L55-GBK）。崩的形状是
  `✅`（U+2705，不在 GBK）让 stdout 写到一半抛 `UnicodeEncodeError` ⇒ 改前 `quality-report` 只印得出
  6 行（半张报告）且 `--output` 的报告文件**根本没写**、`auto-test` stdout **0 字节**，两条都是 **rc=1**。
  ⇒ 「CI 读到判负」其实是「CI 读到了 emoji」：判决码被一个装饰符占掉。改成 `[通过] / [未通过]`（指标行）
  与 `[通过] / [失败]`（auto-test 报告行）后：`quality-report` stdout 6 行变 **16 行**、报告文件 0 →
  **1547 字节**；`auto-test` stdout 0 字节变 **24 行**。字节数只在同一份语料同一次运行内可比，
  两解释器之间那 8 字节差来自报告里的时刻 ⇒ 不进任何断言。
- **本轮自造**：我给 6 条命令写的 `--gate` help 文案里用了 `⇒`（U+21D2，同样不在 GBK）⇒
  `python cli.py quality-report -h` 在 GBK 管道下 **rc=1、stdout 0 字符**（Temp `l55/helpgbk.py`，
  NONCE-L55-HELPGBK）。抓到它的正是本轮给自己新写的那条守卫（AST 扫 parser 里全部 help 常量）——
  **一小时前立的判据，第一次跑就命中我自己刚写的行**。文案改成「未通过时退出码 1」后复跑：五条
  抽样 `-h` 全 **rc=0 且 stdout 有内容**。改后全仓 CLI 面向模块（10 个 handler 文件 + `parser.py` /
  `io.py` / `verdict.py` / `auto_test.py` / 根 `cli.py`）的字符串常量非 GBK 计数为 **0**
  （Temp `l55/gbkconst.py`，NONCE-L55-GBKCONST）。

**守卫从推导、不抄清单**（A91 的修法原话）：`tests/integration/test_cli_verdict_wiring.py`（新，30 例，
两解释器各 **30 passed**，6.04 s / 6.18 s）钉三条集合等式 ——
S1「parser 里声明 `--gate` 的命令」== S2「handler 源码里出现 `enforce=args.gate` 的命令」（各 6 条）；
S3「handler 源码里出现 `verdict_exit(`」== S4「子命令 help 里含『退出码』」（各 10 条）；
S5「L53 那四条」⊆ S3。改前基线（Temp `l55/verify.py`，NONCE-L55-VERIFY）：S3 = 空集、S4 = 3 条、
`dependency` 接了判决却不宣称 ⇒ **双向不等的两种形状各命中一处** ⇒ 这两条等式不是同义反复。
另加 4 格参数化的 `verdict_exit` 单元、13 条行为用例（默认恒 0 / 带旗判负 / 通过档仍 0 /
无判决位命令传 `--gate` ⇒ **2** 而非 1），以及 help 文本 GBK 可编码 + 子进程 GBK 管道实跑。

**注入 7 模式**（Temp `l55/inj.py`，NONCE-L55-INJ，两解释器红集**逐字相同**、只有耗时差，
还原后 `parser.py` / `quality.py` sha 前缀 `68e3ff6d8b5ca4e8` / `58cc1fe18c4c00e9` 与快照一致）：
I1 摘掉 `quality-report` 的 `--gate` 声明 ⇒ **6 红**（两条集合等式 + 三条行为 + 一条 GBK 实跑）；
I2 handler 忘传 `enforce`（默认档被改成恒判负）⇒ **3 红**；I3 摘掉整条接线 ⇒ **4 红**；
I4 把 `✅/❌` 装回来 ⇒ **1 红**，且**只有**子进程那条红（进程内 `redirect_stdout` 不落编码 ⇒
这条就是「必须真起子进程」的理由）；I5 help 不再宣称退出码 ⇒ **1 红**（S3≠S4）；
I6 help 里塞回 `⇒` ⇒ **2 红**（AST 判据 + 子进程 `-h`）；I7 无害对照（只加一行注释）⇒ **30 passed 全绿**。
红集互不相同 ⇒ 没有冗余守卫；I2 / I5 / I6 是**反向劣化**（改得「更严」或「更好看」）也有人报警。

**破坏面**：`--gate` 是新增可选旗标 ⇒ 不传即旧行为（默认恒 0 一字未改，I2 就是「改掉默认档」的劣化，
3 红）；被改的两个字符（`✅/❌` → `[通过]/[未通过]`、`[失败]`）实测 `tests/` 里 **0 处**断言引用；
help 文案变化不进任何断言。**仓外脚本未量**（与 A89 同一条诚实账）。

**性能不主张任何方向**：`verdict_exit` 是每条命令末尾的一次函数调用，落在进程启动与 IO 的噪音底下，
本轮不做 A/B、也不引用任何 µs 数。唯一可量的变化在行为面：GBK 管道下两份报告从「没写出来」变成
「写全了」—— 那是修缺陷的收益，不是代价。

**覆盖表那两列的换算本轮纠正了两次**：aug 全量 TOTAL 的分支列从 L54 的 **3,564** 变成 **3,560**（−4），
同一行的语句列是 +17。先用 AST 数 `if`：删 3 处 `if …: sys.exit(1)`、`verdict.py` 新增 1 处 ⇒ 净 **−2**，
对不上；再用「弧总数的一半」算 ⇒ 得 **+7**，连方向都反了，而那只公式在**本轮没碰的文件上就已经错**
（`pipeline.py` 数出 143 对，报告列写着 78）。第三次直接调 coverage 自己的 `PythonParser` 才对上：
**Branch 列 = 源行有多个后继的弧条数**（双向各计一次，顺序弧不计入），在 `pipeline` / `preview` /
`privacy` / `quality` 四个文件上与报告列逐位相同（78 / 24 / 22 / 62）。按这个换算重算本轮 7 个文件：
`data_ops.py` **−4**、`quality.py` **−2**、`verdict.py` **+2**、其余四份 **0** ⇒ 合计 **−4**，与读数闭合；
机制就是 `if` 换成 `verdict_exit(布尔实参)` —— 判决还在、分支点没了，这正是「唯一出口」要的形状，
只是它在覆盖表上表现为**分支总数下降**。写下这段是为了下一轮：谁把 −4 读成「测试变弱了」都会读反。

**新立三债**：**A98** —— 本轮只把**会崩的字符**换掉，没碰根因（CLI 的 stdout 仍按 locale 编码），
数据集里只要有一条含 emoji 的样本，`quality-report` / `audit` 这类命令在 GBK 管道下仍会整条崩成 rc=1，
与判决码同值；根治要把出口编码钉成 UTF-8，而那会改「中文输出重定向到文件的字节契约」（下游按 GBK
读的脚本会花），是对外破坏性变更，得单独拍。**L56 更正并结案**：这句「得单独拍」的前提是「根治
只有两档、两档都动对外契约」，而实测还有第三档 —— 只把错误处理器从 `strict` 换成 `replace`、
不动编码 ⇒ 能被编码的字节一个都不变（A/B 实测见 §3.31），所以 A98 不必再等拍板。**A99** —— `--gate` 是二值档，六条命令里只有
`quality-report` 带阈值（`--threshold`），`check-leakage` 的 `leak_rate` 与 `audit` 的发现条数这两个
**计数形判决位没有旋钮** ⇒ 「泄漏率超过 1% 才算失败」今天仍然只能人读 JSON。**A100** ——
`doctor` / `auto-test` / `migrate` 三条的**判负**路径只由桩覆盖（真值随本机装了哪些包、语料内容与运行
时刻而变，承 L52「断言里不写随环境而变的数」），真实端到端只钉到「不崩」与「默认恒 0」两档。

### 3.31 L56：把「每轮手跑的自查」冻成用例 —— 文档结构五判据 + 编码不再参与退出码（关闭 A93 / A98，无新立）

本轮的选题不是新功能，而是**同一笔债连付三轮**：A93 从 L53 起每轮靠 `Temp/` 里的一次性脚本手跑
（L53 立粗体维、L54 立表格维 + 「数竖线先剥行内码」、L55 立「每表第二行必须是分隔行」），而
`Temp/` 不入库 ⇒ 判据随轮次流失，L55 因此给自己排了「L56 首选」。两件事一次做完。

**其一：仓库 Markdown 的五条结构判据进测试**（`tests/unit/test_docs_markdown_structure.py`，53 例）。
判据 = 行尾不混用 / 围栏闭合 / 粗体成对 / 剥行内码后列数一致 / 每表第二行必须是分隔行。
**范围从 A93 立项写的「四份主文档」扩到「顶层 + `docs/` 平铺的全部 9 份」**，理由是先测后写：
一次扫完 9 份（Temp `l56/probe.txt`）⇒ 五维全绿、0 份需要修 ⇒ 扩容零成本，且下一轮新增文档自动进守卫。
不用递归扫：实测 `rglob("*.md")` 会拽进 `web/node_modules` 里 400+ 份第三方 README（全仓 `.md` 4,354 份），
那不是本仓库的交付面。守卫配三条防空转（发现集非空、表格 / 围栏 / 粗体三维各自「确实看到了东西」）
与六条变异例。**注入自证**（Temp `l56/inject.txt`）：把 9 份真实文档读进内存，逐维各注入一处缺陷
（注入必须用各文档自己的行尾，否则给纯 LF 文档追加 `\n` 根本测不到「混用」那一维）⇒
**45 处注入 45 处被抓、跨维串扰 0、基线 9 份全绿**；全程不写任何仓库文件。

**其二：把编码从退出码的语义里摘出去**（A98 结案）。L55 说这一档「得单独拍」，是因为把根治想成了
只有两档（钉 UTF-8 / 给崩溃另立一档码），而两档都改对外契约 —— **这个前提是错的**：还有第三档，
**只换错误处理器、不换编码**（`augmentor/cli/io.py:harden_stdio()`，由 `cli.py:main()` 第一件事调用）。
能被 locale 编码的字符走同一条编码路径 ⇒ 字节不变；只有今天**根本产不出来**（抛 `UnicodeEncodeError`）
的那批字符改落一个 `?`。破坏面按 A98 的要求先量了：

- **stdout 侧**（Temp `l56/gbkblast_pre.txt` / `_post.txt`，`PYTHONIOENCODING=gbk` + 含 emoji 语料
  × 16 条命令）：改前 **4 条崩成 rc=1**（`check-leakage` / `stats` / `preview` / `search`，前三条
  stdout 0 字节），其中 `check-leakage` 正是 L55 刚接上 `--gate` 的六条之一 ⇒ 「CI 门禁」在一台
  GBK 机器上会被一条 emoji 样本变成「永远红」。改后 16 条全 rc=0、0 条编码崩。
- **非破坏性 A/B**（Temp `l56/gbkbytes.txt`：A = `git show HEAD:augmentor/cli.py` 抄出的改前入口，
  B = 工作树入口，同一语料逐条比 stdout 的 sha1）：**10 条逐字节相同**、4 条正是改前崩掉的那批、
  2 条（`monitor` / `benchmark`）**同一入口连跑两次自身即变 ⇒ 不可判定**（Temp `l56/ctrl.txt`），
  按 L52 那条纪律不进任何主张。`clean --output` 的落盘文件 sha A/B 相同。
- **为什么是 `replace` 而不是 `backslashreplace`**：兜底字符必须留在 JSON 的合法字符集里，
  `\U0001f342` 会把一份本来能 `json.loads` 的报告变成带非法转义的废文档 ⇒ 这条口径钉在
  `tests/integration/test_cli_stdio_encoding.py`（12 例，含一条「GBK 管道今天仍然编不出 emoji」
  的前提断言，防的是整批用例空过）。判据有效性同样用 A/B 自证：同一批命令换 HEAD 入口跑
  ⇒ 4 条全红（Temp `l56/inj2.txt`）。
- **落盘侧是同一根因**（Temp `l56/encaudit.txt`，AST 扫全仓已跟踪 `.py`）：文本档 `open()` 不带
  `encoding` 的产品代码只剩 `versioning.py` 两处写（`:197` / `:372`），而读它的那处 `read_text()`
  同样不带 ⇒ 本轮三处一起钉成 `utf-8`。**为什么这不算破坏**：那里只写 `_generate_version_id()`
  产出的 `v_<数字时间戳>[_<数字>]`，纯 ASCII ⇒ UTF-8 与 GBK 逐字节相同，旧文件用新读法读得出。
  判据冻成 `tests/unit/test_no_locale_text_io.py`（AST 扫产品包 104 份源文件，实测 0 违规 ⇒
  守卫落地即绿，此后新增一处不带编码的文本读写当场红）。
- **一处自造缺陷要记**：第一版审计用正则 `\bopen\(([^()]*)\)` 数，而 `open(x.with_suffix('.txt'), 'w')`
  这种带嵌套括号的调用**整条匹配不上** ⇒ 它报「219 个 open / 6 处不带编码」，AST 版是「221 个 open /
  5 处不带编码，其中 3 处是二进制档、2 处正是 `versioning.py` 那两个」。正则版不只是数错，它把
  方向也指错了（两份名单里没有真缺陷）。这是 L55 那条「引用计数器之前先验它的换算」在本轮第一次
  现形，且现形于我自己刚写的第一版脚本 ⇒ 新增纪律：**判据型脚本一律走 AST，不用正则数代码**。

**性能不主张任何方向**：`harden_stdio()` 是进程启动期两次属性判断加至多两次 `reconfigure`，落在
解释器启动与 import 的噪音底下，本轮不做 A/B、也不引用任何 µs 数。

### 3.32 L57：把 `logging` 那三个旋钮接上负载 —— 「写了才动手」+ 两侧同判（关闭 A97 / A101 / A105，新立 A102–A104 / A106）

**选题由来是一行 `grep`**：`config.logging` / `LoggingConfig` 的命中自 L54 起**全在
`config.py` 自己体内** ⇒ `level` / `file` / `format` 三键零应用方。旋钮存在、指针不接负载：
改 `logging: {level: ERROR}` 静音不了 L54 那条 WARNING，`logging.file` 对应的「日志进文件」
这件事也从来不存在。本轮补的就是那根指针（`augmentor/logging_setup.py`，新文件）。

**口径一「写了才动手、只动写出的键」不是洁癖而是两个面的形状冲突**：`api/main.py:42-45`
自己 `basicConfig(level=INFO, format="…%(name)s…")`，CLI 进程却是 root 无 handler、走
`logging.lastResort`（WARNING + `%(message)s` 裸消息落 stderr）。同一个默认值不可能同时等于
两个面，所以 `load_config` 只在配置文件里**真的出现 `logging` 节**时调
`apply_logging_config(config.logging, written=set(raw_logging))`（`config.py:584`），
`written` 里没出现的键一律不动（`logging_setup.py:132` 起）。默认三档按 CLI 今天的形状写
（`WARNING` / 空串 / `%(message)s`），实测（Temp `l57/probe1.txt` P0/P1）装上同档 handler
之后同一条 WARNING 的 stderr **逐字节不变**。

**破坏面先量后写**（Temp `l57/ab.txt` NONCE-L57-AB，四臂真子进程逐字节比）：默认档
**14 条命令的不等名单为空**，另 2 条（`monitor` / `benchmark`）同一入口连跑自身即变
（快照 ID / 时间戳）⇒ 按 L52 那条纪律**不进任何主张**，不是「测过了」。
写出非默认值（`level: INFO` + 带 `%(asctime)s` 的 format）之后 7 条命令 stderr 变多、
`logging.file` 落盘 14 行而控制台**不少字** —— 都是「用户明确要求了」的那一侧。

**口径二「判据两侧同一份」**：允许集 `LOGGING_LEVELS`（`logging_setup.py:42`）只有一处定义，
`config_validator` 的 `choices` 规格与运行时 `level_number` 都引它；`format` 的可渲染性判据
调同一个 `build_formatter`，理由是 `Formatter("%(nope)s")` 构造期合法、到发第一条日志才抛，
而 `Handler.emit` 会把它变成**每条日志一行**的 `--- Logging error ---` + 31 行 traceback
（实测 `l57/probe2.txt` P2/P3）。两侧同判冻成三条集合等式（`tests/unit/test_config_validator.py`
的 `non_empty` / `choices` / `renderable` 各自等于运行时真判的那份），承 L51 的 A77 教训。

**A105：本轮自己造出来的代价，同轮量出来、同轮修掉**。接上判据之后 `load_config` 在
**配置里从没写 `logging` 节**的机器上慢 **+30~+37 µs、三轮同号**（Temp `l57/cost_prememo.txt`
NONCE-L57-COST，head/工作树交替三轮）。根因不在判据而在它被挂了**两遍** —— 一次加载构造两次
`LoggingConfig`（`AppConfig()` 的 default_factory、`_load_section` 各一次），`__post_init__`
每次都要对着真 record 试渲染默认串。计数是**直接包装 `logging.LogRecord.__init__` 数出来的**
（Temp `l57/rec.txt` / `rec_prememo.txt` NONCE-L57-REC）：修前 2 条/次加载、修后
无节 0 条、写出 `format` 1 条（剩下的是装配真要用的那个 Formatter）。修法是缓存
「已通过」的格式串（`assert_format_renderable`，`logging_setup.py:95`），**判据一条不省**：
缓存只收 `build_formatter` 通过之后的串，坏串永远走真判据。修后同一读数是
`−1.1 / +6.6 / +5.5 µs`、**不再同号** ⇒ 按口径不进主张；还站得住的只有
`AppConfig()` 纯构造 **+0.4~+0.7 µs（同号）**。两条变异坐实这组用例不是摆设：删掉缓存短路
⇒ 恰 3 红（正是那三条计数断言）；把缓存改成「含坏串一起收」⇒ 2 红（每次都出声那条 +
SDK 直构拒收那条）。

**两处仪器自身的坑，都记下来**（同一族的「解析器返回空 ≠ 没有热点」）：其一，
`pstats.print_stats(22)` 会**截断**，于是 HEAD 臂里「看不见 `LogRecord.__init__`」与
「它没被调用」在同一份报告里长得一样 —— 前 22 名之外才是真相，改成直接取未截断的
`Stats.stats` 字典（键 = 文件/行号/函数，值元组形状**先实测**：本解释器长度 5，
按文档印象取 `v[2]` 会把 `tottime` 取成 `primitive_calls`）。其二，比对脚本第一版把
`magnitude` 按**秒**打印而把每轮差值按 µs 打印 ⇒ 整列显示 `+0.00 µs`，与本轮
「预测侧 `blast.py` 用 `len(decoded_str)`（字符）、实测侧用字节」造成 5 条假不闭合
是同一个错：**对账之前先统一单位，单位差不是行为差**。

**A101（同轮撞出、同轮修）**：节名写了却没给值（`logging:`）或写成标量（`logging: app.log`）
时，`_load_section` 在 `conf.get` 上抛 `AttributeError: 'NoneType' object has no attribute 'get'`，
实测**每一节**同形（`l57/probe1.txt` P4/P5，`web` / `quality` / `dedup` 全中）⇒ 与 A94 是同一根因
的第二层，修法同 A94：null = 该节全默认，标量 = 明说摆错了形状。

**测试面**：`tests/unit/test_logging_wiring_a97.py` 82 例（含 root handler 按精确类型筛、
`_empty_root()` 在**测试体内**清空 —— pytest 的 `LogCaptureHandler` 是 `StreamHandler` 子类
且挂在 call 阶段之后，在夹具里清无效，实测两条断言因此假绿过）+
`tests/integration/test_cli_logging_wiring.py` 17 例（真子进程、`--config` 必须排在子命令**之前**、
判据词「键没人读取」与诊断面共用 `ConfigValidator.UNREAD_MARKER` 一个常量）。
文档两处随轮改：`API.md` 的 L57 勘误（上一轮那句「目前没有把它静音的配置旋钮」已成假）、
§3.29 末尾的勘误（「本轮答不了」已被本轮答完，原文一字未删）。

**新立四条，都不在本轮动手**：A102 = `logging` 节在 API 面生效**没有行为面用例**
（`apply_logging_config` 的非测试调用点只有 `config.py:584` 一处，`api/` 与 `cli.py` 各 0 命中；
`tests/` 里 TestClient 与 logging 同屏的只有 `test_api_dataset_system_tools.py`）；
A103 = `logging.file` 打不开时**整条配置被拒**（`rc=1`，集成用例 `file: nope_dir/x.log` 实测），
「配错一个路径就连模型名都用不了」这一刀切口径未拍；A104 = `_open_file_handler`
（`logging_setup.py:122-124`）用裸 `logging.FileHandler`，**无轮转、无大小上限** ⇒ 长跑进程
日志无限增长；A106 = `apply_logging_config` 每次加载都先 `_detach_installed` 再重装 ⇒
写出三键的加载 **+194~+204 µs**（`cost.txt`，三轮同号）且每次**关开一次日志文件**，
并发调用 `load_config`（`/api/config` 每请求读盘）时还存在「handler 被摘掉」的空窗。
性能主张只取上面「同号」那部分；§3.31 曾声明「本轮不做 A/B、不引用 µs」，本轮引用 µs
是因为选题本身就是代价，仪器是 head/工作树**交替三轮** + 直接计数，不是单轮差值。

## 4. 核心数据流

### 4.1 数据增强主流程

```
输入 JSON
   │
   ├─ 1. 读取并统计条目数
   │
   ├─ 2. 断点判定
   │     task_id = augment_{md5(输入文件绝对路径)[:12]}
   │     ├─ 存在断点 → load_checkpoint → 合并增量 → 得到 remaining_indices
   │     └─ 不存在   → create_checkpoint → remaining_indices = 全部索引
   │
   ├─ 3. 恢复时载入上一轮输出（避免覆盖为空）
   │
   ├─ 4. 逐条生成变体（串行 / ThreadPoolExecutor 并行）
   │     └─ 每条：调用模型 → 解析 JSON 变体 → 质量过滤 → 滑动窗口去重
   │        └─ update_progress(idx, success, score) → 按间隔写增量
   │
   ├─ 5. save_checkpoint()  → 增量并入主文件并删除增量
   │
   ├─ 6. 全局去重 deduplicate_and_filter
   │
   ├─ 7. 写出输出 JSON
   │
   ├─ 8. 自动版本快照（config.versioning.auto_snapshot）
   │
   └─ 9. 返回报告（含 progress 进度快照）
```

### 4.2 质量评估流程

```
输入 JSON
   ├─ QualityScorer.batch_score
   │     ├─ 语义相似度：原始 instruction vs 生成 instruction
   │     ├─ 回答相关性：instruction vs output
   │     └─ 多样性：与该批次其他变体的差异度
   │     → 加权总分 + passed 判定
   ├─ Deduplicator.generate_report → 移除率、重复组
   └─ ReportGenerator.generate
         ├─ 分桶统计（0.0-0.2 / 0.2-0.4 / 0.4-0.6 / 0.6-0.8 / 0.8-1.0）
         ├─ 过滤统计与去重汇总
         └─ 规则引擎 → improvement_suggestions
```

### 4.3 导出流程

```
输入 JSON
   ├─ 校验格式合法性（在开启线程之前）
   ├─ PreviewGenerator.preview  （可选，仅预览不落盘）
   │     ├─ 必填字段校验
   │     └─ 长文本截断（200 字符）
   └─ Exporter.export / export_batch
         ├─ jsonl        → 每行一条 JSON
         ├─ llama_factory→ instruction/input/output
         ├─ alpaca       → 同上，字段名标准化
         ├─ sharegpt     → conversations[]（依赖 history）
         └─ chatml       → messages[]（依赖 history）
```

`export_batch` 支持 `数据集 × 格式` 的笛卡尔积并行导出，`use_parallel=True` 时按组合开线程。

## 5. 并发模型

| 场景 | 机制 | 可配置项 |
|------|------|---------|
| 数据集增强 | `ThreadPoolExecutor` + `as_completed` | `augmentation.num_threads` |
| 多轮对话生成 | `ThreadPoolExecutor` | `ContextAugmentor.max_workers` |
| 领域扩展 | `ThreadPoolExecutor`（按策略并发） | `DomainExpander.max_workers` |
| 批量导出 | `ThreadPoolExecutor`（按组合并发） | `export_batch(use_parallel=)` |
| 多语言翻译 | `ThreadPoolExecutor` | `MultilingualSupport.max_workers` |
| API 阻塞调用 | 线程池（`run_in_thread`） | 事件循环默认 |
| 断点写入 | `threading.Lock` 保护 | — |

`AugmentorPipeline._lock` 保护共享状态的写入；`CheckpointManager._lock` 保护断点更新的原子性。

并行路径下结果按**原始索引**回填，最终顺序与串行一致，不会因线程调度而乱序。

## 6. 错误处理约定

| 层次 | 策略 |
|------|------|
| 单条数据生成失败 | 记录 ERROR 日志，该条标记为 failed，不中断整体任务 |
| 模型调用失败 | 指数退避重试，`generate(max_retries=…)` 传的是**总尝试次数**（不是额外次数）且下界 0，仍失败则抛出由上层捕获。**缺省档位自 L45 起来自配置 `augmentation.max_retries` / `retry_delay`**（经 `create_model_backend` 透传，见 §3.20）；重试口径唯一在 `retry.with_retries` 这一层 —— 传输层自 L46 起不配任何重试（`HTTPAdapter` 只留连接池，见 §3.21），因为对 POST 它本来一次都不会跑，留着只会让人误算出「两层叠乘的请求上界」。**注意封顶分两支、互不相犯（L47 拍定，A72 关闭）**：退避计算那一支归调用方传的 `max_delay`（模型后端传 `_MAX_RETRY_DELAY = 30.0`）；服务端给 `Retry-After` 时走另一支，只归 `retry.MAX_RETRY_AFTER`（300 s）封顶，`max_delay` 参不到场 —— 这是刻意的，把「45 秒后再来」夹成 30 s 等于对刚说过别敲门的服务端提前敲门。默认档一次 `generate()` 的最坏总等待因此是 2 × 300 = 600 s 而不是 60 s，两行的算法见 §3.22。**自 L49 起两副封顶里的第二副交到配置面**：`augmentation.max_retry_wait`（默认 300.0，只能夹小不能放大）与 `augmentation.retry_jitter`（默认 0.0，0-1）⇒ 最坏总等待可用一行写完 `(attempts − 1) × max(_MAX_RETRY_DELAY × (1 + retry_jitter), max_retry_wait)`，五档实测见 §3.24 |
| 可选依赖缺失 | 降级并记录 WARNING，功能跳过而非崩溃 |
| 必需依赖缺失 | 抛 `ImportError` 并给出安装命令，不静默 |
| ↑ 的具体落点：`models/base.py:_get_session`（L46 起） | 上两行原本在这里**互相打架**：`requests`（必需）与 `requests.adapters.HTTPAdapter`（可降级）共用一个 `except ImportError`，于是必需依赖缺失也会先打一条「requests 未安装，使用基础连接」再抛。现拆为「`import requests` 在 `try` 外 ⇒ 必需缺失直接抛；只有 HTTPAdapter 包在可降级的 `try/except` 里 ⇒ 降级为无连接池裸 Session」，两条约定各归各位（详见 §3.21） |
| 配置缺失 | 使用 dataclass 默认值；环境变量缺失替换为空字符串 |
| 配置文件损坏 | 抛出异常，不吞掉 |
| API 层 | 转换为 `HTTPException`，`404` / `400` / `500` 语义明确 |
| 计数 / 分页 / 窗口 / 步长 / 配额 / 保留数 / **重试次数 / 退避档位**旋钮越界 | 在 SDK 入参处一次判掉（`validation.require_count`），抛 `DataValidationError`；CLI 变退出码 1，API 变 400 |
| **时长 / 秒数**旋钮越界（`retry_delay` / `default_retry_delay`，L45 起） | 同一条口径但换判据：`validation.require_seconds(name, value, minimum=0.0)` 拒 `bool` / 字符串 / NaN / 负值，接受 int 与 `+inf`。**不混进 `require_count`**（它按口径拒绝非整数）。配置文件侧由 `config_validator` 的 KNOWN_FIELDS 同判（0–60 s），两侧同判由 `test_validator_and_runtime_use_the_same_verdict` 钉住 —— 静态校验面与运行时判据不一致时，症状是「校验通过但一跑就炸」或反之，见 §3.20。**判据覆盖面自 L47 起不再只有配置那两处**：`retry.compute_delay` 与 `retry.with_retries` 的 `base_delay` / `max_delay` 也在各自入参处判（后者判在调用 `func` **之前**，否则坏值会先烧掉一次真实请求、再从循环里抛出与参数错误无关的 `TypeError`，原始异常丢失），见 §3.22 |
| **无量纲倍率**旋钮越界（退避 `factor`，L47 起） | 第三条判据 `validation.require_positive(name, value)`：**严格大于 0**，拒 `bool` / 字符串 / NaN / 0 / 负数，接受 int 与 `+inf`。不复用 `require_seconds` 是因为它的文案会把「秒数」安到一个没有单位的倍率上（指错根因），也不做可配下界是因为这类值只有「每档乘多少」一种合法读法——实测 `factor=0` 的退避是 `[1.0, 0.0, 0.0]`（忙重试）、`-2` 是 `[1.0, 0.0, 4.0]`（隔档完全不等待的非单调序列）。**`0 < factor < 1` 放行**：实测 `[1.0, 0.5, 0.25]` 非负、单调、有界，没有该拒的形状。同见 §3.22 |
| **闭区间比例**旋钮越界（退避抖动 `jitter`，L48 起） | 第四条判据 `validation.require_ratio(name, value, minimum=0.0, maximum=1.0)`：**判据族里唯一连上界一起判的成员**，因为比例旋钮越界的症状是「上限从别处冒出来」——`jitter` 加在 `min(max_delay, …)` **之后**，实测贴顶档位 `jitter=1.0` 抽出 30.05 ~ 59.87 s、`50.0` 抽出 32.27 ~ **1523.54 s**，越过 `max_delay` 的比例都是 1.000；判在 0-1 内可把那一支证死在 2 × `max_delay`。下界同样不多余：`-5.0` 与 `NaN` 过不了 `if jitter > 0`，与「没传参数」逐字同答；`True` 则被算术读成 1.0 = **最大档**抖动。`None` 放行（= 没传），实测 59.7 ns 短路 / 235.3 ns 全判。同见 §3.23 |
| **等待预算**双旋钮越界（`augmentation.max_retry_wait` / `retry_jitter`，L49 起） | 第五条判据形状 = **给 `require_seconds` 加可选 `maximum` 参数**（不另立新判据：它判的还是「秒数」，文案根因不变；L47 另立 `require_positive` 是因为根因不同才分家）。`max_retry_wait` 判 0-300 闭区间且**上界硬编在 `MAX_RETRY_AFTER`** —— 「300 s 封顶」这句承诺本身就是判据，加判据前实测 `max_retry_wait=inf` 配 `Retry-After: 3000` 交出 `[3000.0, 3000.0]`（默认档合计 6000 s），判后同一入参在第一次请求**之前**抛 `DataValidationError`；`retry_jitter` 直接复用第四条 `require_ratio`。这两个旋钮是**本仓第一对「静态校验面与运行时判据全区间一致」**的配置项（`test_validator_and_runtime_agree_on_every_axis`；旧旋钮 `retry_delay ≤ 60` / `max_retries ≤ 20` 仍是校验器独有上界，记在 A77）。同见 §3.24 |
| 静态校验面比运行时**更松**（bool / NaN 混进数值字段，L49 起） | `_validate_known_fields` 的内联类型判定原先按 `isinstance(value, (int, float))` 读，而 `isinstance(True, int)` 恒真 ⇒ 7 个数值规格键 7/7 把 YAML 里的 `true` 判成合法，四道运行时判据却全部拒 bool —— 症状是 `validate-config` 绿灯、建管道即 `DataValidationError`。修法：内联判定排 bool（`type: bool` 的开关字段不受影响）+ 补 NaN 判据。两条洞覆盖面不同：`true` 在旧形状下 **7/7 个数值键完全无报错**，NaN 只漏 **4 个 `type: float` 键**（3 个 int 键靠 `isinstance` 本来就拦得住）⇒ 各堵一片、不重复；注入分判也各成一档（摘 bool 排除 ⇒ 2 红，摘 NaN 判据 ⇒ 5 红）。根因是 A70 那套**只有测试在调用**的死助手：它们早就排了 bool 却没有 NaN 判据，两套口径各拿对半边、活的那套恰好是错的半边。同见 §3.24 |
| **校验形 CLI 命令的「判决」**（`validate` / `validate-config` / `dependency --action validate`，L53 起） | 上面几行管的是**异常**怎么翻译（抛 `DataValidationError` ⇒ CLI exit 1 / API 400）；这一行管**判决**（命令正常跑完、但结论是「不合格」）怎么翻译。口径：**判负 ⇒ `sys.exit(1)`，通过 ⇒ 不抛 `SystemExit`**（沿用测试面 159 处 `assert code is None` 那套既有约定，本轮不新造形状）；**只有 WARNING 不判负**（与 L52 的「写了没人读」定级同轴，否则诊断通道出声即挡路）。**报告形命令（`audit` / `check-leakage` / `doctor`）刻意不接** —— 三条 handler 体内零判决词（37 个 `run_*` handler 普查，Temp NONCE-L53-CENSUS），接上等于新造一条「有发现即失败」的隐含契约。先例：`health-gate`（`cli/commands/quality.py:205`）。**另有三条 `quality-report` / `auto-test` / `migrate` 有判决语义却仍恒 0**（`overall_passed` / `failed_tests` / `failed_items`）——那是本轮拍定不动的欠账，要拍的是产品口径不是机制，记在 A91 边界 ①。同一批还把 `cli.py:46-53` 的 `load_config` 挪进 `try` ⇒ 配置加载期异常与 handler 异常同形（「`错误: …` + exit 1」），CLI 的失败从此只有一种形状。**已知空洞（A92）**：`is_valid = valid_count == len(items)` 让**空数据集空洞地判「有效」**（SDK / CLI / API 三层同形，实测 `exit 0` + 200 `{'is_valid': True, 'total_items': 0}`）⇒ 退出码 0 只代表「没有不合格的记录」，不代表「验过东西」。详见 §3.28 |
| **配置加载期「写了没人读」的出声面**（`load_config`，L54 起） | 同一条判据有**两条通道**而不是一份文案两份实现：诊断面 `validate_config` 的 `warnings` 与产品面 `load_config` 的 `logging.warning` 由同一个 `_warn_unread_keys` 产生，两条通道的等式用常量 `UNREAD_MARKER = "没人读取"` 做机械锚（比对方式就是「这个词在不在文案里」，抄写的两份文案迟早漂移）。三条否决定下形状：**不写 stdout**（那是各命令的输出契约，`stats` 是 JSON）、**不进退出码**（承上一行的「只有 WARNING 不判负」）、**走 `logging` 而非 `print`**（API 进程 `basicConfig` 之后与它同一出口，未配置时由 `logging.lastResort` 落 stderr；**注意配置里的 `logging.level` 目前管不住它** ⇒ A97）。**只出「没人读」，不出「环境变量未设置」**（后者条数是本机 shell 的函数，见 §3.27 那条纪律）。出厂 `config.yaml` 零命中 ⇒ 默认档一次运行 **0 行**额外输出；代价实测 aug **+74.6 µs（1.09 %）** / py314 **+146.9 µs（2.17 %）**，同进程正反双序同号 ⇒ 可判定。同一轮把「0 字节 / 只含注释的配置文件」从裸 `TypeError` 改成与「路径不存在」同档（全默认 + `exit 0`，A94），但这留下一对**对立判决**：产品面「全默认、通过」对 诊断面「`is_valid=False` + 配置必须是字典类型」⇒ **A96**，要拍的是产品口径。详见 §3.29 |
| **CLI 退出码的三种形状 + 判决的唯一出口**（`cli/verdict.py:verdict_exit`，L55 起） | `0` = 判决通过、或这条命令压根不判负；`1` = **判决未通过**（只由 `verdict_exit` 产生）**或** handler 抛异常被 `cli.py:main()` 翻译成「`错误: …` + 1」⇒ 同样是 1，**靠 stderr 有无「错误:」分「判负」与「崩溃」**，这条二义性本轮刻意没动（改异常档要动对外契约）；`2` = argparse 用法错误（无判决位的命令传 `--gate` 即落此档，实测 `stats --gate` ⇒ 2 且 stderr 含 `unrecognized arguments`，**不落 1**）。上一行那句「报告形命令刻意不接」自本轮起变成**默认不接、`--gate` 才接**：六条报告形命令（`quality-report` / `audit` / `check-leakage` / `doctor` / `auto-test` / `migrate`）各带一条旗标，默认档一字未改。守卫从推导不抄清单：`tests/integration/test_cli_verdict_wiring.py` 钉三条集合等式（parser 声明 `--gate` 的命令 == handler 里传 `enforce=args.gate` 的，6 条；handler 源码出现 `verdict_exit(` == 子命令 help 含「退出码」，10 条），改前基线是空集对 3 条 ⇒ 不是同义反复。另立一条**编码护栏**：CLI 面向文案里的字符必须能被 GBK 编码（`⇒` / `✅` / `❌` 都不行）—— 子进程 stdio 在本机默认 `gbk`，一个字符能让整条命令崩成 rc=1，即**判决码被一个装饰符占掉**（本轮实测两起，一起既有、一起本轮自造）。欠账：`--gate` 是二值档、计数形判决位没有阈值（A99），三条命令的判负路径只由桩覆盖（A100）；曾列的「含 emoji 的用户数据仍会崩（A98）」自 L56 起由 stdio 编码兜底按住 ⇒ 见下一行与 §3.31。详见 §3.30 |
| **编码与文档结构这类「过程判据」改由用例守**（L56 起） | 两半同因：文本 I/O 跟着 locale 走。**stdout / stderr** 在 CLI 入口把错误处理器从 `strict` 换成 `replace`（`augmentor/cli/io.py:harden_stdio()`）⇒ **编码再也吃不掉退出码**，编不出的字符落一个 `?`、JSON 输出仍可 `json.loads`（选 `replace` 而不是 `backslashreplace` 的理由就是这条可解析性：`\U0001f342` 会把报告变成带非法转义的废文档）；**产品包内**文本读写必须显式带 `encoding=`，由 AST 扫 104 份源文件守着（`tests/unit/test_no_locale_text_io.py`，落地时 0 违规）。**能编码的字节一个都没变**（改前入口与改后入口 A/B 比 stdout sha1：10 条相同、4 条是改前崩掉的那批、2 条自身连跑即变因而不可判定 ⇒ 不进主张）。另一半：仓库 9 份交付面 Markdown 的五条结构判据（行尾不混用 / 围栏闭合 / 粗体成对 / 剥行内码后列数一致 / 每表第二行必须是分隔行）从「每轮手跑 `Temp/` 脚本」冻进 `tests/unit/test_docs_markdown_structure.py`，配 45 处内存注入自证。新增纪律：**判据型脚本一律走 AST，不用正则数代码**（正则第一版把带嵌套括号的 `open(...)` 调用整条漏掉 ⇒ 数错且把方向也指错）。详见 §3.31 |
| 断点文件损坏 | 记录 ERROR 并返回 `None`，退化为从头开始 |

「取前 N 条」这一类旋钮（`limit` / `offset` / `top_k` / `preview_size` / `batch_size`
/ `size` / `n` / `diversity_sample_size` / `max_backups` / `chunk_size` / `target_size`
/ `num_topics` / `topics_per_strategy` / `num_questions_per_topic`）在实现里都落到下标
切片或算术，而切片对越界值不报错、只换语义：`[:top_k]` 在 `top_k` 为负时读成「丢掉末尾
几个」，`[-limit:]` 在 `limit` 为 0 时读成「全要」，`int(target_size * 占比)` 在
`target_size` 为负时静默交出 0 条。L44 把同一判据扩到重试的两个计数旋钮
（`max_retries` / `compute_delay` 的 `attempt`），它们是同族的**另一种形状**：不是换语义，
而是**换症状类别** —— `range(0, max_retries + 1)` 在负数下直接为空，函数一次都不被调用、
最后 `raise last_exc` 变成 `raise None`，抛出一条与「参数写错」毫无关系的
`TypeError: exceptions must derive from BaseException`；而 `attempt - 1` 是指数，
`attempt=0` 被算成 2 的负一次幂 ⇒ 静默交出**比第一档更短**的等待（实测 0.5 / 0.25 / 0.0625 倍）。
所以判据集中在
`augmentor/validation.py:require_count` 一处，各调用点不再各自校验（API 侧也因此不需要
`ge=` 约束，见 `api/routes/dataset_tools.py` 的 `SearchRequest` 与 `SampleRequest`）。
五条边界值得记住：
**0 是合法值**（「一条都不要」，与「没传参数」`None` 必须区分开，因此回落一律写
`x if x is None else default`；`aggregator.aggregate_weighted` 的 `None` 按默认
`DEFAULT_TARGET_SIZE` 读，因为 API 的 `Optional[int]` 字段会把「没填」原样传成 `None`），
**判参先于数据短路**（空输入配坏参数仍要报参数错，否则坏参数会被空结果掩护掉），
**判参先于副作用**（`backup.clean_old_backups` 的守卫排在 `DatasetBackup(backup_dir)`
之前，因为构造本身会 mkdir 并写一份 `index.json`；`streaming` 的守卫排在 `StreamReader`
构造的第一行，所以坏步长在 `StreamAugmentor.__init__` 就报，不必等一份数据读完——
注意实测 `StreamWriter.__init__` 并不落盘（`open()` 发生在 `__enter__`），
「先建 reader 才不留空产物」不成立，可证的只有「构造期即拒且磁盘无产物」），
**判参先于花钱的副作用**（`expander` 的守卫排在模型调用之前——否则 `num_topics=-1`
会把「生成 -1 个」这种自相矛盾的提示词真发出去），
**0 在不可逆语义下不放行**（`max_backups=0` 与手滑想打的 10 无从分辨，而后果是删光全部
备份，所以那类旋钮的下界是 1）。取 `minimum=1` 的站点共五处，各自的理由都是「窗口没有
0 条这个合法读法」：`ActiveLearningLoop.batch_size`（每轮必须选出样本）、
`cleaner.clean_batch_optimized.batch_size`（`range()` 的步长）、
`quality.QualityScorer.diversity_sample_size`（空参照会被判成「完全多样」）、
`backup.clean_old_backups.max_backups`、`streaming.StreamReader.chunk_size`（分块步长；
实测 HEAD 把 `0` 与 `-1` 都静默读成 1，真实 6902 条从 7 块变 6902 块，端到端慢 61%）。
其余站点下界是 0。

向量检索的 `top_k` 是两个后端共享的旋钮，HEAD 却给出两种不同的错法：FAISS 兜底路径
`min(-1, n)` 之后 `[:k]` 连吃两次末位裁剪（实测 4 条向量要 `-1` 拿到 2 条），真 chromadb
对 `0` 与负数抛库内裸 `TypeError`。判据在两侧各自接线，且 chromadb 自己把 `0` 读成
「0 条结果」而不是交给库去报错——同一个旋钮在两个后端必须同答。

**读数对账的两条口径（L57）**：① **预测与实测对账之前先统一单位**。本轮的破坏面预测把 16 条命令
的 stderr 增量写成「+72 B / +273 B / …」，实测第一版按**字节**对账 ⇒ 5 条「不闭合」，而这 5 条
正是含中文的那 5 条 —— 预测侧取的是 `len(decoded_str)`（字符），实测侧取的是字节 ⇒ 单位差被读成了
行为差。改按同一单位（字符）对账后不闭合名单为空。**两个数不相等时先问它们是不是同一种数**，
这条与 L55「引用计数器之前先验它的换算」同族。② **仪器返回空 ≠ 没有信号**。`pstats` 的
`print_stats(22)` 会截断，于是 HEAD 臂里「看不见某个函数」与「它没被调用」在同一份报告里同形；
换成直接取未截断的 `Stats.stats` 字典，并且**值元组的形状先实测再取索引**（本解释器长度 5，
按文档印象取 `v[2]` 会把 `tottime` 取成 `primitive_calls`）。同一族的还有显示侧：一份表里
`magnitude` 按秒打印、每轮差值按 µs 打印 ⇒ 整列 `+0.00 µs`。


## 7. 测试架构

```
tests/
├── conftest.py
│   ├── AI_DIR 注入 sys.path
│   ├── 假环境变量（避免模型后端构造时校验失败）
│   ├── autouse fixture：强制 model_manager 走 fallback（禁止测试期下载模型）
│   ├── sample_items：5 条中文客服问答（含 1 组近似重复）
│   └── tmp_data_file
│
├── unit/          按模块一文件，覆盖正常/边界/错误路径
│   └── test_visualizer.py 通过向 sys.modules 注入假 matplotlib / wordcloud，
│       在未安装重型依赖的环境下仍能覆盖真实绘图分支
│
├── integration/
│   ├── test_api.py      TestClient + monkeypatch deps._pipeline 注入临时目录管道
│   └── test_pipeline.py FakeModelBackend 返回固定 JSON 变体
│
└── e2e/
    └── test_web.py      前端文件结构断言 + 前后端接口契约比对
```

覆盖率门禁：`pytest.ini` 中 `--cov-fail-under=80`，低于 80% 时进程退出码非 0。

## 8. 已知限制

| 限制 | 说明 |
|------|------|
| 断点不保存生成内容 | 断点只记录索引，恢复时依赖已有的输出文件；输出文件缺失时仅能保留本轮结果 |
| 同输入文件自动续传 | 任务 ID 由输入文件派生，重复运行会跳过已完成条目；从头重跑需先删除断点 |
| 主动学习索引语义 | `select_samples` 返回的 `_al_index` 是候选内相对位置，`run()` 中需映射回原始下标 |
| 情感分析为词典法 | 中英词典计数，未使用模型，长文本与反讽场景准确率有限 |
| 图像/音频仅元数据 | 降级路径只解析文件头尺寸/采样率，不做内容理解 |
| 前端未纳入 CI 构建 | 当前 CI 仅校验前端源码结构与接口契约，未执行 `npm run build` |
| `sampler` 的 `why` 分支不可达 | `"什么" in text` 判断先于 `"为什么"`，导致「为什么」类问题被归为 `what` |
