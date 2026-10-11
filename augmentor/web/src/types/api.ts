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
 * 两个已知的「声明比实际宽松/严格」之处：
 *
 * 1. 后端在**这个分支里不返回 `status`**（见 `checkpoint.py: get_progress`），
 *    所以收窄只能靠 `'task_id' in result`，不能靠读 `status`。
 * 2. 路由上开了 `response_model_exclude_none=True`，所以某个任务字段**恰为
 *    `null` 时该键会被整个省略**——即「键永远存在」并不成立。页面里对
 *    `avg_quality_score` 用 `> 0` 判断、对 `progress` 用 `|| 0` 兜底，
 *    正好也是这个原因（`undefined > 0` 为 false，不会渲染 NaN）。
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

// ============ 服务状态（`GET /api/status`）============

/**
 * 可选依赖诊断（`DiagnosticsReport.to_dict()`）
 *
 * `all_required_present` 与 `available_count` 在后端是**计算属性**而非字段，
 * 但它们出现在响应里，因此这里照实声明。
 */
export interface DependencyDiagnostics {
  installed: Record<string, boolean>
  missing: string[]
  degraded_features: string[]
  all_required_present: boolean
  available_count: number
}

/**
 * `GET /api/status`
 *
 * 与 `/api/health` 的区别：health 只报存活（给容器探针用，刻意不含可能失败的
 * 字段），status 会真的去构造管道并检查依赖，因此可能比 health 慢或失败。
 */
export interface StatusResponse {
  status: string
  version: string
  model_default: string
  model_available: boolean
  dependencies: DependencyDiagnostics
}

// ============ 隐私脱敏 ============

/** `GET /api/privacy/patterns`：可用的 PII 模式名 */
export interface PiiPatternsResponse {
  default: string[]
  extra: string[]
}

/**
 * 脱敏报告（`SanitizeReport.to_dict()`）
 *
 * `matches` 是「模式名 → 命中次数」，`total_matches` 是它的总和（后端计算属性）。
 */
export interface SanitizeReport {
  total_items: number
  touched_items: number
  matches: CountDistribution
  total_matches: number
}

/** `POST /api/privacy/sanitize`：脱敏后的数据 + 报告 */
export interface SanitizeResponse {
  items: DataItem[]
  report: SanitizeReport
}

// ============ 泄漏检测 ============

/**
 * `POST /api/leakage/check`
 *
 * `total_leaks` / `leak_rate` / `is_clean` 是 `LeakageReport` 上的计算属性，
 * 必须声明——它们确实在响应里。
 */
export interface LeakageResponse {
  train_size: number
  test_size: number
  exact_leaks: number
  fuzzy_leaks: number
  total_leaks: number
  leak_rate: number
  is_clean: boolean
  leaked_examples: Record<string, unknown>[]
}

// ============ 数据集就绪审计 ============

/**
 * `POST /api/audit`
 *
 * 后端直接拿 `AuditReport`（dataclass）当 response_model，因此这里的字段
 * 与它的 `to_dict()` 完全一致。
 */
export interface AuditResponse {
  total_items: number
  pii_matches: number
  duplicate_count: number
  duplicate_rate: number
  empty_field_rate: number
  leak_count: number
  leak_rate: number
  findings: string[]
  ready: boolean
}

// ============ 离群点与画像 ============

/** 单条离群样本（`{index, value, item}`） */
export interface OutlierEntry {
  index: number
  value: number
  item: DataItem
}

/** `POST /api/quality/outliers` */
export interface OutliersResponse {
  total_items: number
  outlier_count: number
  outlier_rate: number
  method: string
  threshold: number
  field: string
  outliers: OutlierEntry[]
}

/** 长度分布统计（单位：字符数） */
export interface LengthStats {
  min: number
  max: number
  avg: number
  median: number
}

/** 单个高频关键词（`DataProfiler._top_keywords()`） */
export interface KeywordCount {
  keyword: string
  count: number
}

/**
 * `POST /api/quality/profiling`
 *
 * 两个容易写错的点（`augmentor/profiling.py`）：
 * - `field_completeness` 是**字段名 → 非空率（0~1）**的扁平字典，
 *   不是「字段名 → 字段画像对象」。`_field_completeness()` 返回 `Dict[str, float]`。
 * - `length_stats` 在空数据集上是 `{}`（`profile()` 的早退分支），
 *   所以这里用 `Partial` —— 四个字段要么全在，要么全不在。
 */
