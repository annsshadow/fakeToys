// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R20 实施：admin 看板「全平台战斗效率趋势卡」渲染行为契约（test-first）。
 *
 * 消费 GET /admin/battle-efficiency 的 BattleEfficiencyResp，
 * 断言汇总/趋势双视图、真实数据 toFixed(1) 格式化、趋势桶按日升序、
 * 逐字空态、AI 四判据与诚实红线（辅助诊断非权威）。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import type { BattleEfficiencyResp, Dashboard } from '@/api'
import { __resetReactionLabelCache } from '@/reactions'

const fetchDashboard = vi.hoisted(() => vi.fn())
const fetchReactions = vi.hoisted(() => vi.fn())
const fetchReactionDamageImpact = vi.hoisted(() => vi.fn())
const getBattleEfficiency = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ fetchDashboard, fetchReactions, fetchReactionDamageImpact, getBattleEfficiency }))
const echartsInit = vi.hoisted(() => vi.fn(() => ({ setOption: vi.fn(), dispose: vi.fn() })))
vi.mock('echarts/core', () => ({ use: vi.fn(), init: echartsInit }))

import DashboardView from '@/views/DashboardView.vue'

function dashboardFixture(overrides: Partial<Dashboard> = {}): Dashboard {
  return {
    users: { total: 682, guests: 0, wechat: 0, banned: 0, new_today: 4 },
    battles: { total: 144, today: 0, wins: 0, avg_duration_ms: 9000 },
    defenses: { total: 20, challenges_today: 0 },
    progression: { avg_max_stage: 0, avg_power: 0, stage_100_clears: 0 },
    economy: { coin_in: 0, coin_out: 0, gem_in: 0, gem_out: 0, orders_paid: 0, orders_pending: 0, sign_ins_today: 0, redeems_today: 0 },
    verification: { checked: 0, matched: 0, mismatched: 0 },
    reaction_usage: [{ reaction: 'overheat', count: 300 }],
    daily_active: [{ date: '2026-11-01', dau: 100 }],
    stage_funnel: [{ level_id: 1, attempts: 500, clears: 480 }],
    ...overrides,
  }
}

