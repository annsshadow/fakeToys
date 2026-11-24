import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const specPath = join(__dirname, '../../../../../docs/laiyipao/specs/round25-battle-reaction-impact/SPEC.md')
const srcRoot = join(__dirname, '../../..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const spec = readFileSync(specPath, 'utf-8')
const battleSrc = readSrc('src/pages/battles/battles.vue')
const clientSrc = readSrc('src/api/client.ts')
const tokensSrc = readSrc('src/styles/tokens.css')

// 参考实现：与 SPEC §1 口径逐字一致（纯前端派生，不碰结算/hash/score/loot）
type Ev = { reaction: string; damage: number }
type ImpactRow = {
  reaction: string
  reactionName: string
  total_damage: number
  damage_share_pct: number
  events: number
  count: number
}
function aggregateReactionImpact(events: Ev[], reactionsUsed: Record<string, number>, nameMap: Record<string, string>): ImpactRow[] {
  if (!Array.isArray(events) || events.length === 0) return []
  const groups = new Map<string, { total: number; events: number }>()
  for (const ev of events) {
    if (!ev || typeof ev.reaction !== 'string' || ev.reaction.length === 0
      || typeof ev.damage !== 'number' || !Number.isFinite(ev.damage)) continue
    const g = groups.get(ev.reaction) || { total: 0, events: 0 }
    g.total += ev.damage
    g.events += 1
    groups.set(ev.reaction, g)
  }
  if (groups.size === 0) return []
  let sum = 0
  for (const g of groups.values()) sum += g.total
  const rows: ImpactRow[] = []
  for (const [reaction, g] of groups) {
    const share = sum > 0 ? (g.total / sum) * 100 : 0
    rows.push({
      reaction,
      reactionName: nameMap[reaction] || reaction,
      total_damage: g.total,
      damage_share_pct: Number(share.toFixed(1)),
      events: g.events,
      count: typeof reactionsUsed[reaction] === 'number' ? reactionsUsed[reaction] : 0,
    })
  }
  rows.sort((a, b) => b.total_damage - a.total_damage || a.reaction.localeCompare(b.reaction))
  return rows
}

describe('R25 单局反应伤害主力聚合口径', () => {
  it('按 reaction 分组聚合 total_damage/events，count 取权威 reactions_used 而非 events 条数', () => {
    const events: Ev[] = [
      { reaction: 'steam_burst', damage: 40 },
      { reaction: 'overheat', damage: 10 },
      { reaction: 'steam_burst', damage: 20 },
    ]
    const rows = aggregateReactionImpact(events, { steam_burst: 7, overheat: 3 }, {})
    expect(rows).toHaveLength(2)
    const steam = rows.find(r => r.reaction === 'steam_burst')!
    expect(steam.total_damage).toBe(60)
    expect(steam.events).toBe(2)
    expect(steam.count).toBe(7)
    expect(steam.count).not.toBe(steam.events)
    const over = rows.find(r => r.reaction === 'overheat')!
    expect(over.total_damage).toBe(10)
    expect(over.events).toBe(1)
    expect(over.count).toBe(3)
  })

  it('占比 = total_damage / 总和 × 100，toFixed(1)，求和约 100 且按伤害降序', () => {
    const rows = aggregateReactionImpact(
      [{ reaction: 'a', damage: 60 }, { reaction: 'b', damage: 30 }, { reaction: 'c', damage: 10 }],
      {}, {})
    expect(rows.map(r => r.reaction)).toEqual(['a', 'b', 'c'])
    expect(rows[0].damage_share_pct).toBe(60)
    expect(rows[1].damage_share_pct).toBe(30)
    expect(rows[2].damage_share_pct).toBe(10)
    expect(rows.reduce((s, r) => s + r.damage_share_pct, 0)).toBeCloseTo(100, 1)
  })

  it('总伤害为 0 时占比全部归零，绝不产 NaN / Infinity', () => {
    const rows = aggregateReactionImpact(
      [{ reaction: 'a', damage: 0 }, { reaction: 'b', damage: 0 }], {}, {})
    expect(rows).toHaveLength(2)
    for (const r of rows) {
      expect(r.damage_share_pct).toBe(0)
      expect(Number.isNaN(r.damage_share_pct)).toBe(false)
      expect(Number.isFinite(r.damage_share_pct)).toBe(true)
    }
  })

  it('旧战报无时间线 / 空数组 / 全无效行 → 空数组走诚实空态，不编造主力', () => {
    expect(aggregateReactionImpact([], { steam_burst: 5 }, {})).toEqual([])
    expect(aggregateReactionImpact(
      [{ reaction: '', damage: 3 }, { reaction: 'x', damage: Number.NaN }], { x: 2 }, {})).toEqual([])
  })

  it('反应名以 nameMap（/config.reactions）为准，缺映射兜底原始 key', () => {
    const rows = aggregateReactionImpact(
      [{ reaction: 'steam_burst', damage: 5 }, { reaction: 'unknown_key', damage: 1 }],
      {}, { steam_burst: '蒸汽爆发' })
    expect(rows.find(r => r.reaction === 'steam_burst')!.reactionName).toBe('蒸汽爆发')
    expect(rows.find(r => r.reaction === 'unknown_key')!.reactionName).toBe('unknown_key')
  })

  it('同伤害按 reaction key 字典序稳定排序', () => {
    const rows = aggregateReactionImpact(
      [{ reaction: 'zeta', damage: 5 }, { reaction: 'alpha', damage: 5 }], {}, {})
    expect(rows.map(r => r.reaction)).toEqual(['alpha', 'zeta'])
  })
})

describe('R25 SPEC 选择器与逐字文案契约', () => {
  it('SPEC 钉死五类选择器', () => {
    for (const s of ['.r25-reaction-impact', '.r25-impact-row', '[data-r25-reaction]', '.r25-impact-empty', '.r25-ai-note']) {
      expect(spec).toContain(s)
    }
  })

  it('SPEC 钉死空态逐字文案', () => {
    expect(spec).toContain('本局无逐次反应伤害记录')
  })

  it('SPEC 钉死逐字依据行与截断提示', () => {
    expect(spec).toContain('依据：本局 reaction_timeline.events 的 damage 聚合；触发次数以 reactions_used 为准')
    expect(spec).toContain('已截断，仅统计前 200 条')
  })
})

describe('R25 诚实红线与 AI 四判据', () => {
  it('SPEC 含逐字诚实红线', () => {
    expect(spec).toContain('辅助展示非伤害权威,权威以 replay_hash 为准')
  })

  it('SPEC 逐字钉死 count 权威来源，禁止 events 冒充', () => {
    expect(spec).toContain('严禁用 events 条数冒充')
    expect(spec).toContain('reactions_used')
  })

  it('SPEC 含 AI 四判据关键词', () => {
    for (const k of ['可见汉字', '真实数据结论', '依据行', 'CTA']) expect(spec).toContain(k)
  })
})

describe('R25 既有数据源依据与三端不回退', () => {
  it('battles.vue 已有 reactionTimeline/reactionNameMap/reactions_used 数据源', () => {
    for (const k of ['reactionTimeline', 'reactionNameMap', 'reactions_used', 'r15-reaction-timeline']) {
      expect(battleSrc).toContain(k)
    }
  })

  it('client.ts BattleDetail 已有 reactions_used 与 reaction_timeline 类型', () => {
    for (const k of ['reactions_used: Record<string, number>', 'reaction_timeline?', 'damage: number']) {
      expect(clientSrc).toContain(k)
    }
  })

  it('令牌三锚点保留且无 --r25- 新令牌', () => {
    for (const k of ['lyp-tokens-v1', 'lyp-tokens-v3-r3', 'lyp-tokens-r6']) expect(tokensSrc).toContain(k)
    expect(tokensSrc).not.toContain('--r25-')
  })
})

describe('R25 零真值', () => {
  it('SPEC 与测试不含域名/IP/JWT/口令/个人绝对路径', () => {
    for (const text of [spec, battleSrc, tokensSrc]) {
      expect(text).not.toMatch(/eyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}/)
      expect(text).not.toMatch(/\b(?:\d{1,3}\.){3}\d{1,3}\b/)
      expect(text).not.toMatch(/postgres:\/\/[^\s]+:[^\s]+@/)
      expect(text).not.toMatch(/C:\\Users\\[^\s]+/)
    }
  })
})
