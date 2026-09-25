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
记 **A58**（当前只有 SDK 显式传 `shuffle=False` 才付这笔钱）。

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
（**×1.2424**，+10.501 ms）、分层 + `shuffle=False` 55.292 ms（**×1.2762**，较分层档 +1.466 ms）。
最后这一档是 **A58 那笔保序代价第一次由产品面触发** —— L41 时它只有 SDK 显式传参才付，现在
CLI `--no-shuffle --stratify` 或请求体 `shuffle=false` 就会付。绝对值跨进程不可比（首轮探针曾读到
分层 + 保序 55.851 ms / 成功档 34.689 ms，与同进程重测差一截，故结论只取比值）。

**证据**：新增 19 个 `def test_`（unit 3 / CLI 7 / API 9）、参数化后 **27** 行；注入 5 项劣化红数
**4/2/4/2/4**，逐条与预定 node-id 同名（每次注入只跑该劣化对应的节点，红数 == 名单长度），
还原后 5 个文件 sha 逐字节一致。全量 **4684 passed / 2 skipped**（75.87 s，覆盖率 **99.02%**），
计数 **4657 + 27 = 4684 精确对上**，0 处既有断言被改写（三个测试文件 `−0` 行）。

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
| 模型调用失败 | 指数退避重试 `max_retries` 次，仍失败则抛出由上层捕获 |
| 可选依赖缺失 | 降级并记录 WARNING，功能跳过而非崩溃 |
| 必需依赖缺失 | 抛 `ImportError` 并给出安装命令，不静默 |
| 配置缺失 | 使用 dataclass 默认值；环境变量缺失替换为空字符串 |
| 配置文件损坏 | 抛出异常，不吞掉 |
| API 层 | 转换为 `HTTPException`，`404` / `400` / `500` 语义明确 |
| 计数 / 分页 / 窗口 / 步长 / 配额 / 保留数旋钮越界 | 在 SDK 入参处一次判掉（`validation.require_count`），抛 `DataValidationError`；CLI 变退出码 1，API 变 400 |
| 断点文件损坏 | 记录 ERROR 并返回 `None`，退化为从头开始 |

「取前 N 条」这一类旋钮（`limit` / `offset` / `top_k` / `preview_size` / `batch_size`
/ `size` / `n` / `diversity_sample_size` / `max_backups` / `chunk_size` / `target_size`
/ `num_topics` / `topics_per_strategy` / `num_questions_per_topic`）在实现里都落到下标
切片或算术，而切片对越界值不报错、只换语义：`[:top_k]` 在 `top_k` 为负时读成「丢掉末尾
几个」，`[-limit:]` 在 `limit` 为 0 时读成「全要」，`int(target_size * 占比)` 在
`target_size` 为负时静默交出 0 条。所以判据集中在
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
