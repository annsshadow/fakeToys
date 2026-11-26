// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 宝石图鉴译名一致性回归（跨页面「同一个东西两个名字」真实缺陷）。
 *
 * gems.vue 曾自带一份 qualityName/attrLabel，译名与权威共享表
 * utils/labels.ts（背包 bag.vue 也用它）不一致：
 *   - gold 在背包显示「金」，图鉴却显示「传说」
 *   - crit 在背包显示「暴击」，图鉴却显示「暴击率」
 * 玩家会把同一颗宝石/同一个词条当成两种东西。这里用组件行为钉住。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Gems from './gems.vue'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
import { mountPage } from '../../test/page'
import { makeConfig } from '../../test/fixtures'

const fetchGems = vi.hoisted(() => vi.fn())

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
  fetchGems,
}))

const mockFetchGems = vi.mocked(fetchGems)

const cfg = () => makeConfig({
  gems: [
    { id: 1, code: 'gem_crit', name: '锐锋石', descr: '', attr: 'crit' },
  ],
} as any)

beforeEach(() => {
  installUniMock()
  vi.clearAllMocks()
})
afterEach(() => { vi.unstubAllGlobals() })

describe('宝石图鉴与背包共用同一套译名', () => {
  it('已拥有金品质宝石：图鉴行与品质环图例都显示「金」而不是「传说」', async () => {
    mockFetchGems.mockResolvedValue({
      gems: [
        { id: 7, gem_id: 1, code: 'gem_crit', name: '锐锋石', descr: '',
          quality: 'gold', affixes: [{ affix: 'crit', value: 30 }], equipped_slot: '' },
      ],
    })
    const { wrapper } = mountPage(Gems, (store) => {
      store.config = cfg()
      store.loggedIn = true
    })
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()

    const row = wrapper.find('.rxn.gem')
    expect(row.exists()).toBe(true)
    // 共享表 qualityName('gold') === '金'，背包同款；图鉴不能自造「传说」
    expect(row.text()).toContain('最佳 金')
    expect(row.text()).not.toContain('最佳 传说')
    expect(wrapper.find('.quality-legend').text()).toContain('金 1')
    expect(wrapper.find('.quality-legend').text()).not.toContain('传说')
  })

  it('属性词条走共享表：crit 显示「暴击」而不是「暴击率」', async () => {
    mockFetchGems.mockResolvedValue({ gems: [] })
    const { wrapper } = mountPage(Gems, (store) => {
      store.config = cfg()
      store.loggedIn = true
    })
    await flushPromises()

    const texts = wrapper.find('.rxn.gem').findAll('text').map((t) => t.text())
    expect(texts).toContain('暴击')
    expect(texts).not.toContain('暴击率')
  })

  it('R10 宝石图鉴给出背包镶嵌入口，避免把图鉴误当成镶嵌页', async () => {
    mockFetchGems.mockResolvedValue({ gems: [] })
    const { wrapper } = mountPage(Gems, (store) => {
      store.config = cfg()
      store.loggedIn = true
    })
    await flushPromises()

    const cta = wrapper.find('.r3-ai-cta.secondary')
    expect(cta.exists()).toBe(true)
    expect(cta.text()).toContain('去背包镶嵌')
    await cta.trigger('click')
    expect((globalThis as unknown as { uni: { navigateTo: ReturnType<typeof vi.fn> } }).uni.navigateTo).toHaveBeenCalledWith({ url: '/pages/bag/bag' })
  })
})
