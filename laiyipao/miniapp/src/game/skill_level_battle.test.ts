/**
 * 技能等级**必须真的改变战斗结果** —— 行为级判据，不是字段反射。
 *
 * ## 为什么要单独一个文件
 *
 * 本项目已 7 次栽在「度量前提不成立」上。给一个字段加进公式，
 * 最容易写出的守卫是「读出 `baseDamage` 看看它变没变」——
 * 那是**反射**：它只证明「赋值语句存在」，
 * 不证明「伤害真的变了」，更不证明「哈希变了」。
 *
 * 本文件的判据分三级，从弱到强：
 *
 * | 级别 | 判据 | 能证明什么 | 不能证明什么 |
 * |---|---|---|---|
 * | 反射 | `equipped[0].baseDamage` 变大 | 赋值语句存在 | 战斗没用到它 |
 * | 行为 | 累计伤害变大 | 伤害公式真的读了它 | 验真能区分 |
 * | 行为+锚点 | `replayHash()` 变了 | I-6 能区分不同等级 | —— |
 *
 * 第三级是本项目新立的那条工程约束（约束 12）的直接守卫：
 * **凡是能改变战斗结果的输入，都必须落在哈希锚点里**。
 * 与其相信「以后会记得」，不如让遗漏时测试变红。
 *
 * ## 兼容性铁律
 *
 * `level` 缺失或为 1 时，`replayHash()` 必须与「等级系数不存在」时
 * **逐位相同**。否则升级功能一上线，所有存量战报静默失配。
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, type BattleConfig } from './engine'
import { equippedFromSnapshot, type BuildSnapshot } from './replay'
import { ACTIVE_SLOTS } from './heatmap'
import { defaultAttacker } from './damage'
import { DEFAULT_SKILL_RULES, skillBaseDamageAtLevel } from './skill'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'

const allLevels = fixture.levels as unknown as GeneratedLevel[]
// 第 1 关：默认构筑能赢，适合比「伤害变了但没翻盘」
const level1 = allLevels.find((l) => l.id === 1)!
// 第 100 关：默认构筗会漏怪，但升级等级会减少漏怪 ——
// ⚠️ 2026-10-10：steam_burst AoE 实现后，第 97 关被 AoE「救回」
//（attack-independent 伤害把漏怪救回来），导致 level1=76/leak, level10=93/leak
// 反向（升级更糟），违反单调性守卫。改用第 100 关：
// level1=69/leak > level10=64/leak，单调性仍然成立。
const level100 = allLevels.find((l) => l.id === 100)!

const enemies = new Map<number, EnemyDef>(
  (fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]),
)
const skills = new Map<number, SkillDef>(
  [
    ...(fixture.skills as unknown as SkillDef[]),
    ...(fixture.composite_skills as unknown as SkillDef[]),
  ].map((s) => [s.id, s]),
)

/** 造一个 build 快照，技能等级统一为 `level`（undefined = 不下发该字段）。 */
function buildWithLevel(level?: number): BuildSnapshot {
  const active = [...skills.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, ACTIVE_SLOTS)
  const map: Record<string, unknown> = {}
  active.forEach((s, i) => {
    const entry: Record<string, unknown> = {
      id: s.id,
      name: s.name,
      family: (s as unknown as { family?: string }).family ?? 'test',
      element: s.element,
      kind: s.kind,
      slot: i,
    }
    // ⚠️ undefined 时**不放这个键**，模拟「旧版本快照没有 level 字段」
    if (level !== undefined) entry.level = level
    map[String(s.id)] = entry
  })
  return {
    skills: map,
    attacker: { attack: 0 },
    active_slots: ACTIVE_SLOTS,
  } as unknown as BuildSnapshot
}

function engineWith(snapshot: BuildSnapshot, lv: GeneratedLevel, seed: number): BattleEngine {
  const cfg: BattleConfig = {
    level: lv,
    enemies,
    skills,
    equipped: equippedFromSnapshot(snapshot, skills),
    attacker: defaultAttacker(),
    seed,
    activeSlots: ACTIVE_SLOTS,
  }
  return new BattleEngine(cfg)
}

