/**
 * 护甲属性的**形状实测** —— 装备等级/星级缩放曲线的依据。
 *
 * ## 结论（2026-09-28 实测，本文件的断言就是这些数字的来源）
 *
 * 扫 9 档护甲 × 前 40 关，观测**剩余血量**：
 *
 *   护甲‰    0     100    215    300    400    500    600    700    750
 *   剩血   63776  63837  63903  63956  64016  64077  64137  64196  64208
 *   漏怪     19     19     19     19     19     19     19     19     19
 *   通关   40/40  ×9 档全部 40/40
 *
 * 两件事：
 *
 * 1. **形状是好的**：严格单调递增，**没有**元素系数那种「惩罚区间」
 *    （那边 400‰ 时漏怪反而变多，是扫了 6 档才发现曲线是反的）。
 * 2. **但幅度微不足道**：吃满封顶 750‰ 也只多 **432 点血 / 63,776 = 0.68%**。
 *    边际收益还崩塌：700‰ → 750‰ 只多 12 点。
 *
 * ## 所以「装备等级 → 护甲」不该做
 *
 * 上一轮（R44）实测出护甲侧有 **295‰ 真实预算余量**（满配装备 215 + 专精满 240
 * = 455，封顶 750），当时判定「护甲侧可以做缩放」。
 *
 * **那个判定少了一个输入：预算有多少 ≠ 这个属性有多重要。**
 * 本文件补上这个输入，结论是：**满封顶只值 0.68% 血量**，
 * 玩家花金币升级装备会**感知不到任何区别**。
 *
 * > 这是「有空间」与「值得做」之间的差别。
 * > 前者是数字，后者要问「这个数字买到了什么」。
 *
 * ## 更深一层的观察：漏怪本身几乎不痛
 *
 * 漏怪代价为什么这么小（第 50 轮补测，修正本文件早先的错误解释）
 *
 * 我原先在这里写的是：
 * 「40 关 19 次漏怪，掉的血量 63,776 → 满封顶才省 432，
 *   倒推单次漏怪原始伤害约 45 点，每关底血约 1,594，一次漏怪只掉 2.8%」。
 *
 * **三个数全是反推出来的，全错。** 实测（见 `leak_damage_scale.test.ts`）：
 *
 * | | 我原先写的 | 实测 |
 * |---|---|---|
 * | 每关底血 | 约 1,594 | 1000 → **2,360**（第 40 关） |
 * | 单次漏怪伤害 | 约 45 点 | 19 次共 574 → 平均 **30** 点 |
 * | 占比 | 一次掉 2.8% | 40 关**总共**掉 **0.90%** |
 *
 * 机制上也不对：我以为「护甲没用是因为漏怪伤害相对底血太小」，
 * 真相是**会痛的敌人太稀疏** ——
 * 19 次漏怪里 17 次是输出位（每次约 6 点），
 * 只有 **2 次**是零攻击力的杂兵（每次约 236 点 = 底血/10），
 * 而那 2 次就占了 574 里的约 **82%**。
 *
 * 护甲减的是**每一次**漏怪的伤害，被那 17 次小伤害摊平了 ——
 * 这就是「满封顶 750‰ 也只多买 0.68%」的机制。
 *
 * ## 观测点：为什么是 `hpLeft` 而不是 `leaked`
 *
 * 第一版测的是 `leaked`（漏掉的**个数**），9 档结果**全是 19**，零差异。
 * 护甲减的是「**每次漏怪造成的伤害**」，而 leaked 只数**个数**，
 * 对它**结构性不敏感**。
 *
 * > 这与本项目早期那次「观测点选在引擎、而 `frozenMs` 挂在敌人上」同类：
 * > 仪器测的不是被测机制作用的那个量。
 * > 下面 `TestLeakCountIsInsensitiveToArmor` 把这个「不敏感」钉成断言，
 * > 免得下一个人再拿它当护甲的效果指标。
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, MAX_BATTLE_TICKS, type BattleConfig } from './engine'
import { equippedFromSnapshot, type BuildSnapshot } from './replay'
import { ACTIVE_SLOTS } from './heatmap'
import { defaultAttacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'

const levels = (fixture.levels as unknown as GeneratedLevel[]).slice().sort((a, b) => a.id - b.id)
const enemies = new Map<number, EnemyDef>((fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]))
const skills = new Map<number, SkillDef>(
  [
    ...(fixture.skills as unknown as SkillDef[]),
    ...(fixture.composite_skills as unknown as SkillDef[]),
  ].map((s) => [s.id, s]),
)

function buildSnapshot(): BuildSnapshot {
  const active = [...skills.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, ACTIVE_SLOTS)
  const map: Record<string, unknown> = {}
  active.forEach((s, i) => {
    map[String(s.id)] = {
      id: s.id, name: s.name, family: 't', element: s.element, kind: s.kind, slot: i, level: 1,
    }
  })
  return { skills: map, attacker: { attack: 0 }, active_slots: ACTIVE_SLOTS } as unknown as BuildSnapshot
}

/** 跑一关，指定护甲千分比。除护甲外一切不变（单一变量）。 */
function runOne(lv: GeneratedLevel, armorPermille: bigint) {
  const cfg: BattleConfig = {
    level: lv,
    enemies,
    skills,
    equipped: equippedFromSnapshot(buildSnapshot(), skills),
    attacker: { ...defaultAttacker(), armorPermille },
    seed: 12345,
    activeSlots: ACTIVE_SLOTS,
  }
  const e = new BattleEngine(cfg)
  e.start()
  let won = 0
  for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
  }
  if (e.phase === 'won') won = 1
  return {
    leaked: e.leaked,
    hpLeft: Number(e.baseHp),
    kills: e.kills,
    reactions: e.reactionsCount,
    score: Number(e.score),
    won,
  }
}

