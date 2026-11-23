// @vitest-environment node
import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import path from 'node:path'

let source = ''
try {
  source = readFileSync(new URL('./QueryTableDesignerApp.vue', import.meta.url), 'utf8')
} catch {
  source = readFileSync(path.resolve(process.cwd(), 'frontend/apps/desktop/src/views/QueryTableDesignerApp.vue'), 'utf8')
}

const AsyncFn = Object.getPrototypeOf(async function () {}).constructor as any
const BT = String.fromCharCode(96)

function stripTs(s: string): string {
  return s
    .replace(/\.(get|post|put|delete|patch)<[^>]+>/g, '.$1')
    .replace(/\s+as\s+any\b/g, '')
    .replace(/:\s*any\b/g, '')
    .replace(/:\s*string\b/g, '')
    .replace(/:\s*boolean\b/g, '')
    .replace(/:\s*number\b/g, '')
    .replace(/:\s*unknown\b/g, '')
}

function bodyOf(sig: string): string {
  const idx = source.indexOf(sig)
  if (idx < 0) throw new Error('sig not found: ' + sig)
  let i = idx + sig.length
  while (i < source.length && source[i] !== '{') i++
  if (i >= source.length) throw new Error('no body: ' + sig)
  const start = i
  let depth = 0
  let quote = ''
  let esc = false
  for (; i < source.length; i++) {
    const c = source[i]
    if (quote) {
      if (esc) { esc = false; continue }
      if (c === '\\') { esc = true; continue }
      if (c === quote) quote = ''
      continue
    }
    if (c === "'" || c === '"' || c === BT) { quote = c; continue }
    if (c === '//' && source[i + 1] === '/') { quote = 'L'; continue }
    if (quote === 'L') { if (source[i] === '\n') quote = ''; continue }
    if (c === '{') depth++
    else if (c === '}') { depth--; if (depth === 0) return source.slice(start, i + 1) }
  }
  throw new Error('unbalanced: ' + sig)
}

function helper(): (r: any) => boolean {
  if (source.indexOf('function queryTableWriteSucceeded') < 0) return (() => false) as any
  return new Function('resp', stripTs(bodyOf('function queryTableWriteSucceeded'))) as any
}

function stubs(resp: any) {
  const success: string[] = []
  const errors: string[] = []
  const writes: string[] = []
  const reload: string[] = []
  const api = {
    put: async (u: string, _b?: any) => { writes.push('PUT ' + u); return resp },
    post: async (u: string, _b?: any) => { writes.push('POST ' + u); return resp },
    get: async (u: string) => { writes.push('GET ' + u); return resp },
  }
  const toast = {
    success: (m: string) => { success.push(String(m)) },
    error: (m: string) => { errors.push(String(m)) },
    info: (_m: string) => { /* noop */ },
  }
  return {
    success, errors, writes, reload, api, toast,
    canSave: { value: true },
    saving: { value: false },
    executing: { value: false },
    form: { value: { name: 'R399-T', queryFlag: 'R399-Q', columns: [] } },
    activeFlag: { value: 'R399-FLAG' },
    executeResult: { value: 'R399-SENTINEL' },
    tables: { value: [] },
    loading: { value: false },
    tablePayload: (_n: string, _q: string, _c: any[]) => ({}),
    designerPaths: {
      tableSave: (f: string) => '/api/query/assemble/designer/table/' + f,
      tableCreate: '/api/query/assemble/designer/table',
      tableExecute: (f: string) => '/api/query/assemble/designer/table/' + f + '/execute',
      tableList: '/api/query/assemble/designer/table/list',
    },
    extractList: (_r: any) => [],
    loadTables: async () => { reload.push('loadTables') },
  }
}

const ARGS = 'canSave, saving, executing, form, activeFlag, executeResult, tables, loading, tablePayload, designerPaths, api, toast, extractList, loadTables, queryTableWriteSucceeded'

async function run(sig: string, resp: any) {
  const s = stubs(resp)
  const fn = new AsyncFn(ARGS, stripTs(bodyOf(sig)))
  await fn(s.canSave, s.saving, s.executing, s.form, s.activeFlag, s.executeResult, s.tables, s.loading, s.tablePayload, s.designerPaths, s.api, s.toast, s.extractList, s.loadTables, helper())
  return s
}

