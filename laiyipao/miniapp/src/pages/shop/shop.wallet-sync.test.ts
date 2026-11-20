// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 商城购买后「全局钱包写后失同步」回归测试。
 *
 * 商城是 revive_token / mastery_reset / gem_wash_token 的唯一发放途径。
 * load() 过去只刷新本页 wallet ref，不回写全局 store.wallet ——
 * 于是在商城买到洗练石、navigateBack 回背包（bag 页没有 onShow，onMounted 不重跑）后，
 * doWash 读到的仍是旧的 store.wallet.tokens.gem_wash_token，玩家明明买了券却被告知「没有洗练券」。
 * 本测试锁的是：购买后全局钱包必须跟随服务端钱包更新。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Shop from './shop.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false),
  onAuthFailure: vi.fn(() => () => {}),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  login: vi.fn(),
  saveLastUsername: vi.fn(),
  fetchShop: vi.fn(),
  buyItem: vi.fn(),
}))

const mockApi = vi.mocked(api, true)
let um: ReturnType<typeof installUniMock>

beforeEach(() => {
  um = installUniMock()
  um.mock.showModal.mockImplementation((opts: any) => {
    opts.success?.({ confirm: true, cancel: false })
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified', config: { skills: [], gems: [] } as any, serverTimeMs: 0,
  })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [] })
  mockApi.fetchShop.mockResolvedValue([
    {
      id: 8, code: 'shop_gem_wash', name: '洗练石', category: 'boost',
      price: { coin: 30000 }, payload: { gem_wash_token: 1 },
      limit_per_day: 0, bought_today: 0,
    },
  ] as any)
  mockApi.buyItem.mockResolvedValue({} as any)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('商城购买后同步全局钱包', () => {
  it('买到洗练石后，全局 store.wallet 跟随服务端更新（券数与余额）', async () => {
    mockApi.fetchWallet.mockResolvedValue({
      coin: 52000, gem: 290, energy: 3, keys: 4, tokens: { gem_wash_token: 0 },
    } as any)
    const { wrapper, store } = mountPage(Shop, (s) => {
      s.loggedIn = true
      s.wallet = { coin: 52000, gem: 290, energy: 3, keys: 4, tokens: { gem_wash_token: 0 } }
    })
    await flushPromises()
    expect(store.wallet.tokens.gem_wash_token).toBe(0)

    // 购买成功后服务端扣了金币、发了 1 张洗练券
    mockApi.fetchWallet.mockResolvedValue({
      coin: 22000, gem: 290, energy: 3, keys: 4, tokens: { gem_wash_token: 1 },
    } as any)
    await wrapper.findAll('.shop-row').at(0)!.find('.link')!.trigger('click')
    await flushPromises()
    await flushPromises()

    expect(mockApi.buyItem).toHaveBeenCalledWith(8)
    // 背包 doWash 读 store.wallet.tokens.gem_wash_token 判断能否洗练：
    // 少了这次同步，玩家在商城买了券、回背包仍被提示「没有洗练券」。
    expect(store.wallet.tokens.gem_wash_token).toBe(1)
    expect(store.wallet.coin).toBe(22000)
  })
})
