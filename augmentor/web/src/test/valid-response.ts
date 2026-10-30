/**
 * 给「只断言请求形状」的用例造一份**合法**响应（Z-FULL 收尾）
 *
 * ## 为什么需要它
 *
 * Z-FULL 给 `services/api.ts` 的 78 个函数全接上 zod 之后，`api.test.ts` /
 * `api.optional-defaults.test.ts` 里那批 `respondWith({})`、`respondWith({ ok: true })`
 * 的 fixture 全成了「坏响应」——这些用例本来只关心 URL / 方法 / 请求体，响应只是
 * 让 `await` 能落地，但校验层分不清「你不在乎」和「后端返回错了」。
 *
 * 修法不是逐个手抄 payload：抄出来的那份会在 schema 改动时静默过期（这正是
 * 这批用例过期的方式），而且 49 处各抄一份也没人读。这里改成**从 schema 现算**：
 * 用 `z.toJSONSchema()` 拿到形状，再取一份最小合法实例（必填键齐全、数组为空、
 * 数值为 0）。schema 变了，fixture 跟着变。
 *
 * ## 为什么按 `方法 + 路径` 查表
 *
 * 同一路径不同动词的响应模型不同（`GET /config` 与 `POST /config`、
 * `GET /system/backups` 与 `POST /system/backups`），只看 URL 会串。表里因此
 * 把动词一起钉住；路径参数统一写成 `:id`，匹配时按一段非斜杠字符展开。
 *
 * ## 判据
 *
 * 表里没有的端点**抛错**，不返回 `{}`——那会让「新端点没登记」退化成
 * 「校验失败但看不出为什么」。`valid-response.test.ts` 逐条自证每个样本都能
 * 被自己的 schema 吃下，防生成器退化成空转。
 */
import { z } from 'zod'

import * as full from '../schemas/full-schemas'
import * as main from '../schemas/main-schemas'

/** `axios.create()` 返回的实例里本层用到的那四个动词（`vi.fn()`） */
interface VerbMocks {
  get: unknown
  post: unknown
  put: unknown
  delete: unknown
}

/** 一条路由：`['GET', '/config', schema]`；`：id` 匹配一段路径参数 */
export type FixtureRoute = readonly [method: string, pattern: string, schema: z.ZodType]

/**
 * 需要合法响应的端点（按「先具体后通配」排列）
 *
 * 只登记 `services/api.ts` 里被接上 schema、且测试会用 `respondWithOk()`
 * 的那些；新增端点时若忘了登记，`respondWithOk()` 会当场抛错。
 */
export const ROUTES: readonly FixtureRoute[] = [
  // ---- 版本 ----
  ['GET', '/versions/history', full.versionHistoryResponseSchema],
  ['POST', '/versions/create', main.createVersionResponseSchema],
  ['POST', '/versions/diff', full.versionDiffResponseSchema],
  ['POST', '/versions/:id/rollback', full.successResponseSchema],
  ['GET', '/versions/:id/data', main.versionDataResponseSchema],
  ['DELETE', '/versions/:id', full.successResponseSchema],
  ['GET', '/versions/:id', full.versionDetailResponseSchema],
  ['GET', '/versions', main.versionListResponseSchema],
  // ---- 数据管理 ----
  ['GET', '/data/list', main.dataFileListResponseSchema],
  ['PUT', '/data/update/:id', full.successResponseSchema],
  ['DELETE', '/data/delete/:id', full.successResponseSchema],
  ['POST', '/data/export', full.exportResponseSchema],
  // ---- 质量 ----
  ['POST', '/quality/report', full.qualityReportResponseSchema],
  ['POST', '/quality/benchmark', full.benchmarkResponseSchema],
  ['POST', '/quality/clean', full.cleanResponseSchema],
  ['POST', '/quality/annotate', full.annotateResponseSchema],
  ['POST', '/quality/outliers', full.outlierResponseSchema],
  ['POST', '/quality/profiling', full.profilingResponseSchema],
  // ---- 导出 ----
  ['GET', '/export/formats', full.exportFormatsResponseSchema],
  ['POST', '/export/preview', full.previewResponseSchema],
  ['POST', '/export/batch', full.batchExportResponseSchema],
  // ---- 多模态 ----
  ['GET', '/multimodal/formats', full.multimodalFormatsResponseSchema],
  ['POST', '/multimodal/process', full.multimodalRecordResponseSchema],
  ['POST', '/multimodal/scan', full.multimodalScanResponseSchema],
  // ---- 配置与健康 ----
  ['GET', '/config', full.configResponseSchema],
  ['POST', '/config', full.configUpdateResponseSchema],
  ['GET', '/models', full.modelsResponseSchema],
  ['GET', '/health', full.healthResponseSchema],
  ['GET', '/status', full.statusResponseSchema],
  // ---- 隐私 ----
  ['GET', '/privacy/patterns', full.piiPatternsResponseSchema],
  ['POST', '/privacy/sanitize', full.sanitizeResponseSchema],
  // ---- 泄漏与审计 ----
  ['POST', '/leakage/check', full.leakageResponseSchema],
  ['POST', '/audit', full.auditReportSchema],
  // ---- 系统（同一路径不同动词的，动词已经进键）----
  ['GET', '/system/backups/verify', full.backupVerifyResponseSchema],
  ['POST', '/system/backups/:id/restore', full.restoreBackupResponseSchema],
  ['GET', '/system/backups', full.backupListResponseSchema],
  ['POST', '/system/backups', full.backupInfoResponseSchema],
  ['GET', '/system/dependency/datasets', full.dependencyDatasetListResponseSchema],
  ['POST', '/system/dependency/datasets', full.datasetInfoResponseSchema],
  ['POST', '/system/monitor', full.qualitySnapshotResponseSchema],
  ['POST', '/system/auto-test', full.testSuiteResponseSchema],
  ['POST', '/system/migrate', full.migrationResponseSchema],
  ['POST', '/system/stream', full.streamReportResponseSchema],
  ['POST', '/system/validate-config', full.configValidationResponseSchema],
  // ---- 数据集工具 ----
  ['POST', '/dataset/compare', full.compareResponseSchema],
  ['POST', '/dataset/convert', full.convertResponseSchema],
  ['POST', '/dataset/merge', full.mergeResponseSchema],
  ['POST', '/dataset/split', full.splitResponseSchema],
  ['POST', '/dataset/sample', full.sampleResponseSchema],
  ['POST', '/dataset/aggregate', full.aggregateResponseSchema],
  ['POST', '/dataset/rag', full.ragResponseSchema],
  ['POST', '/dataset/semantic-search', full.semanticSearchResponseSchema],
]

