# 02 · 服务端（核心引擎 + REST API）

> 服务端是平台能力的**唯一实现来源**，由两层构成：
> ① **核心引擎** `augmentor/`（纯 Python，不依赖 Web 框架，能力真正所在）
> ② **REST 层** `api/`（FastAPI 薄适配，把能力暴露成 77 个 HTTP 端点并托管 Web 静态产物）
>
> 客户端（Web）与 CLI 都不直接碰数据或 LLM，而是通过这一层驱动。

---

## 1. 整体架构

分层单向依赖（与 [`01 § 3`](01-overview-and-terminology.md) 一致）：

```
api/ (REST 薄适配)  ─┐
                     ├──►  augmentor/pipeline.py  (编排)
cli/ (CLI 薄适配)  ──┘          │
                                ▼
              能力层 (models/ quality/ dedup/ export/ versioning/ vector/ …)
                                │
                                ▼
              基础设施层 (config.py / model_manager.py / visualizer.py)
```

**为什么分两层**：核心引擎保持可独立测试、可被 CLI 与 API 复用、不绑框架；
REST 层只做「协议转换 + 横切关注点（鉴权/限流/追踪/上传闸）+ 路径白名单」，
不写业务。这让「同一能力三种驱动方式行为一致」成为可验证的结构性保证。

---

## 2. 模块划分

### 2.1 核心引擎 `augmentor/`（按职责分组）

| 分组 | 主要模块 | 职责 |
|------|----------|------|
| **编排** | `pipeline.py` `data_pipeline.py` | `AugmentorPipeline` 主流程；`DataPipeline` 分阶段执行 |
| **配置** | `config.py` `config_validator.py` `logging_setup.py` | dataclass 配置中心；配置校验；日志装配 |
| **生成（LLM）** | `models/`（`base/factory/ernie/openai_model/ollama/claude/gemini`）`context.py` `expander.py` `prompts.py` | 多模型后端工厂 + 重试/会话池；多轮上下文；领域扩展；提示词 |
| **质量** | `quality.py` `dedup.py` `report.py` `benchmark.py` `quality_report.py` `quality_gate.py` `quality_monitor.py` `quality_trend.py` `outlier.py` | 三维评分、向量去重、报告、基准、健康门禁、监控、离群点 |
| **数据加工** | `data/`（`cleaner/annotator/image/audio/multimodal`）`cleaner.py` `annotator` | 清洗、自动标注（实体/意图/情感）、多模态 |
| **转换/导出** | `export.py` `export_enhanced.py` `converter.py` `preview.py` `rag.py` `multilingual.py` `language.py` | 13 种格式导出、格式转换、RAG、多语言 |
| **存储/版本** | `versioning.py` `version_control.py` `checkpoint.py` `backup.py` `vector/`（`base/faiss/chromadb`） | 版本快照/回滚、断点、备份、向量库 |
| **分析/检索** | `analytics.py` `statistics.py` `comparison.py` `compare_enhanced.py` `search_enhanced.py` `indexer.py` `semantic.py` `feature_detect.py` `profiling.py` `impact.py` | 洞察、统计、A/B 对比、检索、语义相似 |
| **安全** | `privacy.py` `leakage.py` `audit.py` | PII 脱敏、泄漏检测、就绪审计 |
| **评估/学习** | `evaluation.py` `active_learning.py` `tracker.py` `sampler.py` | BLEU/ROUGE、主动学习、实验追踪、采样 |
| **运维/工具** | `dependency.py` `migration.py` `streaming.py` `cache.py` `atomic_write.py` `retry.py` `diagnostics.py` `auto_test.py` `auto_config.py` `aggregator.py` `data_splitter.py` `data_ops.py` `dataset_ops.py` `memory_monitor.py` `report_md.py` `json_extract.py` | 依赖登记、迁移、流式、缓存、原子写、重试、自测、聚合、分割 |
| **CLI 实现** | `cli/`（`parser.py` `io.py` `verdict.py` + `commands/`） | 详见 [`04-cli.md`](04-cli.md) |

