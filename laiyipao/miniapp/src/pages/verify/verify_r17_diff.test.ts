// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R17 验真页「逐次反应序列对照卡」渲染契约（test-first）。
 *
 * 这条卡是**辅助线索**，不是验真判据：
 *  - 摘要只陈述真实的 diff 数字，不能编造；
 *  - 逐行对照只在差异处出现，首处差异按 1 基展示；
 *  - 旧战报缺 timeline 走诚实空态，不能冒充"一致"；
 *  - AI 四判据（汉字 AI / 真实结论 / 依据行 / 权威裁定 CTA）齐备，
 *    但绝不出现「验真通过/失败」这类权威词，也不复用 matched 作结论色。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Verify from './verify.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig } from '../../test/fixtures'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(), fetchConfigIfVersion: vi.fn(), fetchWallet: vi.fn(), fetchMe: vi.fn(),
  fetchLoadout: vi.fn(), saveLoadout: vi.fn(), setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false), onAuthFailure: vi.fn(() => () => {}),
  loadProfile: vi.fn(() => null), saveProfile: vi.fn(), login: vi.fn(), saveLastUsername: vi.fn(),
  getReplay: vi.fn(), verifyReplay: vi.fn(),
}))

vi.mock('@/game/replay', () => ({
  replay: vi.fn(),
  prettyHash: vi.fn((h: string) => 'pretty(' + h + ')'),
  prettyDuration: vi.fn((ms: number) => ms + 'ms'),
}))

const mockApi = vi.mocked(api, true)
const replayModule = await import('@/game/replay')
const runReplayMock = (vi.mocked(replayModule, true) as any).replay as ReturnType<typeof vi.fn>

const tl = (events: any[], truncated = false) => ({
  schema_version: 'reaction-timeline-v1', events, truncated, total: events.length,
})
const ev = (wave: number, tick: number, reaction: string, enemy_uid: number, damage: number) => ({
  wave, tick, reaction, enemy_uid, damage,
})

const replayInfoFixture = {
  battle_id: 9, level_id: 3, level: { wave_count: 2 }, seed: '42',
  build: { skills: {}, attacker: { attack: 100, crit_permille: 50, element_coef_permille: 400 } },
  created_at: '2026-01-01T00:00:00Z', expected_hash: 'abcdef0123456789',
}

beforeEach(() => {
  installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  } as any)
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: makeConfig(), serverTimeMs: 0 } as any)
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} } as any)
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.getReplay.mockResolvedValue(replayInfoFixture as any)
  mockApi.verifyReplay.mockResolvedValue({ matched: true, actual_hash: 'abc', expected_hash: 'abcdef0123456789' } as any)
  runReplayMock.mockReturnValue({
    matched: true, computedHash: 'abc',
    stats: { kills: 5, leaked: 1, waves: 2, durationMs: 1234 },
  } as any)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

async function fetchInfo(wrapper: any) {
  const input = wrapper.find('input')
  ;(input.element as HTMLInputElement).value = '9'
  await input.trigger('input')
  await wrapper.findAll('.btn').find((b: any) => b.text() === '取复现信息')!.trigger('click')
  await flushPromises()
}

async function doReplay(wrapper: any) {
  await wrapper.findAll('.btn').find((b: any) => b.text().includes('本地重放并提交'))!.trigger('click')
  await new Promise((r) => setTimeout(r, 60))
  await flushPromises()
}

describe('R17 验真页逐次反应序列对照卡', () => {
  it('两端有序列：摘要真实统计+仅差异行+首处差异+依据与权威裁定CTA', async () => {
    const local = [ev(1, 0, 'steam_burst', 7, 12), ev(1, 1, 'overheat', 8, 20.5), ev(2, 3, 'steam_burst', 9, 30)]
    const server = [ev(1, 0, 'steam_burst', 7, 12), ev(1, 1, 'overheat', 8, 99.9)]
    runReplayMock.mockReturnValue({
      matched: true, computedHash: 'abc',
      stats: { kills: 5, leaked: 1, waves: 2, durationMs: 1234 },
      reactionTimeline: local,
    } as any)
    mockApi.getReplay.mockResolvedValue({ ...replayInfoFixture, reaction_timeline: tl(server) } as any)

    const { wrapper } = mountPage(Verify)
    await flushPromises()
    await fetchInfo(wrapper)
    await doReplay(wrapper)

    const card = wrapper.find('.r17-reaction-diff')
    expect(card.exists(), '验真页应有 R17 序列对照卡').toBe(true)
    expect(card.attributes('role')).toBe('region')
    expect(card.text()).toContain('AI · 逐次反应序列对照')
    expect(card.find('.r17-diff-summary').text()).toContain('一致 1 条 · 差异 2 条')
    expect(card.find('.r17-first-diff').text()).toBe('首处差异：第 2 条')
    const rows = card.findAll('.r17-diff-row')
    expect(rows.map((r: any) => r.attributes('data-r17-index'))).toEqual(['1', '2'])
    expect(rows[0].text()).toContain('第 1 波 · 1 tick · 过热 · #8 · 20.5伤害')
    expect(rows[0].text()).toContain('第 1 波 · 1 tick · 过热 · #8 · 99.9伤害')
    expect(rows[1].text()).toContain('服务端')
    expect(card.text()).toContain('依据：本地重放 reaction_timeline.events 与服务端 reaction_timeline.events 逐条比对，辅助对照线索')
    expect(card.text()).toContain('辅助对照线索，非验真判据；权威裁定仍以 replay_hash 为准')
    expect(card.findAll('.verdict-box').length).toBe(0)
    expect(card.text()).not.toContain('验真通过')
    expect(card.text()).not.toContain('验真失败')
    const cta = card.findAll('button, .btn, a').find((b: any) => b.text().includes('查看权威裁定'))
    expect(cta, '应有查看权威裁定 CTA').toBeTruthy()
  })

  it('旧战报无 timeline：诚实空态，不声称一致', async () => {
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    await fetchInfo(wrapper)
    await doReplay(wrapper)
    const card = wrapper.find('.r17-reaction-diff')
    expect(card.exists()).toBe(true)
    expect(card.find('.r17-diff-empty').text()).toBe('该战报无逐次反应记录，暂无法对照（旧版本结算）')
    expect(card.find('.r17-diff-summary').exists()).toBe(false)
    expect(card.findAll('.r17-diff-row').length).toBe(0)
  })

  it('截断态逐字提示，且不改变 matched 的权威结论', async () => {
    runReplayMock.mockReturnValue({
      matched: true, computedHash: 'abc',
      stats: { kills: 5, leaked: 1, waves: 2, durationMs: 1234 },
      reactionTimeline: [ev(1, 0, 'steam_burst', 7, 12)], reactionTimelineTruncated: true,
    } as any)
    mockApi.getReplay.mockResolvedValue({
      ...replayInfoFixture,
      reaction_timeline: tl([ev(1, 0, 'steam_burst', 7, 12)], true),
    } as any)
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    await fetchInfo(wrapper)
    await doReplay(wrapper)
    const card = wrapper.find('.r17-reaction-diff')
    expect(card.text()).toContain('已截断，仅显示前 200 条')
  })
})