function bucket(b: string, shots: number, hits: number, accuracy: number) {
  return { bucket: b, battles: 1, shots, hits, accuracy_pct: accuracy }
}
function efficiencyFixture(overrides: Partial<BattleEfficiencyResp> = {}): BattleEfficiencyResp {
  return {
    window_days: 30,
    window_from: '2026-10-25',
    window_to: '2026-11-24',
    battles: 120,
    shots: 1000,
    hits: 457,
    accuracy_pct: 45.7,
    avg_heat_max: 63.26,
    leaked_per_battle: 2.34,
    avg_duration_ms: 48200,
    buckets: [
      bucket('2026-11-01', 20, 8, 40),
      bucket('2026-11-02', 10, 6, 60),
      bucket('2026-11-03', 30, 15, 50),
    ],
    ai_insight: {},
    ...overrides,
  }
}
function reactionSpec(key: string, name: string) {
  return { key, name, base_coef: 60, attack_weight_pct: 300, status_duration_ms: 0, aoe_radius: 0, spread_stacks: 0, dispel_shield: false, amplify_pct: 0, armor_shred_permille: 0, descr: '' }
}
async function mountDashboard() {
  const w = mount(DashboardView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  await new Promise((r) => setTimeout(r, 60))
  await flushPromises()
  return w
}
async function toTrendView(w: any) {
  await w.find('.r20-view-trend').trigger('click')
  await flushPromises()
}

beforeEach(() => {
  vi.clearAllMocks()
  __resetReactionLabelCache()
  fetchDashboard.mockResolvedValue(dashboardFixture())
  fetchReactions.mockResolvedValue([reactionSpec('overheat', '过热')])
  fetchReactionDamageImpact.mockResolvedValue({ window_days: 30, window_from: '', window_to: '', excluded_old_battles: 0, items: [] })
  getBattleEfficiency.mockResolvedValue(efficiencyFixture())
})

describe('R20 admin 全平台战斗效率趋势（渲染行为）', () => {
  it('容器 role=region、标题含可见汉字 AI、双视图按钮存在且默认汇总', async () => {
    const w = await mountDashboard()
    const card = w.find('.r20-efficiency-trend')
    expect(card.exists()).toBe(true)
    expect(card.attributes('role')).toBe('region')
    expect(card.text()).toContain('AI')
    expect(w.find('.r20-view-summary').exists()).toBe(true)
    expect(w.find('.r20-view-trend').exists()).toBe(true)
    expect(w.find('.r20-view-summary').attributes('aria-pressed')).toBe('true')
    expect(getBattleEfficiency).toHaveBeenCalled()
  })

  it('汇总视图：真实数据 + toFixed(1) 格式化，场次为整数，非有限数显示破折号', async () => {
    const w = await mountDashboard()
    let row = w.find('.r20-efficiency-row')
    expect(row.exists()).toBe(true)
    expect(row.text()).toContain('120')
    expect(row.text()).toContain('45.7%')
    expect(row.text()).toContain('63.3')
    expect(row.text()).toContain('2.3')
    expect(row.text()).toContain('48.2')
    w.unmount()

    getBattleEfficiency.mockResolvedValue(efficiencyFixture({
      accuracy_pct: Number.NaN,
      avg_heat_max: Number.POSITIVE_INFINITY,
    }))
    const w2 = await mountDashboard()
    row = w2.find('.r20-efficiency-row')
    expect(row.text()).toContain('—')
    expect(row.text()).not.toContain('NaN')
    expect(row.text()).not.toContain('Infinity')
  })

  it('趋势视图：buckets 按 bucket 升序，data-r20-bucket 逐字一致', async () => {
    const w = await mountDashboard()
    await toTrendView(w)
    const bars = w.findAll('.r20-trend-bar')
    expect(bars.length).toBe(3)
    expect(bars.map((b: any) => b.attributes('data-r20-bucket'))).toEqual([
      '2026-11-01', '2026-11-02', '2026-11-03',
    ])
    expect(w.find('.r20-view-trend').attributes('aria-pressed')).toBe('true')
  })

  it('空态 battles=0：逐字「近 30 天无战斗记录」，无趋势条，窗口仍如实显示', async () => {
    getBattleEfficiency.mockResolvedValue(efficiencyFixture({ battles: 0, shots: 0, hits: 0, buckets: [] }))
    const w = await mountDashboard()
    const card = w.find('.r20-efficiency-trend')
    expect(card.find('.r20-efficiency-empty').exists()).toBe(true)
    expect(card.find('.r20-efficiency-empty').text()).toBe('近 30 天无战斗记录')
    expect(card.text()).toContain('2026-10-25')
    expect(card.text()).toContain('2026-11-24')
    expect(card.text()).not.toContain('NaN')
    expect(card.text()).not.toContain('undefined')
    await toTrendView(w)
    expect(w.findAll('.r20-trend-bar').length).toBe(0)
  })

  it('AI 四判据：可见 AI + 真实结论(命中率+趋势) + 依据行逐字 + 查看战报 CTA', async () => {
    const w = await mountDashboard()
    const note = w.find('.r20-ai-note')
    expect(note.exists()).toBe(true)
    expect(note.attributes('role')).toBe('note')
    expect(note.text()).toContain('AI')
    expect(note.text()).toContain('45.7%')
    expect(note.text()).toContain('上升')
    expect(note.text()).toContain('依据：近 30 天 battle_records 的 shots/hits/heat_max 聚合；场次以 battle_records 行数为准')
    const cta = note.find('.r20-ai-cta')
    expect(cta.exists()).toBe(true)
    expect(cta.text()).toContain('查看战报')
    expect(cta.attributes('href')).toContain('/admin/battles')
  })

  it('诚实红线：辅助诊断非权威、不覆盖 replay_hash', async () => {
    const w = await mountDashboard()
    const note = w.find('.r20-honesty-note')
    expect(note.exists()).toBe(true)
    expect(note.text()).toContain('辅助诊断')
    expect(note.text()).toContain('不覆盖 replay_hash')
  })
})
