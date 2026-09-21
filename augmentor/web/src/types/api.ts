/**
 * 后端 API 的响应类型
 *
 * ## 这些类型从哪来
 *
 * 全部来自对后端**真实响应**的探测，而不是读 `api/routes/*.py` 里的
 * `return {...}` 手抄。区别很重要：
 *
 * - 有些字段来自 `to_dict()`（在 `augmentor/` 下，不在路由里），只看路由会漏；
 * - 同一个端点可能返回不同形态：`/api/augment/progress` 在没有进行中的任务时
 *   只返回 `{status: 'no_checkpoint'}`，有任务时才返回完整进度；
 * - 手抄容易把「有时缺失」的字段写成必填，而那正是 TS 里最该标 `?` 的地方。
 *
 * 探测方式：用 `fastapi.testclient.TestClient` 打全部 36 个端点，把 JSON 渲染成
 * TS 风格的类型描述。`get_progress` 的完整形态、`ImageInfo` / `AudioInfo` 这类
 * 只在真有媒体文件时才出现的结构，另从源码读齐。
 *
 * ## 已知局限（不要误读成保证）
 *
 * 这是**编译期声明**，不是运行时校验。`axios` 的 `response.data` 是 `any`，
 * 赋给这些接口不会报错，因此后端若改了响应形态，`tsc` 不会发现。
 *
 * 本层解决的是「页面层不再到处写 `any`、字段名写错能在编译期发现」，
 * **不是**「前后端契约有机器保证」。要后者得引入运行时 schema 校验
 * （如 zod），不在本次改动范围内。
 */

// ============ 通用 ============

/**
 * 数据集单条记录
 *
 * `/api/data/load` 与 `/api/versions/{id}/data` 都是**原样透传**文件里的 JSON，
 * 后端不校验字段，所以这三个字段是「本项目数据集的约定」，不是后端保证。
 */
export interface DataItem {
  instruction: string
  input: string
  output: string
}

/** 写操作（更新 / 删除 / 回滚）的统一成功响应 */
export interface MutationResponse {
  success: boolean
}

/** 字符串 → 数值的分布统计（问题类型、语言、意图、情感…） */
export type CountDistribution = Record<string, number>

// ============ 数据管理 ============

/** `GET /api/data/list` 里的单个文件 */
export interface DataFileInfo {
  name: string
  path: string
  size: number
}

export interface DataFileListResponse {
  files: DataFileInfo[]
}

/** `GET /api/data/load/{filename}` */
export interface DataPageResponse {
  total: number
  page: number
  page_size: number
  items: DataItem[]
}

/** `POST /api/data/upload` */
export interface UploadResponse {
  success: boolean
  path: string
  count: number
}

/** `POST /api/data/export`（按目录批量导出多种格式） */
export interface DatasetExportResponse {
  success: boolean
  /** 格式名 → 落盘路径 */
  files: Record<string, string>
}

// ============ 数据增强 ============

/** `POST /api/augment/start` */
export interface StartAugmentResponse {
  success: boolean
  message: string
}

/** 没有进行中的任务时的进度响应 */
export interface NoCheckpointProgress {
  status: string
}

/**
 * 有进行中的任务时的进度详情
 *
 * 注意：后端在**这个分支里不返回 `status`**（见 `checkpoint.py: get_progress`），
 * 所以收窄只能靠 `'task_id' in result`，不能靠读 `status`。
 */
export interface AugmentProgressDetail {
  task_id: string
  total_items: number
  processed_items: number
  failed_items: number
  /** 0 ~ 1 */
  progress: number
  /** 已格式化，如 `"42.0%"` */
  progress_percent: string
  /** 已格式化，如 `"0:01:23"` */
  elapsed_time: string
  remaining_time: string
  /** ISO 8601 */
  start_time: string
  last_update: string
  avg_quality_score: number
}

/**
 * `GET /api/augment/progress`
 *
 * 两种形态互斥：无任务时只有 `status`，有任务时只有下面那一组字段。
 */
export type AugmentProgress = NoCheckpointProgress | AugmentProgressDetail

/** `GET /api/augment/checkpoints`（`list_checkpoints()` 返回的是任务 ID 列表） */
export interface CheckpointListResponse {
  checkpoints: string[]
}

