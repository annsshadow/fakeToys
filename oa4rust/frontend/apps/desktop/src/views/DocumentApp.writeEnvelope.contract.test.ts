import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'

const src = readFileSync(new URL('./DocumentApp.vue', import.meta.url), 'utf-8')

function bodyOf(sig: string): string {
  const i = src.indexOf(sig)
  if (i < 0) return ''
  let depth = 0
  let started = false
  for (let j = i; j < src.length; j++) {
    const ch = src[j]
    if (ch === '{') { depth++; started = true }
    else if (ch === '}') { depth--; if (started && depth === 0) return src.slice(i, j + 1) }
  }
  return ''
}

function stripTs(s: string): string {
  return s
    .replace(/\(([A-Za-z_$][\w$]*)\s*:\s*[A-Za-z_$][\w$<>|\[\], ]*\)/g, '($1)')
    .replace(/\)\s*:\s*[A-Za-z_$][\w$<>|\[\], ]*\s*\{/g, '){')
    .replace(/\b(const|let|var)\s+([A-Za-z_$][\w$]*)\s*:\s*[A-Za-z_$][\w$<>|\[\], ]*\s*=/g, '$1 $2 =')
    .replace(/\s+as\s+[A-Za-z_$][\w$<>|\[\], ]*/g, '')
}

function makeSucceeded(): (r: any) => boolean {
  const helperSrc = bodyOf('function documentWriteSucceeded(resp:any)')
  if (!helperSrc) return () => false
  return new Function(stripTs(helperSrc) + '\nreturn documentWriteSucceeded;')() as (r: any) => boolean
}

const succeeded = makeSucceeded()

type Case = { name: string; sig: string; item?: any; ok: string; kind?: string }
const CASES: Case[] = [
  { name: 'onCipherPublish', sig: 'async function onCipherPublish(item: DocItem)', item: { id: 'D1', title: 'Doc1' }, ok: '密文已发布' },
  { name: 'onCreate', sig: 'async function onCreate()', ok: 'CREATE', kind: 'create' },
  { name: 'onDelete', sig: 'async function onDelete(item: DocItem)', item: { id: 'D1', title: 'Doc1' }, ok: 'DELETE', kind: 'delete' },
  { name: 'onCommend', sig: 'async function onCommend(item: DocItem)', item: { id: 'D1' }, ok: '已推荐' },
  { name: 'onTop', sig: 'async function onTop(item: DocItem)', item: { id: 'D1' }, ok: '已置顶' },
  { name: 'onPublish', sig: 'async function onPublish(item: DocItem)', item: { id: 'D1' }, ok: '已发布', kind: 'refresh' },
  { name: 'onUncommend', sig: 'async function onUncommend(item: DocItem)', item: { id: 'D1' }, ok: '已取消推荐' },
  { name: 'onUnTop', sig: 'async function onUnTop(item: DocItem)', item: { id: 'D1' }, ok: '已取消置顶' },
  { name: 'onPublishCancel', sig: 'async function onPublishCancel(item: DocItem)', item: { id: 'D1' }, ok: '已撤销发布', kind: 'refresh' },
  { name: 'onNotify', sig: 'async function onNotify(item: DocItem)', item: { id: 'D1' }, ok: '已发送通知' },
  { name: 'onViewRecord', sig: 'async function onViewRecord(item: DocItem)', item: { id: 'D1' }, ok: '已记录浏览' },
]

function makeCtx(resp: any, opts: any = {}) {
  const calls: any = { api: [], toastOk: [], toastErr: [], doSearch: 0, confirm: null }
  const api = new Proxy({}, {
    get(_t: any, method: string) {
      return (...args: any[]) => {
        calls.api.push({ method, args })
        return resp instanceof Error ? Promise.reject(resp) : Promise.resolve(resp)
      }
    },
  })
  const toast = {
    success: (m: any) => { calls.toastOk.push(String(m)) },
    info: (m: any) => { calls.toastOk.push(String(m)) },
    error: (m: any) => { calls.toastErr.push(String(m)) },
  }
  const doSearch = () => { calls.doSearch++ }
  const confirmMsg = async (m: string) => { calls.confirm = m; return !!opts.confirmed }
  const prompt = () => (opts.promptText === undefined ? 'cipher-text' : opts.promptText)
  const creating = { value: false }
  const showCreate = { value: opts.showCreate === undefined ? true : opts.showCreate }
  const createForm = { value: { title: opts.title === undefined ? 'T' : opts.title, content: '' } }
  const items = { value: [{ id: 'D1', title: 'Doc1' }, { id: 'D2', title: 'Doc2' }] }
  const deletingId = { value: '' }
  return { calls, api, toast, doSearch, confirmMsg, prompt, creating, showCreate, createForm, items, deletingId }
}

