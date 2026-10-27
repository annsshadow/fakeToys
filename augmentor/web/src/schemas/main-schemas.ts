/**
 * 主链路接口的 zod 运行时 schema（Z 批次，10-26-r4 计划 §5）
 *
 * ## 为什么需要这一层
 *
 * `types/api.ts` 的 61 个接口是**编译期**声明，`axios` 的 `response.data` 是 `any`，
 * 后端若改了响应形态（少回键 / 类型漂移 / 键名变更），`tsc` 不会发现，页面只能靠
 * `undefined` 兜底或运行时崩。本层把「高频消费型」接口的响应形状用 zod 描述，
 * 在 `services/api.ts` 的返回前跑 `safeParse`，契约漂移在**运行时**被抓住。
 *
 * ## ground truth 是后端的 `response_model`，不是 `types/api.ts`
 *
 * 每个 schema 逐键对照 `augmentor/api/routes/*.py` 的 `response_model` 类定义
 * （字段名 / 可选性 / 容器类型），**不**按 TS 接口反推。两处已知差异按后端放宽：
 *
 * - `DataLoadResponse.items` / `VersionDataResponse.items` 声明为 `List[Dict[str, Any]]`
 *   ⇒ 元素是**任意对象**，不能写成 `DataItem`（`instruction`/`input`/`output` 是本项目
 *   的数据集约定，不是后端保证）；
 * - `DatasetAnalysisResponse` 的三个字段都是 `Dict[str, Any]` ⇒ 只断言外层键集。
 *
 * `VersionInfo.label` 反过来要紧一档：后端是 `Optional[str]` 且**不**排除 null，
 * 键恒在、值可为 null ⇒ `.nullable()` 而不是 `.optional()`。
 *
 * ## 覆盖范围
 *
 * 主链路 13 个接口（数据管理 / 数据增强 / 数据分析 / 版本管理 / 质量评估），
 * 不是全量 61 个接口——全量接线是后续轮次（§7 Z-FULL）。
 */
import { z } from 'zod'

// ============ 数据管理 ============

/** `GET /api/data/list` → `DataFileListResponse` */
export const dataFileInfoSchema = z.object({
  name: z.string(),
  path: z.string(),
  size: z.number(),
})

export const dataFileListResponseSchema = z.object({
  files: z.array(dataFileInfoSchema),
})

/** `GET /api/data/load/{filename}` → `DataLoadResponse`（`items` 元素是任意对象） */
export const dataPageResponseSchema = z.object({
  total: z.number(),
  page: z.number(),
  page_size: z.number(),
  items: z.array(z.record(z.string(), z.unknown())),
  indices: z.array(z.number()),
})

/** `POST /api/data/upload` → `DataUploadResponse` */
export const uploadResponseSchema = z.object({
  success: z.boolean(),
  path: z.string(),
  count: z.number(),
})

// ============ 数据增强 ============

/** `POST /api/augment/start` → `MessageResponse`（`api/schemas.py`） */
export const startAugmentResponseSchema = z.object({
  success: z.boolean(),
  message: z.string(),
})

/**
 * `GET /api/augment/progress` → `AugmentProgressResponse`
 *
 * 两形态互斥（`checkpoint.py: get_progress`）：空态只有 `status` 一键；任务态是
 * 11 个任务字段且**不含** `status`。路由开了 `response_model_exclude_none=True`
 * ⇒ 值为 null 的键会被整个省略。实测只有 `avg_quality_score` 会出现 null
 * （`_calculate_avg_quality_score` 无样本时回 None），故它单独 `.optional()`。
 *
 * `{}` 两形都不匹配 —— 这是有意的非空断言，防「字段全没了也判绿」。
 */
export const noCheckpointProgressSchema = z.object({
  status: z.string(),
})

export const augmentProgressDetailSchema = z.object({
  task_id: z.string(),
  total_items: z.number(),
  processed_items: z.number(),
  failed_items: z.number(),
  progress: z.number(),
  progress_percent: z.string(),
  elapsed_time: z.string(),
  remaining_time: z.string(),
  start_time: z.string(),
  last_update: z.string(),
  /** 无样本时后端回 None，键被 `exclude_none` 省略 */
  avg_quality_score: z.number().optional(),
})

export const augmentProgressSchema = z.union([
  noCheckpointProgressSchema,
  augmentProgressDetailSchema,
])

/** `GET /api/augment/checkpoints` → `CheckpointListResponse` */
export const checkpointListResponseSchema = z.object({
  checkpoints: z.array(z.string()),
})

// ============ 数据分析 ============

/**
 * `GET /api/analyze/{filename}` → `DatasetAnalysisResponse`
 *
 * 后端三个字段都是 `Dict[str, Any]`，只在**外层键集**上有承诺；内层结构由
 * `Pipeline.analyze_dataset()` 的实现决定，不属于 `response_model` 契约，
 * 因此不在这里固化（否则后端调整分析指标就会被误判为契约漂移）。
 */
export const analyzeResponseSchema = z.object({
  coverage_analysis: z.record(z.string(), z.unknown()),
  statistics: z.record(z.string(), z.unknown()),
  dedup_report: z.record(z.string(), z.unknown()),
})

/** `GET /api/visualize/{filename}` → `VisualizeResponse`（图表名 → 落盘路径） */
export const visualizeResponseSchema = z.object({
  charts: z.record(z.string(), z.string()),
})

// ============ 版本管理 ============

/** `VersionInfo`（`label` 键恒在、值可为 null） */
export const versionInfoSchema = z.object({
  version_id: z.string(),
  label: z.string().nullable(),
  description: z.string(),
  created_at: z.string(),
  item_count: z.number(),
})

/** `GET /api/versions` → `VersionListResponse` */
export const versionListResponseSchema = z.object({
  versions: z.array(versionInfoSchema),
})

/** `POST /api/versions/create` → `VersionCreateResponse` */
export const createVersionResponseSchema = z.object({
  success: z.boolean(),
  version_id: z.string(),
})

/** `GET /api/versions/{id}/data` → `VersionDataResponse`（`items` 元素是任意对象） */
export const versionDataResponseSchema = z.object({
  items: z.array(z.record(z.string(), z.unknown())),
})

// ============ 质量评估 ============

/** `POST /api/quality/evaluate` → `QualityEvaluateResponse` */
export const qualityEvaluationResponseSchema = z.object({
  total_samples: z.number(),
  passed_samples: z.number(),
  filtered_samples: z.number(),
  pass_rate: z.number(),
  avg_score: z.number(),
  threshold: z.number(),
  semantic_evaluated: z.boolean(),
  effective_weights: z.array(z.number()),
})

/** `POST /api/quality/dedup` → `DedupResponse` */
export const dedupResponseSchema = z.object({
  original_count: z.number(),
  deduplicated_count: z.number(),
  removed_count: z.number(),
  duplicate_groups: z.number(),
  kept_indices: z.array(z.number()),
})

// ============ 错误消息构造 ============

/**
 * 把 `safeParse` 失败转成可定位的中文错误信息
 *
 * 只保留「接口名 + 第一个出问题的字段 + issue code」，不裸抛 zod 的 `issues` 原文
 * （与后端 `to_http_error` 的收口判据同族：库内部措辞不透给页面）。
 */
export const zodErrorMessage = (schemaName: string, issues: readonly z.core.$ZodIssue[]): string => {
  const first = issues[0]
  const loc = first?.path.join('.') || '(root)'
  const code = first?.code || 'unknown'
  return `接口 ${schemaName} 的响应不符合契约：字段「${loc}」（${code}）。请检查前后端版本是否一致。`
}
