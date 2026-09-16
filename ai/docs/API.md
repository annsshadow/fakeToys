# API 文档

REST API 由 FastAPI 提供，默认监听 `http://localhost:8000`。交互式文档见 `/docs`（Swagger UI）。

```bash
uvicorn api.main:app --host 0.0.0.0 --port 8000
```

## 通用约定

| 项目 | 说明 |
|------|------|
| Base URL | `http://<host>:<port>` |
| 请求体 | `Content-Type: application/json`（上传接口除外） |
| 响应体 | JSON |
| 响应头 | 每个响应携带 `X-Process-Time`（秒，浮点数） |
| 文件参数 | 所有 `input_file` / `filename` 均为**服务端路径**，非 URL |

### 错误码

| 状态码 | 含义 | 触发场景 |
|--------|------|----------|
| 200 | 成功 | — |
| 400 | 请求参数错误 | 索引越界、扩展名不支持、`chunk_overlap >= chunk_size` |
| 404 | 资源不存在 | 文件不存在、版本不存在 |
| 422 | 请求体校验失败 | 缺少必填字段、字段类型错误（由 FastAPI 自动返回） |
| 500 | 服务端错误 | 模型调用失败、依赖缺失、磁盘写入失败 |

错误响应格式统一为：

```json
{"detail": "文件不存在"}
```

### 慢请求

中间件会记录每个请求的处理耗时。超过 **2.0 秒**会写入 WARNING 日志，便于定位性能瓶颈。

---

## 1. 健康检查

### `GET /api/health`

返回服务状态。

```json
{"status": "ok"}
```

---

## 2. 配置与模型

### `GET /api/config`

返回当前生效的关键配置。

```json
{
  "default_model": "ernie",
  "augmentation": {"variants_per_seed": 5, "num_threads": 40},
  "quality": {"enabled": true, "threshold": 0.6},
  "dedup": {"enabled": true, "threshold": 0.9},
  "export": {"default_format": "jsonl", "formats": ["jsonl", "llama_factory", "alpaca", "sharegpt", "chatml"]},
  "vector": {"enabled": false, "backend": "faiss"},
  "rag": {"enabled": false, "default_format": "llamaindex"},
  "multimodal": {"enabled": false}
}
```

### `GET /api/models`

列出配置文件中定义的模型后端。

```json
{"models": ["ernie", "openai", "ollama", "claude", "gemini"], "default": "ernie"}
```

---

## 3. 数据管理

### `GET /api/data/list`

扫描当前工作目录下以 `train_data` 开头的 JSON 文件。

```json
{
  "files": [
    {"name": "train_data.json", "path": "train_data.json", "size": 123456}
  ]
}
```

### `GET /api/data/load/{filename}`

分页加载数据，支持关键词搜索。

| 参数 | 位置 | 类型 | 默认 | 说明 |
|------|------|------|------|------|
| `filename` | path | string | — | 数据文件路径 |
| `page` | query | int | `1` | 页码，从 1 开始 |
| `page_size` | query | int | `20` | 每页条数 |
| `search` | query | string | `""` | 在 `instruction` 与 `output` 中做不区分大小写的子串匹配 |

```json
{
  "total": 200,
  "page": 1,
  "page_size": 20,
  "items": [{"instruction": "如何申请入住？", "input": "", "output": "通过 App 提交申请。"}]
}
```

`404` 文件不存在；`500` 读取或解析失败。

### `PUT /api/data/update/{filename}`

替换指定索引的单条数据。

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `filename` | path | string | 数据文件路径 |
| `index` | query | int | 待替换的索引 |
| body | body | object | 新的数据条目（任意 JSON 对象） |

```json
{"success": true}
```

`400` 索引越界。

### `DELETE /api/data/delete/{filename}`

删除指定索引的数据。参数同 `PUT`。返回 `{"success": true}`，`400` 索引越界。

### `POST /api/data/upload`

上传 JSON 文件（`multipart/form-data`）。

| 字段 | 类型 | 说明 |
|------|------|------|
| `file` | file | JSON 文件，须为 UTF-8 编码的 JSON 数组 |

文件以 `file.filename` 为名保存到服务端当前目录。

```json
{"success": true, "path": "train_data_uploaded.json", "count": 120}
```

---

## 4. 数据增强

### `POST /api/augment/start`

以**后台任务**方式启动增强，立即返回。进度通过 `GET /api/augment/progress` 轮询。

```json
{
  "input_file": "train_data.json",
  "output_file": "train_data_augmented.json",
  "use_quality": true,
  "use_dedup": true,
  "use_checkpoint": true
}
```

```json
{"success": true, "message": "增强任务已启动"}
```

### `GET /api/augment/progress`

返回当前断点进度。无进行中的任务时返回 `{"status": "no_checkpoint"}`。

