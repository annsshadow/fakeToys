// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const SRC_PATH = new URL('./FindDesignerApp.vue', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1')
const src = readFileSync(SRC_PATH, 'utf8')

function bodyOf(name: string): string {
  const sig = 'function ' + name + '('
  const idx = src.indexOf(sig)
  if (idx < 0) return ''
  const open = src.indexOf('{', idx)
  if (open < 0) return ''
  let depth = 0, inS = 0, inD = 0, inT = 0, esc = false
  for (let j = open; j < src.length; j++) {
    const cc = src.charCodeAt(j)
    if (esc) { esc = false; continue }
    if (cc === 92) { esc = true; continue }
    if (cc === 39 && !inD && !inT) { inS = inS ? 0 : 1; continue }
    if (cc === 34 && !inS && !inT) { inD = inD ? 0 : 1; continue }
    if (cc === 96 && !inS && !inD) { inT = inT ? 0 : 1; continue }
    if (inS || inD || inT) continue
    if (cc === 123) depth++
    else if (cc === 125) { depth--; if (depth === 0) return src.slice(open + 1, j) }
  }
  return ''
}

function stripTs(s: string): string {
  return s
    .replace(/catch\s*\(\s*(\w+)\s*:\s*any\s*\)/g, 'catch($1)')
    .replace(/\b([A-Za-z_$][\w$]*)\s*:\s*any\b/g, '$1')
    .replace(/\b(const|let|var)\s+(\w+)\s*:\s*[^=;]+=/g, '$1 $2 =')
}

function makeHelper(): (r: any) => boolean {
  let body = ''
  try { body = stripTs(bodyOf('findDesignerWriteSucceeded')) } catch (e) { body = '' }
  if (!body) return () => false
  const F = new Function('resp', '"use strict"; ' + body) as (r: any) => boolean
  return (r: any) => !!F(r)
}

function makeCtx(overrides: any, fakeId?: boolean): any {
  const calls: any[] = [], info: any[] = [], errors: any[] = [], refetches: any[] = []
  const id = fakeId ? 'R428-FAKE-NEW-ID' : 'R428-FAKE-FLAG'
  const selected = { value: { id, name: fakeId ? '未命名' : '测试查询配置', flag: fakeId ? '' : 'R428-FAKE-FLAG', config: '{}' } }
  const config = { value: '{"a":1}' }
  const resp = Object.prototype.hasOwnProperty.call(overrides, '__resp') ? overrides.__resp : { type: 'success', saved: true }
  const put = async (url: string, payload: any) => {
    calls.push({ m: 'PUT', url, payload })
    if (overrides.__throw) throw new Error(overrides.__throw)
    return resp
  }
  const api = { put }
  const toast = {
    info: (m: string) => info.push(m),
    error: (m: string) => errors.push(m),
  }
  const refetch = async () => { refetches.push(1) }
  return { selected, config, api, toast, refetch, __helper: makeHelper(), calls, info, errors, refetches }
}

async function runFn(name: string, ctx: any): Promise<any> {
  const body = stripTs(bodyOf(name))
  const pre = [
    '"use strict";',
    'var selected = ctx.selected;',
    'var config = ctx.config;',
    'var api = ctx.api;',
    'var toast = ctx.toast;',
    'var refetch = ctx.refetch;',
    'var findDesignerWriteSucceeded = ctx.__helper;',
    'return function ' + name + '() {' + body + '};',
  ].join('\n')
  const fn = new Function('ctx', pre)(ctx) as () => void
  fn()
  await new Promise<void>((resolve) => setTimeout(resolve, 0))
  return ctx
}

function ok(c: any): boolean { return c.info.some((m: string) => m.indexOf('保存成功') >= 0) }
function errHas(c: any, k: string): boolean { return c.errors.some((m: string) => m.indexOf(k) >= 0) }

