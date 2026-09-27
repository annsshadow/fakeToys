/**
 * 服务端新加的集合边界**必须接受真实引擎产出的上报**。
 *
 * ## 为什么需要这条
 *
 * `server/internal/domain/battle_collections.go` 给 5 个集合类上报字段加了边界：
 * `elements_used` 合计 ≤ hits、`reactions_used` 合计 ≤ reactions、
 * `terrain_used` ⊆ 该关地形种类、`card_picks` 长度 ≤ 波数……
 *
 * 这些边界的**风险方向是反的**：写松了只是少拦一点作弊，
 * 写紧了会**拒绝玩家的正常对局** —— 而那表现为「打完一局提示结算失败」，
 * 是最伤的一种 bug。
 *
 * e2e 证明不了这件事：它发的是**合成 payload**，
 * 而合成 payload 是照着我的边界写的，属于自证。
 *
 * 所以这条测试取**引擎真实跑出来的 `report()`**，
 * 逐条核对它满足服务端会检查的每一个不等式。
 *
 * ## 判据的方向
 *
 * 全部是「引擎产出 ≤ 服务端上界」的单向不等式。
 * 反向（「服务端上界 ≤ 引擎产出」）会要求服务端上界取值可用，
 * 那是服务端的事，不该由客户端测试断言。
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, MAX_BATTLE_TICKS, type BattleConfig } from './engine'
import { equippedFromSnapshot, type BuildSnapshot } from './replay'
import { ACTIVE_SLOTS } from './heatmap'
import { defaultAttacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'

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

function play(lv: GeneratedLevel) {
  const cfg: BattleConfig = {
    level: lv,
    enemies,
    skills,
    equipped: equippedFromSnapshot(buildSnapshot(), skills),
    attacker: defaultAttacker(),
    seed: 12345,
    activeSlots: ACTIVE_SLOTS,
  }
  const e = new BattleEngine(cfg)
  e.start()
  for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
  }
  return e.settleInput(0)
}

/** 地形种类全集 —— 与 Go 侧 allTerrainKinds() 同源（章节的 terrain_kind）。 */
const ALL_TERRAIN = new Set(
  Array.from(
    new Set(
      levels
        .map((l) => (l as unknown as { terrain?: { kind: string }[] }).terrain ?? [])
        .flatMap((ts) => ts.map((t) => t.kind)),
    ),
  ),
)

