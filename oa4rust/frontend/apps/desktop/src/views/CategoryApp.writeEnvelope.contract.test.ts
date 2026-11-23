// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const SRC = readFileSync(new URL('./CategoryApp.vue', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1'), 'utf8')

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

const helperFound = SRC.indexOf('function categoryWriteSucceeded(') >= 0
function makeHelper(): (r: unknown) => boolean {
  if (!helperFound) return () => false
  const body = strip(bodyOf('function categoryWriteSucceeded(', 0))
  const F = new Function('resp', '"use strict"; ' + body)
  return (r: unknown) => !!F(r)
}

function makeCtx(resp: any, opts: { confirm?: boolean; reject?: boolean; prompts?: string[] } = {}) {
  const calls: any[] = [], errors: string[] = []
  const prompts = opts.prompts !== undefined ? opts.prompts : ['FAKE-APP-ID', 'R440测试分类']
  let pi = 0
  const ctx: any = {
    prompt: (_msg: string) => { const v = prompts[pi++]; calls.push({ m: 'PROMPT', v }); return v === undefined ? null : v },
    confirmMsg: async (_msg: string) => { calls.push({ m: 'CONFIRM' }); return opts.confirm !== false },
    api: {
      post: async (u: string, p: any) => { calls.push({ m: 'POST', u, p }); if (opts.reject) throw new Error('HTTP 500'); return resp },
      delete: async (u: string) => { calls.push({ m: 'DELETE', u }); if (opts.reject) throw new Error('HTTP 500'); return resp },
    },
    load: () => { calls.push({ m: 'LOAD' }) },
    toast: { error: (s: string) => errors.push(s) },
    item: { id: 'FAKE-CATEGORY-ID', name: 'R440测试分类' },
    calls, errors,
  }
  return ctx
}

async function runCreate(c: any) {
  const body = strip(bodyOf('async function createCategory(', 0))
  const pre = [
    '"use strict";',
    'var prompt = c.prompt, api = c.api, load = c.load, toast = c.toast;',
    'var categoryWriteSucceeded = helper;',
    'return (async function () {' + body + '})();',
  ].join(' ')
  const F = new Function('c', 'helper', pre)
  await F(c, makeHelper())
}

async function runDelete(c: any) {
  const body = strip(bodyOf('async function deleteCategory(', 0))
  const pre = [
    '"use strict";',
    'var confirmMsg = c.confirmMsg, api = c.api, load = c.load, toast = c.toast;',
    'var categoryWriteSucceeded = helper;',
    'return (async function (item) {' + body + '})(c.item);',
  ].join(' ')
  const F = new Function('c', 'helper', pre)
  await F(c, makeHelper())
}

describe('categoryWriteSucceeded helper', () => {
  it('type:error 判失败', () => { expect(helperFound).toBe(true); expect(makeHelper()({ type: 'error', message: 'application not found' })).toBe(false) })
  it('success:false 判失败', () => { expect(helperFound).toBe(true); expect(makeHelper()({ success: false, message: 'denied' })).toBe(false) })
  it('type:success 判成功', () => { expect(helperFound).toBe(true); expect(makeHelper()({ type: 'success', data: { id: 'X' } })).toBe(true) })
  it('type 缺省且 success 非 false 判成功', () => { expect(helperFound).toBe(true); expect(makeHelper()({ data: { deleted: true } })).toBe(true) })
  it('非对象 null undefined 数字 判失败', () => { expect(helperFound).toBe(true); const h = makeHelper(); expect(h(null)).toBe(false); expect(h(undefined)).toBe(false); expect(h(42)).toBe(false) })
  it('不强求权威 bool: deleted:0 成功', () => { expect(helperFound).toBe(true); expect(makeHelper()({ type: 'success', data: { deleted: 0 } })).toBe(true) })
  it('未知 type 判失败', () => { expect(helperFound).toBe(true); expect(makeHelper()({ type: 'weird', data: {} })).toBe(false) })
})

