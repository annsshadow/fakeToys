// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const SRC = readFileSync(new URL('./FormDesigner.vue', import.meta.url), 'utf8')

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
  const body = stripTs(bodyOf('formDesignerWriteSucceeded'))
  if (!body) return () => false
  const F = new Function('resp', '"use strict"; ' + body)
  return (r: any) => !!F(r)
}

interface Ctx {
  currentForm: any; routeAppId: any; layoutConfig: any; mobileLayout: any
  validationRules: any; validationGroups: any; serializeFormDefinition: any
  formSavePayload: any; api: any; loadForms: any; toast: any
  _writes: any[]; _info: string[]; _errors: string[]; _loads: number[]
}

function makeCtx(resp: any, mode: string = 'update'): Ctx {
  const writes: any[] = [], info: string[] = [], errors: string[] = [], loads: number[] = []
  const ctx: any = {
    currentForm: { value: { id: mode === 'create' ? '' : 'R421-FAKE-FORM', name: '测试表单', flag: '', appId: 'default', status: 'draft', fields: [], definition: {} } },
    routeAppId: { value: 'default' },
    layoutConfig: { value: { columns: 1, gutter: 16 } },
    mobileLayout: { value: { root: [], columns: 1, gutter: 16 } },
    validationRules: { value: {} },
    validationGroups: { value: {} },
    serializeFormDefinition: (def: any) => def,
    formSavePayload: (def: any, meta: any) => Object.assign({ definition: def }, meta),
    api: {
      put: async (url: string, payload: any) => { writes.push({ method: 'PUT', url, payload }); return resp },
      post: async (url: string, payload: any) => { writes.push({ method: 'POST', url, payload }); return { data: { id: 'NEWID' } } },
    },
    loadForms: async () => { loads.push(1) },
    toast: { info: (m: string) => info.push(m), error: (m: string) => errors.push(m) },
    _writes: writes, _info: info, _errors: errors, _loads: loads,
  }
  return ctx as Ctx
}

async function runSave(ctx: Ctx, helper: (r: any) => boolean) {
  const body = stripTs(bodyOf('saveForm'))
  const pre = [
    '"use strict";',
    'var currentForm = ctx.currentForm;',
    'var routeAppId = ctx.routeAppId;',
    'var layoutConfig = ctx.layoutConfig;',
    'var mobileLayout = ctx.mobileLayout;',
    'var validationRules = ctx.validationRules;',
    'var validationGroups = ctx.validationGroups;',
    'var serializeFormDefinition = ctx.serializeFormDefinition;',
    'var formSavePayload = ctx.formSavePayload;',
    'var api = ctx.api;',
    'var loadForms = ctx.loadForms;',
    'var toast = ctx.toast;',
    'var formDesignerWriteSucceeded = helper;',
    'return (async function () {' + body + '})();',
  ].join(' ')
  const F = new Function('ctx', 'helper', pre)
  await F(ctx, helper)
}

describe('formDesignerWriteSucceeded helper', () => {
  it('type:error 判失败', () => { expect(makeHelper()({ type: 'error', message: 'form not found' })).toBe(false) })
  it('success:false 判失败', () => { expect(makeHelper()({ type: 'success', success: false })).toBe(false) })
  it('type:success 判成功', () => { expect(makeHelper()({ type: 'success', data: {} })).toBe(true) })
  it('type缺省success非false 判成功', () => { expect(makeHelper()({ data: { id: 'X' } })).toBe(true) })
  it('非对象null/undefined/数字 判失败', () => { const h = makeHelper(); expect(h(null)).toBe(false); expect(h(undefined)).toBe(false); expect(h(123)).toBe(false) })
  it('未知type 判失败', () => { expect(makeHelper()({ type: 'weird' })).toBe(false) })
})

describe('saveForm 更新分支', () => {
  it('type:error 失败不写回不刷新不弹成功且透传message', async () => {
    const c = makeCtx({ type: 'error', message: 'form not found' })
    await runSave(c, makeHelper())
    expect(c._errors.some(m => m.indexOf('form not found') >= 0)).toBe(true)
    expect(c._info.some(m => m.indexOf('保存成功') >= 0)).toBe(false)
    expect(c._loads.length).toBe(0)
    expect(JSON.stringify(c.currentForm.value.definition)).toBe('{}')
  })
  it('success:false 失败同样拦截', async () => {
    const c = makeCtx({ type: 'success', success: false, message: 'R421 denied' })
    await runSave(c, makeHelper())
    expect(c._errors.some(m => m.indexOf('R421 denied') >= 0)).toBe(true)
    expect(c._info.length).toBe(0)
    expect(c._loads.length).toBe(0)
  })
  it('type:success 成功写回+loadForms+toast.info', async () => {
    const c = makeCtx({ type: 'success', data: {} })
    await runSave(c, makeHelper())
    expect(c._info.some(m => m.indexOf('保存成功') >= 0)).toBe(true)
    expect(c._loads.length).toBe(1)
    expect(c._errors.length).toBe(0)
  })
  it('不带权威bool的成功信封仍判成功', async () => {
    const c = makeCtx({ data: { id: 'X' } })
    await runSave(c, makeHelper())
    expect(c._info.some(m => m.indexOf('保存成功') >= 0)).toBe(true)
  })
  it('HTTP500非2xx catch toast.error且不写回', async () => {
    const c = makeCtx(null)
    c.api.put = async (url: string, payload: any) => { c._writes.push({ method: 'PUT', url, payload }); throw new Error('HTTP 500') }
    await runSave(c, makeHelper())
    expect(c._errors.some(m => m.indexOf('HTTP 500') >= 0)).toBe(true)
    expect(c._loads.length).toBe(0)
    expect(c._info.length).toBe(0)
  })
  it('PUT URL与payload正确', async () => {
    const c = makeCtx({ type: 'success' })
    await runSave(c, makeHelper())
    expect(c._writes[0].url).toBe('/api/form/R421-FAKE-FORM')
    expect(c._writes[0].payload.name).toBe('测试表单')
    expect(c._writes[0].payload.status).toBe('draft')
  })
})

describe('saveForm 创建分支', () => {
  it('创建成功写id+写回+loadForms+toast.info', async () => {
    const c = makeCtx({ type: 'success' }, 'create')
    await runSave(c, makeHelper())
    expect(c.currentForm.value.id).toBe('NEWID')
    expect(c._info.some(m => m.indexOf('保存成功') >= 0)).toBe(true)
    expect(c._loads.length).toBe(1)
  })
})
