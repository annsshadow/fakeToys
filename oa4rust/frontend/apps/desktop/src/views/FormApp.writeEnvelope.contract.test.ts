// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const SRC = readFileSync(new URL('./FormApp.vue', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1'), 'utf8')

function bodyOf(signature: string, from: number): string {
  const start = SRC.indexOf(signature, from)
  if (start < 0) return ''
  const open = SRC.indexOf('{', start)
  let depth = 0, inS = 0, inD = 0, inT = 0, esc = false
  for (let i = open; i < SRC.length; i++) {
    const c = SRC.charCodeAt(i)
    if (esc) { esc = false; continue }
    if (c === 92) { esc = true; continue }
    if (c === 39 && !inD && !inT) { inS = inS ? 0 : 1; continue }
    if (c === 34 && !inS && !inT) { inD = inD ? 0 : 1; continue }
    if (c === 96 && !inS && !inD) { inT = inT ? 0 : 1; continue }
    if (inS || inD || inT) continue
    if (c === 123) depth++
    else if (c === 125) { depth--; if (depth === 0) return SRC.slice(open + 1, i) }
  }
  return ''
}

function strip(s: string): string {
  return s
    .replace(/catch\s*\(\s*(\w+)\s*:\s*any\s*\)/g, 'catch($1)')
    .replace(/\b(\w+)\s*:\s*any\b/g, '$1')
    .replace(/<boolean>/g, '')
}

const helperFound = SRC.indexOf('function formWriteSucceeded(') >= 0
function makeHelper(): (r: unknown) => boolean {
  if (!helperFound) return () => false
  const body = strip(bodyOf('function formWriteSucceeded(', 0))
  const F = new Function('resp', '"use strict"; ' + body)
  return (r: unknown) => !!F(r)
}

interface SaveCtx {
  saving: any; mform: any; editingForm: any; api: any; showCreate: any
  loadList: any; toast: any; calls: any[]; info: string[]; errors: string[]
}

function makeSaveCtx(resp: any, mode: 'put' | 'post' | 'reject' = 'put'): SaveCtx {
  const calls: any[] = [], info: string[] = [], errors: string[] = []
  const ctx: any = {
    saving: { value: false },
    mform: { value: { name: '测试表单', schema: '{}' } },
    editingForm: { value: mode === 'post' ? null : { id: 'FAKE-FORM-ID' } },
    api: {
      put: async (u: string, p: any) => { calls.push({ m: 'PUT', u, p }); if (mode === 'reject') throw new Error('HTTP 500'); return resp },
      post: async (u: string, p: any) => { calls.push({ m: 'POST', u, p }); if (mode === 'reject') throw new Error('HTTP 500'); return resp },
    },
    showCreate: { value: true },
    loadList: async () => { calls.push({ m: 'LOAD' }) },
    toast: { info: (s: string) => info.push(s), error: (s: string) => errors.push(s) },
    calls, info, errors,
  }
  return ctx as SaveCtx
}

async function runSave(c: SaveCtx) {
  const body = strip(bodyOf('async function saveForm(', 0))
  const pre = [
    '"use strict";',
    'var saving = c.saving, mform = c.mform, editingForm = c.editingForm,',
    'api = c.api, showCreate = c.showCreate, loadList = c.loadList, toast = c.toast;',
    'var formWriteSucceeded = helper;',
    'return (async function () {' + body + '})();',
  ].join(' ')
  const F = new Function('c', 'helper', pre)
  await F(c, makeHelper())
}

interface DelCtx {
  confirmMsg: any; api: any; items: any; toast: any
  calls: any[]; info: string[]; errors: string[]
}

function makeDelCtx(resp: any, opts: { confirm?: boolean; reject?: boolean } = {}): DelCtx {
  const calls: any[] = [], info: string[] = [], errors: string[] = []
  const ctx: any = {
    confirmMsg: async (_msg: string) => { calls.push({ m: 'CONFIRM' }); return opts.confirm !== false },
    api: { delete: async (u: string) => { calls.push({ m: 'DELETE', u }); if (opts.reject) throw new Error('HTTP 500'); return resp } },
    items: { value: [{ id: 'FAKE-FORM-ID', name: '测试表单' }, { id: 'OTHER', name: '保留项' }] },
    toast: { info: (s: string) => info.push(s), error: (s: string) => errors.push(s) },
    calls, info, errors,
  }
  return ctx as DelCtx
}

async function runDel(c: DelCtx) {
  const body = strip(bodyOf('async function deleteForm(', 0))
  const pre = [
    '"use strict";',
    'var confirmMsg = c.confirmMsg, api = c.api, items = c.items, toast = c.toast;',
    'var formWriteSucceeded = helper;',
    'return (async function (f) {' + body + '})(c.f);',
  ].join(' ')
  const F = new Function('c', 'helper', pre)
  await F(c, makeHelper())
}

