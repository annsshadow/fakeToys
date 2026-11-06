// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { ApiClient, ApiError, AuthenticationError, PermissionError, RequestCancelledError } from './api'

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

/** 可由测试控制分块节奏的 SSE 响应；`error` 模拟浏览器 abort 时响应体报错。 */
function controllableResponse(): {
  response: Response
  push: (chunk: string) => void
  close: () => void
  error: (err: unknown) => void
} {
  const encoder = new TextEncoder()
  let ctrl: ReadableStreamDefaultController<Uint8Array> | null = null
  const stream = new ReadableStream<Uint8Array>({
    start(controller) {
      ctrl = controller
    },
  })
  return {
    response: new Response(stream, { status: 200, headers: { 'Content-Type': 'text/event-stream' } }),
    push: (chunk) => ctrl?.enqueue(encoder.encode(chunk)),
    close: () => ctrl?.close(),
    error: (err) => ctrl?.error(err),
  }
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
    const fetchMock = vi
      .fn()
      .mockResolvedValue(
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

    expect(fetchMock.mock.calls[0][0]).toBe('https://web.example.test/api/ai_assemble_control/chat/completion/stream')
    const init = fetchMock.mock.calls[0][1] as RequestInit
    expect(init.method).toBe('POST')
    expect(init.body).toBe(JSON.stringify({ message: 'hi' }))
    expect(events).toEqual([
      ['token', '{"conversationId":"c1","token":"你"}'],
      ['token', '{"token":"好"}'],
    ])
  })

  it('tolerates CRLF separators and non-token events without data loss', async () => {
    const fetchMock = vi
      .fn()
      .mockResolvedValue(sseResponse(['event: token\r\ndata: {"token":"a"}\r\n\nevent: done\r\ndata: [DONE]\r\n\r\n']))
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
    await expect(new ApiClient().stream('/api/x', {}, () => {})).rejects.toBeInstanceOf(PermissionError)

    const fetch401 = vi.fn().mockResolvedValue(new Response('no', { status: 401 }))
    vi.stubGlobal('fetch', fetch401)
    await expect(new ApiClient().stream('/api/x', {}, () => {})).rejects.toBeInstanceOf(AuthenticationError)
  })
})

describe('ApiClient.stream 调用方取消（R143）', () => {
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

  function abortableStream() {
    const encoder = new TextEncoder()
    let ctrl: ReadableStreamDefaultController<Uint8Array> | null = null
    const stream = new ReadableStream<Uint8Array>({
      start(controller) {
        ctrl = controller
      },
    })
    return {
      response: new Response(stream, { status: 200, headers: { 'Content-Type': 'text/event-stream' } }),
      push: (chunk: string) => ctrl?.enqueue(encoder.encode(chunk)),
      error: (err: unknown) => ctrl?.error(err),
    }
  }

  it('caller abort 真的断开连接，并以 RequestCancelledError 结束（不是空闲超时）', async () => {
    const s = abortableStream()
    let transportAborted = false
    vi.stubGlobal(
      'fetch',
      vi.fn((_url: string, init?: RequestInit) => {
        init?.signal?.addEventListener('abort', () => {
          transportAborted = true
          s.error(new DOMException('aborted', 'AbortError'))
        })
        return Promise.resolve(s.response)
      }),
    )

    const external = new AbortController()
    const p = new ApiClient().stream('/api/ai/stream', {}, () => {}, { signal: external.signal })
    s.push('event: token\ndata: {"token":"a"}\n\n')
    external.abort()

    await expect(p).rejects.toBeInstanceOf(RequestCancelledError)
    await expect(p).rejects.toMatchObject({ name: 'RequestCancelledError', status: 0 })
    expect(transportAborted).toBe(true)
  })

  it('已取消的 signal 传入时立即拒绝，不建立流', async () => {
    // 真实 fetch 在已 abort 的 signal 下会立刻 reject（不会发网络请求）。
    const fetchMock = vi.fn((_url: string, init?: RequestInit) =>
      init?.signal?.aborted
        ? Promise.reject(Object.assign(new Error('aborted'), { name: 'AbortError' }))
        : Promise.resolve(new Response('', { status: 200 })),
    )
    vi.stubGlobal('fetch', fetchMock)
    const external = new AbortController()
    external.abort()

    await expect(new ApiClient().stream('/api/x', {}, () => {}, { signal: external.signal })).rejects.toBeInstanceOf(
      RequestCancelledError,
    )
    // 没有读取任何流：连响应体都没有，调用方只看到可识别的取消。
    expect(fetchMock).toHaveBeenCalledTimes(1)
  })
})

