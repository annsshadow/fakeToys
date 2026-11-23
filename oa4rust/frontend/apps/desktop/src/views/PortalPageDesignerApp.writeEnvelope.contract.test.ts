// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const SRC = new URL('./PortalPageDesignerApp.vue', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1')
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
  try { body = stripTs(bodyOf('portalLayoutWriteSucceeded')) }
  catch { body = 'return false;' }
  return new Function('resp', '"use strict"; ' + body) as (resp: any) => boolean
}

interface Ctx {
  api: any; toast: any; prompt: any; confirmMsg: any;
  calls: any[]; toasts: any[]; __helper: (resp: any) => boolean;
}

function makeCtx(overrides: any): Ctx {
  const calls: any[] = []
  const toasts: any[] = []
  const hasResp = Object.prototype.hasOwnProperty.call(overrides, '__resp')
  const resp = hasResp ? overrides.__resp : { type: 'success', success: true }
  const api = {
    post: async (...args: any[]) => { calls.push({ m: 'post', args }); if (overrides.__throw) throw new Error(overrides.__throw); return resp },
    delete: async (...args: any[]) => { calls.push({ m: 'delete', args }); if (overrides.__throw) throw new Error(overrides.__throw); return resp },
    put: async (...args: any[]) => { calls.push({ m: 'put', args }); if (overrides.__throw) throw new Error(overrides.__throw); return resp },
    get: async (...args: any[]) => { calls.push({ m: 'get', args }); return resp },
  }
  const toast = {
    success: (m: string) => toasts.push({ k: 'success', m }),
    error: (m: string) => toasts.push({ k: 'error', m }),
  }
  const prompt = () => (Object.prototype.hasOwnProperty.call(overrides, '__id') ? overrides.__id : 'R408-FAKE-LAYOUT')
  const confirmMsg = async () => overrides.__confirmed !== false
  return { api, toast, prompt, confirmMsg, calls, toasts, __helper: buildHelper() }
}

async function runLayoutOp(op: string, overrides: any): Promise<Ctx> {
  const ctx = makeCtx(overrides)
  const body = stripTs(bodyOf('layoutOp'))
  const pre = [
    '"use strict";',
    'var api = __ctx.api;',
    'var toast = __ctx.toast;',
    'var prompt = __ctx.prompt;',
    'var confirmMsg = __ctx.confirmMsg;',
    'var portalLayoutWriteSucceeded = __ctx.__helper;',
    'return async function layoutOp(op) {' + body + '};',
  ].join('\n')
  const fn = new Function('__ctx', pre)(ctx) as (op: string) => Promise<void>
  await fn(op)
  return ctx
}

function hasToast(ctx: Ctx, k: 'success' | 'error'): boolean {
  return ctx.toasts.some(t => t.k === k)
}
function toastText(ctx: Ctx, k: 'success' | 'error'): string {
  const t = ctx.toasts.find(t => t.k === k)
  return t ? t.m : ''
}

describe('portalLayoutWriteSucceeded helper', () => {
  it('type:error 判失败', () => {
    expect(buildHelper()({ type: 'error', message: 'layout not found' })).toBe(false)
  })
  it('success:false 判失败', () => {
    expect(buildHelper()({ success: false, message: 'R408 denied', data: {} })).toBe(false)
  })
  it('type:success 判成功', () => {
    expect(buildHelper()({ type: 'success', success: true, data: { deleted: 0 } })).toBe(true)
  })
  it('type 缺省且 success 非 false 判成功', () => {
    expect(buildHelper()({ success: true, data: { deleted: 1 } })).toBe(true)
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
  it('不强求权威 bool，deleted:0 仍成功', () => {
    expect(buildHelper()({ type: 'success', data: { deleted: 0 } })).toBe(true)
  })
})

describe('layoutOp 删除分支写链守卫', () => {
  it('deletePost type:error layout not found → 红toast透传不弹成功', async () => {
    const ctx = await runLayoutOp('deletePost', { __resp: { type: 'error', message: 'layout not found' } })
    expect(hasToast(ctx, 'error')).toBe(true)
    expect(toastText(ctx, 'error')).toContain('layout not found')
    expect(hasToast(ctx, 'success')).toBe(false)
  })
  it('deleteDel type:error → 红toast透传不弹成功', async () => {
    const ctx = await runLayoutOp('deleteDel', { __resp: { type: 'error', message: 'layout not found' } })
    expect(hasToast(ctx, 'error')).toBe(true)
    expect(toastText(ctx, 'error')).toContain('layout not found')
    expect(hasToast(ctx, 'success')).toBe(false)
  })
  it('success:false → 红toast透传后端 message', async () => {
    const ctx = await runLayoutOp('deletePost', { __resp: { success: false, message: 'R408 denied' } })
    expect(toastText(ctx, 'error')).toContain('R408 denied')
    expect(hasToast(ctx, 'success')).toBe(false)
  })
  it('非对象响应 → 失败不弹成功', async () => {
    const ctx = await runLayoutOp('deletePost', { __resp: undefined })
    expect(hasToast(ctx, 'error')).toBe(true)
    expect(hasToast(ctx, 'success')).toBe(false)
  })
  it('type:success 真正成功仍绿toast', async () => {
    const ctx = await runLayoutOp('deletePost', { __resp: { type: 'success', success: true, data: { deleted: 1 } } })
    expect(hasToast(ctx, 'success')).toBe(true)
    expect(hasToast(ctx, 'error')).toBe(false)
  })
  it('type 缺省 success 非 false 仍成功', async () => {
    const ctx = await runLayoutOp('deleteDel', { __resp: { success: true, data: { deleted: 1 } } })
    expect(hasToast(ctx, 'success')).toBe(true)
    expect(hasToast(ctx, 'error')).toBe(false)
  })
  it('prompt 空ID零写零toast', async () => {
    const ctx = await runLayoutOp('deletePost', { __id: '' })
    expect(ctx.calls.length).toBe(0)
    expect(ctx.toasts.length).toBe(0)
  })
  it('confirm 取消零写零toast', async () => {
    const ctx = await runLayoutOp('deletePost', { __confirmed: false })
    expect(ctx.calls.length).toBe(0)
    expect(ctx.toasts.length).toBe(0)
  })
  it('非2xx抛错走 catch toast.error', async () => {
    const ctx = await runLayoutOp('deletePost', { __throw: 'HTTP 500: Internal Server Error' })
    expect(hasToast(ctx, 'error')).toBe(true)
    expect(toastText(ctx, 'error')).toContain('HTTP 500')
    expect(hasToast(ctx, 'success')).toBe(false)
  })
  it('deleteDel 端点与 body.id 保持不变', async () => {
    const ctx = await runLayoutOp('deleteDel', { __id: 'R408-FAKE-LAYOUT', __resp: { type: 'success' } })
    const del = ctx.calls.find(c => c.m === 'delete')
    expect(del).toBeDefined()
    expect(del.args[0]).toBe('/api/portal/assemble/surface/delete/layout')
    expect(del.args[1].body.id).toBe('R408-FAKE-LAYOUT')
  })
})

describe('保存分支本轮不改（仅L1记录）', () => {
  it('savePost 保持原成功提示路径不变', async () => {
    const ctx = await runLayoutOp('savePost', { __resp: { type: 'error', message: 'x' } })
    expect(hasToast(ctx, 'success')).toBe(true)
  })
})