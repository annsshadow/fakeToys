// @vitest-environment node
// R393 PC QueryManagerDeep 写链失败信封修复契约：从源码提取真实函数体做 new Function 真执行注入桩。
// 防 TS7006：所有注入桩回调参数均显式 : any/: string 类型标注。
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

const SRC = resolve(__dirname, 'QueryManagerDeep.vue')
const source = readFileSync(SRC, 'utf8')

function bodyOf(sig: string): string | null {
  const start = source.indexOf(sig)
  if (start < 0) return null
  const open = source.indexOf('{', start)
  if (open < 0) return null
  let depth = 0
  for (let i = open; i < source.length; i++) {
    const ch = source[i]
    if (ch === '{') depth++
    else if (ch === '}') { depth--; if (depth === 0) return source.slice(open, i + 1) }
  }
  return null
}

function stripTs(code: string): string {
  return code
    .replace(/\s+as\s+(any|string|number|boolean|unknown|never|Response|ViewItem)\b/g, '')
    .replace(/\b(\w+):\s*(any|string|number|boolean|unknown|never|Response|ViewItem)\b/g, '$1')
}

// helper 源字符串：父版缺失时返回恒 false 的桩，保证 RED
function helperSource(): string {
  const b = bodyOf('function queryManagerWriteSucceeded')
  return b ? stripTs(b) : '{}'
}

type Fn = (arg: string) => Promise<void>
type Ctx = { calls: string[]; writes: number; fn: Fn }
function makeFn(name: string, param: string, resp: any, promptVals: string[], confirmOk: boolean): Ctx {
  const body = bodyOf('async function ' + name + '(')
  if (!body) throw new Error('function not found: ' + name)
  const calls: string[] = []
  let writes = 0
  const api = new Proxy({}, { get: (_t: any, m: string) => async (...args: any[]) => {
    void m; void args; writes++; if (resp instanceof Error) throw resp; return resp
  } })
  const toast = {
    success: (m: string) => calls.push('S:' + m),
    error: (m: string) => calls.push('E:' + m),
    warning: (m: string) => calls.push('W:' + m),
  }
  let pi = 0
  const prompt = (_q: string, _d?: string): string => promptVals[pi++] ?? 'X'
  const confirmMsg = async (_q: string): Promise<boolean> => confirmOk
  // 字符串拼接避免模板字面量在 run_code/JS 层被提前插值
  const src = '(function queryManagerWriteSucceeded(resp) ' + helperSource() + ')';
  const helper = new Function('return ' + src)() as (r: any) => boolean
  const fnSrc = '(async function(' + param + '){' + stripTs(body) + '})'
  const fn = new Function('api','toast','prompt','confirmMsg','queryManagerWriteSucceeded','return ' + fnSrc)(
    api, toast, prompt, confirmMsg, helper) as Fn
  return { calls, get writes() { return writes }, fn }
}

const WRITE_FNS: [string, string, string][] = [
  ['qdCreate', 'kind', 'query'],
  ['qdEdit', 'kind', 'query'],
  ['qdDelete', 'kind', 'query'],
  ['qdPerm', 'kind', 'query'],
  ['qdMisc', 'op', 'search'],
  ['qdWrite2', 'op', 'statEdit'],
  ['qdDeleteDesigner', 'op', 'designer'],
  ['qdInput', 'op', 'compare'],
  ['qdTableRow', 'op', 'insert'],
]
function errText(calls: string[]): string { return calls.filter(c => c.startsWith('E:')).join('|') }
function okText(calls: string[]): string { return calls.filter(c => c.startsWith('S:')).join('|') }

describe('R393 queryManagerWriteSucceeded helper 真执行', () => {
  const hs = helperSource()
  const h = new Function('return (function(resp) ' + hs + ')')() as (r: any) => boolean
  it('type error -> false', () => { expect(h({ type: 'error', message: 'x' })).toBe(false) })
  it('success false -> false', () => { expect(h({ success: false })).toBe(false) })
  it('type success no bool -> true', () => { expect(h({ type: 'success' })).toBe(true) })
  it('type missing success not false -> true', () => { expect(h({ data: { count: 0 } })).toBe(true) })
  it('null undefined string number -> false', () => {
    expect(h(undefined)).toBe(false); expect(h(null)).toBe(false)
    expect(h('success')).toBe(false); expect(h(42)).toBe(false)
  })
  it('idempotent success deleted:0 still true (no strict bool)', () => {
    expect(h({ type: 'success', data: { deleted: 0, count: 0 } })).toBe(true)
  })
})

describe('R393 九个真实写函数真执行：失败信封不得弹成功', () => {
  for (const [name, param, arg] of WRITE_FNS) {
    it(name + ' type:error 红toast透传message无成功', async () => {
      const ctx = makeFn(name, param, { type: 'error', message: 'R393后端拒绝' }, ['X'], true)
      await ctx.fn(arg)
      expect(errText(ctx.calls)).toContain('R393后端拒绝')
      expect(okText(ctx.calls)).toBe('')
    })
    it(name + ' success:false 红toast无成功', async () => {
      const ctx = makeFn(name, param, { success: false, message: 'R393后端拒绝2' }, ['X'], true)
      await ctx.fn(arg)
      expect(errText(ctx.calls)).toContain('R393后端拒绝2')
      expect(okText(ctx.calls)).toBe('')
    })
    it(name + ' type:success 真成功仍弹成功无错误', async () => {
      const ctx = makeFn(name, param, { type: 'success', data: { count: 1 } }, ['X'], true)
      await ctx.fn(arg)
      expect(okText(ctx.calls)).not.toBe('')
      expect(errText(ctx.calls)).toBe('')
    })
    it(name + ' 幂等 success deleted:0 弹成功', async () => {
      const ctx = makeFn(name, param, { type: 'success', data: { deleted: 0 } }, ['X'], true)
      await ctx.fn(arg)
      expect(okText(ctx.calls)).not.toBe('')
      expect(errText(ctx.calls)).toBe('')
    })
  }
})

describe('R393 异常处理与确认守卫', () => {
  it('qdDelete confirm取消零写无toast', async () => {
    const ctx = makeFn('qdDelete', 'kind', { type: 'success' }, ['X'], false)
    await ctx.fn('query')
    expect(ctx.writes).toBe(0)
    expect(ctx.calls).toEqual([])
  })
  it('qdWrite2 statDelete confirm取消零写无toast', async () => {
    const ctx = makeFn('qdWrite2', 'op', { type: 'success' }, ['X'], false)
    await ctx.fn('statDelete')
    expect(ctx.writes).toBe(0)
    expect(ctx.calls).toEqual([])
  })
  it('qdDeleteDesigner confirm取消零写无toast', async () => {
    const ctx = makeFn('qdDeleteDesigner', 'op', { type: 'success' }, ['X'], false)
    await ctx.fn('designer')
    expect(ctx.writes).toBe(0)
    expect(ctx.calls).toEqual([])
  })
  it('500 非2xx catch toast.error 无成功', async () => {
    const ctx = makeFn('qdCreate', 'kind', new Error('HTTP 500'), ['X'], true)
    await ctx.fn('query')
    expect(errText(ctx.calls)).toContain('HTTP 500')
    expect(okText(ctx.calls)).toBe('')
  })
  it('403 非2xx catch toast.error 无成功', async () => {
    const ctx = makeFn('qdMisc', 'op', new Error('HTTP 403'), ['X'], true)
    await ctx.fn('search')
    expect(errText(ctx.calls)).toContain('HTTP 403')
    expect(okText(ctx.calls)).toBe('')
  })
})
