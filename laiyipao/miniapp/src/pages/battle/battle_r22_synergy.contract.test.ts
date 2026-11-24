import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { REACTION_PAIRS } from '@/utils/reactionPairs'
import { lookupReaction, REACTION_ORDER, type Element, type ReactionKey } from '@/game/elements'

// 测试位置：laiyipao/miniapp/src/pages/battle/；上 5 级到 worktree 根（docs 所在）
const specPath = join(__dirname, '../../../../../docs/laiyipao/specs/round22-card-reaction-synergy/SPEC.md')
const srcRoot = join(__dirname, '../..')
const readSpec = () => readFileSync(specPath, 'utf-8')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

// 参考实现：按 SPEC §2 口径的确定性差集算法（实施任务的业务代码尚未落地，
// 这里用引擎真源 REACTION_PAIRS/lookupReaction 钉死算法口径与期望值）。
function coveredReactions(elements: Element[]): Set<ReactionKey> {
  const out = new Set<ReactionKey>()
  for (const a of elements) for (const b of elements) {
    if (a === b) continue
    const k = lookupReaction(a, b)
    if (k) out.add(k)
  }
  return out
}
function synergy(equipped: Element[], card: Element) {
  const before = coveredReactions(equipped)
  const after = coveredReactions([...equipped, card])
  const added = REACTION_ORDER.filter((k) => after.has(k) && !before.has(k))
  const reinforced = REACTION_ORDER.filter((k) => after.has(k) && before.has(k))
  return { before, after, added, reinforced }
}

describe('R22 契约守卫：SPEC 钉死派生口径与算法', () => {
  it('SPEC 钉死数据基础（引擎真源，纯客户端派生）', () => {
    const spec = readSpec()
    for (const k of ['REACTION_PAIRS', 'lookupReaction', 'equippedFromSnapshot', 'applyElement', 'ELEMENT_NAME', 'Card.element', '纯客户端派生', '不进 replay_hash', '不改结算']) {
      expect(spec, `缺条款 ${k}`).toContain(k)
    }
  })
  it('SPEC 钉死覆盖集合/差集/三分枝文案', () => {
    const spec = readSpec()
    for (const k of ['C(E)', '差集', 'AI · 新增【', '可搭配', 'AI · 强化已有【', '确定性']) {
      expect(spec, `缺算法条款 ${k}`).toContain(k)
    }
  })
})

describe('R22 选择器与 AI 四判据', () => {
  it('SPEC 钉死 r22 选择器与逐字空态/红线', () => {
    const spec = readSpec()
    for (const sel of ['.r22-synergy-card', '.r22-synergy-tag', '[data-r22-reaction]', '.r22-synergy-empty', '.r22-ai-note']) {
      expect(spec, `缺选择器 ${sel}`).toContain(sel)
    }
    expect(spec).toContain('该卡无元素，无法预判反应（属性/机制卡）')
    expect(spec).toContain('当前构筑还没有元素技能，无法预判反应')
  })
  it('SPEC 钉死 AI 四判据与诚实红线逐字串', () => {
    const spec = readSpec()
    for (const s of [
      '可见汉字「AI」',
      '真实数据结论',
      '依据：已装备元素 × 手牌元素，经 REACTION_PAIRS(源自引擎 lookupReaction)派生',
      '去装备/选它',
      '辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准',
      '零真值',
    ]) {
      expect(spec, `缺 AI/红线条款 ${s}`).toContain(s)
    }
  })
})

describe('R22 三端不回退基线（miniapp）', () => {
  it('reactionPairs/elements 真源保留', () => {
    expect(readSrc('utils/reactionPairs.ts')).toContain('REACTION_PAIRS')
    expect(readSrc('game/elements.ts')).toContain('export function lookupReaction')
  })
  it('battle.vue 选卡基线保留（预判为增量叠加）', () => {
    const s = readSrc('pages/battle/battle.vue')
    for (const k of ['REACTION_PAIRS', 'elementName', 'showCards', 'const hand', 'takeCard', 'discardCard']) {
      expect(s, `battle.vue 基线丢失 ${k}`).toContain(k)
    }
  })
  it('tokens: v1/v3-r3/r6 锚点保留且无 r22 新增令牌', () => {
    const s = readSrc('styles/tokens.css')
    expect(s).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(s).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(s).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(s).not.toContain('--r22-')
  })
})

describe('R22 确定性反应对与差集（引擎真源）', () => {
  it('已装备焰+冰覆盖蒸汽爆发（按引擎真相，非误传的超导）', () => {
    expect(lookupReaction('fire', 'ice')).toBe('steam_burst')
    expect(lookupReaction('ice', 'fire')).toBe('steam_burst')
    expect([...coveredReactions(['fire', 'ice'])]).toEqual(['steam_burst'])
  })
  it('焰+冰再选电：新增过热与超导，差集不含已覆盖', () => {
    const r = synergy(['fire', 'ice'], 'lightning')
    expect(r.before.has('steam_burst')).toBe(true)
    expect(r.added).toEqual(['overheat', 'superconduct'])
    expect(r.added).not.toContain('steam_burst')
    expect(r.reinforced).toEqual(['steam_burst'])
  })
  it('重复已有元素：无新增，识别为强化已有', () => {
    const r = synergy(['fire', 'ice'], 'fire')
    expect(r.added).toEqual([])
    expect(r.reinforced).toContain('steam_burst')
  })
  it('差集确定性：相同输入两次结果逐项相等', () => {
    const a = synergy(['fire', 'ice'], 'lightning')
    const b = synergy(['fire', 'ice'], 'lightning')
    expect(a.added).toEqual(b.added)
    expect(a.reinforced).toEqual(b.reinforced)
  })
  it('REACTION_PAIRS 与 lookupReaction 同源：每条对都能反查到反应', () => {
    for (const [key, pairs] of Object.entries(REACTION_PAIRS)) {
      expect(pairs.length, `${key} 无元素对`).toBeGreaterThan(0)
      for (const [a, b] of pairs) expect(lookupReaction(a, b)).toBe(key)
    }
  })
})
