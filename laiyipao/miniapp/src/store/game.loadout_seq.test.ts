// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R280：loadLoadout 并发序号守卫（行为级回归）。
 *
 * me 页 onMounted 与 onShow 都会调 loadLoadout，bag 页熔炼/挂载、reactions 页
 * onLoad+onMounted 双调亦然。onShow 注释明确「从背包改过槽位再回来时重新拉权威
 * 槽位」——服务端槽位会在两次 loadLoadout 之间变化。弱网下先发的旧响应若后到，
 * 会用旧槽位覆盖新槽位；而 loadout 参与回放哈希，且是防线快照的权威来源，
 * 覆盖后会把玩家没装备的技能写进防线。用序号守卫丢弃陈旧响应，
 * 与同文件 refreshProfile / refreshWallet 守卫同构。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useGameStore } from './game'
import * as api from '@/api/client'
import { installUniMock } from '../test/uni-mock'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  login: vi.fn(),
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  saveLastUsername: vi.fn(),
  onAuthFailure: vi.fn(() => () => {}),
}))

const mockApi = vi.mocked(api, true)
let store: ReturnType<typeof useGameStore>

beforeEach(() => {
  installUniMock()
  setActivePinia(createPinia())
  store = useGameStore()
  vi.spyOn(console, 'warn').mockImplementation(() => {})
})

afterEach(() => {
  vi.restoreAllMocks()
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('loadLoadout 并发序号守卫（R280）', () => {
  it('并发时旧 loadout 响应后到不覆盖新响应的 loadout 与 equippedSkillIds', async () => {
    // 旧请求先发但挂起（模拟弱网），新请求后发但先返回。
    let resolveOld!: (v: any) => void
    const oldLoadout = new Promise<any>((r) => { resolveOld = r })
    mockApi.fetchLoadout.mockImplementationOnce(() => oldLoadout)
    const stale = store.loadLoadout()

    mockApi.fetchLoadout.mockResolvedValueOnce({ skill_ids: [9, 8, 7, 6] })
    const fresh = store.loadLoadout()

    // 新请求落地：槽位与出战技能都是最新值。
    await fresh
    expect(store.loadout).toEqual([9, 8, 7, 6])
    expect(store.equippedSkillIds).toEqual([9, 8, 7, 6])

    // 旧请求后到：必须被序号守卫丢弃，不许写回旧槽位。
    resolveOld({ skill_ids: [1, 2] })
    await stale
    expect(store.loadout).toEqual([9, 8, 7, 6])
    expect(store.equippedSkillIds).toEqual([9, 8, 7, 6])
  })

  it('persistLoadout 成功后，在途的旧 loadLoadout 响应不得覆盖新保存的权威槽位（R282）', async () => {
    // 跨页竞态：me 页 onShow 发起 loadLoadout（弱网挂起），用户切到 bag 页改槽位并
    // persistLoadout 成功写入新权威槽位；此时挂起的旧 loadLoadout 响应才后到。
    // loadoutReqSeq 是跨页共享的序号，persist 的权威写必须让在途 load 失效，
    // 否则旧槽位会覆盖刚保存的权威槽位（loadout 参与回放哈希、是防线快照权威来源）。
    let resolveStale!: (v: any) => void
    const staleResp = new Promise<any>((r) => { resolveStale = r })
    mockApi.fetchLoadout.mockImplementationOnce(() => staleResp)
    const stale = store.loadLoadout() // seq=N，挂起

    mockApi.saveLoadout.mockResolvedValueOnce({ skill_ids: [9, 8, 7, 6] })
    expect(await store.persistLoadout([9, 8, 7, 6])).toBe(true)
    expect(store.loadout).toEqual([9, 8, 7, 6])

    // 旧 loadLoadout 响应后到，携带保存前的旧槽位。
    resolveStale({ skill_ids: [1, 2, 3, 4] })
    await stale
    expect(store.loadout).toEqual([9, 8, 7, 6])
    expect(store.equippedSkillIds).toEqual([9, 8, 7, 6])
  })
})