export interface ProfilingResponse {
  total_items: number
  /** 字段名 → 非空率（0~1） */
  field_completeness: Record<string, number>
  length_stats: Partial<LengthStats>
  duplicate_rate: number
  language_distribution: CountDistribution
  /** 按频次降序 */
  top_keywords: KeywordCount[]
}

/**
 * `POST /api/dataset/language-profile`
 *
 * `supported_langs` / `default_target_lang` 是**本次生效值**（取自
 * `config.multilingual.*`），不是常量 —— 换配置就换值，所以渲染时要显出来而不是
 * 在前端写死一张语言表。`outside_supported` 里出现 `mixed` 不是错误：它是检测标签
 * 而不是语言码（`api/routes/language.py` 的模型注释）。
 */
export interface LanguageProfileResponse {
  total_items: number
  /** 检测标签 → 条数 */
  distribution: Record<string, number>
  /** 检测标签 → 占比（0~1） */
  ratios: Record<string, number>
  outside_supported: string[]
  supported_langs: string[]
  default_target_lang: string
}

/**
 * `POST /api/dataset/evaluate`
 *
 * `details` 在 `include_details=false` 时是**空数组**，而 `sample_count` 仍是真实条数
 * —— 别看 details 长度判断有没有跑成。契约里那个默认值是 true，是本层显式关掉的
 * （页面不渲逐条明细）。`metrics` 的键集随配置里的 `evaluation.metrics` 变，
 * 所以按字典渲染而不是列死。
 */
export interface EvaluateResponse {
  metrics: Record<string, number>
  sample_count: number
  details: Record<string, unknown>[]
}

/**
 * `GET /api/dataset/vector-status`
 *
 * `enabled` 是「增强结束时自动建库」那只闸的**生效值**，`exists` 是磁盘上究竟有没有
 * 一份可读回的索引。两件事分开看：`enabled=false` 且 `exists=true` 是上一轮建过、
 * 这一轮关了闸；`enabled=true` 且 `exists=false` 是配了还没跑过。
 */
export interface VectorStatusResponse {
  enabled: boolean
  exists: boolean
  count: number
  backend: string
  dimension: number
  collection: string
  storage_dir: string
}

/**
 * `POST /api/dataset/search` 的高亮条目
 *
 * 值固定是三段 `[命中前, 命中文本, 命中后]`（SDK `_generate_highlights` 用切片拼出），
 * 这里写成 `string[]` 而不是元组：页面只按位取用，元组类型会让测试里的夹具
 * 为了迁就类型而少写段数，反而测不出「少一段」这种真实畸形。
 */
export interface SearchHighlightItem {
  /** 命中条目在**整份文件**里的下标，可直接指回 `input_file` */
  index: number
  highlights: Record<string, string[]>
}

/**
 * `POST /api/dataset/search`
 *
 * `highlights` 只在**子串能落进原文**的那几路非空：实测 `method=regex` 时
 * `total_matches=1` 而 `highlights=[]`。所以「命中条数」与「高亮段数」必须分开渲染
 * —— 用高亮为空来表示「没搜到」会把 regex 那一档报成零命中。
 * 后四键只在本次请求真的消费了它时非空（`fuzzy` 才带阈值、`ngram` 才带 n、
 * 传了 `filters` 才带过滤读数），因此全部可空。
 */
export interface SearchResponse {
  query: string
  method: string
  total_matches: number
  query_time_ms: number
  items: Record<string, unknown>[]
  highlights: SearchHighlightItem[]
  fuzzy_threshold: number | null
  ngram_n: number | null
  applied_filters: Record<string, unknown>[] | null
  matches_before_filters: number | null
}

/** 一条验证问题（`ValidationIssue.to_dict()` 的键） */
export interface DatasetValidationIssue {
  field: string
  message: string
  /** 与 SDK `ValidationSeverity` 的三个枚举值一一对应 */
  severity: 'error' | 'warning' | 'info'
  index: number | null
}

/**
 * `POST /api/dataset/validate`
 *
 * `issues` 是 SDK 收窄过的**样本**（`ValidationResult.to_dict()` 取前 100 条），
 * 而 `error_count` / `warning_count` 按全量算 ⇒ 「警告 137 条、表里只有 100 行」
 * 是正常读数，页面不能拿样本行数当总数。
 */
export interface DatasetValidationResponse {
  is_valid: boolean
  total_items: number
  valid_items: number
  error_count: number
  warning_count: number
  issues: DatasetValidationIssue[]
}

