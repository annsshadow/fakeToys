import { describe, it, expect } from 'vitest'
import vectors from '@vectors/formula_vectors.json'
import { replaySkillsSegmentOf } from './engine'
import { skillBaseDamageAtLevel, DEFAULT_SKILL_RULES } from './skill'

// 回放前缀 S 段的**跨端契约**（第 56 轮）。
//
// Go 侧对应 `server/internal/domain/replay_skills_contract_test.go`，
// 两侧读 `server/testdata/formula_vectors.json` 的**同一个**字面期望值。
//
// ## 为什么这道测试必须存在
//
// 服务端在结算时用 Go 侧重算这个串，与客户端上报的**逐字节**比对。
// 两端格式一旦漂移 —— 哪怕只是分隔符从 `:` 变成 `,` ——
// 后果是**所有合法玩家的每一局**都被判成作弊，
// 而客户端单测、Go 单测、服务端 e2e 会**全部照样绿**。
//
// `scripts/e2e.ps1` 帮不上忙：它自己构造 payload 且不带这个字段（发空串，
// 服务端按设计放行）。所以「上报了必须对得上」这半边，只有这里在守。

interface ReplaySkillsVector {
  name: string
  _why?: string
  skills: Array<{
    slot: number
    skill_id: number
    level: number
    base_damage: number
    heat_cost: number
    apply_stacks: number
  }>
  expected: string
}

const cases = (vectors as unknown as { replay_skills: { cases: ReplaySkillsVector[] } })
  .replay_skills?.cases

describe('回放前缀 S 段（跨端契约）', () => {
  it('契约向量不是空的', () => {
    // 空数组时 forEach 一条都不跑，测试会静默变成空转。
    // 向量被误删/被清空必须红。
    expect(Array.isArray(cases)).toBe(true)
    expect(cases.length).toBeGreaterThan(0)
  })

  it('每个用例名唯一', () => {
    const names = cases.map((c) => c.name)
    expect(new Set(names).size).toBe(names.length)
  })

  for (const c of cases) {
    it(`${c.name} —— ${c._why ?? ''}`, () => {
      const got = replaySkillsSegmentOf(
        c.skills.map((s) => ({
          slot: s.slot,
          skillId: s.skill_id,
          // 等级在这里烘进底伤，与 Go 的 SkillBaseDamageAtLevel 同一套公式
          baseDamage: skillBaseDamageAtLevel(
            DEFAULT_SKILL_RULES,
            BigInt(s.base_damage),
            s.level,
          ),
          applyStacks: BigInt(s.apply_stacks),
          heatCost: BigInt(s.heat_cost),
        })),
      )
      expect(got).toBe(c.expected)
    })
  }
})

// ⚠️ 这一条不是「凑覆盖率」，它盯的是上一批最贵的一个坑：
//
// `.sort()` 无参数时按 **UTF-16 码元**比较，不是按槽位数值。
// 所以 slot=10 排在 slot=2 **前面**。
//
// Go 侧第一版写成了按槽位排序，slot ≥ 10 的对局会被判不匹配 ——
// 而代码读起来完全合理、单测也全绿（当时用例只有 slot < 10）。
describe('S 段排序是字符串序，不是槽位数值序', () => {
  it('slot 10 排在 slot 2 前面', () => {
    const seg = replaySkillsSegmentOf([
      { slot: 2, skillId: 20, baseDamage: 100n, applyStacks: 1n, heatCost: 20n },
      { slot: 10, skillId: 30, baseDamage: 100n, applyStacks: 1n, heatCost: 20n },
    ])
    expect(seg).toBe('10:30:100:1:20,2:20:100:1:20')
  })

  it('与「按槽位排序」的写法确实不同 —— 否则上面那条就是废话', () => {
    const skills = [
      { slot: 2, skillId: 20, baseDamage: 100n, applyStacks: 1n, heatCost: 20n },
      { slot: 10, skillId: 30, baseDamage: 100n, applyStacks: 1n, heatCost: 20n },
    ]
    const bySlot = [...skills].sort((a, b) => a.slot - b.slot).join(',')
    expect(bySlot).not.toBe(replaySkillsSegmentOf(skills))
  })
})

describe('S 段与上报走同一个来源', () => {
  it('空构筑产出空串（不是 ",," 之类）', () => {
    // 空技能玩家必须能被放行，否则他们每局都被判不符。
    expect(replaySkillsSegmentOf([])).toBe('')
  })

  it('同一份构筑无论传入顺序如何，S 段都相同', () => {
    // 上报与哈希两处共用这个函数；若它对顺序敏感，
    // 客户端与 Go 的 map/slice 遍历顺序差异就会造成随机误判。
    const mk = (slot: number, id: number) => ({
      slot,
      skillId: id,
      baseDamage: BigInt(100 + slot),
      applyStacks: 1n,
      heatCost: 20n,
    })
    const a = replaySkillsSegmentOf([mk(0, 1), mk(1, 2), mk(2, 3)])
    const b = replaySkillsSegmentOf([mk(2, 3), mk(0, 1), mk(1, 2)])
    expect(a).toBe(b)
  })
})