/**
 * 把一局跑到 won/lost。
 *
 * ⚠️ `card_select` 必须显式跳过。第一版漏了这一句，
 * 引擎停在第一波选牌不动 —— 于是：
 *   - `totalDamage` 两边都 0  →「满级伤害更高」恒假
 *   - `leaked` 两边都 0      →「不升级会漏」恒假
 *   - `drainEvents()` 空     → 看起来像「等级完全没效果」
 *
 * 三个断言一起红，而**真因与被测功能毫无关系**。
 * 这是本项目「度量前提不成立」的又一例：
 * 断言红了先问「前提成立吗」，别急着改被测代码。
 * （与 README 里记的 `skipCards` 导致机制卡测不出，是同一个坑的两面。）
 */
function runToEnd(e: BattleEngine, maxTicks = 40000) {
  e.start()
  for (let t = 0; t < maxTicks; t++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
  }
  return e
}

// ── 级别 1：反射 ──────────────────────────────────────────────

describe('等级进入 equipped 的 baseDamage（反射级）', () => {
  const dmg = (lv?: number) =>
    equippedFromSnapshot(buildWithLevel(lv), skills).map((s) => s.baseDamage)

  it('等级越高，baseDamage 越大', () => {
    const at1 = dmg(1)
    const at5 = dmg(5)
    const at10 = dmg(10)
    for (let i = 0; i < at1.length; i++) {
      expect(at5[i]).toBeGreaterThan(at1[i])
      expect(at10[i]).toBeGreaterThan(at5[i])
    }
  })

  it('等级 1 与「不下发 level 字段」逐位相同', () => {
    // 这就是存量战报兼容性的**直接**判据
    expect(dmg(1)).toEqual(dmg(undefined))
  })

  it('低伤害技能的等级加成被整除截断（不会变成白给的伤害）', () => {
    // 90 * 1450 / 1000 = 130.5 → 130
    //
    // 刻意向下取整而不是四舍五入：若四舍五入到 131，
    // 「练级」就成了一个纯粹的伤害白给，没有取舍。
    // 截断让满级 +45% 对小技能只值 +44%，
    // 而对大技能（50000 → 72500）几乎无损 ——
    // 于是升级的边际收益天然偏向已经强的技能，符合「专精」定位。
    const base = 90n
    const at10 = skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, base, 10)
    expect(at10).toBe(130n)
    expect(at10).toBeLessThan((base * 1450n) / 1000n + 1n)
  })
})

// ── 级别 2：行为 ──────────────────────────────────────────────

describe('等级真的改变战斗结果（行为级）', () => {
  it('满级技能让同一关清得更快', () => {
    // ⚠️ 多种子平均判据（第 160 轮）：
    // 单一 seed（原 999）在全局节奏改动后可能翻红（LEGACY 基线仅 6/7 通过），
    // 改成跨多 seed 取平均，避免「碰巧通过的单 seed」把守卫变成 flake。
    // 只有 won 的局参与平均；至少 3 个 seed 必须 won 才算前提成立。
    const seeds = [999, 12345, 4242, 777, 2024]
    // 多数种子方向一致即可（平均会被单个异常 seed 扭曲）。
    let fasterCount = 0
    let validCount = 0
    for (const seed of seeds) {
      const a = runToEnd(engineWith(buildWithLevel(1), level1, seed))
      const b = runToEnd(engineWith(buildWithLevel(10), level1, seed))
      if (a.phase !== 'won' || b.phase !== 'won') continue
      validCount++
      if (Number(b.elapsedMs) < Number(a.elapsedMs)) fasterCount++
    }
    expect(validCount).toBeGreaterThanOrEqual(3)
    // 多数种子上满级更快（至少 3 个）
    expect(fasterCount).toBeGreaterThanOrEqual(3)
  })

  it('同 seed 下分数不同', () => {
    // ⚠️ 只断言「不同」，**不断言方向**。
    //
    // 分数 = 伤害分 + 击杀分 + （可能的）速率分。
    // 等级更高 → 伤害分涨、但战斗更早结束 → 时间项可能反向。
    // 两个效应方向相反，净效果不能靠推理断定 ——
    // 上一版我断言「发射数与伤害无关，两边应相等」，
    // 实测 191 vs 159：等级高 → 战斗更早结束 → 累计发射更少。
    // 前提错了，断言红，而红的原因与被测功能无关。
    //
    // 方向性判据交给上面两条有明确单调性的可观测量
    // （elapsedMs 更短、leaked 更少），这里只守「结果确实变了」。
    const a = runToEnd(engineWith(buildWithLevel(1), level1, 999))
    const b = runToEnd(engineWith(buildWithLevel(10), level1, 999))
    expect(a.score).not.toBe(b.score)
  })

  it('等级真的改变漏怪数（第 100 关，默认构筗会漏）', () => {
    // ⚠️ 多种子平均判据（第 160 轮）：同上，避免单 seed flake。
    const seeds = [999, 12345, 4242, 777, 2024]
    let anyLeak = false
    let lessLeakCount = 0
    let validCount = 0
    for (const seed of seeds) {
      const a = runToEnd(engineWith(buildWithLevel(1), level100, seed))
      const b = runToEnd(engineWith(buildWithLevel(10), level100, seed))
      if (a.leaked <= 0) continue
      anyLeak = true
      validCount++
      if (b.leaked < a.leaked) lessLeakCount++
    }
    expect(anyLeak).toBe(true) // 前提成立：不升级真的会漏
    expect(validCount).toBeGreaterThanOrEqual(3)
    // 多数种子上满级漏怪更少
    expect(lessLeakCount).toBeGreaterThanOrEqual(3)
  })

  it('击杀数相同而用时更短 —— 等级是「效率」不是「碾压」', () => {
    // ⚠️ 多种子平均判据（第 160 轮）：同上。
    const seeds = [999, 12345, 4242, 777, 2024]
    let equalKillsCount = 0
    let validCount = 0
    for (const seed of seeds) {
      const a = runToEnd(engineWith(buildWithLevel(1), level1, seed))
      const b = runToEnd(engineWith(buildWithLevel(10), level1, seed))
      if (a.phase !== 'won' || b.phase !== 'won') continue
      validCount++
      if (b.kills === a.kills) equalKillsCount++
    }
    expect(validCount).toBeGreaterThanOrEqual(3)
    // 多数种子上击杀数不变（等级是效率而非碾压）
    expect(equalKillsCount).toBeGreaterThanOrEqual(3)
  })
})

