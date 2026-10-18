/**
 * 登出的凭证处理守卫。
 *
 * # 为什么这一组值得单独写
 *
 * 登出会**轮换** `guest_token`。客户端若没把新凭证存下来，
 * 下一次冷启动会带着已作废的旧 token 去登录 ——
 * 服务端的语义是「未知凭证 → 建新号」，于是玩家拿到一个**空账号**：
 * 金币、关卡、专精全部消失，**且没有任何报错**。
 *
 * 那比「凭证没轮换」严重得多：凭证问题静默，丢进度会被立刻发现
 * 并当成一个无法解释的 bug。所以这条顺序必须被机器守住，
 * 而不是靠「我记得要存」。
 *
 * # 复用既有的 uni-mock 而不是自己造
 *
 * 本项目已有 `src/test/uni-mock.ts`（内存 storage + 可编程 request），
 * `src/api/client.test.ts` 全部用它。第一版我在这里手搓了一份
 * `globalThis.uni` 并 stub `fetch` —— 结果**一条都没测到**：
 * 小程序端走的是 `uni.request` 而不是 `fetch`，fetch 从未被调用，
 * 而断言「本地状态没变」在「什么请求都没发」的情况下**照样成立**。
 *
 * 这正是「夹具错了却全绿」的形态，所以改成复用既有夹具。
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import { logout } from '@/api/client'
import { installUniMock, type UniMock } from './uni-mock'

const TOKEN_KEY = 'lyp_token'
const REFRESH_KEY = 'lyp_refresh'
const GUEST_KEY = 'lyp_guest'

let um: { mock: UniMock; storage: Map<string, string> }

/** 让下一次 uni.request 返回指定的响应。 */
function respond(statusCode: number, data: unknown) {
  um.mock.request.mockImplementationOnce((opts: any) =>
    opts.success({ statusCode, data }),
  )
}
/** 让下一次 uni.request 走 fail（网络层失败）。 */
function failNet(errMsg = 'request:fail net') {
  um.mock.request.mockImplementationOnce((opts: any) => opts.fail({ errMsg }))
}

beforeEach(() => {
  um = installUniMock()
  um.storage.set(TOKEN_KEY, 'access-1')
  um.storage.set(REFRESH_KEY, 'refresh-1')
  um.storage.set(GUEST_KEY, 'guest-old')
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('logout 的凭证处理', () => {
  it('夹具自检：storage 可写可读可删，request 可编程', () => {
    expect(um.storage.get(TOKEN_KEY)).toBe('access-1')
    um.storage.set('__probe', 'x')
    expect(um.storage.get('__probe')).toBe('x')
    um.mock.removeStorageSync('__probe')
    expect(um.storage.has('__probe')).toBe(false)
  })

  it('成功后：新 guest_token 写入，access/refresh 清掉', async () => {
    respond(200, { revoked: 2, guest_token: 'guest-new' })

    const res = await logout()

    expect(res.serverNotified).toBe(true)
    // ⚠️ 本文件的核心断言：新凭证必须落盘。
    expect(um.storage.get(GUEST_KEY)).toBe('guest-new')
    expect(um.storage.has(TOKEN_KEY)).toBe(false)
    expect(um.storage.has(REFRESH_KEY)).toBe(false)
  })

  it('服务端没返回 guest_token 时绝不能把本地那份清掉', async () => {
    // 「什么都不存」是安全的一侧：这次登出没生效，但下次还能登回去。
    // 反过来（setGuestToken('')）会让玩家**丢进度** —— 症状远更糟。
    respond(200, { revoked: 1 })

    const res = await logout()

    expect(res.serverNotified).toBe(true)
    expect(um.storage.get(GUEST_KEY)).toBe('guest-old')
    expect(um.storage.has(TOKEN_KEY)).toBe(false)
  })

  it('网络失败：清 access/refresh，但保留 guest_token', async () => {
    // 服务端没轮换它，本地那份仍然有效；清掉等于让玩家丢进度。
    failNet()

    const res = await logout()

    expect(res.serverNotified).toBe(false)
    expect(um.storage.get(GUEST_KEY)).toBe('guest-old')
    // 本机令牌必须清 —— 用户的意图是「我不信任这台设备了」
    expect(um.storage.has(TOKEN_KEY)).toBe(false)
    expect(um.storage.has(REFRESH_KEY)).toBe(false)
  })

  it('登出请求必须带 access token 并用 POST 打 /me/logout', async () => {
    // 只断言本地状态的话，一个「忘了 Authorization 头」的实现也能全绿 ——
    // 服务端会回 401、不轮换凭证、本地照清，四条断言全部通过，
    // 而「登出」其实从未真正发生。这条守住那个缺口。
    respond(200, { revoked: 1, guest_token: 'guest-new' })

    await logout()

    expect(um.mock.request).toHaveBeenCalledTimes(1)
    const opts = um.mock.request.mock.calls[0][0] as any
    expect(opts.url).toMatch(/\/me\/logout$/)
    expect(opts.method).toBe('POST')
    expect(opts.header.Authorization).toBe('Bearer access-1')
  })

  it('服务端 500：本地照清、凭证不换，如实返回「未通知」', async () => {
    respond(500, { error: { code: 'internal_error', message: 'x' } })

    const res = await logout()

    expect(res.serverNotified).toBe(false)
    expect(um.storage.get(GUEST_KEY)).toBe('guest-old')
    expect(um.storage.has(TOKEN_KEY)).toBe(false)
  })
})