describe('ApiClient.stream 空闲超时（R134）', () => {
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
    vi.useFakeTimers()
  })

  afterEach(() => {
    vi.useRealTimers()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  function fetchHonoringAbort(resp: () => Response, onAbort: () => void) {
    return vi.fn((_url: string, init?: RequestInit) => {
      init?.signal?.addEventListener('abort', onAbort)
      return Promise.resolve(resp())
    })
  }

  it('连续 60s 收不到任何字节：以 ApiError(status 0) 结束，而不是永远挂起', async () => {
    const ctrlStream = controllableResponse()
    // 浏览器 abort 会让未读的响应体报错，从而结束挂起的 reader.read()
    vi.stubGlobal(
      'fetch',
      fetchHonoringAbort(
        () => ctrlStream.response,
        () => ctrlStream.error(new DOMException('aborted', 'AbortError')),
      ),
    )
    const p = new ApiClient().stream('/api/ai/stream', {}, () => {})
    let settled = false
    void p.then(
      () => {
        settled = true
      },
      () => {
        settled = true
      },
    )
    await vi.advanceTimersByTimeAsync(59_000)
    expect(settled).toBe(false)
    await vi.advanceTimersByTimeAsync(2_000)
    expect(settled).toBe(true)
    await expect(p).rejects.toMatchObject({ name: 'ApiError', status: 0, message: '流式响应超时' })
    expect(vi.getTimerCount()).toBe(0)
  })

  it('服务端连响应头都不发：同样在 60s 判定死连接', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn(
        (_url: string, init?: RequestInit) =>
          new Promise<Response>((_resolve, reject) => {
            init?.signal?.addEventListener('abort', () => reject(new DOMException('aborted', 'AbortError')))
          }),
      ),
    )
    const p = new ApiClient().stream('/api/ai/stream', {}, () => {})
    let settled = false
    void p.then(
      () => {
        settled = true
      },
      () => {
        settled = true
      },
    )
    await vi.advanceTimersByTimeAsync(59_000)
    expect(settled).toBe(false)
    await vi.advanceTimersByTimeAsync(2_000)
    await expect(p).rejects.toBeInstanceOf(ApiError)
  })

  it('收到分块会重置计时：总时长 >60s 但每段间隔 <60s 不误杀', async () => {
    const ctrlStream = controllableResponse()
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(ctrlStream.response))
    const events: string[] = []
    const p = new ApiClient().stream('/api/ai/stream', {}, (_e, d) => events.push(d))
    let settled = false
    void p.then(
      () => {
        settled = true
      },
      () => {
        settled = true
      },
    )
    await vi.advanceTimersByTimeAsync(30_000)
    ctrlStream.push('event: token\ndata: {"token":"a"}\n\n')
    await vi.advanceTimersByTimeAsync(0)
    expect(events).toEqual(['{"token":"a"}'])
    // 距上一条数据已 59s（总时长 89s）：证明计时按「最后收字节」重置，不是总时长 60s
    await vi.advanceTimersByTimeAsync(59_000)
    expect(settled).toBe(false)
    ctrlStream.push('event: token\ndata: {"token":"b"}\n\n')
    await vi.advanceTimersByTimeAsync(0)
    expect(settled).toBe(false)
    ctrlStream.close()
    await vi.advanceTimersByTimeAsync(0)
    await p
    expect(events).toEqual(['{"token":"a"}', '{"token":"b"}'])
    expect(vi.getTimerCount()).toBe(0)
  })

  it('timeoutMs 可覆盖空闲超时（5s）', async () => {
    const ctrlStream = controllableResponse()
    vi.stubGlobal(
      'fetch',
      fetchHonoringAbort(
        () => ctrlStream.response,
        () => ctrlStream.error(new DOMException('aborted', 'AbortError')),
      ),
    )
    const p = new ApiClient().stream('/api/ai/stream', {}, () => {}, { timeoutMs: 5_000 })
    void p.catch(() => undefined)
    await vi.advanceTimersByTimeAsync(4_000)
    expect(vi.getTimerCount()).toBe(1)
    await vi.advanceTimersByTimeAsync(2_000)
    await expect(p).rejects.toMatchObject({ status: 0, message: '流式响应超时' })
  })
})
