# AI 训练数据增强工具

用于生成房产客服问答场景的 AI 模型训练数据，基于多种 LLM API 和 NLP 增强技术，将少量种子数据扩充为多轮问答对。

## 功能特性

- **多模型支持**: 百度 ERNIE、OpenAI、Ollama、Claude、Gemini
- **智能增强**: 自动将种子数据扩充为多种提问方式
- **质量控制**: 语义相似度、回答相关性、多样性多维评分
- **智能去重**: 基于向量相似度的智能去重
- **断点续传**: 支持中断后继续处理
- **多格式导出**: JSONL、Llama-Factory、Alpaca、ShareGPT、ChatML、CSV
- **流式处理**: 支持大数据集的分块处理
- **数据集对比**: A/B测试对比功能
- **数据集验证**: 格式验证和数据清洗
- **格式转换**: 多种格式之间的转换
- **隐私脱敏**: 可配置 PII 模式（含增强模式），输出命中明细
- **泄漏检测**: 训练集/测试集之间的完全重复与模糊重复检测
- **就绪审计**: 把脱敏、重复、空字段、泄漏信号合成一个 go / no-go 判定
- **数据画像与离群点**: 字段完整度、长度分布、高频关键词；Z-Score / IQR 长度离群检测
- **HTTP API + Web 界面**: 68 个 REST 端点，全部声明响应契约；React 前端 11 个功能页

## 数据格式

训练数据为 JSON 数组，每条记录包含以下字段：

```json
{
  "input": "",
  "instruction": "用户提问内容",
  "output": "客服回答内容"
}
```

## Web 界面

前端在 `web/`（React 18 + Ant Design 5 + Vite）。

```bash
cd web
npm install
npm run dev      # http://localhost:3000，/api 自动代理到 :8000
```

```bash
npm run build    # tsc --noEmit && vite build → web/dist
npm run lint     # eslint --max-warnings 0
npm test         # vitest run
```

功能页：

| 路径 | 页面 | 覆盖能力 |
|------|------|----------|
| `/dashboard` | 仪表盘 | 健康状态、模型列表、版本快照、快速增强入口 |
| `/` | 数据管理 | 文件列表、分页与搜索、编辑/删除、上传、载入演示数据、按格式导出 |
| `/augment` | 数据增强 | 启动增强任务、进度、断点列表 |
| `/quality` | 质量中心 | 评估 / 去重 / 报告 / 清洗 / 标注 / 基准 / 离群点检测 / 数据画像 |
| `/security` | 数据安全 | 隐私脱敏 / 泄漏检测 / 就绪审计 |
| `/export` | 导出中心 | 格式清单、预览、批量导出 |
| `/analysis` | 数据分析 | 覆盖度分析、统计、可视化图表 |
| `/multimodal` | 多模态 | 单条处理、目录扫描 |
| `/versions` | 版本管理 | 版本列表、对比、回滚、历史 |
| `/settings` | 配置管理 | 读取 / 覆盖配置、模型列表 |
| `/system` | 系统状态 | 服务综合状态、依赖明细、降级功能 |

> 服务层（`web/src/services/api.ts`）有一道**接线守门**测试：`api.ts` 导出的
> 每个函数都必须在页面源码里被真正引用（`import` 语句与注释都不算）。
> 「加了封装却忘了接页面」这类漏法类型检查、lint、覆盖率都发现不了。

## HTTP API

FastAPI 提供 **68 个端点**，全部声明 `response_model` —— `/openapi.json` 里
不存在「无 schema 的 200 响应」。契约由
`tests/integration/test_api_openapi_contract.py` 双向守门：既要每个端点都声明
契约，也要每个已声明契约的端点都真实存在。

```bash
uvicorn api.main:app --host 0.0.0.0 --port 8000
```

交互式文档见 `/docs`，端点总览与字段说明见 [API 文档](docs/API.md)。

