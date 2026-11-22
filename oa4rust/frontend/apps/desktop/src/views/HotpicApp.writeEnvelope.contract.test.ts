// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R382 契约：PC HotpicApp.vue 三个真实写链在 HTTP200 + type:error，或 200 + type:success
 * 但权威 deleted:false 时，必须错误提示、不得误报成功、不得删行/刷新。
 * type 判定为主：type:success（或 type 缺省且 success 非 false）为成功前提；仅调用方传 flagKey
 * 时附加校验 data[flagKey] === true（userDelete2/onDelete 删除端点有权威 deleted bool）。
 * 源码级函数体真执行（Function/AsyncFunction 注入 api.post/delete/prompt/toast/confirmMsg），非文本断言。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, './HotpicApp.vue'), 'utf8')

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
const helperBody = safeBody('function hotpicWriteSucceeded(')
const succeeded: Succeeded = helperBody
  ? (new Function('resp', 'flagKey', stripTs(helperBody!)) as Succeeded)
  : (() => false) as Succeeded

function ref<T>(v: T): { value: T } { return { value: v } }
interface Toasts { success: string[]; error: string[] }
interface Opts {
  op?: string
  resp?: unknown
  reject?: Error
  prompts?: string[]
  confirmed?: boolean
}

function compile(sig: string, names: string[], isAsync: boolean): (...a: unknown[]) => unknown {
  const body = safeBody(sig)
  expect(body, sig + ' 函数体应可提取').not.toBeNull()
  const plain = stripTs(body!)
  if (isAsync) return new AsyncFn(...names, plain) as (...a: unknown[]) => Promise<void>
  return new Function(...names, plain) as (...a: unknown[]) => void
}

function makeCtx(opts: Opts) {
  const toasts: Toasts = { success: [], error: [] }
  const toast = {
    success: (msg: string): void => { toasts.success.push(msg) },
    error: (msg: string): void => { toasts.error.push(msg) },
  }
  let postCalls = 0, deleteCalls = 0, confirmCalls = 0, promptCalls = 0
  const promptQueue = [...(opts.prompts ?? ['R382输入'])]
  const prompt = (): string => { promptCalls++; return promptQueue.shift() ?? 'R382输入' }
  const confirmMsg = async (): Promise<boolean> => { confirmCalls++; return opts.confirmed !== false }
  const respond = async (): Promise<unknown> => { if (opts.reject) throw opts.reject; return opts.resp }
  const api = {
    post: async (): Promise<unknown> => { postCalls++; return respond() },
    delete: async (): Promise<unknown> => { deleteCalls++; return respond() },
  }
  const item = { id: 'R382-ITEM-1', title: 'R382热帖' }
  const other = { id: 'R382-ITEM-2', title: '其它' }
  const items = ref<any[]>([item, other])
  const names = ['op', 'prompt', 'confirmMsg', 'api', 'toast', 'item', 'items', 'hotpicWriteSucceeded']
  const args = [opts.op ?? 'changeTitle', prompt, confirmMsg, api, toast, item, items, succeeded]
  return {
    toasts, items,
    runWrite: async (): Promise<void> => { await (compile('async function hotpicWrite(', names.slice(0, 5).concat('hotpicWriteSucceeded'), true) as (...a: unknown[]) => Promise<void>)(opts.op ?? 'changeTitle', prompt, confirmMsg, api, toast, succeeded) },
    runMore: async (): Promise<void> => { await (compile('async function hotpicMore(', names.slice(0, 5).concat('hotpicWriteSucceeded'), true) as (...a: unknown[]) => Promise<void>)(opts.op ?? 'userDelete2', prompt, confirmMsg, api, toast, succeeded) },
    runDelete: async (): Promise<void> => { await (compile('async function onDelete(', ['item', 'confirmMsg', 'api', 'toast', 'items', 'hotpicWriteSucceeded'], true) as (...a: unknown[]) => Promise<void>)(item, confirmMsg, api, toast, items, succeeded) },
    counts: (): Record<string, number> => ({ postCalls, deleteCalls, confirmCalls, promptCalls }),
  }
}

describe('R382 hotpicWriteSucceeded 真执行（type 为主，flagKey 可选）', () => {
  it('type:success 无 bool 判成功（flagKey 未传不强求 bool）', () => {
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
    expect(succeeded({ type: 'success', data: { deleted: true } }, 'deleted')).toBe(true)
    expect(succeeded({ type: 'success', data: { deleted: false } }, 'deleted')).toBe(false)
    expect(succeeded({ type: 'success', data: {} }, 'deleted')).toBe(false)
    expect(succeeded({ type: 'success', data: { deleted: 1 } }, 'deleted')).toBe(false)
  })
  it('undefined/null/非对象判失败', () => {
    expect(succeeded(undefined)).toBe(false)
    expect(succeeded(null)).toBe(false)
    expect(succeeded('x')).toBe(false)
  })
})

