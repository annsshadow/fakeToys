// @vitest-environment node
import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import path from 'node:path'

let source = ''
try {
  source = readFileSync(new URL('./AppInfoApp.vue', import.meta.url), 'utf8')
} catch {
  source = readFileSync(path.resolve(process.cwd(), 'frontend/apps/desktop/src/views/AppInfoApp.vue'), 'utf8')
}

const AsyncFn = Object.getPrototypeOf(async function () {}).constructor as any
const BT = String.fromCharCode(96)

function stripTs(s: string): string {
  return s
    .replace(/\s+as\s+any\b/g, '')
    .replace(/:\s*any\b/g, '')
    .replace(/:\s*string\b/g, '')
    .replace(/:\s*boolean\b/g, '')
    .replace(/:\s*number\b/g, '')
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
    if (c === '{') depth++
    else if (c === '}') { depth--; if (depth === 0) return source.slice(start, i + 1) }
  }
  throw new Error('unbalanced: ' + sig)
}

function helper(): (r: any) => boolean {
  if (source.indexOf('function appinfoWriteSucceeded') < 0) return (() => false) as any
  return new Function('resp', stripTs(bodyOf('function appinfoWriteSucceeded'))) as any
}

function stubs(resp: any, confirm = true) {
  const success: string[] = []
  const errors: string[] = []
  const writes: string[] = []
  const refresh: string[] = []
  const api = {
    post: async (u: string, _b?: any) => { writes.push('POST ' + u); return resp },
    get: async (u: string) => { writes.push('GET ' + u); return resp },
    delete: async (u: string) => { writes.push('DELETE ' + u); return resp },
  }
  const toast = {
    success: (m: string) => { success.push(String(m)) },
    error: (m: string) => { errors.push(String(m)) },
    info: (m: string) => { /* noop */ },
  }
  const confirmMsg = async (_m: string) => confirm
  const prompt = (_m: string, _d?: string) => 'R397-VIEWER'
  const refreshFn = () => { refresh.push('refresh') }
  return { success, errors, writes, refresh, api, toast, confirmMsg, prompt, refreshFn }
}

async function runWritePerms(resp: any, confirm = true) {
  const s = stubs(resp, confirm)
  const fn = new AsyncFn('item', 'api', 'toast', 'confirmMsg', 'prompt', 'refresh', 'appinfoWriteSucceeded', stripTs(bodyOf('async function writePerms(')))
  await fn({ id: 'R397-APP' }, s.api, s.toast, s.confirmMsg, s.prompt, s.refreshFn, helper())
  return s
}

describe('appinfoWriteSucceeded 信封判定', () => {
  it('type:error application not found 判失败', () => {
    expect(helper()({ type: 'error', message: 'application not found' })).toBe(false)
  })
  it('success:false 判失败', () => { expect(helper()({ type: 'success', success: false })).toBe(false) })
  it('type:success 判成功', () => { expect(helper()({ type: 'success', data: {} })).toBe(true) })
  it('type:success + data:{} 无权威bool 判成功', () => { expect(helper()({ type: 'success', data: { granted: 0 } })).toBe(true) })
  it('type缺省且success非false 判成功', () => { expect(helper()({ data: {} })).toBe(true) })
  it('null/undefined/非对象 判失败', () => {
    expect(helper()(null)).toBe(false)
    expect(helper()(undefined)).toBe(false)
    expect(helper()('x')).toBe(false)
  })
})

describe('writePerms 设权限写链', () => {
  it('type:error application not found 不弹成功且透传message', async () => {
    const s = await runWritePerms({ type: 'error', message: 'application not found' })
    expect(s.success).toEqual([])
    expect(s.errors.join(' ')).toContain('application not found')
    expect(s.writes.length).toBe(1)
    expect(s.refresh).toEqual([])
  })
  it('success:false 不弹成功且透传message', async () => {
    const s = await runWritePerms({ type: 'success', success: false, message: 'R397拒绝:success false' })
    expect(s.success).toEqual([])
    expect(s.errors.join(' ')).toContain('R397拒绝:success false')
  })
  it('type:success + data:{} 保留原成功提示', async () => {
    const s = await runWritePerms({ type: 'success', data: {} })
    expect(s.success.some((m) => m.indexOf('应用权限已写入') >= 0)).toBe(true)
    expect(s.errors).toEqual([])
  })
  it('type:success + granted 计数 仍成功(不强求bool)', async () => {
    const s = await runWritePerms({ type: 'success', data: { granted: 2 } })
    expect(s.success.some((m) => m.indexOf('应用权限已写入') >= 0)).toBe(true)
  })
  it('confirm取消零写不弹成功', async () => {
    const s = await runWritePerms({ type: 'success' }, false)
    expect(s.writes.length).toBe(0)
    expect(s.success).toEqual([])
  })
})
