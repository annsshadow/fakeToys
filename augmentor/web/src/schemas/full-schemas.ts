/**
 * 全量响应契约的 zod schema（Z-FULL，10-27 计划 §4）
 *
 * ground truth 仍是后端 pydantic `response_model` 的类注解（`augmentor/api` 下的路由模块）。
 * 本文件由「类名 → 字段名 / 注解」静态生成后人工复核；跨语言键集与必填性由
 * `main-schemas.conformance.test.ts` 双向比对。
 *
 * 复杂值（`Dict[str, Any]` / `Any`）按后端承诺只校验外层键，内层不固化——
 * 与 `main-schemas.ts` 的既有口径一致。
 */
import { z } from 'zod'

/** `api/routes/dataset_tools.py:351` → AggregateResponse */
export const aggregateResponseSchema = z.object({
  output_file: z.string(),
  aggregated_count: z.number(),
  source_counts: z.record(z.string(), z.number()),
  removed_duplicates: z.number(),
  conflicts: z.array(z.record(z.string(), z.unknown())),
})

/** `api/routes/quality.py:88` → AnnotateResponse */
export const annotateResponseSchema = z.object({
  total_items: z.number(),
  entity_count: z.number(),
  avg_entities_per_item: z.number(),
  intent_distribution: z.record(z.string(), z.number()),
  sentiment_distribution: z.record(z.string(), z.number()),
  text_key: z.string(),
})

/** `augmentor/audit.py:28` → AuditReport */
export const auditReportSchema = z.object({
  total_items: z.number(),
  pii_matches: z.number(),
  duplicate_count: z.number(),
  duplicate_rate: z.number(),
  empty_field_rate: z.number(),
  leak_count: z.number().nullable(),
  leak_rate: z.number().nullable(),
  findings: z.array(z.string()),
  ready: z.boolean(),
})

/** `api/routes/dataset_tools.py:360` → AutoConfigResponse */
export const autoConfigResponseSchema = z.object({
  quality_threshold: z.number(),
  dedup_threshold: z.number(),
  recommended_sample_size: z.number(),
  reasoning: z.array(z.string()),
  source_stats: z.record(z.string(), z.unknown()),
})

/** `api/routes/system_ops.py:766` → BackupCleanupResponse */
export const backupCleanupResponseSchema = z.object({
  deleted_count: z.number(),
  max_backups: z.number(),
})

/** `api/routes/system_ops.py:253` → BackupInfoResponse */
export const backupInfoResponseSchema = z.object({
  backup_id: z.string(),
  timestamp: z.string(),
  source_path: z.string(),
  backup_path: z.string(),
  item_count: z.number(),
  file_size: z.number(),
  checksum: z.string(),
})

/** `api/routes/system_ops.py:264` → BackupListResponse */
export const backupListResponseSchema = z.object({
  backups: z.array(backupInfoResponseSchema),
})

/** `api/routes/system_ops.py:742` → BackupVerifyEntryResponse */
export const backupVerifyEntryResponseSchema = z.object({
  backup_id: z.string(),
  exists: z.boolean(),
  checksum_match: z.boolean(),
  item_count: z.number(),
  timestamp: z.string(),
})

/** `api/routes/system_ops.py:756` → BackupVerifyResponse */
export const backupVerifyResponseSchema = z.object({
  results: z.array(backupVerifyEntryResponseSchema),
})

/** `api/routes/export.py:32` → BatchExportResponse */
export const batchExportResponseSchema = z.object({
  success: z.boolean(),
  results: z.record(z.string(), z.record(z.string(), z.string())),
})

/** `api/routes/quality.py:98` → BenchmarkResponse */
export const benchmarkResponseSchema = z.object({
  sample_count: z.number(),
  metrics: z.record(z.string(), z.number()),
  all_metrics: z.record(z.string(), z.number()),
  threshold: z.number(),
  timestamp: z.string(),
})

/** `api/routes/quality.py:73` → CleanResponse */
export const cleanResponseSchema = z.object({
  original_count: z.number(),
  cleaned_count: z.number(),
  dropped_count: z.number(),
  changed_count: z.number(),
  language_distribution: z.record(z.string(), z.number()),
  issues: z.record(z.string(), z.number()),
})

