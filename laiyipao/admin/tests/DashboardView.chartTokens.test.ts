// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * F-10-4：看板 ECharts 配色必须消费设计令牌，不得硬编码。
 *
 * 审计依据 docs/laiyipao/reviews/ROUND1_DESIGN_REVIEW.md:
 * DashboardView.vue 的 axis/legend/series 里散落 #8b949e/#30363d 等魔法色，
 * 与 tokens.css 定义并存却在令牌改色时纹丝不动。
 * 本文件先钉住「源码不含魔法色 + 运行时 getComputedStyle 值流入 option」，
 * 再让实现转绿。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'
import type { Dashboard } from '@/api'
import { __resetReactionLabelCache } from '@/reactions'

const dashboardSource = readFileSync(
  join(dirname(fileURLToPath(import.meta.url)), '..', 'src', 'views', 'DashboardView.vue'),
  'utf8',
)

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
    ],
    daily_active: [
      { date: '2026-01-01', dau: 100 },
      { date: '2026-01-02', dau: 140 },
    ],
    stage_funnel: [{ level_id: 1, attempts: 500, clears: 300 }],
  }
}

function reactionSpec(key: string, name: string) {
  return {
    key, name, base_coef: 60, attack_weight_pct: 300, status_duration_ms: 0,
    aoe_radius: 0, dispel_shield: false, amplify_pct: 0, descr: '',
  }
}

/** 用假令牌值替换 getComputedStyle，其余属性原样透传，避免影响 Element Plus。 */
function mockTheme(tokens: Record<string, string>) {
  const original = window.getComputedStyle.bind(window)
  return vi.spyOn(window, 'getComputedStyle').mockImplementation(((el: Element, pseudo?: string | null) => {
    const real = original(el, pseudo ?? undefined)
    return new Proxy(real, {
      get(target, prop) {
        if (prop === 'getPropertyValue') return (name: string) => tokens[name] ?? target.getPropertyValue(name)
        const value = Reflect.get(target, prop, target)
        return typeof value === 'function' ? value.bind(target) : value
      },
    })
  }) as typeof window.getComputedStyle)
}

const THEME: Record<string, string> = {
  '--lyp-dim': '#96a0ad',
  '--lyp-border': '#2b3540',
  '--el-fire': '#ff6b35',
  '--el-fire-soft': 'rgba(255,107,53,0.18)',
  '--el-corrosion': '#7ee787',
  '--el-kinetic': '#c9a7ff',
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

describe('F-10-4 源码契约：ECharts 不硬编码颜色', () => {
  it('DashboardView 不含审计点名的魔法色 #8b949e / #30363d', () => {
    expect(dashboardSource).not.toMatch(/#8b949e/i)
    expect(dashboardSource).not.toMatch(/#30363d/i)
  })

  it('通过 getComputedStyle 读取 --lyp-dim / --lyp-border / 元素色令牌', () => {
    expect(dashboardSource).toMatch(/getComputedStyle/)
    for (const name of ['--lyp-dim', '--lyp-border', '--el-fire', '--el-corrosion', '--el-kinetic']) {
      expect(dashboardSource, '缺少令牌 ' + name).toContain(name)
    }
  })
})

describe('F-10-4 运行时契约：令牌值流入 ECharts option', () => {
  it('axis / legend / series 颜色等于 getComputedStyle 返回的令牌值', async () => {
    const spy = mockTheme(THEME)
    try {
      const wrapper = mount(DashboardView, { global: { plugins: [ElementPlus] } })
      await flushPromises()
      await new Promise((r) => setTimeout(r, 60))
      await flushPromises()
      const options = chartStub.setOption.mock.calls.map((c) => c[0] as any)
      expect(options.length).toBeGreaterThanOrEqual(3)

      const dau = options.find((o) => o.series?.[0]?.type === 'line')
      expect(dau?.xAxis?.axisLine?.lineStyle?.color).toBe(THEME['--lyp-border'])
      expect(dau?.xAxis?.axisLabel?.color).toBe(THEME['--lyp-dim'])
      expect(dau?.series?.[0]?.lineStyle?.color).toBe(THEME['--el-fire'])
      expect(dau?.series?.[0]?.areaStyle?.color).toBe(THEME['--el-fire-soft'])

      const funnel = options.find((o) => o.legend)
      expect(funnel?.legend?.textStyle?.color).toBe(THEME['--lyp-dim'])
      expect(funnel?.series?.[0]?.itemStyle?.color).toBe(THEME['--lyp-border'])
      expect(funnel?.series?.[1]?.itemStyle?.color).toBe(THEME['--el-corrosion'])

      const reaction = options.find((o) => o.series?.[0]?.type === 'bar' && !o.legend)
      expect(reaction?.series?.[0]?.itemStyle?.color).toBe(THEME['--el-kinetic'])
      expect(reaction?.series?.[0]?.label?.color).toBe(THEME['--lyp-dim'])
      wrapper.unmount()
    } finally {
      spy.mockRestore()
    }
  })
})
