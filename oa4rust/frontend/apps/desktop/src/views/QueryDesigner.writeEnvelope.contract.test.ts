// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const SRC = readFileSync(new URL('./QueryDesigner.vue', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1'), 'utf8')

function bodyOf(name: string): string {
  const sig = 'function ' + name + '('
  const i = SRC.indexOf(sig)
  if (i < 0) return ''
  const start = SRC.indexOf(String.fromCharCode(123), i)
  let depth = 0, inS = 0, inD = 0, inT = 0, esc = false
  for (let j = start; j < SRC.length; j++) {
    const cc = SRC.charCodeAt(j)
    if (esc) { esc = false; continue }
    if (cc === 92) { esc = true; continue }
    if (cc === 39 && !inD && !inT) { inS = inS ? 0 : 1; continue }
    if (cc === 34 && !inS && !inT) { inD = inD ? 0 : 1; continue }
    if (cc === 96 && !inS && !inD) { inT = inT ? 0 : 1; continue }
    if (inS || inD || inT) continue
    if (cc === 123) depth++
    else if (cc === 125) { depth--; if (depth === 0) return SRC.slice(start + 1, j) }
  }
  return ''
}

function stripTs(s: string): string {
  return s
    .replace(/catch\s*\(\s*(\w+)\s*:\s*any\s*\)/g, 'catch($1)')
    .replace(/\b(const|let|var)\s+(\w+)\s*:\s*any\b/g, '$1 $2')
    .replace(/\b([A-Za-z_$][\w$]*)\s*:\s*any\b/g, '$1')
}

function makeHelper(): (r: any) => boolean {
  const body = stripTs(bodyOf('queryDesignerWriteSucceeded'))
  if (!body) return () => false
  const F = new Function('resp', '"use strict"; ' + body)
  return (r: any) => !!F(r)
}

interface Ctx {
  querySaving: any; mform: any; editingQuery: any; showModal: any
  api: any; loadQueries: any; toast: any
  _writes: any[]; _info: string[]; _errors: string[]; _loads: number[]
}

function makeCtx(resp: any, mode: string): Ctx {
  const writes: any[] = [], info: string[] = [], errors: string[] = [], loads: number[] = []
  const ctx: any = {
    querySaving: { value: false },
    mform: { value: { name: '测试查询', category: '通用', sql: 'select 1' } },
    editingQuery: { value: mode === 'create' ? null : { id: 'R431-FAKE' } },
    showModal: { value: true },
    api: {
      put: async (url: string, payload: any) => { writes.push({ method: 'PUT', url, payload }); return resp },
      post: async (url: string, payload: any) => { writes.push({ method: 'POST', url, payload }); return resp },
    },
    loadQueries: async () => { loads.push(1) },
    toast: { info: (m: string) => info.push(m), error: (m: string) => errors.push(m) },
    _writes: writes, _info: info, _errors: errors, _loads: loads,
  }
  return ctx as Ctx
}

async function runSave(ctx: Ctx, helper: (r: any) => boolean) {
  const body = stripTs(bodyOf('saveQuery'))
  const pre = [
    '"use strict";',
    'var querySaving = ctx.querySaving;',
    'var mform = ctx.mform;',
    'var editingQuery = ctx.editingQuery;',
    'var showModal = ctx.showModal;',
    'var api = ctx.api;',
    'var loadQueries = ctx.loadQueries;',
    'var toast = ctx.toast;',
    'var queryDesignerWriteSucceeded = helper;',
    'return (async function () {' + body + '})();',
  ].join(' ')
  const F = new Function('ctx', 'helper', pre)
  await F(ctx, helper)
}

describe('queryDesignerWriteSucceeded helper', () => {
  it('type:error 判失败', () => { expect(makeHelper()({ type: 'error', message: 'query design not found' })).toBe(false) })
  it('success:false 判失败', () => { expect(makeHelper()({ type: 'success', success: false })).toBe(false) })
  it('type:success 判成功', () => { expect(makeHelper()({ type: 'success', data: {} })).toBe(true) })
  it('type缺省success非false 判成功', () => { expect(makeHelper()({ data: { id: 'X' } })).toBe(true) })
  it('非对象null/undefined/数字 判失败', () => { const h = makeHelper(); expect(h(null)).toBe(false); expect(h(undefined)).toBe(false); expect(h(123)).toBe(false) })
  it('不强求 named bool，saved:0 仍成功', () => { expect(makeHelper()({ type: 'success', data: { saved: 0 } })).toBe(true) })
})

