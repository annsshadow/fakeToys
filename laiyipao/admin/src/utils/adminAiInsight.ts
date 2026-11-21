// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R5 admin 四视图 AI 洞察结论（AI_VISUAL_SYSTEM.md §1D.4）。
 *
 * 诚实标注「规则引擎」：每页结论从当前页数据按固定规则推导，
 * 不编造置信度、不假装对话式模型、不引入随机/当前时间依赖。
 */
import type { AiInsight } from '@/utils/aiInsight'

type Input = Record<string, unknown>
function base(headline: string, evidence: string, ctaLabel: string, ctaPath: string): AiInsight {
  return { kind: 'funnel', headline, evidence, ctaLabel, ctaPath, asOf: '' }
}

/** 技能页：热量效率最低（DPS/热量）的技能最可能是数值失衡点。 */
/**
 * 四页统一入口：视图层只传当前页数据，规则在确定性函数内。
 */
export function buildAdminAiInsight(
  view: 'skills' | 'users' | 'recomputes' | 'battles',
  data: { skills?: Array<Input>; recipes?: Array<Input>; users?: Array<Input>; summary?: Input | null; battles?: Array<Input> },
): AiInsight {
  switch (view) {
    case 'skills': return buildSkillsInsight(data.skills ?? [], data.recipes ?? [])
    case 'users': return buildUsersInsight(data.users ?? [])
    case 'recomputes': return buildRecomputesInsight(data.summary ?? null)
    case 'battles': return buildBattlesInsight(data.battles ?? [])
  }
}

export function buildSkillsInsight(skills: Array<Input>, recipes: Array<Input>): AiInsight {
  if (!skills.length) return base('数据不足：当前没有技能，无法评估数值平衡', '需要至少一条技能记录', '查看配方', '/admin/skills')
  const scored = skills.map(s => ({
    s,
    eff: Number(s.heat_cost) > 0 ? Number(s.base_damage) * (1000 / Math.max(1, Number(s.cooldown_ms))) / Number(s.heat_cost) : Number(s.base_damage),
  })).sort((a, b) => a.eff - b.eff)
  const worst = scored[0]
  return base(
    `「${worst.s.name}」热量效率最低（每点热量 ${worst.eff.toFixed(1)} DPS），建议优先复核数值平衡`,
    `共 ${skills.length} 个技能 / ${recipes.length} 条配方，热量效率越低越难在 100 点预算内打出伤害`,
    '复核技能数值',
    '/admin/skills',
  )
}

/** 用户页：高战力但停留在第 1 关的账号是刷分嫌疑，优先人工核查。 */
export function buildUsersInsight(users: Array<Input>): AiInsight {
  if (!users.length) return base('数据不足：当前没有用户记录，无法识别异常账号', '需要至少一条用户记录', '查看用户', '/admin/users')
  const suspects = users.filter(u => Number(u.power) > 5000 && Number(u.max_stage) <= 1)
  if (!suspects.length) return base('当前页未发现高战力低关卡的异常账号，账号成长曲线正常', `本页共 ${users.length} 名用户`, '查看用户', '/admin/users')
  const top = suspects.sort((a, b) => Number(b.power) - Number(a.power))[0]
  return base(
    `玩家「${top.nickname}」战力 ${top.power} 却停留第 ${top.max_stage} 关，建议核查资源来源`,
    `本页 ${users.length} 名用户中 ${suspects.length} 名命中高战力低关卡信号（战力>5000 且关卡≤1）`,
    '核查该用户',
    '/admin/users',
  )
}

/** 重算页：不符率最高的关卡优先复核，区分引擎漂移与作弊需看同关多号分布。 */
export function buildRecomputesInsight(summary: Input | null): AiInsight {
  if (!summary) return base('数据不足：总账尚未加载，无法定位不符关卡', '需要统计窗口总账', '查看审计', '/admin/recomputes')
  const levels = (summary.levels as Array<Input>) ?? []
  if (!levels.length) return base('当前窗口无关卡级不符统计，暂无需复核的关卡', '总账已加载但关卡分布为空', '查看审计', '/admin/recomputes')
  const worst = [...levels].sort((a, b) => Number(b.rate_permille) - Number(a.rate_permille))[0]
  const rate = (Number(worst.rate_permille) / 10).toFixed(1)
  const hint = Number(worst.max_user_mismatches) >= Number(worst.mismatched) ? '不符集中在少数账号，倾向核查作弊' : '不符分散在多账号，倾向核查引擎漂移'
  return base(
    `第 ${worst.level_id} 关不符率最高（${rate}%），建议优先复核`,
    `该关 ${worst.mismatched}/${worst.total} 条不符；${hint}，最终以逐条证据为准`,
    '查看不符台账',
    '/admin/recomputes',
  )
}

/** 战报页：验真不匹配与守恒可疑是本页最值得先看的两类异常。 */
export function buildBattlesInsight(battles: Array<Input>): AiInsight {
  if (!battles.length) return base('数据不足：当前页没有战报，无法识别异常对局', '需要至少一条战报记录', '查看战报', '/admin/battles')
  const mismatched = battles.filter(b => Number(b.verify_mismatched) > 0).length
  const suspicious = battles.filter(b => {
    const tot = Number(b.total_enemies ?? 0)
    return tot > 0 && Number(b.kills) + Number(b.leaked) > tot
  }).length
  if (!mismatched && !suspicious) return base('当前页未发现验真异常或守恒可疑的战报，本页数据看起来正常', `本页共 ${battles.length} 场战报`, '查看战报', '/admin/battles')
  return base(
    `本页发现 ${mismatched} 场验真异常、${suspicious} 场守恒可疑，建议优先复核`,
    `验真异常＝重放哈希不符；守恒可疑＝击杀+漏怪超过该关总怪数，两者都需人工确认`,
    '筛选异常战报',
    '/admin/battles',
  )
}