> **3.0 破坏性变更（API 面）**
>
> 1. **所有端点现在都声明 `response_model`。** 响应会按 schema 过滤：模型里
>    未声明的键会被**静默裁掉**。绝大多数端点的字段集与改动前一致（模型是按
>    真实响应逐个探测建的，不是读代码猜的），但从此「响应里有什么」由契约
>    决定，不再由 handler 的 `return` 决定。
> 2. `GET /api/augment/progress` 启用 `response_model_exclude_none=True`：
>    某个任务字段恰为 `null` 时，**该键会被整个省略**。这是为了让「无任务」
>    与「有任务」两种形态共用同一份 OpenAPI 契约而做的取舍。
> 3. `GET /api/demo/data` 不再返回 `JSONResponse`（那会绕过 `response_model`，
>    让契约退化成装饰）。响应形态不变，仍是**裸数组**。
> 4. **配置文件写入闭环修好了，写法有变**：`save_config` 现在把默认模型写进
>    `models.default`（`load_config` 唯一认的键），不再写顶层 `default_model`；
>    旧文件里的顶层 `default_model` 会被归一化进 `models.default`。
> 5. **密钥不再明文落盘**：写配置时，模型密钥只在原文件里是 `${ENV_VAR}`
>    占位符时才保留，否则从输出里移除。
> 6. `ConfigValidator` 里 `app` / `app.name` / `app.version` 由**必填降为可选**。
>    以前 `save_config` 写出的配置**必然校验失败**（`AppConfig` 不建模 `app` 段、
>    `load_config` 也从不读它），所以这是修 bug，不是放松校验。`models` /
>    `models.default` 仍为必填。
> 7. 新增两个路由分组：`dataset`（12 个端点，CLI 数据集工具的 API 化）与
>    `system`（13 个端点，依赖诊断 / 备份 / 监控 / 迁移等运维能力）。
>    写操作走 `verify_api_key` 鉴权。

## 项目结构

```
augmentor/
├── augmentor/          # 核心增强包
│   ├── models/         # 模型后端
│   ├── data/           # 数据处理
│   ├── vector/         # 向量数据库
│   ├── frameworks/     # 框架集成
│   ├── config.py       # 配置管理
│   ├── pipeline.py     # 增强管道
│   ├── quality.py      # 质量评分
│   ├── dedup.py        # 智能去重
│   ├── export.py       # 多格式导出
│   ├── export_enhanced.py  # 增强导出
│   ├── streaming.py    # 流式处理
│   ├── comparison.py   # 数据集对比
│   ├── compare_enhanced.py  # 增强比较
│   ├── dataset_ops.py  # 数据集操作
│   ├── validation.py   # 数据验证
│   ├── converter.py    # 格式转换
│   ├── indexer.py      # 数据索引
│   ├── cache.py        # 缓存管理
│   ├── config_validator.py  # 配置验证
│   ├── analytics.py    # 数据分析
│   ├── cleaner.py      # 数据清洗
│   ├── quality_report.py  # 质量报告
│   ├── visualize_enhanced.py  # 增强可视化
│   ├── backup.py       # 数据备份
│   ├── search_enhanced.py  # 增强搜索
│   ├── statistics.py   # 数据统计
│   ├── version_control.py  # 版本控制
│   └── exceptions.py   # 自定义异常
├── web/                # Web前端 (React + Ant Design)
├── tests/              # 测试套件
├── docs/               # 文档
├── docker/             # Docker配置
├── archive/            # 归档文件
├── cli.py              # CLI入口
└── config.yaml         # 配置文件
```

## 环境要求

- Python 3.8+
- 依赖包见 `requirements.txt`

## 环境变量

运行前需设置模型 API 密钥（根据使用的模型）：

```bash
# 百度 ERNIE
export BAIDU_API_KEY=<your_api_key>
export BAIDU_SECRET_KEY=<your_secret_key>

# OpenAI
export OPENAI_API_KEY=<your_api_key>

# Anthropic Claude
export ANTHROPIC_API_KEY=<your_api_key>

# Google Gemini
export GOOGLE_API_KEY=<your_api_key>
```

## CLI 使用方法

### 基本命令

```bash
# 增强数据集
python cli.py augment --input train_data.json --output augmented_data.json

# 导出数据集
python cli.py export --input data.json --output-dir exports/

# 质量评估
python cli.py quality --input data.json --report quality_report.md

# 数据清洗
python cli.py clean --input raw_data.json --output clean_data.json

# 数据集对比
python cli.py compare --dataset-a data_v1.json --dataset-b data_v2.json

# 流式处理
python cli.py stream --input large_data.json --output processed.json --operation quality
```

