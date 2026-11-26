// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R12 跨系统成长优先级（纯函数）。
 * 口径完全复用首页 index.vue 的 dims + marginalGain：服务端 buildRating 五维值、
 * config.rating_weights 权重份额、gain = 权重份额 × 缺口、按 gain 降序取第一。
 * 目的是让 skills/equipment/gems/bag 四页与首页 aiInsight 指向同一个短板，
 * 消除真浏览器实测发现的「四页各自为政、结论互相矛盾」。
 * 不 import store/game/api，不写任何配置，不编造数值。
 */
export interface GrowthPriority {
  label: string
  value: string
  /** 当前完成百分比（整数，避免长浮点直出） */
  pct: number
  weightPct: number
  /** 权重份额 × 缺口，仅作跨维可比排序的补分收益 */
  gain: number
  hint: string
  hintLabel: string
  basis: string
}

interface Dimension {
  label: string
  key: string
  weightKey: string
  cap: number
}

const DIMENSIONS: Dimension[] = [
  { label: '元素覆盖', key: 'element_coverage', weightKey: 'element_coverage', cap: 5 },
  { label: '反应链', key: 'reaction_coverage', weightKey: 'reaction_coverage', cap: 7 },
  { label: '专精投入', key: 'mastery_done', weightKey: 'mastery_done', cap: 8 },
  { label: '装备契合', key: 'equipment_synergy', weightKey: 'equipment_synergy', cap: 18 },
  { label: '机制深度', key: 'mechanic_depth', weightKey: 'mechanic_depth', cap: 8 },
]

// 与首页 DIM_HINT 逐字一致：维度 label 是服务端中文，去哪补是纯客户端导航。
const DIM_HINT: Record<string, { path: string; label: string }> = {
  元素覆盖: { path: '/pages/bag/bag', label: '去背包配元素' },
  反应链: { path: '/pages/reactions/reactions', label: '看反应搭配' },
  专精投入: { path: '/pages/mastery/mastery', label: '去点亮专精' },
  装备契合: { path: '/pages/equipment/equipment', label: '去配装备' },
  机制深度: { path: '/pages/bag/bag', label: '去装机制卡' },
}

/**
 * @param rating 服务端 me.build_rating（五维数值字段）
 * @param weights 服务端 config.rating_weights（五维权重字段）
 * @returns 增益最大的可导航短板；无数据 / 权重全 0 / 全部补满时返回 null（诚实空态）
 */
export function computeGrowthPriority(rating: unknown, weights: unknown): GrowthPriority | null {
  const r: Record<string, unknown> | null = rating && typeof rating === 'object' ? rating as Record<string, unknown> : null
  if (!r) return null
  const w: Record<string, unknown> | null = weights && typeof weights === 'object' ? weights as Record<string, unknown> : null
  const wOf = (k: string): number => Number(w?.[k] ?? 0)
  const weightSum = DIMENSIONS.reduce((s, d) => s + wOf(d.weightKey), 0)
  if (!(weightSum > 0)) return null

  const rows = DIMENSIONS.map((d) => {
    const cur = Number(r[d.key] ?? 0) || 0
    const gap = Math.max(0, d.cap - cur)
    const weightPct = Math.round((wOf(d.weightKey) / weightSum) * 100)
    return {
      label: d.label,
      value: `${cur}/${d.cap}`,
      pct: Math.round(Math.min(100, (cur / d.cap) * 100)),
      weightPct,
      gain: weightPct * gap,
      hint: DIM_HINT[d.label]?.path as string | undefined,
      hintLabel: DIM_HINT[d.label]?.label as string | undefined,
    }
  }).filter((row) => row.gain > 0 && row.hint && row.hintLabel)
  rows.sort((a, b) => b.gain - a.gain)
  const top = rows[0]
  if (!top) return null
  return {
    ...top,
    hint: top.hint!,
    hintLabel: top.hintLabel!,
    basis: '依据：服务端五维评分（build_rating）/ 权重份额 × 缺口 · 当前存档 · 更新于本次进入',
  }
}

/**
 * R48 消费层：把 computeGrowthPriority 的结论转成页面可安全消费的 CTA 视图。
 * 短板导航目标就是当前页（自指）时，避免 uni.navigateTo 自己造成死循环：
 * 文案改为「在本页强化 + 维度」，由调用页在 self=true 时执行页内动作；
 * 非自指时逐字保留 hintLabel，导航行为完全不变。纯函数，不 import store/game/api。
 */
export interface R12CtaView {
  self: boolean
  label: string
}

export function resolveR12Cta(priority: GrowthPriority | null, selfRoute: string): R12CtaView {
  if (!priority) return { self: false, label: '' }
  const self = priority.hint === selfRoute
  return { self, label: self ? '在本页强化' + priority.label : priority.hintLabel }
}
