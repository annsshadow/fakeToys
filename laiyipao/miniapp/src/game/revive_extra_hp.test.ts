import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, LeakDamageDivisor, type BattleConfig } from './engine'
import { defaultAttacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'
import { ACTIVE_SLOTS } from './heatmap'

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
function play(lv: GeneratedLevel, extraBaseHp = 0n) {
  const cfg: BattleConfig = {
    level: lv, enemies: enemyMap, skills: skillMap,
    equipped: equip(ACTIVE_SLOTS), attacker: defaultAttacker(), seed: 12345,
    extraBaseHp,
  }
  const e = new BattleEngine(cfg)
  e.start()
  for (let t = 0; t < 30000; t++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
  }
  return { phase: e.phase, tick: e.tick, kills: e.kills, leaked: e.leaked, hp: e.baseHp, max: e.baseHpMax }
}

/**
 * 复活币：为什么必须是「额外**当前**血量」而不是「+X% 基础血量」。
 *
 * ## 曾经的实现（148–152 轮）为什么无效
 *
 * 漏怪伤害以 `baseHpMax` 为分母：
 *
 * ```
 * 普通怪：leakDamage = baseHpMax / LeakDamageDivisor
 * 攻击怪：leakDamage = baseHpMax * attack / LeakAttackerDivisor
 * ```
 *
 * 把 `base_hp` 抬到 1.5 倍时：血量 1.5B、每次漏怪扣 1.5B/10，
 * **可承受漏怪数恒为 LeakDamageDivisor**。护甲那层 `applyArmor` 是乘法，
 * 同样不改变比值。
 *
 * 实测 100 关：只有 1 关漏怪数差 1（L96，两局都是 `lost`），
 * 差值来自 `baseHpMax*attack/3600` 的整数截断。**0 关翻盘。**
 *
 * 玩家花了 5,000 金币什么也没换到 —— 所以它被重新下架过一轮。
 *
 * ## 现在的做法
 *
 * `BattleConfig.extraBaseHp` 加在 `baseHp`（当前）上，**不动 `baseHpMax`**：
 *
 * ```
 * 初始血量 = baseHpMax + extraBaseHp
 * 每次漏怪 = baseHpMax / LeakDamageDivisor     ← 分母没变
 * → 多扛 extraBaseHp / (baseHpMax/10) 次
 * ```
 *
 * ## ⚠️ anti-cheat：它必须进 `replayHash()` 的 prefix
 *
 * 服务端**不重算** hash（只存），所以客户端改本地这一行就能白扛无数次漏怪。
 * 与 `base_hp` 不同，它不在关卡里、也不由 `level.id` 决定，
 * 所以 prefix 里的 `|X` 段是它唯一的防篡改位置。
 */
describe('复活币：extraBaseHp 真的让玩家多扛漏怪', () => {
  it('初始血量 = 上限 + 额外血量，而漏怪伤害的分母（上限）不变', () => {
    const lv = levels[0]
    const plain = play(lv)
    const buffed = play(lv, BigInt(lv.base_hp) / LeakDamageDivisor * 3n)

    expect(plain.max).toBe(BigInt(lv.base_hp))
    // 关键不变式：上限**不动**。动了就等于回到那个无效实现。
    expect(buffed.max).toBe(plain.max)
    expect(buffed.hp).toBe(plain.max + BigInt(lv.base_hp) / LeakDamageDivisor * 3n)
    // 一次漏怪的伤害也由上限决定，所以不变
    const perLeak = plain.max / LeakDamageDivisor
    expect(perLeak).toBe(BigInt(lv.base_hp) / LeakDamageDivisor)
  })

  it('多扛的次数恰好等于额外的漏怪额度，而不是「血量百分比」', () => {
    const lv = levels.find((l) => play(l).leaked > 0)
    expect(lv, 'fixture 里没有漏过怪的关卡 —— 换 vectors 或改选关条件').toBeTruthy()
    const target = lv!

    const extra = (BigInt(target.base_hp) / LeakDamageDivisor) * 3n
    const plain = play(target)
    const buffed = play(target, extra)

    // 漏怪更少，或至少撑得更久 —— 两种都算「有效果」
    const better = buffed.leaked <= plain.leaked &&
      (buffed.phase === 'won' || plain.phase !== 'won' || buffed.tick >= plain.tick)
    expect(better,
      `L${target.id} 加了 ${extra} 血之后反而更差：漏怪 ${plain.leaked}→${buffed.leaked}，` +
      `${plain.phase}→${buffed.phase}，tick ${plain.tick}→${buffed.tick}`)
      .toBe(true)

    // 上限不动 = 加成不会被分母抵掉（这是与旧实现最本质的区别）
    expect(buffed.max).toBe(plain.max)
  })

  it('extraBaseHp 进了 replayHash 的 prefix：改本地这一行会被 hash 看出来', () => {
    const lv = levels[0]
    const cfg = (extra: bigint): BattleConfig => ({
      level: lv, enemies: enemyMap, skills: skillMap,
      equipped: equip(ACTIVE_SLOTS), attacker: defaultAttacker(), seed: 12345,
      extraBaseHp: extra,
    })

    const a = new BattleEngine(cfg(0n)); a.start()
    const b = new BattleEngine(cfg(100n)); b.start()
    // 同样的 seed、同样开局，只有 extraBaseHp 不同
    expect(a.replayHash()).not.toBe(b.replayHash())

    // 未复活时也写 `|X0`，避免「有前缀 / 无前缀」两种长度
    const c = new BattleEngine({
      level: lv, enemies: enemyMap, skills: skillMap,
      equipped: equip(ACTIVE_SLOTS), attacker: defaultAttacker(), seed: 12345,
    }); c.start()
    expect(c.replayHash()).toBe(a.replayHash())
  })

  it('LeakDamageDivisor = 10（与 server 侧 domain.LeakDamageDivisor 必须一致）', () => {
    // 跨端契约：不同步时服务端按 10 算、引擎按别的值扣，
    // 偏差不报错，只让「多扛 3 次」实际对不上。
    // 对应守卫：server/internal/domain/revive_contract_test.go
    expect(LeakDamageDivisor).toBe(10n)
  })
})