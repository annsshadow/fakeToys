// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const SRC = new URL('./PortalApp.vue', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1')
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
  try { body = stripTs(bodyOf('portalAppWriteSucceeded')) }
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
  return Object.prototype.hasOwnProperty.call(overrides, '__resp') ? overrides.__resp : { type: 'success', success: true, data: { id: 'R415-FAKE', updated: true, deleted: true } }
}
function hasToast(toasts: any[], k: string): boolean { return toasts.some(t => t.k === k) }
function toastText(toasts: any[], k: string): string { const t = toasts.find(t => t.k === k); return t ? t.m : '' }

interface Ctx {
  api: any; toast: any; prompt: any; confirmMsg: any; calls: any[]; toasts: any[]; __helper: (resp: any) => boolean;
}
function makeCtx(overrides: any): Ctx {
  const calls: any[] = [], toasts: any[] = []
  const resp = respOf(overrides)
  const mk = (m: string) => async (...args: any[]) => { calls.push({ m, args }); if (overrides.__throw) throw new Error(overrides.__throw); return resp }
  const api = { put: mk('put'), post: mk('post'), delete: mk('delete'), get: mk('get') }
  const prompt = () => overrides.__id !== undefined ? overrides.__id : 'R415-FAKE'
  const confirmMsg = async () => overrides.__confirm !== false
  return { api, toast: mkToast(toasts), prompt, confirmMsg, calls, toasts, __helper: buildHelper() }
}

async function runFn(name: string, paramName: string, paramVal: string, overrides: any): Promise<Ctx> {
  const ctx = makeCtx(overrides)
  const body = stripTs(bodyOf(name))
  const pre = [
    '"use strict";',
    'var api = __ctx.api;', 'var toast = __ctx.toast;',
    'var prompt = __ctx.prompt;', 'var confirmMsg = __ctx.confirmMsg;',
    'var portalAppWriteSucceeded = __ctx.__helper;',
    'return async function ' + name + '(' + paramName + ') {' + body + '};',
  ].join('\n')
  const fn = new Function('__ctx', pre)(ctx) as (p: string) => Promise<void>
  await fn(paramVal)
  return ctx
}

describe('portalAppWriteSucceeded helper', () => {
  it('type:error 判失败', () => { expect(buildHelper()({ type: 'error', message: 'portal not found' })).toBe(false) })
  it('success:false 判失败', () => { expect(buildHelper()({ success: false, message: 'R415 denied' })).toBe(false) })
  it('type:success 判成功', () => { expect(buildHelper()({ type: 'success', data: { id: 'x', updated: true } })).toBe(true) })
  it('type 缺省 success 非 false 判成功', () => { expect(buildHelper()({ success: true, data: { deleted: 1 } })).toBe(true); expect(buildHelper()({ data: { ok: 1 } })).toBe(true) })
  it('未知 type 判失败', () => { expect(buildHelper()({ type: 'weird', success: true })).toBe(false) })
  it('非对象 null/undefined 判失败', () => { expect(buildHelper()(null)).toBe(false); expect(buildHelper()(undefined)).toBe(false); expect(buildHelper()('ok')).toBe(false) })
  it('不强求 named bool，updated:0 仍成功', () => { expect(buildHelper()({ type: 'success', data: { updated: 0 } })).toBe(true) })
})