> `__init__.py` 是**对外导出面**（`__version__ = "3.0.0"` 全仓唯一版本声明），
> 把上述能力与数据类统一 re-export；4 组历史歧义名（`QualityReport` / `ValidationResult` 等）
> 已拆成语义明确的 `PipelineQualityReport` / `DatasetQualityReport` / `DataValidationResult` /
> `ConfigValidationResult`，旧歧义名已移除。

### 2.2 REST 层 `api/`

| 文件/目录 | 职责 |
|-----------|------|
| `main.py` | 应用装配：注册 15 个 router + `/api/health`；装配中间件（CORS / 请求日志 / 请求追踪 / 上传闸 / 匿名写闸 / 限流）；挂载 `web/dist` 静态产物；4xx 异常渲染兜底 |
| `deps.py` | 共享依赖：管道单例（双检锁）、数据白名单解析、`verify_api_key` 鉴权、异步文件 IO、异常→HTTP 映射（`to_http_error` / `raise_internal_error`）、`mutate_json_file` 读-改-写加锁 |
| `middleware/` | `rate_limit`（滑动窗口限流）、`trace`（X-Request-ID）、`RequestLoggingMiddleware`、`upload_gate`（上传体尺寸闸）、`write_auth_gate`（匿名写传输闸） |
| `routes/` | 15 个按领域拆分的 router（下表） |

**15 个 router → 77 端点**（按 OpenAPI tag 归组）：

| router 文件 | tag | 端点数 | 覆盖 |
|-------------|-----|:------:|------|
| `data.py` | `data` | 8 | 数据文件列表 / 分页读取 / 增删改 / 上传 / 分析 / 可视化 |
| `dataset_tools.py` | `dataset` | 14 | 统计 / 验证 / 搜索 / 对比 / 特征 / 转换 / 合并 / 采样 / 分割 / 聚合 / RAG / 自动配置 / 评估 / 影响 |
| `augment.py` | `augment` | 3 | 启动增强 / 进度 / 断点列表 |
| `quality.py` | `quality` | 9 | 评估 / 去重 / 报告 / 清洗 / 标注 / 基准 / 离群 / 画像 / 健康门禁 |
| `export.py` | `export` | 4 | 导出 / 批量 / 预览 / 格式清单 |
| `version.py` | `version` | 8 | 版本列表 / 创建 / 对比 / 数据 / 回滚 / 历史 / 删除 |
| `config.py` | `config` | 3 | 配置读写 / 模型清单 |
| `multimodal.py` | `multimodal` | 3 | 处理 / 扫描 / 格式 |
| `privacy.py` | `privacy` | 2 | 脱敏 / PII 模式 |
| `leakage.py` | `leakage` | 1 | 泄漏检测 |
| `audit.py` | `audit` | 1 | 就绪审计 |
| `status.py` | `status` | 1 | 服务综合状态 |
| `system_ops.py` | `system` | 15 | 依赖诊断 / 配置校验 / 质量监控 / 自测 / 迁移 / 流式 / 备份 / 依赖登记 |
| `vector.py` | (dataset) | 2 | 语义检索 / 向量库状态 |
| `language.py` | (dataset) | 2 | 语言画像 / 批量翻译 |

> 注：`vector.py` 与 `language.py` 的端点挂 `dataset` tag。15 个 router 端点合计 **76**，
> 加 `main.py` 内联的 `GET /api/health`（亦声明 `response_model`）= **77**。
> 逐条请求/响应契约见 [`docs/augmentor/API.md`](../API.md)。

### 2.3 中间件执行顺序（关键）

`main.py` 用 `FastAPI(middleware=[...])` + `add_middleware(...)` 组合。**顺序有实测依据**：

