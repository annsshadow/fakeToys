// @vitest-environment node
// R391 PC QueryViewApp 写链失败信封修复契约：源码级提取真实函数体 + new Function/new AsyncFunction 真执行注入桩。
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

const SRC = resolve(__dirname, 'QueryViewApp.vue')
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
    else if (ch === '}') {
      depth--
      if (depth === 0) return source.slice(open, i + 1)
    }
  }
  return null
}

function stripTs(code: string): string {
  return code
    .replace(/\s+as\s+(any|string|number|boolean|unknown|never|Response|ViewItem)\b/g, '')
    .replace(/\b(\w+):\s*(any|string|number|boolean|unknown|never|Response|ViewItem)\b/g, '$1')
}

function makeHelper(): (r: any) => boolean {
  const body = bodyOf('function queryViewWriteSucceeded')
  if (!body) return () => false
  const src = '(function(resp)' + stripTs(body) + ')'
  return new Function('return ' + src)()
}

type Ctx = { calls: any[]; toasts: any[]; api: any; toast: any; confirmMsg: any; prompt: any }

function makeCtx(mode: string, cancel: boolean): Ctx {
  const calls: any[] = []
  const toasts: any[] = []
  const respond = (method: string, url: string) => {
    calls.push({ method, url })
    if (mode === 'typeError') return Promise.resolve({ type: 'error', message: 'R391后端拒绝:type error' })
    if (mode === 'successFalse') return Promise.resolve({ success: false, message: 'R391后端拒绝:success false' })
    if (mode === 'http500') return Promise.reject(new Error('HTTP 500: Internal Server Error'))
    if (mode === 'http403') return Promise.reject(new Error('没有权限执行该操作'))
    return Promise.resolve({ type: 'success', data: { count: 0 } })
  }
  const api = new Proxy({}, { get: (_t, method: string) => (url: string, body?: any) => respond(method, url) })
  const toast = {
    success: (m: string) => { toasts.push({ level: 'success', message: m }) },
    error: (m: string) => { toasts.push({ level: 'error', message: m }) },
    info: (m: string) => { toasts.push({ level: 'info', message: m }) },
  }
  const confirmMsg = () => Promise.resolve(!cancel)
  const prompt = (msg: string) => (typeof msg === 'string' && msg.indexOf('JSON') >= 0 ? '{"name":"R391"}' : 'R391TEST')
  return { calls, toasts, api, toast, confirmMsg, prompt }
}

async function runFn(name: string, op: string, ctx: Ctx, helper: (r: any) => boolean) {
  const body = bodyOf('async function ' + name)
  if (!body) throw new Error('function not found: ' + name)
  const factory = new Function('api', 'toast', 'confirmMsg', 'prompt', 'queryViewWriteSucceeded', 'encodeURIComponent',
    'return (async function(op)' + stripTs(body) + ')')
  const fn = factory(ctx.api, ctx.toast, ctx.confirmMsg, ctx.prompt, helper, encodeURIComponent)
  await fn(op)
}

function hasSuccess(ctx: Ctx) { return ctx.toasts.some(t => t.level === 'success') }
function hasError(ctx: Ctx) { return ctx.toasts.some(t => t.level === 'error') }
function errorText(ctx: Ctx) { return ctx.toasts.filter(t => t.level === 'error').map(t => t.message).join('|') }

describe('R391 queryViewWriteSucceeded helper', () => {
  const helper = makeHelper()
  it('type error -> false', () => { expect(helper({ type: 'error', message: 'x' })).toBe(false) })
  it('success false -> false', () => { expect(helper({ success: false })).toBe(false) })
  it('type success no bool -> true', () => { expect(helper({ type: 'success' })).toBe(true) })
  it('type missing success not false -> true', () => { expect(helper({ data: { count: 0 } })).toBe(true) })
  it('null undefined string number -> false', () => {
    expect(helper(undefined)).toBe(false)
    expect(helper(null)).toBe(false)
    expect(helper('success')).toBe(false)
    expect(helper(42)).toBe(false)
  })
  it('type success with flag false still true (no strict bool)', () => {
    expect(helper({ type: 'success', data: { deleted: false, resumed: false } })).toBe(true)
  })
})

