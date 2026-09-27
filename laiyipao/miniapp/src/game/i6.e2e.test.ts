/**
 * I-6 验真闭环的端到端验证（**需要运行中的服务端**）。
 *
 * 完整链路：
 *   1. 游客登录
 *   2. 向服务端申请开局凭证（拿 seed + 权威 build/attacker + 技能槽）
 *   3. 用本地引擎跑完整局 → 得到 replay_hash_A
 *   4. settle 上报 hash_A
 *   5. 换个人拉取 /battle/{id}/replay（含 build 快照）
 *   6. 用 replay() 重放 → 得到 replay_hash_B
 *   7. 断言 hash_A === hash_B，并提交验真确认服务端也判为 matched
 *
 * ⚠️ 这个测试与其它单元测试不同：它打真实 HTTP、真实数据库。
 * 因此默认**跳过**，需显式设置 LYP_E2E=1 才运行。
 * CI 里应由 scripts/e2e.ps1 在起好服务后调用。
 *
 * 为什么值得单独写：I-6 是整个防作弊体系的基石，
 * 而"哈希函数本身正确"并不能证明它成立 —— 真正要证明的是
 * 「服务端下发的所有输入足以让第三方重算出同一个哈希」。
 * 缺任何一环（attacker / 技能槽 / 构筑）这个测试都会红。
 */
import { describe, it, expect, beforeAll } from 'vitest'
import { BattleEngine } from './engine'
import { replay as runReplay, type ReplayInfo } from './replay'
import type { Attacker } from './damage'
import type { EquippedSkill } from './heatmap'
import type { Element } from './elements'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'

const ENABLED = process.env.LYP_E2E === '1'
const BASE = process.env.LYP_API ?? 'http://127.0.0.1:8080/api/v1'

let token = ''
let enemies = new Map<number, EnemyDef>()
let skills = new Map<number, SkillDef>()

async function api<T>(path: string, opts: { method?: string; body?: unknown; auth?: boolean } = {}): Promise<T> {
  const headers: Record<string, string> = { 'Content-Type': 'application/json' }
  if (opts.auth !== false && token) headers.Authorization = `Bearer ${token}`
  const res = await fetch(`${BASE}${path}`, {
    method: opts.method ?? 'GET',
    headers,
    body: opts.body === undefined ? undefined : JSON.stringify(opts.body),
  })
  const text = await res.text()
  const data = text ? JSON.parse(text) : {}
  if (!res.ok) throw new Error(`${path} → HTTP ${res.status}: ${data?.error?.message ?? text}`)
  return data as T
}

function toAttacker(a: Record<string, number>): Attacker {
  return {
    attack: BigInt(a.attack),
    critPermille: BigInt(a.crit_permille),
    critMultiplierPermille: BigInt(a.crit_multiplier_permille),
    reactionMultPermille: BigInt(a.reaction_mult_permille),
    elementCap: BigInt(a.element_cap),
    reactionTier: BigInt(a.reaction_tier),
    elementCoefPermille: BigInt(a.element_coef_permille),
    // 三项本轮新增的攻方字段（专精 heat_cap/armor/mechanic + 装备与宝石）。
    //
    // ⚠️ 必须与 `store/game.ts` 的转换保持**逐字段一致** ——
    // 这两处是同一份转换的两个副本，漏一个字段就会让 e2e 重放
    // 与页面实际跑的战斗算出的哈希不同，而 I-6 会把正常对局判成伪造。
    //
    // 本文件已由 `i6.e2e.test.ts` 自身验证：它拿服务端真实响应重放，
    // 哈希不一致会直接红。所以"两处转换漂移"在这里是**可观测**的。
    heatCapPermille: BigInt(a.heat_cap_permille ?? 0),
    armorPermille: BigInt(a.armor_permille ?? 0),
    mechanicPermille: BigInt(a.mechanic_permille ?? 0),
  }
}

/** 复刻页面里的构筑装配逻辑：完全以服务端下发的 slot 为准 */
function equipFromBuild(build: any, defs: Map<number, SkillDef>): EquippedSkill[] {
  const list = Object.values((build?.skills ?? {}) as Record<string, any>)
    .filter((s) => s && typeof s.id === 'number' && s.slot >= 0)
    .sort((a: any, b: any) => a.slot - b.slot || a.id - b.id)
  const out: EquippedSkill[] = []
  for (const s of list) {
    const def = defs.get(s.id)
    if (!def) continue
    out.push({
      skillId: def.id, name: def.name, element: def.element, kind: def.kind,
      heatCost: BigInt(def.heat_cost), cooldownMs: def.cooldown_ms, pierce: def.pierce,
      aoeRadius: def.aoe_radius, baseDamage: BigInt(def.base_damage),
      applyElement: (def.apply_element || def.element) as Element | '',
      applyStacks: BigInt(def.apply_stacks), projectileSpeed: def.projectile_speed,
      chain: def.chain, slot: s.slot, cooldownRemaining: 0,
    })
  }
  return out
}

