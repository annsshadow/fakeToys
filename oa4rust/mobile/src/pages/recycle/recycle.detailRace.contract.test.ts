// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// recycle.detailRace.contract.test.ts: 源码级钉住回收站详情只认最新一次
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const src = readFileSync(fileURLToPath(new URL('./recycle.vue', import.meta.url)), 'utf8')

describe('recycle 详情只认最新一次（第86轮）', () => {
  it('引入 createLatestGate 并创建详情令牌', () => {
    expect(src).toMatch(/import\s+\{\s*createLatestGate\s*\}\s+from\s+['"]@\/utils\/latest['"]/)
    expect(src).toMatch(/detailGate\s*=\s*createLatestGate\(\)/)
  })

  it('showDetail 开头 begin 且赋值前 isLatest 守卫', () => {
    const fn = src.slice(src.indexOf('async function showDetail'))
    const begin = fn.indexOf('detailGate.begin()')
    const fetched = fn.indexOf('const fetched = ((await recycleApi.detail')
    const guard = fn.indexOf('if (!detailGate.isLatest(token)) return')
    const assign = fn.indexOf('detail.value = fetched')
    expect(begin).toBeGreaterThanOrEqual(0)
    expect(fetched).toBeGreaterThan(begin)
    expect(guard).toBeGreaterThan(fetched)
    expect(guard).toBeLessThan(assign)
  })

  it('catch 分支也守卫', () => {
    const fn = src.slice(src.indexOf('async function showDetail'))
    expect(fn).toMatch(/catch\s*\{\s*if\s*\(!detailGate\.isLatest\(token\)\)\s*return\s*detail\.value\s*=\s*r/)
  })
})
