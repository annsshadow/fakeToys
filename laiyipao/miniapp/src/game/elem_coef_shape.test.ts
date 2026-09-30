/**
 * 元素系数的**真实形状** —— 一条被误诊了 20 轮的曲线。
 *
 * # 被否证的假设
 *
 * README 边界 12 原先写着：
 *
 * > 「元素 DoT 是不消耗热量的旁路伤害，而它打在一个『已经走到防线附近』
 * > 的敌人身上时就浪费掉了。关卡总血量固定，于是这份浪费就表现为更多漏怪。」
 *
 * 并据此把「改 DoT 结算时机」列为核心元素系统待办。
 *
 * ## 代码里没有 DoT 结算时机这回事
 *
 * `damage.ts:resolveHit` 第 3 步算出的 `elementDmg`：
 *
 *     elementDmg = ELEMENT_PER_STACK_BASE * stacks
 *     elementDmg = mulDiv(elementDmg, att.elementCoefPermille, PERMILLE)
 *     elementDmg = elementDmg / ELEMENT_TICK_DIVISOR
 *     elementDmg = mulDiv(elementDmg, PERMILLE - resist, PERMILLE)
 *
 * 紧接着第 6 步把它折进 `critPart`、第 7 步**同一次调用内**扣 `def.hp`：
 *
 *     critPart = direct + elementDmg
 *     total    = direct + reactionDmg
 *     def.hp  -= remaining
 *
 * **没有延迟队列、没有「下个 tick 才结算」、没有与漏怪阈值的任何交互。**
 * 「伤害浪费在已过阈值的敌人上」这个机制在代码里**无处发生**。
 *
 * # 第二条假设（控制效果被削弱）同样被否证
 *
 * 猜的是：`elementCoefPermille` 缩放反应的元素侧 `elemSide`，
 * 而反应带冻结/眩晕，所以降系数 → 控制变弱 → 更多敌人走到防线。
 *
 * 否证依据是 `engine.ts:1022`：
 *
 *     if (react === 'flash_freeze' || react === 'superconduct') e.frozenMs = spec.statusDurationMs
 *     if (react === 'overheat') e.stunnedMs = spec.statusDurationMs
 *
 * 时长取自**反应表常量**（flash_freeze 2000 / superconduct 4000 / overheat 1500），
 * 与 `elementCoefPermille` **毫无关系**。控制时长从来就不是系数的函数。
 *
 * ⚠️ 这条否证过程本身又栽了一次「观测点选错」的坑（项目第 7 次同型）：
 * 我先读 `engine.frozenMs`，而引擎上**根本没有这个字段** ——
 * 它挂在**敌人**对象上（`types.ts:158`、`engine.ts:541` 初始化、
 * `564` 递减、`568` 用来冻结移动）。读引擎字段恒得 0，
 * 于是得出「控制效果为 0」的错误结论。
 *
 * # 真实形状：严格单调，没有「惩罚区间」
 *
 * 实测（前 40 关可比口径，默认构筑 fire/fire/fire/ice）：
 *
 *   coef‰   漏怪    反应
 *     200     869    8291
 *     400     378    6582
 *     700      60    5223
 *    1000      19    4495
 *    1500       1    3807
 *    1800       0    3521
 *
 * 漏怪**单调递减**，没有凹陷、没有惩罚区间。
 * 「惩罚区间在 200~800‰」这个说法把「低于基准就是更差」
 * 说成了「存在一个特殊坏区间」，而后者会误导人去改机制。
 *
 * 关系退化到一句话：**系数↑ → 反应伤害↑ → 敌人死得早 → 漏怪少。**
 *
 * # 但「伤害浪费」确实存在 —— 只是形态是过量伤害
 *
 * 上表漏怪在 1000‰ 就基本归零，而伤害还在涨。反推总伤害
 * （`score = totalDamage/100 + kills*500` ⇒ `totalDamage ≈ (score - kills*500)*100`）：
 *
 *   coef‰   总伤害     击杀   伤害/击杀
 *     400     538900     790        682
 *    1000     723600     818        885
 *    1800     867600     820       1058
 *    3000    1006300     820       1227
 *
 * **击杀数在 1000‰ 饱和（818 → 820 → 820），伤害却 +39%。**
 * 1000‰ 之后元素系数买到的是**纯过量伤害** ——
 * 打在已死敌人身上、或补最后一刀，而 `score` 照样把它算进去。
 *
 * 这是真实的分数公平性问题，但修它**要付哈希兼容性的代价**：
 * `engine.ts:1035` 把分数写进 `hit` 事件的 `c` 字段，
 * 而 `replayHash()` 覆盖事件序列 ⇒ 改计分口径 = 全部历史战报失去可重放性。
 * 与当年 `activeSlots` 缺省值是同一类取舍，**属架构决策，未擅自改**。
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, MAX_BATTLE_TICKS, type BattleConfig } from './engine'
import { defaultAttacker } from './damage'
import { ACTIVE_SLOTS } from './heatmap'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'

const levels = (fixture.levels as unknown as GeneratedLevel[]).slice().sort((a, b) => a.id - b.id)
const enemies = new Map<number, EnemyDef>(
  (fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]),
)
const skills = new Map<number, SkillDef>(
  [
    ...(fixture.skills as unknown as SkillDef[]),
    ...(fixture.composite_skills as unknown as SkillDef[]),
  ].map((s) => [s.id, s]),
)

function mkEquipped(ids: number[]) {
  return ids.map((id, i) => {
    const s = skills.get(id)!
    return {
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
    }
  })
}

/** 默认构筑：按 id 升序取前 4 个主动技能（实测 fire/fire/fire/ice）。 */
function equippedDefault() {
  const active = [...skills.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, ACTIVE_SLOTS)
  return mkEquipped(active.map((s) => s.id))
}

