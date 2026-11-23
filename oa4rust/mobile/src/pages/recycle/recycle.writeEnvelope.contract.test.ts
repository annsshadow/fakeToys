// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R424 契约：H5 回收站彻底删除写链 discardResponse 吞信封 P1。
 * 真父 77abd74c0：recycleApi.remove 带 discardResponse:true → SDK 吞掉 200 后端信封 →
 * recycle.vue remove() 无条件 toast 已删除；后端 200+type:error(file not in recycle bin) 被误报成功。
 * 本契约源码级函数体真执行（new AsyncFn），不引入新依赖；真父应 RED，修复后 GREEN。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const pageSrc = readFileSync(resolve(import.meta.dirname, './recycle.vue'), 'utf8')
const idxSrc = readFileSync(resolve(import.meta.dirname, '../../services/index.ts'), 'utf8')

function bodyOf(source: string, signature: string): string {
  const start = source.indexOf(signature)
  if (start < 0) throw new Error('missing signature: ' + signature)
  const open = source.indexOf('{', start)
  let depth = 0
  for (let i = open; i < source.length; i++) {
    if (source[i] === '{') depth++
    else if (source[i] === '}') {
      depth--
      if (depth === 0) return source.slice(open + 1, i)
    }
  }
  throw new Error('unbalanced braces: ' + signature)
}

const helperFound = pageSrc.includes('function recycleDeleteSucceeded(')
const helperRaw = helperFound ? bodyOf(pageSrc, 'function recycleDeleteSucceeded(') : ''
const strip = (s: string): string => s.replace(/:\s*any\b/g, '').replace(/:\s*boolean\b/g, '').replace(/<boolean>/g, '')
const AsyncFn = Object.getPrototypeOf(async function () {}).constructor
const Fn = Object.getPrototypeOf(function () {}).constructor

function makeHelper(): (r: unknown) => boolean {
  if (!helperFound) return () => false
  return new Fn('resp', strip(helperRaw)) as (r: unknown) => boolean
}

const removeBody = strip(bodyOf(pageSrc, 'async function remove('))

interface Toast { title: string; icon: string }

function runRemove(envelope: unknown, opts: { confirm?: boolean; reject?: Error } = {}) {
  const toasts: Toast[] = []
  let writes = 0
  let loadCount = 0
  let lastPreviousRows: unknown[] | null = null
  const rows = { value: [{ id: 'FAKE-R424', name: '测试项' }, { id: 'OTHER', name: '保留项' }] }
  const uniStub = {
    showToast: (o: Toast): void => { toasts.push(o) },
    showModal: (o: { success: (m: { confirm: boolean }) => void; fail: () => void }): void => {
      if (opts.confirm === false) o.fail()
      else o.success({ confirm: true })
    },
  }
  const busy = { value: false }
  const r = { id: 'FAKE-R424', name: '测试项' }
  const recycleApiStub = {
    remove: async (id: string): Promise<unknown> => {
      writes++
      void id
      if (opts.reject) throw opts.reject
      return envelope
    },
  }
  const loadStub = async (previousRows?: unknown[]): Promise<void> => {
    loadCount++
    lastPreviousRows = previousRows ?? null
  }
  const nameOfStub = (row: { name?: string }): string => String(row.name ?? '未命名')
  const invoke = new AsyncFn('uni', 'busy', 'r', 'recycleApi', 'recycleDeleteSucceeded', 'load', 'rows', 'nameOf', removeBody)
  return {
    toasts,
    busy,
    rows,
    invoke: (): Promise<void> => invoke(uniStub, busy, r, recycleApiStub, makeHelper(), loadStub, rows, nameOfStub),
    writes: (): number => writes,
    loadCount: (): number => loadCount,
    previousRows: (): unknown[] | null => lastPreviousRows,
  }
}

