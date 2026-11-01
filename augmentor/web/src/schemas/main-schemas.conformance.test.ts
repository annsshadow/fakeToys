/**
 * zod schema ↔ 后端 pydantic `response_model` 的跨语言契约守卫
 *
 * ## 为什么是「静态解析」而不是「打真接口」
 *
 * `services/api.ts` 的 zod 校验解决的是「运行时形状不对就抛」，但**schema 自己写错**
 * 时它抓不到——schema 与后端一致地错、或者把必填写成可选，`safeParse` 照样绿。
 * 本文件的判据是：把 `augmentor/api` 下路由文件的响应模型字段名与可选性**直接解析出来**，
 * 和 zod 的 key 集逐键比对。这样 zod 的 ground truth 不靠人复述，而是后端源码本身。
 *
 * 本机没有 augmentor 的 Python 运行时（`fastapi`/`pydantic` 均不可用，见计划 §8），
 * 所以不走 `TestClient` 生成真实响应——那一路的运行时证据由后端的 pytest 套件提供，
 * 本文件只守「两侧声明是否同形」。
 *
 * ## Z-FULL（10-27 计划 §3）：覆盖面从 15 个模型扩到全部 85 个
 *
 * `full-schemas.ts` 每个 schema 上方那行注释就是它与后端类的映射
 * （`文件:行号 → 类名`），本文件**直接读那行注释**建表，而不是手抄 70 条
 * ——手抄的映射会在后端搬类时静默失效，而注释就在被守卫的文件里。
 *
 * 同时补上三类此前解析不到的响应模型：
 * - **继承**：`ConfigUpdateResponse(MessageResponse)` / `VersionDetailResponse(VersionInfo)`
 *   ⇒ 基类字段先合并，再比键集；
 * - **dataclass**：`AuditReport`（`augmentor/audit.py`）不是 pydantic 类，
 *   但 FastAPI 支持 dataclass 作 `response_model`，字段形状同一套解析；
 * - **路由覆盖**：扫 `response_model=` 的类名，要求每个都有 schema（防「新端点没接线」）。
 *
 * ## 防恒真的三道锁
 *
 * 1. **非空元断言**：模型数、总键数、每个模型的键数都断言 ≥ 1；解析器找不到类时
 *    返回 `null` 而不是空数组（空数组会让「⊆」永远成立）。
 * 2. **字面量锚点**：`VersionInfo` 的键集写成字面量数组，解析器若退化成空转立刻红。
 * 3. **反向注入**：给键集人为多塞/少塞一个键，断言比对函数真的能报出差异。
 */
import { readFileSync, readdirSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

import { describe, expect, it } from 'vitest'
import { z } from 'zod'

import * as fullSchemas from './full-schemas'
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

/** `augmentor/`：本文件在 `augmentor/web/src/schemas/`，上三级就是项目根 */
const AUGMENTOR_DIR = resolve(dirname(fileURLToPath(import.meta.url)), '../../..')
/** `augmentor/api`：路由与 schemas.py 的根 */
const API_DIR = resolve(AUGMENTOR_DIR, 'api')

/** pydantic / dataclass 模型里的一个字段 */
interface ModelField {
  name: string
  /** 有 `= 默认值`（含 `= None`）⇒ 键可能缺席；无默认值 ⇒ 响应里恒在 */
  hasDefault: boolean
}

/** 一个类的解析结果（含继承合并后的字段） */
interface ParsedModel {
  fields: ModelField[]
  bases: string[]
}

/**
 * 从 Python 源码里解析 `class <name>(...)` 的字段
 *
 * 只处理本仓响应模型的实际写法：顶层 `class X(BaseModel):`、类体 4 空格缩进、
 * 每字段一行注解、docstring 用 `"""`。解析不到类时返回 `null`——**不返回空数组**，
 * 空数组会让后续的集合包含判断恒真。
 */
function parseOwnFields(source: string, className: string): ParsedModel | null {
  const lines = source.split(/\r?\n/)
  // 两种类头都要认：pydantic 的 `class X(BaseModel):` 与 dataclass 的 `class X:`
  const start = lines.findIndex((line) =>
    new RegExp(`^class\\s+${className}\\s*(\\(|:)`).test(line),
  )
  if (start === -1) return null

  const header = /^class\s+\w+\s*(?:\(([^)]*)\))?\s*:/.exec(lines[start])
  const bases = header?.[1]
    ? header[1]
        .split(',')
        .map((base) => base.trim())
        .filter((base) => base.length > 0)
    : []

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

  return { fields, bases }
}

/**
 * `{类名: {file, source}}` —— 解析继承时要跨文件找基类
 *
 * 只扫 `augmentor/api/**` 与 Python 包内的 `augmentor/augmentor/audit.py`（dataclass
 * 响应模型）。同一类名出现两次是案面出了歧义，直接抛错而不是让后一个静默覆盖。
 */