```
请求 → RequestTrace → RequestLogging → CORS → [上传尺寸闸 → 匿名写闸 → 限流] → 路由
```

- **上传尺寸闸 → 匿名写传输闸 → 限流** 装在 CORS **之内**（最内层）：
  超限 429 / 越权 403 才能带上 `Access-Control-Allow-Origin`，浏览器才读得到可行动文案；
  且 CORS 预检在此被短路、不消耗限流预算。
- **尺寸闸**：声明了 `Content-Length` 的写请求在**解析前**判长度，超 `web.max_upload_bytes`
  直接 413（「未解析、未落盘」）；chunked 无长度声明时由解析后按实际字节累加兜底。
- **匿名写传输闸**：未设 `AUGMENTOR_API_KEY` 时，只放行**回环地址**来的写请求，
  非本机匿名写返回 403（文案指明「设 key」或「绑回 127.0.0.1」）。

---

## 3. 核心业务流程（服务端视角）

### 3.1 数据增强（主线）

`AugmentorPipeline.augment_dataset(input, output, use_checkpoint, use_quality_check, use_dedup, use_parallel)`：

1. **建任务**：`build_task_id(input)` 由输入路径 md5 前 12 位派生 → `augment_<id>`。
2. **加载模型后端**：`models/factory.create_model_backend`（读 `models.default`），构造期校验密钥。
3. **逐 seed 生成变体**：`augment_seed` 调 LLM（带 `retry` 指数退避 + 抖动 + `Retry-After`），
   `context.py` 在多轮场景拼历史；`expander.py` 可选扩展主题。
4. **质量评分**：`QualityScorer` 三维加权（相似度/相关性/多样性），低于 `quality.threshold` 过滤。
5. **向量去重**：`Deduplicator`（`vector/` 后端；缺 faiss 回退 numpy），相似度 > `dedup.threshold` 判重。
6. **断点续传**：`CheckpointManager` 按 `auto_save_interval` 增量落盘；已完成条目跳过，
   上一轮输出载入合并而非覆盖。
7. **版本快照**：`versioning.auto_snapshot` 开启时增强完成自动打版本。

### 3.2 质量门禁

`quality_gate.build_default_gate` / `gate_dataset_health`：把**完整性 / 多样性 / 质量分布均衡性 / 覆盖率**
压成综合分与 go/no-go；`POST /api/quality/health-gate` 接收 `pass_rate`、重复率、完整性三档阈值。

### 3.3 数据安全

`privacy.py`（`PiiSanitizer` 脱敏）→ `leakage.py`（`LeakageDetector` 训/测泄漏）→
`audit.py`（`DatasetAuditor` 综合 go/no-go）。三者对应 CLI `sanitize` / `check-leakage` / `audit`。

---

## 4. 关键接口与数据结构

> 这里是**端点结构**而非逐字段契约（逐字段见 `API.md`）。

### 4.1 通用响应约定

- **所有 77 个端点都声明 `response_model`**，响应按 Pydantic schema 过滤（未声明键被静默裁掉）。
- 统一 `SuccessResponse` 形态：`{"success": bool, ...}`。
- 写操作（改配置/删数据/上传/导出/回滚）挂 `Depends(verify_api_key)`；
  设了 `AUGMENTOR_API_KEY` 缺 `X-API-Key` → 401。
- 路径参数一律过 `resolve_data_path`（数据白名单），越界 → 403；读时不存在 → 404。

### 4.2 数据集形态契约（数据白名单内）

```json
[
  {"input": "", "instruction": "用户提问", "output": "客服回答"},
  {"instruction": "退租要提前多久？", "input": "", "output": "提前 30 天…",
   "history": [{"role":"user","content":"怎么申请入住？"},
               {"role":"assistant","content":"通过官方 App 提交申请。"}],
   "system": "你是安居乐寓的智能客服"}
]
```