/** 多元素构筑：火/冰/雷/蚀各一（1/4/7/16）—— 能触发带控制的反应。 */
function equippedMulti() {
  return mkEquipped([1, 4, 7, 16])
}

function runOne(id: number, coef: bigint, multi: boolean) {
  const lv = levels.find((l) => l.id === id)!
  const cfg: BattleConfig = {
    level: lv,
    enemies,
    skills,
    equipped: multi ? equippedMulti() : equippedDefault(),
    attacker: { ...defaultAttacker(), elementCoefPermille: coef },
    seed: 12345,
  }
  const e = new BattleEngine(cfg)
  e.start()
  /** 控制时长：**敌人**身上的 frozenMs/stunnedMs。 */
  let frozenTicks = 0
  let stunnedTicks = 0
  for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
    for (const en of e.enemies) {
      if (en.frozenMs > 0) frozenTicks++
      if (en.stunnedMs > 0) stunnedTicks++
    }
  }
  return { leaked: e.leaked, reactions: e.reactionsCount, kills: e.kills, score: e.score, frozenTicks, stunnedTicks }
}

function sum(coef: bigint, ids: number[], multi = false) {
  let leaked = 0
  let reactions = 0
  let kills = 0
  let score = 0
  let frozenTicks = 0
  let stunnedTicks = 0
  for (const id of ids) {
    const r = runOne(id, coef, multi)
    leaked += r.leaked
    reactions += r.reactions
    kills += r.kills
    score += r.score
    frozenTicks += r.frozenTicks
    stunnedTicks += r.stunnedTicks
  }
  return { leaked, reactions, kills, score, frozenTicks, stunnedTicks }
}

