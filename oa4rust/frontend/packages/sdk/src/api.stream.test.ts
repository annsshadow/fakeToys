// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { ApiClient, AuthenticationError, PermissionError } from './api'

function sseResponse(chunks: string[], status = 200): Response {
  const encoder = new TextEncoder()
  const stream = new ReadableStream<Uint8Array>({
    start(controller) {
      for (const c of chunks) controller.enqueue(encoder.encode(c))
      controller.close()
    },
  })
  return new Response(stream, { status, headers: { 'Content-Type': 'text/event-stream' } })
}

describe('ApiClient.stream', () => {
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

  it('POSTs JSON and parses SSE token events across chunk boundaries', async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      sseResponse([
        'event: token\ndata: {"conversationId":"c1","token":"你"}\n\nevent: token\ndata',
        ': {"token":"好"}\n\n',
      ]),
    )
    vi.stubGlobal('fetch', fetchMock)

    const events: Array<[string, string]> = []
    await new ApiClient().stream('/api/ai_assemble_control/chat/completion/stream', { message: 'hi' }, (e, d) => {
      events.push([e, d])
    })

    expect(fetchMock.mock.calls[0][0]).toBe(
      'https://web.example.test/api/ai_assemble_control/chat/completion/stream',
    )
    const init = fetchMock.mock.calls[0][1] as RequestInit
    expect(init.method).toBe('POST')
    expect(init.body).toBe(JSON.stringify({ message: 'hi' }))
    expect(events).toEqual([
      ['token', '{"conversationId":"c1","token":"你"}'],
      ['token', '{"token":"好"}'],
    ])
  })

  it('tolerates CRLF separators and non-token events without data loss', async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      sseResponse(['event: token\r\ndata: {"token":"a"}\r\n\nevent: done\r\ndata: [DONE]\r\n\r\n']),
    )
    vi.stubGlobal('fetch', fetchMock)

    const events: Array<[string, string]> = []
    await new ApiClient().stream('/api/x', {}, (e, d) => events.push([e, d]))

    expect(events).toEqual([
      ['token', '{"token":"a"}'],
      ['done', '[DONE]'],
    ])
  })

  it('maps HTTP 403 to PermissionError and 401 (after failed refresh) to AuthenticationError', async () => {
    const fetch403 = vi.fn().mockResolvedValue(new Response('no', { status: 403 }))
    vi.stubGlobal('fetch', fetch403)
    await expect(
      new ApiClient().stream('/api/x', {}, () => {}),
    ).rejects.toBeInstanceOf(PermissionError)

    const fetch401 = vi.fn().mockResolvedValue(new Response('no', { status: 401 }))
    vi.stubGlobal('fetch', fetch401)
    await expect(
      new ApiClient().stream('/api/x', {}, () => {}),
    ).rejects.toBeInstanceOf(AuthenticationError)
  })
})
