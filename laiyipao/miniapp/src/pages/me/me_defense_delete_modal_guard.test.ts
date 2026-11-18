// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 116 轮 laiyipao 真 bug（R266）：me.vue 删除防线二次确认弹窗期没有在途守卫。
 *
 * onDeleteMyDefense() 先 await uni.showModal 的 Promise，确认后才调
 * api.deleteMyDefense()。但弹窗挂起期间没有任何 guard 置位：玩家双击
 * 「删除防线」会弹出两个确认框，两次都点「删除」就发两次删除请求；
 * 第一次成功删除防线，第二次服务端返回 ErrNotFound「你还没有设置防线」
 * （progression.go DeleteMyDefense：DELETE 影响 0 行即报错），弹莫名错误。
 *
 * 与已修的 battle.vue abandon()（放弃局）、shop.vue doBuy()（购买）
 * 完全同构：守卫必须在 showModal 之前置位，取消与 finally 都要复位。
 *
 * 本测试令弹窗挂起，模拟双击：断言只弹一次确认、只发一次删除请求。
 * 另断言取消后守卫复位，玩家仍能再次点击完成删除（防过度守卫）。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Me from './me.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
import { mountPage } from '../../test/page'
import { makeConfig, makeLevel, makeSkill } from '../../test/fixtures'
import type { DefenseView } from '@/game/defense'

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
  fetchDefenses: vi.fn(),
  saveDefense: vi.fn(),
  deleteMyDefense: vi.fn(),
  challengeDefense: vi.fn(),
  fetchLevel: vi.fn(),
  fetchMyEquipment: vi.fn(),
  fetchMastery: vi.fn(),
  updateNickname: vi.fn(),
  myChallengeHistory: vi.fn(),
  myDefenseChallenges: vi.fn(),
  fetchMyStars: vi.fn(),
  myBattleHistory: vi.fn(),
  myPurchases: vi.fn(),
  myRedemptions: vi.fn(),
  myOrders: vi.fn(),
}))

vi.mock('@/game/defense', async (importOriginal) => {
  const actual = await importOriginal<typeof import('@/game/defense')>()
  return {
    runChallenge: vi.fn(),
    validateSnapshot: vi.fn((c: DefenseView) => ({ ok: !String(c.snapshot_hash).startsWith('bad') })),
    isShieldActive: actual.isShieldActive,
  }
})

const mockApi = vi.mocked(api, true)

function defenseView(over: Partial<DefenseView> = {}): DefenseView {
  return {
    id: 5,
    name: '我的防线',
    owner_name: '对手',
    power: 100,
    element_coverage: 3,
    wins: 1,
    losses: 2,
    snapshot_hash: 'abcdef1234567890',
    snapshot: { skills: [1] },
    can_challenge: true,
    challenge_blocked: '',
    ...over,
  } as DefenseView
}

let um: ReturnType<typeof installUniMock>

beforeEach(() => {
  um = installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified',
    config: makeConfig({ levels: [makeLevel()], skills: [makeSkill({ id: 1, apply_element: '' })] }),
    serverTimeMs: 0,
  })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1] })
  mockApi.fetchMyEquipment.mockResolvedValue({ items: [] })
  mockApi.fetchMastery.mockResolvedValue({ points: 3, nodes: [11], families: [] })
  mockApi.fetchDefenses.mockResolvedValue({
    mine: defenseView({ shielded_until: '', my_attempts_today: 0 } as any),
    candidates: [],
    attempt_limit: 5,
  } as any)
  mockApi.saveDefense.mockResolvedValue({ defense: defenseView() } as any)
  mockApi.deleteMyDefense.mockResolvedValue({ ok: true } as any)
  mockApi.fetchLevel.mockResolvedValue(makeLevel({ id: 1, waves: [{ wave_index: 0, spawns: [] }] }) as any)
  mockApi.myDefenseChallenges.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myChallengeHistory.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.fetchMyStars.mockReset().mockResolvedValue({ stars: {} } as any)
  mockApi.myBattleHistory.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myPurchases.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myRedemptions.mockReset().mockResolvedValue({ items: [] } as any)
  mockApi.myOrders.mockReset().mockResolvedValue({ items: [] } as any)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

async function mountedWithDefense() {
  const { wrapper } = mountPage(Me)
  await flushPromises()
  triggerUniHook(wrapper.vm, 'onShow')
  await flushPromises()
  const delBtn = wrapper.findAll('.btn').find((b) => b.text().includes('删除防线'))
  expect(delBtn).toBeDefined()
  return { wrapper, delBtn: delBtn! }
}

describe('me.vue 删除防线确认弹窗期的在途守卫（R266）', () => {
  it('弹窗待决期间再次点击删除防线只弹一次确认、只发一次删除请求', async () => {
    // 弹窗全部挂起；保存每个 resolve，最后一次性确认所有已弹出的框。
    const resolvers: Array<() => void> = []
    um.mock.showModal.mockImplementation((opts: any) => {
      resolvers.push(() => opts.success?.({ confirm: true, cancel: false }))
    })

    const { delBtn } = await mountedWithDefense()

    // 第一次点击：弹窗挂起，玩家尚未确认
    await delBtn.trigger('click')
    await flushPromises()
    expect(um.mock.showModal).toHaveBeenCalledTimes(1)

    // 弹窗仍待决时再次点击：修复前守卫未置位 → 第二个确认框；
    // 两个都确认 → 两次 deleteMyDefense，第二次被服务端判为 ErrNotFound。
    await delBtn.trigger('click')
    await flushPromises()
    expect(um.mock.showModal).toHaveBeenCalledTimes(1)

    resolvers.forEach((resolve) => resolve())
    await flushPromises()
    expect(mockApi.deleteMyDefense).toHaveBeenCalledTimes(1)
  })

  it('取消确认后守卫复位，玩家仍可再次删除', async () => {
    let confirmed = false
    let resolveModal: () => void = () => {}
    um.mock.showModal.mockImplementation((opts: any) => {
      resolveModal = () => opts.success?.({ confirm: confirmed, cancel: !confirmed })
    })

    const { delBtn } = await mountedWithDefense()

    // 第一次点击后取消：不应发删除请求
    await delBtn.trigger('click')
    await flushPromises()
    resolveModal()
    await flushPromises()
    expect(mockApi.deleteMyDefense).not.toHaveBeenCalled()

    // 守卫已复位 → 再次点击并确认 → 正常删除一次
    confirmed = true
    await delBtn.trigger('click')
    await flushPromises()
    resolveModal()
    await flushPromises()
    expect(mockApi.deleteMyDefense).toHaveBeenCalledTimes(1)
  })
})
