// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { RequestCancelledError } from '@oa4rust/sdk'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { createSearchRunner } from './searchRunner'

function deferred<T>(): { promise: Promise<T>; resolve: (value: T) => void } {
  let resolve!: (value: T) => void
  const promise = new Promise<T>((res) => {
    resolve = res
  })
  return { promise, resolve }
}

/** 假 transport：只有 signal 被 abort 才以取消错误结束，或由测试手动放行。 */
function transport<T>(signal: AbortSignal, gate: { promise: Promise<T> }): Promise<T> {
  return new Promise<T>((resolve, reject) => {
    signal.addEventListener('abort', () => reject(new RequestCancelledError()), { once: true })
    gate.promise.then(resolve, reject)
  })
}

afterEach(() => {
  vi.useRealTimers()
})

describe('createSearchRunner：连续搜索只落地最后一次（R145）', () => {
  it('快速连续两次搜索：第一次被真断开，只有第二次结果落地', async () => {
    const runner = createSearchRunner()
    const gateA = deferred<string[]>()
    // 用对象承载回调内赋值，避免 TS 控制流把 null 初始化窄化成 never。
    const state: { signal: AbortSignal | null; oldLanded: boolean } = { signal: null, oldLanded: false }
    const first = runner.run(async (signal) => {
      state.signal = signal
      const rows = await transport(signal, gateA)
      state.oldLanded = true
      return rows
    })
    const second = runner.run(async (signal) => {
      if (signal.aborted) throw new RequestCancelledError()
      return ['B']
    })

    expect(await second).toEqual(['B'])
    expect(await first).toBeUndefined()
    expect(state.oldLanded).toBe(false)
    expect(state.signal?.aborted).toBe(true)
  })

  it('连续触发 10 次：前 9 次全部未落地，最后一次正常返回', async () => {
    const runner = createSearchRunner()
    const landed: number[] = []
    const gates = Array.from({ length: 10 }, () => deferred<number>())
    const results = gates.map((gate) =>
      runner.run(async (signal) => {
        const v = await transport(signal, gate)
        landed.push(v)
        return v
      }),
    )
    for (let i = 0; i < 9; i++) gates[i]?.resolve(i + 1)
    gates[9]?.resolve(10)

    const settled = await Promise.all(results.slice(0, 9))
    expect(settled.every((v) => v === undefined)).toBe(true)
    expect(await results[9]).toBe(10)
    expect(landed).toEqual([10])
  })

  it('取消错误静默：页面拿到 undefined，不会弹「请求失败」', async () => {
    const runner = createSearchRunner()
    const result = await runner.run(async () => {
      throw new RequestCancelledError()
    })
    expect(result).toBeUndefined()
  })

  it('真实失败照常抛给页面', async () => {
    const runner = createSearchRunner()
    await expect(
      runner.run(async () => {
        throw new Error('HTTP 500')
      }),
    ).rejects.toThrow('HTTP 500')
  })

  it('被取代的旧请求即使失败也不覆盖最新状态', async () => {
    const runner = createSearchRunner()
    const gate = deferred<void>()
    // 旧任务故意不理会 signal：模拟 SDK 之外的迟延失败。
    const first = runner.run(async () => {
      await gate.promise
      throw new Error('stale failure')
    })
    const second = runner.run(async () => 'B')
    expect(await second).toBe('B')

    // 旧请求晚到且抛真实错误：runner 按「已过期」丢弃，不抛给页面。
    gate.resolve()
    await expect(first).resolves.toBeUndefined()
  })

  it('onSettled 只由最新一次触发调用（旧请求不提前复位 loading）', async () => {
    const runner = createSearchRunner()
    const gate = deferred<string[]>()
    const settled: number[] = []
    const first = runner.run(
      async (signal) => {
        await transport(signal, gate)
        return ['A']
      },
      { onSettled: () => settled.push(1) },
    )
    const second = runner.run(async () => ['B'], { onSettled: () => settled.push(2) })

    await second
    await first
    expect(settled).toEqual([2])
  })
})

describe('createSearchRunner：输入驱动搜索的防抖（R145）', () => {
  it('300ms 内连续输入只发最后一次请求', async () => {
    vi.useFakeTimers()
    const runner = createSearchRunner({ debounceMs: 300 })
    const calls: string[] = []
    const trigger = (kw: string) =>
      runner.run(async () => {
        calls.push(kw)
        return kw
      })

    void trigger('a')
    await vi.advanceTimersByTimeAsync(50)
    void trigger('ab')
    await vi.advanceTimersByTimeAsync(50)
    void trigger('abc')
    await vi.advanceTimersByTimeAsync(300)

    expect(calls).toEqual(['abc'])
  })

  it('防抖等待期间再次触发：旧的在途请求立即被断开，不必等 300ms', async () => {
    vi.useFakeTimers()
    const runner = createSearchRunner({ debounceMs: 300 })
    const gate = deferred<string[]>()
    const state: { signal: AbortSignal | null; landed: boolean } = { signal: null, landed: false }
    const first = runner.run(async (signal) => {
      state.signal = signal
      await transport(signal, gate)
      state.landed = true
      return []
    })
    await vi.advanceTimersByTimeAsync(300)
    expect(state.signal?.aborted).toBe(false)

    const second = runner.run(async () => ['B'])
    // 关键：新触发发生的那一刻就断开旧请求，而不是等下一次防抖结束后才 abort。
    expect(state.signal?.aborted).toBe(true)
    await vi.advanceTimersByTimeAsync(300)

    expect(await second).toEqual(['B'])
    expect(await first).toBeUndefined()
    expect(state.landed).toBe(false)
  })

  it('cancel() 打断防抖等待：清空关键词不会再发出请求', async () => {
    vi.useFakeTimers()
    const runner = createSearchRunner({ debounceMs: 300 })
    const calls: string[] = []
    void runner.run(async () => {
      calls.push('a')
    })
    runner.cancel()
    await vi.advanceTimersByTimeAsync(300)
    expect(calls).toEqual([])
  })

  it('负的防抖值按 0 处理，立即发起', async () => {
    const runner = createSearchRunner({ debounceMs: -100 })
    const calls: number[] = []
    await runner.run(async () => {
      calls.push(1)
      return 1
    })
    expect(calls).toEqual([1])
  })
})
