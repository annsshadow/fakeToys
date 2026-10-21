/** 一次性探针：复现 replay_vectors.json 的某个用例，导出完整 replay 事件流 + 渲染事件。 */
import { readFileSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { equippedFromSnapshot, type BuildSnapshot } from '../src/game/replay'
import { scoreRulesFromServer } from '../src/game/score'
import { BattleEngine } from '../src/game/engine'

const here = dirname(fileURLToPath(import.meta.url))
const serverRoot = resolve(here, '../../server')
const fixture = JSON.parse(
  readFileSync(resolve(serverRoot, 'testdata/smoke_levels.json'), 'utf8'),
)
const doc = JSON.parse(readFileSync(resolve(serverRoot, 'testdata/replay_vectors.json'), 'utf8'))

const idx = Number(process.argv[2] ?? 1)
const out = process.argv[3] ?? join(tmpdir(), 'ts-replay.txt')
const c = doc.cases[idx]

const level = fixture.levels.find((l: any) => l.id === c.level_id)
const enemies = new Map<number, any>((fixture.enemies as any[]).map((e) => [e.id, e]))
const skills = new Map<number, any>(
  [...fixture.skills, ...(fixture.composite_skills as any[])].map((s) => [s.id, s]),
)
const equipped = equippedFromSnapshot(c.build as BuildSnapshot, skills as never)
const att = {
  attack: BigInt(c.build.attacker.attack),
  critPermille: BigInt(c.build.attacker.crit_permille),
  critMultiplierPermille: BigInt(c.build.attacker.crit_multiplier_permille),
  reactionMultPermille: BigInt(c.build.attacker.reaction_mult_permille),
  elementCap: BigInt(c.build.attacker.element_cap),
  reactionTier: BigInt(c.build.attacker.reaction_tier),
  elementCoefPermille: BigInt(c.build.attacker.element_coef_permille),
  heatCapPermille: BigInt(c.build.attacker.heat_cap_permille ?? 0),
  armorPermille: BigInt(c.build.attacker.armor_permille ?? 0),
  mechanicPermille: BigInt(c.build.attacker.mechanic_permille ?? 0),
}
const engine = new BattleEngine({
  level: level as never,
  enemies: enemies as never,
  skills: skills as never,
  equipped: equipped as never,
  attacker: att as never,
  seed: BigInt(c.seed),
  activeSlots: c.build.active_slots,
  scoreRules: scoreRulesFromServer(fixture.score_rules),
})
engine.setReplayScript(c.card_picks)
engine.start()
let ticks = 0
const evs: any[] = []
while (ticks < 36000) {
  const ph = engine.phase
  if (ph === 'won' || ph === 'lost') break
  evs.push(...(engine.step() as any[]))
  ticks++
}

const SEP = '|'
const NL = String.fromCharCode(10)
const ev = evs
const evLines = ev.map((e: any) => {
  if (e.type === 'hit') return ['hit', String(e.damage)].join(SEP)
  if (e.type === 'reaction') return ['reaction', String(e.reaction), String(e.damage)].join(SEP)
  return String(e.type)
})
writeFileSync(out + '.events', evLines.join(NL), 'utf8')

const rp = (engine as any).replay as any[]
const rpLines = rp.map((e: any) => [e.t, e.type, e.a, e.b, e.c].join(SEP))
writeFileSync(out, rpLines.join(NL), 'utf8')
console.log('case', c.level_id, 'ticks', ticks, 'replay', rpLines.length, 'events', evLines.length)
console.log('skills:', (equipped as any[]).map((e: any) =>
  `${e.slot}:${e.skillId}:${e.baseDamage}:${e.applyStacks}:${e.heatCost}`).join(','))
console.log('picks:', JSON.stringify((engine as any).cardPicks))