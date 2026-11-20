// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 135 轮 laiyipao 真 bug（R285）：verify.vue 的 doReplay 缺少在途守卫。
 *
 * doReplay() 把 replaying 仅用于按钮文案，未做 return 守卫；
 * 第一次点击启动 schedulePending 的 30ms 延迟回调，回调触发后 pendingTimer
 * 被置 null，submit() 里 await verifyReplay 期间 replaying 仍为 true ——
 * 这段在途窗口内再次点击会启动新的延迟回调并发第二次 verifyReplay。
 * 服务端每次 VerifyReplay 都 INSERT 一行 replay_verifications（见
 * server/internal/service/verification.go 的 compareAndRecord/recordVerification），
 * 即使 recordVerification 按 (battle, verifier, hash) 幂等，双点也污染
 * 「本次验真记录」与统计口径。本测试让首次 verifyReplay 挂起，在途期间再点一次，
 * 断言只提交一次。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Verify from './verify.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig } from '../../test/fixtures'

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
  getReplay: vi.fn(),
  verifyReplay: vi.fn(),
}))

vi.mock('@/game/replay', () => ({
  replay: vi.fn(),
  prettyHash: vi.fn((h: string) => `pretty(${h})`),
  prettyDuration: vi.fn((ms: number) => `${ms}ms`),
}))

const mockApi = vi.mocked(api, true)
const replayModule = await import('@/game/replay')
const mockReplay = vi.mocked(replayModule, true)
const runReplayMock = mockReplay.replay as unknown as ReturnType<typeof vi.fn>

const replayInfoFixture = {
  battle_id: 9,
  level_id: 3,
  level: { wave_count: 2 },
  seed: '42',
  build: {
    skills: {
      d: { slot: 0, name: '燃烧弹', element: 'fire' },
    },
    attacker: { attack: 100, crit_permille: 50, element_coef_permille: 400 },
  },
  created_at: '2026-01-01T00:00:00Z',
  expected_hash: 'abcdef0123456789',
}

let releaseVerify: ((v: unknown) => void) | null = null

beforeEach(() => {
  installUniMock()
  releaseVerify = null
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  } as any)
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: makeConfig(), serverTimeMs: 0 } as any)
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} } as any)
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.getReplay.mockResolvedValue(replayInfoFixture as any)
  // 首次 verifyReplay 挂起，构造「提交在途」窗口
  mockApi.verifyReplay.mockImplementation((() => new Promise((resolve) => {
    releaseVerify = resolve
  })) as any)
  runReplayMock.mockReturnValue({
    matched: true,
    computedHash: 'abc',
    stats: { kills: 5, leaked: 1, waves: 2, durationMs: 1234 },
  } as any)
})

afterEach(async () => {
  releaseVerify?.({ matched: true, actual_hash: 'abc', expected_hash: 'abcdef0123456789' })
  await flushPromises()
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('verify.vue doReplay 在途守卫（R285）', () => {
  it('首次验真提交在途时再次点击不会重复提交 verifyReplay', async () => {
    const { wrapper } = mountPage(Verify)
    await flushPromises()

    // 取复现信息
    const input = wrapper.find('input')
    ;(input.element as HTMLInputElement).value = '9'
    await input.trigger('input')
    await wrapper.findAll('.btn').find((b: any) => b.text() === '取复现信息')!.trigger('click')
    await flushPromises()
    expect(mockApi.getReplay).toHaveBeenCalledWith(9)

    const replayBtn = () => wrapper.findAll('.btn').find((b: any) => b.text().includes('本地重放并提交'))!

    // 第一次点击：等 30ms 延迟回调跑完，verifyReplay 已进入在途（挂起）
    await replayBtn().trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(mockApi.verifyReplay).toHaveBeenCalledTimes(1)
    expect(releaseVerify).not.toBeNull()

    // 在途期间再点：无守卫时会启动新 timer 并发第二次 verifyReplay
    await replayBtn().trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(mockApi.verifyReplay).toHaveBeenCalledTimes(1)
  })
})
