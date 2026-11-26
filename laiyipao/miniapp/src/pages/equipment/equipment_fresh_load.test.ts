// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R3 真实缺陷回归：装备图鉴深链接/刷新进入时没有任何代码拉 /config，
 * 页面却显示「装备表为空（配置未加载？）」。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises } from '@vue/test-utils'

import Equipment from './equipment.vue'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig } from '../../test/fixtures'

const fetchConfigIfVersion = vi.hoisted(() => vi.fn())

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfigIfVersion,
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
}))

beforeEach(() => {
  installUniMock()
  vi.clearAllMocks()
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('装备图鉴冷启动加载（R3）', () => {
  it('直接打开 /pages/equipment/equipment 时页面自行拉 /config 并渲染装备表', async () => {
    const cfg = makeConfig({
      equipment: [
        { id: 1, name: '焰纹长弓', element: 'fire', descr: '火系武器', slot: 'weapon', tier: 2 },
        { id: 2, name: '寒霜护甲', element: 'ice', descr: '冰系护甲', slot: 'armor', tier: 1 },
      ],
    })
    fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: cfg, serverTimeMs: 0 })

    const { wrapper, store } = mountPage(Equipment, (s) => {
      s.loggedIn = true
      s.config = null
    })
    expect(store.config).toBeNull()

    await flushPromises()

    expect(fetchConfigIfVersion).toHaveBeenCalledTimes(1)
    expect(wrapper.findAll('.rxn.equip')).toHaveLength(2)
    expect(wrapper.text()).toContain('焰纹长弓')
    expect(wrapper.text()).not.toContain('装备表为空（配置未加载？）')
  })
})