// ============ 数据分析 ============

/** 长度分布。`short` / `medium` / `long` 是**比例**（0~1），不是条数。 */
export interface LengthDistribution {
  short: number
  medium: number
  long: number
  avg_length: number
}

/** 复杂度分布，同为比例 */
export interface ComplexityDistribution {
  simple: number
  medium: number
  complex: number
  avg_complexity: number
}

export interface TopicDistribution {
  unique_words: number
  /** 词 → 出现次数（这里是**计数**，不是比例） */
  top_words: CountDistribution
  coverage_score: number
}

export interface CoverageAnalysis {
  total_items: number
  /** 问题类型 → **比例**（0~1） */
  question_type_distribution: CountDistribution
  length_distribution: LengthDistribution
  topic_distribution: TopicDistribution
  complexity_distribution: ComplexityDistribution
}

export interface DatasetStatistics {
  total_items: number
  avg_length: number
  min_length: number
  max_length: number
  unique_words: number
  /** 词 → 出现次数 */
  top_words: CountDistribution
}

/** 去重统计。`analyze` 的 `dedup_report` 与质量报告的 `dedup_summary` 是同一结构。 */
export interface DedupSummary {
  original_count: number
  deduplicated_count: number
  removed_count: number
  removal_rate: number
  duplicate_groups_count: number
  avg_group_size: number
  threshold: number
}

/** `GET /api/analyze/{filename}` */
export interface AnalyzeResponse {
  coverage_analysis: CoverageAnalysis
  statistics: DatasetStatistics
  dedup_report: DedupSummary
}

/** `GET /api/visualize/{filename}`（图表名 → 图片落盘路径；matplotlib 缺失时为空对象） */
export interface VisualizeResponse {
  charts: Record<string, string>
}

// ============ 版本管理 ============

export interface VersionInfo {
  version_id: string
  label: string
  description: string
  created_at: string
  item_count: number
}

/** `GET /api/versions/{id}` 比列表多一个 `metadata` */
export interface VersionDetail extends VersionInfo {
  metadata: Record<string, unknown>
}

export interface VersionListResponse {
  versions: VersionInfo[]
}

/** `POST /api/versions/create` */
export interface CreateVersionResponse {
  success: boolean
  version_id: string
}

export interface VersionDataResponse {
  items: DataItem[]
}

/** `POST /api/versions/diff` */
export interface VersionDiffResponse {
  version1: string
  version2: string
  added_count: number
  removed_count: number
  modified_count: number
}

export interface VersionHistoryEntry {
  action: string
  version_id: string
  /** ISO 8601 */
  timestamp: string
  /** 附加信息随 `action` 而变：创建时是 `{label, item_count}`，回滚时是 `{}` */
  detail: Record<string, unknown>
}

export interface VersionHistoryResponse {
  history: VersionHistoryEntry[]
}

// ============ 质量评估 ============

/** `POST /api/quality/evaluate` */
export interface QualityEvaluationResponse {
  total_samples: number
  passed_samples: number
  filtered_samples: number
  pass_rate: number
  avg_score: number
  threshold: number
  semantic_evaluated: boolean
  effective_weights: number[]
}

/** `POST /api/quality/dedup` */
export interface DedupResponse {
  original_count: number
  deduplicated_count: number
  removed_count: number
  duplicate_groups: number
  kept_indices: number[]
}

/** 单个指标的均值 / 最小 / 最大 */
export interface MetricStats {
  mean: number
  min: number
  max: number
}

/**
 * 质量报告的图表数据
 *
 * `score_histogram` 是**桶名 → 条数**的字典（`report.py: _build_charts`），
 * 不是 `{labels, values}`；`metric_radar` 才是 `{labels, values}`。
 * 这两者形态不同，混用会直接抛 `TypeError`。
 */
export interface QualityReportCharts {
  /** 桶名（如 `"0.6-0.8"`）→ 落入该桶的样本数 */
  score_histogram: CountDistribution
  metric_radar: {
    labels: string[]
    values: number[]
  }
}

