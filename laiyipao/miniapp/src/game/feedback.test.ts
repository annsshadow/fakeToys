// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from 'vitest'
import { damageShareFromEvents, defeatReview, reactionCue, summarizeCardPicks } from './feedback'
import { decodeDiscardCode, PICK_CODE_BASE } from './engine'

describe('R9 战斗反馈归因（game/feedback）', () => {
  it('card_picks 解码取/弃/跳，缺数据返回诚实空态', () => {
    // -1 = 跳过；-7 = 弃原手牌第 0 张后取剩余第 0 张（见 engine.ts 枚举表）
    expect(summarizeCardPicks([0, -1, -7, 2])).toEqual({
      taken: 3,
      discarded: 1,
      skipped: 1,
      total: 4,
    })
    expect(summarizeCardPicks([]).total).toBe(0)
  })
  it('本地 card_picks 解码与引擎权威解码逐码一致', () => {
    for (let pick = -22; pick <= -3; pick++) {
      const d = decodeDiscardCode(-pick - PICK_CODE_BASE)
      expect(d).not.toBe(null)
      if (!d) continue
      const s = summarizeCardPicks([pick])
      expect(s.taken).toBe(d.take >= 0 ? 1 : 0)
      expect(s.discarded).toBe((d.mask & 1) + ((d.mask >> 1) & 1) + ((d.mask >> 2) & 1))
    }
  })

  it('反应伤害占比只从真实 hit/reaction 事件计算', () => {
    const share = damageShareFromEvents([
      { type: 'hit', damage: 1000n },
      { type: 'reaction', reaction: 'overheat', damage: 250n, resisted: false },
    ])
    expect(share).toEqual({ reactionDamage: 250, totalDamage: 1000, ratioPct: 25 })
    expect(damageShareFromEvents([]).ratioPct).toBe(null)
  })

  it('反应因果提示带真实名称、元素组合、依据与 CTA；缺元素时不编造', () => {
    const cue = reactionCue('overheat', 'fire', 'lightning', 1234n)
    expect(cue.name).toBe('过热')
    expect(cue.pair).toBe('焰+电')
    expect(cue.cue).toContain('密集波 25% 溅射')
    expect(cue.cue).toContain('依据 reaction=overheat/damage=1234')
    expect(cue.cue).toContain('点卡凑同元素')

    const missing = reactionCue('steam_burst', '', undefined, 0n)
    expect(missing.pair).toBe('元素组合未上报')
    expect(missing.cue).toContain('元素组合未上报')
  })

  it('失败复盘只从真实波次/击杀/漏怪/选卡数据给根因与动作，缺数据不编造', () => {
    const review = defeatReview({
      wave: 7, totalWaves: 8, kills: 23, leaked: 24, reactions: 0,
      heatMax: 92, cardPicks: Array.from({ length: 6 }, () => -1),
      elementsUsed: { fire: 351 },
    })
    expect(review).toContain('第 7/8 波失守')
    expect(review).toContain('漏怪 24/47（51%）')
    expect(review).toContain('跳过选卡 6 波')
    expect(review).toContain('仅焰 1 种元素')
    expect(review).toContain('峰值热量 92/100')
    expect(review).toContain('下一局')
    expect(review).toContain('第二种元素')
    expect(review).toContain('依据：wave/kills/leaked/reactions/card_picks/heat_max/elements_used')
    expect(review).not.toContain('。；')

    expect(defeatReview({ wave: null, kills: null, leaked: null, reactions: null })).toBe(
      '本局缺少失败复盘所需的波次/击杀/漏怪/反应数据，不做估算。',
    )
  })
})