describe('R424 H5 回收站彻底删除写链信封契约', () => {
  it('index.ts recycleApi.remove 去 discardResponse 且端点/DELETE 不变', () => {
    const start = idxSrc.indexOf('remove: (id: string)')
    expect(start, 'remove 定义').toBeGreaterThan(-1)
    const seg = idxSrc.slice(start, start + 200)
    expect(seg).toContain('/api/recycle/delete/')
    expect(seg).not.toContain('discardResponse: true')
  })

  it('recycleApi.empty 仍保留 discardResponse（零改见证）', () => {
    const start = idxSrc.indexOf('empty: () => mapi.delete')
    expect(start, 'empty 定义').toBeGreaterThan(-1)
    const seg = idxSrc.slice(start, start + 160)
    expect(seg).toContain('discardResponse: true')
  })

  it('新增 recycleDeleteSucceeded helper（type 为主不强求 bool）', () => {
    expect(helperFound, 'recycle.vue 应含 recycleDeleteSucceeded').toBe(true)
  })

  it('helper: type:error 判失败', () => {
    expect(helperFound).toBe(true)
    expect(makeHelper()({ type: 'error', message: 'file not in recycle bin', data: {} })).toBe(false)
  })

  it('helper: success:false 判失败', () => {
    expect(helperFound).toBe(true)
    expect(makeHelper()({ success: false, message: '拒绝' })).toBe(false)
  })

  it('helper: type:success 判成功', () => {
    expect(helperFound).toBe(true)
    expect(makeHelper()({ type: 'success', data: { id: 'x', deleted: true } })).toBe(true)
  })

  it('helper: type 缺省且 success 非 false 判成功', () => {
    expect(helperFound).toBe(true)
    expect(makeHelper()({ data: { id: 'x' } })).toBe(true)
  })

  it('helper: 非对象/null/undefined 判失败', () => {
    expect(helperFound).toBe(true)
    const h = makeHelper()
    expect(h(null)).toBe(false)
    expect(h(undefined)).toBe(false)
    expect(h(42)).toBe(false)
  })

  it('helper: 不强求权威 bool，deleted:0 仍判成功', () => {
    expect(helperFound).toBe(true)
    expect(makeHelper()({ type: 'success', data: { id: 'x', deleted: 0 } })).toBe(true)
  })

  it('remove: 200+type:error(file not in recycle bin) 错误 toast 透传不弹已删除不刷新', async () => {
    const c = runRemove({ type: 'error', message: 'file not in recycle bin', data: {} })
    await c.invoke()
    const titles = c.toasts.map((t) => t.title)
    expect(titles).toContain('file not in recycle bin')
    expect(titles).not.toContain('已删除')
    expect(c.loadCount()).toBe(0)
  })

  it('remove: success:false 错误 toast 透传不弹已删除不刷新', async () => {
    const c = runRemove({ success: false, message: 'R424 denied' })
    await c.invoke()
    const titles = c.toasts.map((t) => t.title)
    expect(titles).toContain('R424 denied')
    expect(titles).not.toContain('已删除')
    expect(c.loadCount()).toBe(0)
  })

  it('remove: 非对象/null 响应判失败，弹缺省错误文案且不刷新', async () => {
    const c = runRemove(null)
    await c.invoke()
    const titles = c.toasts.map((t) => t.title)
    expect(titles).toContain('删除失败，请刷新后重试')
    expect(titles).not.toContain('已删除')
    expect(c.loadCount()).toBe(0)
  })

  it('remove: type:success 弹已删除并 load 乐观排除已删行', async () => {
    const c = runRemove({ type: 'success', data: { id: 'FAKE-R424', deleted: true } })
    await c.invoke()
    const titles = c.toasts.map((t) => t.title)
    expect(titles).toContain('已删除')
    expect(c.loadCount()).toBe(1)
    const prev = c.previousRows() as Array<{ id: string }> | null
    expect(prev).not.toBeNull()
    expect(prev!.map((x) => x.id)).toEqual(['OTHER'])
  })

  it('remove: type 缺省 success 非 false 仍判成功并刷新', async () => {
    const c = runRemove({ data: { id: 'FAKE-R424' } })
    await c.invoke()
    expect(c.toasts.map((t) => t.title)).toContain('已删除')
    expect(c.loadCount()).toBe(1)
  })

  it('remove: 非2xx reject 走 catch 显示错误提示且不刷新', async () => {
    const c = runRemove(undefined, { reject: new Error('HTTP 403') })
    await c.invoke()
    expect(c.toasts.map((t) => t.title)).toContain('HTTP 403')
    expect(c.toasts.map((t) => t.title)).not.toContain('已删除')
    expect(c.loadCount()).toBe(0)
  })

  it('remove: showModal 取消零写、无 toast、不刷新、busy 复位', async () => {
    const c = runRemove({ type: 'success', data: {} }, { confirm: false })
    await c.invoke()
    expect(c.writes()).toBe(0)
    expect(c.toasts).toHaveLength(0)
    expect(c.loadCount()).toBe(0)
    expect(c.busy.value).toBe(false)
  })
})
