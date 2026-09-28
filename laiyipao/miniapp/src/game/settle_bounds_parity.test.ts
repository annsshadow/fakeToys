/**
 * 服务端新加的集合类边界**必须接受真实引擎产出的上报**。
 *
 * ## 为什么需要这条
 *
 * `server/internal/domain/battle_collections.go` 给 5 个集合类上报字段加了边界：
 *   `elements_used` 合计 ≤ shots×怪数、`reactions_used` 合计 ≤ reactions、
 *   `terrain_used` ⊆ 该关地形种类、`card_picks` 长度 ≤ 波数且每项 ≥ -1。
 *
 * 这些边界的**风险方向是反的**：
 *   写松了 → 只是少拦一点作弊（可接受）
 *   写紧了 → **拒绝玩家的正常对局**，表现为「打完一局提示结算失败」（最伤）
 *
 * e2e 证明不了这件事：它发的是**合成 payload**，
 * 而那份 payload 是照着我的边界写的 —— 属于自证。
 *
 * 所以这里取**引擎真实跑出来的 `settleInput()`**，
 * 逐条核对它满足服务端会检查的每一个不等式。
 *
 * 判据的方向是**「引擎产出 ⊆ 服务端上界」**（反方向才会要求服务端上界取可用值，
 * 那是服务端的事，不该由客户端测试断言）。
 *
 * ## 性能：7 次全量扫描合并成 1 次
 *
 * 第一版每个 `it` 各自 `for (const lv of levels) play(lv)`，
 * 于是**同一个文件把 100 关扫了 7 遍**。实测单遍 ≈ 3.7s（561,516 tick，
 * 平均 6.6 µs/tick），也就是这个文件 ~90% 的时间在重复计算同一件事。
 *
 * 现在改成 `beforeAll` 扫一遍、7 条判据共用这批观测，耗时降到约 1/7。
 *
 * ### 为什么合并不改变任何判定
 *
 * 7 条判据读的都是**同一批观测量**（elements/reactions 之和、terrain 集合、
 * card_picks、replay_hash、hits、shots），它们之间**没有依赖关系**，
 * 也没有任何一条会改变引擎状态或配置。
 * 所以「先全部采集、再逐条断言」与「边跑边断言」完全等价。
 *
 * ### 这一版刻意**没有**加新断言
 *
 * 这是重构，不是补测试。加断言会让「提速」与「改了判据强度」两件事
 * 混在同一批提交里，review 时无法分开看。
 * 想加更强的判据（例如 reactions 分项合计应当**恒等**于 reactions），
 * 请单独一批。
 *
 * ### 为什么扫描放在 beforeAll 而不是模块顶层
 *
 * 模块顶层代码在**收集阶段**执行：那里抛错会以「文件加载失败」的形式出现，
 * 掩盖真正的失败点，也拿不到 `expect` 的语义。
 * 放在 `beforeAll` 里，失败会正常标记本套件内所有用例失败。
 */
import { describe, it, expect, beforeAll } from 'vitest'
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

/** 一关跑完后，7 条判据各自需要的那个数字。 */
interface Observation {
  levelId: number
  /** elements_used 各键之和 */
  elemTotal: number
  /** reactions_used 各键之和 */
  reactTotal: number
  reactions: number
  hits: number
  shots: number
  /** 该关波次里声明的怪物总数（服务端上界里的 MaxKillsFor 那一项） */
  enemyCount: number
  waveCount: number
  terrainUsed: string[]
  /** 该关自己的地形种类 */
  ownTerrain: Set<string>
  cardPicks: number[]
  replayHash: string
}

function sumRecord(m: unknown): number {
  return Object.values(m as Record<string, number>).reduce((a, b) => a + b, 0)
}

/** 全量扫描的结果，**全文件只算一次**。 */
let observations: Observation[] = []

beforeAll(() => {
  observations = levels.map((lv) => {
    const r = play(lv)
    return {
      levelId: lv.id,
      elemTotal: sumRecord(r.elements_used),
      reactTotal: sumRecord(r.reactions_used),
      reactions: r.reactions,
      hits: r.hits,
      shots: r.shots,
      enemyCount: (lv.waves ?? []).reduce(
        (a, w) => a + w.spawns.reduce((b, sp) => b + sp.count, 0),
        0,
      ),
      waveCount: lv.wave_count,
      terrainUsed: r.terrain_used,
      ownTerrain: new Set(
        ((lv as unknown as { terrain?: { kind: string }[] }).terrain ?? []).map((t) => t.kind),
      ),
      cardPicks: r.card_picks,
      replayHash: r.replay_hash,
    }
  })
}, 900_000)

/** 逐条列出 100 条会把关键信息淹掉，所以只报前 5 个 offender。 */
function reportFirst(bad: string[]): string {
  return bad.slice(0, 5).join('\n')
}