function compile(c: Case, ctx: any): (item?: any) => Promise<void> {
  const body = bodyOf(c.sig)
  if (!body) throw new Error('function body not found: ' + c.sig)
  const names = ['api','toast','doSearch','confirmMsg','prompt','creating','showCreate','createForm','items','deletingId','documentWriteSucceeded']
  const args = [ctx.api, ctx.toast, ctx.doSearch, ctx.confirmMsg, ctx.prompt, ctx.creating, ctx.showCreate, ctx.createForm, ctx.items, ctx.deletingId, succeeded]
  return new Function(...names, stripTs(body) + '\nreturn ' + c.name + ';')(...args) as (item?: any) => Promise<void>
}

describe('R378 DocumentApp write envelope (real execution)', () => {
  it('helper: type:error fails', () => { expect(succeeded({ type: 'error', message: 'x' })).toBe(false) })
  it('helper: success:false fails', () => { expect(succeeded({ success: false })).toBe(false) })
  it('helper: success with isTop:false still passes (no bool requirement)', () => { expect(succeeded({ type: 'success', data: { isTop: false } })).toBe(true) })
  it('helper: success with removed:0 still passes', () => { expect(succeeded({ type: 'success', data: { removed: 0 } })).toBe(true) })
  it('helper: missing type with data passes', () => { expect(succeeded({ data: { id: 'x' } })).toBe(true) })
  it('helper: null fails', () => { expect(succeeded(null)).toBe(false) })

  for (const c of CASES) {
    it(c.name + ': type:error -> error toast, no success, no side effect', async () => {
      const ctx = makeCtx({ type: 'error', message: 'R378 backend error' }, { confirmed: true })
      const fn = compile(c, ctx)
      await fn(c.item)
      expect(ctx.calls.toastOk.length).toBe(0)
      expect(ctx.calls.toastErr.join(' ')).toContain('R378 backend error')
      if (c.kind === 'create') {
        expect(ctx.showCreate.value).toBe(true)
        expect(ctx.calls.doSearch).toBe(0)
      } else if (c.kind === 'delete') {
        expect(ctx.items.value.length).toBe(2)
      } else if (c.kind === 'refresh') {
        expect(ctx.calls.doSearch).toBe(0)
      }
    })

    it(c.name + ': type:success -> success toast + side effect', async () => {
      const ctx = makeCtx({ type: 'success', data: { id: 'D1' } }, { confirmed: true })
      const fn = compile(c, ctx)
      await fn(c.item)
      expect(ctx.calls.toastErr.length).toBe(0)
      if (c.kind === 'create') {
        expect(ctx.showCreate.value).toBe(false)
        expect(ctx.calls.doSearch).toBeGreaterThanOrEqual(1)
      } else if (c.kind === 'delete') {
        expect(ctx.items.value.length).toBe(1)
      } else {
        expect(ctx.calls.toastOk.join(' ')).toContain(c.ok)
        if (c.kind === 'refresh') expect(ctx.calls.doSearch).toBeGreaterThanOrEqual(1)
      }
    })
  }

  it('onDelete: confirm cancel -> zero write', async () => {
    const ctx = makeCtx({ type: 'success' }, { confirmed: false })
    const fn = compile(CASES.filter(x => x.name === 'onDelete')[0], ctx)
    await fn({ id: 'D1' })
    expect(ctx.calls.api.length).toBe(0)
    expect(ctx.items.value.length).toBe(2)
  })

  it('onCommend: 500 catch -> error toast', async () => {
    const ctx = makeCtx(new Error('HTTP 500'))
    const fn = compile(CASES.filter(x => x.name === 'onCommend')[0], ctx)
    await fn({ id: 'D1' })
    expect(ctx.calls.toastOk.length).toBe(0)
    expect(ctx.calls.toastErr.join(' ')).toContain('HTTP 500')
  })

  it('onCommend: 403 catch -> error toast', async () => {
    const ctx = makeCtx(new Error('没有权限执行该操作'))
    const fn = compile(CASES.filter(x => x.name === 'onCommend')[0], ctx)
    await fn({ id: 'D1' })
    expect(ctx.calls.toastOk.length).toBe(0)
    expect(ctx.calls.toastErr.join(' ')).toContain('没有权限')
  })
})