describe('designerUpdate 写链守卫', () => {
  it('type:error → error透传不弹成功', async () => { const c = await runFn('designerUpdate','kind','portal',{ __resp: { type: 'error', message: 'portal not found' } }); expect(toastText(c.toasts,'error')).toContain('portal not found'); expect(hasToast(c.toasts,'success')).toBe(false) })
  it('success:false → error透传不弹成功', async () => { const c = await runFn('designerUpdate','kind','portal',{ __resp: { success: false, message: 'R415 denied' } }); expect(toastText(c.toasts,'error')).toContain('R415 denied'); expect(hasToast(c.toasts,'success')).toBe(false) })
  it('非对象响应 → 失败不弹成功', async () => { const c = await runFn('designerUpdate','kind','portal',{ __resp: undefined }); expect(hasToast(c.toasts,'error')).toBe(true); expect(hasToast(c.toasts,'success')).toBe(false) })
  it('type:success → toast portal 已更新', async () => { const c = await runFn('designerUpdate','kind','portal',{ __resp: { type: 'success', data: { updated: true } } }); expect(toastText(c.toasts,'success')).toContain('portal 已更新'); expect(hasToast(c.toasts,'error')).toBe(false) })
  it('端点保持 + 空id零写', async () => { const c = await runFn('designerUpdate','kind','portal',{ __resp: { type: 'success' } }); expect(c.calls[0].args[0]).toContain('/api/portal/assemble/designer/portal/'); const z = await runFn('designerUpdate','kind','portal',{ __id: '' }); expect(z.calls.length).toBe(0) })
  it('非2xx 500 走 catch toast.error', async () => { const c = await runFn('designerUpdate','kind','portal',{ __throw: 'HTTP 500: Internal Server Error' }); expect(toastText(c.toasts,'error')).toContain('HTTP 500'); expect(hasToast(c.toasts,'success')).toBe(false) })
})

describe('designerDelete 写链守卫', () => {
  it('type:error → error透传不弹成功', async () => { const c = await runFn('designerDelete','kind','portal',{ __resp: { type: 'error', message: 'portal not found' } }); expect(toastText(c.toasts,'error')).toContain('portal not found'); expect(hasToast(c.toasts,'success')).toBe(false) })
  it('success:false → error透传不弹成功', async () => { const c = await runFn('designerDelete','kind','portal',{ __resp: { success: false, message: 'R415 denied' } }); expect(toastText(c.toasts,'error')).toContain('R415 denied'); expect(hasToast(c.toasts,'success')).toBe(false) })
  it('type:success → toast portal 已删除', async () => { const c = await runFn('designerDelete','kind','portal',{ __resp: { type: 'success', data: { deleted: true } } }); expect(toastText(c.toasts,'success')).toContain('portal 已删除'); expect(hasToast(c.toasts,'error')).toBe(false) })
  it('confirm取消 → 零写零toast', async () => { const c = await runFn('designerDelete','kind','portal',{ __confirm: false }); expect(c.calls.length).toBe(0); expect(c.toasts.length).toBe(0) })
  it('端点保持', async () => { const c = await runFn('designerDelete','kind','portal',{ __resp: { type: 'success' } }); expect(c.calls[0].args[0]).toContain('/api/portal/assemble/designer/portal/') })
})

describe('designerMisc 写链守卫', () => {
  it('type:error → error透传不弹成功', async () => { const c = await runFn('designerMisc','op','inputCompare',{ __resp: { type: 'error', message: 'input not found' } }); expect(toastText(c.toasts,'error')).toContain('input not found'); expect(hasToast(c.toasts,'success')).toBe(false) })
  it('success:false → error透传不弹成功', async () => { const c = await runFn('designerMisc','op','inputCompare',{ __resp: { success: false, message: 'R415 denied' } }); expect(toastText(c.toasts,'error')).toContain('R415 denied'); expect(hasToast(c.toasts,'success')).toBe(false) })
  it('type:success → toast 门户设计器操作已提交', async () => { const c = await runFn('designerMisc','op','inputCompare',{ __resp: { type: 'success' } }); expect(toastText(c.toasts,'success')).toContain('门户设计器操作已提交'); expect(hasToast(c.toasts,'error')).toBe(false) })
  it('inputCompare 端点保持', async () => { const c = await runFn('designerMisc','op','inputCompare',{ __resp: { type: 'success' } }); expect(c.calls[0].args[0]).toContain('/api/portal/assemble/designer/input/compare') })
  it('非2xx 500 走 catch toast.error', async () => { const c = await runFn('designerMisc','op','inputCompare',{ __throw: 'HTTP 500: Internal Server Error' }); expect(toastText(c.toasts,'error')).toContain('HTTP 500'); expect(hasToast(c.toasts,'success')).toBe(false) })
})