/** `api/routes/dataset_tools.py:322` → CompareResponse */
export const compareResponseSchema = z.object({
  quality_comparison: z.record(z.string(), z.unknown()),
  overlap_comparison: z.record(z.string(), z.unknown()),
})

/** `api/routes/config.py:28` → ConfigResponse */
export const configResponseSchema = z.object({
  default_model: z.string(),
  augmentation: z.record(z.string(), z.unknown()),
  quality: z.record(z.string(), z.unknown()),
  dedup: z.record(z.string(), z.unknown()),
  export: z.record(z.string(), z.unknown()),
  vector: z.record(z.string(), z.unknown()),
  rag: z.record(z.string(), z.unknown()),
  multimodal: z.record(z.string(), z.unknown()),
  versioning: z.record(z.string(), z.unknown()),
  visualization: z.record(z.string(), z.unknown()),
})

/** `api/routes/config.py:78` → ConfigUpdateResponse */
export const configUpdateResponseSchema = z.object({
  success: z.boolean(),
  message: z.string(),
  ignored_keys: z.array(z.string()),
})

/** `api/routes/system_ops.py:135` → ConfigValidationIssue */
export const configValidationIssueSchema = z.object({
  path: z.string(),
  message: z.string(),
  severity: z.string(),
  line: z.number().nullable(),
})

/** `api/routes/system_ops.py:143` → ConfigValidationResponse */
export const configValidationResponseSchema = z.object({
  is_valid: z.boolean(),
  summary: z.record(z.string(), z.number()),
  errors: z.array(configValidationIssueSchema),
  warnings: z.array(configValidationIssueSchema),
  info: z.array(configValidationIssueSchema),
})

/** `api/routes/dataset_tools.py:260` → ConvertResponse */
export const convertResponseSchema = z.object({
  input_file: z.string(),
  output_file: z.string(),
  source_format: z.string(),
  target_format: z.string(),
  input_count: z.number(),
  output_count: z.number(),
})

/** `api/routes/system_ops.py:223` → DatasetInfoResponse */
export const datasetInfoResponseSchema = z.object({
  dataset_id: z.string(),
  name: z.string(),
  description: z.string(),
  path: z.string(),
  item_count: z.number(),
  created_at: z.string(),
  updated_at: z.string(),
  tags: z.array(z.string()),
  metadata: z.record(z.string(), z.unknown()),
})

/** `api/routes/data.py:93` → DemoItem */
export const demoItemSchema = z.object({
  instruction: z.string(),
  input: z.string(),
  output: z.string(),
})

/** `api/routes/system_ops.py:126` → DependenciesResponse */
export const dependenciesResponseSchema = z.object({
  installed: z.record(z.string(), z.boolean()),
  missing: z.array(z.string()),
  degraded_features: z.array(z.string()),
  all_required_present: z.boolean(),
  available_count: z.number(),
})

/** `api/routes/system_ops.py:236` → DependencyDatasetListResponse */
export const dependencyDatasetListResponseSchema = z.object({
  datasets: z.array(datasetInfoResponseSchema),
})

/** `api/routes/system_ops.py:241` → DependencyGraphResponse */
export const dependencyGraphResponseSchema = z.object({
  nodes: z.array(z.unknown()),
  edges: z.array(z.unknown()),
  issues: z.array(z.record(z.string(), z.unknown())),
})

/** `api/routes/dataset_tools.py:389` → EvaluateResponse */
export const evaluateResponseSchema = z.object({
  metrics: z.record(z.string(), z.number()),
  sample_count: z.number(),
  details: z.array(z.record(z.string(), z.unknown())),
})

/** `api/routes/export.py:47` → ExportFormatsResponse */
export const exportFormatsResponseSchema = z.object({
  formats: z.array(z.string()),
})

/** `api/routes/export.py:26` → ExportResponse */
export const exportResponseSchema = z.object({
  success: z.boolean(),
  files: z.record(z.string(), z.string()),
})

/** `api/routes/dataset_tools.py:334` → FieldFeature */
export const fieldFeatureSchema = z.object({
  name: z.string(),
  feature_type: z.string(),
  coverage: z.number(),
  unique_ratio: z.number(),
  description: z.string(),
})

/** `api/routes/dataset_tools.py:343` → FeaturesResponse */
export const featuresResponseSchema = z.object({
  total_items: z.number(),
  field_features: z.array(fieldFeatureSchema),
  sparse_fields: z.array(z.string()),
  intent_distribution: z.record(z.string(), z.number()),
})

