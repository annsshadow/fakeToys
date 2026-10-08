import { describe, it, expect } from 'vitest'
import { Terrain, toFixed, type TerrainContext } from './terrain'
import { applyArmor } from './fixed'
import type { Enemy } from './types'
import type { Element } from './elements'

/**
 * 油桶火区的伤害必须走**标准管线**（第 77 轮）。
 *
 * # 缺陷：火区同时绕过了护甲与 elementCap
 *
 * 原实现：
 *
 * ```ts
 * for (const e of ctx.enemiesInRadius(this.x, this.y, 90)) {
 *   e.hp -= ctx.terrainTick          // ← 裸减，不过护甲
 *   e.hitFlashMs = 120
 *   const cur = e.stacks.get('fire') ?? 0n
 *   e.stacks.set('fire', cur < 4n ? cur + 1n : cur)   // ← 上限写死 4n
 * }
 * ```
 *
 * ## 一、护甲被完全绕过
 *
 * 引擎里敌人是按 `new Defender(e.hp, e.shield, e.armorPermille)`
 * 处理的（engine.ts:1283），所以**每一次普通命中都过护甲**。
 * 火区直接 `e.hp -=`，等于给敌人开了一条「无视护甲」通道 ——
 * 高护甲敌人（levelgen 各章 `ArmorPermille` 200~250‰）
 * 在火区里承受的**相对**伤害比在别处高得多。
 *
 * ## 二、元素层数上限写死 4n
 *
 * `applyElement` 的注释（types.ts:149）写着：
 *
 * > 施加元素层数时按 elementCap 夹紧（terrain.ts 里的油桶不能直接改元素栈）
 *
 * **而油桶正是直接改元素栈的那一处。** 类型注释明明白白指名了它，
 * 它却没照做。
 *
 * 后果与 `element_cap` 卡（轮 72 的单位错误）同型：
 * 上限 < 4 的构筑（默认 3）能白拿 1 层，
 * 而上限 > 4 的构筑（后期章节 + 「元素容器」卡）拿不到任何收益。
 *
 * # 为什么判据是「相对护甲单调」而不是「绝对值」
 *
 * 本文件不含 `resolveHit`（它要 attacker/defender/hit 三件套）。
 * 而绝对值断言会依赖 `terrainTick` 的具体取值 ——
 * 那是**夹具的巧合性质**，不是契约。
 *
 * 更重要的是：修好之后同一个夹具下
 * 「0 护甲 → 减伤 = 全额、750‰ 护甲 → 减到 1/4」这个**关系**成立，
 * 而它在修复前不成立（全是全额）。
 * **判据要落在「关系」上，那才是契约。**
 */

/** 造一个可控的假敌人（字段齐备，与真实 Enemy 同形）。 */
function fakeEnemy(armorPermille: bigint, hp = 10_000n): Enemy & { stacks: Map<Element, bigint> } {
  const stacks = new Map<Element, bigint>()
  return {
    uid: 1,
    defId: 1,
    name: 'probe',
    category: 'normal',
    x: toFixed(500),
    y: toFixed(500),
    hp,
    maxHp: hp,
    shield: 0n,
    armorPermille,
    flyHeight: 0,
    burrow: false,
    isBoss: false,
    resist: new Map(),
    stacks,
    applyElement(e: Element, add: bigint, cap: bigint) {
      const cur = stacks.get(e) ?? 0n
      if (add <= 0n) return
      stacks.set(e, cur < cap ? cur + add : cur)
    },
    speed: 100n,
    attack: 0n,
    attackRange: 0n,
    attackInterval: 0,
    attackCooldown: 0,
    frozenMs: 0,
    stunnedMs: 0,
    slowedMs: 0,
    amplifyPermille: 0n,
    knockback: 0n,
  } as unknown as Enemy & { stacks: Map<Element, bigint> }
}