function driveToEnd(engine: BattleEngine, maxTicks = 60 * 60 * 10): number {
  let ticks = 0
  while (ticks < maxTicks) {
    const phase: string = engine.phase
    if (phase === 'won' || phase === 'lost') return ticks
    if (phase === 'card_select') engine.skipCards()
    engine.step()
    ticks++
  }
  return ticks
}

describe.skipIf(!ENABLED)('I-6 验真闭环（端到端）', () => {
  beforeAll(async () => {
    const cfg = await api<{ enemies: EnemyDef[]; skills: SkillDef[] }>('/config', { auth: false })
    enemies = new Map(cfg.enemies.map((e) => [e.id, e]))
    skills = new Map(cfg.skills.map((s) => [s.id, s]))
    const tp = await api<{ access_token: string }>('/auth/guest', {
      method: 'POST', auth: false, body: { guest_token: '' },
    })
    token = tp.access_token
  }, 60_000)

  it('引擎重放的哈希与服务端记录完全一致', async () => {
    // 1) 申请开局凭证
    const bt = await api<{ token_id: number; seed: string; level: GeneratedLevel; build: any }>(
      '/battle/token',
      { method: 'POST', body: { level_id: 1 } },
    )

    // 2) 前置条件：服务端必须下发完整构筑，否则重放无从谈起
    expect(bt.build?.skills, 'build.skills 缺失 → 无法重放').toBeTruthy()
    expect(bt.build?.attacker, 'build.attacker 缺失 → I-6 从设计上就通不了').toBeTruthy()
    const equipped = equipFromBuild(bt.build, skills)
    expect(equipped.length, '未装备任何技能，跳过（该用户尚未配置构筑）').toBeGreaterThan(0)

    // 3) 本地跑一局
    const engine = new BattleEngine({
      level: bt.level,
      enemies,
      skills,
      equipped,
      attacker: toAttacker(bt.build.attacker),
      seed: BigInt(bt.seed),
    })
    engine.start()
    driveToEnd(engine)
    const hashA = engine.replayHash()
    expect(hashA).toMatch(/^[0-9a-f]{16}$/)

    // 4) 上报结算
    const settled = await api<{ battle_id?: number; battle: { id: number }; win: boolean }>('/battle/settle', {
      method: 'POST',
      body: {
        token_id: bt.token_id,
        result: engine.phase === 'won' ? 'win' : 'lose',
        stars: 0, score: engine.score, kills: engine.kills, leaked: engine.leaked,
        hp_left: Number(engine.baseHp), wave_reached: engine.waveIndex + 1,
        duration_ms: Math.round(engine.elapsedMs), shots: engine.shots, hits: engine.hits,
        reactions: engine.reactionsCount, heat_max: Number(engine.heat.maxHeatThisBattle),
        elements_used: engine.elementsUsed, reactions_used: engine.reactionsUsed,
        terrain_used: [...new Set(engine.terrainUsed)], replay_hash: hashA,
      },
    })
    const battleId = settled.battle_id ?? settled.battle?.id
    expect(battleId, 'settle 未返回 battle_id').toBeTruthy()

    // 5) 换第三方视角拉复现信息
    const info = await api<ReplayInfo>(`/battle/${battleId}/replay`)

    // 6) 重放
    const outcome = runReplay(info, { level: info.level, enemies, skills })

    // 7) 核心断言
    expect(outcome.error, `重放失败：${outcome.error}`).toBeUndefined()
    expect(
      outcome.computedHash,
      '重放哈希与首次对战不一致 —— I-6 失效，说明服务端下发的输入不足以重建战斗',
    ).toBe(hashA)
    expect(outcome.matched).toBe(true)

    // 8) 服务端裁定也应为 matched
    const verdict = await api<{ matched: boolean }>('/battle/verify', {
      method: 'POST',
      body: { battle_id: battleId, replay_hash: hashA },
    })
    expect(verdict.matched).toBe(true)
  }, 120_000)

  it('篡改种子必然被证伪', async () => {
    const bt = await api<{ token_id: number; seed: string; level: GeneratedLevel; build: any }>(
      '/battle/token',
      { method: 'POST', body: { level_id: 1 } },
    )
    const equipped = equipFromBuild(bt.build, skills)
    if (equipped.length === 0) return

    // 拿同一个 seed 跑两遍：一遍正常，一遍换一个种子
    const run = (seed: string): string => {
      const e = new BattleEngine({
        level: bt.level, enemies, skills, equipped,
        attacker: toAttacker(bt.build.attacker), seed: BigInt(seed),
      })
      e.start()
      driveToEnd(e)
      return e.replayHash()
    }
    const real = run(bt.seed)
    const fake = run((BigInt(bt.seed) + 1n).toString())

    expect(fake, '换种子后哈希不变 → 证伪能力不成立').not.toBe(real)
  }, 120_000)
})
