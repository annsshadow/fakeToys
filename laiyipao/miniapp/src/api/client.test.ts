/**
 * API 客户端单元测试（node 环境，mock 全局 uni）。
 *
 * 覆盖目标：
 *  - uni.request 的成功 / HTTP 错误 / 网络失败 / 401 刷新重试全部分支
 *  - refresh token 的单飞（single-flight）：并发 401 只发一次刷新请求
 *    （不做单飞会把刚换来的新令牌 clearTokens 掉，静默登出 —— 必须锁死）
 *  - token 注入与 auth:false 的差异
 *  - 本地存储封装 get/set/del 的容错分支
 *  - 每个导出函数的 URL / method / body 拼装（与 Go 侧路由逐字对应）
 */
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import {
  request,
  ApiError,
  getToken,
  getGuestToken,
  setGuestToken,
  setTokenInternal,
  clearTokens,
  guestLogin,
  fetchWallet,
  fetchConfig,
  fetchMe,
  fetchLoadout,
  saveLoadout,
  upgradeSkill,
  startBattle,
  settleBattle,
  getReplay,
  verifyReplay,
  fetchMastery,
  allocateMastery,
  fetchTasks,
  claimTask,
  signIn,
  fetchShop,
  buyItem,
  redeem,
  fetchLeaderboard,
  fetchDefenses,
  saveDefense,
  challengeDefense,
  diagnose,
  BASE_URL,
} from './client'
import { installUniMock } from '../test/uni-mock'

/** 预排好的 uni.request 响应队列：按调用顺序弹出并同步回调 success/fail */
function queueResponses(resps: Array<{ statusCode?: number; data?: unknown; errMsg?: string }>) {
  um.mock.request.mockImplementation((opts: any) => {
    const r = resps.shift()
    expect(r, 'uni.request 调用次数超出队列长度').toBeTruthy()
    if (r!.errMsg !== undefined) opts.fail({ errMsg: r!.errMsg })
    else opts.success({ statusCode: r!.statusCode, data: r!.data })
  })
}

/** 取第 n 次请求的参数 */
function callOf(n: number): any {
  return um.mock.request.mock.calls[n][0]
}

let um: ReturnType<typeof installUniMock>