/**
 * `POST /api/dataset/auto-config`
 *
 * `reasoning` 是**逐条理由的清单**而不是单串文本（实测：`["重复率 0.00 较低，去重阈值下调至 0.85"]`）。
 * `source_stats` 是观测值原样带回，实测 `quality_pass_rate` 可以为 `null`
 * （那份夹具没有可判质量的字段）⇒ 渲染必须走空值分支，不能假设一定有数。
 */
export interface AutoConfigResponse {
  quality_threshold: number
  dedup_threshold: number
  recommended_sample_size: number
  reasoning: string[]
  source_stats: Record<string, unknown>
}

// ============ 影响评估 / 语义检索（A279 第三批接线，L227） ============

/**
 * `impact` 里 `before` / `after` 两侧的同一份快照
 *
 * 六个键逐字来自 `augmentor/impact.py:24` 的 `AugmentationMetrics.to_dict()`。
 * `extra` 是 SDK 留的开放扩展位（现为空对象），所以不假装知道它有什么。
 */
export interface ImpactSnapshot {
  total_items: number
  unique_instructions: number
  avg_length: number
  length_std: number
  duplicate_rate: number
  extra: Record<string, unknown>
}

/**
 * `POST /api/dataset/impact`
 *
 * `gains` 固定四项（实测：`scale_gain` / `diversity_gain` / `dedup_gain` /
 * `length_spread_gain`），值可正可负，0.0 的含义随项不同——「规模没涨」与「重复率没降」
 * 不是一回事，所以页面按项逐个显示，不合并成一个总分。
 * `beneficial` 是路由替调用方做的判定（口径 `min_scale_gain`），不是四项之一的名字。
 */
export interface ImpactResponse {
  before: ImpactSnapshot
  after: ImpactSnapshot
  gains: Record<string, number>
  beneficial: boolean
}

/** 语义检索的单条命中：`item` 是原条目整份回带，`text` 是参与编码的那个字段的值 */
export interface SemanticSearchHit {
  index: number
  score: number
  text: string
  item: Record<string, unknown>
}

/**
 * `POST /api/dataset/semantic-search`
 *
 * `backend` / `dimension` / `collection` 是**本次生效值**（取自 `config.vector.*`），
 * 契约注释写得明说：`dimension` 一改排名就变，不回显生效值就等于让调用方猜 ⇒ 页面要显示。
 * `total` 实测就是 `results` 的长度（`top_k` 上限收窄后一致），不是「库里总条数」。
 * 空查询实测**照样回条目且 score 全 0**（退化读数），所以页面必须在门口拦住空串。
 */
export interface SemanticSearchResponse {
  query: string
  total: number
  backend: string
  dimension: number
  collection: string
  results: SemanticSearchHit[]
}

// ============ 字段级统计 / 特征普查 / 双集对比（A279 第四批接线，L228） ============

/**
 * `field_statistics` 里单个字段的一行
 *
 * 十二个键逐字取自真实回执（`DatasetStatistics` 的字段级统计）。
 * `top_values` 是 `[值, 次数]` 的**二元组数组**，不是 `{值: 次数}` 的字典——
 * 这一点与 `content_statistics.vocabulary.top_words` 同形，与
 * `GET /api/analyze/{file}` 里那个 `top_words` 对象**不同形**，两者不能共用一个渲染器。
 */
export interface FieldStatisticsEntry {
  field_name: string
  total_count: number
  filled_count: number
  empty_count: number
  fill_rate: number
  avg_length: number
  min_length: number
  max_length: number
  median_length: number
  std_deviation: number
  unique_count: number
  top_values: [string, number][]
}

/**
 * `POST /api/dataset/stats`
 *
 * `field_statistics` / `quality_metrics` 在契约面上是 `additionalProperties` 开放的字典，
 * 键名**来自数据集本身**（实测：`a.json` 回 `instruction` 与 `output`），所以页面按回执的键渲染，
 * 不在 TSX 里写字段名清单。`summary` 是后端已经排好多行的文本，重排它就是二次说谎的机会。
 * 实测坑一：`fields` 里给一个数据集没有的键，端点照样回 **200** 并给这一格填全零（幽灵字段），
 * 所以「可选字段清单」只能来自回执，不能来自猜测。
 * 实测坑二：空数据集（`[]`）回执是 `total_items: 0` 且 **三个字典全空**——
 * `field_statistics`、`quality_metrics` 是 `{}`，`content_statistics` 也是 `{}` 而不是
 * 「有结构但值为零」，两个子键一个都不出现。所以这里把两个子键标成可选，
 * 页面按「键在不在」分档渲染，而不是替端点编一个 0。
 */
