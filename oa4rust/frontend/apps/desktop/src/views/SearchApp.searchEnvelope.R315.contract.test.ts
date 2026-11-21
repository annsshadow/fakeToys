// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

/**
 * R315 D1 真 bug 回归契约：后端 /api/queryview/search 返回
 * { data: { designList: [{id,name,category}], viewList: [...] } } 分组信封，
 * 旧实现把整个 r.data 对象塞进 results（ref<any[]>），模板 .length 为 undefined，
 * 三个结果分支全不命中，用户看到「有数据却不渲染」。
 */
const src = readFileSync(fileURLToPath(new URL('./SearchApp.vue', import.meta.url)), 'utf8')

function doSearchBody(source: string): string {
  const start = source.indexOf('async function doSearch()')
  expect(start, '未找到 doSearch 函数').toBeGreaterThan(-1)
  const next = source.indexOf('async function loadSearchTwin()', start)
  return source.slice(start, next > -1 ? next : source.indexOf('</script>', start))
}

describe('SearchApp 搜索结果信封解包契约（R315 D1）', () => {
  const body = doSearchBody(src)

  it('回归守卫：不得把整个 r.data 信封直接赋给 results', () => {
    expect(body).not.toContain('results.value = (r?.data ?? []) as any[]')
    expect(body).not.toMatch(/results\.value\s*=\s*\(?\s*r\?\.data\s*\?\?\s*\[\]/)
  })

  it('必须解包 designList 与 viewList 两个真实分组字段', () => {
    expect(body).toContain('designList')
    expect(body).toContain('viewList')
  })

  it('每项映射为扁平数组字段：title<-name、desc<-category、type 区分 design/view', () => {
    expect(body).toMatch(/title:\s*\w+\.name/)
    expect(body).toMatch(/desc:\s*\w+\.category/)
    expect(body).toContain('type: \'design\'')
    expect(body).toContain('type: \'view\'')
  })

  it('results 必须赋值为合并后的真正的数组', () => {
    expect(body).toMatch(/results\.value\s*=\s*\[/)
    expect(body).toMatch(/\.\.\.designList\.map\(/)
    expect(body).toMatch(/\.\.\.viewList\.map\(/)
  })

  it('不回退：保留搜索端点、取消 Runner 与失败提示', () => {
    expect(src).toContain('api.post(\'/api/queryview/search\'')
    expect(body).toContain('searchRun.run(')
    expect(src).toContain('describeListError')
  })
})
