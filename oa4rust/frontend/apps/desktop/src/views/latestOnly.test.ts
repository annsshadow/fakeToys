// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { createGeneration } from './latestOnly'

describe('切页竞态：只接受最后一次请求的结果（R141）', () => {
  it('快速切换 10 个会话，响应乱序返回时最终只显示第 10 个', () => {
    const generation = createGeneration()
    let shown = ''
    const pending: Array<() => void> = []

    for (let i = 1; i <= 10; i++) {
      const chat = `chat-${i}`
      const token = generation.next()
      // 切换瞬间先清空并占位（与页面行为一致）。
      shown = `${chat}:loading`
      pending.push(() => {
        if (generation.isCurrent(token)) shown = `${chat}:messages`
      })
    }

    // 乱序返回：旧请求先回，最后一次（第 10 个）最后回；旧响应不得覆盖新页面。
    for (const index of [2, 7, 0, 3, 8, 1, 5, 4, 6, 9]) {
      pending[index]()
      if (index !== 9) expect(shown).toBe('chat-10:loading')
    }
    expect(shown).toBe('chat-10:messages')
  })

  it('过期请求的成功与失败分支都不该落地', () => {
    const generation = createGeneration()
    const first = generation.next()
    const second = generation.next()
    const applied: string[] = []

    if (generation.isCurrent(first)) applied.push('first:success')
    if (generation.isCurrent(second)) applied.push('second:success')
    // 失败分支同理：过期请求不能把新页面的错误态改掉。
    if (generation.isCurrent(first)) applied.push('first:error')

    expect(applied).toEqual(['second:success'])
  })

  it('每次 next() 都让此前所有 token 失效，停止后再来一次仍以最新为准', () => {
    const generation = createGeneration()
    const tokens = [generation.next(), generation.next(), generation.next()]
    expect(tokens.map((token) => generation.isCurrent(token))).toEqual([false, false, true])
    expect(generation.isCurrent(0)).toBe(false)
    expect(generation.isCurrent(99)).toBe(false)
  })

  it('AI 会话页：选会话与发消息都接入代际校验，旧响应不得覆盖新会话', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'AIChatApp.vue'), 'utf8')
    expect(source.split('\n').length).toBeGreaterThan(300)
    expect(source).toContain("from './latestOnly'")
    expect(source).toContain('createGeneration()')

    const selectStart = source.indexOf('async function selectChat')
    expect(selectStart).toBeGreaterThan(-1)
    const selectBody = source.slice(selectStart, selectStart + 900)
    expect(selectBody).toContain('const token = chatGeneration.next()')
    expect(selectBody).toContain('chatGeneration.isCurrent(token)')

    const sendStart = source.indexOf('async function sendMessage')
    expect(sendStart).toBeGreaterThan(-1)
    const sendBody = source.slice(sendStart, sendStart + 1400)
    expect(sendBody).toContain('chatGeneration.next()')
    expect(sendBody).toContain('chatGeneration.isCurrent(token)')
  })
})
