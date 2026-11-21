// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R4 §1C.2 admin 四视图标杆升级契约（t36）。
 * ContentView/EconomyView/DefensesView/EquipmentView 此前是传统 EP 后台：
 * 无分区眉题、无两级指标带、无智能空态、无可见 AI。本契约先钉结构红线。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'
const srcOf = (n: string) => readFileSync(join(dirname(fileURLToPath(import.meta.url)), '..', 'src', 'views', n), 'utf8')
const VIEWS = ['ContentView.vue', 'EconomyView.vue', 'DefensesView.vue', 'EquipmentView.vue']

describe('R4 §1C.2 四视图六组件结构（源码级）', () => {
  for (const view of VIEWS) {
    const SRC = srcOf(view)
    it(view + ' 有 section-kicker 眉题（中文+英文小标）', () => {
      expect(SRC).toContain('section-kicker')
      expect(SRC).toContain('section-kicker-en')
    })
    it(view + ' 有 kpi-strip 两级指标带', () => {
      expect(SRC).toContain('kpi-strip')
      expect(SRC).toContain('kpi-main')
      expect(SRC).toContain('kpi-sub-card')
    })
    it(view + ' 有 filter-bar 与 result-count aria-live', () => {
      expect(SRC).toContain('filter-bar')
      expect(SRC).toContain('result-count')
      expect(SRC).toContain('aria-live')
    })
    it(view + ' 有 data-grid 表容器与 tabular-nums', () => {
      expect(SRC).toContain('data-grid')
      expect(SRC).toContain('tabular-nums')
    })
    it(view + ' 有 empty-state 智能空态（非裸 el-empty）', () => {
      expect(SRC).toContain('empty-state')
      expect(SRC).toContain('empty-state-action')
    })
  }
  it('filter-bar 不用裸 accent 文字（对比度红线）', () => {
    for (const view of VIEWS) {
      const SRC = srcOf(view)
      expect(SRC.includes('color:var(--accent)'), view).toBe(false)
    }
  })
})

// ---- DOM 级：EconomyView 可见 AI 洞察（净流入数据结论）----
const fetchEconomy = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ fetchEconomy }))
import EconomyView from '@/views/EconomyView.vue'

async function mountEconomy(overrides: Record<string, unknown> = {}) {
  fetchEconomy.mockResolvedValue({
    flows: [{ currency: 'coin', source: 'battle', delta: 5000, count: 100 }],
    orders: [], shop: [], ...overrides,
  })
  const w = mount(EconomyView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  return w
}

describe('R4 §1C.2 EconomyView 可见 AI 元素（DOM 级）', () => {
  beforeEach(() => { vi.clearAllMocks() })
  it('有 .ai-insight：可见汉字 AI + 净流入数据结论 + 依据 + CTA', async () => {
    const w = await mountEconomy()
    const ai = w.find('.ai-insight')
    expect(ai.exists(), '应有可见 AI 洞察卡').toBe(true)
    expect(ai.text()).toContain('AI')
    expect(ai.text()).toContain('依据')
    expect(ai.text()).toMatch(/回收|净流入|贬值/)
    expect(ai.find('button, a').exists(), 'AI 卡应有可点 CTA').toBe(true)
  })
  it('无流水数据时 AI 卡走诚实空态，不编造结论', async () => {
    const w = await mountEconomy({ flows: [] })
    const ai = w.find('.ai-insight')
    expect(ai.exists()).toBe(true)
    expect(ai.text()).toMatch(/数据不足|暂无|尚无/)
  })
})