### 数据集操作

```bash
# 合并数据集
python cli.py merge --inputs data1.json data2.json --output merged.json

# 采样数据集
python cli.py sample --input data.json --output sampled.json --size 1000

# 分层采样（按指定字段分组配额，字段留空会报错而不是退化成随机挑条）
python cli.py sample --input data.json --output sampled.json --method stratified \
    --size 1000 --stratify-key instruction

# 分割数据集
python cli.py split --input data.json --output-dir splits/

# 分层分割（--stratify 委托 DataSplitter；--no-shuffle 只回排段内顺序）
python cli.py split --input data.json --output-dir splits/ --stratify \
    --stratify-key instruction --seed 7

# 数据集统计
python cli.py stats --input data.json
```

### 验证和转换

```bash
# 验证数据集格式（判负 ⇒ 退出码 1；只有 warning 时不判负）
python cli.py validate --input data.json --preset basic

# 转换数据格式
python cli.py convert --input data.json --output data.jsonl --format jsonl

# 搜索数据集
python cli.py search --input data.json --query "租房" --method contains

# 验证配置文件（报 ERROR ⇒ 退出码 1；「写了没人读」的键只出声、不判负）
python cli.py validate-config --config config.yaml

# 数据分析（JSON：洞察 + 质量/多样性/完整性分数 + 覆盖分析 + 去重报告）
python cli.py analyze --input data.json
python cli.py analyze --input data.json --output analysis_report.json

# 数据清洗（规则式：--rules 可多选）
# 默认规则：去空 / 去重 / 去URL / 去HTML标签 / 去控制字符 / 空白归一化 / 标点归一化 / 去首尾空白
python cli.py clean --input data.json --output cleaned.json
python cli.py clean --input data.json --output cleaned.json --rules remove_empty remove_duplicates
# 保留 URL：把 remove_urls 从 --rules 里去掉（等价于旧的 --no-url-removal）
python cli.py clean --input data.json --output cleaned.json --rules remove_empty trim_whitespace

# 数据导出（单文件用 --output + --format；批量用 --output-dir + --formats）
python cli.py export --input data.json --output exported.json --format alpaca
python cli.py export --input data.json --output-dir out/ --formats jsonl csv
python cli.py export --input data.json --output sample.json --max-items 100 --shuffle --seed 42

# 质量报告
python cli.py quality-report --input data.json --output quality_report.json

# 数据可视化（不传 --output 时生成图表；传 --output 时输出文本/JSON 报告）
python cli.py visualize --input data.json --output-dir visualizations/
python cli.py visualize --input data.json --output report.json --format json

# 数据备份
python cli.py backup --action create --input data.json --name my_backup
python cli.py backup --action list
python cli.py backup --action restore --name my_backup --output restored.json
python cli.py backup --action delete --name my_backup

# 搜索（exact / contains / ngram / fuzzy / regex，支持 --offset 分页）
python cli.py search --input data.json --query "租房" --method contains
python cli.py search --input data.json --query "租房" --method fuzzy
python cli.py search --input data.json --query "租房" --method ngram --limit 20 --offset 10

# 统计（逐字段填充率 / 平均长度 / 唯一值 + 整体质量指标）
python cli.py stats --input data.json
python cli.py stats --input data.json --output stats.json

# 比较（质量向 A/B 结论 + 重叠度对比 + 建议；名称缺省取文件名 stem）
python cli.py compare --dataset-a data1.json --dataset-b data2.json
python cli.py compare --dataset-a data1.json --dataset-b data2.json --output comparison.json
python cli.py compare --dataset-a data1.json --dataset-b data2.json --name-a 线上集 --name-b 候选集

# 版本管理（单一后端 DatasetVersionManager，独立 --versions-dir）
python cli.py version --action create --input data.json --description "初始版本"
python cli.py version --action list
python cli.py version --action history          # 操作日志（JSON，按时间倒序）
python cli.py version --action compare --version v1.0.0 --version-b v1.0.1
python cli.py version --action compare --version v1.0.0   # 缺省与当前版本比较
python cli.py version --action load --version v1.0.0 --output loaded.json
python cli.py version --action rollback --version v1.0.0
```

