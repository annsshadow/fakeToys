// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R384 契约：PC RecycleApp.vue 五个真实写链在 HTTP200 + type:error、
 * 权威 resumed/deleted:false、批量 allSettled fulfilled 假成功时，
 * 必须错误提示、不得删行/清空/误报成功。
 * type 判定为主：type:success（或 type 缺省且 success 非 false）为成功前提；
 * 传 flagKey 时额外要求 data[flagKey] 或顶层 [flagKey] 严格 === true。
 * 源码级函数体真执行（Function/AsyncFunction 注入 ref/api/toast/confirmMsg），非文本断言。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, './RecycleApp.vue'), 'utf8')

function bodyOf(source: string, signature: string): string {
  const start = source.indexOf(signature)
  if (start < 0) throw new Error('未找到函数: ' + signature)
  const open = source.indexOf('{', start)
  if (open < 0) throw new Error('未找到函数体: ' + signature)
  let depth = 0, quote: string | null = null, esc = false
  for (let i = open; i < source.length; i++) {
    const c = source[i]
    if (quote) {
      if (esc) { esc = false; continue }
      if (c === '\\') { esc = true; continue }
      if (c === quote) quote = null
      continue
    }
    if (c === "'" || c === '"' || c === '\`') { quote = c; continue }
    if (c === '{') depth++
    else if (c === '}') { depth--; if (depth === 0) return source.slice(open + 1, i) }
  }
  throw new Error('花括号未配对: ' + signature)
}
function safeBody(signature: string): string | null {
  try { return bodyOf(src, signature) } catch { return null }
}
function stripTs(body: string): string {
  return body
    .replace(/\(([A-Za-z_$][\w$]*): ?[A-Za-z_$][\w$<>|\[\]]*\)/g, '($1)')
    .replace(/catch \(([A-Za-z_$][\w$]*): ?[A-Za-z_$][\w$<>|\[\]]*\)/g, 'catch ($1)')
    .replace(/\b(const|let) ([A-Za-z_$][\w$]*): ?[A-Za-z_$][\w$<>|\[\]]*/g, '$1 $2')
    .replace(/\s+as\s+[A-Za-z_$][\w$<>|\[\]]*/g, '')
}

const AsyncFn = Object.getPrototypeOf(async function () {}).constructor
type Succeeded = (resp: unknown, flagKey?: string) => boolean
const helperBody = safeBody('function recycleWriteSucceeded(')
const succeeded: Succeeded = helperBody
  ? (new Function('resp', 'flagKey', stripTs(helperBody!)) as Succeeded)
  : (() => false) as Succeeded

function ref<T>(v: T): { value: T } { return { value: v } }
interface Toasts { success: string[]; error: string[] }
interface Opts {
  resp?: unknown
  resps?: unknown[]
  reject?: Error
  confirmed?: boolean
  busy?: boolean
  selected?: string[]
}

function compile(sig: string, names: string[]): (...a: unknown[]) => Promise<void> {
  const body = safeBody(sig)
  expect(body, sig + ' 函数体应可提取').not.toBeNull()
  return new AsyncFn(...names, stripTs(body!)) as (...a: unknown[]) => Promise<void>
}

function makeCtx(opts: Opts) {
  const toasts: Toasts = { success: [], error: [] }
  const toast = {
    success: (msg: string): void => { toasts.success.push(msg) },
    error: (msg: string): void => { toasts.error.push(msg) },
  }
  let postCalls = 0, deleteCalls = 0, confirmCalls = 0, respondIdx = 0
  const response = (): unknown => {
    if (opts.reject) throw opts.reject
    if (opts.resps) return opts.resps[respondIdx++]
    return opts.resp
  }
  const api = {
    post: async (): Promise<unknown> => { postCalls++; return response() },
    delete: async (): Promise<unknown> => { deleteCalls++; return response() },
  }
  const confirmMsg = async (): Promise<boolean> => { confirmCalls++; return opts.confirmed !== false }
  const a = { id: 'R384-A', name: '项目A', deleted: true }
  const b = { id: 'R384-B', name: '项目B', deleted: true }
  const c = { id: 'R384-C', name: '项目C', deleted: true }
  const allItems = ref<any[]>([a, b, c])
  const selectedIds = ref<string[]>(opts.selected ? [...opts.selected] : [])
  const busy = ref<boolean>(opts.busy ?? false)
  const inject = ['recycleWriteSucceeded', 'ref', 'api', 'toast', 'confirmMsg', 'allItems', 'selectedIds', 'busy', 'item']
  const args = [succeeded, ref, api, toast, confirmMsg, allItems, selectedIds, busy, a]
  const run = (sig: string): Promise<void> => (compile(sig, inject) as (...a: unknown[]) => Promise<void>)(...args)
  return {
    toasts, allItems, selectedIds, busy,
    runResume: () => run('async function resume('),
    runPermanentDelete: () => run('async function permanentDelete('),
    runBatchResume: () => run('async function batchResume('),
    runBatchDelete: () => run('async function batchDelete('),
    runEmptyRecycle: () => run('async function emptyRecycle('),
    counts: (): Record<string, number> => ({ postCalls, deleteCalls, confirmCalls }),
  }
}