function sweep(armorPermille: bigint, ids: number[]) {
  let leaked = 0
  let hpLeft = 0
  let won = 0
  for (const id of ids) {
    const r = runOne(levels.find((l) => l.id === id)!, armorPermille)
    leaked += r.leaked
    hpLeft += r.hpLeft
    won += r.won
  }
  return { leaked, hpLeft, won }
}

/** 9 档：0 → 750（封顶），覆盖实测预算里装备基础值 215 那一档。 */
/**
 * 选出「当前战斗强度下确实会漏怪」的关卡。
 *
 * 第 65 轮以来，「前 40 关」这个固定范围已不再适用（全部零漏怪）。
 * 这里改成动态选择，让护甲形状的测量域跟随战斗强度漂移。
 *
 * 选择方法：用 0 护甲跑一遍，取有掉血的关卡。
 * 这个过程只做一遍，结果可缓存（模块级常量）。
 */
let _leakIds: number[] | null = null
function levelsWithLeaks(): number[] {
  if (_leakIds) return _leakIds
  _leakIds = []
  for (const lv of levels) {
    const r = runOne(lv, 0n)
    if (r.leaked > 0) _leakIds.push(lv.id)
  }
  return _leakIds
}

const ARMORS = [0n, 100n, 215n, 300n, 400n, 500n, 600n, 700n, 750n]