describe('saveQuery 更新分支 (PUT)', () => {
  it('type:error 失败透传message不关modal不刷新', async () => {
    const c = makeCtx({ type: 'error', message: 'query design not found' }, 'edit')
    await runSave(c, makeHelper())
    expect(c._errors.some(m => m.indexOf('query design not found') >= 0)).toBe(true)
    expect(c.showModal.value).toBe(true)
    expect(c._loads.length).toBe(0)
  })
  it('success:false 失败同样拦截', async () => {
    const c = makeCtx({ success: false, message: 'R431 denied' }, 'edit')
    await runSave(c, makeHelper())
    expect(c._errors.some(m => m.indexOf('R431 denied') >= 0)).toBe(true)
    expect(c.showModal.value).toBe(true)
    expect(c._loads.length).toBe(0)
  })
  it('type:success 成功关modal+loadQueries', async () => {
    const c = makeCtx({ type: 'success', data: { saved: true } }, 'edit')
    await runSave(c, makeHelper())
    expect(c.showModal.value).toBe(false)
    expect(c._loads.length).toBe(1)
    expect(c._errors.length).toBe(0)
  })
  it('不带权威bool的成功信封仍判成功', async () => {
    const c = makeCtx({ data: { id: 'X' } }, 'edit')
    await runSave(c, makeHelper())
    expect(c.showModal.value).toBe(false)
    expect(c._loads.length).toBe(1)
  })
  it('HTTP500非2xx catch toast.error且modal保留', async () => {
    const c = makeCtx(null, 'edit')
    c.api.put = async (url: string, payload: any) => { c._writes.push({ method: 'PUT', url, payload }); throw new Error('HTTP 500') }
    await runSave(c, makeHelper())
    expect(c._errors.some(m => m.indexOf('HTTP 500') >= 0)).toBe(true)
    expect(c.showModal.value).toBe(true)
    expect(c._loads.length).toBe(0)
  })
  it('PUT URL与payload正确', async () => {
    const c = makeCtx({ type: 'success' }, 'edit')
    await runSave(c, makeHelper())
    expect(c._writes[0].method).toBe('PUT')
    expect(c._writes[0].url).toBe('/api/query/assemble/designer/save/R431-FAKE')
    expect(c._writes[0].payload.name).toBe('测试查询')
    expect(c._writes[0].payload.query).toBe('select 1')
  })
})

describe('saveQuery 创建分支 (POST)', () => {
  it('type:error 失败透传不关modal不刷新', async () => {
    const c = makeCtx({ type: 'error', message: 'query design not found' }, 'create')
    await runSave(c, makeHelper())
    expect(c._errors.some(m => m.indexOf('query design not found') >= 0)).toBe(true)
    expect(c.showModal.value).toBe(true)
    expect(c._loads.length).toBe(0)
  })
  it('type:success 成功关modal+刷新且POST端点正确', async () => {
    const c = makeCtx({ type: 'success', data: { id: 'NEW' } }, 'create')
    await runSave(c, makeHelper())
    expect(c.showModal.value).toBe(false)
    expect(c._loads.length).toBe(1)
    expect(c._writes[0].method).toBe('POST')
    expect(c._writes[0].url).toBe('/api/query/assemble/designer/create')
  })
})

describe('saveQuery 在途与校验拦截', () => {
  it('querySaving在途直接return零写', async () => {
    const c = makeCtx({ type: 'success' }, 'edit')
    c.querySaving.value = true
    await runSave(c, makeHelper())
    expect(c._writes.length).toBe(0)
    expect(c._loads.length).toBe(0)
  })
  it('空名称校验toast.info并return零写', async () => {
    const c = makeCtx({ type: 'success' }, 'edit')
    c.mform.value.name = ''
    await runSave(c, makeHelper())
    expect(c._writes.length).toBe(0)
    expect(c._info.some(m => m.indexOf('请输入查询名称') >= 0)).toBe(true)
  })
})
