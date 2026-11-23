// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const SRC = new URL('./PortalDesigner.vue', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1')
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

function onChangeBody(): string {
  const marker = 'input.onchange = async () => {'
  const idx = src.indexOf(marker)
  if (idx < 0) throw new Error('onchange not found')
  const open = idx + marker.length - 1
  let depth = 0
  for (let i = open; i < src.length; i++) {
    const ch = src[i]
    if (ch === '{') depth++
    else if (ch === '}') { depth--; if (depth === 0) return src.slice(open + 1, i) }
  }
  throw new Error('unbalanced onchange')
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
  try { body = stripTs(bodyOf('portalDesignerWriteSucceeded')) }
  catch { body = 'return false;' }
  return new Function('resp', '"use strict"; ' + body) as (resp: any) => boolean
}

function mkToast(toasts: any[]) {
  return {
    success: (m: string) => toasts.push({ k: 'success', m }),
    error: (m: string) => toasts.push({ k: 'error', m }),
    info: (m: string) => toasts.push({ k: 'info', m }),
  }
}
function respOf(overrides: any): any {
  return Object.prototype.hasOwnProperty.call(overrides, '__resp') ? overrides.__resp : { type: 'success', success: true, data: { id: 'R413-FAKE', saved: true, content: 'x' } }
}
function hasToast(toasts: any[], k: string): boolean { return toasts.some(t => t.k === k) }
function toastText(toasts: any[], k: string): string { const t = toasts.find(t => t.k === k); return t ? t.m : '' }

async function runSaveDesign(overrides: any) {
  const calls: any[] = [], toasts: any[] = []
  const resp = respOf(overrides)
  const api = { put: async (...args: any[]) => { calls.push({ m: 'put', args }); if (overrides.__throw) throw new Error(overrides.__throw); return resp } }
  const activeId = { value: Object.prototype.hasOwnProperty.call(overrides, '__id') ? overrides.__id : 'R413-FAKE' }
  const saving = { value: false }
  const widgets = { value: [{ id: 'w1' }] }
  const serializePortalContent = (w: any) => ({ widgets: w })
  const designerPaths = { portalSave: (id: string) => '/api/portal/assemble/designer/save/' + id }
  const ctx = { api, toast: mkToast(toasts), activeId, saving, widgets, serializePortalContent, designerPaths, __helper: buildHelper() }
  const body = stripTs(bodyOf('saveDesign'))
  const pre = [
    '"use strict";',
    'var api = __ctx.api;', 'var toast = __ctx.toast;',
    'var activeId = __ctx.activeId;', 'var saving = __ctx.saving;',
    'var widgets = __ctx.widgets;', 'var serializePortalContent = __ctx.serializePortalContent;',
    'var designerPaths = __ctx.designerPaths;',
    'var portalDesignerWriteSucceeded = __ctx.__helper;',
    'return async function saveDesign() {' + body + '};',
  ].join('\n')
  const fn = new Function('__ctx', pre)(ctx) as () => Promise<void>
  await fn()
  return { calls, toasts }
}

async function runOps(op: string, overrides: any) {
  const calls: any[] = [], toasts: any[] = []
  const resp = respOf(overrides)
  const mk = (m: string) => async (...args: any[]) => { calls.push({ m, args }); if (overrides.__throw) throw new Error(overrides.__throw); return resp }
  const api = { post: mk('post'), get: mk('get'), delete: mk('delete'), put: mk('put') }
  const activeId = { value: 'R413-FAKE' }
  const prompt = () => overrides.__promptVal !== undefined ? overrides.__promptVal : 'R413-FAKE'
  const confirmMsg = async () => true
  const ctx = { api, toast: mkToast(toasts), activeId, prompt, confirmMsg, __helper: buildHelper() }
  const body = stripTs(bodyOf('pdDesignerOps'))
  const pre = [
    '"use strict";',
    'var api = __ctx.api;', 'var toast = __ctx.toast;',
    'var activeId = __ctx.activeId;', 'var prompt = __ctx.prompt;',
    'var confirmMsg = __ctx.confirmMsg;',
    'var portalDesignerWriteSucceeded = __ctx.__helper;',
    'return async function pdDesignerOps(op) {' + body + '};',
  ].join('\n')
  const fn = new Function('__ctx', pre)(ctx) as (o: string) => Promise<void>
  await fn(op)
  return { calls, toasts }
}

async function runFileUpload(overrides: any) {
  const calls: any[] = [], toasts: any[] = []
  const resp = respOf(overrides)
  const api = { post: async (...args: any[]) => { calls.push({ m: 'post', args }); if (overrides.__throw) throw new Error(overrides.__throw); return resp } }
  const file = { text: async () => 'fake-content' }
  const input = { files: overrides.__noFile ? [] : [file] }
  const fid = 'R413-FAKE-FID'
  const ctx = { api, toast: mkToast(toasts), input, fid, __helper: buildHelper() }
  const body = stripTs(onChangeBody())
  const pre = [
    '"use strict";',
    'var api = __ctx.api;', 'var toast = __ctx.toast;',
    'var input = __ctx.input;', 'var fid = __ctx.fid;',
    'var portalDesignerWriteSucceeded = __ctx.__helper;',
    'return async function onchange() {' + body + '};',
  ].join('\n')
  const fn = new Function('__ctx', pre)(ctx) as () => Promise<void>
  await fn()
  return { calls, toasts }
}

