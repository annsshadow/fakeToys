// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { isRequestCancelled, RequestCancelledError } from '@oa4rust/sdk'
import { describe, expect, it } from 'vitest'
import { createRequestGuard } from './requestGuard'

/** 假 transport：不主动返回，只有 signal 被 abort 才结束——用来证明「连接真的断了」。 */
function fakeTransport(signal: AbortSignal): Promise<never> {
  return new Promise((_resolve, reject) => {
    signal.addEventListener('abort', () => reject(new RequestCancelledError()), { once: true })
  })
}

describe('createRequestGuard：切换即断开旧请求（R143）', () => {
  it('begin() 先 abort 上一次请求；旧请求以取消错误结束，新请求仍在等', async () => {
    const guard = createRequestGuard()
    const firstSignal = guard.begin()
    const firstRequest = fakeTransport(firstSignal)
    const secondSignal = guard.begin()

    await expect(firstRequest).rejects.toBeInstanceOf(RequestCancelledError)
    expect(firstSignal.aborted).toBe(true)
    expect(secondSignal.aborted).toBe(false)

    guard.abort()
    expect(secondSignal.aborted).toBe(true)
  })

  it('连续切换 10 次：前 9 个请求全部被断开，只有最后一次存活', async () => {
    const guard = createRequestGuard()
    const signals: AbortSignal[] = []
    const pending: Array<Promise<never>> = []
    for (let i = 1; i <= 10; i++) {
      const signal = guard.begin()
      signals.push(signal)
      pending.push(fakeTransport(signal))
    }

    await Promise.all(
      pending.slice(0, 9).map((request) => expect(request).rejects.toBeInstanceOf(RequestCancelledError)),
    )
    expect(signals.slice(0, 9).map((signal) => signal.aborted)).toEqual(Array(9).fill(true))
    expect(signals[9].aborted).toBe(false)
  })

  it('abort() 后再 begin()：新 signal 未被取消，可继续发起请求', () => {
    const guard = createRequestGuard()
    const first = guard.begin()
    guard.abort()
    expect(first.aborted).toBe(true)

    const second = guard.begin()
    expect(second.aborted).toBe(false)
  })

  it('取消错误可识别，普通请求失败不可识别（UI 据此决定是否静默）', () => {
    expect(isRequestCancelled(new RequestCancelledError())).toBe(true)
    expect(isRequestCancelled(new Error('boom'))).toBe(false)
    expect(isRequestCancelled(null)).toBe(false)
  })
})
