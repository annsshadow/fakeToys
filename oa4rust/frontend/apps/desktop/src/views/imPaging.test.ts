// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

import { fetchMessagePage, mergeMessagePages } from './imPaging'

interface TestMessage {
  id: string
  content: string
}

/** 120 条消息的内存假后端：按 URL 里的 page/size 切片，模拟真实 im/msg/list。 */
function fakeMessageBackend(total = 120) {
  const all: TestMessage[] = Array.from({ length: total }, (_, i) => ({
    id: `m-${String(i + 1).padStart(3, '0')}`,
    content: `消息 ${i + 1}`,
  }))
  return async (path: string): Promise<{ data: TestMessage[] }> => {
    const matched = /\/list\/(\d+)\/size\/(\d+)/.exec(path)
    if (!matched) throw new Error(`请求未带页码: ${path}`)
    const page = Number(matched[1])
    const size = Number(matched[2])
    const start = (page - 1) * size
    return { data: all.slice(start, start + size) }
  }
}

describe('IMChat 分页（R104）', () => {
  it('连续加载 3 页：每页不重复，第 3 页后到底', async () => {
    const post = fakeMessageBackend(120)
    let loaded: TestMessage[] = []
    let last = await fetchMessagePage<TestMessage>(post, 'c-1', 1)
    loaded = mergeMessagePages(loaded, last.items)

    for (let page = 2; page <= 3; page++) {
      last = await fetchMessagePage<TestMessage>(post, 'c-1', page)
      loaded = mergeMessagePages(loaded, last.items)
    }

    // 3 页 -> 50 + 50 + 20
    expect(loaded).toHaveLength(120)
    expect(new Set(loaded.map((m) => m.id)).size).toBe(120)
    // 第 1、2 页满页 -> 可能还有；第 3 页 20 条 -> 到底
    expect(last.page).toBe(3)
    expect(last.items).toHaveLength(20)
    expect(last.hasMore).toBe(false)
  })

  it('同一页重复拉取不会产生重复消息', async () => {
    const post = fakeMessageBackend(120)
    const first = await fetchMessagePage<TestMessage>(post, 'c-1', 1)
    const again = await fetchMessagePage<TestMessage>(post, 'c-1', 1)
    const loaded = mergeMessagePages(first.items, again.items)
    expect(loaded).toHaveLength(50)
  })

  it('IMChat.vue 使用页码拼装并渲染到底提示', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'IMChat.vue'), 'utf8')
    expect(source).toContain('fetchMessagePage')
    expect(source).toContain('mergeMessagePages')
    expect(source).toContain('已经到底了')
    expect(source).toContain('!msgHasMore')
  })
})