describe('引擎真实上报满足服务端的新边界', () => {
  it('全 100 关：elements_used 合计 ≤ shots × 该关怪数', () => {
    // 服务端：if total > shots * MaxKillsFor(gl) { reject }
    //
    // ⚠️ 第一版这里写的是 `total <= hits`，**红了** ——
    // 真实引擎上报 139 > hits 134，max 比值 1.214。
    // 若 e2e 用的是照着这个界写的合成 payload，
    // 这个 bug 会一路带到线上，表现为「玩家打完一局提示结算失败」。
    //
    // 根因：`hits++` 只数直接命中，而 `elementsUsed` 在 `hitEnemy` 里，
    // AoE 与链式也走 `hitEnemy` 却不计入 `hits`。
    const bad: string[] = []
    for (const lv of levels) {
      const r = play(lv)
      const total = Object.values(r.elements_used as Record<string, number>).reduce(
        (a, b) => a + b,
        0,
      )
      const enemyCount = (lv.waves ?? []).reduce(
        (a, w) => a + w.spawns.reduce((b, sp) => b + sp.count, 0),
        0,
      )
      const cap = r.shots * enemyCount
      if (total > cap) bad.push(`L${lv.id}: 元素合计 ${total} > 上界 ${cap}`)
    }
    expect(bad.slice(0, 5).join('\n')).toBe('')
  }, 900_000)

  it('记录实测比值：合计/hits 会超过 1，而合计/(shots×怪数) 远小于 1', () => {
    // 把两个比值都钉住，作为「上界取哪个」的依据。
    // 若哪天 `hits` 的口径改成也统计 AoE/链式，
    // 第一条会变红 —— 那时可以把上界收紧回 hits，收益是更强的保护。
    let maxOverHits = 0
    let maxOverShotsEnemies = 0
    for (const lv of levels) {
      const r = play(lv)
      const total = Object.values(r.elements_used as Record<string, number>).reduce(
        (a, b) => a + b,
        0,
      )
      const enemyCount = (lv.waves ?? []).reduce(
        (a, w) => a + w.spawns.reduce((b, sp) => b + sp.count, 0),
        0,
      )
      maxOverHits = Math.max(maxOverHits, total / Math.max(r.hits, 1))
      maxOverShotsEnemies = Math.max(
        maxOverShotsEnemies,
        total / Math.max(r.shots * enemyCount, 1),
      )
    }
    // 实测 1.214 —— 明确大于 1，所以「≤ hits」这个界是错的
    expect(maxOverHits).toBeGreaterThan(1)
    // 实测 0.0316 —— 上界有约 30 倍余量，安全
    expect(maxOverShotsEnemies).toBeLessThan(0.5)
  }, 900_000)

  it('全 100 关：reactions_used 合计 ≤ reactions', () => {
    // 服务端：if totalR > in.Reactions { reject }
    const bad: string[] = []
    for (const lv of levels) {
      const r = play(lv)
      const total = Object.values(r.reactions_used as Record<string, number>).reduce(
        (a, b) => a + b,
        0,
      )
      if (total > r.reactions) bad.push(`L${lv.id}: 反应分项合计 ${total} > reactions ${r.reactions}`)
    }
    expect(bad.slice(0, 5).join('\n')).toBe('')
  }, 900_000)

  it('全 100 关：terrain_used 无重复、且是已知地形种类', () => {
    const bad: string[] = []
    for (const lv of levels) {
      const r = play(lv)
      const seen = new Set<string>()
      for (const t of r.terrain_used) {
        if (seen.has(t)) bad.push(`L${lv.id}: 地形 ${t} 重复`)
        seen.add(t)
        if (!ALL_TERRAIN.has(t)) bad.push(`L${lv.id}: 未知地形 ${t}`)
      }
    }
    expect(bad.slice(0, 5).join('\n')).toBe('')
  }, 900_000)

  it('全 100 关：terrain_used ⊆ 该关自己的地形种类', () => {
    // 服务端：if !distinct[t] { reject }
    //
    // 这条是本轮最险的一条 —— 引擎的地形来自 `cfg.level.terrain`，
    // 而 `generateTerrain` 产出的 1~3 个地形**全是同一个 kind**，
    // 100 关里 75 关没有地形。所以只要引擎不动态造地形，它必然成立。
    // 但「必然」是推断，不是事实 —— 这里用实测把它钉住。
    const bad: string[] = []
    for (const lv of levels) {
      const r = play(lv)
      const own = new Set(((lv as unknown as { terrain?: { kind: string }[] }).terrain ?? []).map((t) => t.kind))
      for (const t of r.terrain_used) {
        if (!own.has(t)) bad.push(`L${lv.id}: 上报了本关没有的地形 ${t}`)
      }
    }
    expect(bad.slice(0, 5).join('\n')).toBe('')
  }, 900_000)

  it('全 100 关：card_picks 长度 ≤ 波数、每项 ≥ -1', () => {
    const bad: string[] = []
    for (const lv of levels) {
      const r = play(lv)
      if (r.card_picks.length > lv.wave_count) {
        bad.push(`L${lv.id}: card_picks ${r.card_picks.length} 项 > 波数 ${lv.wave_count}`)
      }
      r.card_picks.forEach((p, i) => {
        if (p < -1) bad.push(`L${lv.id}: card_picks[${i}] = ${p} < -1`)
      })
    }
    expect(bad.slice(0, 5).join('\n')).toBe('')
  }, 900_000)

  it('全 100 关：replay_hash 是 16 位十六进制（服务端上限 64）', () => {
    const bad: string[] = []
    for (const lv of levels) {
      const r = play(lv)
      if (!/^[0-9a-f]{16}$/.test(r.replay_hash)) {
        bad.push(`L${lv.id}: replay_hash = ${r.replay_hash}`)
      }
    }
    expect(bad.slice(0, 5).join('\n')).toBe('')
  }, 900_000)
})