function pythonClassIndex(): Map<string, { file: string; source: string }> {
  const index = new Map<string, { file: string; source: string }>()
  const files = readdirSync(API_DIR, { recursive: true, encoding: 'utf8' })
    .filter((name) => name.endsWith('.py'))
    .map((name) => resolve(API_DIR, name))
  // dataclass 响应模型在 Python 包内（`augmentor/augmentor/audit.py`），路径按注释写法解析
  files.push(resolve(AUGMENTOR_DIR, 'augmentor/audit.py'))

  for (const path of files) {
    const source = readFileSync(path, 'utf8')
    for (const match of source.matchAll(/^class\s+(\w+)\s*(?:\(|:)/gm)) {
      const name = match[1]
      if (index.has(name)) {
        throw new Error(`类名 ${name} 在 ${path} 与 ${index.get(name)!.file} 重复`)
      }
      index.set(name, { file: path, source })
    }
  }
  return index
}

const CLASS_INDEX = pythonClassIndex()

/** 递归解析字段：基类字段在前、自身字段在后（与 pydantic 的序列化顺序一致） */
function parseModelFields(className: string): ModelField[] | null {
  const resolveClass = (name: string, seen: Set<string>): ModelField[] | null => {
    if (seen.has(name)) return null // 循环继承是坏案面：判 null，让上面的断言响
    seen.add(name)
    const entry = CLASS_INDEX.get(name)
    if (!entry) return null
    const parsed = parseOwnFields(entry.source, name)
    if (!parsed) return null
    const inherited: ModelField[] = []
    for (const base of parsed.bases) {
      if (base === 'BaseModel' || base === 'object') continue
      const baseFields = resolveClass(base, seen)
      if (baseFields === null) return null
      inherited.push(...baseFields)
    }
    return [...inherited, ...parsed.fields]
  }
  return resolveClass(className, new Set())
}

/** 某个类定义所在文件里，`class <name>(` 的行号（注释里的映射必须与它一致） */
function classLine(source: string, className: string): number | null {
  const lines = source.split(/\r?\n/)
  const idx = lines.findIndex((line) => new RegExp(`^class\\s+${className}\\s*(\\(|:)`).test(line))
  return idx === -1 ? null : idx + 1
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
  /** 相对 `augmentor/` 的路径 */
  file: string
  /** 注释里声明的行号（仅生成的 full-schemas 有） */
  line?: number
}

/** 主链路 15 个（Z 批次）——schema 变量名与类名不同名，必须显式列 */
const MODELS: ModelCase[] = [
  { schema: dataFileInfoSchema, className: 'DataFileInfo', file: 'api/routes/data.py' },
  {
    schema: dataFileListResponseSchema,
    className: 'DataFileListResponse',
    file: 'api/routes/data.py',
  },
  { schema: dataPageResponseSchema, className: 'DataLoadResponse', file: 'api/routes/data.py' },
  { schema: uploadResponseSchema, className: 'DataUploadResponse', file: 'api/routes/data.py' },
  {
    schema: analyzeResponseSchema,
    className: 'DatasetAnalysisResponse',
    file: 'api/routes/data.py',
  },
  { schema: visualizeResponseSchema, className: 'VisualizeResponse', file: 'api/routes/data.py' },
  { schema: startAugmentResponseSchema, className: 'MessageResponse', file: 'api/schemas.py' },
  {
    schema: checkpointListResponseSchema,
    className: 'CheckpointListResponse',
    file: 'api/routes/augment.py',
  },
  {
    schema: augmentProgressSchema,
    className: 'AugmentProgressResponse',
    file: 'api/routes/augment.py',
  },
  { schema: versionInfoSchema, className: 'VersionInfo', file: 'api/routes/version.py' },
  {
    schema: versionListResponseSchema,
    className: 'VersionListResponse',
    file: 'api/routes/version.py',
  },
  {
    schema: createVersionResponseSchema,
    className: 'VersionCreateResponse',
    file: 'api/routes/version.py',
  },
  {
    schema: versionDataResponseSchema,
    className: 'VersionDataResponse',
    file: 'api/routes/version.py',
  },
  {
    schema: qualityEvaluationResponseSchema,
    className: 'QualityEvaluateResponse',
    file: 'api/routes/quality.py',
  },
  { schema: dedupResponseSchema, className: 'DedupResponse', file: 'api/routes/quality.py' },
]

const FULL_SCHEMA_MODULE = fullSchemas as unknown as Record<string, z.ZodType>

/**
 * `full-schemas.ts` 里「`文件:行号` → 类名 + export const 变量名」的映射
 *
 * 正则贴着生成器的输出形状写；生成器改了注释格式，这里会解析出 0 条，
 * 下面的「覆盖面非空」断言当场红——不会退化成「什么都没比也全绿」。
 */
const FULL_MODELS: ModelCase[] = [
  ...readFileSync(resolve(AUGMENTOR_DIR, 'web/src/schemas/full-schemas.ts'), 'utf8').matchAll(
    /\/\*\* `([^`]+):(\d+)` → (\w+) \*\/\r?\nexport const (\w+) = z\.object\(/g,
  ),
].map((match) => {
  const [, file, line, className, schemaName] = match
  const schema = FULL_SCHEMA_MODULE[schemaName]
  if (!schema) throw new Error(`full-schemas.ts 里找不到导出 ${schemaName}`)
  return { schema, className, file, line: Number(line) }
})

/** 源文件只读一次 */
const sourceCache = new Map<string, string>()
function readSource(relativePath: string): string {
  const cached = sourceCache.get(relativePath)
  if (cached !== undefined) return cached
  const text = readFileSync(resolve(AUGMENTOR_DIR, relativePath), 'utf8')
  sourceCache.set(relativePath, text)
  return text
}

describe('schema 解析器自证（防恒真）', () => {
  it('能从 version.py 读出 VersionInfo 的完整字段（字面量锚点）', () => {
    const fields = parseModelFields('VersionInfo')

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
    expect(parseModelFields('NoSuchModel')).toBeNull()
  })

  it('继承会先合并基类字段（ConfigUpdateResponse / VersionDetailResponse）', () => {
    expect(parseModelFields('ConfigUpdateResponse')?.map((field) => field.name)).toEqual([
      'success',
      'message',
      'ignored_keys',
    ])
    expect(parseModelFields('VersionDetailResponse')?.map((field) => field.name)).toEqual([
      'version_id',
      'label',
      'description',
      'created_at',
      'item_count',
      'metadata',
    ])
  })

  it('dataclass 响应模型也能解析（AuditReport）', () => {
    const fields = parseModelFields('AuditReport')
    expect(fields?.map((field) => field.name)).toEqual([
      'total_items',
      'pii_matches',
      'duplicate_count',
      'duplicate_rate',
      'empty_field_rate',
      'leak_count',
      'leak_rate',
      'findings',
      'ready',
    ])
  })

  it('反向注入：键集多一个 / 少一个都必须被报出来', () => {
    const expected = parseModelFields('VersionInfo')
    expect(expected).not.toBeNull()
    const names = expected!.map((field) => field.name)

    expect(diffKeys([...names, '__injected__'], names)).toEqual(['zod 多出：__injected__'])
    expect(diffKeys(names.slice(1), names)).toEqual([`zod 缺少：${names[0]}`])
    expect(diffKeys(names, names)).toEqual([])
  })
})

const ALL_MODELS = [...MODELS, ...FULL_MODELS]

describe('zod schema ↔ 后端 response_model 契约', () => {
  it('覆盖面非空：85 个模型、全部字段都可解析', () => {
    expect(MODELS).toHaveLength(15)
    expect(FULL_MODELS).toHaveLength(70)
    expect(ALL_MODELS).toHaveLength(85)

    let pythonFields = 0
    for (const model of ALL_MODELS) {
      const fields = parseModelFields(model.className)
      expect(fields, `${model.file} 里找不到 ${model.className}`).not.toBeNull()
      expect(fields!.length, `${model.className} 解析出 0 个字段`).toBeGreaterThan(0)
      pythonFields += fields!.length
    }
    expect(pythonFields).toBe(389)
  })

  it('full-schemas 的「文件:行号」映射与类定义现量一致', () => {
    for (const model of FULL_MODELS) {
      const actual = classLine(readSource(model.file), model.className)
      expect(actual, `${model.file} 里找不到 ${model.className}`).not.toBeNull()
      expect(actual, `${model.className} 注释行号与现量不符`).toBe(model.line)
    }
  })

  it('每个路由的 response_model 都有 schema（防新端点漏接线）', () => {
    const routeFiles = readdirSync(API_DIR, { recursive: true, encoding: 'utf8' })
      .filter((name) => name.endsWith('.py'))
      .map((name) => resolve(API_DIR, name))
    routeFiles.push(resolve(API_DIR, 'main.py'))

    // `response_model=List[X]` 这类 typing 容器不是模型，单独排除；其余类名必须是 schema
    const TYPING = new Set(['List', 'Optional', 'Dict', 'Union', 'Any'])
    const declared = new Set<string>()
    for (const path of routeFiles) {
      const source = readFileSync(path, 'utf8')
      for (const match of source.matchAll(/response_model\s*=\s*(\w+)/g)) {
        if (!TYPING.has(match[1])) declared.add(match[1])
      }
    }

    const covered = new Set(ALL_MODELS.map((model) => model.className))
    const missing = [...declared].filter((name) => !covered.has(name)).sort()
    expect(declared.size).toBeGreaterThan(70)
    expect(missing).toEqual([])
  })

  for (const model of ALL_MODELS) {
    it(`${model.className}：键集与必填性都对齐`, () => {
      const pythonFields = parseModelFields(model.className)
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
            `${model.className}.${field.name} 后端恒在，zod 却允许缺席`,
          ).toBe(false)
        }
      } else {
        // union 表达不出「某键恒在」（它只要求落在任一分支），所以只有后端全字段
        // 都有默认值时才允许用 union 建模。当前仅 AugmentProgressResponse 走这支。
        expect(
          pythonFields!.filter((field) => !field.hasDefault).map((field) => field.name),
          `${model.className} 有必填字段，不能用 union 建模`,
        ).toEqual([])
      }
    })
  }
})
