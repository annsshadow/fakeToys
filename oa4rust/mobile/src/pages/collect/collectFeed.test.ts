// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { loadCollectRows, mapCollectRows } from './collectFeed'

describe('移动端收藏列表：失败与空列表必须可区分（R111）', () => {
  it('刷新失败保留旧收藏并返回可见错误，而不是清空成「暂无收藏」', async () => {
    const previous = [{ id: 'c-1', title: '收藏一', url: 'https://example.test' }]
    const result = await loadCollectRows(
      async () => {
        throw new Error('Network error')
      },
      'p-1',
      previous,
    )

    expect(result.ok).toBe(false)
    if (result.ok) throw new Error('unreachable')
    // 旧数据必须还在：onShow 刷新失败不该抹掉用户正在看的收藏。
    expect(result.rows).toHaveLength(1)
    expect(result.rows[0]?.id).toBe('c-1')
    expect(result.error).toContain('收藏加载失败')
    expect(result.error).toContain('Network error')
  })

  it('首次加载失败也给非空文案（非 Error 抛出也不静默）', async () => {
    const result = await loadCollectRows(async () => {
      throw 'boom'
    }, 'p-1')
    expect(result.ok).toBe(false)
    if (result.ok) throw new Error('unreachable')
    expect(result.rows).toEqual([])
    expect(result.error).toBe('收藏加载失败：未知错误')
  })

  it('真实空结果才判定为没有收藏', async () => {
    const result = await loadCollectRows(async () => ({ data: [] }), 'p-1', [{ id: 'c-1', title: '收藏一', url: '' }])
    expect(result.ok).toBe(true)
    expect(result.rows).toEqual([])
  })

  it('只保留本人收藏，丢弃无 id 与形状异常的行', () => {
    const rows = mapCollectRows(
      [
        { id: 'c-1', title: '我的', url: 'u1', personId: 'p-1' },
        { id: 'c-2', title: '别人的', url: 'u2', personId: 'p-2' },
        { id: '', title: '无 id', url: 'u3', person_id: 'p-1' },
        { title: '无标题也留下', url: 42, person_id: 'p-1' },
      ],
      'p-1',
    )
    expect(rows).toHaveLength(1)
    expect(rows[0]).toEqual({ id: 'c-1', title: '我的', url: 'u1' })
  })

  it('无标题/无链接的行归一为稳定展示值', () => {
    const rows = mapCollectRows([{ id: 'c-1', personId: 'p-1' }], 'p-1')
    expect(rows).toEqual([{ id: 'c-1', title: '（无标题）', url: '' }])
  })

  it('响应体不是数组时归一为空列表，形状异常不得当成「有数据」', () => {
    // 后端异常返回对象（列表字段缺失）时，旧实现会遍历 undefined 抛错；
    // 归一为 [] 让页面走「空」分支，而不是崩溃或显示半截数据。
    expect(mapCollectRows({ total: 0 }, 'p-1')).toEqual([])
    expect(mapCollectRows(undefined, 'p-1')).toEqual([])
  })

  it('拿不到 personId 时不展示任何收藏（不能把别人的收藏列出来）', () => {
    // collect_list 返回全员收藏，过滤条件依赖本人 id；未登录时宁可空列表，也不泄露他人数据。
    expect(mapCollectRows([{ id: 'c-1', personId: 'p-1' }], null)).toEqual([])
    expect(mapCollectRows([{ id: 'c-1', person_id: 'p-1' }], undefined)).toEqual([])
  })

  it('页面按失败/空/列表三分支渲染，且失败有重试入口', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'collect.vue'), 'utf8')
    // 规模下限：防止文件被截断后断言空转。
    expect(source.split('\n').length).toBeGreaterThan(100)
    expect(source).toContain("from './collectFeed'")
    expect(source).toContain('loadCollectRows')
    expect(source).toContain('loadError')
    // 失败分支必须先于空列表分支，否则错误又会显示成「暂无收藏」。
    const errorBranch = source.indexOf('rows.length === 0 && loadError')
    const emptyBranch = source.indexOf('暂无收藏')
    expect(errorBranch).toBeGreaterThan(-1)
    expect(emptyBranch).toBeGreaterThan(errorBranch)
    expect(source).toContain('重试')
    // 旧实现：失败时直接 rows.value = []。不允许再出现。
    expect(source).not.toContain('rows.value = []')
  })
})