/** `POST /api/quality/report` */
export interface QualityReportResponse {
  total_samples: number
  passed_samples: number
  filtered_samples: number
  pass_rate: number
  /** 桶名 → 条数 */
  score_distribution: CountDistribution
  /** 目前后端恒为空对象 */
  filter_statistics: Record<string, unknown>
  /** 指标名 → 统计量 */
  metric_summary: Record<string, MetricStats>
  dedup_summary: DedupSummary
  improvement_suggestions: string[]
  charts: QualityReportCharts
}

/** `POST /api/quality/clean` */
export interface CleanResponse {
  original_count: number
  cleaned_count: number
  dropped_count: number
  changed_count: number
  /** 语言代码 → 条数 */
  language_distribution: CountDistribution
  issues: {
    noise_removed: number
    format_normalized: number
    dropped: number
  }
}

/** `POST /api/quality/annotate` */
export interface AnnotateResponse {
  total_items: number
  entity_count: number
  avg_entities_per_item: number
  intent_distribution: CountDistribution
  sentiment_distribution: CountDistribution
  /** 标注取的是哪个字段 */
  text_key: string
}

/** `POST /api/quality/benchmark` */
export interface BenchmarkResponse {
  sample_count: number
  /** 指标名 → 数值（保留的指标子集） */
  metrics: CountDistribution
  /** 全量指标 */
  all_metrics: CountDistribution
  threshold: number
  /** ISO 8601 */
  timestamp: string
}

// ============ 导出 ============

export interface ExportFormatsResponse {
  formats: string[]
}

/**
 * `POST /api/export/preview`
 *
 * `converted_data` 的字段随目标格式而变（alpaca 与 sharegpt 的键不同），
 * 所以只能是开放对象——这也正是页面用 `Object.keys(converted_data[0])`
 * 动态生成表格列的原因。
 */
export interface ExportPreviewResponse {
  format: string
  original_data: DataItem[]
  converted_data: Record<string, unknown>[]
  format_info: {
    format: string
    total_items: number
    preview_items: number
    fields: string[]
    required_fields: string[]
  }
  warnings: string[]
}

/** `POST /api/export/batch` */
export interface BatchExportResponse {
  success: boolean
  /** 数据集名 → (格式名 → 落盘路径) */
  results: Record<string, Record<string, string>>
}

// ============ 多模态 ============

/** `ImageInfo.to_dict()`（`augmentor/data/image.py`） */
export interface ImageInfo {
  path: string
  format: string
  width: number
  height: number
  mode: string
  size_bytes: number
  valid: boolean
  error: string
  metadata: Record<string, unknown>
}

/** `AudioInfo.to_dict()`（`augmentor/data/audio.py`） */
export interface AudioInfo {
  path: string
  format: string
  /** 秒 */
  duration: number
  sample_rate: number
  channels: number
  size_bytes: number
  valid: boolean
  error: string
  metadata: Record<string, unknown>
}

/** `MultimodalRecord.to_dict()` */
export interface MultimodalRecord {
  text: string
  image: ImageInfo | null
  audio: AudioInfo | null
  modalities: string[]
  valid: boolean
  errors: string[]
}

export interface MultimodalFormatsResponse {
  image_extensions: string[]
  audio_extensions: string[]
}

/** `POST /api/multimodal/scan`（`generate_report()` 的字段 + 逐条记录） */
export interface MultimodalScanResponse {
  total_records: number
  valid_records: number
  invalid_records: number
  /** 模态名 → 条数 */
  modality_counts: CountDistribution
  error_count: number
  errors: string[]
  records: MultimodalRecord[]
}

// ============ 配置与健康检查 ============

export interface AppConfig {
  default_model: string
  augmentation: {
    variants_per_seed: number
    num_threads: number
  }
  quality: {
    enabled: boolean
    threshold: number
  }
  dedup: {
    enabled: boolean
    threshold: number
  }
  export: {
    default_format: string
    formats: string[]
  }
  vector: {
    enabled: boolean
    backend: string
  }
  rag: {
    enabled: boolean
    default_format: string
  }
  multimodal: {
    enabled: boolean
  }
}

/** `POST /api/config` */
export interface UpdateConfigResponse {
  success: boolean
  message: string
}

export interface ModelsResponse {
  models: string[]
  default: string
}

/** `GET /api/health` */
export interface HealthResponse {
  status: string
  version: string
}