describe('引擎真实上报满足服务端的新边界', () => {
  it('全 100 关：elements_used 合计 ≤ shots × 该关怪数', () => {
    // 服务端：if total > shots * MaxKillsFor(gl) { reject }
    //
    // ⚠️ 第一版这里写的是 `total <= hits`，**红了** ——
    // 真实引擎上报 139 > hits 134，max 比值 1.214。
    // 若 e2e 用的是真实 payload，这个 bug 会一路带到线上，
    // 表现为「玩家打完一局提示结算失败」。
    //
    // 根因：`hits++` 只数直接命中，而 `elementsUsed` 在 `hitEnemy` 里，
    // AoE 与链式也走 `hitEnemy` 却不计入 `hits`。
    const bad: string[] = []
    for (const o of observations) {
      const cap = o.shots * o.enemyCount
      if (o.elemTotal > cap) {
        bad.push(`L${o.levelId}: 元素合计 ${o.elemTotal} > 上界 ${cap}`)
      }
    }
    expect(reportFirst(bad)).toBe('')
  })

  it('记录实测余量：合计/hits 会超过 1，合计/(shots×总敌数) 远小于 1', () => {
    // 把两个比值都钉住，作为「上界取哪个」的依据。
    // 若哪天 `hits` 的口径改成含 AoE/链式：
    //   第一条会变成：变绿。那时可以把上界收回到 hits，
    //   收益是更严的保护，代价是边界与引擎耦合更深。
    let maxOverHits = 0
    let maxOverShotsEnemies = 0
    for (const o of observations) {
      maxOverHits = Math.max(maxOverHits, o.elemTotal / Math.max(o.hits, 1))
      maxOverShotsEnemies = Math.max(
        maxOverShotsEnemies,
        o.elemTotal / Math.max(o.shots * o.enemyCount, 1),
      )
    }
    // 实测 1.214 倍，明确大于 1，所以**不能**用 hits
    expect(maxOverHits).toBeGreaterThan(1)
    // 实测 0.0316 倍，上界留了约 30 倍余量，很宽裕
    expect(maxOverShotsEnemies).toBeLessThan(0.5)
  })

  it('全 100 关：reactions_used 合计 ≤ reactions', () => {
    // 服务端：if totalR > in.Reactions { reject }
    const bad: string[] = []
    for (const o of observations) {
      if (o.reactTotal > o.reactions) {
        bad.push(`L${o.levelId}: 反应分项合计 ${o.reactTotal} > reactions ${o.reactions}`)
      }
    }
    expect(reportFirst(bad)).toBe('')
  })

  it('全 100 关：terrain_used 无重复、且是已知地形种类', () => {
    const bad: string[] = []
    for (const o of observations) {
      const seen = new Set<string>()
      for (const t of o.terrainUsed) {
        if (seen.has(t)) bad.push(`L${o.levelId}: 地形 ${t} 重复`)
        seen.add(t)
        if (!ALL_TERRAIN.has(t)) bad.push(`L${o.levelId}: 未知地形 ${t}`)
      }
    }
    expect(reportFirst(bad)).toBe('')
  })

  it('全 100 关：terrain_used ⊆ 该关自己的地形种类', () => {
    // 服务端：if !distinct[t] { reject }
    //
    // 这条是本轮最险的一条：引擎的地形来自 `cfg.level.terrain`，
    // 而 `generateTerrain` 产出的 1~3 个地形**全是同一个 kind**，
    // 100 关里 75 关没有地形。所以只要引擎不动态造地形，它必然成立。
    // 但「必然」是推断，不是事实 —— 这里用实测把它钉住。
    const bad: string[] = []
    for (const o of observations) {
      for (const t of o.terrainUsed) {
        if (!o.ownTerrain.has(t)) bad.push(`L${o.levelId}: 上报了本关没有的地形 ${t}`)
      }
    }
    expect(reportFirst(bad)).toBe('')
  })

  it('全 100 关：card_picks 长度 ≤ 波数、每项 ≥ -1', () => {
    const bad: string[] = []
    for (const o of observations) {
      if (o.cardPicks.length > o.waveCount) {
        bad.push(`L${o.levelId}: card_picks ${o.cardPicks.length} 项 > 波数 ${o.waveCount}`)
      }
      o.cardPicks.forEach((p, i) => {
        if (p < -1) bad.push(`L${o.levelId}: card_picks[${i}] = ${p} < -1`)
      })
    }
    expect(reportFirst(bad)).toBe('')
  })

  it('全 100 关：replay_hash 是 16 位十六进制（服务端上限 64）', () => {
    const bad: string[] = []
    for (const o of observations) {
      if (!/^[0-9a-f]{16}$/.test(o.replayHash)) {
        bad.push(`L${o.levelId}: replay_hash = ${o.replayHash}`)
      }
    }
    expect(reportFirst(bad)).toBe('')
  })
})