describe('R391 qvWrite write envelope', () => {
  const helper = makeHelper()
  it('moreLikeThis type error -> error toast no success', async () => {
    const ctx = makeCtx('typeError', false)
    await runFn('qvWrite', 'moreLikeThis', ctx, helper)
    expect(hasError(ctx)).toBe(true)
    expect(errorText(ctx)).toContain('R391后端拒绝')
    expect(hasSuccess(ctx)).toBe(false)
  })
  it('moreLikeThis true success -> success toast', async () => {
    const ctx = makeCtx('trueSuccess', false)
    await runFn('qvWrite', 'moreLikeThis', ctx, helper)
    expect(hasSuccess(ctx)).toBe(true)
    expect(hasError(ctx)).toBe(false)
  })
  it('rowDelete confirm cancel -> zero write', async () => {
    const ctx = makeCtx('trueSuccess', true)
    await runFn('qvWrite', 'rowDelete', ctx, helper)
    expect(ctx.calls.length).toBe(0)
    expect(hasSuccess(ctx)).toBe(false)
  })
  it('rowDelete type error after confirm -> error toast no success', async () => {
    const ctx = makeCtx('typeError', false)
    await runFn('qvWrite', 'rowDelete', ctx, helper)
    expect(ctx.calls.length).toBe(1)
    expect(hasError(ctx)).toBe(true)
    expect(errorText(ctx)).toContain('R391后端拒绝')
    expect(hasSuccess(ctx)).toBe(false)
  })
})

describe('R391 qvMore write envelope', () => {
  const helper = makeHelper()
  it('stmtExec type error -> error toast no success', async () => {
    const ctx = makeCtx('typeError', false)
    await runFn('qvMore', 'stmtExec', ctx, helper)
    expect(hasError(ctx)).toBe(true)
    expect(errorText(ctx)).toContain('R391后端拒绝')
    expect(hasSuccess(ctx)).toBe(false)
  })
  it('stmtExec true success -> success toast', async () => {
    const ctx = makeCtx('trueSuccess', false)
    await runFn('qvMore', 'stmtExec', ctx, helper)
    expect(hasSuccess(ctx)).toBe(true)
  })
})

describe('R391 qvRows write envelope', () => {
  const helper = makeHelper()
  it('rowDelete type error -> error toast no success', async () => {
    const ctx = makeCtx('typeError', false)
    await runFn('qvRows', 'rowDelete', ctx, helper)
    expect(ctx.calls.length).toBe(1)
    expect(hasError(ctx)).toBe(true)
    expect(errorText(ctx)).toContain('R391后端拒绝')
    expect(hasSuccess(ctx)).toBe(false)
  })
  it('rowDelete confirm cancel -> zero write', async () => {
    const ctx = makeCtx('trueSuccess', true)
    await runFn('qvRows', 'rowDelete', ctx, helper)
    expect(ctx.calls.length).toBe(0)
    expect(hasSuccess(ctx)).toBe(false)
  })
  it('rowInsert true success -> success toast', async () => {
    const ctx = makeCtx('trueSuccess', false)
    await runFn('qvRows', 'rowInsert', ctx, helper)
    expect(hasSuccess(ctx)).toBe(true)
  })
})

describe('R391 qvDesignerRows write envelope', () => {
  const helper = makeHelper()
  it('update type error -> error toast no success', async () => {
    const ctx = makeCtx('typeError', false)
    await runFn('qvDesignerRows', 'update', ctx, helper)
    expect(hasError(ctx)).toBe(true)
    expect(errorText(ctx)).toContain('R391后端拒绝')
    expect(hasSuccess(ctx)).toBe(false)
  })
  it('update true success -> branch success toast', async () => {
    const ctx = makeCtx('trueSuccess', false)
    await runFn('qvDesignerRows', 'update', ctx, helper)
    expect(hasSuccess(ctx)).toBe(true)
    expect(ctx.toasts.some(t => t.level === 'success' && t.message.indexOf('设计器表行已更新') >= 0)).toBe(true)
  })
  it('delete confirm cancel -> zero write', async () => {
    const ctx = makeCtx('trueSuccess', true)
    await runFn('qvDesignerRows', 'delete', ctx, helper)
    expect(ctx.calls.length).toBe(0)
  })
  it('delete type error -> error toast no success', async () => {
    const ctx = makeCtx('typeError', false)
    await runFn('qvDesignerRows', 'delete', ctx, helper)
    expect(ctx.calls.length).toBe(1)
    expect(hasError(ctx)).toBe(true)
    expect(errorText(ctx)).toContain('R391后端拒绝')
    expect(hasSuccess(ctx)).toBe(false)
  })
  it('build type error -> error toast no success', async () => {
    const ctx = makeCtx('typeError', false)
    await runFn('qvDesignerRows', 'build', ctx, helper)
    expect(hasError(ctx)).toBe(true)
    expect(errorText(ctx)).toContain('R391后端拒绝')
    expect(hasSuccess(ctx)).toBe(false)
  })
})
