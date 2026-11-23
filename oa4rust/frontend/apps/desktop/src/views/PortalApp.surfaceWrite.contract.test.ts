// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const SRC = readFileSync(new URL('./PortalApp.vue', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1'), 'utf8')

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

const helperFound = SRC.includes('function portalSurfaceWriteSucceeded(')
function makeHelper(): (r: any) => boolean {
  const body = stripTs(bodyOf('portalSurfaceWriteSucceeded'))
  if (!body) return () => false
  const F = new Function('resp', '"use strict"; ' + body)
  return (r: any) => !!F(r)
}

const surfaceBody = stripTs(bodyOf('surfaceOps'))

interface Ctx {
  op: string; api: any; prompt: any; confirmMsg: any; toast: any
  _writes: any[]; _gets: number[]; _success: string[]; _errors: string[]; _confirm: boolean
}

function makeCtx(op: string, resp: any, confirmResult = true): Ctx {
  const writes: any[] = [], gets: number[] = [], success: string[] = [], errors: string[] = []
  const ctx: any = {
    op,
    api: {
      post: async (url: string, payload: any) => { writes.push({ method: 'POST', url, payload }); return resp },
      delete: async (url: string) => { writes.push({ method: 'DELETE', url }); return resp },
      get: async (url: string) => { gets.push(1); writes.push({ method: 'GET', url }); return { type: 'success', data: {} } },
    },
    prompt: (_label: string, def: string) => def || 'x',
    confirmMsg: async () => confirmResult,
    toast: { success: (m: string) => success.push(m), error: (m: string) => errors.push(m), info: (m: string) => errors.push(m) },
    _writes: writes, _gets: gets, _success: success, _errors: errors, _confirm: confirmResult,
  }
  return ctx as Ctx
}

async function runSurface(ctx: Ctx, helper: (r: any) => boolean) {
  const pre = [
    '"use strict";',
    'var op = ctx.op;',
    'var api = ctx.api;',
    'var prompt = ctx.prompt;',
    'var confirmMsg = ctx.confirmMsg;',
    'var toast = ctx.toast;',
    'var portalSurfaceWriteSucceeded = helper;',
    'return (async function () {' + surfaceBody + '})();',
  ].join(' ')
  const F = new Function('ctx', 'helper', pre)
  await F(ctx, helper)
}

const WRITE_OPS = ['create', 'publish', 'dictSet', 'dictDel', 'scriptByName']
const READ_OPS = ['portalPage', 'pageById', 'dictData']

describe('portalSurfaceWriteSucceeded helper', () => {
  it('helper 存在', () => { expect(helperFound).toBe(true) })
  it('type:error 判失败', () => { expect(makeHelper()({ type: 'error', message: 'surface not found' })).toBe(false) })
  it('success:false 判失败', () => { expect(makeHelper()({ success: false })).toBe(false) })
  it('type:success 判成功', () => { expect(makeHelper()({ type: 'success', data: {} })).toBe(true) })
  it('type缺省 success 非 false 判成功', () => { expect(makeHelper()({ data: { id: 'X' } })).toBe(true) })
  it('非对象 null/undefined/数字 判失败', () => { const h = makeHelper(); expect(h(null)).toBe(false); expect(h(undefined)).toBe(false); expect(h(42)).toBe(false) })
})

describe('surfaceOps 五写分支守卫', () => {
  for (const op of WRITE_OPS) {
    it(op + ' 200+type:error 失败红toast不弹绿成功', async () => {
      const c = makeCtx(op, { type: 'error', message: op + ' surface error' })
      await runSurface(c, makeHelper())
      expect(c._errors.some(m => m.indexOf(op + ' surface error') >= 0)).toBe(true)
      expect(c._success.some(m => m.indexOf('门户表面操作已提交') >= 0)).toBe(false)
    })
    it(op + ' success:false 失败红toast不弹绿成功', async () => {
      const c = makeCtx(op, { success: false, message: 'R434 denied' })
      await runSurface(c, makeHelper())
      expect(c._errors.some(m => m.indexOf('R434 denied') >= 0)).toBe(true)
      expect(c._success.length).toBe(0)
    })
    it(op + ' type:success 成功才弹绿成功', async () => {
      const c = makeCtx(op, { type: 'success', data: { id: 'X' } })
      await runSurface(c, makeHelper())
      expect(c._success.some(m => m.indexOf('门户表面操作已提交') >= 0)).toBe(true)
      expect(c._errors.length).toBe(0)
    })
  }
  it('非2xx reject 走 catch 红toast不弹成功', async () => {
    const c = makeCtx('publish', null)
    c.api.post = async (url: string, payload: any) => { c._writes.push({ method: 'POST', url, payload }); throw new Error('HTTP 500') }
    await runSurface(c, makeHelper())
    expect(c._errors.some(m => m.indexOf('HTTP 500') >= 0)).toBe(true)
    expect(c._success.length).toBe(0)
  })
})

describe('surfaceOps 只读GET与confirm守卫保持', () => {
  for (const op of READ_OPS) {
    it(op + ' 只读GET零改:仅发起GET且不触发守卫错误', async () => {
      const c = makeCtx(op, null)
      await runSurface(c, makeHelper())
      expect(c._gets.length).toBe(1)
      expect(c._errors.length).toBe(0)
    })
  }
  it('dictDel confirm 返回 false 零写、无toast', async () => {
    const c = makeCtx('dictDel', { type: 'success' }, false)
    await runSurface(c, makeHelper())
    expect(c._writes.filter(w => w.method === 'DELETE').length).toBe(0)
    expect(c._success.length).toBe(0)
    expect(c._errors.length).toBe(0)
  })
  it('create 端点正确', async () => {
    const c = makeCtx('create', { type: 'success' })
    await runSurface(c, makeHelper())
    expect(c._writes[0].method).toBe('POST')
    expect(c._writes[0].url).toBe('/api/portal/assemble/surface/create')
  })
})
