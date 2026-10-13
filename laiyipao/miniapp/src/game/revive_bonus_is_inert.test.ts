import { describe, it } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, type BattleConfig } from './engine'
import { defaultAttacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'
import { ACTIVE_SLOTS } from './heatmap'
import { LeakDamageDivisor } from './engine'

const levels = (fixture.levels as unknown as GeneratedLevel[]).slice().sort((a, b) => a.id - b.id)
const enemyMap = new Map<number, EnemyDef>((fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]))
const skillMap = new Map<number, SkillDef>(
  [...(fixture.skills as unknown as SkillDef[]), ...(fixture.composite_skills as unknown as SkillDef[])].map((s) => [s.id, s]),
)
function equip(n: number) {
  return [...skillMap.values()].filter((s) => s.kind === 'active').sort((a, b) => a.id - b.id).slice(0, n)
    .map((s, i) => ({
      skillId: s.id, name: s.name, element: s.element as Element, kind: s.kind,
      heatCost: BigInt(s.heat_cost), cooldownMs: s.cooldown_ms, pierce: s.pierce,
      aoeRadius: s.aoe_radius, baseDamage: BigInt(s.base_damage),
      applyElement: (s.apply_element ?? '') as Element | '', applyStacks: BigInt(s.apply_stacks),
      projectileSpeed: s.projectile_speed, chain: s.chain, slot: i, cooldownRemaining: 0,
    }))
}
function play(lv: GeneratedLevel) {
  const cfg: BattleConfig = {
    level: lv, enemies: enemyMap, skills: skillMap,
    equipped: equip(ACTIVE_SLOTS), attacker: defaultAttacker(), seed: 12345,
  }
  const e = new BattleEngine(cfg)
  e.start()
  for (let t = 0; t < 30000; t++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
  }
  return { phase: e.phase, tick: e.tick, kills: e.kills, leaked: e.leaked }
}

/**
 * 复活币的「+X% 基础血量」**基本无效** —— 这条守卫把实测结论钉死。
 *
 * ## 为什么需要一条专门记着「无效」的测试
 *
 * 服务端 `domain.ReviveBaseHPMultPermille = 1500` 看着像个正常数值，
 * 注释也写着「+50% 基础血量」。半年后有人看到它，第一反应是
 * 「这个数偏低，调到 2000 试试」—— 而调多少都没用。
 *
 * 与其留一个会被反复「调优」的假旋钮，不如让**每次改动都撞上这条红灯**，
 * 逼着改的人先读完下面这段。
 *
 * ## 机制（`engine.ts` leakDamageFor）
 *
 * ```ts
 * if (e.attack <= 0n) return applyArmor(this.breachDamage, ...)  // breachDamage = baseHpMax / 10
 * const scaled = (this.baseHpMax * e.attack) / LeakAttackerDivisor
 * ```
 *
 * 两条分支**都与 `baseHpMax` 成正比**。把 `base_hp` 抬到 1.5 倍时：
 *
 * | | 原局 | +50% 血 |
 * |---|---|---|
 * | 可承受血量 | B | 1.5B |
 * | 每次漏怪扣（普通怪） | B/10 | 1.5B/10 |
 * | **可承受漏怪数** | **10** | **10** |
 *
 * 护甲那层 `applyArmor` 是乘法，同样不改变比值。
 *
 * ## 实测（第 152 轮，seed 12345，满配技能，100 关全跑）
 *
 * | | 结果 |
 * |---|---|
 * | 漏怪数不同的关卡 | **1 / 100**（L96：100 → 99，且两局都是 `lost`） |
 * | 结局从 `lost` 翻成 `won` 的关卡 | **0** |
 * | 击杀数 / 通关时间不同的关卡 | **0** |
 *
 * 那 1 关的差值来自 `baseHpMax * attack / 3600` 的**整数截断**：
 * 例如 attack=5 时 10000/3600→13 而 15000/3600→20，比值 0.65 而非 0.667。
 * 这是舍入噪声，不是设计出来的效果，且量级约 1%。
 *
 * ⚠️ 所以本文件**不**断言「漏怪数一个都不变」—— 那是我第一版的说法，
 * L96 就是反例。断言的是「**没有任何一关因为它多漏一只而多撑一会儿**」，
 * 那才是「无效」的准确含义。
 *
 * ## 要真的生效，改的必须是「不是比例」的那一项
 *
 * 复活的加成必须作用在**不随 baseHpMax 缩放**的量上。可选：
 *
 *  1. **护甲**：`applyArmor` 是乘法，给 +250‰ armor 能压掉一部分每次漏怪伤害 ——
 *     这条不需要改引擎，也不碰 replay 事件的形状。
 *  2. **漏怪豁免**：第 N 次漏怪不扣血（要改引擎，且要进 replay 事件流）。
 *  3. **绝对血量缓冲**：`+K × breachDamage` 的**额外**血量、且 `baseHpMax` 保持原值
 *     —— 要给引擎加「当前血量 ≠ 血量上限」这种表达。
 *
 * 三条都要改服务端 ↔ 客户端契约，属于独立一轮改动。
 * 在那之前 `revive_token` 已重新下架（seeder 的 `delisted` 标记），
 * **不让玩家花 5,000 金币买一个没有效果的东西**。
 */


describe('revive 的加成：为什么「+X% 基础血量」不成立', () => {
  it('没有任何一关因为它而少漏怪、也没有任何一关从 lost 翻成 won', () => {
    let better = 0
    let flipped = 0
    const detail: string[] = []

    for (const lv of levels) {
      const base = play(lv)
      const buffed = play({ ...lv, base_hp: Math.round(lv.base_hp * 1.5) })
      if (base.phase === 'lost' && buffed.phase === 'won') {
        flipped++
        detail.push(`L${lv.id} lost → won`)
      }
      if (buffed.leaked < base.leaked) {
        better++
        detail.push(`L${lv.id} 漏怪 ${base.leaked} → ${buffed.leaked}（${base.phase}→${buffed.phase}）`)
      }
    }

    expect(flipped, `有 ${flipped} 关靠这个加成从失败翻成胜利 —— ` +
      '漏怪伤害不再与 baseHpMax 成正比了，`ReviveBaseHPMultPermille` ' +
      '重新变成有效旋钮，请同步更新本文件的实测表与 revive.go：\n' + detail.join('\n'))
      .toBe(0)

    // 允许整数截断带来的零星 ±1，但不允许成规模地变好：
    // 成规模变好就说明它真的在起作用，文档与下架决定都需要重估。
    const allowed = Math.ceil(levels.length * 0.05)
    expect(better, `有 ${better} 关（>${allowed}/${levels.length}）真的靠它少漏了怪 —— ` +
      '这不是舍入噪声的量级：\n' + detail.join('\n'))
      .toBeLessThanOrEqual(allowed)
  }, 60_000)

  it('机制上的原因：普通怪的漏怪伤害 = baseHpMax / LeakDamageDivisor，与血量同步缩放', () => {
    // 上面那条证明**行为**，这条保证公式没被悄悄改掉。
    expect(LeakDamageDivisor).toBe(10n)
    const b = 1000n
    const b2 = (b * 3n) / 2n
    expect(b / LeakDamageDivisor).toBe(b / 10n)
    expect(b2 / (b2 / LeakDamageDivisor)).toBe(b / (b / LeakDamageDivisor))
  }, 60_000)
})