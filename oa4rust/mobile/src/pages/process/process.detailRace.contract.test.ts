// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const src = readFileSync(fileURLToPath(new URL('./process.vue', import.meta.url)), 'utf8')

describe('process 详情浮层只认最新一次（第91轮）', () => {
  it('创建 detailGate 且 openDetail 以 begin 起头', () => {
    expect(src.includes('detailGate = createLatestGate()')).toBe(true)
    const fn = src.slice(src.indexOf('async function openDetail'))
    expect(fn.includes('detailGate.begin()')).toBe(true)
  })

  it('赋值前设置 isLatest 守卫，过期响应不写详情', () => {
    const fn = src.slice(src.indexOf('async function openDetail'))
    const guard = fn.indexOf('if (!detailGate.isLatest(token)) return')
    const assign = fn.indexOf('detailAttachments.value = att')
    const clear = fn.indexOf('detailLoading.value = false')
    expect(guard).toBeGreaterThanOrEqual(0)
    expect(assign).toBeGreaterThan(guard)
    expect(clear).toBeGreaterThan(guard)
  })

  it('关闭浮层使在途详情请求失效', () => {
    const fn = src.slice(src.indexOf('function closeDetail'))
    const body = fn.slice(0, fn.indexOf('\n}'))
    expect(body.includes('detailGate.begin()')).toBe(true)
  })
})
