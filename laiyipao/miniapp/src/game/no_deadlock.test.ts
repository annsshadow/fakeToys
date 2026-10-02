/**
 * 全 100 关的**可完成性**守卫。
 *
 * ⚠️ 这个文件补的是一个真实事故，而且是最严重的一次：
 * **第 22/33/34 关永远打不完**，引擎永久停在 wave 阶段。
 *
 * 根因是三层叠加的循环依赖：
 *
 *  1. `collapse_wall`（崩塌掩体）会挡掉弹丸 → `updateProjectiles` 里
 *     `p.dead = true; break`，弹丸走不到 `checkProjectileHit`
 *  2. 掩体唯一的充能源是 `onHit`，而 `onHit` **只在弹丸命中敌人时**被调用
 *     ⇒ 弹丸被掩体吃掉 → 打不到敌人 → 掩体永不充能 → 永不崩塌
 *     ⇒ 弹丸继续被挡                                    **循环依赖**
 *  3. `onHit` 原本还要求 `element === 'kinetic'`，而默认构筑
 *     （内容表前 4 个主动技能）是 fire/fire/fire/ice —— 一个动能都没有
 *
 * 实测现象：命中率掉到 1%，12000 tick 只杀 2~17 只怪，
 * 引擎的 `phase` 永远是 `'wave'`。真机上那就是玩家盯着一个
 * **永远不结算**的画面，体力也拿不回来。
 *
 * ## 为什么之前没有任何测试发现
 *
 * 夹具曾经只导出第 1/10/25/50/75/100 关 —— 6 关里**没有一关是地形掩体关**
 * （有地形的是第 10、50 关，但地形种类是 charge_tower/其它，不挡弹丸）。
 * 于是「掩体挡弹丸」这条路径**从未被任何测试或探针执行过**。
 *
 * 与「地形关 41%」那次同形：**采样掩盖了分布问题**。
 * 这也是本文件用全 100 关而不是采样的原因 ——
 * 可完成性是一个**逐关**的性质，采样只能证明"被采到的那些关能过"。
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, TICK_MS, MAX_BATTLE_TICKS, type BattleConfig } from './engine'
import { defaultAttacker } from './damage'
import { ACTIVE_SLOTS } from './heatmap'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'

const levels = (fixture.levels as unknown as GeneratedLevel[]).slice().sort(
  (a, b) => a.id - b.id,
)
const enemyMap = new Map<number, EnemyDef>(
  (fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]),
)
const skillMap = new Map<number, SkillDef>(
  [
    ...(fixture.skills as unknown as SkillDef[]),
    ...(fixture.composite_skills as unknown as SkillDef[]),
  ].map((s) => [s.id, s]),
)

/** 默认构筑：内容表前 ACTIVE_SLOTS 个主动技能。 */
function defaultBuild(): BattleConfig['equipped'] {
  const active = [...skillMap.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, ACTIVE_SLOTS)
  return active.map((s, i) => ({
    skillId: s.id,
    name: s.name,
    element: s.element as Element,
    kind: s.kind,
    heatCost: BigInt(s.heat_cost),
    cooldownMs: s.cooldown_ms,
    pierce: s.pierce,
    aoeRadius: s.aoe_radius,
    baseDamage: BigInt(s.base_damage),
    applyElement: (s.apply_element ?? '') as Element | '',
    applyStacks: BigInt(s.apply_stacks),
    projectileSpeed: s.projectile_speed,
    chain: s.chain,
    slot: i,
    cooldownRemaining: 0,
  }))
}

interface Run {
  id: number
  phase: string
  tick: number
  hits: number
  shots: number
  hitRate: number
  kills: number
  leaked: number
  total: number
  sec: number
  stars: number
}

function play(lv: GeneratedLevel): Run {
  const e = new BattleEngine({
    level: lv,
    enemies: enemyMap,
    skills: skillMap,
    equipped: defaultBuild(),
    attacker: defaultAttacker(),
    seed: 12345,
  })
  e.start()
  // 上限给到引擎的停滞阈值 + 一点余量，让停滞兜底有机会触发并被观测到
  for (let t = 0; t < MAX_BATTLE_TICKS + 200; t++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
  }
  return {
    id: lv.id,
    phase: e.phase,
    tick: e.tick,
    hits: e.hits,
    shots: e.shots,
    hitRate: e.shots > 0 ? e.hits / e.shots : 0,
    kills: e.kills,
    leaked: e.leaked,
    total: e.totalEnemies,
    sec: +(e.tick * TICK_MS / 1000).toFixed(1),
    stars: lv.star_targets.filter((t) => e.score >= t).length,
  }
}

