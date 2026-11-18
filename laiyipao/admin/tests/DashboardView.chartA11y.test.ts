// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 第88轮：看板三张 ECharts Canvas 图对读屏不可见（WCAG 1.1.1）。
 * 容器是裸 <div>，既无 role/可访问名称，也无等价文本数据表。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import type { Dashboard } from '@/api'
import { __resetReactionLabelCache } from '@/reactions'

const fetchDashboard = vi.hoisted(() => vi.fn())
const fetchReactions = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ fetchDashboard, fetchReactions }))
const chartStub = { setOption: vi.fn(), dispose: vi.fn() }
const echartsInit = vi.hoisted(() => vi.fn())
vi.mock('echarts/core', () => ({ use: vi.fn(), init: echartsInit }))

import DashboardView from '@/views/DashboardView.vue'

function fixture(): Dashboard {
  return {
    users: { total: 1200, guests: 300, wechat: 900, banned: 5, new_today: 12 },
    battles: { total: 0, today: 3, wins: 0, avg_duration_ms: 9000 },
    defenses: { total: 20, challenges_today: 4 },
    progression: { avg_max_stage: 42.5, avg_power: 8800, stage_100_clears: 2 },
    economy: { coin_in: 100, coin_out: 40, gem_in: 7, gem_out: 3, orders_paid: 9, orders_pending: 1 },
    verification: { checked: 1000, matched: 990, mismatched: 10 },
    reaction_usage: [
      { reaction: 'steam_burst', count: 30 },
      { reaction: 'overheat', count: 10 },
      { reaction: 'custom_rx', count: 20 },
    ],
    daily_active: [
      { date: '2026-01-01', dau: 100 },
      { date: '2026-01-02', dau: 140 },
    ],
    stage_funnel: [{ level_id: 1, attempts: 500, clears: 300 }],
  }
}

async function mountDashboard() {
  const wrapper = mount(DashboardView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  await new Promise((r) => setTimeout(r, 60))
  await flushPromises()
  return wrapper
}

function reactionSpec(key: string, name: string) {
  return {
    key, name, base_coef: 60, attack_weight_pct: 300, status_duration_ms: 0,
    aoe_radius: 0, dispel_shield: false, amplify_pct: 0, descr: '',
  }
}

beforeEach(() => {
  vi.clearAllMocks()
  __resetReactionLabelCache()
  echartsInit.mockReturnValue(chartStub)
  fetchDashboard.mockResolvedValue(fixture())
  fetchReactions.mockResolvedValue([
    reactionSpec('steam_burst', '蒸汽爆发'),
    reactionSpec('overheat', '过热'),
  ])
})

describe('DashboardView 图表等价文本（第88轮）', () => {
  it('三张图容器都有 role=img 与含数据的 aria-label', async () => {
    const wrapper = await mountDashboard()
    const charts = wrapper.findAll('.chart-canvas')
    expect(charts.length).toBe(3)
    for (const c of charts) {
      expect(c.attributes('role')).toBe('img')
      expect((c.attributes('aria-label') || '').length).toBeGreaterThan(4)
    }
    expect(charts[0].attributes('aria-label')).toContain('2 日')
    expect(charts[1].attributes('aria-label')).toContain('3 种')
    expect(charts[2].attributes('aria-label')).toContain('1 关')
  })

  it('每张图都有 sr-only 等价数据表，含 caption 与行列 scope', async () => {
    const wrapper = await mountDashboard()
    const tables = wrapper.findAll('table.chart-data-sr')
    expect(tables.length).toBe(3)
    for (const t of tables) {
      expect(t.find('caption').exists()).toBe(true)
      expect(t.findAll('th[scope="col"]').length).toBeGreaterThan(0)
      expect(t.findAll('th[scope="row"]').length).toBeGreaterThan(0)
    }
    expect(tables[0].text()).toContain('2026-01-01')
  })
})
