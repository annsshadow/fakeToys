# 企业级 AI 训练数据平台 — 使用指南

从数据准备到模型评估的完整工作流平台。在原有单脚本工具 `enhanceTXT.py` 的基础上重构为分层架构，
提供数据增强、质量管理、版本控制、多格式导出、多模态处理、REST API 与 Web UI。

- [架构说明](ARCHITECTURE.md)
- [API 文档](API.md)
- [部署指南](DEPLOYMENT.md)

---

## 1. 环境要求

| 项目 | 要求 |
|------|------|
| Python | 3.10+（开发验证环境为 3.13） |
| Node.js | 18+（仅构建 Web UI 时需要） |
| 操作系统 | Windows / Linux / macOS |

### 依赖分层

`requirements.txt` 按用途分组，**核心依赖为必需，其余均可选**。所有可选依赖都实现了降级路径：
缺失时功能自动退化，不会导致进程崩溃。

| 分组 | 包 | 缺失时的行为 |
|------|-----|-------------|
| 核心 | `requests` `pyyaml` `numpy` `pydantic>=2` | 必需，无降级 |
| Web | `fastapi` `uvicorn` `python-multipart` | REST API 不可用 |
| 测试 | `pytest` `pytest-cov` `httpx` | 无法运行测试 |
| 语义模型 | `sentence-transformers` | 回退到 n-gram 相似度 |
| 向量库 | `faiss-cpu` | 回退到 numpy 精确检索 |
| 向量库 | `chromadb` | 该后端构造时抛 `ImportError`，`faiss` 仍可用 |
| 可视化 | `matplotlib` `wordcloud` `scikit-learn` | 对应图表跳过并返回空路径 |
| 多模态 | `Pillow` `soundfile` | 回退到标准库解析文件头（PNG/GIF/BMP/JPEG/WAV） |
| LLM 框架 | `langchain` `llama-index` | 返回等价的普通字典结构 |

## 2. 安装

```bash
cd augmentor

# 推荐使用虚拟环境
python -m venv .venv
source .venv/bin/activate        # Windows: .venv\Scripts\activate

# 仅安装核心依赖（最小可用）
pip install requests pyyaml numpy "pydantic>=2" fastapi uvicorn python-multipart

# 或安装全部依赖（含可选功能）
pip install -r requirements.txt
```

## 3. 环境变量

模型后端在**构造阶段**即校验密钥，缺失会直接报错。至少配置一个后端：

```bash
# 百度 ERNIE（默认后端）
export BAIDU_API_KEY=<your_api_key>
export BAIDU_SECRET_KEY=<your_secret_key>

# OpenAI
export OPENAI_API_KEY=<your_api_key>

# Anthropic Claude
export ANTHROPIC_API_KEY=<your_api_key>

# Google Gemini
export GOOGLE_API_KEY=<your_api_key>
```

Windows CMD：

```cmd
set BAIDU_API_KEY=<your_api_key>
set BAIDU_SECRET_KEY=<your_secret_key>
```

Ollama 本地模型无需密钥，只需保证服务可访问（默认 `http://localhost:11434`）。

配置文件 `config.yaml` 中通过 `${VAR}` 语法引用环境变量：

```yaml
models:
  default: ernie
  ernie:
    type: baidu
    api_key: ${BAIDU_API_KEY}
    secret_key: ${BAIDU_SECRET_KEY}
```

未设置的环境变量会被替换为空字符串，而不是让整个配置加载失败。

## 4. 数据格式

输入为标准 JSON 数组：

```json
[
  {
    "input": "",
    "instruction": "用户提问内容",
    "output": "客服回答内容"
  }
]
```

多轮对话数据额外携带 `history` 与 `system`：

```json
{
  "instruction": "那退租要提前多久？",
  "input": "",
  "output": "退租需提前 30 天在 App 内提交申请。",
  "history": [
    {"role": "user", "content": "怎么申请入住？"},
    {"role": "assistant", "content": "通过官方 App 提交申请。"}
  ],
  "system": "你是安居乐寓的智能客服"
}
```

## 5. 快速开始

### 5.1 数据增强

```bash
python cli.py augment --input train_data.json --output train_data_augmented.json
```