/** `api/routes/quality.py:476` → GateReportResponse */
export const gateReportResponseSchema = z.object({
  verdict: z.string(),
  passed: z.boolean(),
  failed_rules: z.array(z.string()),
  warned_rules: z.array(z.string()),
  errored_rules: z.array(z.string()),
  metrics: z.record(z.string(), z.unknown()),
})

/** `api/routes/quality.py:459` → HealthMetricsResponse */
export const healthMetricsResponseSchema = z.object({
  completeness: z.number(),
  diversity: z.number(),
  quality_balance: z.number(),
  coverage: z.number(),
})

/** `api/routes/quality.py:467` → HealthReportResponse */
export const healthReportResponseSchema = z.object({
  health_score: z.number(),
  level: z.string(),
  metrics: healthMetricsResponseSchema,
  weights: z.array(z.number()),
  total_samples: z.number(),
})

/** `api/routes/quality.py:494` → HealthGateResponse */
export const healthGateResponseSchema = z.object({
  health: healthReportResponseSchema,
  gate: gateReportResponseSchema,
  skipped_rules: z.array(z.string()),
})

/** `api/routes/status.py:42` → HealthResponse */
export const healthResponseSchema = z.object({
  status: z.string(),
  version: z.string(),
})

/** `api/routes/dataset_tools.py:377` → ImpactResponse */
export const impactResponseSchema = z.object({
  before: z.record(z.string(), z.unknown()),
  after: z.record(z.string(), z.unknown()),
  gains: z.record(z.string(), z.number()),
  beneficial: z.boolean(),
})

/** `api/routes/language.py:52` → LanguageProfileResponse */
export const languageProfileResponseSchema = z.object({
  total_items: z.number(),
  distribution: z.record(z.string(), z.number()),
  ratios: z.record(z.string(), z.number()),
  outside_supported: z.array(z.string()),
  supported_langs: z.array(z.string()),
  default_target_lang: z.string(),
})

/** `api/routes/leakage.py:27` → LeakageResponse */
export const leakageResponseSchema = z.object({
  train_size: z.number(),
  test_size: z.number(),
  exact_leaks: z.number(),
  fuzzy_leaks: z.number(),
  total_leaks: z.number(),
  leak_rate: z.number(),
  is_clean: z.boolean(),
  leaked_examples: z.array(z.record(z.string(), z.unknown())),
})

/** `api/routes/dataset_tools.py:270` → MergeResponse */
export const mergeResponseSchema = z.object({
  input_files: z.number(),
  output_file: z.string(),
  total_input: z.number(),
  total_output: z.number(),
  removed_duplicates: z.number(),
})

/** `api/routes/system_ops.py:203` → MigrationResponse */
export const migrationResponseSchema = z.object({
  migration_id: z.string(),
  source_path: z.string(),
  target_path: z.string(),
  total_items: z.number(),
  migrated_items: z.number(),
  failed_items: z.number(),
  rules_applied: z.array(z.string()),
  errors: z.array(z.record(z.string(), z.unknown())),
  timestamp: z.string(),
})

/** `api/routes/config.py:49` → ModelsResponse */
export const modelsResponseSchema = z.object({
  models: z.array(z.string()),
  default: z.string(),
})

/** `api/routes/multimodal.py:51` → MultimodalFormatsResponse */
export const multimodalFormatsResponseSchema = z.object({
  image_extensions: z.array(z.string()),
  audio_extensions: z.array(z.string()),
})

/** `api/routes/multimodal.py:22` → MultimodalRecordResponse */
export const multimodalRecordResponseSchema = z.object({
  text: z.string(),
  image: z.record(z.string(), z.unknown()).nullable(),
  audio: z.record(z.string(), z.unknown()).nullable(),
  modalities: z.array(z.string()),
  valid: z.boolean(),
  errors: z.array(z.string()),
})

/** `api/routes/multimodal.py:37` → MultimodalScanResponse */
export const multimodalScanResponseSchema = z.object({
  total_records: z.number(),
  valid_records: z.number(),
  invalid_records: z.number(),
  modality_counts: z.record(z.string(), z.number()),
  error_count: z.number(),
  errors: z.array(z.string()),
  records: z.array(multimodalRecordResponseSchema),
})

