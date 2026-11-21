// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R3 §1B 六组件模式契约（t28）。
 *
 * 审计依据 docs/laiyipao/design/AI_VISUAL_SYSTEM.md §1B：
 * 四个次级视图（关卡/技能/用户/重算）此前是「标题 + 说明 + 拥挤 toolbar
 * + 等权密集表」的传统 Element Plus 后台，无分区眉题、无两级指标带、
 * 无智能空态、无可见 AI 元素。本契约先钉住结构红线，再让实现转绿。
 *
 * 断言强度：DOM 级 mount + 源码级反模式双管；不弱化既有视图测试。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'
import type { AdminLevelView } from '@/api'

const srcOf = (name: string) =>
  readFileSync(join(dirname(fileURLToPath(import.meta.url)), '..', 'src', 'views', name), 'utf8')

const VIEWS = ['LevelsView.vue', 'SkillsView.vue', 'UsersView.vue', 'RecomputesView.vue']

describe('R3 §1B 四视图结构改造（源码级）', () => {
  for (const view of VIEWS) {
    const SRC = srcOf(view)
    it(view + '：有 section-kicker 眉题（中文眉题 + 英文小标）', () => {
      expect(SRC).toContain('section-kicker')
      expect(SRC).toContain('section-kicker-en')
    })
    it(view + '：有两级指标带 kpi-strip（主 KPI + 次 KPI）', () => {
      expect(SRC).toContain('kpi-strip')
      expect(SRC).toContain('kpi-main')
      expect(SRC).toContain('kpi-sub-card')
    })
    it(view + '：有 filter-bar 与 result-count aria-live', () => {
      expect(SRC).toContain('filter-bar')
      expect(SRC).toContain('result-count')
      expect(SRC).toContain('aria-live')
    })
    it(view + '：有 data-grid 表容器与 tabular-nums 数字', () => {
      expect(SRC).toContain('data-grid')
      expect(SRC).toContain('tabular-nums')
    })
    it(view + '：有 empty-state 智能空态（非裸 el-empty）', () => {
      expect(SRC).toContain('empty-state')
      expect(SRC).toContain('empty-state-action')
    })
  }

  it('filter-bar 活动态不出现 color:var(--accent) 叠 accent-soft（对比度红线）', () => {
    for (const view of VIEWS) {
      const SRC = srcOf(view)
      expect(SRC.includes('color: var(--accent)'), view + ' 不得用裸 accent 文字').toBe(false)
      expect(SRC.includes('color:var(--accent)'), view + ' 不得用裸 accent 文字').toBe(false)
    }
  })
})

// ---- DOM 级：LevelsView 的可见 AI 元素与智能空态 ----
const fetchLevels = vi.hoisted(() => vi.fn())
const updateLevel = vi.hoisted(() => vi.fn())
const regenerateLevels = vi.hoisted(() => vi.fn())
const fetchLevelWaves = vi.hoisted(() => vi.fn())
const box = vi.hoisted(() => ({ prompt: vi.fn(), confirm: vi.fn() }))
vi.mock('@/api', () => ({ fetchLevels, updateLevel, regenerateLevels, fetchLevelWaves }))
vi.mock('element-plus', async (importOriginal) => {
  const mod = await importOriginal<typeof import('element-plus')>()
  return { ...mod, ElMessageBox: box }
})

import LevelsView from '@/views/LevelsView.vue'

function levelFixture(overrides: Partial<AdminLevelView> = {}): AdminLevelView {
  return {
    id: 1, chapter: 1, name: '第1关', seed: 's1', base_hp: 1000, wave_count: 5,
    difficulty: 1200, energy_cost: 6, is_boss: false, enabled: true,
    terrain_config: [{ kind: 'oil_drum', x: 600, y: 600, param: 50 }],
    star_targets: [60, 120, 180], attempts: 12, clears: 8,
    player_authority: 'generator', applied_to_players: false,
    clear_rate: 55, avg_wave: 3.2, ...overrides,
  }
}

async function mountLevels(items: AdminLevelView[] = []) {
  fetchLevels.mockResolvedValue({ items, total: items.length })
  const wrapper = mount(LevelsView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  return wrapper
}

describe('R3 §1B.4 LevelsView 可见 AI 元素（DOM 级）', () => {
  beforeEach(() => { vi.clearAllMocks() })

  it('有 .ai-insight 卡：可见汉字 AI + 基于当前数据的结论 + 依据 + 可点 CTA', async () => {
    const worst = levelFixture({ id: 7, name: '第7关', clear_rate: 3.2 })
    const good = levelFixture({ id: 8, name: '第8关', clear_rate: 99 })
    const w = await mountLevels([worst, good])
    const ai = w.find('.ai-insight')
    expect(ai.exists(), '应有可见 AI 洞察卡').toBe(true)
    expect(ai.text()).toContain('AI')
    expect(ai.text()).toContain('依据')
    expect(ai.text()).toMatch(new RegExp('第7关'))
    expect(ai.find('button').exists(), 'AI 卡应有可点 CTA').toBe(true)
  })

  it('无数据时 AI 卡走智能空态，不编造结论', async () => {
    const w = await mountLevels([])
    const ai = w.find('.ai-insight')
    expect(ai.exists()).toBe(true)
    expect(ai.text()).toContain('AI')
    expect(ai.text()).toMatch(/数据不足|暂无/)
  })

  it('section-kicker 眉题含中文与英文小标', async () => {
    const w = await mountLevels([levelFixture()])
    const k = w.find('.section-kicker')
    expect(k.exists()).toBe(true)
    expect(k.text()).toMatch(/关卡/)
    expect(k.text()).toMatch(/[A-Za-z]/)
  })

  it('智能空态含 action 按钮与 role=status', async () => {
    const w = await mountLevels([])
    const e = w.find('.empty-state')
    expect(e.exists(), '空表应渲染 empty-state').toBe(true)
    expect(e.attributes('role')).toBe('status')
    expect(e.find('button').exists(), '空态应有动作按钮').toBe(true)
  })
})
