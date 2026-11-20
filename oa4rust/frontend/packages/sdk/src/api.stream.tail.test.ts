// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { ApiClient } from './api'

function sseResponse(chunks: string[]): Response {
  const encoder = new TextEncoder()
  const stream = new ReadableStream<Uint8Array>({
    start(controller) {
      for (const c of chunks) controller.enqueue(encoder.encode(c))
      controller.close()
    },
  })
  return new Response(stream, { status: 200, headers: { 'Content-Type': 'text/event-stream' } })
}

describe('ApiClient.stream 流尾事件', () => {
  beforeEach(() => {
    vi.stubGlobal('window', { location: { origin: 'https://web.example.test' } })
    vi.stubGlobal('localStorage', {
      getItem: () => {
        throw new Error('must not read localStorage')
      },
    })
    vi.stubGlobal('sessionStorage', {
      getItem: () => {
        throw new Error('must not read sessionStorage')
      },
    })
  })
  afterEach(() => {
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('服务端在流结束时未补空行：最后一条 data 事件仍须回调', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(sseResponse(['event: token\ndata: {"token":"first"}\n\nevent: done\ndata: [DONE]\n'])),
    )
    const events: Array<[string, string]> = []
    await new ApiClient().stream('/api/ai/stream', {}, (e, d) => events.push([e, d]))
    expect(events).toEqual([
      ['token', '{"token":"first"}'],
      ['done', '[DONE]'],
    ])
  })

  it('流尾为截断的多字节字符时：flush 字节必须接在未闭合尾部事件之后', async () => {
    const encoder = new TextEncoder()
    const first = encoder.encode('data: hel') // 未闭合尾部事件（无终止空行）
    const truncated = encoder.encode('好').slice(0, 2) // 截断的多字节字符，flush 时输出 U+FFFD
    const stream = new ReadableStream<Uint8Array>({
      start(controller) {
        controller.enqueue(first)
        controller.enqueue(truncated)
        controller.close()
      },
    })
    vi.stubGlobal(
      'fetch',
      vi
        .fn()
        .mockResolvedValue(new Response(stream, { status: 200, headers: { 'Content-Type': 'text/event-stream' } })),
    )
    const events: Array<[string, string]> = []
    await new ApiClient().stream('/api/ai/stream', {}, (e, d) => events.push([e, d]))
    // 修复前：flush 字节被拼在尾部事件之前，最后一条事件内容错乱。
    expect(events).toEqual([['message', 'hel\uFFFD']])
  })
})
