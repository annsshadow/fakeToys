// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R259：移动端发消息不得把 HTTP 200 的业务失败当成功。
 *
 * 行为单测钉住「只有 data.sent===true 才算送达」；源码契约钉住 chat.vue 的
 * send() 确实消费该判定，且判定发生在写占位行/清输入框之前——否则业务再正确
 * 也不会被页面调用，测试仍能失败。
 */
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'
import { messageWasSent } from './chatHistory'

const src = readFileSync(fileURLToPath(new URL('./chat.vue', import.meta.url)), 'utf8')

describe('R259 发消息仅认服务端落库确认', () => {
  it('业务错误信封 data 为 null 时不算送达', () => {
    expect(messageWasSent({ data: null } as never)).toBe(false)
    expect(messageWasSent(undefined)).toBe(false)
  })

  it('sent=false 或缺失时不算送达，仅 sent===true 算送达', () => {
    expect(messageWasSent({ data: { sent: false } })).toBe(false)
    expect(messageWasSent({ data: {} })).toBe(false)
    expect(messageWasSent({ data: { sent: 'true' } })).toBe(false)
    expect(messageWasSent({ data: { sent: true } })).toBe(true)
  })

  it('chat.vue 在写占位行前用 messageWasSent 守卫发送结果', () => {
    const send = src.slice(src.indexOf('async function send()'))
    const guard = send.indexOf('if (!messageWasSent(resp))')
    const local = send.indexOf('local-')
    const clear = send.indexOf('text.value = ' + "'" + "'")
    expect(guard).toBeGreaterThanOrEqual(0)
    expect(local).toBeGreaterThan(guard)
    expect(clear).toBeGreaterThan(guard)
  })
})
