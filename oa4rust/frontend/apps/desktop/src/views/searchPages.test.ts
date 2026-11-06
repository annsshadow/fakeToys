// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/**
 * R145 源码契约：四个「关键词进服务端」的搜索页必须共用 `createSearchRunner`，
 * 并真的把 `signal` 交给 SDK（不是靠丢弃结果假装取消）。
 */
const PAGES = ['SearchApp.vue', 'OrgViewer.vue', 'DocumentApp.vue', 'QueryViewApp.vue'] as const

function source(name: string): string {
  return readFileSync(resolve(__dirname, name), 'utf8')
}

describe('搜索页取消契约（R145）', () => {
  it('四个搜索页都经过 createSearchRunner，且请求带 signal', () => {
    for (const page of PAGES) {
      const src = source(page)
      // 防呆：确保真的读到了页面源码，而不是空文件也能过。
      expect(src.length).toBeGreaterThan(2000)
      expect(src).toContain("from './searchRunner'")
      expect(src).toContain('createSearchRunner(')
      expect(src).toContain('searchRun.run(')
      expect(src).toMatch(/(\{ signal \}|,\s*signal,)/)
    }
  })

  it('OrgViewer 逐个字符触发的搜索走 300ms 防抖，清空关键词即取消在途请求', () => {
    const src = source('OrgViewer.vue')
    expect(src).toContain('createSearchRunner({ debounceMs: 300 })')
    expect(src).toContain('searchRun.cancel()')
  })

  it('SearchApp 失败不再静默：有 catch 且渲染 ListLoadError', () => {
    const src = source('SearchApp.vue')
    expect(src).toContain('searchError.value = describeListError')
    expect(src).toContain('<ListLoadError')
    expect(src).toContain('@retry="doSearch"')
  })
})
