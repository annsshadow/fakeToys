// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, expect, it } from 'vitest'
import { buildSkillsInsight, buildUsersInsight, buildRecomputesInsight, buildBattlesInsight } from '@/utils/adminAiInsight'

describe('R5 四视图 AI 洞察结论（确定性）', () => {
  it('技能页：热量效率最低技能入结论', () => {
    const r = buildSkillsInsight([
      { name: '强技能', base_damage: 100, heat_cost: 10, cooldown_ms: 500 },
      { name: '弱焰弹', base_damage: 10, heat_cost: 50, cooldown_ms: 2000 },
    ], [])
    expect(r.headline).toContain('弱焰弹')
    expect(r.ctaPath).toBe('/admin/skills')
  })
  it('用户页：高战力低关卡账号入结论', () => {
    const r = buildUsersInsight([{ nickname: '异常号', power: 9000, max_stage: 1 }])
    expect(r.headline).toContain('异常号')
  })
  it('重算页：不符率最高关卡入结论', () => {
    const r = buildRecomputesInsight({ levels: [{ level_id: 7, rate_permille: 200, mismatched: 10, total: 50, max_user_mismatches: 3 }] })
    expect(r.headline).toContain('7')
  })
  it('战报页：验真异常计数入结论', () => {
    const r = buildBattlesInsight([{ verify_mismatched: 2, total_enemies: 12, kills: 50, leaked: 20 }])
    expect(r.headline).toContain('验真异常')
  })
  it('空数据诚实空态', () => {
    expect(buildSkillsInsight([], []).headline).toContain('数据不足')
    expect(buildUsersInsight([]).headline).toContain('数据不足')
    expect(buildRecomputesInsight(null).headline).toContain('数据不足')
    expect(buildBattlesInsight([]).headline).toContain('数据不足')
  })
})