export interface StatisticsResponse {
  dataset_name: string
  total_items: number
  summary: string
  field_statistics: Record<string, FieldStatisticsEntry>
  content_statistics: {
    text_length?: { avg: number; min: number; max: number; total: number }
    vocabulary?: {
      total_words: number
      unique_words: number
      top_words: [string, number][]
    }
  }
  quality_metrics: Record<string, number>
}

/** 单个字段的特征行；`description` 实测只有「常规特征」与「稀疏特征」两种措辞 */
export interface FieldFeatureEntry {
  name: string
  feature_type: string
  coverage: number
  unique_ratio: number
  description: string
}

/**
 * `POST /api/dataset/features`
 *
 * 稀疏判据住在 `augmentor/feature_detect.py:59` 的 `min_coverage`（出厂默认 0.1），
 * **这只端点不接受调用方改它**（请求模型只有 `input_file`，实测多送 `text_field` 会被静默忽略），
 * 所以页面只读不装旋钮。`feature_type` 是 `_infer_type` 的五档字面量之一
 * （`numeric` / `text` / `boolean` / `mixed` / `unknown`，实测全空字段落进 `unknown` 且
 * 同时被标成稀疏特征），后端没有对应常数可引，所以渲染时未知值原样显示而不编标签。
 * `intent_distribution` 是**条数**（键只包含真出现过的意图），与「覆盖度分析」那张图的
 * 同一分类器占比是一个数据的两副口径；实测空集与「一句问话都没有」的集都会回 `{}`。
 */
export interface FeaturesResponse {
  total_items: number
  field_features: FieldFeatureEntry[]
  sparse_fields: string[]
  intent_distribution: Record<string, number>
}

/** 质量面的单集分数与两侧差值共用这一格形状 */
export interface QualityScoreFace {
  avg_score: number
  pass_rate: number
  min_score: number
  max_score: number
}

/**
 * `compare` 的质量侧（`DatasetComparator.compare().to_dict()`）
 *
 * `winner` 实测三态：`"a"` / `"b"` / `"tie"`（同一份文件自己比自己就是 `tie`），
 * 页面按三态渲染而不是「有 winner 就行」。
 * `similarity_to_a` / `similarity_to_b` 是**恒为 0.0 的死面**：`augmentor/comparison.py` 里
 * 只在 dataclass 默认值与 `to_dict()` 出现，全仓没有任何赋值点，实测两份不同数据也照样 0.0
 * ⇒ 页面不渲染它们（账本 **A287**）。
 */
export interface QualityComparisonFace {
  dataset_a_name: string
  dataset_b_name: string
  dataset_a_count: number
  dataset_b_count: number
  quality_a: QualityScoreFace
  quality_b: QualityScoreFace
  quality_diff: QualityScoreFace
  dedup_a: { duplication_rate: number; unique_count: number }
  dedup_b: { duplication_rate: number; unique_count: number }
  length_a: { avg: number; min: number; max: number; std: number }
  length_b: { avg: number; min: number; max: number; std: number }
  vocabulary_a: { unique_chars: number; total_chars: number }
  vocabulary_b: { unique_chars: number; total_chars: number }
  similarity_to_a: number
  similarity_to_b: number
  winner: string
  summary: string
}

/**
 * 重叠面的单字段行
 *
 * 明细条目的形状是**固定的三元组** `{instruction, value_a, value_b}`
 * （`augmentor/compare_enhanced.py:289`）：`_compare_fields` 压根不接受 `key_fields`，
 * 逐条配对口径永远是 `instruction`。所以「换匹配键」只动 `metrics` 那一组数，
 * 这张表的行不会跟着变——这一档不写清就会被读成「明细也跟着我选的键走」。
 */
export interface FieldComparisonEntry {
  field_name: string
  in_a_only: number
  in_b_only: number
  in_both: number
  type_mismatches: number
  value_differences: Record<string, unknown>[]
}

