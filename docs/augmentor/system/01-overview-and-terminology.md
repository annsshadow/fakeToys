# 01 · 平台总览与术语说明

## 1. 平台定位

`augmentor` 是一个 **AI 模型训练数据增强平台**，把少量「种子数据」加工为可进训练 / 微调的
**大样本、多形态、经过质量门禁**的数据集。它由三端组成，共享同一套核心引擎能力：

- **服务端**（`augmentor/` + `api/`）：纯 Python 核心引擎 + FastAPI REST 接口，是所有能力的唯一实现来源。
- **客户端 / Web 管理端**（`web/`）：React SPA，把服务端能力做成可点选的操作台（数据管理、增强、质量、安全、导出、分析、多模态、版本、配置、系统运维）。
- **命令行端 / CLI**（`cli.py` + `augmentor/cli/`）：37 个子命令，面向脚本、CI、批处理与离线环境。

**核心设计原则：分层单向依赖。** 核心引擎不依赖任何 Web 框架；`api` 与 `cli` 都是核心引擎的
**薄适配层**。因此同一能力在 Python API、HTTP、命令行三种驱动方式下行为一致。

```
┌──────────────────────────────────────────────────────────────────────┐
│ 表现层（端）                                                             │
│  Web UI (web/)        CLI (cli.py)        REST API (api/)            │
├──────────────────────────────────────────────────────────────────────┤
│ 编排层                                                           │
│  augmentor/pipeline.py  AugmentorPipeline（组装并调度全部能力模块）        │
├──────────────────────────────────────────────────────────────────────┤
│ 能力层                                                           │
│  生成: models/ context/ expander/  质量: quality/ dedup/ benchmark/      │
│  数据: data/                转换: export/ rag/ multilingual/ converter/  │
│  存储: versioning/ checkpoint/ vector/   评估: evaluation/ active_learning/ │
├──────────────────────────────────────────────────────────────────────┤
│ 基础设施层                                                       │
│  config.py（配置中心）  model_manager.py（语义模型单例）  visualizer.py   │
└──────────────────────────────────────────────────────────────────────┘
```

依赖方向**严格单向向下**：`api` / `cli` → `pipeline` → 能力层 → 基础设施层。
能力层内部**不反向**依赖 `pipeline`，保证可独立测试与复用。

---

## 2. 三端职责对照

| 维度 | 服务端（核心引擎 + API） | 客户端（Web 管理端） | 命令行端（CLI） |
|------|--------------------------|----------------------|-----------------|
| 代码位置 | `augmentor/`、`api/` | `web/` | `cli.py`、`augmentor/cli/` |
| 语言/框架 | Python 3.10+ / FastAPI / Pydantic v2 | React 19 / TypeScript / Vite 8 / Ant Design 6 | Python / argparse |
| 运行 | `uvicorn api.main:app`（:8000） | `npm run dev`（:3000）或 `npm run build` 后由后端静态托管 | `python cli.py <cmd>` |
| 职责 | 实现全部能力、暴露 77 个 REST 端点、挂载前端静态产物 | 把能力做成表单/图表/表格，驱动 REST | 把能力做成子命令，离线/脚本化驱动 |
| 数据存储 | 本地 JSON + 可选向量库（faiss/chromadb）+ 断点/版本目录 | 无自有存储，全走 API | 同服务端，读本地文件 |

> **没有移动端**：仓内无原生 App / React Native / Flutter / Ionic 代码或产物。

---

## 3. 术语表（Glossary）

按出现频率与重要性排列，供三端文档统一引用。

### 3.1 数据形态