默认启用质量检查、去重与断点续传。逐项关闭：

```bash
python cli.py augment --input train_data.json --output out.json \
    --no-quality --no-dedup --no-checkpoint
```

### 5.2 断点续传

任务 ID 由**输入文件路径**派生（`augment_<md5前12位>`），因此同一输入文件的重复运行会自动续传：

- 已完成（含失败）的条目会被跳过；
- 上一轮已写入的输出会被载入并与本轮结果合并，不会覆盖为空。

如需从头重跑，先删除断点：

```bash
python -c "from augmentor import AugmentorPipeline; \
AugmentorPipeline().checkpoint_manager.delete_checkpoint( \
    AugmentorPipeline.build_task_id('train_data.json'))"
```

查看断点列表：`GET /api/augment/checkpoints`。

### 5.3 质量评估与报告

```bash
# 评估 + 输出过滤后的数据 + 生成 Markdown 报告
python cli.py quality --input train_data.json \
    --output train_data_passed.json \
    --report quality_report.md
```

报告包含：总分分布直方图、通过率、各分项均值、过滤统计、去重汇总与改进建议。
报告路径以 `.json` 结尾时输出 JSON 结构，否则输出 Markdown。

### 5.4 导出

```bash
# 导出指定格式
python cli.py export --input train_data.json --output-dir exports \
    --formats jsonl alpaca sharegpt

# 不指定 --formats 则导出全部支持的格式
python cli.py export --input train_data.json --output-dir exports
```

导出前预览转换结果（不改动磁盘数据）：

```bash
python cli.py preview --input train_data.json --format sharegpt --size 5
```

### 5.5 数据清洗

```bash
python cli.py clean --input raw.json --output cleaned.json

# 保留 URL：从默认规则里去掉 remove_urls
python cli.py clean --input raw.json --output cleaned.json \
  --rules remove_empty remove_duplicates remove_html_tags remove_control_chars \
            normalize_whitespace remove_special_chars trim_whitespace
```

清洗内容包括：HTML 标签与实体、URL、零宽字符、控制字符、空白折叠，以及全角标点归一化为半角。

### 5.6 自动标注

```bash
python cli.py annotate --input train_data.json --output annotated.json
```

为每条数据附加：实体（电话/邮箱/URL/金额/日期/身份证）、意图分类（how_to / what_is / why /
comparison / price / policy / troubleshooting / greeting）、情感倾向。

### 5.7 RAG 数据格式

```bash
python cli.py rag --input train_data.json --output rag_data.json --format llamaindex
```

`--format` 可选 `llamaindex` / `langchain` / `custom`。

### 5.8 质量基准

```bash
# 首次运行：把当前指标保存为基准
python cli.py benchmark --input train_data.json --save-baseline

# 后续运行：与基准对比
python cli.py benchmark --input train_data_new.json --report benchmark.md
```

基准中不存在的指标会标记为 `no_baseline`，**不参与** improved/regressed 统计，避免被误判为 0 造成虚假提升。

### 5.9 版本管理

```bash
python cli.py version --action list
python cli.py version --action create --input train_data.json
python cli.py version --action compare --version <v1> --version-b <v2>
python cli.py version --action rollback --version <v1>
python cli.py version --action history
```

`history` 读取 `history.jsonl`，按时间倒序返回 create / delete / rollback 操作记录。

### 5.10 分析与可视化

```bash
python cli.py analyze --input train_data.json
python cli.py visualize --input train_data.json --output-dir visualizations
```

## 6. 启动 REST API 与 Web UI

### 后端

```bash
uvicorn api.main:app --host 0.0.0.0 --port 8000
```

交互式 API 文档：<http://localhost:8000/docs>

### 前端

```bash
cd web
npm install
npm run build          # 产物输出到 web/dist
```

`api/main.py` 检测到 `web/dist` 存在时会自动挂载为静态目录，直接访问
<http://localhost:8000> 即可。开发模式：

```bash
npm run dev            # Vite 开发服务器，热更新
```

## 7. 配置说明

配置文件为 `config.yaml`，通过 `--config` 指定其他路径。主要配置段：