/** 点燃一个油桶并跑一 tick，返回被烧的敌人。 */
function burnOnce(
  armorPermille: bigint,
  elementCap: bigint,
  terrainTick = 1000n,
  ticks = 1,
): { enemy: Enemy & { stacks: Map<Element, bigint> }; before: bigint } {
  const t = new Terrain({ kind: 'oil_drum', x: 500, y: 500, param: 4 } as never)
  t.onHit('fire', 500n, toFixed(500), toFixed(500))
  expect(t.state, '油桶应当已被点燃 —— 若没点燃，本用例什么也没测到').toBe('burning')

  const enemy = fakeEnemy(armorPermille)
  const before = enemy.hp
  const ctx: TerrainContext = {
    enemies: [enemy],
    projectiles: [],
    terrainTick,
    elementCap,
    within: () => false,
    enemiesInRadius: () => [enemy],
    onKill: () => {},
    onTerrainTrigger: () => {},
  }
  // ⚠️ `ticks` 是必需的：每 tick 只叠 1 层。
  //
  // 我第一版只跑 1 tick 却断言「火层应等于 cap」—— 而敌人初始是 0 层，
  // 所以实测 1 ≠ 3。那不是 bug，是**我没数 tick**。
  //
  // 这与 README 记的「守卫的夹具温和度」同族：
  // 断言必须落在「跑够次数之后」的那一层。
  for (let k = 0; k < ticks; k++) t.update(50, ctx)
  return { enemy, before }
}

describe('油桶火区的伤害管线（第 77 轮）', () => {
  it('零护甲时全额扣除（与修复前的行为一致 —— 这条不许变）', () => {
    const { enemy, before } = burnOnce(0n, 3n, 1000n)
    expect(before - enemy.hp, '零护甲应当扣满 terrainTick').toBe(1000n)
  })

  it('护甲真的生效：750‰ 护甲下只扣 1/4', () => {
    // ⚠️ 判据用**同一个** applyArmor 算期望值 —— 那是契约的锚点。
    // 不写死 250n 是因为 terrainTick 是夹具选的值；
    // 而「火区与 resolveHit 用同一个护甲函数」才是要守的东西。
    const tick = 1000n
    const { enemy, before } = burnOnce(750n, 3n, tick)
    expect(before - enemy.hp, '750‰ 护甲下应当只扣 1/4').toBe(
      applyArmor(tick, 750n),
    )
    expect(before - enemy.hp).toBe(250n)
  })

  it('护甲单调：护甲越高扣得越少（修复前是恒定值）', () => {
    const tick = 1000n
    const damages = [0n, 250n, 500n, 750n].map(
      (armor) => {
        const { enemy, before } = burnOnce(armor, 3n, tick)
        return before - enemy.hp
      },
    )
    for (let i = 1; i < damages.length; i++) {
      expect(
        damages[i]! < damages[i - 1]!,
        `护甲递增时扣血应递减，实测 ${damages.map(String).join(' >= ')}`,
      ).toBe(true)
    }
  })

  it('护甲封顶 75%（超出的部分不再减伤）', () => {
    const tick = 1000n
    const capped = burnOnce(999_999n, 3n, tick)
    const at750 = burnOnce(750n, 3n, tick)
    expect(capped.before - capped.enemy.hp).toBe(at750.before - at750.enemy.hp)
  })

  it('元素层数按 ctx.elementCap 夹，而不是写死的 4', () => {
    // 跑 8 tick（远超过任何 cap），看它停在哪。
    //
    // 旧写法 `cur < 4n ? cur + 1n : cur` 无论 cap 是多少都停在 4：
    //   cap=3 → 停在 4（越过上限 1 层）
    //   cap=10 → 停在 4（比上限少 6 层）
    const at3 = burnOnce(0n, 3n, 1000n, 8)
    expect(at3.enemy.stacks.get('fire'), 'cap=3 时火层应停在 3').toBe(3n)

    const at10 = burnOnce(0n, 10n, 1000n, 8)
    expect(at10.enemy.stacks.get('fire'), 'cap=10 时 8 tick 应到 8 层').toBe(8n)

    // ⚠️ 关键区分：修复前两者**都**是 4。
    // 只断言「≤ cap」抓不到上限 >4 的那一半（4 ≤ 10 也成立），
    // 所以必须断言一个**大于 4** 的 cap 下的确切层数。
    expect(at10.enemy.stacks.get('fire')!).toBeGreaterThan(4n)
  })

  it('cap=1 与 cap=0 都必须被尊重', () => {
    // cap=0：关卡 element_cap 为 0 的极端情形 —— 一层都不该上。
    const z = burnOnce(0n, 0n, 1000n, 8)
    expect(z.enemy.stacks.get('fire') ?? 0n, 'cap=0 时不该有火层').toBe(0n)

    const o = burnOnce(0n, 1n, 1000n, 8)
    expect(o.enemy.stacks.get('fire'), 'cap=1 时应停在 1').toBe(1n)
  })
})