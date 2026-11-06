// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { describeLoadError, listState, loadList } from './loadList'

describe('列表加载统一口径（R113）', () => {
  it('失败保留旧数据并给出非空文案', async () => {
    const result = await loadList<{ id: string }>(
      async () => {
        throw new Error('Network error')
      },
      { previousRows: [{ id: 'a' }], label: '联系人加载失败' },
    )
    expect(result.ok).toBe(false)
    if (result.ok) throw new Error('unreachable')
    expect(result.rows).toEqual([{ id: 'a' }])
    expect(result.error).toBe('联系人加载失败：Network error')
  })

  it('非 Error 抛出也归一为「未知错误」', async () => {
    const result = await loadList(
      async () => {
        throw 'boom'
      },
      { label: '公告加载失败' },
    )
    expect(result.ok).toBe(false)
    if (result.ok) throw new Error('unreachable')
    expect(result.error).toBe('公告加载失败：未知错误')
  })

  it('真实空结果才返回 ok + 空数组', async () => {
    const result = await loadList<{ id: string }>(async () => ({ data: [] }), {
      previousRows: [{ id: 'a' }],
    })
    expect(result.ok).toBe(true)
    expect(result.rows).toEqual([])
  })

  it('形状异常归一为空数组而不是崩溃', async () => {
    const result = await loadList(async () => ({ data: { total: 0 } }))
    expect(result.ok).toBe(true)
    expect(result.rows).toEqual([])
  })

  it('业务映射可复用统一出口（收藏页只留本人）', async () => {
    const result = await loadList(
      async () => ({
        data: [
          { id: 'a', mine: true },
          { id: 'b', mine: false },
        ],
      }),
      {
        map: (data) =>
          Array.isArray(data) ? (data as { id: string; mine: boolean }[]).filter((r) => r.mine).map((r) => r.id) : [],
      },
    )
    expect(result.ok).toBe(true)
    expect(result.rows).toEqual(['a'])
  })

  it('三分支判定：错误先于空态，首屏加载先于两者', () => {
    expect(listState({ loading: true, loaded: false, count: 0, error: 'err' })).toBe('loading')
    expect(listState({ loading: false, loaded: true, count: 0, error: 'err' })).toBe('error')
    expect(listState({ loading: false, loaded: true, count: 0, error: '' })).toBe('empty')
    expect(listState({ loading: true, loaded: true, count: 2, error: 'err' })).toBe('list')
  })

  it('加载中与失败态互斥（R137）：有数据时失败不退化为错误态，无错误时不提前报错', () => {
    // 已有数据 + 上一次失败：列表仍可见（错误由页面横幅承担），不整屏切回 error
    expect(listState({ loading: false, loaded: true, count: 3, error: '公告加载失败：请求超时' })).toBe('list')
    // 从未加载过且无错误：不能报错（只能空态或加载态）
    expect(listState({ loading: false, loaded: false, count: 0, error: '' })).toBe('empty')
    // 首屏加载中即便残留错误：仍以加载态为准，错误不提前暴露
    expect(listState({ loading: true, loaded: false, count: 0, error: 'err' })).toBe('loading')
  })

  it('describeLoadError 对空 message 也给稳定文案', () => {
    expect(describeLoadError('加载失败', new Error('  '))).toBe('加载失败：未知错误')
  })
})
