// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import {
  CHAT_NEAR_BOTTOM_PX,
  distanceFromBottom,
  isNearBottom,
  pendingAfterAppend,
  shouldFollowToLatest,
} from './chatScroll'

describe('会话滚动跟随判定（R148）', () => {
  it('距底部：正常/贴底/内容不足一屏/负值钳到 0', () => {
    expect(distanceFromBottom({ scrollTop: 100, scrollHeight: 800, clientHeight: 600 })).toBe(100)
    expect(distanceFromBottom({ scrollTop: 200, scrollHeight: 800, clientHeight: 600 })).toBe(0)
    // 内容不足一屏（scrollHeight < clientHeight）：不能算出负数。
    expect(distanceFromBottom({ scrollTop: 0, scrollHeight: 300, clientHeight: 600 })).toBe(0)
    expect(distanceFromBottom({ scrollTop: -10, scrollHeight: 800, clientHeight: 600 })).toBe(210)
  })

  it('近底部阈值：等于 80 算在底部，81 不算', () => {
    const base = { scrollHeight: 1000, clientHeight: 500 }
    expect(isNearBottom({ ...base, scrollTop: 420 })).toBe(true)
    expect(isNearBottom({ ...base, scrollTop: 419 })).toBe(false)
    expect(isNearBottom({ ...base, scrollTop: 420 }, 79)).toBe(false)
    expect(CHAT_NEAR_BOTTOM_PX).toBe(80)
  })

  it('首屏未拿到尺寸也跟随；玩家翻旧消息时不跟随', () => {
    expect(shouldFollowToLatest(null)).toBe(true)
    expect(shouldFollowToLatest({ scrollTop: 500, scrollHeight: 1500, clientHeight: 500 })).toBe(false)
    expect(shouldFollowToLatest({ scrollTop: 420, scrollHeight: 1000, clientHeight: 500 })).toBe(true)
  })

  it('未跟随的新消息只累加，跟随即清零', () => {
    expect(pendingAfterAppend(0, 2, false)).toBe(2)
    expect(pendingAfterAppend(2, 3, false)).toBe(5)
    expect(pendingAfterAppend(5, 1, true)).toBe(0)
    expect(pendingAfterAppend(5, 0, false)).toBe(5)
  })
})

describe('chat.vue 滚动接线契约（R148）', () => {
  const source = readFileSync(resolve(__dirname, 'chat.vue'), 'utf8')

  it('scroll-view 绑定 scroll-into-view 与滚动监听，并有底部锚点', () => {
    expect(source.length).toBeGreaterThan(3000)
    expect(source).toContain(':scroll-into-view="scrollIntoView"')
    expect(source).toContain('@scroll="onScroll"')
    expect(source).toContain('id="chat-bottom"')
  })

  it('三处消息变更（首屏/发送/轮询）都经过跟随判定', () => {
    expect(source).toContain('shouldFollowToLatest')
    expect(source).toContain('pendingAfterAppend')
    const calls = source.match(/afterMessagesChanged\(/g) ?? []
    // 定义 1 次 + 首屏/发送/轮询各 1 次（加载更早消息不跟随，不得计入）。
    expect(calls.length).toBeGreaterThanOrEqual(4)
  })

  it('存在「有新消息」入口，点击回到最新', () => {
    expect(source).toContain('pendingCount > 0')
    expect(source).toContain('@tap="scrollToLatest"')
  })
})