/**
 * `compare` 的重叠面（`compare_datasets_enhanced`）
 *
 * `key_fields` 的默认权威**不在请求模型里**（`CompareRequest.key_fields` 默认 `None`），
 * 而在 `augmentor/compare_enhanced.py:126` 的 `key_fields or ["instruction"]`。
 * 实测「同问不同答」的一对数据集（instruction 全同、output 全不同）：默认 ⇒ `common_items 4`、
 * `similarity_score 1.0`，换成 `output` ⇒ `0` 与 `0.0`——这一档真动重叠面的数，但它**只**动
 * `metrics`：同一响应里 `field_comparisons` 连同 `value_differences` 逐字节不变（明细永远按
 * `instruction` 配对，见上一条注）。实测坑：给一个两侧都没有的键 ⇒ `common_items: 1`、
 * `similarity_score: 1.0`（取键时全部塌成同一个空标识）——幽灵匹配键比幽灵字段名更危险，
 * 它把「毫无重叠」谎报成「完全重叠」。所以页面的候选只能来自回执的 `field_comparisons` 键。
 * `size_ratio` 声明是 `float`，但 `compare_enhanced.py:190` 在 B 侧为空时存的是
 * `float('inf')`，Pydantic 序列化到线上是 **`null`**（实测 `size_b: 0` 那一条）——
 * 严格 JSON 面是安全的，但 TS 不能把它当纯 `number`，故这里标 `number | null`（账本 **A288**）。
 */
export interface OverlapComparisonFace {
  dataset_a_name: string
  dataset_b_name: string
  metrics: {
    size_a: number
    size_b: number
    size_diff: number
    size_ratio: number | null
    common_items: number
    unique_a: number
    unique_b: number
    similarity_score: number
    field_overlap: number
  }
  field_comparisons: Record<string, FieldComparisonEntry>
  summary: string
  recommendations: string[]
}

/** `POST /api/dataset/compare`：两个面各自带 `summary`，不是一句话的两半，是两套口径 */
export interface CompareResponse {
  quality_comparison: QualityComparisonFace
  overlap_comparison: OverlapComparisonFace
}

// ============ 写盘型数据集变换（A279 第五批接线，L229） ============

/**
 * `POST /api/dataset/convert`（与 `converter.convert_file()` 的返回键一一对应）
 *
 * 两个「名字」键回的都是**实际生效值**：请求省略 `source_format` 时这里给的是推断结果
 * （实测 `a.json` 回 `json`），不是空串——所以页面显示回执这两格，而不是回显用户的选择。
 * `input_file` / `output_file` 回的是解析后的**绝对路径**（与导出类端点同一口径，
 * `docs/API.md` 的 dataset 分组说明里已把「落盘后的绝对路径」当成对外形状写明）。
 * 读写两侧的合法拼法**不是同一份清单**：读边十三项含 `xls`，写边十二项不含
 * （`augmentor/converter.py:56` 明确拒写老 BIFF 容器），所以两个下拉各用各的清单。
 */
export interface ConvertResponse {
  input_file: string
  output_file: string
  source_format: string
  target_format: string
  input_count: number
  output_count: number
}

/**
 * `POST /api/dataset/merge`
 *
 * `removed_duplicates` 只在 `deduplicate` 为真时才可能非零（实测同一份文件合并两次：
 * 开去重 `total_output 5 / removed 5`，关去重 `10 / 0`）。SDK 还另外产出一个
 * `truncated_by_max_items`（`augmentor/dataset_ops.py:220`），但响应模型没有声明它，
 * FastAPI 按模型字段过滤 ⇒ 线上永远看不到这一格（账本 **A290**）。
 */
export interface MergeResponse {
  input_files: number
  output_file: string
  total_input: number
  total_output: number
  removed_duplicates: number
}

/** `POST /api/dataset/split` 的单支产物：落盘绝对路径 + 条数（**0 条也占一格**） */
export interface SplitPart {
  file: string
  count: number
}

/**
 * `POST /api/dataset/split`
 *
 * `splits` 恒有 `train` / `val` / `test` 三键（实测 `0.8/0.1/0.1` 打 6 条时 `test` 是
 * `count: 0` 而**不是缺键**），所以页面按回执的键渲染表格，但不要把「少一段」当成可能形状。
 * `output_dir` 与每支的 `file` 都是解析后的绝对路径。
 */
export interface SplitResponse {
  input_file: string
  output_dir: string
  splits: Record<string, SplitPart>
}

// ============ 采样 / 聚合 / RAG 转换（A279 第六批接线，L230） ============

