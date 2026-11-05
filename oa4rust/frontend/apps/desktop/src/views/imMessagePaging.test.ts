// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { buildImPagingPath, fetchImPagingPage, normalizePagingRows } from './imMessagePaging'

interface Row {
  id: string
  content: string
  sender: string
  create_time: string
}

function fakePagingBackend(total = 50) {
  const all: Row[] = Array.from({ length: total }, (_, i) => ({
    id: `m-${String(i + 1).padStart(3, '0')}`,
    content: `消息 ${i + 1}`,
    sender: 'system',
    create_time: '2026-11-05 10:00',
  }))
  const calls: string[] = []
  const fetchPage = async (path: string): Promise<unknown> => {
    calls.push(path)
    const matched = /\/paging\/(\d+)\/size\/(\d+)/.exec(path)
    if (!matched) throw new Error(`请求未带页码: ${path}`)
    const page = Number(matched[1])
    const size = Number(matched[2])
    const start = (page - 1) * size
    return { data: all.slice(start, start + size) }
  }
  return { fetchPage, calls }
}

describe('IM 消息分页诊断：页码必须进请求且结果被使用（R106）', () => {
  it('连续翻 3 页：每页 URL 带对应页码，结果不重复，末页 hasNext=false', async () => {
    const { fetchPage, calls } = fakePagingBackend(50)

    const first = await fetchImPagingPage(fetchPage, 1)
    const second = await fetchImPagingPage(fetchPage, 2)
    const third = await fetchImPagingPage(fetchPage, 3)

    expect(calls[0]).toContain('/paging/1/size/20')
    expect(calls[1]).toContain('/paging/2/size/20')
    expect(calls[2]).toContain('/paging/3/size/20')

    expect(first.rows).toHaveLength(20)
    expect(first.hasNext).toBe(true)
    expect(second.rows).toHaveLength(20)
    expect(second.hasNext).toBe(true)
    expect(third.rows).toHaveLength(10)
    expect(third.hasNext).toBe(false)

    const ids = [...first.rows, ...second.rows, ...third.rows].map((row) => row.id)
    expect(ids).toHaveLength(50)
    expect(new Set(ids).size).toBe(50)
  })

  it('页码夹取：0 与负数按第 1 页请求，绝不把非法页码送进 URL', async () => {
    const { fetchPage, calls } = fakePagingBackend(20)
    await fetchImPagingPage(fetchPage, 0)
    await fetchImPagingPage(fetchPage, -3)
    expect(calls[0]).toBe(buildImPagingPath(1))
    expect(calls[1]).toBe(buildImPagingPath(1))
    expect(calls[0]).not.toContain('/paging/0/')
  })

  it('响应形状归一：数组 / data.list / data.data 与 createTime 兜底', () => {
    expect(normalizePagingRows({ data: [{ id: 'a', content: 'x', sender: 's', create_time: 't' }] })).toEqual([
      { id: 'a', content: 'x', sender: 's', createTime: 't' },
    ])
    expect(normalizePagingRows({ data: { list: [{ id: 'b', createTime: 't2' }] } })).toEqual([
      { id: 'b', content: '', sender: '', createTime: 't2' },
    ])
    expect(normalizePagingRows({ data: { data: [{ id: 'c' }] } })[0]?.id).toBe('c')
    expect(normalizePagingRows({ data: { total: 0 } })).toEqual([])
    // 无 id 的行不算消息（避免渲染不可定位的项）
    expect(normalizePagingRows({ data: [{ content: 'no id' }] })).toEqual([])
  })

  it('页面用返回结果渲染并提供可见的上一页/下一页入口', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'IMChat.vue'), 'utf8')
    // 规模下限：防止文件被截断后断言空转。
    expect(source.split('\n').length).toBeGreaterThan(1000)
    expect(source).toContain("from './imMessagePaging'")
    expect(source).toContain('openMsgPaging')
    expect(source).toContain('msgPagingPage')
    expect(source).toContain('msgPagingRows')
    expect(source).toContain('msgPagingHasNext')
    expect(source).toContain('上一页')
    expect(source).toContain('下一页')
    // 旧实现：按钮固定请求第 1 页并丢弃结果。
    expect(source).not.toContain("imMore('msgPaging')")
  })
})
