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
ai/
├── cli.py                     命令行入口（build_parser + 11 个子命令）
├── config.yaml                默认配置
├── requirements.txt           依赖清单（分组 + 版本上界）
├── pytest.ini                 测试与覆盖率门禁配置
├── .coveragerc                覆盖率报告配置
│
├── augmentor/                 核心引擎
│   ├── __init__.py            对外导出面（版本 2.0.0）
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
│   └── frameworks/            LLM 框架互转
│       ├── langchain.py
│       └── llamaindex.py
│
├── api/                       REST API
│   ├── main.py                FastAPI 应用装配（29 行）
│   ├── deps.py                依赖注入：管道单例、异步文件 IO
│   ├── middleware/            请求日志与耗时统计
│   └── routes/                按领域拆分的 7 个 router
│       ├── data.py            数据 CRUD / 上传 / 分析 / 可视化
│       ├── augment.py         增强任务与进度
│       ├── quality.py         评估 / 去重 / 报告 / 清洗 / 标注 / 基准
│       ├── export.py          导出 / 批量导出 / 预览 / 格式清单
│       ├── version.py         版本 CRUD / 对比 / 回滚 / 历史
│       ├── config.py          配置与模型清单
│       └── multimodal.py      多模态处理与扫描
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