| 术语 | 含义 |
|------|------|
| **种子数据 / seed** | 输入的少量问答对（JSON 数组），平台增强的起点 |
| **变体 / variant** | 由一条 seed 经 LLM 生成的等价/改写问答对，`augmentation.variants_per_seed` 控制数量 |
| **数据集 / dataset** | 一份 JSON 数组文件，是平台一切操作的默认输入/输出单元 |
| **数据项 / record / item** | 数据集里的一条对象，标准字段 `instruction`（问）/ `output`（答）/ `input`（多轮上下文，常空） |
| **多轮对话数据** | 额外带 `history`（role/content 列表）与 `system`（系统设定）的数据项 |
| **导出格式 / ExportFormat** | `json / jsonl / csv / tsv / alpaca / sharegpt / chatml / llama_factory / vicuna / belle / openai / huggingface / raw` 等（**权威清单**在 `ExportFormat` 枚举，运行时 `GET /api/export/formats` 可查） |

### 3.2 质量与门禁

| 术语 | 含义 |
|------|------|
| **质量评分 / quality score** | 对变体打的三维分：**语义相似度、回答相关性、多样性**，加权（默认 `0.3/0.4/0.3`），低于 `quality.threshold`（默认 0.6）的样本被过滤 |
| **质量门禁 / health gate** | 把「这份数据能不能进训练」压成单一判决，`health-gate` 命令 / `POST /api/quality/health-gate`，判负时退出码/标志置位 |
| **去重 / dedup** | 基于向量相似度的去重，`dedup.threshold`（默认 0.9）以上判为重复 |
| **离群点 / outlier** | 长度异常样本检测（Z-Score / IQR），`POST /api/quality/outliers` |
| **数据画像 / profiling** | 字段完整度、长度分布、高频关键词的统计侧写，`POST /api/quality/profiling` |
| **基准 / benchmark** | 质量指标基线，`benchmark` 命令与 `POST /api/quality/benchmark`；缺失指标记 `no_baseline` 不参与提升/回归统计 |

### 3.3 安全与合规

| 术语 | 含义 |
|------|------|
| **PII 脱敏 / sanitize** | 用可配置 PII 模式（电话/邮箱/URL/金额/日期/身份证/信用卡等）替换敏感值并输出命中明细 |
| **泄漏检测 / leakage** | 训练集与测试集间的完全重复与模糊重复检测 |
| **就绪审计 / audit** | 把脱敏、重复、空字段、泄漏信号合成单一 go / no-go 判决 |
| **写操作鉴权** | 环境变量 `AUGMENTOR_API_KEY`；设置后所有写操作要求请求头 `X-API-Key`，不匹配返回 401 |
| **数据白名单 / data roots** | `web.data_roots` 定义的目录白名单，客户端路径 resolve 后必须落在其中，否则 403 |

### 3.4 工程与部署

| 术语 | 含义 |
|------|------|
| **管道 / pipeline** | `AugmentorPipeline`，编排中枢，组装并调度全部能力模块 |
| **断点 / checkpoint** | 增强任务的中断续传单元，任务 ID 由输入文件路径派生（`augment_<md5前12位>`） |
| **版本 / version** | 数据集快照与回滚单元（`DatasetVersionManager`，独立 `--versions-dir`） |
| **降级 / fallback** | 可选依赖缺失时功能自动退化而不崩溃（如无语义模型 → n-gram 相似度；无 faiss → numpy 精确检索） |
| **依赖分层** | `requirements-core.txt`（最小可运行）/ `requirements.txt`（全量）/ `requirements-dev.txt`（测试） |

---

## 4. 关键业务流程（三端共享）

完整流程见 [`05-interactions-and-dataflow.md`](05-interactions-and-dataflow.md)。这里给出主线：

```
种子数据 → [数据管理: 上传/清洗/校验/转换] 
        → [增强: LLM 生成变体 + 质量评分 + 去重 + 断点续传]
        → [质量中心: 报告 / 基准 / 离群 / 画像 / 健康门禁]
        → [安全: 脱敏 / 泄漏 / 就绪审计]
        → [导出: 多格式 / RAG / 翻译]
        → [版本: 快照 / 回滚 / 历史]
        → 进入训练 / 微调
```