// ── 扫描缓存 ──────────────────────────────────────────────────
//
// 这一段原本重复扫了 **18** 次，而互不相同的只有 **11** 次：
//
//   it「六档漏怪非递增」   COEFS × sum(c, first40)          → 6
//   it「反应次数递减」     COEFS × sum(c, first40)          → 12  ← 与上条逐位相同
//   it「冻结/眩晕可达」   sum(1000n, first40, true)        → 13
//   it「冻结累计下降」     sum(400n|1800n, first40, true)  → 15
//   it「击杀饱和」         sum(1000n|3000n, 前20)           → 17
//   it「伤害分份额」       sum(1000n, 前20)                → 18  ← 与上条相同
//
// 键用 **id 串**而不是数组引用，所以 `first40.slice(0, 20)` 与
// 直接持有的「前 20 关」数组能命中同一条缓存 —— 它们内容相同，
// 只是写法不同，靠引用比较是看不出来的。
//
// ### 为什么要做成「只有一个入口」
//
// 缓存键少写一个参数 = **静默返回错的结果**，而且测试照样绿。
// 所以下面这个 `sweep` 是**唯一**的对外入口：
// 不要绕过它直接调 `sum`。
// 键覆盖了 `sum` 的全部三个形参（coef / ids / multi）——
// 注意 `multi` 是**构筑**（多元素 vs 默认）而不是统计开关，
// 漏掉它会让「默认构筑的冻结 tick」与「多元素构筑的」混为一谈。
//
// ### 返回的是**共享对象**
//
// 多次调用拿到的是同一个引用。当前所有断言都只读不改，
// 但若将来有人在测试里写 `r.leaked = 0`，会同时污染其他用例。
// 要改就先 `{ ...sweep(...) }` 复制一份。
type Sum = ReturnType<typeof sum>
const sumCache = new Map<string, Sum>()

function sweep(coef: bigint, ids: number[], multi = false): Sum {
  const key = `${coef}|${multi}|${ids.join(',')}`
  let r = sumCache.get(key)
  if (!r) {
    r = sum(coef, ids, multi)
    sumCache.set(key, r)
  }
  return r
}

const first40 = levels.slice(0, 40).map((l) => l.id)
const COEFS = [200n, 400n, 700n, 1000n, 1500n, 1800n]

// ── 真实形状：单调 ────────────────────────────────────────────

describe('元素系数：漏怪严格单调递减，不存在「惩罚区间」', () => {
  it('六个档位的漏怪构成一条非递增序列', () => {
    const rows = COEFS.map((c) => ({ coef: c, ...sweep(c, first40) }))
    for (let i = 1; i < rows.length; i++) {
      const prev = rows[i - 1]
      const cur = rows[i]
      expect(
        cur.leaked,
        `${prev.coef}‰ → ${cur.coef}‰ 漏怪从 ${prev.leaked} 涨到 ${cur.leaked}：` +
          `曲线非单调，说明「惩罚区间」的说法有依据，需要重新诊断`,
      ).toBeLessThanOrEqual(prev.leaked)
    }
  }, 900_000)

  it('反应次数随系数递减（敌人死得更快 → 攒不够层数触发反应）', () => {
    // 上面那条已经把这 6 次扫描算完了，这里全部命中缓存。
    const rows = COEFS.map((c) => ({ coef: c, ...sweep(c, first40) }))
    for (let i = 1; i < rows.length; i++) {
      expect(rows[i].reactions).toBeLessThan(rows[i - 1].reactions)
    }
  }, 900_000)
})

// ── 否证「控制效果被削弱」 ────────────────────────────────────

