// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * AI 运营洞察的确定性结论生成（AI_VISUAL_SYSTEM.md §1A.3）。
 *
 * 诚实地标注为「规则引擎」：结论只从看板数据按固定优先级推导，
 * 不编造置信度、不假装对话式模型。同输入恒等输出，无随机/当前时间依赖。
 */
import type { Dashboard } from '@/api'
import { describeDashboardChart, type ChartRow } from '@/utils/chartA11y'

export type AiInsightKind = 'funnel' | 'reaction' | 'dau' | 'empty'

export interface AiInsight {
  kind: AiInsightKind
  headline: string
  evidence: string
  ctaLabel: string
  ctaPath: string
  /** 数据截至日期（最近一天），空数据时为空串 */
  asOf: string
}

const MIN_DAU_DAYS = 3

/**
 * 按固定优先级产出一条最有行动价值的洞察：
 * 1. 漏斗流失最大的关 → 复核难度（/admin/levels）
 * 2. 最高频元素反应 → 复核配方平衡（/admin/skills）
 * 3. 活跃首尾变化 → 看战斗与召回（/admin/battles）
 * 4. 活跃不足 3 天 → 诚实空态
 */
export function buildAiInsight(
  data: Dashboard | null,
  labelOf: (key: string) => string,
): AiInsight {
  const days = data?.daily_active ?? []
  const asOf = days.length ? days[days.length - 1].date : ''
  const empty = (headline: string): AiInsight => ({
    kind: 'empty',
    headline,
    evidence: '',
    ctaLabel: '',
    ctaPath: '',
    asOf,
  })

  if (!data || days.length < MIN_DAU_DAYS) {
    return empty(`数据不足：AI 洞察需要至少 ${MIN_DAU_DAYS} 天活跃记录，当前 ${days.length} 天`)
  }

  // 规则 1：找「尝试 - 通关」绝对流失最大的关。
  const funnel = data.stage_funnel ?? []
  if (funnel.length) {
    const worst = funnel.reduce((a, b) => (b.attempts - b.clears > a.attempts - a.clears ? b : a))
    const lost = worst.attempts - worst.clears
    const rows: ChartRow[] = funnel.map((x) => ({ label: `第${x.level_id}关`, value: x.attempts }))
    return {
      kind: 'funnel',
      headline: `第 ${worst.level_id} 关流失最大（尝试 ${worst.attempts} → 通关 ${worst.clears}，净流失 ${lost}），建议优先复核关卡难度`,
      evidence: describeDashboardChart({ title: '关卡漏斗', chartType: 'bar', unit: '关', metric: '尝试', rows }).label,
      ctaLabel: '复核关卡难度',
      ctaPath: '/admin/levels',
      asOf,
    }
  }

  // 规则 2：最高频元素反应。
  const usage = data.reaction_usage ?? []
  if (usage.length) {
    const top = usage.reduce((a, b) => (b.count > a.count ? b : a))
    const rows: ChartRow[] = [...usage]
      .sort((a, b) => a.count - b.count)
      .map((x) => ({ label: labelOf(x.reaction), value: x.count }))
    return {
      kind: 'reaction',
      headline: `「${labelOf(top.reaction)}」是最高频元素反应（触发 ${top.count} 次），建议优先复核其配方平衡`,
      evidence: describeDashboardChart({ title: '元素反应使用分布', chartType: 'bar', unit: '种', metric: '触发', rows }).label,
      ctaLabel: '查看反应配方',
      ctaPath: '/admin/skills',
      asOf,
    }
  }

  // 规则 3：活跃首尾变化。
  const first = days[0].dau
  const last = days[days.length - 1].dau
  const delta = last - first
  const rows: ChartRow[] = days.map((x) => ({ label: x.date, value: x.dau }))
  return {
    kind: 'dau',
    headline: `近 ${days.length} 日活跃从 ${first} 变为 ${last}（${delta >= 0 ? '+' : ''}${delta}），建议结合战斗数据复核留存`,
    evidence: describeDashboardChart({ title: '近 14 日活跃', chartType: 'line', unit: '日', metric: '活跃', rows }).label,
    ctaLabel: '查看战斗记录',
    ctaPath: '/admin/battles',
    asOf,
  }
}
