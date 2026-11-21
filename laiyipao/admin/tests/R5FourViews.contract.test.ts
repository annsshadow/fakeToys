// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R5 admin 四视图结构性升级契约（t48）。
 *
 * §1D 逐页映射：Skills / Users / Recomputes / Battles 四页都必须有
 * 眉题 + 双级 KPI + FilterBar + DataGrid + EmptyState + 真实 AI 洞察卡，
 * 且 AI 卡四判据齐全（可见汉字 AI + 真实数据结论 + 依据 + CTA）。
 * 先钉住源码/结构红线，再让实现转绿。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { createMemoryHistory, createRouter } from 'vue-router'
import { describe, expect, it, vi, beforeEach } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'

const srcOf = (name: string) =>
  readFileSync(join(dirname(fileURLToPath(import.meta.url)), '..', 'src', 'views', name), 'utf8')

const VIEWS = ['SkillsView.vue', 'UsersView.vue', 'RecomputesView.vue', 'BattlesView.vue']

describe('R5 §1D 四视图结构升级（源码级）', () => {
  for (const view of VIEWS) {
    const SRC = srcOf(view)
    it(view + '：眉题 + 双级 KPI + FilterBar + DataGrid + EmptyState 齐全', () => {
      expect(SRC).toContain('section-kicker')
      expect(SRC).toContain('section-kicker-en')
      expect(SRC).toContain('kpi-strip')
      expect(SRC).toContain('kpi-main')
      expect(SRC).toContain('kpi-sub-card')
      expect(SRC).toContain('filter-bar')
      expect(SRC).toContain('result-count')
      expect(SRC).toContain('data-grid')
      expect(SRC).toContain('empty-state')
      expect(SRC).toContain('empty-state-action')
    })
    it(view + '：接入真实 AI 洞察卡（共享组件四判据 + 当前页数据结论）', () => {
      expect(SRC).toContain('<AiInsightCard')
      expect(SRC).toContain('buildAdminAiInsight')
      // 结论来自当前页数据 ref，非写死文案
      const DATA_VAR: Record<string, string> = {
        'SkillsView.vue': 'skills.value', 'UsersView.vue': 'users.value',
        'RecomputesView.vue': 'summary.value', 'BattlesView.vue': 'battles.value',
      }
      expect(SRC).toContain(DATA_VAR[view])
    })
    it(view + '：AI 卡不在数据区之后隐藏（位于 kpi-strip 之前）', () => {
      const ai = SRC.indexOf('<AiInsightCard')
      const kpi = SRC.indexOf('<div class="kpi-strip"')
      expect(ai).toBeGreaterThan(-1)
      expect(kpi).toBeGreaterThan(-1)
      expect(ai).toBeLessThan(kpi)
    })
  }
  it('共享 AiInsightCard 四判据齐全（汉字 AI + 结论 + 依据 + CTA）', () => {
    const CARD = readFileSync(join(dirname(fileURLToPath(import.meta.url)), '..', 'src', 'components', 'AiInsightCard.vue'), 'utf8')
    expect(CARD).toContain('ai-insight')
    expect(CARD).toContain('ai-badge')
    expect(CARD).toContain('ai-headline')
    expect(CARD).toContain('ai-evidence')
    expect(CARD).toContain('ai-cta')
    expect(CARD).toContain('规则引擎')
  })
  it('filter-bar 无 color:var(--accent) 叠 accent-soft 对比度红线', () => {
    for (const view of VIEWS) {
      const SRC = srcOf(view)
      expect(SRC.includes('color: var(--accent)')).toBe(false)
      expect(SRC.includes('color:var(--accent)')).toBe(false)
    }
  })
})

// ===== DOM 级：四视图 AI 洞察卡可见、有真实结论、依据与 CTA =====
const fetchSkills = vi.hoisted(() => vi.fn())
const fetchUsers = vi.hoisted(() => vi.fn())
const fetchRecomputes = vi.hoisted(() => vi.fn())
const fetchRecomputeStats = vi.hoisted(() => vi.fn())
const fetchRecomputeMode = vi.hoisted(() => vi.fn())
const fetchBattles = vi.hoisted(() => vi.fn())
const fetchBattleDetail = vi.hoisted(() => vi.fn())
const fetchReactions = vi.hoisted(() => vi.fn())
const verifyBattle = vi.hoisted(() => vi.fn())
const box = vi.hoisted(() => ({ prompt: vi.fn(), confirm: vi.fn() }))

vi.mock('@/api', () => ({
  fetchSkills, fetchUsers, fetchRecomputes, fetchRecomputeStats, fetchRecomputeMode,
  fetchBattles, fetchBattleDetail, fetchReactions, verifyBattle,
  exportRecomputesCsv: vi.fn(), setRecomputeMode: vi.fn(), exportUsersCsv: vi.fn(),
  exportUserFlowsCsv: vi.fn(), updateSkill: vi.fn(), banUser: vi.fn(), unbanUser: vi.fn(), grantUser: vi.fn(),
}))
vi.mock('element-plus', async (importOriginal) => {
  const mod = await importOriginal<typeof import('element-plus')>()
  return { ...mod, ElMessageBox: box }
})