describe('findDesignerWriteSucceeded helper', () => {
  it('type:error 判失败', () => { expect(makeHelper()({ type: 'error', message: 'query design not found' })).toBe(false) })
  it('success:false 判失败', () => { expect(makeHelper()({ type: 'success', success: false })).toBe(false) })
  it('type:success 判成功', () => { expect(makeHelper()({ type: 'success', saved: true })).toBe(true) })
  it('type缺省success非false 判成功', () => { expect(makeHelper()({ saved: true, data: {} })).toBe(true) })
  it('非对象null/undefined/数字 判失败', () => { const h = makeHelper(); expect(h(null)).toBe(false); expect(h(undefined)).toBe(false); expect(h(123)).toBe(false) })
  it('未知type 判失败', () => { expect(makeHelper()({ type: 'weird' })).toBe(false) })
})

describe('save 真列表行路径', () => {
  it('200+type:error 不弹成功且透传message', async () => {
    const c = await runFn('save', makeCtx({ __resp: { type: 'error', message: 'query design not found' } }))
    expect(errHas(c, 'query design not found')).toBe(true)
    expect(ok(c)).toBe(false)
    expect(c.info.length).toBe(0)
    expect(c.calls.length).toBe(1)
    expect(c.refetches.length).toBe(0)
  })
  it('success:false 同样拦截', async () => {
    const c = await runFn('save', makeCtx({ __resp: { type: 'success', success: false, message: 'R428 denied' } }))
    expect(errHas(c, 'R428 denied')).toBe(true)
    expect(ok(c)).toBe(false)
    expect(c.refetches.length).toBe(0)
  })
  it('非对象响应判失败', async () => {
    const c = await runFn('save', makeCtx({ __resp: null }))
    expect(c.errors.length).toBe(1)
    expect(ok(c)).toBe(false)
  })
  it('type:success 成功弹保存成功', async () => {
    const c = await runFn('save', makeCtx({}))
    expect(ok(c)).toBe(true)
    expect(c.errors.length).toBe(0)
    expect(c.refetches.length).toBe(1)
  })
  it('type缺省saved非统一bool也成功', async () => {
    const c = await runFn('save', makeCtx({ __resp: { id: 'X', saved: true, name: 'n', query: '{}' } }))
    expect(ok(c)).toBe(true)
  })
})

describe('save createNew 假id路径', () => {
  it('200+type:error 假id仍拦截不弹成功', async () => {
    const c = await runFn('save', makeCtx({ __resp: { type: 'error', message: 'query design not found' } }, true))
    expect(errHas(c, 'query design not found')).toBe(true)
    expect(ok(c)).toBe(false)
  })
  it('type:success 假id成功弹保存成功', async () => {
    const c = await runFn('save', makeCtx({ __resp: { type: 'success', saved: true } }, true))
    expect(ok(c)).toBe(true)
  })
})

describe('save 异常与端点', () => {
  it('HTTP500 走catch红toast', async () => {
    const c = await runFn('save', makeCtx({ __throw: 'HTTP 500' }))
    expect(errHas(c, 'HTTP 500')).toBe(true)
    expect(ok(c)).toBe(false)
  })
  it('HTTP403 走catch红toast', async () => {
    const c = await runFn('save', makeCtx({ __throw: '没有权限' }))
    expect(errHas(c, '没有权限')).toBe(true)
    expect(ok(c)).toBe(false)
  })
  it('PUT端点与payload保持', async () => {
    const c = await runFn('save', makeCtx({}))
    expect(c.calls[0].m).toBe('PUT')
    expect(c.calls[0].url.indexOf('/api/query/assemble/designer/save/R428-FAKE-FLAG')).toBeGreaterThanOrEqual(0)
    expect(c.calls[0].payload.query).toBe('{"a":1}')
    expect(c.refetches.length).toBe(1)
  })
})

describe('preview 零写', () => {
  it('preview 只弹配置预览不产生PUT', async () => {
    const c = await runFn('preview', makeCtx({}))
    expect(c.info.some((m: string) => m.indexOf('配置预览') >= 0)).toBe(true)
    expect(c.calls.length).toBe(0)
    expect(ok(c)).toBe(false)
  })
})
