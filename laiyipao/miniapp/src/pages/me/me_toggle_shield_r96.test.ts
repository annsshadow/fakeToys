// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 96 轮：护盾切换必须以服务端返回的 shielded_until 为准，
 * 不能用 shielded.value = !shielded.value 本地取反。
 * 服务端可能因并发/策略未真正开启（返回 shielded_until 仍为空），
 * 此时按钮必须维持「开启」且不能显示不存在的护盾剩余时间。
 */
import { describe, it, expect, vi, beforeEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Me from './me.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig, makeLevel, makeSkill } from '../../test/fixtures'
vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(), fetchConfigIfVersion: vi.fn(), fetchWallet: vi.fn(), fetchMe: vi.fn(),
  fetchLoadout: vi.fn(), saveLoadout: vi.fn(), setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false), onAuthFailure: vi.fn(() => () => {}),
  loadProfile: vi.fn(() => null), saveProfile: vi.fn(), login: vi.fn(), saveLastUsername: vi.fn(),
  fetchDefenses: vi.fn(), saveDefense: vi.fn(), challengeDefense: vi.fn(), fetchLevel: vi.fn(),
  fetchMyEquipment: vi.fn(), fetchMastery: vi.fn(), updateNickname: vi.fn(),
}))
vi.mock('@/game/defense', async (importOriginal) => {
  const actual = await importOriginal<typeof import('@/game/defense')>()
  return { runChallenge: vi.fn(), validateSnapshot: vi.fn(() => ({ ok: true })), isShieldActive: actual.isShieldActive }
})
const mockApi = vi.mocked(api, true)
function defenseView(over: Record<string, unknown> = {}): any {
  return { id: 5, name: '我的防线', owner_name: '对手', power: 100, element_coverage: 3,
    wins: 1, losses: 2, snapshot_hash: 'abcdef1234567890', snapshot: { skills: [1] },
    can_challenge: true, challenge_blocked: '', ...over }
}
beforeEach(() => {
  installUniMock()
  mockApi.guestLogin.mockResolvedValue({ access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 } } as any)
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified',
    config: makeConfig({ levels: [makeLevel()], skills: [makeSkill({ id: 1, apply_element: '' })] }), serverTimeMs: 0 } as any)
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} } as any)
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1] } as any)
  mockApi.fetchMyEquipment.mockResolvedValue({ items: [{ id: 1, code: 'w_p1', name: '炮', slot: 'weapon', tier: 1, element: 'kinetic', descr: '', base_armor: 0, base_bonus_pct: 50, unlock_level: 1, owned: true, equipped: true, level: 1, star: 1 }] } as any)
  mockApi.fetchMastery.mockResolvedValue({ points: 3, nodes: [11], families: [] } as any)
  mockApi.fetchLevel.mockResolvedValue(makeLevel({ id: 1, waves: [{ wave_index: 0, spawns: [] }] }) as any)
})

describe('me.vue toggleShield 以服务端返回为准', () => {
  it('服务端未给 shielded_until 时，按钮不翻转、也不显示护盾剩余时间', async () => {
    mockApi.fetchDefenses.mockResolvedValue({
      mine: defenseView({ shielded_until: undefined }), candidates: [], attempt_limit: 3,
    } as any)
    // 服务端没有真正开启护盾：返回的 defense 依然无 shielded_until
    mockApi.saveDefense.mockResolvedValue({ defense: defenseView({ shielded_until: undefined }) } as any)
    const { wrapper } = mountPage(Me)
    await flushPromises()
    const shieldBtn = () => wrapper.findAll('.btn').find((b) => b.text().includes('24h 护盾'))!
    expect(shieldBtn().text()).toContain('开启 24h 护盾')
    await shieldBtn().trigger('click')
    await flushPromises()
    expect(mockApi.saveDefense).toHaveBeenCalledWith(expect.objectContaining({ shield_hours: 24 }))
    // 权威状态来自服务端：仍是「开启」，且没有剩余时间文案
    expect(shieldBtn().text()).toContain('开启 24h 护盾')
    expect(wrapper.text()).not.toContain('护盾剩余')
  })
})