/**
 * `POST /api/dataset/sample`（与 `DatasetOperations.sample_file()` 的返回键一一对应）
 *
 * `output_count` 可以合法地是 **0**：`size=0`，或 `ratio` 小到 `int()` 归零，实测都是
 * 200 并在盘上留一份内容为 `[]` 的空文件（不是「不落盘」也不是报错）。`input_file` /
 * `output_file` 都是解析后的绝对路径。这一面**不回**采样方法、seed 或分层键——回执里
 * 只有「从哪读、写到哪、几条进几条出」，所以页面显示的是自己发出去的那几格。
 */
export interface SampleResponse {
  input_file: string
  output_file: string
  input_count: number
  output_count: number
}

/**
 * `POST /api/dataset/aggregate` 回执里 `conflicts` 的一项
 *
 * 只有 `strategy="consistent"` 那一档会产出（其余三档实测恒为 `[]`）。三键由
 * `augmentor/aggregator.py` 的 `aggregate_consistent()` 现造，`values` 与 `sources`
 * **等长且逐位对应**（同一个源出现两次就报两次，实测 `sources: ["甲","甲","乙","乙"]`）。
 */
export interface AggregateConflict {
  key: string
  values: string[]
  sources: string[]
}

/**
 * `POST /api/dataset/aggregate`
 *
 * `aggregated_count` 是**落盘条数**，`source_counts` 是各源**读进来**的条数，两者不必
 * 有关系（实测 consistent 单源同问不同答 ⇒ `source_counts {"甲":2}` 而 `aggregated_count 0`：
 * 两条都被判成冲突整份丢掉）。`conflicts` 是**明细整份上线**，与本组「写盘变换类只回写到哪、
 * 写了多少」的约定不一致，SDK 的 `to_dict()` 同一件事只报 `conflict_count`（账本 **A293**）。
 */
export interface AggregateResponse {
  output_file: string
  aggregated_count: number
  source_counts: Record<string, number>
  removed_duplicates: number
  conflicts: AggregateConflict[]
}

/**
 * `POST /api/dataset/rag`
 *
 * `format` 回的是**生效**的那一档：请求省略（或送空串，两者实测等价）时它给的是
 * `config.rag.default_format`（实测 `llamaindex`），不是空串 ⇒ 页面照抄回执。
 * `record_count` 实测三档都等于 `input_count`（短文本不触发切块），但三档落盘的**键集
 * 各不相同**（langchain 2 键 / llamaindex 6 键 / custom 5 键），所以这一格不能读成
 * 「条数 = 块数」的保证。
 */
export interface RagResponse {
  output_file: string
  format: string
  input_count: number
  record_count: number
}

// ============ 系统 · 数据备份（A279 第七批接线，L231） ============

/**
 * `GET /api/system/backups` 列表里的一项，也是 `POST /api/system/backups` 的整份回执
 * （与 `BackupInfo.to_dict()` 的七键逐字相等）
 *
 * `backup_id` **就是创建时传的 `name`**（省略时才是 `backup_<YYYYmmdd_HHMMSS>`），所以
 * 同名创建两次会覆盖同一份文件、并在索引里留下两条同 id 的记录（实测 list 回三条、
 * 磁盘只有一份），删一次把两条索引项一起清掉（`augmentor/backup.py` 的 delete 按 id 过滤
 * 全部匹配项）。`source_path` / `backup_path` 是**服务端的绝对路径**，页面照抄显示。
 * `timestamp` 是 isoformat，而 id 里那个时间戳是**另一次** `datetime.now()`（秒级），
 * 跨秒时两者会对不上。`checksum` 是备份文件的 sha256——它**只在恢复时写日志**，
 * 校验和不匹配照样 200 并把改坏的内容交回来（实测），所以这一格不能读成「完整性保证」。
 */
export interface BackupInfo {
  backup_id: string
  timestamp: string
  source_path: string
  backup_path: string
  item_count: number
  file_size: number
  checksum: string
}

/** `GET /api/system/backups`：空目录实测回 `{"backups": []}`，不是 404 也不是 null */
export interface BackupListResponse {
  backups: BackupInfo[]
}

/**
 * `POST /api/system/backups/{backup_id}/restore`
 *
 * 落点键叫 **`output_path`**，而同族其余五只写盘端点叫 `output_file`（`split` 叫
 * `output_dir`）——这一格是 SDK `restore()` 的返回键原样上线，不是笔误可改的（账本 A277
 * 当年订正的正是「文档把这一族一律写成 `output_path`」）。恢复**覆盖**目标文件（实测把
 * 一份写有别的内容的文件整份换掉），且不做校验和拦截。
 */
export interface RestoreBackupResponse {
  backup_id: string
  output_path: string
  item_count: number
}