describe('R384 recycleWriteSucceeded 真执行（type 为主，flagKey 可选）', () => {
  it('type:success 无 bool 判成功，flagKey 未传不强求 bool', () => {
    expect(succeeded({ type: 'success', data: {} })).toBe(true)
    expect(succeeded({ type: 'success', data: { deleted: false } })).toBe(true)
  })
  it('type 缺省且 success 非 false 判成功（legacy）', () => {
    expect(succeeded({ success: true, data: {} })).toBe(true)
    expect(succeeded({ data: {} })).toBe(true)
  })
  it('type:error / success:false 判失败', () => {
    expect(succeeded({ type: 'error', message: 'x' })).toBe(false)
    expect(succeeded({ type: 'success', success: false })).toBe(false)
  })
  it('flagKey 传入时权威 bool 必须严格 true', () => {
    expect(succeeded({ type: 'success', data: { resumed: true } }, 'resumed')).toBe(true)
    expect(succeeded({ type: 'success', resumed: true }, 'resumed')).toBe(true)
    expect(succeeded({ type: 'success', data: { resumed: false } }, 'resumed')).toBe(false)
    expect(succeeded({ type: 'success', data: {} }, 'resumed')).toBe(false)
    expect(succeeded({ type: 'success', data: { deleted: 1 } }, 'deleted')).toBe(false)
  })
  it('undefined/null/非对象判失败', () => {
    expect(succeeded(undefined)).toBe(false)
    expect(succeeded(null)).toBe(false)
    expect(succeeded('x')).toBe(false)
  })
})

describe('R384 单项目写链失败保留行', () => {
  it('resume type:error：不透传成功、保留行', async () => {
    const ctx = makeCtx({ resp: { type: 'error', message: 'R384 resume rejected' } })
    await ctx.runResume()
    expect(ctx.toasts.success.length).toBe(0)
    expect(ctx.toasts.error.join('|')).toContain('R384 resume rejected')
    expect(ctx.allItems.value.length).toBe(3)
    expect(ctx.selectedIds.value.length).toBe(0)
  })
  it('resume resumed:false：权威 bool 失败保留行', async () => {
    const ctx = makeCtx({ resp: { type: 'success', data: { resumed: false }, message: '未恢复' } })
    await ctx.runResume()
    expect(ctx.toasts.success.length).toBe(0)
    expect(ctx.toasts.error.join('|')).toContain('未恢复')
    expect(ctx.allItems.value.length).toBe(3)
  })
  it('permanentDelete type:error：透传错误、不删行', async () => {
    const ctx = makeCtx({ resp: { type: 'error', message: 'R384 delete rejected' } })
    await ctx.runPermanentDelete()
    expect(ctx.toasts.error.join('|')).toContain('R384 delete rejected')
    expect(ctx.allItems.value.length).toBe(3)
  })
  it('permanentDelete deleted:false：保留行', async () => {
    const ctx = makeCtx({ resp: { type: 'success', data: { deleted: false }, message: '未删除' } })
    await ctx.runPermanentDelete()
    expect(ctx.toasts.success.length).toBe(0)
    expect(ctx.toasts.error.join('|')).toContain('未删除')
    expect(ctx.allItems.value.length).toBe(3)
  })
  it('permanentDelete confirm 取消：零 DELETE、零行变动', async () => {
    const ctx = makeCtx({ confirmed: false, resp: { type: 'success', data: { deleted: true } } })
    await ctx.runPermanentDelete()
    expect(ctx.counts().deleteCalls).toBe(0)
    expect(ctx.allItems.value.length).toBe(3)
  })
  it('resume 真成功（resumed:true）删行', async () => {
    const ctx = makeCtx({ resp: { type: 'success', data: { resumed: true } } })
    await ctx.runResume()
    expect(ctx.toasts.error.length).toBe(0)
    expect(ctx.allItems.value.length).toBe(2)
    expect(ctx.allItems.value.some((x: any) => x.id === 'R384-A')).toBe(false)
  })
  it('permanentDelete 真成功（deleted:true）删行', async () => {
    const ctx = makeCtx({ resp: { type: 'success', data: { deleted: true } } })
    await ctx.runPermanentDelete()
    expect(ctx.toasts.error.length).toBe(0)
    expect(ctx.allItems.value.length).toBe(2)
  })
  it('非2xx resume：catch 报错保留行', async () => {
    const ctx = makeCtx({ reject: new Error('HTTP 500') })
    await ctx.runResume()
    expect(ctx.toasts.success.length).toBe(0)
    expect(ctx.toasts.error.join('|')).toContain('HTTP 500')
    expect(ctx.allItems.value.length).toBe(3)
  })
})

