// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { PENDING_PAGE_SIZE, PENDING_PROBE_SIZE, pendingBadge } from './pendingCount'

describe('工作台待办计数（R149）', () => {
  it('未满哨兵页时展示真实条数', () => {
    expect(pendingBadge(0)).toEqual({ text: '0', overflow: false })
    expect(pendingBadge(1)).toEqual({ text: '1', overflow: false })
    expect(pendingBadge(49)).toEqual({ text: '49', overflow: false })
    expect(pendingBadge(50)).toEqual({ text: '50', overflow: false })
  })

  it('拿满哨兵页时展示 50+，不再把本页条数当成总数', () => {
    expect(PENDING_PAGE_SIZE).toBe(50)
    expect(PENDING_PROBE_SIZE).toBe(51)
    expect(pendingBadge(51)).toEqual({ text: '50+', overflow: true })
    expect(pendingBadge(200)).toEqual({ text: '50+', overflow: true })
  })

  it('拿不到数或非法值时展示占位符', () => {
    expect(pendingBadge(null)).toEqual({ text: '–', overflow: false })
    expect(pendingBadge(undefined)).toEqual({ text: '–', overflow: false })
    expect(pendingBadge(Number.NaN)).toEqual({ text: '–', overflow: false })
    expect(pendingBadge(-1)).toEqual({ text: '–', overflow: false })
  })

  it('页面按哨兵页请求并展示格式化结果', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'index.vue'), 'utf8')
    expect(source.split('\n').length).toBeGreaterThan(150)
    expect(source).toContain("from './pendingCount'")
    expect(source).toContain('pendingBadge')
    expect(source).toContain('const badge = pendingBadge(total)')
    expect(source).toContain('pendingBadgeText.value = badge.text')
    // 旧实现：固定请求 50 条，且把本页条数当总数直接展示。
    expect(source).not.toContain('processApi.pendingList(1, 50)')
    expect(source).toContain('processApi.pendingList(1, PENDING_PROBE_SIZE)')
    expect(source).toContain('{{ loading ?')
  })
})
