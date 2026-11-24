import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { REACTION_PAIRS } from '@/utils/reactionPairs'
import { ELEMENT_ORDER, lookupReaction, REACTION_ORDER, type Element, type ReactionKey } from '@/game/elements'

// 测试位置 laiyipao/miniapp/src/pages/skills/：上 5 级到 worktree 根（docs 所在），上 3 级到 miniapp
const specPath = join(__dirname, '../../../../../docs/laiyipao/specs/round23-loadout-reaction-coverage/SPEC.md')
const srcRoot = join(__dirname, '../../..')
const readSpec = () => readFileSync(specPath, 'utf-8')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

// 参考实现：按 SPEC §2 口径，用引擎真源钉死覆盖集合与补齐差集（业务代码尚未落地）。
function covered(elements: Element[]): ReactionKey[] {
  const set = new Set<ReactionKey>()
  for (const a of elements) for (const b of elements) {
    if (a === b) continue
    const k = lookupReaction(a, b)
    if (k) set.add(k)
  }
  return REACTION_ORDER.filter((k) => set.has(k))
}
function bestGap(equipped: Element[]): { element: Element; added: ReactionKey[] } | null {
  const before = new Set(covered(equipped))
  let best: { element: Element; added: ReactionKey[] } | null = null
  for (const e of ELEMENT_ORDER) {
    const after = new Set(covered([...equipped, e]))
    const added = REACTION_ORDER.filter((k) => after.has(k) && !before.has(k))
    if (added.length && (!best || added.length > best.added.length)) best = { element: e, added }
  }
  return best
}

describe('R23 契约守卫：SPEC 钉死数据基础与算法', () => {
  it('SPEC 钉死数据基础（引擎真源，纯客户端派生）', () => {
    const spec = readSpec()
    for (const k of ['store.loadout', 'skill.element', 'apply_element', 'REACTION_PAIRS', 'lookupReaction', 'ELEMENT_NAME', '纯客户端派生', '不进 replay_hash', '不改结算']) {
      expect(spec, '缺条款 ' + k).toContain(k)
    }
  })
  it('SPEC 钉死覆盖集合/补齐差集/三分枝', () => {
    const spec = readSpec()
    for (const k of ['C(E)', 'added', '当前构筑没有元素技能，无法诊断反应覆盖', '当前构筑已覆盖全部可触发反应，无需补齐', '确定性']) {
      expect(spec, '缺算法条款 ' + k).toContain(k)
    }
  })
})

describe('R23 选择器与 AI 四判据', () => {
  it('SPEC 钉死 r23 选择器与逐字空态', () => {
    const spec = readSpec()
    for (const sel of ['.r23-coverage-card', '.r23-coverage-tag', '[data-r23-reaction]', '.r23-gap-suggest', '.r23-coverage-empty', '.r23-ai-note']) {
      expect(spec, '缺选择器 ' + sel).toContain(sel)
    }
  })
  it('SPEC 钉死 AI 四判据与诚实红线逐字串', () => {
    const spec = readSpec()
    for (const s of [
      '可见汉字「AI」',
      '真实数据结论',
      '依据：构筑槽元素 × REACTION_PAIRS(源自引擎 lookupReaction)派生',
      '去调整出战配置',
      '辅助诊断非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准',
      '零真值',
    ]) {
      expect(spec, '缺 AI/红线条款 ' + s).toContain(s)
    }
  })
})

describe('R23 三端不回退基线（miniapp）', () => {
  it('引擎真源与 store loadout 基线保留', () => {
    expect(readSrc('src/utils/reactionPairs.ts')).toContain('REACTION_PAIRS')
    const e = readSrc('src/game/elements.ts')
    expect(e).toContain('export function lookupReaction')
    expect(e).toContain('REACTION_ORDER')
    const g = readSrc('src/store/game.ts')
    expect(g).toContain('const loadout = ref<number[]>([0, 0, 0, 0])')
    expect(g).toContain('equippedElements')
  })
  it('skills.vue 构筑台基线保留（诊断卡为增量）', () => {
    const s = readSrc('src/pages/skills/skills.vue')
    for (const k of ['store.loadout.slice(0, 4)', 'allSkillDefs', 'ELEMENT_NAME', 'persistLoadout']) {
      expect(s, 'skills.vue 基线丢失 ' + k).toContain(k)
    }
  })
  it('tokens: v1/v3-r3/r6 锚点保留且无 r23 新增令牌', () => {
    const s = readSrc('src/styles/tokens.css')
    expect(s).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(s).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(s).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(s).not.toContain('--r23-')
  })
})

describe('R23 确定性覆盖集合与补齐差集（引擎真源）', () => {
  it('已装备焰+冰覆盖蒸汽爆发（按引擎真相）', () => {
    expect(lookupReaction('fire', 'ice')).toBe('steam_burst')
    expect(lookupReaction('ice', 'fire')).toBe('steam_burst')
    expect(covered(['fire', 'ice'])).toEqual(['steam_burst'])
  })
  it('焰+冰补电：新增过热与超导，差集不含已覆盖', () => {
    const g = bestGap(['fire', 'ice'])
    expect(g).not.toBeNull()
    expect(g!.element).toBe('lightning')
    expect(g!.added).toContain('overheat')
    expect(g!.added).toContain('superconduct')
    expect(g!.added).not.toContain('steam_burst')
  })
  it('无元素构筑走诚实空态（单个元素无法单独成反应）', () => {
    expect(covered([])).toEqual([])
    expect(bestGap([])).toBeNull()
  })
  it('单元素构筑补另一元素可新增反应', () => {
    const g = bestGap(['fire'])
    expect(g).not.toBeNull()
    expect(g!.added.length).toBeGreaterThan(0)
  })
  it('全五元素构筑无可补齐（已覆盖全部反应）', () => {
    expect(bestGap([...ELEMENT_ORDER])).toBeNull()
  })
  it('确定性：相同输入两次结果逐项相等', () => {
    expect(covered(['fire', 'ice', 'lightning'])).toEqual(covered(['fire', 'ice', 'lightning']))
    expect(bestGap(['fire', 'ice'])).toEqual(bestGap(['fire', 'ice']))
  })
  it('REACTION_PAIRS 与 lookupReaction 同源：每条对可反查', () => {
    for (const [key, pairs] of Object.entries(REACTION_PAIRS)) {
      expect(pairs.length, key + ' 无元素对').toBeGreaterThan(0)
      for (const [a, b] of pairs) expect(lookupReaction(a, b)).toBe(key)
    }
  })
})

describe('R23-BUG 回归：深链接直入必须加载权威构筑（假空态防护）', () => {
  it('doLoadPage 幂等加载路径必须 loadLoadout，否则把默认[0,0,0,0]谎报成空构筑', () => {
    const s = readSrc('src/pages/skills/skills.vue')
    const start = s.indexOf('async function doLoadPage')
    expect(start, '找不到 doLoadPage').toBeGreaterThan(-1)
    const end = s.indexOf('function loadPage()', start)
    const body = s.slice(start, end > start ? end : start + 1400)
    expect(body, 'R23-BUG: doLoadPage 未加载权威出战构筑，深链接直入会把默认全 0 谎报成空构筑')
      .toContain('store.loadLoadout()')
  })
})
