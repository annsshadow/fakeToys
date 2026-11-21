// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R3 真实缺陷回归：宝石图鉴深链接/刷新进入时没有任何代码拉 /config，
 * 页面却一本正经地显示「宝石表为空（配置未加载？）」。
 *
 * 与 gems.test.ts 的区别：这里**不**手工给 store.config 赋值、也**不**替页面调用
 * loadConfig()——那正是此前测试漏掉页面自身加载职责的原因。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises } from '@vue/test-utils'

import Gems from './gems.vue'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig } from '../../test/fixtures'

const fetchConfigIfVersion = vi.hoisted(() => vi.fn())
const fetchGems = vi.hoisted(() => vi.fn())

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
  fetchGems,
}))

beforeEach(() => {
  installUniMock()
  vi.clearAllMocks()
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('宝石图鉴冷启动加载（R3）', () => {
  it('直接打开 /pages/gems/gems 时页面自行拉 /config 并渲染宝石表', async () => {
    const cfg = makeConfig({
      gems: [
        { id: 1, code: 'gem_crit', name: '锐锋石', attr: 'crit', descr: '暴击宝石' },
        { id: 2, code: 'gem_hp', name: '磐石', attr: 'hp', descr: '生命宝石' },
      ],
    })
    fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: cfg, serverTimeMs: 0 })
    fetchGems.mockResolvedValue({ gems: [] })

    // loggedIn=true 但 config=null：模拟会话里已登录、配置尚未加载的深链接/刷新。
    const { wrapper, store } = mountPage(Gems, (s) => {
      s.loggedIn = true
      s.config = null
    })
    expect(store.config).toBeNull()

    await flushPromises()

    // 页面必须自己负责加载权威配置，而不是靠调用方或测试代劳。
    expect(fetchConfigIfVersion).toHaveBeenCalledTimes(1)
    expect(wrapper.findAll('.rxn.gem')).toHaveLength(2)
    expect(wrapper.text()).toContain('锐锋石')
    expect(wrapper.text()).not.toContain('宝石表为空（配置未加载？）')
  })
})