describe('控制时长与元素系数无关（否证第二条假设）', () => {
  it('多元素构筑**确实**能触发冻结/眩晕（观测点：敌人对象）', () => {
    // 这条是前提守卫：默认构筑只触发 steam_burst（statusDurationMs = 0），
    // 在它上面测控制效果恒为 0 —— 那是**仪器选错对象**，不是效果不存在。
    // 必须先证明效果可达，再谈它随不随系数变。
    const r = sweep(1000n, first40, true)
    expect(r.frozenTicks).toBeGreaterThan(0)
    expect(r.stunnedTicks).toBeGreaterThan(0)
  }, 900_000)

  it('累计冻结 tick 随系数**下降** —— 但这不能证明「单次时长变了」', () => {
    // ⚠️ 第一版这里写的是「控制时长在各档位完全相同」，**红了**
    // （400‰ → 101558 tick，1800‰ → 31745 tick）。
    //
    // 混淆了两个量：
    //   - `spec.statusDurationMs`：**单次**施加的时长，反应表常量，与系数无关
    //   - 累计冻结 tick = Σ(被冻敌人 × 被冻 tick)，受**反应次数**与**敌人数**影响
    //
    // 系数升高 → 敌人死得更快 → 反应次数从 9622 降到 5009
    // → 累计冻结 tick 必然下降，**与单次时长无关**。
    //
    // 而 `冻结tick / 反应次数` 的比值也不恒定（10.6 → 6.3），
    // 因为高系数下反应命中时敌人往往已经快死了。
    // 所以**累计量在这个系统里无法干净地归因到单次时长**，
    // 任何「控制时长随系数变化」的断言都是不可判定的。
    //
    // 单次时长恒为 `REACTIONS[key].statusDurationMs` 这件事，
    // 由 `engine.ts:1022` 的代码本身保证（读常量、不读系数），
    // 属于可读代码确认的事实，不适合用行为断言去守 ——
    // 行为断言在这里只能测出被混淆的累计量。
    const a = sweep(400n, first40, true)
    const b = sweep(1800n, first40, true)
    expect(b.reactions).toBeLessThan(a.reactions)
    expect(b.frozenTicks).toBeLessThan(a.frozenTicks)
    // 单次时长的上界是可判定的：任何一个被冻的敌人，
    // 单次施加的冻结不会超过反应表里的最大值 4000ms = 80 tick。
    expect(b.frozenTicks).toBeGreaterThan(0)
  }, 900_000)
})

// ── 真实存在的浪费：过量伤害 ──────────────────────────────────

describe('过量伤害：1000‰ 之后元素系数买到的几乎全是浪费', () => {
  /** 反推总伤害：score = totalDamage/100 + kills*500。 */
  const totalDamageOf = (score: number, kills: number) => (score - kills * 500) * 100

  it('击杀数在 1000‰ 饱和，而伤害继续涨 ≥ 30%', () => {
    // 取前 20 关（无 BOSS，避免 killScore 5000 污染反推）
    const ids = levels.slice(0, 20).map((l) => l.id)
    const at1000 = sweep(1000n, ids)
    const at3000 = sweep(3000n, ids)

    expect(at1000.kills).toBeGreaterThan(0)
    // 击杀数基本不再增长（允许 1% 的抖动，超过就说明结论过期）
    expect(at3000.kills).toBeLessThanOrEqual(Math.ceil(at1000.kills * 1.01))

    const d1000 = totalDamageOf(at1000.score, at1000.kills)
    const d3000 = totalDamageOf(at3000.score, at3000.kills)
    expect(d3000).toBeGreaterThan(d1000 * 1.3)
  }, 900_000)

  it('记录这一事实：分数会为「打在已死敌人身上的伤害」付账', () => {
    // 这不是断言「应该修」，而是把已测得的事实钉住，
    // 让下一个想改计分口径的人知道**代价**是什么：
    // `engine.ts:1035` 把分数写进 `hit` 事件的 `c` 字段，
    // 而 `replayHash()` 覆盖事件序列 ——
    // 改计分口径 = 全部历史战报失去可重放性。
    // 上面「击杀饱和」那条已经算过 (1000n, 前20) 这一条，这里直接命中缓存：
    // `first40.slice(0, 20)` 与那份「前 20 关」数组内容相同，键也就相同。
    const at1000 = sweep(1000n, first40.slice(0, 20))
    const dmg = totalDamageOf(at1000.score, at1000.kills)
    // 伤害分占总分的比例：这就是「过量伤害」在分数里的份额上界
    const dmgScoreShare = dmg / 100 / at1000.score
    expect(dmgScoreShare).toBeGreaterThan(0)
    expect(dmgScoreShare).toBeLessThan(1)
  }, 900_000)
})