import SkillsView from '@/views/SkillsView.vue'
import UsersView from '@/views/UsersView.vue'
import RecomputesView from '@/views/RecomputesView.vue'
import BattlesView from '@/views/BattlesView.vue'

async function router() {
  const r = createRouter({ history: createMemoryHistory(), routes: [{ path: '/:p(.*)*', component: { template: '<div/>' } }] })
  await r.push('/')
  await r.isReady()
  return r
}
function aiAssertions(wrapper: any, where: string) {
  const card = wrapper.find('.ai-insight')
  expect(card.exists(), where + ' 应有 AI 洞察卡').toBe(true)
  expect(card.find('.ai-badge').text()).toContain('AI')
  expect(card.find('.ai-headline').text().length).toBeGreaterThan(6)
  expect(card.find('.ai-evidence').text().length).toBeGreaterThan(2)
  expect(card.find('.ai-cta').exists(), where + ' 应可点击 CTA').toBe(true)
}

beforeEach(() => vi.clearAllMocks())

describe('R5 四视图 AI 卡（DOM 级）', () => {
  it('Skills：热量效率最低的技能被识别为失衡', async () => {
    fetchSkills.mockResolvedValue({
      items: [{ id: 1, code: 'a', name: '弱焰弹', family: 'flame', element: 'fire', kind: 'active', base_damage: 10, heat_cost: 50, cooldown_ms: 2000, pierce: 0, aoe_radius: 0, apply_element: 'fire', apply_stacks: 0, projectile_speed: 1000, chain: 0, unlock_level: 1, descr: '' }],
      recipes: [],
    })
    const w = mount(SkillsView, { global: { plugins: [ElementPlus, await router()] } })
    await flushPromises()
    aiAssertions(w, '技能页')
    expect(w.find('.ai-insight .ai-headline').text()).toContain('弱焰弹')
  })
  it('Users：高战力低关卡的异常账号被识别', async () => {
    fetchUsers.mockResolvedValue({
      items: [{ id: 1, nickname: '异常号', is_guest: false, status: 1, max_stage: 1, power: 9000, coin: 0, gem: 0, last_login_at: '2026-01-02T03:04:05Z', created_at: '2025-12-01T00:00:00Z', verify_checked: 1, verify_mismatched: 0 }],
      total: 1,
    })
    const w = mount(UsersView, { global: { plugins: [ElementPlus, await router()] } })
    await flushPromises()
    aiAssertions(w, '用户页')
    expect(w.find('.ai-insight .ai-headline').text()).toContain('异常号')
  })
  it('Recomputes：不符率最高的关卡被识别', async () => {
    fetchRecomputeStats.mockResolvedValue({
      mode: 'enforce', engine_mode: 'enforce', skills_mode: 'enforce', enforcing: true, configured_mode: 'enforce', demoted: false, demoted_reason: '',
      since: '2026-01-01T00:00:00Z', breaker_rate_permille: 20, breaker_min_samples: 200,
      totals: { total: 100, mismatched: 10, rejected: 10, rate_permille: 100 },
      levels: [{ level_id: 7, total: 50, mismatched: 10, rate_permille: 200, max_user_mismatches: 3 }],
    })
    fetchRecomputeMode.mockResolvedValue({ mode: 'enforce', engine_mode: 'enforce', skills_mode: 'enforce', enforcing: true, configured_mode: 'enforce', demoted: false, demoted_reason: '' })
    fetchRecomputes.mockResolvedValue({ items: [], total: 0 })
    const w = mount(RecomputesView, { global: { plugins: [ElementPlus, await router()] } })
    await flushPromises()
    aiAssertions(w, '重算页')
    expect(w.find('.ai-insight .ai-headline').text()).toContain('7')
  })
  it('Battles：验真异常/守恒可疑的战报被识别', async () => {
    fetchBattles.mockResolvedValue({
      items: [{ id: 1, user_id: 9, nickname: '可疑号', level_id: 3, result: 'win', stars: 3, score: 88000, kills: 50, leaked: 20, wave_reached: 5, duration_ms: 12345, reactions: 5, heat_max: 90, replay_hash: 'abcd', created_at: '2026-01-02T03:04:05Z', total_enemies: 12, verify_checked: 3, verify_mismatched: 2 }],
      total: 1,
    })
    fetchReactions.mockResolvedValue({})
    const w = mount(BattlesView, { global: { plugins: [ElementPlus, await router()] } })
    await flushPromises()
    aiAssertions(w, '战报页')
    expect(w.find('.ai-insight .ai-headline').text()).toContain('异常')
  })
})