describe('R382 hotpicWrite 写链失败信封修复', () => {
  it('type:error：透传 message、不弹成功', async () => {
    const c = makeCtx({ op: 'changeTitle', resp: { type: 'error', message: 'R382 backend error' } })
    await c.runWrite()
    expect(c.toasts.success.length).toBe(0)
    expect(c.toasts.error.join('|')).toContain('R382 backend error')
  })
  it('success:false：不弹成功', async () => {
    const c = makeCtx({ op: 'create', resp: { type: 'success', success: false, message: '未生效' } })
    await c.runWrite()
    expect(c.toasts.success.length).toBe(0)
    expect(c.toasts.error.join('|')).toContain('未生效')
  })
  it('type:success 无 bool：成功提示保留', async () => {
    const c = makeCtx({ op: 'changeTitle', resp: { type: 'success', data: {} } })
    await c.runWrite()
    expect(c.toasts.success.join('|')).toContain('热图操作已提交')
    expect(c.toasts.error.length).toBe(0)
  })
  it('非2xx：catch 正确报错不误报成功', async () => {
    const c = makeCtx({ op: 'create', reject: new Error('HTTP 500') })
    await c.runWrite()
    expect(c.toasts.success.length).toBe(0)
    expect(c.toasts.error.join('|')).toContain('HTTP 500')
  })
})

describe('R382 hotpicMore userDelete2 写链修复', () => {
  it('type:error：不弹成功、透传 message', async () => {
    const c = makeCtx({ op: 'userDelete2', prompts: ['R382-ID', 'R382-SUB'], resp: { type: 'error', message: 'R382 delete rejected' } })
    await c.runMore()
    expect(c.toasts.success.length).toBe(0)
    expect(c.toasts.error.join('|')).toContain('R382 delete rejected')
  })
  it('deleted:false：权威 bool 失败，不弹成功', async () => {
    const c = makeCtx({ op: 'userDelete2', prompts: ['R382-ID', 'R382-SUB'], resp: { type: 'success', data: { deleted: false }, message: '未删除' } })
    await c.runMore()
    expect(c.toasts.success.length).toBe(0)
    expect(c.toasts.error.join('|')).toContain('未删除')
  })
  it('confirm 取消：零 DELETE、零 toast', async () => {
    const c = makeCtx({ op: 'userDelete2', prompts: ['R382-ID', 'R382-SUB'], confirmed: false, resp: { type: 'success' } })
    await c.runMore()
    expect(c.counts().deleteCalls).toBe(0)
    expect(c.toasts.success.length + c.toasts.error.length).toBe(0)
  })
  it('deleted:true：成功提示保留', async () => {
    const c = makeCtx({ op: 'userDelete2', prompts: ['R382-ID', 'R382-SUB'], resp: { type: 'success', data: { deleted: true } } })
    await c.runMore()
    expect(c.toasts.success.join('|')).toContain('热图操作已提交')
  })
})

describe('R382 onDelete 写链失败不删行', () => {
  it('type:error：不透传成功、保留行', async () => {
    const c = makeCtx({ resp: { type: 'error', message: 'R382 not found' } })
    await c.runDelete()
    expect(c.toasts.success.length).toBe(0)
    expect(c.items.value.length).toBe(2)
    expect(c.items.value[0].id).toBe('R382-ITEM-1')
  })
  it('core delete 成功信封无 deleted：按 type 判成功删行', async () => {
    const c = makeCtx({ resp: { type: 'success', data: { success: true } } })
    await c.runDelete()
    expect(c.toasts.success.length).toBe(0)
    expect(c.toasts.error.length).toBe(0)
    expect(c.items.value.length).toBe(1)
  })
  it('confirm 取消：零 DELETE、零行变动', async () => {
    const c = makeCtx({ confirmed: false, resp: { type: 'success' } })
    await c.runDelete()
    expect(c.counts().deleteCalls).toBe(0)
    expect(c.items.value.length).toBe(2)
  })
  it('非2xx：catch 正确报错不删行', async () => {
    const c = makeCtx({ reject: new Error('HTTP 403') })
    await c.runDelete()
    expect(c.toasts.success.length).toBe(0)
    expect(c.toasts.error.join('|')).toContain('HTTP 403')
    expect(c.items.value.length).toBe(2)
  })
  it('core delete success:true 成功删行保留', async () => {
    const c = makeCtx({ resp: { type: 'success', data: { success: true } } })
    await c.runDelete()
    expect(c.items.value.length).toBe(1)
    expect(c.items.value[0].id).toBe('R382-ITEM-2')
  })
})