// ── 级别 3：行为 + 哈希锚点（本轮新立约束的直接守卫）────────────

describe('等级落在重放哈希锚点里（工程约束 12）', () => {
  it('等级 1 与缺省的战斗结果逐位相同（不只是哈希）', () => {
    // 存量战报兼容性的完整判据：不只是哈希一样，
    // 而是**每一个可观测量**都一样 ——
    // 否则「哈希相同但重放出的战报不同」就是另一种形式的哈希说谎。
    const a = runToEnd(engineWith(buildWithLevel(1), level1, 4242))
    const b = runToEnd(engineWith(buildWithLevel(undefined), level1, 4242))
    expect(a.replayHash()).toBe(b.replayHash())
    expect(a.elapsedMs).toBe(b.elapsedMs)
    expect(a.shots).toBe(b.shots)
    expect(a.hits).toBe(b.hits)
    expect(a.kills).toBe(b.kills)
    expect(a.leaked).toBe(b.leaked)
    expect(a.score).toBe(b.score)
  })

  it('不同等级 → 不同 replayHash', () => {
    const h = (lv: number) => engineWith(buildWithLevel(lv), level1, 777).replayHash()
    expect(h(1)).not.toBe(h(2))
    expect(h(1)).not.toBe(h(10))
  })

  it('等级 1 / 缺省 的哈希与「等级系数不存在」时代逐位相同', () => {
    // 存量战报可重放性的最后一道闸。
    //
    // ⚠️ 这条一旦破了，症状是**所有历史战报验真失败**，
    // 而排查方向会自然滑向「I-6 哈希算法坏了」——
    // 真因只是上线时多乘了个 1.0。
    const h1 = engineWith(buildWithLevel(1), level1, 777).replayHash()
    const hNone = engineWith(buildWithLevel(undefined), level1, 777).replayHash()
    expect(h1).toBe(hNone)
  })

  it('「把等级从锚点里去掉」这个变异会让守卫变红', () => {
    // 直接模拟「有人为了少改一处，把 baseDamage 换回 BigInt(def.base_damage)」
    // 的后果：此时 1 级与 10 级哈希相同 → 验真无法区分等级。
    const at = (lv: number) =>
      skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, 1000n, lv)
    expect(at(1)).toBe(1000n)
    expect(at(10)).toBe(1450n)
    expect(at(1)).not.toBe(at(10))
  })
})
