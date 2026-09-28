/**
 * 引擎热循环基线测量。
 *
 * 目的：在**动手优化之前**先拿到数字。
 * 本项目的教训是「前提不成立时改代码」—— 而「某处很慢」正是一个
 * 未经测量的假设。
 *
 * 测量三件事：
 *  1. 单局战斗的墙钟时间（玩家实际等待的东西）
 *  2. 每 tick 的耗时分布（定位热点）
 *  3. bigint 运算的占比（TS 里最可能的热点）
 */
import { describe, it } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, MAX_BATTLE_TICKS, TICK_MS } from './engine'
import { equippedFromSnapshot, type BuildSnapshot } from './replay'
import { ACTIVE_SLOTS } from './heatmap'
import { defaultAttacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'

const levels = (fixture.levels as unknown as GeneratedLevel[]).slice().sort((a, b) => a.id - b.id)
const enemies = new Map<number, EnemyDef>((fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]))
const skills = new Map<number, SkillDef>(
  [...(fixture.skills as unknown as SkillDef[]), ...(fixture.composite_skills as unknown as SkillDef[])].map((s) => [s.id, s]),
)

function snap(): BuildSnapshot {
  const a = [...skills.values()].filter((s) => s.kind === 'active').sort((x, y) => x.id - y.id).slice(0, ACTIVE_SLOTS)
  const m: Record<string, unknown> = {}
  a.forEach((s, i) => { m[String(s.id)] = { id: s.id, name: s.name, family: 't', element: s.element, kind: s.kind, slot: i, level: 1 } })
  return { skills: m, attacker: { attack: 0 }, active_slots: ACTIVE_SLOTS } as unknown as BuildSnapshot
}

function mk(lv: GeneratedLevel): BattleEngine {
  return new BattleEngine({
    level: lv, enemies, skills,
    equipped: equippedFromSnapshot(snap(), skills),
    attacker: defaultAttacker(), seed: 12345, activeSlots: ACTIVE_SLOTS,
  })
}

describe('引擎性能基线', () => {
  it('单局战斗：墙钟 / tick 数 / 每 tick 微秒', () => {
    const samples: Array<{ id: number; ms: number; ticks: number; usPerTick: number; won: boolean }> = []
    for (const lv of levels) {
      const e = mk(lv)
      const t0 = process.hrtime.bigint()
      e.start()
      for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
        if (e.phase === 'won' || e.phase === 'lost') break
        if (e.phase === 'card_select') e.skipCards()
        e.step()
      }
      const ms = Number(process.hrtime.bigint() - t0) / 1e6
      const ticks = e.tick
      samples.push({
        id: lv.id, ms, ticks,
        usPerTick: ticks > 0 ? (ms * 1000) / ticks : 0,
        won: e.phase === 'won',
      })
    }
    const total = samples.reduce((a, b) => a + b.ms, 0)
    const totalTicks = samples.reduce((a, b) => a + b.ticks, 0)
    const slowest = [...samples].sort((a, b) => b.ms - a.ms).slice(0, 5)
    const fastest = [...samples].sort((a, b) => a.ms - b.ms).slice(0, 3)

    console.log(`  关卡数           ${samples.length}`)
    console.log(`  模拟总时长        ${total.toFixed(0)} ms  (真实游戏时长 ${(totalTicks * TICK_MS / 1000).toFixed(0)} s)`)
    console.log(`  总 tick 数        ${totalTicks}`)
    console.log(`  平均每 tick       ${((total * 1000) / totalTicks).toFixed(1)} µs`)
    console.log(`  平均每关          ${(total / samples.length).toFixed(2)} ms`)
    console.log(`  最慢 5 关: ` + slowest.map((s) => `L${s.id}=${s.ms.toFixed(0)}ms/${s.ticks}t`).join('  '))
    console.log(`  最快 3 关: ` + fastest.map((s) => `L${s.id}=${s.ms.toFixed(0)}ms`).join('  '))
    console.log(`  通关数            ${samples.filter((s) => s.won).length}/${samples.length}`)

    // 关键判据：一局战斗的**预算**。
    // 小程序里玩家等一局，如果超过 ~50ms 就有明显卡顿感。
    const over50 = samples.filter((s) => s.ms > 50)
    console.log(`  >50ms 的关卡:     ${over50.length}  ${over50.slice(0, 6).map((s) => `L${s.id}`).join(',')}`)
  }, 900_000)

  it('per-tick 成本是否随敌人数增长（O(n²) 检测）', () => {
    // 若每 tick 成本随敌人数量超线性增长，说明存在嵌套遍历。
    // 用「每 tick 微秒 / 同屏敌人数」在关卡间比较。
    const rows: string[] = []
    for (const id of [1, 20, 40, 60, 80, 100]) {
      const lv = levels.find((l) => l.id === id)!
      const total = lv.waves.reduce((a, w) => a + w.spawns.reduce((b, s) => b + s.count, 0), 0)
      const e = mk(lv)
      const t0 = process.hrtime.bigint()
      e.start()
      for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
        if (e.phase === 'won' || e.phase === 'lost') break
        if (e.phase === 'card_select') e.skipCards()
        e.step()
      }
      const ms = Number(process.hrtime.bigint() - t0) / 1e6
      rows.push(`L${id}: 总怪 ${String(total).padStart(3)}  ${String(e.tick).padStart(5)} tick  ${(ms * 1000 / e.tick).toFixed(1)} µs/tick`)
    }
    console.log('  ' + rows.join('\n  '))
  }, 900_000)
})
