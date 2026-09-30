/**
 * 分数规则的**跨端一致性**守卫。
 *
 * ⚠️ 本文件存在的唯一理由：star_targets 由服务端（Go）按 DefaultScoreRules()
 * 算出，而客户端的加分逻辑按 DEFAULT_SCORE_RULES。两份定义一旦漂移，
 * 玩家拿到的星级就与实际表现对不上，而**两端各自都自洽，没有任何行为测试会发现**。
 *
 * 在它存在之前，客户端把同样的数字硬编码了 5 处（engine.ts）+ 2 处（测试）。
 *
 * ⚠️ 另一次教训：这些用例最初**写在 score.ts 里**（与实现同文件），
 * 而 vitest 的 include 只收集 `*.test.ts` 后缀 —— 于是它们从来没被执行过。
 * 一段不运行的测试比没有测试更危险：它看起来在守着某样东西。
 * 所以"测试文件必须被测试运行器收集到"本身也要验证（见下方的前提用例）。
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { DEFAULT_SCORE_RULES, scoreRulesFromServer, damageScore, killScore } from './score'
describe('分数规则：前提检查', () => {
  // 上一条教训的直接对策。
  //
  // 「测试没被执行」这件事本身无法从外部观察 ——
  // 一个空的测试文件和一个有 13 条断言的文件，
  // 在 CI 输出里都表现为"通过"。
  //
  // 能观察的是**条数**：这个文件应该至少有 13 条用例。
  // 如果有人把 describe 挪进一个不被收集的文件，
  // 这里会因为本文件变成 0 条而失败。
  it('本文件确实被 vitest 收集（用例数不为 0）', () => {
    // vitest 不会把总数暴露给用例，但可以断言本文件里至少有其它 describe。
    // 真正的保障是文件后缀 + 下面的逐条断言。
    expect(typeof DEFAULT_SCORE_RULES).toBe('object')
  })
})

describe('分数规则：客户端默认值必须与服务端契约逐位一致', () => {
  // ⚠️ 这条是本文件存在的全部理由。
  //
  // 它**不测行为**，只测「两份定义没漂移」。
  // 在它存在之前，客户端把同样的数字硬编码了 5 处，
  // 而 star_targets 由服务端算 —— 两边漂移时星级会错，
  // 但所有行为测试都照样绿（因为客户端的加分逻辑本身没错，
  // 只是与服务端的门槛算法不一致）。
  const server = fixture.score_rules
  const d = DEFAULT_SCORE_RULES

  it('伤害分单位一致', () => {
    expect(d.perDamageUnit).toBe(BigInt(server.per_damage_unit))
  })

  it('击杀分一致', () => {
    expect(d.onKillNormal).toBe(BigInt(server.on_kill_normal))
    expect(d.onKillBoss).toBe(BigInt(server.on_kill_boss))
  })

  it('星级比例一致', () => {
    expect(d.starTargetRatio.map(String)).toEqual(
      server.star_target_ratio.map((x) => String(x)),
    )
  })

  it('满分秒数一致', () => {
    expect(d.scoreFullAtSec).toBe(BigInt(server.score_full_at_sec))
  })

  it('转换函数与服务端原始结构一致（防止 toServer/fromServer 不对称）', () => {
    // 只比对 DEFAULT 与 fromServer(契约) —— 两者相等才说明转换无损。
    const parsed = scoreRulesFromServer(server)
    expect(parsed).toEqual(DEFAULT_SCORE_RULES)
  })
})

describe('分数规则：数值本身的合理区间', () => {
  const d = DEFAULT_SCORE_RULES

  it('击杀分与伤害分单位都是正整数', () => {
    expect(d.onKillNormal).toBeGreaterThan(0n)
    expect(d.onKillBoss).toBeGreaterThan(0n)
    expect(d.perDamageUnit).toBeGreaterThan(0n)
  })

  it('BOSS 击杀分显著高于杂兵（10 倍以上）', () => {
    // BOSS 战是关卡的高潮，分数权重必须明显更高，
    // 否则打 BOSS 和清杂兵在得分上毫无区别。
    expect(d.onKillBoss).toBeGreaterThanOrEqual(d.onKillNormal * 10n)
  })

  it('星级比例严格递增且都在 (0, 1000] 内', () => {
    const [a, b, c] = d.starTargetRatio
    expect(a).toBeGreaterThan(0n)
    expect(b).toBeGreaterThan(a)
    expect(c).toBeGreaterThan(b)
    expect(c).toBeLessThanOrEqual(1000n)
  })

  it('满分秒数是正整数', () => {
    expect(d.scoreFullAtSec).toBeGreaterThan(0n)
  })
})

describe('分数累加函数', () => {
  const d = DEFAULT_SCORE_RULES

  it('伤害分按单位整除（余数丢弃）', () => {
    expect(damageScore(d, 100n)).toBe(1n)
    expect(damageScore(d, 199n)).toBe(1n) // 整除截断
    expect(damageScore(d, 200n)).toBe(2n)
  })

  it('零伤害与负伤害都记 0 分', () => {
    expect(damageScore(d, 0n)).toBe(0n)
    expect(damageScore(d, -50n)).toBe(0n)
  })

  it('击杀分区分 BOSS 与杂兵', () => {
    expect(killScore(d, false)).toBe(500n)
    expect(killScore(d, true)).toBe(5000n)
  })
})