| 配置段 | 关键项 | 说明 |
|--------|--------|------|
| `models` | `default` | 默认模型后端，可选 `ernie` / `openai` / `ollama` / `claude` / `gemini` |
| `augmentation` | `variants_per_seed` `num_threads` `auto_save_interval` | 变体数量、并发线程、增量保存间隔 |
| `quality` | `threshold` `weights` | 质量阈值；权重为 `[语义相似度, 回答相关性, 多样性]`，默认 `[0.3, 0.4, 0.3]` |
| `dedup` | `threshold` | 相似度高于此值判定为重复，默认 `0.9` |
| `export` | `default_format` `formats` | 导出格式清单 |
| `versioning` | `storage_dir` `auto_snapshot` | 版本存储目录与增强后自动快照 |
| `multilingual` | `default_target_lang` `supported_langs` | 翻译增强 |
| `rag` | `chunk_size` `chunk_overlap` | 分块大小与重叠，要求 `chunk_overlap < chunk_size` |
| `evaluation` | `metrics` `reference_field` | BLEU / ROUGE-L / 相似度 |
| `vector` | `backend` `dimension` | `faiss` 或 `chromadb` |
| `multimodal` | `image_extensions` `audio_extensions` | 支持的文件扩展名 |
| `benchmark` | `baseline_file` `metrics` | 基准文件与指标清单 |
| `active_learning` | `strategy` `batch_size` `max_iterations` | `uncertainty` / `diversity` / `hybrid` / `random` |
| `frameworks` | `frameworks` | `langchain` / `llamaindex` |
| `web` | `host` `port` `static_dir` | Web 服务监听地址与静态目录 |
| `logging` | `level` `file` | 日志级别与输出文件 |

### 使用 Python API

```python
from augmentor import AugmentorPipeline

pipeline = AugmentorPipeline()          # 读取默认 config.yaml

report = pipeline.augment_dataset(
    "train_data.json",
    "train_data_augmented.json",
    use_checkpoint=True,
    use_quality_check=True,
    use_dedup=True,
    use_parallel=True,
)

print(report["input_count"], "->", report["output_count"])
print(report["progress"]["progress_percent"])
```

## 8. 运行测试

```bash
pytest                       # 使用 pytest.ini 中的默认参数
pytest -m "not slow"         # 只跑快速用例
```

`pytest.ini` 已配置：

- `--cov=augmentor --cov=api --cov=cli`
- `--cov-fail-under=80`（覆盖率低于 80% 时退出码非 0）
- `--cov-report=term-missing:skip-covered`
- `--cov-report=xml:coverage.xml`

覆盖率不足 80% 会直接判定失败。查看 HTML 报告：

```bash
pytest --cov-report=html:htmlcov
```

## 9. 常见问题

**Q：启动时报 `ImportError: ChromaDB 后端需要安装 chromadb`**
A：`pip install chromadb`，或把 `config.yaml` 中 `vector.backend` 改为 `faiss`。

**Q：相似度评分很低，变体大量被过滤**
A：未安装 `sentence-transformers` 时会回退到 n-gram 相似度，与语义相似度差异较大。
安装 `sentence-transformers` 可获得更准确的语义评分；或调低 `quality.threshold`。

**Q：可视化没有生成任何图片**
A：`matplotlib` / `wordcloud` / `scikit-learn` 缺失时对应图表会被跳过并返回空路径，
属预期降级行为。安装对应依赖即可。

**Q：重复运行 `augment` 没有产出新数据**
A：断点续传识别到同一输入文件已处理完成，跳过了全部条目并复用已有输出。
如需重跑，先删除断点（见 5.2）。

**Q：前端页面打开是空白**
A：确认已执行 `npm run build` 且 `web/dist` 存在；后端仅在静态目录存在时挂载它。
未构建时可直接使用 `npm run dev` 启动开发服务器。

**Q：Windows 上中文词云乱码**
A：词云依赖 `C:/Windows/Fonts/simhei.ttf`，请确认该字体存在。

## 10. 注意事项

- 百度 API 有调用频率限制与费用，请合理控制 `augmentation.num_threads`
- 原始种子数据包含业务敏感信息，请勿对外泄露
- 日志默认输出到 `app.log`
- `checkpoints/` `experiments/` `visualizations/` 目录会在管道初始化时自动创建