顶层必须是**数组**、元素必须是**对象**（`assert_dataset_shape` 在 `load_items` 与上传侧共用）。

### 4.3 端点示例（按 tag，方法 + 路径 + 用途）

```
# data
GET    /api/data/list                      列数据文件
GET    /api/data/load/{filename}?page=&page_size=&search=   分页读取（回 indices 作写坐标）
PUT    /api/data/update/{filename}         改单条（写操作）
DELETE /api/data/delete/{filename}         删单条（写操作）
POST   /api/data/upload                    上传落盘（写操作，受 max_upload_bytes 闸）
GET    /api/analyze/{filename}            分析
GET    /api/visualize/{filename}          可视化

# dataset（工具族）
POST   /api/dataset/{stats|validate|search|compare|features|convert|merge|
                     sample|split|aggregate|rag|auto-config|evaluate|impact|
                     language-profile|semantic-search|translate|vector-status}

# augment
POST   /api/augment/start                 启动增强（写）
GET    /api/augment/progress               进度
GET    /api/augment/checkpoints            断点列表

# quality / export / version / config / multimodal / privacy / leakage / audit / status / system
（逐条见 API.md；system 15 只涵盖 依赖诊断/配置校验/质量监控/自测/迁移/流式/备份/依赖登记）
```

---

## 5. 配置中心

`config.yaml` 经 `config.py` 的 `AppConfig`（dataclass）加载，`${VAR}` 语法引用环境变量
（未设置替换为空串而非报错）。**逐节全键清单**见
[`docs/augmentor/README.md § 配置说明`](../README.md) 与 `config.yaml` 内注释。

关键段：

| 段 | 要点 |
|----|------|
| `models` | `default` + 每后端条目（`temperature/top_p/max_output_tokens/request_timeout` 区间在 `config.py` 常量） |
| `augmentation` | `variants_per_seed`(1–100)、`num_threads`(1–100)、`auto_save_interval`(≥1)、重试五档（`max_retries`/`retry_delay`/`max_retry_wait`/`retry_jitter`/`request_timeout`） |
| `quality` | `threshold`(0.6)、`weights`[0.3,0.4,0.3] |
| `dedup` | `threshold`(0.9) |
| `versioning` | `storage_dir: data/versions`、`auto_snapshot` |
| `vector` | `backend`(faiss/chromadb)、`dimension`(384)、`storage_dir`、`collection` |
| `web` | `host`/`port`/`static_dir`、`data_roots`（白名单，默认 `["data"]`）、`cors_*`、`max_upload_bytes`(256MiB)、限流三键 |

**写操作鉴权**：环境变量 `AUGMENTOR_API_KEY`（非配置文件项）。

---

## 6. 目录结构（服务端）

```
augmentor/
├── api/
│   ├── main.py                  应用装配（中间件/路由/静态/异常兜底）
│   ├── deps.py                  管道单例/白名单/鉴权/异步IO/异常映射
│   ├── middleware/
│   │   ├── rate_limit.py       滑动窗口限流（RateLimiter）
│   │   ├── trace.py            请求追踪（X-Request-ID）
│   │   ├── upload_gate.py      上传体尺寸闸（解析前 413）
│   │   └── write_auth_gate.py  匿名写传输闸（未设 key 时只放行回环写）
│   └── routes/                 data/ dataset_tools/ augment/ quality/ export/
│                                version/ config/ multimodal/ privacy/ leakage/
│                                audit/ status/ system_ops/ vector/ language/
└── augmentor/
    ├── __init__.py            对外导出面（__version__=3.0.0）
    ├── pipeline.py            AugmentorPipeline（编排中枢）
    ├── config.py              dataclass 配置中心 + load_config
    ├── models/  vector/  data/  frameworks/
    ├── cli/                  CLI 实现（见 04）
    └── *.py                  能力模块（质量/去重/导出/版本/审计/脱敏/泄漏/检索/备份/…）
```
