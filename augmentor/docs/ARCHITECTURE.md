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
│   └── cli/                   CLI 实现（36 个子命令，按领域分 9 组）
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

文件读写同理，由 `deps.read_json_file` / `write_json_file` 封装为异步接口。

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

`ReportGenerator` 基于评分与去重结果生成结构化报告与**可执行的改进建议**，
而不是只输出一堆数字。建议规则见 [API 文档](API.md#post-apiqualityreport)。

### 3.8 质量基准的缺失指标处理

`QualityBenchmark.compare_with_baseline` 对基准中不存在的指标标记 `status: "no_baseline"`
且 `baseline`/`delta` 为 `null`，**不参与** improved/regressed 统计。
早期实现把缺失值当作 `0`，会把「基准里没有这个指标」误报成「指标从 0 提升到 X」的虚假改进。

### 3.9 去重的分块相似度矩阵

`Deduplicator` 对 N 条数据计算 N×N 相似度矩阵，N 较大时内存会爆炸。
实现按 `chunk_size=1000` 分块计算，只保留超过阈值的相似对。

`find_similar_pairs()` 早期调用了不存在的 `_compute_similarity_matrix`，
运行期必然 `AttributeError`；已修正为调用实际存在的 `_compute_similarity_matrix_chunked`。

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
| 断点文件损坏 | 记录 ERROR 并返回 `None`，退化为从头开始 |

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
