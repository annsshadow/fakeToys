import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { REACTION_PAIRS } from '@/utils/reactionPairs'
import { ELEMENT_ORDER, lookupReaction, type Element, type ReactionKey } from '@/game/elements'

// 测试位置：laiyipao/miniapp/src/pages/battle/；上 5 级到 worktree 根（docs 所在）
const specPath = join(__dirname, '../../../../../docs/laiyipao/specs/round24-live-element-hud/SPEC.md')
const srcRoot = join(__dirname, '../../..')
const readSpec = () => readFileSync(specPath, 'utf-8')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

interface Hint { reaction: ReactionKey; existing: Element; incoming: Element }

// 参考实现：按 SPEC §2 口径，从引擎真源 REACTION_PAIRS + lookupReaction 派生
function hintsForEnemy(elements: readonly Element[]): Hint[] {
  const set = new Set(elements)
  const out: Hint[] = []
  const seen = new Set<string>()
  for (const reaction of Object.keys(REACTION_PAIRS) as ReactionKey[]) {
    for (const [existing, incoming] of REACTION_PAIRS[reaction]) {
      if (!set.has(existing) || set.has(incoming)) continue
      const key = lookupReaction(existing, incoming)
      if (key !== reaction) continue
      const id = reaction + '|' + existing + '|' + incoming
      if (!seen.has(id)) { seen.add(id); out.push({ reaction, existing, incoming }) }
    }
  }
  return out
}

describe('R24 只读元素层 HUD 契约', () => {
  it('G1 纯派生提示遵循引擎反应真相（焰+冰=蒸汽爆发，焰+电=过热，冰+电=超导）', () => {
    const fire = hintsForEnemy(['fire'])
    expect(fire.some(h => h.reaction === 'steam_burst' && h.incoming === 'ice')).toBe(true)
    expect(fire.some(h => h.reaction === 'overheat' && h.incoming === 'lightning')).toBe(true)
    expect(fire.some(h => h.reaction === 'superconduct')).toBe(false)
    const ice = hintsForEnemy(['ice'])
    expect(ice.some(h => h.reaction === 'superconduct' && h.incoming === 'lightning')).toBe(true)
    expect(ice.some(h => h.reaction === 'steam_burst' && h.incoming === 'fire')).toBe(true)
    const lightning = hintsForEnemy(['lightning'])
    expect(lightning.some(h => h.reaction === 'overheat' && h.incoming === 'fire')).toBe(true)
    expect(lightning.some(h => h.reaction === 'superconduct' && h.incoming === 'ice')).toBe(true)
  })

  it('G1 空集合与全元素集合均走诚实空态，不编造提示', () => {
    expect(hintsForEnemy([])).toEqual([])
    expect(hintsForEnemy(ELEMENT_ORDER)).toEqual([])
    expect(lookupReaction('fire', 'fire')).toBeNull()
  })

  it('G2 引擎只读快照范例存在且用 slice 返回副本', () => {
    const engine = readSrc('src/game/engine.ts')
    expect(engine).toContain('reactionTimeline(): ReactionTimelineEvent[] {')
    expect(engine).toContain('return this.reactionTimelineEvents.slice()')
    expect(readSpec()).toContain('elementLayerHud()')
  })

  it('G2 五类选择器与空态/依据逐字文案被钉死', () => {
    const spec = readSpec()
    for (const s of ['.r24-element-hud', '.r24-enemy-row', '[data-r24-uid]', '.r24-reaction-hint', '.r24-hud-empty', '.r24-ai-note']) {
      expect(spec).toContain(s)
    }
    expect(spec).toContain('当前场上无可预判的元素反应')
    expect(spec).toContain('依据：敌人 stacks 元素层 × REACTION_PAIRS(源自引擎 lookupReaction) 派生；只读快照不进 replay_hash')
  })

  it('G3 诚实红线逐字守卫', () => {
    const spec = readSpec()
    expect(spec).toContain('辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准')
    expect(spec).toContain('不得写回引擎状态')
    expect(spec).toContain('不编造提示')
  })

  it('G4 AI 四判据关键词齐备', () => {
    const spec = readSpec()
    expect(spec).toContain('可见汉字「AI」标签')
    expect(spec).toContain('真实数据结论')
    expect(spec).toContain('依据行')
    expect(spec).toContain('可执行 CTA')
  })

  it('G5 令牌锚点保留且禁用 --r24- 新令牌', () => {
    const tokens = readSrc('src/styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toContain('--r24-')
  })

  it('G5 源码依据仍成立：enemies 运行时数组、stacks Map、battle.vue tick 路径', () => {
    const engine = readSrc('src/game/engine.ts')
    expect(engine).toContain('enemies: Map<number, EnemyDef>')
    expect(engine).toContain('this.enemies.push(e)')
    const types = readSrc('src/game/types.ts')
    expect(types).toContain('stacks: Map<Element, bigint>')
    const battle = readSrc('src/pages/battle/battle.vue')
    expect(battle).toContain('const renderMode = ref<')
    expect(battle).toContain('setInterval(fallbackTick')
  })
})