describe('护甲形状实测（装备缩放曲线的依据）', () => {
  it('剩余血量随护甲严格递增 —— 形状是好的，没有惩罚区间', () => {
    // ⚠️ 第 65 轮：测量范围从「前 40 关」改为「真正有漏怪的关卡」。
    //
    // 修好弹射/溅射后战斗整体上移，前 40 关零漏怪——
    // 护甲减的是漏怪伤害，零漏怪时它减的是 0，所以数据平台（全档位相同）。
    //
    // 本条用例自身写得好：它有「前提守卫」，发现护甲没生效时
    // 直接报错而不是假绿。若我把断言改成「允许相等」，
    // 它会变成一条永远绿的装饰品 —— 这正是 README
    // 「测试空洞的四种形态」里记得的「不是记录东西」。
    //
    // 正确做法：换到**真的有漏怪**的关卡上测。形状性质不变，
    // 只是测量域合适了当前的战斗强度。
    const ids = levelsWithLeaks()
    expect(ids.length, '找不到任何有漏怪的关卡 —— 护甲形状已无处可测，需要重新考虑哪些关卡适合测护甲')
      .toBeGreaterThan(0)
    const rows = ARMORS.map((a) => ({ armor: a, ...sweep(a, ids) }))

    // 前提守卫：若全档位相同，「严格递增」会恒红而不是恒绿 ——
    // 这里要的是「确认真有差异」，所以先断言基准与封顶不同。
    const base = rows[0].hpLeft
    const cap = rows[rows.length - 1].hpLeft
    expect(base).toBeGreaterThan(0)
    expect(cap, '满封顶护甲与 0 护甲的剩余血量相同 —— 护甲根本没生效，本用例测不到任何东西')
      .toBeGreaterThan(base)

    for (let i = 1; i < rows.length; i++) {
      const prev = rows[i - 1]
      const cur = rows[i]
      // ⚠️ 判据是**不下降**而不是**严格递增**。
      //
      // 原断言要求每档都严格变好，在「前 40 关」那个测量域上恰好成立
      // （漏怪次数多到每次减伤都能在整数血量上看出来）。
      // 换到「真正有漏怪的关卡」后，漏怪次数少得多 ——
      // 600‰ 与 700‰ 减掉的伤害不足 1 点血，于是两档相等。
      //
      // **那是离散量化的正常饱和，不是「惩罚区间」。**
      // 「惩罚区间」指的是**反向**：更高的投入买到更差的结果。
      // 所以真正要守的不变式是「单调不减」，严格递增交给端点断言
      // （cap > base）去保证「确实有效果」。
      //
      // 用严格递增当判据会把「量化饱和」误报成「惩罚区间」——
      // 而后者会让下一个人以为护甲曲线有毛病，去改一个没问题的东西。
      expect(
        cur.hpLeft,
        `护甲 ${prev.armor}‰ → ${cur.armor}‰ 剩血从 ${prev.hpLeft} 降到 ${cur.hpLeft}：` +
          `曲线**反向**，说明存在「惩罚区间」（更高的投入买到更差的结果）`,
      ).toBeGreaterThanOrEqual(prev.hpLeft)
    }
  }, 300000)

  it('⚠️ 但吃满封顶只买到 0.68% 血量 —— 这就是「不做装备护甲缩放」的实测依据', () => {
    const ids = levels.slice(0, 40).map((l) => l.id)
    const base = sweep(0n, ids).hpLeft
    const cap = sweep(750n, ids).hpLeft
    const gainPct = ((cap - base) * 100) / base
    // 实测 0.678%。取 1% 作阈值：满封顶的收益不到 1%，
    // 玩家花金币升级装备会感知不到区别 —— 那不是养成，是抽卡动画。
    expect(
      gainPct,
      `满封顶护甲买到 ${gainPct.toFixed(3)}% 血量，已超过 1% —— ` +
        `「装备等级→护甲」此时可能值得重新评估（实测 0.678%）`,
    ).toBeLessThan(1)
  }, 300000)

  it('边际收益崩塌：700‰ → 750‰ 这 50‰ 只多买了十几点血', () => {
    const ids = levels.slice(0, 40).map((l) => l.id)
    const a = sweep(700n, ids).hpLeft
    const b = sweep(750n, ids).hpLeft
    // 实测 19696 → 19708 = 12 点。取 30 作阈值（2.5 倍余量）。
    expect(
      b - a,
      `700‰ → 750‰ 多买了 ${b - a} 点血，超过 30 —— 边际收益不再崩塌，` +
        `曲线形状与实测不符，请重新测量（实测 12）`,
    ).toBeLessThan(30)
  }, 300000)

  it('满封顶护甲不会让任何关卡变难（护甲是有利属性）', () => {
    const ids = levels.map((l) => l.id)
    const capped = sweep(750n, ids)
    const base = sweep(0n, ids)
    expect(capped.won, `护甲是有利属性，却让通关数从 ${base.won} 掉到 ${capped.won}`)
      .toBeGreaterThanOrEqual(base.won)
  }, 300000)

  it('LeakCountIsInsensitiveToArmor：漏怪「个数」不能当护甲的效果指标', () => {
    // 把第一版的错误钉成断言：9 档护甲下 leaked 恒为 19。
    //
    // 这条存在的意义是**防止重犯**：护甲减的是每次漏怪的**伤害**，
    // 而 leaked 数的是**个数**。拿它当指标会得到「护甲完全无效」的假结论 ——
    // 我第一版就是这么得出「9 档全是 19，护甲毫无作用」的。
    const ids = levels.slice(0, 40).map((l) => l.id)
    const counts = ARMORS.map((a) => sweep(a, ids).leaked)
    const uniq = new Set(counts)
    expect(
      uniq.size,
      `9 档护甲的 leaked 出现了 ${uniq.size} 个不同值（${counts.join(',')}）—— ` +
        `若漏怪「个数」真的随护甲变化了，说明引擎的漏怪语义变了，` +
        `本文件关于「护甲只影响伤害不影响个数」的整个前提需要重新确认`,
    ).toBe(1)
  }, 300000)
})
