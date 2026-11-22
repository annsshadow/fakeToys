// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R352 契约：H5 search.vue 双来源信封解包。
 * R351 审计坐实：全局 data={designList,viewList}、论坛 data={data:[...]} 分组信封，
 * 页面用 (g.data ?? []).map 直接遍历对象 → 任何关键词崩溃为 .map is not a function。
 * 源码级 bodyOf 提取 doSearch + new AsyncFn 真执行注入桩，剥离 TS 断言，不引入新依赖。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, './search.vue'), 'utf8')

function bodyOf(source: string, signature: string): string {
  const start = source.indexOf(signature)
  if (start < 0) throw new Error('未找到函数: ' + signature)
  const open = source.indexOf('{', start)
  let depth = 0
  for (let i = open; i < source.length; i++) {
    if (source[i] === '{') depth++
    else if (source[i] === '}') {
      depth--
      if (depth === 0) return source.slice(open + 1, i)
    }
  }
  throw new Error('花括号未配对: ' + signature)
}

// 剥离 TS 断言/标注使函数体可被 new AsyncFn 真执行（纯字符串替换，无正则转义）
const doBody = bodyOf(src, 'async function doSearch()')
  .replace(/ as \{[^}]*\}/g, '')
  .replace(/ as Row\[\]/g, '')
  .replace(/const (\w+): Row\[\]/g, 'const $1')
  .replace(/: string\[\]/g, '')

const AsyncFn = Object.getPrototypeOf(async function () {}).constructor

interface Ctx {
  keyword: { value: string }
  loading: { value: boolean }
  searched: { value: boolean }
  searchError: { value: string }
  rows: { value: Array<Record<string, unknown>> }
  calls?: string[]
}

function makeCtx(kw: string): Ctx {
  return {
    keyword: { value: kw },
    loading: { value: false },
    searched: { value: false },
    searchError: { value: '' },
    rows: { value: [] },
  }
}

function loadErr(label: string, error: unknown): string {
  const msg = (error as { message?: string })?.message ?? '未知错误'
  return label + '：' + msg
}

async function runSearch(gResp: unknown, bResp: unknown, kw: string): Promise<Ctx> {
  const ctx = makeCtx(kw)
  const calls: string[] = []
  const searchApi = {
    global: async function (k: string): Promise<unknown> { calls.push('G:' + k); return gResp },
    bbsSubject: async function (k: string): Promise<unknown> { calls.push('B:' + k); return bResp },
  }
  const fn = new AsyncFn('keyword', 'loading', 'searched', 'searchError', 'rows', 'searchApi', 'describeLoadError', doBody)
  await fn(ctx.keyword, ctx.loading, ctx.searched, ctx.searchError, ctx.rows, searchApi, loadErr)
  Object.assign(ctx, { calls })
  return ctx
}

describe('R352 search.vue 双来源信封解包契约', () => {
  it('全局双层信封 designList+viewList 与论坛内层 data 解包为行并加 _from', async () => {
    const g = { data: { key: 'a', designList: [{ id: 'g1' }, { id: 'g2' }, { id: 'g3' }], viewList: [{ id: 'v1' }] } }
    const b = { data: { total: 2, data: [{ id: 'b1' }, { id: 'b2' }] } }
    const ctx = await runSearch(g, b, 'a')
    expect(ctx.rows.value.length, '3 design + 1 view + 2 bbs').toBe(6)
    expect(ctx.rows.value.filter((r) => r._from === '全局').length).toBe(4)
    expect(ctx.rows.value.filter((r) => r._from === '论坛').length).toBe(2)
    expect(ctx.searchError.value).toBe('')
  })

  it('全局 designList/viewList 缺失或坏数据不崩溃，论坛单层数组兼容', async () => {
    const ctx = await runSearch({ data: { designList: null, viewList: 'bad' } }, { data: [{ id: 'b1' }] }, 'x')
    expect(ctx.rows.value.length).toBe(1)
    expect(ctx.rows.value[0].id).toBe('b1')
    expect(ctx.searchError.value).toBe('')
  })

  it('两来源各自 try/catch：全局失败不阻断论坛，失败文案可见且无 .map 报错', async () => {
    const ctx = makeCtx('a')
    const searchApi = {
      global: async function (): Promise<unknown> { throw new Error('网络超时') },
      bbsSubject: async function (): Promise<unknown> { return { data: { data: [{ id: 'b1' }] } } },
    }
    const fn = new AsyncFn('keyword', 'loading', 'searched', 'searchError', 'rows', 'searchApi', 'describeLoadError', doBody)
    await fn(ctx.keyword, ctx.loading, ctx.searched, ctx.searchError, ctx.rows, searchApi, loadErr)
    expect(ctx.rows.value.length).toBe(1)
    expect(ctx.rows.value[0]._from).toBe('论坛')
    expect(ctx.searchError.value).toContain('全局检索失败')
    expect(ctx.searchError.value).not.toContain('.map is not a function')
  })

  it('两来源都成功但都空：rows=[] 且无错误（空态不被错误掩盖）', async () => {
    const ctx = await runSearch({ data: { designList: [], viewList: [] } }, { data: { data: [] } }, 'nope')
    expect(ctx.rows.value.length).toBe(0)
    expect(ctx.searchError.value).toBe('')
  })

  it('端点关键词参数不变且解包后无 .map 错误', async () => {
    const ctx = await runSearch({ data: { designList: [{ id: 'g1' }], viewList: [] } }, { data: { data: [] } }, '  a  ')
    expect(ctx.searchError.value).not.toContain('.map')
    expect(ctx.calls).toEqual(['G:a', 'B:a'])
    expect(src).toContain('searchApi.global(')
    expect(src).toContain('searchApi.bbsSubject(')
  })

  it('写读链保护：两来源 try/catch/失败提示/结果保留/deep-link/空态模板仍在', () => {
    expect(src).toContain("failed.push(describeLoadError('全局检索失败', error))")
    expect(src).toContain("failed.push(describeLoadError('论坛检索失败', error))")
    expect(src).toContain('if (!failed.length || out.length > 0) rows.value = out')
    expect(src).toContain('未找到结果')
    expect(src).toContain('/pages/bbs/topic?id=')
  })
})
