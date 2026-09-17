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

## 数据格式

训练数据为 JSON 数组，每条记录包含以下字段：

```json
{
  "input": "",
  "instruction": "用户提问内容",
  "output": "客服回答内容"
}
```

## 项目结构

```
ai/
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

# 分割数据集
python cli.py split --input data.json --output-dir splits/

# 数据集统计
python cli.py stats --input data.json
```

### 验证和转换

```bash
# 验证数据集格式
python cli.py validate --input data.json --preset basic

# 转换数据格式
python cli.py convert --input data.json --output data.jsonl --format jsonl

# 搜索数据集
python cli.py search --input data.json --query "租房" --method contains

# 验证配置文件
python cli.py validate-config --config config.yaml

# 数据分析
python cli.py analyze --input data.json --output analysis_report.json

# 增强数据清洗
python cli.py clean-enhanced --input data.json --output cleaned.json --rules remove_empty remove_duplicates

# 增强数据导出
python cli.py export-enhanced --input data.json --output exported.json --format alpaca

# 质量报告
python cli.py quality-report --input data.json --output quality_report.json

# 数据可视化
python cli.py visualize --input data.json --output visualization.txt

# 数据备份
python cli.py backup --action create --input data.json --name my_backup
python cli.py backup --action list
python cli.py backup --action restore --name my_backup --output restored.json
python cli.py backup --action delete --name my_backup
```

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
