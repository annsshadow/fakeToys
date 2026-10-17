import { readFileSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { BattleEngine } from '../src/game/engine'
import { scoreRulesFromServer } from '../src/game/score'

const here = dirname(fileURLToPath(import.meta.url))
const serverRoot = resolve(here, '../../server')
const fixture = JSON.parse(readFileSync(resolve(serverRoot, 'testdata/smoke_levels.json'), 'utf8'))

const ATT = {
  attack: 100n, critPermille: 150n, critMultiplierPermille: 1500n,
  reactionMultPermille: 1200n, elementCap: 3n, reactionTier: 1n,
  elementCoefPermille: 1000n, heatCapPermille: 0n, armorPermille: 200n,
  mechanicPermille: 1000n,
} as never

function toEq(def: any, slot: number) {
  return {
    skillId: def.id, name: def.name, element: def.element, kind: def.kind,
    heatCost: BigInt(def.heat_cost), cooldownMs: def.cooldown_ms, pierce: def.pierce,
    aoeRadius: def.aoe_radius, baseDamage: BigInt(def.base_damage),
    applyElement: def.apply_element ?? '', applyStacks: BigInt(def.apply_stacks ?? 0),
    projectileSpeed: def.projectile_speed, chain: def.chain, slot, cooldownRemaining: 0,
  }
}

// 关卡 -> 地形种类
const terrLevels = new Map<number, string>()
for (const l of fixture.levels) for (const t of (l.terrain ?? [])) terrLevels.set(l.id, t.kind)
const byKind = new Map<string, number[]>()
for (const [id, k] of terrLevels) {
  if (!byKind.has(k)) byKind.set(k, [])
  byKind.get(k)!.push(id)
}

const BUILDS: Array<{ name: string; ids: number[] }> = [
  { name: 'weak1', ids: [1] },
  { name: 'weak2', ids: [1, 4] },
  { name: 'kinetic', ids: [10, 13] },
  { name: 'fire', ids: [1, 2, 3] },
  { name: 'ice', ids: [4, 5, 6] },
  { name: 'light', ids: [7, 8, 9] },
  { name: 'corr', ids: [16, 17, 18] },
]
const SEEDS = ['1', '7', '42', '12345', '-1', '3', '11', '99', '2024', '555']

const skillDefs = new Map<number, any>(
  [...fixture.skills, ...(fixture.composite_skills)].map((x) => [x.id, x]),
)
const enemyDefs = new Map<number, any>(fixture.enemies.map((e) => [e.id, e]))

for (const [kind, levels] of byKind) {
  console.log(`\n=== ${kind} ===`)
  for (const id of levels.slice(0, 3)) {
    const level = fixture.levels.find((l: any) => l.id === id)
    for (const b of BUILDS) {
      for (const seed of SEEDS) {
        const engine = new BattleEngine({
          level: level as never, enemies: enemyDefs, skills: skillDefs,
          equipped: b.ids.map((sid, i) => toEq(skillDefs.get(sid), i)) as never,
          attacker: ATT, seed: BigInt(seed), scoreRules: scoreRulesFromServer(fixture.score_rules),
          activeSlots: 4,
        } as never)
        engine.start()
        let ticks = 0
        for (; ticks < 30000; ticks++) {
          if (engine.phase !== 'wave' && engine.phase !== 'prepare') break
          engine.step()
        }
        const trg = (engine as any).terrains.filter((t: any) => t.triggered).length
        const states = (engine as any).terrains.map((t: any) => t.state).join(',')
        const blocked = (engine as any).terrains.some((t: any) => t.state !== 'idle' && t.state !== 'collapsed')
        const charged = (engine as any).terrains.some((t: any) => t.charge > 0)
        const angled = (engine as any).terrains.some((t: any) => t.angle !== 0)
        if (trg > 0 || blocked || charged || angled) {
          console.log(`  lvl${id} ${b.name.padEnd(8)} seed=${seed.padStart(6)} ticks=${String(ticks).padStart(5)} triggered=${trg} states= charge= angle=${states} shots=${(engine as any).shots} kills=${(engine as any).kills} leaked=${(engine as any).leaked}`)
        }
      }
    }
  }
}