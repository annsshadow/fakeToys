// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R345 P1 真 bug 回归契约（t151）：
 * 后端 POST /api/queryview/search 返回分组信封 {data:{designList,key,viewList}}，
 * 旧实现 `views.value = r.data ?? []` 把信封对象直接塞进 views，模板 views.length=undefined、
 * v-for 遍历对象键 → 恒 3 行垃圾、真实结果不显示、无结果关键词也无法走空态。
 * 本契约从源码提取 doSearch 函数体，剥离 TS 类型断言后注入 api/searchRun/ref 桩真执行，
 * 断言分组信封被解包映射为 ViewItem[]。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'QueryViewApp.vue'), 'utf8')

function doSearchBody(sourceText: string): string {
  const start = sourceText.indexOf('async function doSearch()')
  expect(start, '找不到 doSearch 函数').toBeGreaterThan(-1)
  const end = sourceText.indexOf('const queryListText', start)
  expect(end, '找不到 doSearch 函数边界').toBeGreaterThan(start)
  return sourceText.slice(start, end)
}

interface Harness {
  urls: Array<{ url: string; payload: unknown }>
  views: { value: any[] }
  loadError: { value: string }
  run: (postResp: unknown, key?: string) => Promise<void>
}

function makeHarness(): Harness {
  const urls: Array<{ url: string; payload: unknown }> = []
  const resp = { value: undefined as unknown }
  const views = { value: [] as any[] }
  const loadError = { value: '' }
  const loading = { value: false }
  const keyword = { value: 'a' }
  const api = {
    post: async (url: string, payload: unknown) => {
      urls.push({ url, payload })
      return resp.value
    },
  }
  const searchRun = {
    run: async (task: (signal: AbortSignal) => Promise<unknown>, hooks?: { onSettled?: () => void }) => {
      const out = await task(new AbortController().signal)
      hooks?.onSettled?.()
      return out
    },
  }
  const body = doSearchBody(source)
    .replace(/:\s*any\[\]/g, '')
    .replace(/\(\s*x\s*:\s*any\s*\)/g, '(x)')
    .replace(/:\s*any\b/g, '')
    .replace(/\s+as\s+any\b/g, '')
  const factory = new Function(
    'api', 'searchRun', 'keyword', 'views', 'loading', 'loadError', 'describeListError',
    body + '\nreturn doSearch;',
  ) as (...args: unknown[]) => () => Promise<void>
  const doSearch = factory(api, searchRun, keyword, views, loading, loadError, (m: string, e: unknown) => String(e))
  return {
    urls,
    views,
    loadError,
    run: async (postResp: unknown, key?: string) => {
      resp.value = postResp
      urls.length = 0
      keyword.value = key ?? 'a'
      views.value = []
      loadError.value = ''
      await doSearch()
    },
  }
}

describe('QueryViewApp 搜索分组信封解包契约（R345 P1）', () => {
  it('分组信封 designList+viewList 解包映射为 ViewItem[]，真实结果可见', async () => {
    const h = makeHarness()
    await h.run({
      data: {
        designList: [{ id: 'd1', name: '设计视图A' }, { id: 'd2', viewName: '设计视图B' }],
        key: 'a',
        viewList: [{ id: 'v1', flag: 'F1', title: '查询视图C' }],
      },
    })
    expect(Array.isArray(h.views.value), 'views 必须是数组，不能是信封对象').toBe(true)
    expect(h.views.value).toHaveLength(3)
    expect(h.views.value[0]).toMatchObject({ id: 'd1', flag: 'd1', name: '设计视图A' })
    expect(h.views.value[1]).toMatchObject({ id: 'd2', name: '设计视图B' })
    expect(h.views.value[2]).toMatchObject({ id: 'v1', flag: 'F1', name: '查询视图C' })
  })

  it('无结果关键词归一为空数组，模板走空态而非 3 行垃圾', async () => {
    const h = makeHarness()
    await h.run({ data: { designList: [], key: 'NO_SUCH', viewList: [] } })
    expect(h.views.value).toEqual([])
  })

  it('坏数据容错：designList 非数组、name 全缺归一为空串不抛错', async () => {
    const h = makeHarness()
    await h.run({ data: { designList: 'oops', viewList: [{ id: 'x' }] } })
    expect(h.views.value).toHaveLength(1)
    expect(h.views.value[0]).toMatchObject({ id: 'x', flag: 'x', name: '' })
  })

  it('端点与 payload 未变：POST /api/queryview/search { key }', async () => {
    const h = makeHarness()
    await h.run({ data: { designList: [], viewList: [] } }, 'mykey')
    expect(h.urls[0]?.url).toBe('/api/queryview/search')
    expect(h.urls[0]?.payload).toEqual({ key: 'mykey' })
  })

  it('写读链模板未回退：空态/v-for/执行/Excel 按钮仍在', () => {
    expect(source).toContain('v-else-if="views.length===0"')
    expect(source).toContain('v-for="v in views"')
    expect(source).toContain('executeView(v)')
    expect(source).toContain('exportExcel(v)')
  })

  it('doSearch 内不再直塞信封：无旧 bug 行且显式解包 designList/viewList', () => {
    const body = doSearchBody(source)
    expect(body).not.toContain('views.value = r.data ?? []')
    expect(body).toContain('designList')
    expect(body).toContain('viewList')
  })
})