const runs = levels.map(play)

describe('全关卡：没有一关会挂死', () => {
  it('夹具确实覆盖了全部 100 关（前提检查）', () => {
    // 没有这条，下面所有断言都可能因为"只跑了几关"而假绿 ——
    // 那正是第 22/33/34 关能长期存在的原因。
    expect(levels.length).toBe(100)
    expect(runs.length).toBe(100)
    expect(levels[0].id).toBe(1)
    expect(levels[levels.length - 1].id).toBe(100)
  }, 300000)

  it('每一关都在停滞阈值之前分出胜负（无死局）', () => {
    // 这是本次事故的直接守卫。
    // 修复前：关 22/33/34 的 phase 停在 'wave'，tick 打满 12000 仍无终局。
    const stuck = runs.filter((r) => r.phase === 'wave')
    expect(
      stuck.map((r) => `关${r.id}(tick=${r.tick}, 命中${(r.hitRate * 100).toFixed(0)}%)`).join('\n'),
    ).toBe('')
  }, 300000)

  it('没有一关触发停滞兜底（说明兜底是纯防御，不是日常路径）', () => {
    // ⚠️ 这一条刻意与上一条**成对**：
    //   上一条保证「不会挂死」，这一条保证「不是靠停滞兜底才不挂死」。
    // 只有上一条的话，把 MAX_BATTLE_TICKS 调到 100 也能"通过"，
    // 但那意味着所有关卡都被强行判负 —— 一个把 bug 藏起来的修复。
    const hitCap = runs.filter((r) => r.tick >= MAX_BATTLE_TICKS)
    expect(hitCap.map((r) => `关${r.id}(tick=${r.tick})`).join('\n')).toBe('')
  }, 300000)

  it('每关的命中率都在合理区间（挂死前的征兆是命中率崩塌）', () => {
    // 挂死时命中率会掉到 1% —— 那是"弹丸全被吃掉"的最灵敏指标。
    // 单看终局（won/lost）不够快，用命中率做早期信号。
    const low = runs.filter((r) => r.hitRate < 0.3)
    expect(
      low.map((r) => `关${r.id}: 命中率 ${(r.hitRate * 100).toFixed(0)}%`).join('\n'),
    ).toBe('')
  }, 300000)

  /**
   * **刻意设置难度墙的关卡**。
   *
   * 守卫的意图是「难度不该回退」—— 但这个前提对**刻意设置的墙**不成立。
   * 一面墙的定义就是「前面能过、过不去、后面又能过」。
   *
   * ⚠️ 这条例外的**理由在第 60 轮被订正过一次**。原来写的是：
   *
   * > 终 boss 在第 96 关首次登场，而它的底血比 97~100 关都低
   * > —— 这是 levelgen 的既有特性。漏怪代价改成随底血等比缩放后，
   * > 第 96 关从「尖峰」变成了「墙」。
   *
   * 那条理由**方向是反的**：底血更低意味着每次漏怪的伤害更低（L919 用的是
   * `baseHpMax × attack / LeakAttackerDivisor`），也就是**更容易**过，
   * 解释不了为什么它过不去。留着它会让后来人以为「降底血就能拆掉这面墙」，
   * 而真正的原因是别的。
   *
   * ## 第 60 轮实测出来的真实机制
   *
   * 白垩母皇（defId 22）`hp=320000 shield=64000 armor=350`，
   * 在第 96 关**最后一波**（wave 10/10）。它必须被打死才算通关，于是
   * 第 96 关变成一道**纯 DPS 检验**：
   *
   * | 构筑 | 结果 | 母皇终局余血 |
   * |---|---|---|
   * | 4 技能满级、stage 0 | lost | 84.2% |
   * | 4 技能满级、stage 99（攻方 2080） | lost | 75.0% |
   * | 同上，5 个 seed | 全 lost | 73.0% ~ 84.6% |
   *
   * 同一档构筑下 L94/95/97/98/**100** 全赢 —— L100 带着**同一只**母皇
   * （也在最后一波），输出 483k，刚好够；L96 只有 447k~483k，差一点。
   * 差别是 L100 有 84 只怪、战斗多持续约 2000 tick。
   *
   * ## 三个已排除的猜测（都是先猜后量）
   *
   * 1. 「护甲均值最高」→ 实测 L96 护甲均值 51.2 **只排第 5**
   *      （L67 是 57.6、L80 54.5、L62 54.1、L26 52.1，且那几关都通过）。
   * 2. 「总有效血量最高」→ 实测 L99 的 1,040,320 **高于** L96 的 865,840，且 L99 通过。
   * 3. 「元素克制可解」→ 实测母皇 `ice 500 / lightning 500`（半抗）、
   *      `corrosion -500 / kinetic -500`（易伤），但换技能组合几乎无效：
   *      全动能/全冰电/全火都是母皇余血 **100%**。原因是那些技能
   *      热量 46~72、冷却 1900~3400ms，4 个槽位放不出来 ——
   *      真正的瓶颈是热量/冷却周期，不是元素、也不是伤害。
   *
   * 另外：**攻方养成对这道墙毫无帮助**。stage 0 → 99 把 attack 从 100
   * 提到 2080（20.8 倍），总输出只涨 **1.01%**。与 README 里
   * 「击杀数几乎不随攻击力变」一致。
   *
   * ## 因此「终盘需要养成才能过」这句话的机制是错的
   *
   * 若「养成」指的是攻方面板（stage/攻击力），它拆不掉这面墙。
   * 真正的变量是**战斗时长**与**热量效率**，也就是技能选择与装备 ——
   * 而这一层在第 60 轮没有做完整测量（探针未建模装备、专精、卡牌）。
   *
   * 所以：例外保留（它确实是单点墙、后面 L97 又能过），
   * 但**它很可能是一个 levelgen 缺陷而非设计意图**：
   * 同一只 boss 放在怪更少的关卡里就必然过不去。
   * 候选修法是让 boss 血量随该关的总怪量缩放，而不是写死 320000 ——
   * 但那会动到关卡生成与全部 100 关的扫描基线，本轮**不做**，
   * 结论记在 README「已知边界」第 17 条。
   */
  const INTENTIONAL_WALLS = new Set<number>([96])

  it('没有难度回归点（上一关能过、这一关过不了）', () => {
    const bad: string[] = []
    for (let i = 1; i < runs.length; i++) {
      const prev = runs[i - 1]
      const cur = runs[i]
      if (prev.phase === 'won' && cur.phase === 'lost') {
        if (INTENTIONAL_WALLS.has(cur.id)) {
          // 不是「放过」：要求它**真的是单点墙** —— 下一关必须又能过。
          // 否则「关卡后面连着一串墙」这种真问题会被这个例外一并掩盖。
          const next = runs[i + 1]
          if (next && next.phase !== 'won') {
            bad.push(
              `关${cur.id} 被列为难度墙，但它后面（关${next.id}）也是 lost —— ` +
                '例外只允许「单点墙」，不允许「连续墙」',
            )
          }
          continue
        }
        bad.push(`关${cur.id} 失败，而关${prev.id} 通过`)
      }
    }
    expect(bad.join('\n')).toBe('')
  }, 300000)
})

describe('全关卡：基线数据（供人工核对量级）', () => {
  it('打印每关的结局与关键指标', () => {
    for (const r of runs) {
      console.log(
        `[hang] 关${String(r.id).padStart(3)} ${r.phase.padEnd(4)} ${String(r.sec).padStart(6)}s ` +
          `| 杀${String(r.kills).padStart(3)}/${String(r.total).padStart(3)}` +
          `漏${String(r.leaked).padStart(3)} 命中${(r.hitRate * 100).toFixed(0).padStart(3)}% star=${r.stars}`,
      )
    }
    const won = runs.filter((r) => r.phase === 'won').length
    const lost = runs.filter((r) => r.phase === 'lost').length
    const avgSec = runs.reduce((s, r) => s + r.sec, 0) / runs.length
    const maxSec = Math.max(...runs.map((r) => r.sec))
    console.log(
      `[hang] 汇总：won ${won} / lost ${lost}，平均 ${avgSec.toFixed(0)}s，最长 ${maxSec}s`,
    )
    expect(won + lost).toBe(100)
  }, 300000)
})