describe('formWriteSucceeded helper', () => {
  it('type:error 判失败', () => { expect(helperFound).toBe(true); expect(makeHelper()({ type: 'error', message: 'form not found' })).toBe(false) })
  it('success:false 判失败', () => { expect(helperFound).toBe(true); expect(makeHelper()({ success: false, message: 'denied' })).toBe(false) })
  it('type:success 判成功', () => { expect(helperFound).toBe(true); expect(makeHelper()({ type: 'success', data: {} })).toBe(true) })
  it('type 缺省 success 非 false 判成功', () => { expect(helperFound).toBe(true); expect(makeHelper()({ data: {} })).toBe(true) })
  it('非对象/null/undefined 判失败', () => { expect(helperFound).toBe(true); const h = makeHelper(); expect(h(null)).toBe(false); expect(h(undefined)).toBe(false); expect(h(42)).toBe(false) })
  it('不强求 named bool，updated:0 仍成功', () => { expect(helperFound).toBe(true); expect(makeHelper()({ type: 'success', data: { updated: 0 } })).toBe(true) })
})

describe('saveForm 写链守卫', () => {
  it('PUT type:error 失败: toast.error 透传、不关modal、不刷新', async () => {
    const c = makeSaveCtx({ type: 'error', message: 'form not found' })
    await runSave(c)
    expect(c.errors.some(s => s.indexOf('form not found') >= 0)).toBe(true)
    expect(c.showCreate.value).toBe(true)
    expect(c.calls.some(x => x.m === 'LOAD')).toBe(false)
    expect(c.info.length).toBe(0)
  })
  it('PUT success:false 失败拦截', async () => {
    const c = makeSaveCtx({ success: false, message: 'R438 denied' })
    await runSave(c)
    expect(c.errors.some(s => s.indexOf('R438 denied') >= 0)).toBe(true)
    expect(c.showCreate.value).toBe(true)
    expect(c.calls.some(x => x.m === 'LOAD')).toBe(false)
  })
  it('PUT type:success 成功: 关modal+刷新', async () => {
    const c = makeSaveCtx({ type: 'success', data: { id: 'FAKE-FORM-ID', updated: true } })
    await runSave(c)
    expect(c.showCreate.value).toBe(false)
    expect(c.calls.some(x => x.m === 'LOAD')).toBe(true)
    expect(c.errors.length).toBe(0)
  })
  it('POST 创建分支成功同样关modal+刷新', async () => {
    const c = makeSaveCtx({ type: 'success', data: { id: 'NEW' } }, 'post')
    await runSave(c)
    expect(c.calls.some(x => x.m === 'POST')).toBe(true)
    expect(c.showCreate.value).toBe(false)
    expect(c.calls.some(x => x.m === 'LOAD')).toBe(true)
  })
  it('非2xx reject 走 catch 报错且不关modal不刷新', async () => {
    const c = makeSaveCtx(null, 'reject')
    await runSave(c)
    expect(c.errors.some(s => s.indexOf('HTTP 500') >= 0)).toBe(true)
    expect(c.showCreate.value).toBe(true)
    expect(c.calls.some(x => x.m === 'LOAD')).toBe(false)
  })
  it('PUT URL与payload正确', async () => {
    const c = makeSaveCtx({ type: 'success' })
    await runSave(c)
    const put = c.calls.find(x => x.m === 'PUT')
    expect(put.u).toBe('/api/form/FAKE-FORM-ID')
    expect(put.p.name).toBe('测试表单')
  })
})

describe('deleteForm 写链守卫', () => {
  it('type:error 失败: toast.error 透传、不移除行', async () => {
    const c = makeDelCtx({ type: 'error', message: 'form not found' })
    ;(c as any).f = { id: 'FAKE-FORM-ID', name: '测试表单' }
    await runDel(c)
    expect(c.errors.some(s => s.indexOf('form not found') >= 0)).toBe(true)
    expect(c.items.value.map((x: any) => x.id)).toEqual(['FAKE-FORM-ID', 'OTHER'])
  })
  it('success:false 失败同样保留行', async () => {
    const c = makeDelCtx({ success: false, message: 'R438 denied' })
    ;(c as any).f = { id: 'FAKE-FORM-ID', name: '测试表单' }
    await runDel(c)
    expect(c.errors.some(s => s.indexOf('R438 denied') >= 0)).toBe(true)
    expect(c.items.value.length).toBe(2)
  })
  it('type:success 成功才移除行', async () => {
    const c = makeDelCtx({ type: 'success', data: { id: 'FAKE-FORM-ID', deleted: true } })
    ;(c as any).f = { id: 'FAKE-FORM-ID', name: '测试表单' }
    await runDel(c)
    expect(c.items.value.map((x: any) => x.id)).toEqual(['OTHER'])
    expect(c.errors.length).toBe(0)
  })
  it('confirm 返回 false 零写、行保留', async () => {
    const c = makeDelCtx({ type: 'success' }, { confirm: false })
    ;(c as any).f = { id: 'FAKE-FORM-ID', name: '测试表单' }
    await runDel(c)
    expect(c.calls.some(x => x.m === 'DELETE')).toBe(false)
    expect(c.items.value.length).toBe(2)
  })
  it('非2xx reject 走 catch 报错且行保留', async () => {
    const c = makeDelCtx(null, { reject: true })
    ;(c as any).f = { id: 'FAKE-FORM-ID', name: '测试表单' }
    await runDel(c)
    expect(c.errors.some(s => s.indexOf('HTTP 500') >= 0)).toBe(true)
    expect(c.items.value.length).toBe(2)
  })
})
