// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

// mobile vitest 未装配 .vue 插件，按仓内源码断言惯例守卫（见 attendanceLoadRace.test.ts）。
const source = readFileSync(resolve(import.meta.dirname, 'message.vue'), 'utf8')

describe('移动端会话列表只认最新一次（R278）', () => {
  const loadStart = source.indexOf('async function load(')
  const loadBody = loadStart > -1 ? source.slice(loadStart, source.indexOf('onShow(', loadStart)) : ''

  it('列表加载入口可重复触发：onShow 每次显示 + 两处无禁用重试按钮，竞态真实存在', () => {
    expect(source).toContain('onShow(async () => {')
    expect(loadBody.includes('if (loading.value)')).toBe(false)
    const retryLines = source.split('\n').filter((l) => l.includes('@tap="load"'))
    expect(retryLines.length).toBeGreaterThanOrEqual(2)
    for (const line of retryLines) expect(line.includes('disabled')).toBe(false)
  })

  it('load() 在发起请求前登记请求序号', () => {
    expect(source).toContain('let loadSeq = 0')
    const seqAt = loadBody.indexOf('const seq = ++loadSeq')
    expect(seqAt).toBeGreaterThan(-1)
    expect(seqAt).toBeLessThan(loadBody.indexOf('await loadConversations'))
  })

  it('过期响应不写 rows/loadError/networkOffline/loading/loaded', () => {
    const guard = loadBody.indexOf('if (seq !== loadSeq) return')
    expect(guard).toBeGreaterThan(-1)
    const awaitAt = loadBody.indexOf('await loadConversations')
    expect(guard).toBeGreaterThan(awaitAt)
    const afterGuard = loadBody.slice(guard)
    for (const write of [
      'rows.value = result.rows',
      'loadError.value = result.error',
      'networkOffline.value = false',
      'loading.value = false',
      'loaded.value = true',
    ]) {
      expect(afterGuard).toContain(write)
    }
  })
})