describe('createCategory 写链守卫', () => {
  it('type:error 失败: toast.error 透传且不 load', async () => {
    const c = makeCtx({ type: 'error', message: 'application not found' })
    await runCreate(c)
    expect(c.errors.some((s: string) => s.indexOf('application not found') >= 0)).toBe(true)
    expect(c.calls.some((x: any) => x.m === 'LOAD')).toBe(false)
  })
  it('success:false 失败拦截不 load', async () => {
    const c = makeCtx({ success: false, message: 'R440 denied' })
    await runCreate(c)
    expect(c.errors.some((s: string) => s.indexOf('R440 denied') >= 0)).toBe(true)
    expect(c.calls.some((x: any) => x.m === 'LOAD')).toBe(false)
  })
  it('type:success 成功才 load', async () => {
    const c = makeCtx({ type: 'success', data: { id: 'NEW' } })
    await runCreate(c)
    expect(c.calls.some((x: any) => x.m === 'LOAD')).toBe(true)
    expect(c.errors.length).toBe(0)
  })
  it('type 缺省 success 非 false 成功 load', async () => {
    const c = makeCtx({ data: { id: 'NEW' } })
    await runCreate(c)
    expect(c.calls.some((x: any) => x.m === 'LOAD')).toBe(true)
    expect(c.errors.length).toBe(0)
  })
  it('prompt 为空零写零 load', async () => {
    const c = makeCtx({ type: 'success' }, { prompts: [''] })
    await runCreate(c)
    expect(c.calls.some((x: any) => x.m === 'POST')).toBe(false)
    expect(c.calls.some((x: any) => x.m === 'LOAD')).toBe(false)
  })
  it('非2xx reject 走 catch 报错且不 load', async () => {
    const c = makeCtx(null, { reject: true })
    await runCreate(c)
    expect(c.errors.some((s: string) => s.indexOf('HTTP 500') >= 0)).toBe(true)
    expect(c.calls.some((x: any) => x.m === 'LOAD')).toBe(false)
  })
  it('POST URL 与 payload 不变', async () => {
    const c = makeCtx({ type: 'success' })
    await runCreate(c)
    const p = c.calls.find((x: any) => x.m === 'POST')
    expect(p.u).toBe('/api/categoryinfo')
    expect(p.p).toEqual({ appId: 'FAKE-APP-ID', name: 'R440测试分类' })
  })
})

describe('deleteCategory 写链守卫', () => {
  it('type:error 失败: toast.error 透传且不 load', async () => {
    const c = makeCtx({ type: 'error', message: 'category not found' })
    await runDelete(c)
    expect(c.errors.some((s: string) => s.indexOf('category not found') >= 0)).toBe(true)
    expect(c.calls.some((x: any) => x.m === 'LOAD')).toBe(false)
  })
  it('success:false 失败拦截不 load', async () => {
    const c = makeCtx({ success: false, message: 'R440 denied' })
    await runDelete(c)
    expect(c.errors.some((s: string) => s.indexOf('R440 denied') >= 0)).toBe(true)
    expect(c.calls.some((x: any) => x.m === 'LOAD')).toBe(false)
  })
  it('type:success 成功才 load', async () => {
    const c = makeCtx({ type: 'success', data: { deleted: true } })
    await runDelete(c)
    expect(c.calls.some((x: any) => x.m === 'LOAD')).toBe(true)
    expect(c.errors.length).toBe(0)
  })
  it('confirm 返回 false 零写零 load', async () => {
    const c = makeCtx({ type: 'success' }, { confirm: false })
    await runDelete(c)
    expect(c.calls.some((x: any) => x.m === 'DELETE')).toBe(false)
    expect(c.calls.some((x: any) => x.m === 'LOAD')).toBe(false)
  })
  it('非2xx reject 走 catch 报错且不 load', async () => {
    const c = makeCtx(null, { reject: true })
    await runDelete(c)
    expect(c.errors.some((s: string) => s.indexOf('HTTP 500') >= 0)).toBe(true)
    expect(c.calls.some((x: any) => x.m === 'LOAD')).toBe(false)
  })
  it('DELETE URL 不变', async () => {
    const c = makeCtx({ type: 'success' })
    await runDelete(c)
    const d = c.calls.find((x: any) => x.m === 'DELETE')
    expect(d.u).toBe('/api/categoryinfo/FAKE-CATEGORY-ID')
  })
})
