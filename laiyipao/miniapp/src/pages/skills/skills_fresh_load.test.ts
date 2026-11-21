// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R3 真实缺陷回归：技能图鉴深链接/刷新进入时没有任何代码拉 /config，
 * 页面却显示「技能表为空（配置未加载？）」，把未加载谎报成空表。
 * 这里不给 store.config 赋值、不替页面调 loadConfig()。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises } from '@vue/test-utils'

import Skills from './skills.vue'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig, makeSkill } from '../../test/fixtures'

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

describe('技能图鉴冷启动加载（R3）', () => {
  it('直接打开 /pages/skills/skills 时页面自行拉 /config 并渲染技能表', async () => {
    const cfg = makeConfig({
      skills: [
        makeSkill({ id: 1, name: '烈焰弹', code: 'fresh_fire' }),
        makeSkill({ id: 2, name: '寒冰弹', code: 'fresh_ice', element: 'ice' as never, family: 'frost' }),
      ],
      composite_skills: [],
    })
    fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: cfg, serverTimeMs: 0 })

    const { wrapper, store } = mountPage(Skills, (s) => {
      s.loggedIn = true
      s.config = null
    })
    expect(store.config).toBeNull()

    await flushPromises()

    expect(fetchConfigIfVersion).toHaveBeenCalledTimes(1)
    expect(wrapper.findAll('.rxn.skill')).toHaveLength(2)
    expect(wrapper.text()).toContain('烈焰弹')
    expect(wrapper.text()).not.toContain('技能表为空（配置未加载？）')
  })
})