```json
{
  "task_id": "augment_9f2c1ab34d5e",
  "total_items": 200,
  "processed_items": 120,
  "failed_items": 2,
  "progress": 0.6,
  "progress_percent": "60.0%",
  "elapsed_time": "2.5分钟",
  "remaining_time": "1.7分钟",
  "start_time": "2026-09-16T17:00:00",
  "last_update": "2026-09-16T17:02:30",
  "avg_quality_score": 0.78
}
```

### `GET /api/augment/checkpoints`

列出已有断点的任务 ID。

```json
{"checkpoints": ["augment_9f2c1ab34d5e"]}
```

---

## 5. 质量评估

以下端点均以 `POST` + JSON body 调用。`QualityRequest` 结构：

```json
{"input_file": "train_data.json", "threshold": 0.6}
```

`DedupRequest` 结构相同，但 `threshold` 默认 `0.9`。

### `POST /api/quality/evaluate`

计算质量评分统计。

```json
{
  "total_samples": 200,
  "passed_samples": 156,
  "filtered_samples": 44,
  "pass_rate": 0.78,
  "avg_score": 0.72,
  "threshold": 0.6
}
```

### `POST /api/quality/dedup`

基于向量相似度去重，返回保留的索引。

```json
{
  "original_count": 200,
  "deduplicated_count": 181,
  "removed_count": 19,
  "duplicate_groups": 12,
  "kept_indices": [0, 1, 2, 4]
}
```

### `POST /api/quality/report`

生成完整质量报告（评分分布、过滤统计、改进建议）。

```json
{
  "total_samples": 200,
  "passed_samples": 156,
  "filtered_samples": 44,
  "pass_rate": 0.78,
  "score_distribution": {"0.0-0.2": 3, "0.2-0.4": 8, "0.4-0.6": 33, "0.6-0.8": 96, "0.8-1.0": 60},
  "filter_statistics": {},
  "metric_summary": {},
  "dedup_summary": {},
  "improvement_suggestions": ["语义相似度均值偏低（0.65），建议调整增强提示词以保持原意"],
  "charts": {}
}
```

`improvement_suggestions` 的判定规则：

| 条件 | 建议 |
|------|------|
| 通过率 < 0.3 | 阈值过严，建议放宽 |
| 通过率 > 0.95 | 阈值过松，建议提高 |
| 语义相似度均值 < 0.7 | 调整提示词以保持原意 |
| 回答相关性均值 < 0.7 | 提升回答与问题的相关性 |
| 多样性均值 < 0.3 | 增加变体多样性 |
| 去重移除率 > 0.3 | 重复度过高，建议检查种子数据 |
| 均不触发 | `"各项指标均在合理区间，数据质量良好"` |

### `POST /api/quality/clean`

数据清洗（去噪、格式标准化、语言检测）。

```json
{
  "original_count": 200,
  "cleaned_count": 198,
  "dropped_count": 2,
  "changed_count": 87,
  "language_distribution": {"zh": 195, "en": 2, "mixed": 1},
  "issues": []
}
```

### `POST /api/quality/annotate`

自动标注（实体识别、意图分类、情感分析）。

```json
{
  "total": 200,
  "entities": {"phone": 12, "email": 8, "money": 45},
  "intents": {"how_to": 88, "what_is": 42, "price": 30},
  "sentiments": {"positive": 90, "neutral": 100, "negative": 10}
}
```

### `POST /api/quality/benchmark`

运行数据质量基准，返回标准化指标。

```json
{
  "pass_rate": 0.78,
  "avg_total_score": 0.72,
  "diversity": 0.54,
  "duplication_rate": 0.09
}
```

---

## 6. 数据导出

### `GET /api/export/formats`

列出支持的导出格式。

```json
{"formats": ["jsonl", "llama_factory", "alpaca", "sharegpt", "chatml"]}
```

### `POST /api/data/export`

导出单个数据集。

```json
{"input_file": "train_data.json", "output_dir": "exports", "formats": ["jsonl", "alpaca"]}
```

`formats` 为 `null` 或省略时导出全部格式。

```json
{
  "success": true,
  "files": {"jsonl": "exports/train_data.jsonl", "alpaca": "exports/train_data_alpaca.json"}
}
```

### `POST /api/export/batch`

一次导出多个数据集 × 多个格式。

```json
{
  "datasets": {"main": "train_data_final.json", "extra": "train_data_extra.json"},
  "output_dir": "exports",
  "formats": ["jsonl", "sharegpt"]
}
```

```json
{
  "success": true,
  "results": {
    "main": {"jsonl": "exports/main.jsonl", "sharegpt": "exports/main_sharegpt.json"},
    "extra": {"jsonl": "exports/extra.jsonl", "sharegpt": "exports/extra_sharegpt.json"}
  }
}
```

格式名非法时在开启线程前即校验并返回 `500`，不会产生部分写入的文件。

### `POST /api/export/preview`

预览格式转换结果（不写磁盘）。

```json
{"input_file": "train_data.json", "format": "sharegpt", "size": 5}
```

