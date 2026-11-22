// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R343 契约：QueryManagerDeep 写操作失效的 queryKey 必须与列表订阅键一致。
 * R342 审计坐实：列表订阅 ['query','defs']，而 create/delete/refresh 三处失效 ['qm','list']，
 * 键不匹配导致写后列表不回显、刷新按钮成为死按钮。源码级静态断言，不引入新依赖。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, 'QueryManagerDeep.vue'), 'utf8')

function count(haystack: string, needle: string): number {
  return haystack.split(needle).length - 1
}

function keyAfter(source: string, anchor: string): string {
  const at = source.indexOf(anchor)
  if (at < 0) throw new Error('缺少锚点: ' + anchor)
  const k = source.indexOf('queryKey: [', at)
  if (k < 0) throw new Error('锚点后缺少 queryKey: ' + anchor)
  const start = k + 'queryKey: ['.length
  const end = source.indexOf(']', start)
  if (end < 0) throw new Error('queryKey 数组未闭合: ' + anchor)
  return source.slice(start, end).trim()
}

function allInvalidateKeys(source: string): string[] {
  const keys: string[] = []
  const marker = 'invalidateQueries('
  let pos = 0
  while (true) {
    const at = source.indexOf(marker, pos)
    if (at < 0) break
    keys.push(keyAfter(source.slice(at), marker))
    pos = at + marker.length
  }
  return keys
}

const subscriptionKey = keyAfter(src, 'const { data, isError, error } = useQuery({')
const errorKey = keyAfter(src, 'useListError({')
const invalidateKeys = allInvalidateKeys(src)

describe('R343 QueryManagerDeep queryKey 失效一致性契约', () => {
  it('列表 useQuery 与 useListError 仍订阅 [query, defs]', () => {
    expect(subscriptionKey).toBe("'query', 'defs'")
    expect(errorKey).toBe("'query', 'defs'")
  })

  it('三处 invalidateQueries 的 queryKey 与列表订阅键一致', () => {
    expect(invalidateKeys).toHaveLength(3)
    for (const key of invalidateKeys) expect(key).toBe(subscriptionKey)
  })

  it('不存在孤立的 [qm, list] 失效键', () => {
    expect(count(src, "queryKey: ['qm', 'list']")).toBe(0)
  })

  it('create/delete/refresh 端点与调用保持原样', () => {
    expect(src).toContain("api.post('/api/query/assemble/designer/create'")
    expect(src).toContain('api.delete(')
    expect(src).toContain('/api/query/assemble/designer/delete/')
    expect(src).toContain('function refresh()')
  })
})
