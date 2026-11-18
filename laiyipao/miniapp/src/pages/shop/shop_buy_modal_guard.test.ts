// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 商城购买确认弹窗期的在途守卫回归（R265）。
 *
 * doBuy 的 buyingId 守卫曾设在 showModal 的 await **之后**：
 * 弹窗待决期间 buyingId 仍是 0，连点第二件会穿过守卫弹出第二个确认框，
 * 两个都确认就发两次 buyItem —— 而 buyItem 每次调用生成不同幂等键
 * （api/client.ts newIdempotencyKey），服务端按两笔成交，玩家被扣两次币。
 *
 * 参照 battle.vue abandon() 的既定做法：守卫必须在 showModal 之前置位。
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

function shopList() {
  return [
    { id: 9, code: 'shop_gem', name: '随机宝石', category: 'gem',
      price: { gem: 60 }, payload: { gem_instance: 1 }, limit_per_day: 3, bought_today: 0 },
    { id: 8, code: 'shop_gem_wash', name: '洗练石', category: 'boost',
      price: { coin: 30000 }, payload: { gem_wash_token: 1 }, limit_per_day: 3, bought_today: 3 },
    { id: 5, code: 'shop_retry', name: '续战令', category: 'boost',
      price: { coin: 5000 }, payload: { revive_token: 1 }, limit_per_day: 5, bought_today: 0 },
  ]
}

beforeEach(() => {
  um = installUniMock()
  um.mock.showModal.mockImplementation((opts: any) => {
    opts.success?.({ confirm: true, cancel: false })
  })
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified', config: { skills: [], gems: [] } as any, serverTimeMs: 0,
  })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [] })
  mockApi.fetchShop.mockResolvedValue(shopList() as any)
  mockApi.fetchWallet.mockResolvedValue({ coin: 52000, gem: 290, energy: 3, keys: 4, tokens: {} })
  mockApi.buyItem.mockResolvedValue({} as any)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('商城确认弹窗期的在途守卫', () => {
  it('弹窗待决期间连点别的商品也只弹一次确认、只发一次购买', async () => {
    let resolveModal: () => void = () => {}
    um.mock.showModal.mockImplementation((opts: any) => {
      resolveModal = () => opts.success?.({ confirm: true, cancel: false })
    })
    let resolveBuy: (v: any) => void = () => {}
    mockApi.buyItem.mockImplementation(() => new Promise((r) => { resolveBuy = r }))

    const { wrapper } = mountPage(Shop)
    await flushPromises()

    // 点第一行（宝石 id=9）：弹窗挂起，用户尚未确认
    await wrapper.findAll('.shop-row').at(0)!.find('.link')!.trigger('click')
    await flushPromises()
    expect(um.mock.showModal).toHaveBeenCalledTimes(1)

    // 弹窗仍待决时点第三行（续战令 id=5，未售罄）：
    // 修复前 buyingId 还是 0，守卫穿透 → 第二个确认框、最终两笔成交。
    await wrapper.findAll('.shop-row').at(2)!.find('.link')!.trigger('click')
    await flushPromises()
    expect(um.mock.showModal).toHaveBeenCalledTimes(1)

    resolveModal()
    await flushPromises()
    expect(mockApi.buyItem).toHaveBeenCalledTimes(1)
    expect(mockApi.buyItem).toHaveBeenCalledWith(9)
    resolveBuy({})
    await flushPromises()
  })

  it('弹窗待决期间取消确认后，购买按钮恢复可点且不发请求', async () => {
    let resolveModal: () => void = () => {}
    um.mock.showModal.mockImplementation((opts: any) => {
      resolveModal = () => opts.success?.({ confirm: false, cancel: true })
    })
    const { wrapper } = mountPage(Shop)
    await flushPromises()

    await wrapper.findAll('.shop-row').at(0)!.find('.link')!.trigger('click')
    await flushPromises()
    resolveModal()
    await flushPromises()
    expect(mockApi.buyItem).not.toHaveBeenCalled()
    expect(wrapper.findAll('.shop-row').at(0)!.text()).toContain('购买')
  })
})
