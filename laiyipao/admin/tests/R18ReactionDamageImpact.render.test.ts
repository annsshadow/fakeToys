// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R18 实施：admin 看板「反应伤害贡献双视图」渲染行为契约（test-first）。
 *
 * 消费 GET /admin/reaction-damage-impact 的 ReactionDamageImpactResp，
 * 断言双视图切换、降序、服务端取名、确定性格式化、逐字诚实文案、
 * AI 四判据与次数图不回退。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import type { Dashboard, ReactionDamageImpactResp } from '@/api'
import { __resetReactionLabelCache } from '@/reactions'

const fetchDashboard = vi.hoisted(() => vi.fn())
const fetchReactions = vi.hoisted(() => vi.fn())
const fetchReactionDamageImpact = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ fetchDashboard, fetchReactions, fetchReactionDamageImpact }))
const echartsInit = vi.hoisted(() => vi.fn(() => ({ setOption: vi.fn(), dispose: vi.fn() })))
vi.mock('echarts/core', () => ({ use: vi.fn(), init: echartsInit }))

import DashboardView from '@/views/DashboardView.vue'

function fixture(overrides: Partial<Dashboard> = {}): Dashboard {
  return {
    users: { total: 682, guests: 0, wechat: 0, banned: 0, new_today: 4 },
    battles: { total: 144, today: 0, wins: 0, avg_duration_ms: 9000 },
    defenses: { total: 20, challenges_today: 0 },
    progression: { avg_max_stage: 0, avg_power: 0, stage_100_clears: 0 },
    economy: { coin_in: 0, coin_out: 0, gem_in: 0, gem_out: 0, orders_paid: 0, orders_pending: 0, sign_ins_today: 0, redeems_today: 0 },
    verification: { checked: 0, matched: 0, mismatched: 0 },
    reaction_usage: [{ reaction: 'overheat', count: 300 }, { reaction: 'steam_burst', count: 100 }],
    daily_active: [{ date: '2026-01-01', dau: 100 }, { date: '2026-01-02', dau: 120 }],
    stage_funnel: [{ level_id: 1, attempts: 500, clears: 480 }],
    ...overrides,
  }
}
function impactFixture(overrides: Partial<ReactionDamageImpactResp> = {}): ReactionDamageImpactResp {
  return {
    window_days: 30,
    window_from: '2026-10-25',
    window_to: '2026-11-24',
    excluded_old_battles: 3,
    items: [
      { reaction: 'overheat', total_damage: 1200, damage_share_pct: 20, events: 40, count: 300 },
      { reaction: 'steam_burst', total_damage: 4800, damage_share_pct: 80, events: 60, count: 100 },
    ],
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
async function toDamageView(w: any) {
  await w.find('.r18-view-damage').trigger('click')
  await flushPromises()
}

beforeEach(() => {
  vi.clearAllMocks()
  __resetReactionLabelCache()
  fetchDashboard.mockResolvedValue(fixture())
  fetchReactions.mockResolvedValue([reactionSpec('overheat', '过热'), reactionSpec('steam_burst', '蒸汽爆发')])
  fetchReactionDamageImpact.mockResolvedValue(impactFixture())
})

describe('R18 admin 反应伤害贡献双视图（渲染行为）', () => {
  it('容器存在且默认次数视图，标题含可见汉字 AI', async () => {
    const w = await mountDashboard()
    const card = w.find('.r18-damage-impact')
    expect(card.exists()).toBe(true)
    expect(card.attributes('role')).toBe('region')
    expect(card.text()).toContain('AI')
    expect(w.find('.r18-view-count').exists()).toBe(true)
    expect(w.find('.r18-view-damage').exists()).toBe(true)
    expect(w.find('.r18-view-count').attributes('aria-pressed')).toBe('true')
  })

  it('切伤害视图：行按 damage_share_pct 降序，data-r18-reaction 用 key，名称用服务端 labelOf', async () => {
    const w = await mountDashboard()
    await toDamageView(w)
    const rows = w.findAll('.r18-damage-row')
    expect(rows.length).toBe(2)
    expect(rows[0].attributes('data-r18-reaction')).toBe('steam_burst')
    expect(rows[1].attributes('data-r18-reaction')).toBe('overheat')
    expect(rows[0].text()).toContain('蒸汽爆发')
    expect(rows[1].text()).toContain('过热')
    expect(rows.map((r: any) => r.attributes('data-r18-reaction'))).toEqual(['steam_burst', 'overheat'])
  })

  it('数值确定性格式化：整数 toFixed(0)、非整数 toFixed(1)、非有限破折号', async () => {
    fetchReactionDamageImpact.mockResolvedValue(impactFixture({
      items: [
        { reaction: 'overheat', total_damage: 1234.5, damage_share_pct: 33.33, events: 10, count: 5 },
        { reaction: 'steam_burst', total_damage: Number.NaN, damage_share_pct: Number.POSITIVE_INFINITY, events: 2, count: 1 },
      ],
    }))
    const w = await mountDashboard()
    await toDamageView(w)
    const rows = w.findAll('.r18-damage-row')
    const over = rows.find((r: any) => r.attributes('data-r18-reaction') === 'overheat')
    const steam = rows.find((r: any) => r.attributes('data-r18-reaction') === 'steam_burst')
    expect(over?.text()).toContain('33.3%')
    expect(over?.text()).toContain('1234.5')
    expect(steam?.text()).toContain('—')
  })

  it('窗口与排除标注逐字，excluded>0 如实显示', async () => {
    const w = await mountDashboard()
    await toDamageView(w)
    expect(w.find('.r18-window-note').text()).toBe('统计窗口：近 30 天（2026-10-25 至 2026-11-24）')
    expect(w.find('.r18-excluded-note').text()).toBe('已排除 3 条无逐次记录的旧战报（不计入伤害）')
  })

  it('excluded_old_battles 为 0 也如实显示 0，不静默隐藏', async () => {
    fetchReactionDamageImpact.mockResolvedValue(impactFixture({ excluded_old_battles: 0 }))
    const w = await mountDashboard()
    await toDamageView(w)
    expect(w.find('.r18-excluded-note').text()).toContain('已排除 0 条')
  })

  it('空态 items=[] 逐字且不编造 NaN/undefined，窗口/排除仍显示', async () => {
    fetchReactionDamageImpact.mockResolvedValue(impactFixture({ items: [] }))
    const w = await mountDashboard()
    await toDamageView(w)
    expect(w.find('.r18-damage-impact').text()).toContain('近 30 天无逐次反应伤害记录')
    expect(w.findAll('.r18-damage-row').length).toBe(0)
    expect(w.find('.r18-window-note').exists()).toBe(true)
    expect(w.find('.r18-excluded-note').exists()).toBe(true)
    expect(w.find('.r18-damage-impact').text()).not.toContain('undefined')
    expect(w.find('.r18-damage-impact').text()).not.toContain('NaN')
  })

  it('AI 四判据：可见 AI + 高频低伤真实结论 + 依据行逐字 + 可执行 CTA', async () => {
    const w = await mountDashboard()
    await toDamageView(w)
    const note = w.find('.r18-ai-note')
    expect(note.exists()).toBe(true)
    expect(note.attributes('role')).toBe('note')
    expect(note.text()).toContain('AI')
    expect(note.text()).toContain('高频低伤')
    expect(note.text()).toContain('20.0%')
    expect(note.text()).toContain('依据：近 30 天 reaction_timeline.events 的 damage 聚合；触发次数以 dashboard_reaction_counts 为准')
    const cta = note.find('.r18-ai-cta')
    expect(cta.exists()).toBe(true)
    expect(cta.attributes('href')).toContain('/admin/battles')
  })

  it('AI 识别低频高伤（低频但伤害贡献高）', async () => {
    fetchReactionDamageImpact.mockResolvedValue(impactFixture({
      items: [
        { reaction: 'overheat', total_damage: 5700, damage_share_pct: 95, events: 5, count: 3 },
        { reaction: 'steam_burst', total_damage: 300, damage_share_pct: 5, events: 95, count: 397 },
      ],
    }))
    const w = await mountDashboard()
    await toDamageView(w)
    expect(w.find('.r18-ai-note').text()).toContain('低频高伤')
  })

  it('诚实说明逐字，辅助非权威不参与结算与 replay_hash', async () => {
    const w = await mountDashboard()
    await toDamageView(w)
    expect(w.find('.r18-honesty-note').text()).toBe(
      '伤害贡献为辅助分析；触发次数以物化视图为准，不参与结算与 replay_hash',
    )
  })

  it('次数图与权威空态保留不回退（reactionRef/reactionRows/尚无反应触发记录）', async () => {
    fetchDashboard.mockResolvedValue(fixture({ reaction_usage: [] }))
    const w = await mountDashboard()
    expect(w.find('[ref="reactionRef"]').exists() || w.find('.chart-canvas').exists()).toBe(true)
    expect(w.text()).toContain('尚无反应触发记录')
    expect(w.find('.r18-damage-impact').exists()).toBe(true)
  })
})
