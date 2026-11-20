// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 商城分组渲染（服务端 ShopItemView.category 契约）。
 *
 * 服务端 economy.LoadShop / seed.go 会把商品按 category 原样下发：
 * currency（金币袋/体力/钥匙…）、starter（首充礼包）、equipment（装备）、
 * gem、boost 都是合法取值。商城页必须把每一种都渲染出来，
 * 否则玩家在客户端永远看不到也买不到这些商品。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Shop from '@/pages/shop/shop.vue'
import { installUniMock } from './uni-mock'
import { mountPage } from './page'

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
  fetchLevel: vi.fn(),
  fetchMyEquipment: vi.fn(),
  fetchMastery: vi.fn(),
  buyItem: vi.fn(),
  fetchShop: vi.fn(),
}))

const shopItems = [
  { id: 1, code: 'shop_coin_small', name: '金币袋', category: 'currency', price: { gem: 10 }, payload: { coin: 20000 }, limit_per_day: 3, bought_today: 0 },
  { id: 2, code: 'shop_energy', name: '体力补给', category: 'currency', price: { gem: 20 }, payload: { energy: 30 }, limit_per_day: 2, bought_today: 0 },
  { id: 3, code: 'shop_retry', name: '重试符', category: 'boost', price: { coin: 5000 }, payload: { revive_token: 1 }, limit_per_day: 5, bought_today: 0 },
  { id: 4, code: 'shop_gem', name: '随机宝石', category: 'gem', price: { gem: 100 }, payload: { gem_instance: 1 }, limit_per_day: 3, bought_today: 0 },
  { id: 5, code: 'shop_firstpay', name: '首充礼包', category: 'starter', price: { coin: 0 }, payload: { gem: 300, coin: 50000, energy: 50 }, limit_per_day: 1, bought_today: 0 },
  { id: 6, code: 'shop_eq_weapon', name: '制式炮管', category: 'equipment', price: { coin: 20000 }, payload: { equipment_1: 1 }, limit_per_day: 1, bought_today: 0 },
]

beforeEach(() => {
  installUniMock()
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('商城 category 分组渲染', () => {
  it('服务端下发的所有 category 商品都出现在页面上', async () => {
    const api = await import('@/api/client')
    vi.mocked(api.fetchShop).mockResolvedValue(shopItems as any)
    vi.mocked(api.fetchWallet).mockResolvedValue({ coin: 0, gem: 0, energy: 0, keys: 0 } as any)

    const { wrapper } = mountPage(Shop, (store) => {
      store.loggedIn = true
    })
    await flushPromises()

    const rendered = wrapper.text()
    for (const it of shopItems) {
      expect(rendered, `商品「${it.name}」(${it.category}) 必须可见`).toContain(it.name)
    }
    expect(wrapper.findAll('.shop-row').length).toBe(shopItems.length)
  })
})
