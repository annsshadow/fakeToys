// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * F-10-3 标杆重构契约（看板）：钉住「主次指标带 + 主次图表」层级结构。
 * 当看板退化回 8 张等权卡平铺、或图表重新等权时，这些断言会失败——不是自述。
 */
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'
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

const SRC = readFileSync(join(dirname(fileURLToPath(import.meta.url)), '..', 'src', 'views', 'DashboardView.vue'), 'utf8')

function fixture(): Dashboard {
  return {
    users: { total: 1200, guests: 300, wechat: 900, banned: 5, new_today: 12 },
    battles: { total: 100, today: 3, wins: 75, avg_duration_ms: 9000 },
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
  return { key, name, base_coef: 60, attack_weight_pct: 300, status_duration_ms: 0,
    aoe_radius: 0, dispel_shield: false, amplify_pct: 0, descr: '' }
}

beforeEach(() => {
  vi.clearAllMocks()
  __resetReactionLabelCache()
  echartsInit.mockReturnValue(chartStub)
  fetchDashboard.mockResolvedValue(fixture())
  fetchReactions.mockResolvedValue([reactionSpec('steam_burst', '蒸汽爆发')])
})

describe('F-10-3 看板标杆重构契约', () => {
  it('KPI 区分主次两层：核心指标带 + 次级指标带，不再是 8 张等权卡平铺', async () => {
    const w = await mountDashboard()
    expect(w.findAll('.kpi-primary .kpi-value').length, '核心指标带应含 >=4 个 KPI').toBeGreaterThanOrEqual(4)
    expect(w.findAll('.kpi-secondary .kpi-value').length, '次级指标带应含 >=4 个 KPI').toBeGreaterThanOrEqual(4)
    expect(w.findAll('.kpi-value').length, '不得丢 KPI 数据').toBe(8)
    w.unmount()
  })

  it('章节有 eyebrow 分区标签与 CSS grid 层级栅格（非 EP 等权 el-row）', async () => {
    const w = await mountDashboard()
    expect(w.findAll('.section-kicker').length, '应有 >=2 个章节 eyebrow').toBeGreaterThanOrEqual(2)
    expect(SRC).toContain('grid-template-columns')
    expect(SRC).toContain('kpi-primary')
    expect(SRC).toContain('kpi-secondary')
    w.unmount()
  })

  it('图表区主次有别：主图带 chart-primary；三图与等价表、AI 卡不回退', async () => {
    const w = await mountDashboard()
    expect(w.findAll('.chart-primary .chart-canvas').length, '应有主图容器').toBeGreaterThanOrEqual(1)
    expect(w.findAll('.chart-canvas').length).toBe(3)
    expect(w.findAll('table.chart-data-sr').length).toBe(3)
    expect(w.find('#ai-insight-card').exists()).toBe(true)
    w.unmount()
  })
})
