// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

describe('移动端只读浏览列表只认最新一次（第243轮）', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'browse.vue'), 'utf8')
  const loadStart = source.indexOf('async function load(')
  const loadBody = loadStart > -1 ? source.slice(loadStart, source.indexOf('onShow(', loadStart)) : ''

  it('页签在加载中仍可点击：列表并发真实存在，不是被 disabled 挡掉的假竞态', () => {
    const tabLine = source.split('\n').find((l) => l.includes('@tap="switchTab(t.key)"')) ?? ''
    expect(tabLine).toContain('switchTab')
    expect(tabLine.includes('disabled')).toBe(false)
    const switchStart = source.indexOf('function switchTab(')
    const switchBody = source.slice(switchStart, source.indexOf('async function open(', switchStart))
    expect(switchBody.includes('if (loading.value)')).toBe(false)
  })

  it('load() 在发起请求前登记列表令牌（详情门之外另设列表门）', () => {
    expect(source.includes('const listGate = createLatestGate()')).toBe(true)
    const begin = loadBody.indexOf('const token = listGate.begin()')
    expect(begin).toBeGreaterThan(-1)
    expect(begin).toBeLessThan(loadBody.indexOf('await loadBrowseRows'))
  })

  it('过期列表响应不写 rows/loadError，也不提前复位新请求的 loading', () => {
    const guard = loadBody.indexOf('if (!listGate.isLatest(token)) return')
    expect(guard).toBeGreaterThan(-1)
    expect(loadBody.indexOf('rows.value = result.rows')).toBeGreaterThan(guard)
    expect(loadBody.indexOf('loadError.value = result.error')).toBeGreaterThan(guard)
    expect(loadBody.indexOf('loading.value = false')).toBeGreaterThan(guard)
  })
})
