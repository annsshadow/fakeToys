// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'BBSForum.vue'), 'utf8')

describe('BBSForum reply refresh contract', () => {
  it('refetch replies after reply published', () => {
    const start = source.indexOf('const replyMutation = useMutation({')
    const end = source.indexOf('function submitReply', start)
    expect(start).toBeGreaterThanOrEqual(0)
    expect(end).toBeGreaterThan(start)
    const onSuccess = source.slice(start, end)

    // replies query 必须导出自己的 refetch，别名 refetchReplies
    expect(source).toContain('refetch: refetchReplies,')
    // 发布回复成功后应刷新 replies 列表，而不是把 topics 列表刷两次
    expect(onSuccess).toContain('refetchReplies()')
    expect(onSuccess).not.toContain('refetch() // refetch replies')
    // 同时保留刷新 topic 计数
    expect(onSuccess).toContain('refetch()')
  })
})
