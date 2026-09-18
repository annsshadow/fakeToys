# AI 数据增强工具 - 使用示例

## 数据集健康度评分

```python
from augmentor.health_score import DatasetHealthScore

scorer = DatasetHealthScore(weights=[0.25, 0.25, 0.25, 0.25])
result = scorer.score(dataset_items)
print(f"健康分数: {result['health_score']}, 等级: {result['level']}")
```

## 内存使用监控

```python
from augmentor.memory_monitor import MemoryMonitor, monitor_memory_usage

@monitor_memory_usage
def process():
    return {"status": "ok"}

monitor = MemoryMonitor(interval_mb=10)
monitor.take_snapshot()
```

## 数据质量趋势追踪

```python
from augmentor.quality_trend import QualityTrendTracker

tracker = QualityTrendTracker()
tracker.record_quality_metrics("dataset_v1", {"completeness": 0.9})
trend = tracker.calculate_trend_direction("completeness", "dataset_v1")
print(trend)  # improving / stable / declining
```

## 领域自适应增强

```python
from augmentor.expander import DomainExpander
from augmentor.models import create_model_backend

expander = DomainExpander(model_backend=create_model_backend(config))
result = expander.generate_adaptive_expansion(items)
print(result.strategy, result.expanded_topics)
```

## CSV/Excel 导入

```python
from augmentor.csv_excel_import import import_dataset

data = import_dataset("seed_data.csv")
```

## 异步增强管道

```python
import asyncio
from augmentor.pipeline import AugmentorPipeline

pipeline = AugmentorPipeline()
result = asyncio.run(pipeline.augment_async(
    input_file="input.json",
    output_file="output.json",
    use_quality_check=True
))
```