describe('queryTableWriteSucceeded 信封判定', () => {
  it('type:error table not found 判失败', () => {
    expect(helper()({ type: 'error', message: 'table not found' })).toBe(false)
  })
  it('success:false 判失败', () => { expect(helper()({ type: 'success', success: false })).toBe(false) })
  it('type:success + data:{} 判成功', () => { expect(helper()({ type: 'success', data: {} })).toBe(true) })
  it('type:success + data:{tableFlag} 无权威bool 判成功', () => { expect(helper()({ type: 'success', data: { tableFlag: 'F1' } })).toBe(true) })
  it('type缺省且success非false 判成功', () => { expect(helper()({ data: {} })).toBe(true) })
  it('null/undefined/非对象 判失败', () => {
    expect(helper()(null)).toBe(false)
    expect(helper()(undefined)).toBe(false)
    expect(helper()('x')).toBe(false)
  })
})

describe('saveTable 保存定义', () => {
  it('type:error table not found 不弹成功且透传message', async () => {
    const s = await run('async function saveTable(', { type: 'error', message: 'table not found' })
    expect(s.success).toEqual([])
    expect(s.errors.join(' ')).toContain('table not found')
    expect(s.activeFlag.value).toBe('R399-FLAG')
    expect(s.writes.length).toBe(1)
  })
  it('success:false 不弹成功且不回填activeFlag', async () => {
    const s = await run('async function saveTable(', { type: 'success', success: false, message: 'R399拒绝', data: { tableFlag: 'BAD' } })
    expect(s.success).toEqual([])
    expect(s.errors.join(' ')).toContain('R399拒绝')
    expect(s.activeFlag.value).toBe('R399-FLAG')
  })
  it('type:success + data:{tableFlag} 保留成功并回填activeFlag', async () => {
    const s = await run('async function saveTable(', { type: 'success', data: { tableFlag: 'NEWFLAG' } })
    expect(s.success.some((m) => m.indexOf('表定义已保存为草稿') >= 0)).toBe(true)
    expect(s.activeFlag.value).toBe('NEWFLAG')
    expect(s.errors).toEqual([])
  })
})

describe('editTableStructure 编辑结构', () => {
  it('type:error 不弹成功且透传message', async () => {
    const s = await run('async function editTableStructure(', { type: 'error', message: 'table not found' })
    expect(s.success).toEqual([])
    expect(s.errors.join(' ')).toContain('table not found')
  })
  it('success:false 不弹成功', async () => {
    const s = await run('async function editTableStructure(', { type: 'success', success: false, message: 'R399编辑拒绝' })
    expect(s.success).toEqual([])
    expect(s.errors.join(' ')).toContain('R399编辑拒绝')
  })
  it('type:success 保留原成功提示', async () => {
    const s = await run('async function editTableStructure(', { type: 'success', data: {} })
    expect(s.success.some((m) => m.indexOf('表结构编辑已提交') >= 0)).toBe(true)
  })
})

describe('executeTable 执行DDL', () => {
  it('type:error 不弹成功、executeResult不赋值', async () => {
    const s = await run('async function executeTable(', { type: 'error', message: 'table not found' })
    expect(s.success).toEqual([])
    expect(s.errors.join(' ')).toContain('table not found')
    expect(s.executeResult.value).toBe('R399-SENTINEL')
  })
  it('success:false 不弹成功、executeResult不赋值', async () => {
    const s = await run('async function executeTable(', { type: 'success', success: false, message: 'R399执行拒绝' })
    expect(s.success).toEqual([])
    expect(s.errors.join(' ')).toContain('R399执行拒绝')
    expect(s.executeResult.value).toBe('R399-SENTINEL')
  })
  it('type:success 保留成功且executeResult=response.data', async () => {
    const s = await run('async function executeTable(', { type: 'success', data: { ok: 1 } })
    expect(s.success.some((m) => m.indexOf('受限 DDL 已执行') >= 0)).toBe(true)
    expect(s.executeResult.value).toEqual({ ok: 1 })
  })
})
