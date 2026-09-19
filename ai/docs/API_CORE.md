# 核心模块 API 文档

## AugmentorPipeline

`pipeline.py` — 增强数据集的主流程管道。

### 方法

- `augment_dataset(input_file, output_file, use_checkpoint=True, use_quality_check=True, use_dedup=True, use_parallel=True) -> Dict`
  增强整个数据集，支持断点续传、并行处理、质量检查和去重。

- `augment_async(input_file, output_file, use_checkpoint=True, use_quality_check=True, use_dedup=True, max_workers=4) -> Dict`
  异步增强数据集，使用 `asyncio` 并行处理。

- `augment_seed(seed_item, use_quality_check=True, existing_generated=None) -> List[Dict]`
  增强单条种子数据。

- `analyze_dataset(input_file) -> Dict`
  分析数据集覆盖率、统计信息和去重报告。

## QualityScorer

`quality.py` — 数据质量评分器。

### 方法

- `score(original, generated, output, existing_generated=None) -> QualityScore`
  计算单条数据的质量评分（语义相似度、回答相关性、多样性）。

- `batch_score(items, existing_generated=None) -> List[QualityScore]`
  批量计算质量评分（优化版）。

- `filter_by_quality(items, existing_generated=None) -> List[Dict]`
  根据质量阈值过滤低质量样本。

## DatasetHealthScore

`health_score.py` — 数据集健康度评分。

### 方法

- `score(items) -> Dict`
  计算数据集健康度评分（完整性、多样性、均衡性、覆盖率）。

## MemoryMonitor

`memory_monitor.py` — 内存使用监控。

### 方法

- `take_snapshot() -> Dict`
  记录当前内存使用快照。

- `get_peak_usage_mb() -> float`
  获取内存峰值。

- `get_trend() -> str`
  获取内存使用趋势（increasing / decreasing / stable / fluctuating）。

## StreamProcessor

`streaming.py` — 流式数据处理。

### 方法

- `process() -> Dict`
  执行流式处理，支持分块读取、处理和写入，内置内存优化。

## ContextAugmentor

`context.py` — 对话上下文增强。

### 方法

- `generate_multi_turn(seed_qa, existing_histories=None) -> Dict`
  基于种子问答生成多轮对话数据。

- `analyze_context_intent(history, current_instruction) -> Dict`
  分析上下文意图变化（上下文感知增强功能）。

- `enhance_with_context_awareness(seed_qa, history=None) -> Dict`
  基于上下文感知的增强生成。
