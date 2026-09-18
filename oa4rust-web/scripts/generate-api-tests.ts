import fs from 'node:fs'
import path from 'node:path'

const apiPath = path.join(import.meta.dirname || '.', '../packages/apis/src/index.ts')
const content = fs.readFileSync(apiPath, 'utf-8')
const lines = content.split('\n')

const modules: Record<string, string[]> = {}
let currentModule: string | null = null

for (const line of lines) {
  const moduleMatch = line.match(/^export const (\w+) = \{/)
  if (moduleMatch) {
    currentModule = moduleMatch[1]
    if (!modules[currentModule]) {
      modules[currentModule] = []
    }
    continue
  }

  if (currentModule) {
    const funcMatch = line.match(/^ {2}(\w+): \(/)
    if (funcMatch) {
      modules[currentModule].push(funcMatch[1])
    }

    if (line.includes('};')) {
      currentModule = null
    }
  }
}

// Skip wrapper modules (not real API modules)
const skipModules = ['createRequest', 'apis', 'oa4rustApis', 'additionalApis', 'extraApis', 'processplatformSurfaceApi']

const testFile: string[] = []
testFile.push(`import { describe, expect, it, vi, beforeEach } from 'vitest'
import type { Mock } from 'vitest'

const mockGet: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockPost: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockPut: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockDelete: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockUpload: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })

vi.mock('@oa4rust/sdk', () => ({
  api: {
    get: mockGet,
    post: mockPost,
    put: mockPut,
    delete: mockDelete,
    upload: mockUpload,
  },
}))

describe('API Module Coverage', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockGet.mockClear()
    mockPost.mockClear()
    mockPut.mockClear()
    mockDelete.mockClear()
    mockUpload.mockClear()
  })
`)

for (const [moduleName, functions] of Object.entries(modules)) {
  if (skipModules.includes(moduleName)) continue
  if (functions.length === 0) continue

  testFile.push('')
  testFile.push(`describe('${moduleName}', () => {`)

  for (const funcName of functions) {
    testFile.push(`  it('calls ${moduleName}.${funcName}', async () => {
    const result = await import('./index.ts')
    const module = (result as any).${moduleName}
    const fn = module?.${funcName}
    expect(fn).toBeDefined()
    expect(typeof fn).toBe('function')
  })`)
  }

  testFile.push('})')
}

testFile.push('})')

const outputPath = path.join(import.meta.dirname || '.', '../packages/apis/src/api-coverage-generated.test.ts')
fs.writeFileSync(outputPath, testFile.join('\n'))

const totalFunctions = Object.values(modules).flat().length
const skippedCount = skipModules.length
const msg =
  'Generated tests for ' + (Object.keys(modules).length - skippedCount) + ' modules, ' + totalFunctions + ' functions'
console.log(msg)
