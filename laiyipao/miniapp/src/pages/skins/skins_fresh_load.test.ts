// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R3 遗留真实缺陷回归：皮肤图鉴深链接/刷新进入时没有任何代码拉 /config，
 * 页面却显示「皮肤表还没加载出来 / 皮肤表为空（配置未加载？）」，
 * 把「尚未加载」谎报成「服务端没有数据」。
 *
 * 这里不给 store.config 赋值、不替页面调 loadConfig()。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises } from '@vue/test-utils'

import Skins from './skins.vue'
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

describe('皮肤图鉴冷启动加载（R3 遗留）', () => {
  it('直接打开 /pages/skins/skins 时页面自行拉 /config 并渲染皮肤表', async () => {
    const cfg = makeConfig({
      skins: [
        { id: 1, code: 'skin_ember', name: '焰纹战甲', rarity: 'epic', passive: 'fire_burn', descr: '焰系皮肤' },
        { id: 2, code: 'skin_frost', name: '霜纹披风', rarity: 'rare', passive: 'ice_freeze', descr: '冰系皮肤' },
      ],
    })
    fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: cfg, serverTimeMs: 0 })

    const { wrapper, store } = mountPage(Skins, (s) => {
      s.loggedIn = true
      s.config = null
    })
    expect(store.config).toBeNull()

    await flushPromises()

    expect(fetchConfigIfVersion).toHaveBeenCalledTimes(1)
    expect(wrapper.findAll('.rxn.skin')).toHaveLength(2)
    expect(wrapper.text()).toContain('焰纹战甲')
    expect(wrapper.text()).toContain('霜纹披风')
    expect(wrapper.text()).not.toContain('皮肤表为空（配置未加载？）')
    expect(wrapper.text()).not.toContain('皮肤表还没加载出来')
  })
})
