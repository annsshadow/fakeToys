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

const FAKE = 'R432-FAKE-QUERY'

interface Ctx {
  selected: any; queries: any; api: any; toast: any
  _writes: any[]; _errors: string[]
}

function makeCtx(resp: any, confirmResult = true): Ctx {
  const writes: any[] = [], errors: string[] = []
  const ctx: any = {
    selected: { value: { id: FAKE, name: '测试查询' } },
    queries: { value: [{ id: FAKE, name: '测试查询' }, { id: 'OTHER', name: '保留查询' }] },
    api: {
      delete: async (url: string) => { writes.push({ method: 'DELETE', url }); return resp },
    },
    toast: { error: (m: string) => errors.push(m), info: (m: string) => errors.push(m) },
    __confirmResult: confirmResult,
    __q: { id: FAKE, name: '测试查询' },
    _writes: writes, _errors: errors,
  }
  return ctx as Ctx
}

async function runDelete(ctx: Ctx, helper: (r: any) => boolean) {
  const body = stripTs(bodyOf('deleteQuery'))
  const pre = [
    '"use strict";',
    'var selected = ctx.selected;',
    'var queries = ctx.queries;',
    'var api = ctx.api;',
    'var toast = ctx.toast;',
    'var queryDesignerWriteSucceeded = helper;',
    'var confirmMsg = async function () { return ctx.__confirmResult; };',
    'var q = ctx.__q;',
    'return (async function () {' + body + '})();',
  ].join(' ')
  const F = new Function('ctx', 'helper', pre)
  await F(ctx, helper)
}

describe('QueryDesigner confirmMsg 导入契约', () => {
  it('import 段含 confirmMsg（否则 deleteQuery 点击触发 ReferenceError 不可达）', () => {
    const m = SRC.match(/import\s*\{\s*([^}]*)\}\s*from\s*'\.\.\/utils\/toast'/)
    expect(m, '应存在从 ../utils/toast 的具名导入').not.toBeNull()
    expect(m![1]).toContain('confirmMsg')
    expect(m![1]).toContain('toast')
  })
})

describe('queryDesignerWriteSucceeded helper（R431 复用）', () => {
  it('type:error 判失败', () => { expect(makeHelper()({ type: 'error', message: 'query design not found or already deleted' })).toBe(false) })
  it('success:false 判失败', () => { expect(makeHelper()({ success: false })).toBe(false) })
  it('type:success 判成功', () => { expect(makeHelper()({ type: 'success', data: { deleted: true } })).toBe(true) })
  it('type缺省 success 非 false 判成功', () => { expect(makeHelper()({ data: { deleted: 0 } })).toBe(true) })
  it('非对象 null/undefined 判失败', () => { const h = makeHelper(); expect(h(null)).toBe(false); expect(h(undefined)).toBe(false) })
})

describe('deleteQuery 写链守卫', () => {
  it('200+type:error 失败：红toast透传、不清selected、不移除行', async () => {
    const c = makeCtx({ type: 'error', message: 'query design not found or already deleted' })
    await runDelete(c, makeHelper())
    expect(c._errors.some(m => m.indexOf('query design not found or already deleted') >= 0)).toBe(true)
    expect(c.selected.value?.id).toBe(FAKE)
    expect(c.queries.value.map((x: any) => x.id)).toContain(FAKE)
  })
  it('success:false 失败：红toast透传、不清selected、不移除行', async () => {
    const c = makeCtx({ success: false, message: 'R432 denied' })
    await runDelete(c, makeHelper())
    expect(c._errors.some(m => m.indexOf('R432 denied') >= 0)).toBe(true)
    expect(c.selected.value?.id).toBe(FAKE)
    expect(c.queries.value.map((x: any) => x.id)).toContain(FAKE)
  })
  it('非对象 null 失败：红toast缺省文案、不移除行', async () => {
    const c = makeCtx(null)
    await runDelete(c, makeHelper())
    expect(c._errors.length).toBeGreaterThan(0)
    expect(c.selected.value?.id).toBe(FAKE)
    expect(c.queries.value.map((x: any) => x.id)).toContain(FAKE)
  })
  it('type:success 成功：清selected、移除行、无错误toast', async () => {
    const c = makeCtx({ type: 'success', data: { id: FAKE, deleted: true } })
    await runDelete(c, makeHelper())
    expect(c.selected.value).toBeNull()
    expect(c.queries.value.map((x: any) => x.id)).not.toContain(FAKE)
    expect(c.queries.value.map((x: any) => x.id)).toContain('OTHER')
    expect(c._errors.length).toBe(0)
  })
  it('type缺省 success 非 false 成功：移除行', async () => {
    const c = makeCtx({ data: { id: FAKE } })
    await runDelete(c, makeHelper())
    expect(c.queries.value.map((x: any) => x.id)).not.toContain(FAKE)
  })
  it('confirm 返回 false：零写、不清selected、不移除行', async () => {
    const c = makeCtx({ type: 'success', data: {} }, false)
    await runDelete(c, makeHelper())
    expect(c._writes.length).toBe(0)
    expect(c.selected.value?.id).toBe(FAKE)
    expect(c.queries.value.map((x: any) => x.id)).toContain(FAKE)
  })
  it('非2xx reject 走 catch：红toast、不移除行', async () => {
    const c = makeCtx(null)
    c.api.delete = async (url: string) => { c._writes.push({ method: 'DELETE', url }); throw new Error('HTTP 403') }
    await runDelete(c, makeHelper())
    expect(c._errors.some(m => m.indexOf('HTTP 403') >= 0)).toBe(true)
    expect(c.queries.value.map((x: any) => x.id)).toContain(FAKE)
  })
  it('DELETE 端点正确', async () => {
    const c = makeCtx({ type: 'success', data: {} })
    await runDelete(c, makeHelper())
    expect(c._writes[0].method).toBe('DELETE')
    expect(c._writes[0].url).toBe('/api/query/assemble/designer/delete/' + FAKE)
  })
})
