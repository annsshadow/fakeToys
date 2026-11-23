// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const SRC = new URL('./ProcessDesigner.vue', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1')
const src = readFileSync(SRC, 'utf8')

function bodyOf(name: string): string {
  const idx = src.indexOf('function ' + name + '(')
  if (idx < 0) throw new Error('function not found: ' + name)
  const open = src.indexOf('{', idx)
  let depth = 0
  for (let i = open; i < src.length; i++) {
    const ch = src[i]
    if (ch === '{') depth++
    else if (ch === '}') { depth--; if (depth === 0) return src.slice(open + 1, i) }
  }
  throw new Error('unbalanced body: ' + name)
}

function stripTs(s: string): string {
  return s
    .replace(/\b([A-Za-z_$][\w$]*)\s*:\s*any\b/g, '$1')
    .replace(/\.([a-z]+)<[^>]*>\(/g, '.$1(')
    .replace(/\bcatch\s*\(\s*(\w+)\s*:\s*\w+\s*\)/g, 'catch ($1)')
    .replace(/\bconst\s+(\w+)\s*:\s*[^=]+=/g, 'const $1 =')
    .replace(/\blet\s+(\w+)\s*:\s*[^=;]+/g, 'let $1')
}

function buildHelper(): (resp: any) => boolean {
  let body: string
  try { body = stripTs(bodyOf('processDesignerWriteSucceeded')) }
  catch { body = 'return false;' }
  return new Function('resp', '"use strict"; ' + body) as (resp: any) => boolean
}

interface Ctx {
  api: any; toast: any; currentProcess: any; currentO2Definition: any;
  calls: any[]; toasts: any[]; __helper: (resp: any) => boolean;
}

function makeCtx(overrides: any): Ctx {
  const calls: any[] = []
  const toasts: any[] = []
  const hasResp = Object.prototype.hasOwnProperty.call(overrides, '__resp')
  const resp = hasResp ? overrides.__resp : { type: 'success', success: true, data: { id: 'R410-FAKE', saved: true, updatedAt: 'x' } }
  const api = {
    post: async (...args: any[]) => { calls.push({ m: 'post', args }); if (overrides.__throw) throw new Error(overrides.__throw); return resp },
  }
  const toast = {
    info: (m: string) => toasts.push({ k: 'info', m }),
    success: (m: string) => toasts.push({ k: 'success', m }),
    error: (m: string) => toasts.push({ k: 'error', m }),
  }
  const initialDef = { nodes: [] }
  const pid = Object.prototype.hasOwnProperty.call(overrides, '__id') ? overrides.__id : 'R410-FAKE'
  const currentProcess = { value: { id: pid, definition: initialDef } }
  const currentO2Definition = (p: any) => ({ nodes: ['from-' + (p?.id || 'x')] })
  return { api, toast, currentProcess, currentO2Definition, calls, toasts, __helper: buildHelper() }
}

async function runSaveProcess(overrides: any): Promise<Ctx> {
  const ctx = makeCtx(overrides)
  const body = stripTs(bodyOf('saveProcess'))
  const pre = [
    '"use strict";',
    'var api = __ctx.api;',
    'var toast = __ctx.toast;',
    'var currentProcess = __ctx.currentProcess;',
    'var currentO2Definition = __ctx.currentO2Definition;',
    'var processDesignerWriteSucceeded = __ctx.__helper;',
    'return async function saveProcess() {' + body + '};',
  ].join('\n')
  const fn = new Function('__ctx', pre)(ctx) as () => Promise<void>
  await fn()
  return ctx
}

function hasToast(ctx: Ctx, k: 'info' | 'success' | 'error'): boolean { return ctx.toasts.some(t => t.k === k) }
function toastText(ctx: Ctx, k: 'info' | 'success' | 'error'): string { const t = ctx.toasts.find(t => t.k === k); return t ? t.m : '' }
function defWritten(ctx: Ctx): boolean { return (ctx.currentProcess.value.definition?.nodes || []).length > 0 }

describe('processDesignerWriteSucceeded helper', () => {
  it('type:error 判失败', () => {
    expect(buildHelper()({ type: 'error', message: 'process definition not found' })).toBe(false)
  })
  it('success:false 判失败', () => {
    expect(buildHelper()({ success: false, message: 'R410 denied', data: {} })).toBe(false)
  })
  it('type:success 判成功', () => {
    expect(buildHelper()({ type: 'success', success: true, data: { id: 'x', saved: true, updatedAt: 't' } })).toBe(true)
  })
  it('type 缺省且 success 非 false 判成功', () => {
    expect(buildHelper()({ success: true, data: { saved: 1 } })).toBe(true)
    expect(buildHelper()({ data: { ok: 1 } })).toBe(true)
  })
  it('未知 type 判失败', () => {
    expect(buildHelper()({ type: 'weird', success: true })).toBe(false)
  })
  it('非对象 null/undefined 判失败', () => {
    expect(buildHelper()(null)).toBe(false)
    expect(buildHelper()(undefined)).toBe(false)
    expect(buildHelper()('ok')).toBe(false)
  })
  it('不强求权威 named bool，saved:0 仍成功', () => {
    expect(buildHelper()({ type: 'success', data: { saved: 0 } })).toBe(true)
  })
})

describe('saveProcess 写链守卫', () => {
  it('type:error process definition not found → error透传不写回不提示成功', async () => {
    const ctx = await runSaveProcess({ __resp: { type: 'error', message: 'process definition not found' } })
    expect(hasToast(ctx, 'error')).toBe(true)
    expect(toastText(ctx, 'error')).toContain('process definition not found')
    expect(hasToast(ctx, 'info')).toBe(false)
    expect(defWritten(ctx)).toBe(false)
  })
  it('success:false → error透传后端 message 不写回', async () => {
    const ctx = await runSaveProcess({ __resp: { success: false, message: 'R410 denied' } })
    expect(toastText(ctx, 'error')).toContain('R410 denied')
    expect(hasToast(ctx, 'info')).toBe(false)
    expect(defWritten(ctx)).toBe(false)
  })
  it('非对象响应 → 失败不提示成功不写回', async () => {
    const ctx = await runSaveProcess({ __resp: undefined })
    expect(hasToast(ctx, 'error')).toBe(true)
    expect(hasToast(ctx, 'info')).toBe(false)
    expect(defWritten(ctx)).toBe(false)
  })
  it('type:success 真正成功 → 写回 definition + info 保存成功', async () => {
    const ctx = await runSaveProcess({ __resp: { type: 'success', success: true, data: { id: 'x', saved: true } } })
    expect(hasToast(ctx, 'info')).toBe(true)
    expect(toastText(ctx, 'info')).toContain('保存成功')
    expect(hasToast(ctx, 'error')).toBe(false)
    expect(defWritten(ctx)).toBe(true)
  })
  it('type 缺省 success 非 false → 成功写回', async () => {
    const ctx = await runSaveProcess({ __resp: { success: true, data: { saved: 1 } } })
    expect(hasToast(ctx, 'info')).toBe(true)
    expect(defWritten(ctx)).toBe(true)
  })
  it('id 缺失零写零toast', async () => {
    const ctx = await runSaveProcess({ __id: '' })
    expect(ctx.calls.length).toBe(0)
    expect(ctx.toasts.length).toBe(0)
  })
  it('非2xx 500 抛错走 catch toast.error', async () => {
    const ctx = await runSaveProcess({ __throw: 'HTTP 500: Internal Server Error' })
    expect(hasToast(ctx, 'error')).toBe(true)
    expect(toastText(ctx, 'error')).toContain('HTTP 500')
    expect(hasToast(ctx, 'info')).toBe(false)
  })
  it('非2xx 403 抛错走 catch toast.error', async () => {
    const ctx = await runSaveProcess({ __throw: '没有权限执行该操作' })
    expect(toastText(ctx, 'error')).toContain('没有权限')
    expect(hasToast(ctx, 'info')).toBe(false)
  })
  it('端点与 payload 保持不变', async () => {
    const ctx = await runSaveProcess({ __resp: { type: 'success' } })
    const post = ctx.calls.find(c => c.m === 'post')
    expect(post).toBeDefined()
    expect(post.args[0]).toContain('/api/processplatform/assemble/designer/save/R410-FAKE')
    expect(post.args[1].processDefinition).toBeDefined()
  })
  it('不强求 named bool：saved:0 成功信封仍写回', async () => {
    const ctx = await runSaveProcess({ __resp: { type: 'success', data: { id: 'x', saved: 0 } } })
    expect(hasToast(ctx, 'info')).toBe(true)
    expect(defWritten(ctx)).toBe(true)
  })
})
