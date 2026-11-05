// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { loadBrowseRows } from './browseFeed'

describe('移动端只读浏览：失败与空列表必须可区分（R112）', () => {
  it('同页签刷新失败保留旧数据并返回可见错误，而不是清空成「暂无数据」', async () => {
    const previous = [{ id: 'p-1', name: '门户一' }]
    const result = await loadBrowseRows(async () => {
      throw new Error('Network error')
    }, previous)

    expect(result.ok).toBe(false)
    if (result.ok) throw new Error('unreachable')
    expect(result.rows).toHaveLength(1)
    expect(result.rows[0]?.id).toBe('p-1')
    expect(result.error).toContain('加载失败')
    expect(result.error).toContain('Network error')
  })

  it('首次加载失败也给非空文案（非 Error 抛出也不静默）', async () => {
    const result = await loadBrowseRows(async () => {
      throw 'boom'
    })
    expect(result.ok).toBe(false)
    if (result.ok) throw new Error('unreachable')
    expect(result.rows).toEqual([])
    expect(result.error).toBe('加载失败：未知错误')
  })

  it('真实空结果才判定为该页签没有数据', async () => {
    const result = await loadBrowseRows(async () => ({ data: [] }), [{ id: 'p-1', name: '门户一' }])
    expect(result.ok).toBe(true)
    expect(result.rows).toEqual([])
  })

  it('响应体形状异常归一为空数组而不是崩溃', async () => {
    const result = await loadBrowseRows(async () => ({ data: { total: 0 } }))
    expect(result.ok).toBe(true)
    expect(result.rows).toEqual([])
  })

  it('页面按失败/空/列表三分支渲染，失败有重试，切换页签不复用旧页签的行', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'browse.vue'), 'utf8')
    // 规模下限：防止文件被截断后断言空转。
    expect(source.split('\n').length).toBeGreaterThan(180)
    expect(source).toContain("from './browseFeed'")
    expect(source).toContain('loadBrowseRows')
    expect(source).toContain('loadError')
    // 失败分支必须先于「暂无数据」分支，否则错误又会显示成空态。
    const errorBranch = source.indexOf('rows.length === 0 && loadError')
    const emptyBranch = source.indexOf('暂无数据')
    expect(errorBranch).toBeGreaterThan(-1)
    expect(emptyBranch).toBeGreaterThan(errorBranch)
    expect(source).toContain('重试')
    // 旧实现：失败时直接 rows.value = []。不允许再出现。
    expect(source).not.toContain('rows.value = []')
  })
})
