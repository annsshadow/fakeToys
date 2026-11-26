// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * F-10-2 / §1A.3：AI 运营洞察结论生成规则（确定性、可测、诚实）。
 * 结论只从看板已有数据按固定优先级推导，不编造置信度、不假装对话式 AI。
 */
import { describe, expect, it } from 'vitest'
import type { Dashboard } from '@/api'
import { buildAiInsight } from '@/utils/aiInsight'

function fixture(overrides: Partial<Dashboard> = {}): Dashboard {
  return {
    users: { total: 682, guests: 0, wechat: 0, banned: 0, new_today: 4 },
    battles: { total: 144, today: 0, wins: 0, avg_duration_ms: 9000 },
    defenses: { total: 20, challenges_today: 0 },
    progression: { avg_max_stage: 0, avg_power: 0, stage_100_clears: 0 },
    economy: { coin_in: 0, coin_out: 0, gem_in: 0, gem_out: 0, orders_paid: 0, orders_pending: 0 },
    verification: { checked: 0, matched: 0, mismatched: 0 },
    reaction_usage: [
      { reaction: 'steam_burst', count: 30 },
      { reaction: 'overheat', count: 10 },
    ],
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
const labelOf = (k: string) => ({ steam_burst: '蒸汽爆发', overheat: '过热' }[k] ?? k)

describe('buildAiInsight 确定性洞察规则', () => {
  it('有漏斗数据时选流失最大的关，结论含关卡号/尝试/通关并建议复核难度', () => {
    const r = buildAiInsight(fixture(), labelOf)
    expect(r.kind).toBe('funnel')
    expect(r.headline).toContain('第 2 关')
    expect(r.headline).toContain('400')
    expect(r.headline).toContain('120')
    expect(r.headline).toMatch(/难度|流失/)
    expect(r.ctaPath).toBe('/admin/levels')
    expect(r.evidence).toContain('关卡漏斗')
  })

  it('漏斗缺失时选最高频反应，结论含反应中文名与次数', () => {
    const r = buildAiInsight(fixture({ stage_funnel: [] }), labelOf)
    expect(r.kind).toBe('reaction')
    expect(r.headline).toContain('蒸汽爆发')
    expect(r.headline).toContain('30')
    expect(r.ctaPath).toBe('/admin/skills')
    expect(r.evidence).toContain('元素反应使用分布')
  })

  it('漏斗与反应都缺失时用活跃首尾变化', () => {
    const r = buildAiInsight(fixture({ stage_funnel: [], reaction_usage: [] }), labelOf)
    expect(r.kind).toBe('dau')
    expect(r.headline).toContain('100')
    expect(r.headline).toContain('150')
    expect(r.ctaPath).toBe('/admin/battles')
    expect(r.evidence).toContain('近 14 日活跃')
  })

  it('活跃记录少于 3 天显示诚实空态，标注当前天数', () => {
    const r = buildAiInsight(fixture({ daily_active: [{ date: '2026-01-01', dau: 100 }] }), labelOf)
    expect(r.kind).toBe('empty')
    expect(r.headline).toContain('数据不足')
    expect(r.headline).toContain('1 天')
    expect(r.evidence).toBe('')
  })

  it('数据为 null 时空态', () => {
    expect(buildAiInsight(null, labelOf).kind).toBe('empty')
  })

  it('数据截至日期取最近一天，且结论不得编造置信度/预测措辞', () => {
    const r = buildAiInsight(fixture(), labelOf)
    expect(r.asOf).toBe('2026-01-03')
    for (const bad of ['置信度', '预测', 'AI 认为', '模型推断']) {
      expect(r.headline).not.toContain(bad)
      expect(r.evidence).not.toContain(bad)
    }
  })

  it('同输入两次结果完全一致（确定性，无随机/时间依赖）', () => {
    const a = fixture()
    expect(buildAiInsight(a, labelOf)).toEqual(buildAiInsight(a, labelOf))
  })

  // t31：R3 终审 low backlog —— 反应使用记录全为 0 时不得输出「最高频反应（触发 0 次）」误导结论。
  it('t31: 反应使用记录全为 0 时输出诚实文案，不声称「最高频」不写「触发 0 次」', () => {
    const r = buildAiInsight(
      fixture({
        stage_funnel: [],
        reaction_usage: [
          { reaction: 'steam_burst', count: 0 },
          { reaction: 'overheat', count: 0 },
        ],
      }),
      labelOf,
    )
    expect(r.kind).toBe('reaction')
    expect(r.headline).not.toContain('最高频')
    expect(r.headline).not.toContain('0 次')
    expect(r.headline).toContain('无触发记录')
    expect(r.headline).toMatch(/检查埋点|引导玩家/)
    expect(r.ctaPath).toBe('/admin/skills')
  })

  it('t31: 部分反应为 0 但最高频 > 0 时保持原有结论不退化', () => {
    const r = buildAiInsight(
      fixture({
        stage_funnel: [],
        reaction_usage: [
          { reaction: 'steam_burst', count: 12 },
          { reaction: 'overheat', count: 0 },
        ],
      }),
      labelOf,
    )
    expect(r.kind).toBe('reaction')
    expect(r.headline).toContain('蒸汽爆发')
    expect(r.headline).toContain('最高频元素反应')
    expect(r.headline).toContain('12 次')
  })
})
