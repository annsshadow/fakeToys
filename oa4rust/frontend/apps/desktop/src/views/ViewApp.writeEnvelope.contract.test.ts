// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const SRC = new URL('./ViewApp.vue', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1')
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
  try { body = stripTs(bodyOf('viewDeleteWriteSucceeded')) }
  catch { body = 'return false;' }
  return new Function('resp', '"use strict"; ' + body) as (resp: any) => boolean
}

function mkToast(toasts: any[]) {
  return {
    success: (m: string) => toasts.push({ k: 'success', m }),
    error: (m: string) => toasts.push({ k: 'error', m }),
  }
}
function respOf(overrides: any): any {
  return Object.prototype.hasOwnProperty.call(overrides, '__resp') ? overrides.__resp : { type: 'success', success: true, data: { deleted: true } }
}
function hasToast(toasts: any[], k: string): boolean { return toasts.some(t => t.k === k) }
function toastText(toasts: any[], k: string): string { const t = toasts.find(t => t.k === k); return t ? t.m : '' }

interface Ctx {
  api: any; toast: any; confirmMsg: any; loadViews: any; calls: any[]; toasts: any[]; state: any; __helper: (resp: any) => boolean;
}
function makeCtx(overrides: any): Ctx {
  const calls: any[] = [], toasts: any[] = []
  const state = { loadViews: 0 }
  const resp = respOf(overrides)
  const mk = (m: string) => async (...args: any[]) => { calls.push({ m, args }); if (overrides.__throw) throw new Error(overrides.__throw); return resp }
  const api = { delete: mk('delete') }
  const confirmMsg = async () => overrides.__confirm !== false
  const loadViews = () => { state.loadViews++ }
  return { api, toast: mkToast(toasts), confirmMsg, loadViews, calls, toasts, state, __helper: buildHelper() }
}

async function runFn(name: string, paramName: string, paramVal: any, overrides: any): Promise<Ctx> {
  const ctx = makeCtx(overrides)
  const body = stripTs(bodyOf(name))
  const pre = [
    '"use strict";',
    'var api = __ctx.api;', 'var toast = __ctx.toast;',
    'var confirmMsg = __ctx.confirmMsg;', 'var loadViews = __ctx.loadViews;',
    'var viewDeleteWriteSucceeded = __ctx.__helper;',
    'return async function ' + name + '(' + paramName + ') {' + body + '};',
  ].join('\n')
  const fn = new Function('__ctx', pre)(ctx) as (p: any) => Promise<void>
  await fn(paramVal)
  return ctx
}

describe('viewDeleteWriteSucceeded helper', () => {
  it('type:error 判失败', () => { expect(buildHelper()({ type: 'error', message: 'view not found' })).toBe(false) })
  it('success:false 判失败', () => { expect(buildHelper()({ success: false, message: 'denied' })).toBe(false) })
  it('type:success 判成功', () => { expect(buildHelper()({ type: 'success', data: { deleted: true } })).toBe(true) })
  it('type缺省 success 非 false 判成功', () => { expect(buildHelper()({ success: true })).toBe(true); expect(buildHelper()({ data: { ok: 1 } })).toBe(true) })
  it('未知 type 判失败', () => { expect(buildHelper()({ type: 'weird', success: true })).toBe(false) })
  it('非对象 null/undefined 判失败', () => { expect(buildHelper()(null)).toBe(false); expect(buildHelper()(undefined)).toBe(false); expect(buildHelper()('ok')).toBe(false) })
  it('不强求 named bool，deleted:0 仍成功', () => { expect(buildHelper()({ type: 'success', data: { deleted: 0 } })).toBe(true) })
})

describe('removeView 写链守卫', () => {
  it('type:error → error透传、不弹成功、不刷新', async () => { const c = await runFn('removeView','v',{ id: 'R418-FAKE-VIEW' },{ __resp: { type: 'error', message: 'view not found' } }); expect(toastText(c.toasts,'error')).toContain('view not found'); expect(hasToast(c.toasts,'success')).toBe(false); expect(c.state.loadViews).toBe(0) })
  it('success:false → error透传、不弹成功、不刷新', async () => { const c = await runFn('removeView','v',{ id: 'R418-FAKE-VIEW' },{ __resp: { success: false, message: 'R418 denied' } }); expect(toastText(c.toasts,'error')).toContain('R418 denied'); expect(hasToast(c.toasts,'success')).toBe(false); expect(c.state.loadViews).toBe(0) })
  it('非对象响应 → 失败不弹成功不刷新', async () => { const c = await runFn('removeView','v',{ id: 'R418-FAKE-VIEW' },{ __resp: undefined }); expect(hasToast(c.toasts,'error')).toBe(true); expect(hasToast(c.toasts,'success')).toBe(false); expect(c.state.loadViews).toBe(0) })
  it('type:success → toast 已删除 + loadViews 一次', async () => { const c = await runFn('removeView','v',{ id: 'R418-FAKE-VIEW' },{ __resp: { type: 'success', data: { deleted: true } } }); expect(toastText(c.toasts,'success')).toContain('已删除'); expect(hasToast(c.toasts,'error')).toBe(false); expect(c.state.loadViews).toBe(1) })
  it('type 缺省 → 成功', async () => { const c = await runFn('removeView','v',{ id: 'R418-FAKE-VIEW' },{}); expect(toastText(c.toasts,'success')).toContain('已删除'); expect(c.state.loadViews).toBe(1) })
  it('confirm取消 → 零写零toast零刷新', async () => { const c = await runFn('removeView','v',{ id: 'R418-FAKE-VIEW' },{ __confirm: false }); expect(c.calls.length).toBe(0); expect(c.toasts.length).toBe(0); expect(c.state.loadViews).toBe(0) })
  it('端点保持 /api/view/{id}', async () => { const c = await runFn('removeView','v',{ id: 'R418-FAKE-VIEW' },{}); expect(c.calls[0].m).toBe('delete'); expect(c.calls[0].args[0]).toContain('/api/view/R418-FAKE-VIEW') })
  it('非2xx 500 走 catch toast.error', async () => { const c = await runFn('removeView','v',{ id: 'R418-FAKE-VIEW' },{ __throw: 'HTTP 500: Internal Server Error' }); expect(toastText(c.toasts,'error')).toContain('HTTP 500'); expect(hasToast(c.toasts,'success')).toBe(false); expect(c.state.loadViews).toBe(0) })
  it('403 无权限走 catch toast.error', async () => { const c = await runFn('removeView','v',{ id: 'R418-FAKE-VIEW' },{ __throw: '没有权限执行该操作' }); expect(toastText(c.toasts,'error')).toContain('没有权限'); expect(hasToast(c.toasts,'success')).toBe(false) })
})