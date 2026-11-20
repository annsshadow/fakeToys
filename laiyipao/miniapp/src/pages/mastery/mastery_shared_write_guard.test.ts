// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 专精页共享写守卫回归测试。
 *
 * toggle（点亮节点）与 resetMastery（重铸）都直接写同一份 selected / points，
 * 须共享同一条在途守卫；否则弱网下两笔请求交叉在途会互相覆盖对方回执，
 * 并用过期状态消耗重铸券（与 R275/R276/R277 同类缺口）。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Mastery from './mastery.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig } from '../../test/fixtures'
import type { MasteryFamily } from '@/game/types'

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
  fetchMastery: vi.fn(),
  allocateMastery: vi.fn(),
  resetMastery: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

function threeLayerFamily(): MasteryFamily {
  return {
    family: 'flame',
    name: '烈焰',
    nodes: [
      { id: 11, family: 'flame', layer: 1, slot: 0, name: '燃料压缩', kind: 'stat', value: 10, prereq_family: '', prereq_layer: 0 },
      { id: 12, family: 'flame', layer: 1, slot: 1, name: '焰心稳定', kind: 'stat', value: 10, prereq_family: '', prereq_layer: 0 },
      { id: 13, family: 'flame', layer: 2, slot: 0, name: '爆燃', kind: 'stat', value: 20, prereq_family: 'flame', prereq_layer: 1 },
      { id: 14, family: 'flame', layer: 2, slot: 1, name: '余烬', kind: 'stat', value: 20, prereq_family: 'flame', prereq_layer: 1 },
      { id: 17, family: 'flame', layer: 2, slot: 2, name: '引燃', kind: 'stat', value: 20, prereq_family: 'flame', prereq_layer: 1 },
      { id: 15, family: 'flame', layer: 3, slot: 0, name: '烈焰风暴', kind: 'stat', value: 40, prereq_family: 'flame', prereq_layer: 2 },
      { id: 16, family: 'flame', layer: 3, slot: 1, name: '灼烧链', kind: 'stat', value: 40, prereq_family: 'flame', prereq_layer: 2 },
    ],
  }
}

beforeEach(() => {
  installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: makeConfig({ mastery_families: [threeLayerFamily()] }), serverTimeMs: 0 })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: { mastery_reset: 2 } })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchMastery.mockResolvedValue({ points: 3, nodes: [11] })
  mockApi.allocateMastery.mockResolvedValue({ points: 2, nodes: [11, 12] })
  mockApi.resetMastery.mockResolvedValue({ reset_cleared: 1, points: 3, nodes: [], layer_limit: 2, base_slots: 4, total_slots: 4 })
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

function deferred<T>(): { promise: Promise<T>; resolve: (v: T) => void } {
  let resolve!: (v: T) => void
  const promise = new Promise<T>((res) => { resolve = res })
  return { promise, resolve }
}

describe('mastery.vue 点亮 / 重铸共享写守卫', () => {
  it('点亮请求在途时点重铸：重铸被挡，不带旧 selected 消耗重铸券', async () => {
    const alloc = deferred<any>()
    mockApi.allocateMastery.mockImplementation(() => alloc.promise)
    const { wrapper } = mountPage(Mastery)
    await flushPromises() // onMounted: login + load + refreshWallet

    // 在途窗口：已点亮 11，点未选节点 12 发起 allocate，allocBusy=true 且 selected 仍为 [11]
    const node12 = wrapper.findAll('.node').find((n) => n.text().includes('焰心稳定'))!
    await node12.trigger('click')
    await flushPromises()
    expect(mockApi.allocateMastery).toHaveBeenCalledTimes(1)
    expect(mockApi.resetMastery).not.toHaveBeenCalled()

    // 点亮在途期间点重铸：应被共享守卫挡下，不消耗道具
    await wrapper.find('.reset-row .link').trigger('click')
    await flushPromises()
    expect(mockApi.resetMastery).not.toHaveBeenCalled()

    // 点亮回执落定后，重铸按钮仍可用
    alloc.resolve({ points: 2, nodes: [11, 12] })
    await flushPromises()
    await wrapper.find('.reset-row .link').trigger('click')
    await flushPromises()
    expect(mockApi.resetMastery).toHaveBeenCalledTimes(1)
  })

  it('重铸请求在途时点亮节点：点亮被挡，不被重铸 nodes:[] 回执覆盖', async () => {
    const reset = deferred<any>()
    mockApi.resetMastery.mockImplementation(() => reset.promise)
    const { wrapper } = mountPage(Mastery)
    await flushPromises()

    // 在途窗口：初始已点亮 [11]，点重铸发起 reset，resetting=true 且 selected 仍为 [11]
    await wrapper.find('.reset-row .link').trigger('click')
    await flushPromises()
    expect(mockApi.resetMastery).toHaveBeenCalledTimes(1)

    // 重铸在途期间点节点 12：应被共享守卫挡下，不发起点亮
    const node12 = wrapper.findAll('.node').find((n) => n.text().includes('焰心稳定'))!
    await node12.trigger('click')
    await flushPromises()
    expect(mockApi.allocateMastery).not.toHaveBeenCalled()

    // 重铸回执落定后才允许点亮
    reset.resolve({ reset_cleared: 1, points: 3, nodes: [], layer_limit: 2, base_slots: 4, total_slots: 4 })
    await flushPromises()
    await node12.trigger('click')
    await flushPromises()
    expect(mockApi.allocateMastery).toHaveBeenCalledTimes(1)
  })
})