```json
{
  "format": "sharegpt",
  "total": 200,
  "preview_count": 5,
  "items": [],
  "warnings": ["数据集为空"]
}
```

`warnings` 会在以下情况出现：数据集为空、必填字段为空、目标格式需要 `history` 但数据未提供。

---

## 7. 版本管理

### `GET /api/versions`

```json
{
  "versions": [
    {"version_id": "20260916_170000_123456", "label": "增强数据", "description": "", "created_at": "2026-09-16T17:00:00", "item_count": 28000}
  ]
}
```

版本 ID 形如 `%Y%m%d_%H%M%S_%f`，精确到微秒并做存在性去重，避免同秒创建时互相覆盖。

### `POST /api/versions/create?filename=<path>`

从数据文件创建快照。

```json
{"label": "v1 基线", "description": "增强前的原始数据"}
```

```json
{"success": true, "version_id": "20260916_170000_123456"}
```

### `POST /api/versions/diff`

对比两个版本。

```json
{"version1": "20260916_170000_123456", "version2": "20260916_180000_654321"}
```

```json
{"version1": "...", "version2": "...", "added_count": 120, "removed_count": 3, "modified_count": 8}
```

### `GET /api/versions/history`

版本操作历史（create / delete / rollback），按时间倒序。

| 参数 | 类型 | 默认 | 说明 |
|------|------|------|------|
| `limit` | query | `20` | 返回条数上限 |

```json
{"history": [{"action": "create", "version_id": "20260916_170000_123456", "detail": "增强数据"}]}
```

### `GET /api/versions/{version_id}`

```json
{
  "version_id": "...", "label": "...", "description": "...",
  "created_at": "...", "item_count": 28000, "metadata": {}
}
```

### `GET /api/versions/{version_id}/data`

```json
{"items": [{"instruction": "...", "output": "..."}]}
```

### `POST /api/versions/{version_id}/rollback`

```json
{"success": true}
```

### `DELETE /api/versions/{version_id}`

```json
{"success": true}
```

> **路由顺序注意**：`/api/versions/history` 必须定义在 `/api/versions/{version_id}` 之前，
> 否则会被路径参数吞掉。

---

## 8. 多模态处理

### `GET /api/multimodal/formats`

```json
{
  "image_extensions": [".jpg", ".jpeg", ".png", ".bmp", ".webp"],
  "audio_extensions": [".wav", ".mp3", ".flac", ".ogg", ".m4a"]
}
```

### `POST /api/multimodal/process`

融合单条多模态记录。`image` / `audio` 为可选的文件路径。

```json
{"text": "客户上传的报修图片", "image": "samples/leak.png", "audio": null}
```

```json
{
  "text": "客户上传的报修图片",
  "image": {"path": "samples/leak.png", "format": "PNG", "width": 800, "height": 600, "size": 20480},
  "audio": null,
  "modalities": ["text", "image"]
}
```

### `POST /api/multimodal/scan`

扫描目录，把同名 stem 的图片与音频融合为一条记录。

```json
{"directory": "samples"}
```

```json
{
  "total_records": 12,
  "modality_distribution": {"text+image": 7, "text+audio": 3, "text+image+audio": 2},
  "records": []
}
```

`400` 目录不存在或不支持的文件扩展名。

---

## 9. 分析与可视化

### `GET /api/analyze/{filename}`

返回覆盖度分析、统计信息与去重报告。

```json
{
  "coverage_analysis": {"total_items": 200, "question_type_distribution": {"how": 0.4}},
  "statistics": {"avg_length": 18.5, "top_words": {}},
  "dedup_report": {"removal_rate": 0.09}
}
```

### `GET /api/visualize/{filename}`

生成图表并返回文件路径。依赖缺失时对应项返回空字符串，会被过滤掉。

```json
{"charts": {"wordcloud": "visualizations/wordcloud.png", "length_distribution": "visualizations/length_distribution.png"}}
```

---

## 10. 调用示例

```bash
# 健康检查
curl http://localhost:8000/api/health

# 启动增强
curl -X POST http://localhost:8000/api/augment/start \
  -H "Content-Type: application/json" \
  -d '{"input_file":"train_data.json","output_file":"out.json"}'

# 轮询进度
curl http://localhost:8000/api/augment/progress

# 质量评估
curl -X POST http://localhost:8000/api/quality/evaluate \
  -H "Content-Type: application/json" \
  -d '{"input_file":"train_data.json","threshold":0.6}'

# 上传文件
curl -X POST http://localhost:8000/api/data/upload \
  -F "file=@train_data_new.json"
```

Python：

```python
import requests

base = "http://localhost:8000"

print(requests.get(f"{base}/api/health").json())

resp = requests.post(
    f"{base}/api/quality/report",
    json={"input_file": "train_data.json", "threshold": 0.6},
    timeout=300,
)
report = resp.json()
print(report["pass_rate"], report["improvement_suggestions"])
```
