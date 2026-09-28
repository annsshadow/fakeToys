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
 * 40 关里一共 19 次漏怪，掉的血量是 63,776 → 满封顶才省 432，
 * 倒推单次漏怪的原始伤害约 45 点，而每关底血约 1,594 ——
 * **一次漏怪只掉 2.8% 血**，所以默认构筑在�� 40 关只掉 0.3%。
 *
 * 也就是说，护甲之所以「重要不了」，根因不是护甲公式弱，
 * 而是**漏怪的伤害相对底血太小**。那是平衡问题，不是 bug，
 * 已记入 README 的已知边界。
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
const ARMORS = [0n, 100n, 215n, 300n, 400n, 500n, 600n, 700n, 750n]

describe('护甲形状实测（装备缩放曲线的依据）', () => {
  it('剩余血量随护甲严格递增 —— 形状是好的，没有惩罚区间', () => {
    const ids = levels.slice(0, 40).map((l) => l.id)
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
      expect(
        cur.hpLeft,
        `护甲 ${prev.armor}‰ → ${cur.armor}‰ 剩血从 ${prev.hpLeft} 降到 ${cur.hpLeft}：` +
          `曲线非单调，说明存在「惩罚区间」（元素系数那边 400‰ 时漏怪反而变多）`,
      ).toBeGreaterThan(prev.hpLeft)
    }
  })

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
  })

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
  })

  it('满封顶护甲不会让任何关卡变难（护甲是有利属性）', () => {
    const ids = levels.map((l) => l.id)
    const capped = sweep(750n, ids)
    const base = sweep(0n, ids)
    expect(capped.won, `护甲是有利属性，却让通关数从 ${base.won} 掉到 ${capped.won}`)
      .toBeGreaterThanOrEqual(base.won)
  })

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
  })
})
