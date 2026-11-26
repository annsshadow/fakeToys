// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * F-10-2 集成：看板在数据加载后渲染 AI 洞察卡，结论来自当前数据（非硬编码占位）。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import type { Dashboard } from '@/api'
import { __resetReactionLabelCache } from '@/reactions'

const fetchDashboard = vi.hoisted(() => vi.fn())
const fetchReactions = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ fetchDashboard, fetchReactions }))
const echartsInit = vi.hoisted(() => vi.fn(() => ({ setOption: vi.fn(), dispose: vi.fn() })))
vi.mock('echarts/core', () => ({ use: vi.fn(), init: echartsInit }))

import DashboardView from '@/views/DashboardView.vue'

function fixture(overrides: Partial<Dashboard> = {}): Dashboard {
  return {
    users: { total: 682, guests: 0, wechat: 0, banned: 0, new_today: 4 },
    battles: { total: 144, today: 0, wins: 0, avg_duration_ms: 9000 },
    defenses: { total: 20, challenges_today: 0 },
    progression: { avg_max_stage: 0, avg_power: 0, stage_100_clears: 0 },
    economy: { coin_in: 0, coin_out: 0, gem_in: 0, gem_out: 0, orders_paid: 0, orders_pending: 0 },
    verification: { checked: 0, matched: 0, mismatched: 0 },
    reaction_usage: [{ reaction: 'steam_burst', count: 30 }],
    daily_active: [
      { date: '2026-01-01', dau: 100 },
      { date: '2026-01-02', dau: 120 },
      { date: '2026-01-03', dau: 150 },
    ],
    stage_funnel: [
      { level_id: 1, attempts: 500, clears: 480 },
      { level_id: 2, attempts: 400, clears: 120 },
      { level_id: 3, attempts: 100, clears: 90 },
    ],
    ...overrides,
  }
}
function reactionSpec(key: string, name: string) {
  return { key, name, base_coef: 60, attack_weight_pct: 300, status_duration_ms: 0, aoe_radius: 0, dispel_shield: false, amplify_pct: 0, descr: '' }
}
async function mountDashboard() {
  const w = mount(DashboardView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  await new Promise((r) => setTimeout(r, 60))
  await flushPromises()
  return w
}

beforeEach(() => {
  vi.clearAllMocks()
  __resetReactionLabelCache()
  fetchReactions.mockResolvedValue([reactionSpec('steam_burst', '蒸汽爆发')])
})

describe('看板 AI 洞察卡集成（F-10-2）', () => {
  it('加载完成后看板含 AI 卡，结论取自当前漏斗数据的最差关', async () => {
    fetchDashboard.mockResolvedValue(fixture())
    const w = await mountDashboard()
    const card = w.find('#ai-insight-card')
    expect(card.exists()).toBe(true)
    expect(card.find('.ai-headline').text()).toContain('第 2 关')
    expect(card.text()).toContain('规则引擎')
    expect(card.find('.ai-cta').exists()).toBe(true)
  })

  it('AI 卡位于 KPI 行之前（标题后的第一块内容）', async () => {
    fetchDashboard.mockResolvedValue(fixture())
    const w = await mountDashboard()
    const page = w.find('.page')
    const children = page.element.children
    const idxAi = Array.from(children).findIndex((el) => el.id === 'ai-insight-card')
    // F-10-3 重构后 KPI 用 .stat-section 承载（不再是 EP .el-row）；断言语义不变：AI 卡在 KPI 之前
    const idxKpi = Array.from(children).findIndex((el) => el.classList.contains('stat-section'))
    expect(idxAi).toBeGreaterThanOrEqual(0)
    expect(idxKpi).toBeGreaterThan(idxAi)
  })

  it('空数据（活跃不足 3 天）时看板 AI 卡显示诚实空态', async () => {
    fetchDashboard.mockResolvedValue(fixture({ daily_active: [{ date: '2026-01-01', dau: 100 }] }))
    const w = await mountDashboard()
    const card = w.find('#ai-insight-card')
    expect(card.find('.ai-empty').text()).toContain('数据不足')
    expect(card.find('.ai-cta').exists()).toBe(false)
  })
})
