// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { describeProcessError, loadProcessPage, mergeProcessRows, type ProcessTab } from './processList'

interface Task {
  id: string
  title: string
}

/** 130 条待办，后端顺序固定（id 递增）。 */
function fakeTaskBackend(total = 130) {
  const all: Task[] = Array.from({ length: total }, (_, i) => ({
    id: `t-${String(i + 1).padStart(3, '0')}`,
    title: `任务 ${i + 1}`,
  }))
  const calls: Array<{ tab: ProcessTab; page: number; size: number }> = []
  const load = async (tab: ProcessTab, page: number, size: number) => {
    calls.push({ tab, page, size })
    const start = (page - 1) * size
    return { data: all.slice(start, start + size) }
  }
  return { load, calls }
}

describe('移动端流程列表分页：页码进请求、失败不折叠成空列表（R109）', () => {
  it('130 条待办连续加载 3 页：页码递增、去重合并、末页到底', async () => {
    const { load, calls } = fakeTaskBackend(130)

    const first = await loadProcessPage<Task>(load, 'pending', 1)
    expect(first.rows).toHaveLength(50)
    expect(first.hasMore).toBe(true)
    expect(first.rows[0]?.id).toBe('t-001')

    let merged = mergeProcessRows<Task>([], first.rows)
    expect(merged).toHaveLength(50)

    const second = await loadProcessPage<Task>(load, 'pending', 2)
    merged = mergeProcessRows<Task>(merged, second.rows)
    expect(merged).toHaveLength(100)
    expect(second.hasMore).toBe(true)

    const third = await loadProcessPage<Task>(load, 'pending', 3)
    merged = mergeProcessRows<Task>(merged, third.rows)
    expect(merged).toHaveLength(130)
    expect(new Set(merged.map((t) => t.id)).size).toBe(130)
    expect(merged[0]?.id).toBe('t-001')
    expect(merged[merged.length - 1]?.id).toBe('t-130')
    expect(third.hasMore).toBe(false)

    // 页码真的进了请求：旧实现固定 (1, 50)。
    expect(calls.map((c) => c.page)).toEqual([1, 2, 3])
    expect(calls.every((c) => c.size === 50)).toBe(true)
    expect(calls.every((c) => c.tab === 'pending')).toBe(true)
  })

  it('页签切换把 tab 带进加载器，已办/我发起不会串数据', async () => {
    const { load, calls } = fakeTaskBackend(10)
    await loadProcessPage<Task>(load, 'done', 1)
    await loadProcessPage<Task>(load, 'started', 1)
    expect(calls.map((c) => c.tab)).toEqual(['done', 'started'])
  })

  it('分页重叠（办理任务导致 OFFSET 位移）按 id 去重，不产生重复行', () => {
    const current = Array.from({ length: 50 }, (_, i) => ({
      id: `t-${String(i + 1).padStart(3, '0')}`,
      title: '',
    }))
    // 期间办结 10 条，第 2 页从 t-041 起，与当前页重叠 t-041 … t-050。
    const shifted = Array.from({ length: 50 }, (_, i) => ({
      id: `t-${String(i + 41).padStart(3, '0')}`,
      title: '',
    }))
    const merged = mergeProcessRows(current, shifted)
    expect(merged).toHaveLength(90)
    expect(new Set(merged.map((t) => t.id)).size).toBe(90)
    expect(merged[0]?.id).toBe('t-001')
    expect(merged[merged.length - 1]?.id).toBe('t-090')
  })

  it('无可用 id 的行原样保留：去重不能以「没有 id」为由丢行', () => {
    // OFFSET 分页会让相邻页重叠；只有能按 id 判重的行才去重，
    // 无 id / 非字符串 id 的行无法判重，必须保留而不是丢弃。
    const current: Array<{ id?: unknown; title: string }> = [
      { id: '', title: '旧行（空 id）' },
      { id: 't-1', title: '旧行 t-1' },
    ]
    const incoming: Array<{ id?: unknown; title: string }> = [
      { title: '无 id 新行' },
      { id: 42, title: '非字符串 id' },
      { id: 't-1', title: '重复行' },
    ]
    const merged = mergeProcessRows(current, incoming)
    expect(merged.map((r) => r.title)).toEqual(['旧行（空 id）', '旧行 t-1', '无 id 新行', '非字符串 id'])
    expect(merged.map((r) => r.id)).toEqual(['', 't-1', undefined, 42])
  })

  it('页码夹取与错误文案：非法页码按 1 请求，非 Error 也有可见提示', async () => {
    const { load, calls } = fakeTaskBackend(50)
    await loadProcessPage<Task>(load, 'pending', 0)
    await loadProcessPage<Task>(load, 'pending', -5)
    expect(calls.map((c) => c.page)).toEqual([1, 1])
    expect(describeProcessError('流程加载失败', new Error('Network error'))).toBe('流程加载失败：Network error')
    expect(describeProcessError('更多流程加载失败', 'boom')).toBe('更多流程加载失败：未知错误')
  })

  it('响应体形状异常归一为空数组而不是崩溃', async () => {
    const result = await loadProcessPage<Task>(async () => ({ data: { total: 0 } }), 'pending', 1)
    expect(result.rows).toEqual([])
    expect(result.hasMore).toBe(false)
  })

  it('页面按失败/空/列表分支渲染，失败有重试、底部有加载更多与到底提示', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'process.vue'), 'utf8')
    // 规模下限：防止文件被截断后断言空转。
    expect(source.split('\n').length).toBeGreaterThan(300)
    expect(source).toContain("from './processList'")
    expect(source).toContain('loadProcessPage')
    expect(source).toContain('mergeProcessRows')
    expect(source).toContain('加载更多')
    expect(source).toContain('已加载')
    expect(source).toContain('loadError')
    expect(source).toContain('重试')
    // 失败分支必须先于「暂无待办」分支，否则错误又会显示成真的空列表。
    const errorBranch = source.indexOf('rows.length === 0 && loadError')
    const emptyBranch = source.indexOf('暂无{{')
    expect(errorBranch).toBeGreaterThan(-1)
    expect(emptyBranch).toBeGreaterThan(errorBranch)
    // 旧实现：三个页签固定第 1 页 50 条；失败时静默清空。
    expect(source).not.toContain('pendingList(1, 50)')
    expect(source).not.toContain('completedList(1, 50)')
    expect(source).not.toContain('startedList(1, 50)')
    expect(source).not.toContain('total.value = resp.count')
  })
})