> **3.0 破坏性变更（命令面）**
>
> 1. 旧的 `*-enhanced` / `*-data` / `version-control` 命令名已**彻底移除**
>    （此前会打印弃用提示并自动映射）。传旧名会被解析器直接拒绝（退出码 2）。
> 2. `--enhanced` 开关本身也已**移除**。它曾经只是「同一能力的两种实现」的
>    选择器：不影响能力，只影响走哪条代码路径。现在 8 对命令各自合并为一套
>    实现，落败侧独有的能力已并进幸存实现：
>
>    | 命令 | 实现 | 并入的独有能力 |
>    | --- | --- | --- |
>    | `export` | `Exporter`（`ExportFormat` 全量格式，含 `raw`） | 单文件模式、`--max-items` / `--shuffle` / `--seed` |
>    | `clean` | 规则式 `DatasetCleaner` | 旧 `DataCleaner` 的噪声清除（`remove_urls` / `remove_html_tags` / `remove_control_chars` 三条规则），旧 `--no-url-removal` → `--rules` 里去掉 `remove_urls` |
>    | `analyze` | `analytics.DatasetAnalyzer` | 旧 `pipeline.analyze_dataset` 的 `coverage_analysis` / `statistics` / `dedup_report` 三段 |
>    | `stats` | `statistics.DatasetStatisticsCalculator` | —（本就是旧 `get_statistics` 的严格超集） |
>    | `visualize` | 报告模式 + 图表模式 | 两种模式并存（由 `--output` 是否给出决定） |
>    | `compare` | `compare_enhanced`（重叠度） | 旧 `comparison.DatasetComparator` 的质量 / 长度 / 词汇对比与获胜方结论，`--name-a` / `--name-b` |
>    | `search` | `EnhancedSearcher` | `ngram` 方法（已移植进 `EnhancedSearcher`） |
>    | `version` | `DatasetVersionManager` | `rollback`（映射到 `set_current_version`）、`history`（新增操作日志 `history.jsonl`）、`delete` |
>
>    3. **stdout 契约不变**：报告型命令的 stdout 仍是机器可读的——`clean` /
>    `analyze` / `stats` 为 JSON，`compare` 为 markdown 摘要，`search` 为
>    「摘要 + 条目 JSON 列表」。`--output` 只把同一份内容落盘，提示语写 stderr，
>    不掺进 stdout。
>
>    另外：`version` 的 `diff` 并入 `compare`。**`stats` 的键名有变**：合并后
>    输出 `statistics.DatasetStatisticsCalculator.to_dict()` 的形状
>    （`total_items` + 嵌套的 `field_statistics`），不再是旧
>    `dataset_ops.get_statistics` 的顶层 `total` / `avg_length` / `min_length` /
>    `max_length` / `unique_instructions`——后者的信息是前者的子集。
>
> `quality` 与 `quality-report` 未合并：前者筛数据、后者出报告，是两种能力。

### 高级功能

```bash
# 版本管理
python cli.py version --action list
python cli.py version --action create --input data.json

# 可视化
python cli.py visualize --input data.json --output-dir visualizations/

# 质量基准
python cli.py benchmark --input data.json --save-baseline

# RAG 格式转换
python cli.py rag --input data.json --output rag_data.json --format llamaindex
```

## 配置说明

配置文件 `config.yaml` 包含所有可配置项：

```yaml
# 模型配置
models:
  default: ernie
  ernie:
    type: baidu
    api_key: ${BAIDU_API_KEY}
    # ...

# 增强配置
augmentation:
  variants_per_seed: 5
  num_threads: 40

# 质量配置
quality:
  enabled: true
  threshold: 0.6
```

## 开发指南

### 运行测试

```bash
python -m pytest tests/ -v
```

### 代码覆盖率

```bash
python -m pytest tests/ --cov=augmentor --cov-report=html
```

## 注意事项

- 模型 API 有调用频率限制和费用，请合理控制并发数
- 日志输出到 `app.log`，仅 ERROR 级别日志打印到控制台
- 原始种子数据包含业务敏感信息，请勿对外泄露

## 文档

详细文档请参阅 `docs/` 目录：
- [API 文档](docs/API.md)
- [架构文档](docs/ARCHITECTURE.md)
- [部署文档](docs/DEPLOYMENT.md)
