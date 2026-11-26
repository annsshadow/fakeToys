// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R3 真实缺陷回归：敌人图鉴深链接/刷新进入时没有任何代码拉 /config，
 * 页面却显示「敌人表为空（配置未加载？）」。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises } from '@vue/test-utils'

import Enemies from './enemies.vue'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig, makeEnemy } from '../../test/fixtures'

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

describe('敌人图鉴冷启动加载（R3）', () => {
  it('直接打开 /pages/enemies/enemies 时页面自行拉 /config 并渲染敌人表', async () => {
    const cfg = makeConfig({
      enemies: [
        makeEnemy({ id: 1, name: '游荡者', code: 'fresh_wanderer' }),
        makeEnemy({ id: 2, name: '飞行者', code: 'fresh_flyer', category: 'flying' as never }),
      ],
    })
    fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: cfg, serverTimeMs: 0 })

    const { wrapper, store } = mountPage(Enemies, (s) => {
      s.loggedIn = true
      s.config = null
    })
    expect(store.config).toBeNull()

    await flushPromises()

    expect(fetchConfigIfVersion).toHaveBeenCalledTimes(1)
    expect(wrapper.findAll('.rxn.enemy')).toHaveLength(2)
    expect(wrapper.text()).toContain('游荡者')
    expect(wrapper.text()).not.toContain('敌人表为空（配置未加载？）')
  })
})
