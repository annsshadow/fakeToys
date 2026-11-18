// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// calendar.detailRace.contract.test.ts: 源码级钉住事件详情只认最新一次
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const src = readFileSync(fileURLToPath(new URL('./calendar.vue', import.meta.url)), 'utf8')

describe('calendar 事件详情只认最新一次（第86轮）', () => {
  it('引入 createLatestGate 并创建详情令牌', () => {
    expect(src).toMatch(/import\s+\{\s*createLatestGate\s*\}\s+from\s+['"]@\/utils\/latest['"]/)
    expect(src).toMatch(/detailGate\s*=\s*createLatestGate\(\)/)
  })

  it('openEvent 开头 begin 令牌，赋值前 isLatest 守卫', () => {
    const fn = src.slice(src.indexOf('async function openEvent'))
    const begin = fn.indexOf('detailGate.begin()')
    const assign = fn.indexOf('detail.value = ((resp')
    const guard = fn.indexOf('if (!detailGate.isLatest(token)) return')
    expect(begin).toBeGreaterThanOrEqual(0)
    expect(guard).toBeGreaterThan(begin)
    expect(guard).toBeLessThan(assign)
  })

  it('catch 分支也守卫，过期错误不写旧事件', () => {
    const fn = src.slice(src.indexOf('async function openEvent'))
    const guarded = fn.match(/catch\s*\{\s*if\s*\(!detailGate\.isLatest\(token\)\)\s*return/)
    expect(guarded).not.toBeNull()
  })

  it('finally 只在最新令牌时清 detailLoading', () => {
    const fn = src.slice(src.indexOf('async function openEvent'))
    expect(fn).toMatch(/finally\s*\{\s*if\s*\(detailGate\.isLatest\(token\)\)\s*detailLoading\.value\s*=\s*false/)
  })
})