/** JSON Schema 里本仓会用到的键（只取生成最小实例需要的那几格） */
interface JsonSchemaNode {
  /** zod 4 的 `.nullable()` 会出成 `['string', 'null']`，不是 anyOf */
  type?: string | string[]
  properties?: Record<string, JsonSchemaNode>
  required?: string[]
  items?: JsonSchemaNode
  anyOf?: JsonSchemaNode[]
  oneOf?: JsonSchemaNode[]
  enum?: unknown[]
}

/** 从 JSON Schema 取一份最小合法实例：必填键齐全，数组取空，数值取 0 */
function minimalValue(node: JsonSchemaNode): unknown {
  const branch = node.anyOf?.[0] ?? node.oneOf?.[0]
  if (branch) return minimalValue(branch)
  if (node.enum) return node.enum[0]
  const types = Array.isArray(node.type) ? node.type : node.type ? [node.type] : []
  // 联合类型里取第一个非 null 分支：`x: string | null` 给 '' 比给 null 更能验证下游
  const type = types.find((candidate) => candidate !== 'null') ?? types[0]
  if (type === 'object' || node.properties) {
    const out: Record<string, unknown> = {}
    for (const key of node.required ?? []) {
      const child = node.properties?.[key]
      if (child) out[key] = minimalValue(child)
    }
    return out
  }
  if (type === 'array') return []
  if (type === 'string') return ''
  if (type === 'number' || type === 'integer') return 0
  if (type === 'boolean') return false
  if (type === 'null') return null
  return {}
}

/** 由 schema 现算的最小合法响应体 */
export function validSample(schema: z.ZodType): unknown {
  return minimalValue(z.toJSONSchema(schema) as JsonSchemaNode)
}

/** 路径模式 → 正则（`:id` 展开成一段非斜杠字符；其余按字面量） */
function toRegExp(pattern: string): RegExp {
  const escaped = pattern.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
  return new RegExp(`^${escaped.replace(/:id/g, '[^/]+')}$`)
}

/** 查表；查不到就抛（新端点没登记时不许静默回 `{}`） */
export function responseFor(method: string, url: string): unknown {
  const path = url.split('?')[0]
  for (const [routeMethod, pattern, schema] of ROUTES) {
    if (routeMethod === method && toRegExp(pattern).test(path)) {
      return validSample(schema)
    }
  }
  throw new Error(
    `valid-response: ${method} ${path} 没有登记合法响应。` +
      '新增端点后请在 ROUTES 里补一行（schema 从 full-schemas / main-schemas 取）。',
  )
}

/**
 * 让本测试里所有请求都收到「按 URL 现算的合法响应」
 *
 * 与 `respondWith(data)` 的区别：那个钉死一份数据、由调用方保证它合法；
 * 这个只保证**形状合法**，测试要断言返回值时仍应用 `respondWith`。
 */
export function respondWithOk(instance: VerbMocks): void {
  const answer = (method: string) => (url: string) =>
    Promise.resolve({ data: responseFor(method, url) })
  ;(instance.get as { mockImplementation: (fn: unknown) => void }).mockImplementation(answer('GET'))
  ;(instance.post as { mockImplementation: (fn: unknown) => void }).mockImplementation(
    answer('POST'),
  )
  ;(instance.put as { mockImplementation: (fn: unknown) => void }).mockImplementation(answer('PUT'))
  ;(instance.delete as { mockImplementation: (fn: unknown) => void }).mockImplementation(
    answer('DELETE'),
  )
}
