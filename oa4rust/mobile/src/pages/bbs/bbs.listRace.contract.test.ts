// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

describe('移动端论坛列表只认最新一次（第265轮）', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'bbs.vue'), 'utf8')
  const loadStart = source.indexOf('async function load(')
  const loadBody = loadStart > -1 ? source.slice(loadStart, source.indexOf('onShow(', loadStart)) : ''

  it('列表加载入口保持可重复触发，竞态真实存在而非被忙标志挡掉', () => {
    expect(loadBody.includes('if (loading.value)')).toBe(false)
    for (const line of source.split('\n')) {
      if (line.includes('@tap="load"') || line.includes('onPullDownRefresh')) {
        expect(line.includes('disabled')).toBe(false)
      }
    }
  })

  it('load() 在发起请求前登记列表令牌', () => {
    expect(source.includes('const listGate = createLatestGate()')).toBe(true)
    expect(source.includes("import { createLatestGate } from '@/utils/latest'")).toBe(true)
    const begin = loadBody.indexOf('const token = listGate.begin()')
    expect(begin).toBeGreaterThan(-1)
    expect(begin).toBeLessThan(loadBody.indexOf('await loadBbsTopics'))
  })

  it('过期列表响应不写 rows/bbsError，也不提前复位新请求的 loading', () => {
    const guard = loadBody.indexOf('if (!listGate.isLatest(token)) return')
    expect(guard).toBeGreaterThan(-1)
    const afterGuard = loadBody.slice(guard)
    expect(afterGuard).toContain('rows.value = result.rows')
    expect(afterGuard).toContain('bbsError.value = result.error')
    expect(afterGuard).toContain('loading.value = false')
    expect(afterGuard).toContain('loaded.value = true')
  })
})
