// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { loadBbsTopics } from './bbsFeed'

interface Topic {
  id: string
  title: string
}

describe('论坛主题：失败与空列表必须可区分（R105）', () => {
  it('刷新失败保留旧主题并返回可见错误，而不是清空成「暂无主题」', async () => {
    const previous: Topic[] = [{ id: 't-1', title: '旧主题' }]
    const result = await loadBbsTopics<Topic>(async () => {
      throw new Error('Network error')
    }, previous)

    expect(result.ok).toBe(false)
    if (result.ok) throw new Error('unreachable')
    // 旧数据必须还在：下拉刷新失败不该抹掉用户正在看的内容。
    expect(result.rows).toHaveLength(1)
    expect(result.rows[0]?.id).toBe('t-1')
    expect(result.error).toContain('主题加载失败')
    expect(result.error).toContain('Network error')
  })

  it('首次加载失败也给非空文案（非 Error 抛出也不静默）', async () => {
    const result = await loadBbsTopics<Topic>(async () => {
      throw 'boom'
    })
    expect(result.ok).toBe(false)
    if (result.ok) throw new Error('unreachable')
    expect(result.error).toBe('主题加载失败：未知错误')
  })

  it('真实空结果才判定为没有主题', async () => {
    const result = await loadBbsTopics<Topic>(async () => ({ data: [] }), [{ id: 't-1', title: '旧主题' }])
    expect(result.ok).toBe(true)
    expect(result.rows).toEqual([])
  })

  it('响应体形状异常归一为空数组而不是崩溃或抛错', async () => {
    const result = await loadBbsTopics<Topic>(async () => ({ data: { total: 0 } }))
    expect(result.ok).toBe(true)
    expect(result.rows).toEqual([])
  })

  it('页面按失败/空/列表三分支渲染，且失败有重试入口', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'bbs.vue'), 'utf8')
    // 规模下限：防止文件被截断后断言空转。
    expect(source.split('\n').length).toBeGreaterThan(100)
    expect(source).toContain('loadBbsTopics')
    expect(source).toContain('bbsError')
    // 失败分支必须先于空列表分支，否则错误又会显示成「暂无主题」。
    const errorBranch = source.indexOf('rows.length === 0 && bbsError')
    const emptyBranch = source.indexOf('论坛暂无主题')
    expect(errorBranch).toBeGreaterThan(-1)
    expect(emptyBranch).toBeGreaterThan(errorBranch)
    expect(source).toContain('重试')
    // 旧实现：失败时直接 rows.value = []。不允许再出现。
    expect(source).not.toContain('rows.value = []')
  })
})