/** `api/routes/quality.py:343` → OutlierResponse */
export const outlierResponseSchema = z.object({
  total_items: z.number(),
  outlier_count: z.number(),
  outlier_rate: z.number(),
  method: z.string(),
  threshold: z.number(),
  field: z.string(),
  outliers: z.array(z.record(z.string(), z.unknown())),
})

/** `api/routes/privacy.py:42` → PiiPatternsResponse */
export const piiPatternsResponseSchema = z.object({
  default: z.array(z.string()),
  extra: z.array(z.string()),
})

/** `api/routes/export.py:38` → PreviewResponse */
export const previewResponseSchema = z.object({
  format: z.string(),
  original_data: z.array(z.record(z.string(), z.unknown())),
  converted_data: z.array(z.record(z.string(), z.unknown())),
  format_info: z.record(z.string(), z.unknown()),
  warnings: z.array(z.string()),
})

/** `api/routes/quality.py:358` → ProfilingResponse */
export const profilingResponseSchema = z.object({
  total_items: z.number(),
  field_completeness: z.record(z.string(), z.unknown()),
  length_stats: z.record(z.string(), z.unknown()),
  duplicate_rate: z.number(),
  language_distribution: z.record(z.string(), z.unknown()),
  top_keywords: z.array(z.unknown()),
})

/** `api/routes/system_ops.py:157` → QualityAlertResponse */
export const qualityAlertResponseSchema = z.object({
  alert_id: z.string(),
  metric_name: z.string(),
  current_value: z.number(),
  threshold_value: z.number(),
  alert_type: z.string(),
  severity: z.string(),
  message: z.string(),
  timestamp: z.string(),
})

/** `api/routes/quality.py:59` → QualityReportResponse */
export const qualityReportResponseSchema = z.object({
  total_samples: z.number(),
  passed_samples: z.number(),
  filtered_samples: z.number(),
  pass_rate: z.number(),
  score_distribution: z.record(z.string(), z.number()),
  filter_statistics: z.record(z.string(), z.number()),
  metric_summary: z.record(z.string(), z.record(z.string(), z.number())),
  dedup_summary: z.record(z.string(), z.unknown()),
  improvement_suggestions: z.array(z.string()),
  charts: z.record(z.string(), z.unknown()),
})

/** `api/routes/system_ops.py:169` → QualitySnapshotResponse */
export const qualitySnapshotResponseSchema = z.object({
  snapshot_id: z.string(),
  timestamp: z.string(),
  metrics: z.record(z.string(), z.number()),
  alerts: z.array(qualityAlertResponseSchema),
})

/** `api/routes/dataset_tools.py:369` → RagResponse */
export const ragResponseSchema = z.object({
  output_file: z.string(),
  format: z.string(),
  input_count: z.number(),
  record_count: z.number(),
})

/** `api/routes/system_ops.py:269` → RestoreBackupResponse */
export const restoreBackupResponseSchema = z.object({
  backup_id: z.string(),
  output_path: z.string(),
  item_count: z.number(),
})

/** `api/routes/dataset_tools.py:279` → SampleResponse */
export const sampleResponseSchema = z.object({
  input_file: z.string(),
  output_file: z.string(),
  input_count: z.number(),
  output_count: z.number(),
})

/** `api/routes/privacy.py:24` → SanitizeReportResponse */
export const sanitizeReportResponseSchema = z.object({
  total_items: z.number(),
  touched_items: z.number(),
  matches: z.record(z.string(), z.number()),
  total_matches: z.number(),
})

/** `api/routes/privacy.py:36` → SanitizeResponse */
export const sanitizeResponseSchema = z.object({
  items: z.array(z.record(z.string(), z.unknown())),
  report: sanitizeReportResponseSchema,
})

/** `api/routes/dataset_tools.py:300` → SearchResponse */
export const searchResponseSchema = z.object({
  query: z.string(),
  method: z.string(),
  total_matches: z.number(),
  query_time_ms: z.number(),
  items: z.array(z.record(z.string(), z.unknown())),
  highlights: z.array(z.record(z.string(), z.unknown())),
  fuzzy_threshold: z.number().nullable(),
  ngram_n: z.number().nullable(),
  applied_filters: z.array(z.record(z.string(), z.unknown())).nullable(),
  matches_before_filters: z.number().nullable(),
})

