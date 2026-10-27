/**
 * zod schema ↔ 后端 pydantic `response_model` 的跨语言契约守卫
 *
 * ## 为什么是「静态解析」而不是「打真接口」
 *
 * `services/api.ts` 的 zod 校验解决的是「运行时形状不对就抛」，但**schema 自己写错**
 * 时它抓不到——schema 与后端一致地错、或者把必填写成可选，`safeParse` 照样绿。
 * 本文件的判据是：把 `augmentor/api` 下路由文件的 `class X(BaseModel)` 字段名与
 * 可选性**直接解析出来**，和 zod 的 key 集逐键比对。这样 zod 的 ground truth
 * 不靠人复述，而是后端源码本身。
 *
 * 本机没有 augmentor 的 Python 解释器（`fastapi`/`pydantic` 均不可用，见计划 §8），
 * 所以不走 `TestClient` 生成真实响应——那一路的运行时证据由后端的 pytest 套件提供，
 * 本文件只守「两侧声明是否同形」。
 *
 * ## 防恒真的三道锁
 *
 * 1. **非空元断言**：模型数、总键数、每个模型的键数都断言 ≥ 1；解析器找不到类时
 *    返回 `null` 而不是空数组（空数组会让「⊆」永远成立）。
 * 2. **字面量锚点**：`VersionInfo` 的键集写成字面量数组，解析器若退化成空转立刻红。
 * 3. **反向注入**：给键集人为多塞/少塞一个键，断言比对函数真的能报出差异。
 *
 * 只覆盖本批接了 zod 的 13 个响应模型（`dataFileInfoSchema` 是
 * `dataFileListResponseSchema` 的元素，单独列一行）。
 */
import { readFileSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

import { describe, expect, it } from 'vitest'
import { z } from 'zod'

import {
  analyzeResponseSchema,
  augmentProgressSchema,
  checkpointListResponseSchema,
  createVersionResponseSchema,
  dataFileInfoSchema,
  dataFileListResponseSchema,
  dataPageResponseSchema,
  dedupResponseSchema,
  qualityEvaluationResponseSchema,
  startAugmentResponseSchema,
  uploadResponseSchema,
  versionDataResponseSchema,
  versionInfoSchema,
  versionListResponseSchema,
  visualizeResponseSchema,
} from './main-schemas'

/** `augmentor/api`：本文件在 `augmentor/web/src/schemas/`，上三级就是 `augmentor/` */
const API_DIR = resolve(dirname(fileURLToPath(import.meta.url)), '../../../api')

/** pydantic 模型里的一个字段 */
interface ModelField {
  name: string
  /** 有 `= 默认值`（含 `= None`）⇒ 键可能缺席；无默认值 ⇒ 响应里恒在 */
  hasDefault: boolean
}

/**
 * 从 Python 源码里解析 `class <name>(...)` 的字段
 *
 * 只处理本仓响应模型的实际写法：顶层 `class X(BaseModel):`、类体 4 空格缩进、
 * 每字段一行注解、docstring 用 `"""`。解析不到类时返回 `null`——**不返回空数组**，
 * 空数组会让后续的集合包含判断恒真。
 */
function parseModelFields(source: string, className: string): ModelField[] | null {
  const lines = source.split(/\r?\n/)
  const start = lines.findIndex((line) => line.startsWith(`class ${className}(`))
  if (start === -1) return null

  const fields: ModelField[] = []
  let inDocstring = false

  for (let i = start + 1; i < lines.length; i += 1) {
    const line = lines[i]
    // 下一个顶层语句 / 下一个 class：当前类体结束
    if (/^\S/.test(line)) break

    const quoteCount = line.split('"""').length - 1
    if (inDocstring) {
      if (quoteCount % 2 === 1) inDocstring = false
      continue
    }
    if (quoteCount % 2 === 1) {
      inDocstring = true
      continue
    }
    if (quoteCount > 0) continue // 单行 docstring

    const match = /^ {4}([A-Za-z_]\w*)\s*:\s*(.+)$/.exec(line)
    if (!match) continue
    const [, name, annotation] = match
    // 注解里不会出现 `=`；出现即字段默认值（`Optional[str] = None` 也算）
    fields.push({ name, hasDefault: annotation.includes('=') })
  }

  return fields
}

/**
 * 取 zod schema 声明的键集
 *
 * - 对象 schema：`shape` 的键；
 * - union（`augmentProgressSchema` 的两形态）：各分支键集的并集，
 *   与 pydantic 那个「全字段 Optional」的单一模型相对照。
 */
function zodKeys(schema: z.ZodType): string[] | null {
  if (schema instanceof z.ZodObject) {
    return Object.keys(schema.shape)
  }
  if (schema instanceof z.ZodUnion) {
    const keys = new Set<string>()
    for (const option of schema.options as z.ZodType[]) {
      const branch = zodKeys(option)
      if (branch === null) return null
      for (const key of branch) keys.add(key)
    }
    return [...keys]
  }
  return null
}

/** 两侧键集差异；空数组 = 同形 */
function diffKeys(actual: string[], expected: string[]): string[] {
  const actualSet = new Set(actual)
  const expectedSet = new Set(expected)
  const problems: string[] = []
  for (const key of actualSet) {
    if (!expectedSet.has(key)) problems.push(`zod 多出：${key}`)
  }
  for (const key of expectedSet) {
    if (!actualSet.has(key)) problems.push(`zod 缺少：${key}`)
  }
  return problems.sort()
}

/** schema 是否允许该键缺席（`undefined` 能过就是可缺席） */
function acceptsMissing(schema: z.ZodType, key: string): boolean {
  if (!(schema instanceof z.ZodObject)) {
    throw new Error('acceptsMissing 只接受 object schema；union 的必填性由调用方另行判定')
  }
  return schema.shape[key].safeParse(undefined).success
}

interface ModelCase {
  schema: z.ZodType
  className: string
  /** 相对 `augmentor/api/` 的路径 */
  file: string
}

const MODELS: ModelCase[] = [
  { schema: dataFileInfoSchema, className: 'DataFileInfo', file: 'routes/data.py' },
  { schema: dataFileListResponseSchema, className: 'DataFileListResponse', file: 'routes/data.py' },
  { schema: dataPageResponseSchema, className: 'DataLoadResponse', file: 'routes/data.py' },
  { schema: uploadResponseSchema, className: 'DataUploadResponse', file: 'routes/data.py' },
  { schema: analyzeResponseSchema, className: 'DatasetAnalysisResponse', file: 'routes/data.py' },
  { schema: visualizeResponseSchema, className: 'VisualizeResponse', file: 'routes/data.py' },
  { schema: startAugmentResponseSchema, className: 'MessageResponse', file: 'schemas.py' },
  { schema: checkpointListResponseSchema, className: 'CheckpointListResponse', file: 'routes/augment.py' },
  { schema: augmentProgressSchema, className: 'AugmentProgressResponse', file: 'routes/augment.py' },
  { schema: versionInfoSchema, className: 'VersionInfo', file: 'routes/version.py' },
  { schema: versionListResponseSchema, className: 'VersionListResponse', file: 'routes/version.py' },
  { schema: createVersionResponseSchema, className: 'VersionCreateResponse', file: 'routes/version.py' },
  { schema: versionDataResponseSchema, className: 'VersionDataResponse', file: 'routes/version.py' },
  {
    schema: qualityEvaluationResponseSchema,
    className: 'QualityEvaluateResponse',
    file: 'routes/quality.py',
  },
  { schema: dedupResponseSchema, className: 'DedupResponse', file: 'routes/quality.py' },
]

/** 源文件只读一次（15 个模型命中 5 个文件） */
const sourceCache = new Map<string, string>()
function readSource(relativePath: string): string {
  const cached = sourceCache.get(relativePath)
  if (cached !== undefined) return cached
  const text = readFileSync(resolve(API_DIR, relativePath), 'utf8')
  sourceCache.set(relativePath, text)
  return text
}

describe('schema 解析器自证（防恒真）', () => {
  it('能从 version.py 读出 VersionInfo 的完整字段（字面量锚点）', () => {
    const fields = parseModelFields(readSource('routes/version.py'), 'VersionInfo')

    expect(fields?.map((field) => field.name)).toEqual([
      'version_id',
      'label',
      'description',
      'created_at',
      'item_count',
    ])
    // `label: Optional[str]` 无默认值 ⇒ 键恒在、值可为 null
    expect(fields?.find((field) => field.name === 'label')?.hasDefault).toBe(false)
  })

  it('解析不到的类返回 null，而不是空数组', () => {
    expect(parseModelFields(readSource('routes/version.py'), 'NoSuchModel')).toBeNull()
  })

  it('反向注入：键集多一个 / 少一个都必须被报出来', () => {
    const expected = parseModelFields(readSource('routes/version.py'), 'VersionInfo')
    expect(expected).not.toBeNull()
    const names = expected!.map((field) => field.name)

    expect(diffKeys([...names, '__injected__'], names)).toEqual(['zod 多出：__injected__'])
    expect(diffKeys(names.slice(1), names)).toEqual([`zod 缺少：${names[0]}`])
    expect(diffKeys(names, names)).toEqual([])
  })
})

describe('zod schema ↔ 后端 response_model 契约', () => {
  it('覆盖面非空：15 个模型、53 个字段', () => {
    expect(MODELS).toHaveLength(15)

    let pythonFields = 0
    for (const model of MODELS) {
      const fields = parseModelFields(readSource(model.file), model.className)
      expect(fields, `${model.file} 里找不到 ${model.className}`).not.toBeNull()
      expect(fields!.length, `${model.className} 解析出 0 个字段`).toBeGreaterThan(0)
      pythonFields += fields!.length
    }
    expect(pythonFields).toBe(53)
  })

  for (const model of MODELS) {
    it(`${model.className}：键集与必填性都对齐`, () => {
      const pythonFields = parseModelFields(readSource(model.file), model.className)
      expect(pythonFields).not.toBeNull()

      const expectedKeys = pythonFields!.map((field) => field.name)
      const actualKeys = zodKeys(model.schema)
      expect(actualKeys, `${model.className} 的 schema 不是 object/union`).not.toBeNull()

      // 双向：zod 不多键、不少键
      expect(diffKeys(actualKeys!, expectedKeys)).toEqual([])

      // 必填方向（§5 判据）：后端无默认值的键，zod 必须不允许缺席
      if (model.schema instanceof z.ZodObject) {
        for (const field of pythonFields!) {
          if (field.hasDefault) continue
          expect(
            acceptsMissing(model.schema, field.name),
            `${model.className}.${field.name} 后端恒在，zod 却允许缺席`
          ).toBe(false)
        }
      } else {
        // union 表达不出「某键恒在」（它只要求落在任一分支），所以只有后端全字段
        // 都有默认值时才允许用 union 建模。当前仅 AugmentProgressResponse 走这支。
        expect(
          pythonFields!.filter((field) => !field.hasDefault).map((field) => field.name),
          `${model.className} 有必填字段，不能用 union 建模`
        ).toEqual([])
      }
    })
  }
})