beforeEach(() => {
  um = installUniMock()
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('本地存储封装（uni storage 容错）', () => {
  it('token 读写与清除', () => {
    expect(getToken()).toBe('')
    setTokenInternal('acc-1', 'ref-1')
    expect(getToken()).toBe('acc-1')
    clearTokens()
    expect(getToken()).toBe('')
  })

  it('游客令牌读写', () => {
    expect(getGuestToken()).toBe('')
    setGuestToken('guest-1')
    expect(getGuestToken()).toBe('guest-1')
  })

  it('getStorageSync 抛异常时返回默认值（存储 API 不可用不炸）', () => {
    um.mock.getStorageSync.mockImplementation(() => {
      throw new Error('storage unavailable')
    })
    expect(getToken()).toBe('')
    expect(getGuestToken()).toBe('')
  })

  it('getStorageSync 返回非字符串（数字）时按缺失处理', () => {
    um.mock.getStorageSync.mockImplementation(() => 42 as unknown as string)
    expect(getToken()).toBe('')
  })

  it('setStorageSync 抛异常时静默退化（写失败不抛出）', () => {
    um.mock.setStorageSync.mockImplementation(() => {
      throw new Error('write failed')
    })
    expect(() => setGuestToken('g')).not.toThrow()
  })

  it('removeStorageSync 抛异常时 clearTokens 不抛出', () => {
    setTokenInternal('a', 'b')
    um.mock.removeStorageSync.mockImplementation(() => {
      throw new Error('remove failed')
    })
    expect(() => clearTokens()).not.toThrow()
  })
})

describe('request：基础分支', () => {
  it('2xx 直接 resolve 响应体；默认 GET、带 Content-Type、超时 20s', async () => {
    queueResponses([{ statusCode: 200, data: { hello: 1 } }])
    const p = request<{ hello: number }>('/ping')
    await expect(p).resolves.toEqual({ hello: 1 })

    const c = callOf(0)
    expect(c.url).toBe(`${BASE_URL}/ping`)
    expect(c.method).toBe('GET')
    expect(c.timeout).toBe(20000)
    expect(c.header['Content-Type']).toBe('application/json')
  })

  it('res.data 为 null 时按空对象收敛（?? 分支）', async () => {
    queueResponses([{ statusCode: 204, data: null }])
    await expect(request('/ping')).resolves.toEqual({})
  })

  it('有 token 时注入 Authorization；无 token 时不注入', async () => {
    setTokenInternal('acc-1', 'ref-1')
    queueResponses([{ statusCode: 200, data: {} }, { statusCode: 200, data: {} }])
    await request('/a')
    clearTokens()
    await request('/b')

    expect(callOf(0).header.Authorization).toBe('Bearer acc-1')
    expect(callOf(1).header.Authorization).toBeUndefined()
  })

  it('auth:false 不注入 Authorization（登录接口本身不能带过期令牌）', async () => {
    setTokenInternal('acc-1', 'ref-1')
    queueResponses([{ statusCode: 200, data: {} }])
    await request('/public', { auth: false })
    expect(callOf(0).header.Authorization).toBeUndefined()
  })

  it('非 2xx 非 401：拒绝 ApiError，优先用服务端 error.code/message', async () => {
    queueResponses([
      { statusCode: 422, data: { error: { code: 'energy_not_enough', message: '体力不足' } } },
    ])
    const err = (await request('/battle/token', { method: 'POST' }).catch((e) => e)) as ApiError
    expect(err).toBeInstanceOf(ApiError)
    expect(err.status).toBe(422)
    expect(err.code).toBe('energy_not_enough')
    expect(err.message).toBe('体力不足')
    expect(err.name).toBe('ApiError')
  })

  it('非 2xx 且响应体没有 error 字段：回退 http_error / HTTP {status}', async () => {
    queueResponses([{ statusCode: 500, data: { foo: 'bar' } }])
    const err = (await request('/x').catch((e) => e)) as ApiError
    expect(err.status).toBe(500)
    expect(err.code).toBe('http_error')
    expect(err.message).toBe('请求失败 HTTP 500')
  })

  it('网络失败（fail 回调）：ApiError(0, network_error)', async () => {
    queueResponses([{ errMsg: 'request:fail timeout' }])
    const err = (await request('/x').catch((e) => e)) as ApiError
    expect(err.status).toBe(0)
    expect(err.code).toBe('network_error')
    expect(err.message).toBe('无法连接服务器：request:fail timeout')
  })

  it('body 透传给 uni.request 的 data', async () => {
    queueResponses([{ statusCode: 200, data: {} }])
    await request('/x', { method: 'POST', body: { level_id: 3 } })
    expect(callOf(0).data).toEqual({ level_id: 3 })
  })
})

describe('request：401 刷新与重试', () => {
  it('401 → 刷新成功 → 用新令牌重试成功', async () => {
    setTokenInternal('expired', 'refresh-1')
    queueResponses([
      { statusCode: 401, data: {} }, // 原请求
      {
        statusCode: 200,
        data: { access_token: 'new-acc', refresh_token: 'new-ref' },
      }, // /auth/refresh
      { statusCode: 200, data: { ok: true } }, // 重试
    ])
    await expect(request('/me')).resolves.toEqual({ ok: true })

    // 刷新请求的拼装
    const refresh = callOf(1)
    expect(refresh.url).toBe(`${BASE_URL}/auth/refresh`)
    expect(refresh.method).toBe('POST')
    expect(refresh.data).toEqual({ refresh_token: 'refresh-1' })

    // 重试带的是**新** access token；存储里也换成新令牌对
    expect(callOf(2).header.Authorization).toBe('Bearer new-acc')
    expect(getToken()).toBe('new-acc')
  })

  it('刷新成功但重试仍非 2xx：拒绝重试响应的 ApiError', async () => {
    setTokenInternal('expired', 'refresh-1')
    queueResponses([
      { statusCode: 401, data: {} },
      { statusCode: 200, data: { access_token: 'new-acc', refresh_token: 'r' } },
      { statusCode: 403, data: { error: { code: 'forbidden', message: '禁止访问' } } },
    ])
    const err = (await request('/me').catch((e) => e)) as ApiError
    expect(err.status).toBe(403)
    expect(err.code).toBe('forbidden')
    expect(err.message).toBe('禁止访问')
  })

  it('刷新成功但重试网络失败：拒绝 network_error', async () => {
    setTokenInternal('expired', 'refresh-1')
    queueResponses([
      { statusCode: 401, data: {} },
      { statusCode: 200, data: { access_token: 'new-acc', refresh_token: 'r' } },
      { errMsg: 'request:fail abort' },
    ])
    const err = (await request('/me').catch((e) => e)) as ApiError
    expect(err.status).toBe(0)
    expect(err.code).toBe('network_error')
  })

  it('重试时新 access token 缺失 → 重试头不带 Authorization（if (t) 假分支）', async () => {
    setTokenInternal('expired', 'refresh-1')
    queueResponses([
      { statusCode: 401, data: {} },
      { statusCode: 200, data: { refresh_token: 'r' } }, // 没有 access_token
      { statusCode: 200, data: {} },
    ])
    await expect(request('/me')).resolves.toEqual({})
    expect(callOf(2).header.Authorization).toBeUndefined()
  })

  it('重试路径：data 为 null 且响应无 error 字段 → 回退 http_error / HTTP {status}', async () => {
    setTokenInternal('expired', 'refresh-1')
    queueResponses([
      { statusCode: 401, data: {} },
      { statusCode: 200, data: { access_token: 'new-acc', refresh_token: 'r' } },
      { statusCode: 502, data: null },
    ])
    const err = (await request('/me').catch((e) => e)) as ApiError
    expect(err.status).toBe(502)
    expect(err.code).toBe('http_error')
    expect(err.message).toBe('HTTP 502')
  })

  it('无 refresh token：清空令牌并拒绝 401 unauthorized（让上层重新登录）', async () => {
    setTokenInternal('expired', '')
    queueResponses([{ statusCode: 401, data: {} }])
    const err = (await request('/me').catch((e) => e)) as ApiError
    expect(err.status).toBe(401)
    expect(err.code).toBe('unauthorized')
    expect(err.message).toBe('登录已过期，请重新进入')
    expect(getToken()).toBe('')
  })

  it('刷新接口返回非 2xx：同样清空令牌并拒绝 401 unauthorized', async () => {
    setTokenInternal('expired', 'refresh-1')
    queueResponses([
      { statusCode: 401, data: {} },
      { statusCode: 400, data: { error: { code: 'bad_refresh' } } },
    ])
    const err = (await request('/me').catch((e) => e)) as ApiError
    expect(err.code).toBe('unauthorized')
    expect(getToken()).toBe('')
  })

  it('刷新请求网络失败：同样走 unauthorized', async () => {
    setTokenInternal('expired', 'refresh-1')
    queueResponses([{ statusCode: 401, data: {} }, { errMsg: 'request:fail net' }])
    const err = (await request('/me').catch((e) => e)) as ApiError
    expect(err.code).toBe('unauthorized')
  })

  it('401 但 auth:false：不走刷新，直接按普通错误拒绝', async () => {
    queueResponses([{ statusCode: 401, data: { error: { code: 'bad_token', message: '令牌无效' } } }])
    const err = (await request('/public', { auth: false }).catch((e) => e)) as ApiError
    expect(err.status).toBe(401)
    expect(err.code).toBe('bad_token')
    // 只有一次请求：没有刷新、没有重试
    expect(um.mock.request).toHaveBeenCalledTimes(1)
  })

  it('并发 401 单飞：N 个请求只发一次刷新（防止新令牌被误 clearTokens）', async () => {
    setTokenInternal('expired', 'refresh-1')
    // 注意时序：请求1 的 401 会**同步**发起刷新（doRefresh 的 Promise executor
    // 同步执行），所以刷新请求排在请求2 的 401 之前。
    queueResponses([
      { statusCode: 401, data: {} }, // 请求1
      { statusCode: 200, data: { access_token: 'new-acc', refresh_token: 'new-ref' } }, // 刷新（仅一次）
      { statusCode: 401, data: {} }, // 请求2（与1并发，命中同一个 in-flight 刷新）
      { statusCode: 200, data: { which: 1 } }, // 重试1
      { statusCode: 200, data: { which: 2 } }, // 重试2
    ])
    const [r1, r2] = await Promise.all([request('/wallet'), request('/me')])
    expect(r1).toEqual({ which: 1 })
    expect(r2).toEqual({ which: 2 })
    expect(um.mock.request).toHaveBeenCalledTimes(5)
    expect(callOf(1).url).toBe(`${BASE_URL}/auth/refresh`)
    // 两次重试都用新令牌
    expect(callOf(3).header.Authorization).toBe('Bearer new-acc')
    expect(callOf(4).header.Authorization).toBe('Bearer new-acc')
  })
})

describe('接口封装：URL / method / body 拼装', () => {
  /** 逐个导出函数断言请求参数 —— 路由与服务端逐字对应，拼错路径 404 很难从 UI 察觉 */
  const cases: Array<{
    name: string
    call: () => Promise<unknown>
    url: string
    method: string
    data?: unknown
    noAuth?: boolean
  }> = [
    { name: 'guestLogin 默认匿名', call: () => guestLogin(), url: '/auth/guest', method: 'POST', data: { guest_token: '', nickname: '' }, noAuth: true },
    { name: 'guestLogin 带昵称与游客令牌', call: () => { setGuestToken('gt-1'); return guestLogin('小明') }, url: '/auth/guest', method: 'POST', data: { guest_token: 'gt-1', nickname: '小明' }, noAuth: true },
    { name: 'fetchWallet', call: () => fetchWallet(), url: '/wallet', method: 'GET' },
    { name: 'fetchConfig 免鉴权', call: () => fetchConfig(), url: '/config', method: 'GET', noAuth: true },
    { name: 'fetchMe', call: () => fetchMe(), url: '/me', method: 'GET' },
    { name: 'fetchLoadout', call: () => fetchLoadout(), url: '/me/loadout', method: 'GET' },
    { name: 'saveLoadout', call: () => saveLoadout([1, 2, 3]), url: '/me/loadout', method: 'PUT', data: { skill_ids: [1, 2, 3] } },
    { name: 'upgradeSkill', call: () => upgradeSkill(7), url: '/me/skills/7/upgrade', method: 'POST' },
    { name: 'startBattle', call: () => startBattle(3), url: '/battle/token', method: 'POST', data: { level_id: 3 } },
    { name: 'settleBattle', call: () => settleBattle({ seed: '1' }), url: '/battle/settle', method: 'POST', data: { seed: '1' } },
    { name: 'getReplay', call: () => getReplay(9), url: '/battle/9/replay', method: 'GET' },
    { name: 'verifyReplay', call: () => verifyReplay(9, 'abc'), url: '/battle/verify', method: 'POST', data: { battle_id: 9, replay_hash: 'abc' } },
    { name: 'fetchMastery', call: () => fetchMastery(), url: '/mastery', method: 'GET' },
    { name: 'allocateMastery', call: () => allocateMastery(4), url: '/mastery/allocate', method: 'POST', data: { node_id: 4 } },
    { name: 'fetchTasks 默认 daily', call: () => fetchTasks(), url: '/tasks?scope=daily', method: 'GET' },
    { name: 'fetchTasks 指定 scope', call: () => fetchTasks('weekly'), url: '/tasks?scope=weekly', method: 'GET' },
    { name: 'claimTask', call: () => claimTask(2), url: '/tasks/2/claim', method: 'POST' },
    { name: 'signIn', call: () => signIn(), url: '/signin', method: 'POST' },
    { name: 'fetchShop', call: () => fetchShop(), url: '/shop', method: 'GET' },
    { name: 'buyItem', call: () => buyItem(5), url: '/shop/5/buy', method: 'POST' },
    { name: 'redeem', call: () => redeem('LYP2026'), url: '/redeem', method: 'POST', data: { code: 'LYP2026' } },
    { name: 'fetchLeaderboard 默认 power', call: () => fetchLeaderboard(), url: '/leaderboard?type=power', method: 'GET' },
    { name: 'fetchLeaderboard 指定类型', call: () => fetchLeaderboard('efficiency'), url: '/leaderboard?type=efficiency', method: 'GET' },
    { name: 'fetchDefenses', call: () => fetchDefenses(), url: '/defenses', method: 'GET' },
    { name: 'saveDefense', call: () => saveDefense({ name: 'x' }), url: '/defenses/save', method: 'POST', data: { name: 'x' } },
    { name: 'challengeDefense', call: () => challengeDefense(3, { won: true }), url: '/defenses/3/challenge', method: 'POST', data: { won: true } },
    { name: 'diagnose', call: () => diagnose(2, 3), url: '/diagnose?level_id=2&failed_times=3', method: 'GET' },
  ]

  for (const c of cases) {
    it(c.name, async () => {
      queueResponses([{ statusCode: 200, data: {} }])
      if (c.noAuth) {
        // 带着旧令牌也必须不带 Authorization（免鉴权接口）
        setTokenInternal('stale', 'stale')
      }
      await c.call()
      expect(um.mock.request).toHaveBeenCalledTimes(1)
      const req = callOf(0)
      expect(req.url).toBe(`${BASE_URL}${c.url}`)
      expect(req.method).toBe(c.method)
      if (c.data === undefined) expect(req.data).toBeUndefined()
      else expect(req.data).toEqual(c.data)
      if (c.noAuth) expect(req.header.Authorization).toBeUndefined()
    })
  }
})