/** `api/routes/vector.py:46` → SemanticSearchResponse */
export const semanticSearchResponseSchema = z.object({
  query: z.string(),
  total: z.number(),
  backend: z.string(),
  dimension: z.number(),
  collection: z.string(),
  results: z.array(z.record(z.string(), z.unknown())),
})

/** `api/routes/dataset_tools.py:287` → SplitPart */
export const splitPartSchema = z.object({
  file: z.string(),
  count: z.number(),
})

/** `api/routes/dataset_tools.py:293` → SplitResponse */
export const splitResponseSchema = z.object({
  input_file: z.string(),
  output_dir: z.string(),
  splits: z.record(z.string(), splitPartSchema),
})

/** `api/routes/dataset_tools.py:236` → StatisticsResponse */
export const statisticsResponseSchema = z.object({
  dataset_name: z.string(),
  total_items: z.number(),
  summary: z.string(),
  field_statistics: z.record(z.string(), z.unknown()),
  content_statistics: z.record(z.string(), z.unknown()),
  quality_metrics: z.record(z.string(), z.number()),
})

/** `api/routes/status.py:23` → StatusResponse */
export const statusResponseSchema = z.object({
  status: z.string(),
  version: z.string(),
  model_default: z.string(),
  model_available: z.boolean(),
  model_retry_count: z.number().nullable(),
  model_retry_wait_seconds: z.number().nullable(),
  model_generate_seconds: z.number().nullable(),
  dependencies: z.record(z.string(), z.unknown()),
})

/** `api/routes/system_ops.py:216` → StreamReportResponse */
export const streamReportResponseSchema = z.object({
  total_input: z.number(),
  total_output: z.number(),
  processed: z.number(),
})

/** `api/schemas.py:15` → SuccessResponse */
export const successResponseSchema = z.object({
  success: z.boolean(),
})

/** `api/routes/system_ops.py:177` → TestResultResponse */
export const testResultResponseSchema = z.object({
  test_id: z.string(),
  test_name: z.string(),
  passed: z.boolean(),
  message: z.string(),
  execution_time_ms: z.number(),
  timestamp: z.string(),
})

/** `api/routes/system_ops.py:187` → TestSuiteResponse */
export const testSuiteResponseSchema = z.object({
  name: z.string(),
  description: z.string(),
  total_tests: z.number(),
  passed_tests: z.number(),
  failed_tests: z.number(),
  pass_rate: z.number(),
  results: z.array(testResultResponseSchema),
})

/** `api/routes/language.py:85` → TranslateResponse */
export const translateResponseSchema = z.object({
  total: z.number(),
  batches: z.number(),
  batch_size: z.number(),
  target_lang: z.string(),
  supported_langs: z.array(z.string()),
  items: z.array(z.record(z.string(), z.unknown())),
  output_file: z.string().nullable(),
  failed: z.number(),
  failed_indices: z.array(z.number()),
  errors: z.array(z.record(z.string(), z.unknown())),
  errors_truncated: z.boolean(),
})

/** `api/routes/dataset_tools.py:250` → ValidationResponse */
export const validationResponseSchema = z.object({
  is_valid: z.boolean(),
  total_items: z.number(),
  valid_items: z.number(),
  error_count: z.number(),
  warning_count: z.number(),
  issues: z.array(z.record(z.string(), z.unknown())),
})

/** `api/routes/vector.py:61` → VectorStatusResponse */
export const vectorStatusResponseSchema = z.object({
  enabled: z.boolean(),
  exists: z.boolean(),
  count: z.number(),
  backend: z.string(),
  dimension: z.number(),
  collection: z.string(),
  storage_dir: z.string(),
})

/** `api/routes/version.py:30` → VersionDetailResponse */
export const versionDetailResponseSchema = z.object({
  version_id: z.string(),
  label: z.string().nullable(),
  description: z.string(),
  created_at: z.string(),
  item_count: z.number(),
  metadata: z.record(z.string(), z.unknown()),
})

/** `api/routes/version.py:46` → VersionDiffResponse */
export const versionDiffResponseSchema = z.object({
  version1: z.string(),
  version2: z.string(),
  added_count: z.number(),
  removed_count: z.number(),
  modified_count: z.number(),
})

/** `api/routes/version.py:58` → VersionHistoryResponse */
export const versionHistoryResponseSchema = z.object({
  history: z.array(z.record(z.string(), z.unknown())),
})
