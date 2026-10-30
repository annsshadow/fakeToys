// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 移动端业务 API 层的**表面完整性**测试：每个导出的端点方法都必须真的发出一次请求。
 *
 * 为什么需要这一层（与 `index.test.ts` / `tests/contracts/mobile-endpoints.test.ts` 分工）：
 * - `tests/contracts/mobile-endpoints.test.ts` 钉的是「路径字符串出现在源码里且后端已注册」；
 * - `index.test.ts` 钉的是「几个关键方法的请求体形状」；
 * - 本文件钉的是**另一件事**：每个方法被调用时，确实走到 `mapi` 并发出一个 `/api/` 请求，
 *   而不是在某个分支里提前 return、拼出空 URL、或者因为参数名写错而静默不动。
 *
 * 三者互补：只做契约扫描会漏掉「路径对但方法体坏了」；只做请求体断言会漏掉
 * 那 100 多个没人写用例的端点。这里的做法是**穷举导出表**，新增端点自动纳入。
 *
 * 边界：本文件不判断 URL 语义是否正确（那是契约测试的职责），只判断
 * 「有请求、有方法、路径在 /api/ 下、且不抛异常」。
 */

import { afterEach, describe, expect, it, vi } from 'vitest'
import * as services from './index'

type Captured = { method: string; url: string; data?: unknown }

/**
 * 端点方法的参数一律被透传（模板插值 / JSON 序列化），给足占位参数即可。
 * 数量取「最宽的那个签名 + 余量」：`queryviewApi.executeV2` 有 4 个参数，
 * 少给一个就会拼出 `/undefined` —— 那是测试的 bug，不是产品 bug。
 */
const DUMMY_ARGS = ['arg0', 'arg1', 'arg2', 'arg3', 'arg4'] as const

/** 最小 uni.request stub：记录调用并回一个成功信封。 */
function installRequestCapture(): Captured[] {
  const calls: Captured[] = []
  vi.stubGlobal('uni', {
    request: (opts: { method: string; url: string; data?: unknown; success: (r: unknown) => void }) => {
      calls.push({ method: opts.method, url: opts.url, data: opts.data })
      opts.success({ statusCode: 200, data: { success: true, data: null } })
    },
    // 上传走 uni.uploadFile（不是 request）：必须同样回调成功，
    // 否则 fileApi 的 Promise 永远不 settle，整条用例挂死。
    uploadFile: (opts: { success?: (r: unknown) => void }) => {
      calls.push({ method: 'UPLOAD', url: '', data: undefined })
      opts.success?.({
        statusCode: 200,
        data: JSON.stringify({ success: true, data: null }),
      })
    },
  })
  return calls
}

/** 导出表里所有「对象形态的 API 分组」（值必须是普通对象，排除 type / 函数 / 常量）。 */
function apiGroups(): Array<[string, Record<string, unknown>]> {
  return Object.entries(services).filter((entry): entry is [string, Record<string, unknown>] => {
    const value = entry[1]
    if (!value || typeof value !== 'object' || Array.isArray(value)) return false
    // 只保留「至少有一个函数成员」的对象，避免把普通常量对象算进来
    return Object.values(value).some((v) => typeof v === 'function')
  })
}