describe('R384 批量写链按 helper 判真', () => {
  it('batchResume 全 type:error：ok=0、失败保留行、零成功提示', async () => {
    const ctx = makeCtx({ selected: ['R384-A', 'R384-B'], resps: [
      { type: 'error', message: 'E-A' }, { type: 'error', message: 'E-B' },
    ] })
    await ctx.runBatchResume()
    expect(ctx.toasts.success.length).toBe(0)
    expect(ctx.toasts.error.join('|')).toContain('E-A')
    expect(ctx.allItems.value.length).toBe(3)
    expect(ctx.selectedIds.value.length).toBe(2)
  })
  it('batchResume 混合成功/失败：仅成功项删行，ok 按真实成功数', async () => {
    const ctx = makeCtx({ selected: ['R384-A', 'R384-B'], resps: [
      { type: 'success', data: { resumed: true } }, { type: 'error', message: 'E-B' },
    ] })
    await ctx.runBatchResume()
    expect(ctx.toasts.success.join('|')).toContain('已恢复 1 个项目')
    expect(ctx.toasts.success.join('|')).toContain('1 个失败')
    expect(ctx.allItems.value.length).toBe(2)
    expect(ctx.allItems.value.some((x: any) => x.id === 'R384-A')).toBe(false)
    expect(ctx.allItems.value.some((x: any) => x.id === 'R384-B')).toBe(true)
    expect(ctx.selectedIds.value).toEqual(['R384-B'])
  })
  it('batchDelete 全 type:error：ok=0、失败保留行', async () => {
    const ctx = makeCtx({ selected: ['R384-A', 'R384-B'], resps: [
      { type: 'error', message: 'E-A' }, { type: 'error', message: 'E-B' },
    ] })
    await ctx.runBatchDelete()
    expect(ctx.toasts.success.length).toBe(0)
    expect(ctx.allItems.value.length).toBe(3)
    expect(ctx.selectedIds.value.length).toBe(2)
  })
  it('batchDelete 混合成功/失败：仅成功项删行，ok 按真实成功数', async () => {
    const ctx = makeCtx({ selected: ['R384-A', 'R384-B'], resps: [
      { type: 'success', data: { deleted: true } }, { type: 'error', message: 'E-B' },
    ] })
    await ctx.runBatchDelete()
    expect(ctx.toasts.success.join('|')).toContain('已永久删除 1 个项目')
    expect(ctx.toasts.success.join('|')).toContain('1 个失败')
    expect(ctx.allItems.value.length).toBe(2)
    expect(ctx.allItems.value.some((x: any) => x.id === 'R384-A')).toBe(false)
    expect(ctx.allItems.value.some((x: any) => x.id === 'R384-B')).toBe(true)
  })
  it('batchResume confirm 取消：零 POST、零行变动', async () => {
    const ctx = makeCtx({ confirmed: false, selected: ['R384-A'], resp: { type: 'success', data: { resumed: true } } })
    await ctx.runBatchResume()
    expect(ctx.counts().postCalls).toBe(0)
    expect(ctx.allItems.value.length).toBe(3)
  })
  it('batch 真成功全部：计数与删行保持原语义', async () => {
    const ctx = makeCtx({ selected: ['R384-A', 'R384-B'], resps: [
      { type: 'success', data: { resumed: true } }, { type: 'success', data: { resumed: true } },
    ] })
    await ctx.runBatchResume()
    expect(ctx.toasts.success.join('|')).toContain('已恢复 2 个项目')
    expect(ctx.toasts.error.length).toBe(0)
    expect(ctx.allItems.value.length).toBe(1)
    expect(ctx.selectedIds.value.length).toBe(0)
  })
})

describe('R384 emptyRecycle 失败不清空', () => {
  it('type:error：透传 message、保留行', async () => {
    const ctx = makeCtx({ resp: { type: 'error', message: 'R384 empty rejected' } })
    await ctx.runEmptyRecycle()
    expect(ctx.toasts.error.join('|')).toContain('R384 empty rejected')
    expect(ctx.allItems.value.length).toBe(3)
  })
  it('confirm 取消：零 DELETE、保留行', async () => {
    const ctx = makeCtx({ confirmed: false, resp: { type: 'success', data: { value: true } } })
    await ctx.runEmptyRecycle()
    expect(ctx.counts().deleteCalls).toBe(0)
    expect(ctx.allItems.value.length).toBe(3)
  })
  it('type:success 真成功：清空全部', async () => {
    const ctx = makeCtx({ resp: { type: 'success', data: { value: true } } })
    await ctx.runEmptyRecycle()
    expect(ctx.toasts.error.length).toBe(0)
    expect(ctx.allItems.value.length).toBe(0)
    expect(ctx.selectedIds.value.length).toBe(0)
  })
})