describe('portalDesignerWriteSucceeded helper', () => {
  it('type:error 判失败', () => { expect(buildHelper()({ type: 'error', message: 'design not found' })).toBe(false) })
  it('success:false 判失败', () => { expect(buildHelper()({ success: false, message: 'R413 denied' })).toBe(false) })
  it('type:success 判成功', () => { expect(buildHelper()({ type: 'success', data: { id: 'x', saved: true, content: 'y' } })).toBe(true) })
  it('type 缺省 success 非 false 判成功', () => { expect(buildHelper()({ success: true, data: { saved: 1 } })).toBe(true); expect(buildHelper()({ data: { ok: 1 } })).toBe(true) })
  it('未知 type 判失败', () => { expect(buildHelper()({ type: 'weird', success: true })).toBe(false) })
  it('非对象 null/undefined 判失败', () => { expect(buildHelper()(null)).toBe(false); expect(buildHelper()(undefined)).toBe(false); expect(buildHelper()('ok')).toBe(false) })
  it('不强求 named bool，saved:0 仍成功', () => { expect(buildHelper()({ type: 'success', data: { saved: 0 } })).toBe(true) })
})

describe('saveDesign 写链守卫', () => {
  it('type:error → error透传不弹成功', async () => { const r = await runSaveDesign({ __resp: { type: 'error', message: 'design not found' } }); expect(toastText(r.toasts,'error')).toContain('design not found'); expect(hasToast(r.toasts,'success')).toBe(false) })
  it('success:false → error透传不弹成功', async () => { const r = await runSaveDesign({ __resp: { success: false, message: 'R413 denied' } }); expect(toastText(r.toasts,'error')).toContain('R413 denied'); expect(hasToast(r.toasts,'success')).toBe(false) })
  it('非对象响应 → 失败不弹成功', async () => { const r = await runSaveDesign({ __resp: undefined }); expect(hasToast(r.toasts,'error')).toBe(true); expect(hasToast(r.toasts,'success')).toBe(false) })
  it('type:success 真成功 → toast 门户布局已保存', async () => { const r = await runSaveDesign({ __resp: { type: 'success', data: { saved: true } } }); expect(hasToast(r.toasts,'success')).toBe(true); expect(toastText(r.toasts,'success')).toContain('门户布局已保存'); expect(hasToast(r.toasts,'error')).toBe(false) })
  it('非2xx 500 走 catch toast.error', async () => { const r = await runSaveDesign({ __throw: 'HTTP 500: Internal Server Error' }); expect(toastText(r.toasts,'error')).toContain('HTTP 500'); expect(hasToast(r.toasts,'success')).toBe(false) })
  it('端点与 payload 保持 + id 缺失零写', async () => { const r = await runSaveDesign({ __resp: { type: 'success' } }); expect(r.calls[0].args[0]).toContain('/api/portal/assemble/designer/save/R413-FAKE'); expect(r.calls[0].args[1].content).toBeDefined(); const z = await runSaveDesign({ __id: '', __resp: { type: 'success' } }); expect(z.calls.length).toBe(0) })
})

describe('saveById 写链守卫', () => {
  it('type:error → error透传无统一成功toast', async () => { const r = await runOps('saveById', { __resp: { type: 'error', message: 'design not found' } }); expect(toastText(r.toasts,'error')).toContain('design not found'); expect(hasToast(r.toasts,'success')).toBe(false) })
  it('success:false → error透传无统一成功toast', async () => { const r = await runOps('saveById', { __resp: { success: false, message: 'R413 denied' } }); expect(toastText(r.toasts,'error')).toContain('R413 denied'); expect(hasToast(r.toasts,'success')).toBe(false) })
  it('type:success → 统一 toast 门户设计器操作已提交', async () => { const r = await runOps('saveById', { __resp: { type: 'success', data: { saved: 0 } } }); expect(toastText(r.toasts,'success')).toContain('门户设计器操作已提交'); expect(hasToast(r.toasts,'error')).toBe(false) })
  it('非2xx 500 走 catch toast.error', async () => { const r = await runOps('saveById', { __throw: 'HTTP 500: Internal Server Error' }); expect(toastText(r.toasts,'error')).toContain('HTTP 500'); expect(hasToast(r.toasts,'success')).toBe(false) })
  it('端点保持', async () => { const r = await runOps('saveById', { __resp: { type: 'success' } }); expect(r.calls[0].args[0]).toContain('/api/portal/assemble/designer/save/') })
  it('其他分支 pageDelete 保持走统一成功toast(不动)', async () => { const r = await runOps('pageDelete', { __resp: { type: 'success' } }); expect(toastText(r.toasts,'success')).toContain('门户设计器操作已提交') })
})

describe('fileUpload onchange 写链守卫', () => {
  it('type:error → error透传不弹资源内容已上传', async () => { const r = await runFileUpload({ __resp: { type: 'error', message: 'file not found' } }); expect(toastText(r.toasts,'error')).toContain('file not found'); expect(hasToast(r.toasts,'success')).toBe(false) })
  it('success:false → error透传不弹成功', async () => { const r = await runFileUpload({ __resp: { success: false, message: 'R413 denied' } }); expect(toastText(r.toasts,'error')).toContain('R413 denied'); expect(hasToast(r.toasts,'success')).toBe(false) })
  it('type:success → toast 资源内容已上传', async () => { const r = await runFileUpload({ __resp: { type: 'success', data: {} } }); expect(toastText(r.toasts,'success')).toContain('资源内容已上传'); expect(hasToast(r.toasts,'error')).toBe(false) })
  it('端点保持 + 无文件零写', async () => { const r = await runFileUpload({ __resp: { type: 'success' } }); expect(r.calls[0].args[0]).toContain('/api/portal/assemble/designer/file/upload/'); const z = await runFileUpload({ __noFile: true }); expect(z.calls.length).toBe(0) })
})