describe('mobile services surface', () => {
  afterEach(() => {
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('exposes the expected API groups', () => {
    // 分组名是跨端契约的一部分（desktop 与 mobile 用同名分组便于对照）；
    // 少一个分组说明某个页面整块能力丢了。
    //
    // `apis` 是聚合对象（成员是其它分组，不是函数），因此不进 apiGroups()，
    // 单独断言它存在。
    const names = apiGroups()
      .map(([name]) => name)
      .sort()
    expect(names).toEqual(
      [
        'annApi',
        'attendanceApi',
        'authApi',
        'bbsApi',
        'calendarApi',
        'cmsApi',
        'collectApi',
        'fileApi',
        'generalApi',
        'meetingApi',
        'messageApi',
        'orgApi',
        'portalApi',
        'processApi',
        'pushApi',
        'queryviewApi',
        'recycleApi',
        'searchApi',
        'statisticsApi',
      ].sort(),
    )
    expect(typeof services.apis).toBe('object')
  })

  // 穷举 130+ 端点，默认 5s 不够；这条用例本身就是「全表扫一遍」的量级。
  it('every exported endpoint issues exactly one /api/ request and never throws', { timeout: 60000 }, async () => {
    const failures: string[] = []
    let checked = 0

    for (const [groupName, group] of apiGroups()) {
      for (const [methodName, member] of Object.entries(group)) {
        if (typeof member !== 'function') continue
        const calls = installRequestCapture()
        checked += 1

        try {
          // 端点方法的参数一律被透传给 mapi（模板字符串插值 / JSON 序列化），
          // 因此传字符串足以走完路径拼接与请求体构造两条分支。
          await (member as (...args: unknown[]) => unknown)(...DUMMY_ARGS)
        } catch (error) {
          failures.push(`${groupName}.${methodName} threw: ${String(error)}`)
          vi.unstubAllGlobals()
          continue
        }

        const requestCalls = calls.filter((c) => c.method !== 'UPLOAD')
        if (requestCalls.length > 1) {
          failures.push(`${groupName}.${methodName} issued ${requestCalls.length} requests, expected at most 1`)
        } else if (requestCalls.length === 1 && !requestCalls[0].url.includes('/api/')) {
          failures.push(`${groupName}.${methodName} requested ${requestCalls[0].url} (not under /api/)`)
        } else if (requestCalls.length === 1 && !/^(GET|POST|PUT|DELETE)$/.test(requestCalls[0].method)) {
          failures.push(`${groupName}.${methodName} used method ${requestCalls[0].method}`)
        }

        vi.unstubAllGlobals()
      }
    }

    expect(failures, failures.join('\n')).toEqual([])
    // 防止「导出表被清空」时这条用例变成空转
    expect(checked).toBeGreaterThan(100)
  })

  it('endpoint paths carry no unresolved template placeholders', { timeout: 60000 }, async () => {
    // 参数插值漏了 `${}` 会得到 `/api/x/undefined` —— 后端 404，前端只看到空列表。
    const offenders: string[] = []

    for (const [groupName, group] of apiGroups()) {
      for (const [methodName, member] of Object.entries(group)) {
        if (typeof member !== 'function') continue
        const calls = installRequestCapture()
        await (member as (...args: unknown[]) => unknown)(...DUMMY_ARGS)
        const url = calls[0]?.url ?? ''
        if (url.includes('undefined') || url.includes('[object Object]') || url.includes('NaN')) {
          offenders.push(`${groupName}.${methodName} -> ${url}`)
        }
        vi.unstubAllGlobals()
      }
    }

    expect(offenders, offenders.join('\n')).toEqual([])
  })
})

/**
 * `clientDeviceToken()` 的行为守卫。
 *
 * 它是推送注册用的**设备凭据**（安全审计 2026-10 明确：可被重放的凭据，
 * 须用密码学安全随机数）。三条契约缺一不可：
 *   1. 首次生成后落 storage，之后**稳定返回同一个值**
 *      —— 每次调用都换新值等于每轮都注册一台新设备，推送目标会无限膨胀；
 *   2. storage 不可用（隐私模式/小程序受限）时必须降级而不是抛错；
 *   3. 有 Web Crypto 时用 `getRandomValues`（16 字节 = 32 位十六进制）。
 */
describe('clientDeviceToken', () => {
  afterEach(() => {
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  function installStorage(initial: Record<string, unknown> = {}) {
    const store: Record<string, unknown> = { ...initial }
    vi.stubGlobal('uni', {
      getStorageSync: (k: string) => store[k] ?? '',
      setStorageSync: (k: string, v: unknown) => {
        store[k] = v
      },
    })
    return store
  }

  it('generates an h5- token on first call and reuses it afterwards', () => {
    const store = installStorage()
    const first = services.clientDeviceToken()

    expect(first.startsWith('h5-')).toBe(true)
    expect(store.oa4rust_device_token).toBe(first)
    // 第二次必须命中 storage，而不是重新生成
    expect(services.clientDeviceToken()).toBe(first)
  })

  it('returns an already persisted token verbatim', () => {
    installStorage({ oa4rust_device_token: 'persisted-token' })
    expect(services.clientDeviceToken()).toBe('persisted-token')
  })

  it('survives storage read/write failures', () => {
    // 隐私模式下 getStorageSync 会抛；写也可能抛。两者都不得让登录流程中断。
    vi.stubGlobal('uni', {
      getStorageSync: () => {
        throw new Error('storage disabled')
      },
      setStorageSync: () => {
        throw new Error('quota exceeded')
      },
    })
    const token = services.clientDeviceToken()
    expect(token.startsWith('h5-')).toBe(true)
  })

  it('uses 32 hex chars from Web Crypto when available', () => {
    installStorage()
    const calls: number[] = []
    vi.stubGlobal('crypto', {
      getRandomValues: (buf: Uint8Array) => {
        calls.push(buf.length)
        buf.fill(0xab)
      },
    })

    const token = services.clientDeviceToken()

    expect(calls).toEqual([16]) // 16 字节
    expect(token).toBe(`h5-${'ab'.repeat(16)}`)
  })

  it('falls back to a time+random suffix when Web Crypto is absent', () => {
    // 原生/小程序目标可能没有 Web Crypto：降级路径也必须产出**互不相同**的值，
    // 否则同一台设备注册多次会撞同一个 token。
    installStorage()
    vi.stubGlobal('crypto', undefined)
    const a = services.clientDeviceToken()
    vi.stubGlobal('uni', {
      getStorageSync: () => '',
      setStorageSync: () => {},
    })
    const b = services.clientDeviceToken()

    expect(a.startsWith('h5-')).toBe(true)
    expect(b.startsWith('h5-')).toBe(true)
    expect(a).not.toBe(b)
  })
})
