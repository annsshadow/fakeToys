// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/**
 * R138：IM 发送是乐观更新（先插气泡、失败再撤）。如果失败只撤气泡不提示，
 * 用户看到的是「消息发出后静默消失」；后端把无权限改成 403 后这会成为常见路径。
 * node 环境无法挂载 .vue，因此用源码契约守住这两个行为。
 */
describe('IMChat 写失败可见性（R138）', () => {
  const src = readFileSync(resolve(import.meta.dirname, 'IMChat.vue'), 'utf8')

  it('sendMutation 的 onError 撤掉乐观气泡后必须 toast.error', () => {
    const start = src.indexOf('onError: (err, _vars, context)')
    expect(start).toBeGreaterThan(-1)
    const block = src.slice(start, start + 400)
    expect(block).toContain('optimisticMsg')
    expect(block).toContain('toast.error(')
    expect(block).toContain('mutationErrorMessage(err)')
  })

  it('markAllRead 不再全静默：allSettled 后聚合 rejected 提示', () => {
    const start = src.indexOf('async function markAllRead')
    expect(start).toBeGreaterThan(-1)
    const block = src.slice(start, start + 900)
    expect(block).toContain('allSettled')
    expect(block).toContain('PromiseRejectedResult')
    expect(block).toContain('toast.error(')
  })
})
