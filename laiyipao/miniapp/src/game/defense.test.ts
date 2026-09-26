/**
 * 防线值守（I-5 客户端闭环）的行为契约。
 *
 * 重点锁住三条：
 *  1. 快照不完整时**必须拒绝挑战** —— 盲报会污染对方的战绩数据
 *  2. 挑战结果可复现（同 seed 同构筑 → 同结果）
 *  3. 上报字段满足服务端 ChallengeInput 的契约
 */
import { describe, it, expect } from 'vitest'
import {
  runChallenge,
  validateSnapshot,
  snapshotDigest,
  snapshotElements,
  type DefenseView,
} from './defense'
import type { EquippedSkill } from './heatmap'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Attacker } from './damage'
import { resolveHit, Defender, defaultAttacker } from './damage'
import type { Element } from './elements'

const zeroResist = { fire: 0, ice: 0, lightning: 0, corrosion: 0, kinetic: 0 }

/**
 * 敌人血量给到 30000（约 2 万点总伤害需求）。
 *
 * 这个数值是刻意选的：它必须高到"低攻方打不完"。
 * 若敌人太薄（3000 血时 0‰ 攻击力的攻方也能全清），
 * 攻方差异就被刷怪节奏掩盖，测出来的结论会是假的。
 *
 * 配套认知：attack 单位是**千分比**。attack 1‰ → 100‰ 只差 8% 单发伤害，
 * 不足以跨过"多打一发"的门槛，因此耗时相同是正常的，不是缺陷。
 * 有意义的对比是 0‰ / 1000‰（+100%）/ 50000‰（+5000%）。
 */
const ENEMIES: EnemyDef[] = [
  {
    id: 1, code: 'warden', name: '守卫', category: 'normal',
    hp: 30000, speed: 40000, armor: 0, shield_hp: 0, attack: 8, attack_range: 0,
    attack_interval: 0, fly_height: 0, burrow: false, is_boss: false,
    resist: zeroResist, descr: '',
  },
]

function skill(id: number, element: Element, name: string): SkillDef {
  return {
    id, code: `s${id}`, name, family: 'flame', element, kind: 'active', descr: '',
    base_damage: 120, heat_cost: 20, cooldown_ms: 300, pierce: 1, aoe_radius: 60,
    apply_element: element, apply_stacks: 2, projectile_speed: 90000, chain: 0,
    unlock_level: 1,
  }
}

const SKILLS: SkillDef[] = [skill(1, 'fire', '燃烧弹'), skill(2, 'ice', '干冰弹')]

function level(): GeneratedLevel {
  return {
    id: 1, chapter: 1, name: '防线测试', seed: '1', base_hp: 800, wave_count: 1,
    difficulty: 1000, energy_cost: 6, element_cap: 3, armor_permille: 0,
    max_reaction_tier: 2, is_boss: false,
    star_targets: [12000, 17000, 19600],
    // 理论满分：star_targets 由它按 StarTargetRatio = [600, 850, 980]‰ 导出，
    // 结算裁剪的上界也锚定在它上面（不是 star_targets[2]）。
    max_score: 20000,
    terrain: [],
    // 8 只怪：
    // 弱攻方打不完，强攻方能全清 —— 3 只时任何攻方都能秒完，区分不出差异
    waves: [{ wave_index: 0, spawns: [{ enemy_id: 1, count: 8, interval: 60, delay: 0 }] }],
  }
}

function equipped(): EquippedSkill[] {
  return SKILLS.map((s, i) => ({
    skillId: s.id, name: s.name, element: s.element, kind: s.kind,
    heatCost: BigInt(s.heat_cost), cooldownMs: s.cooldown_ms, pierce: s.pierce,
    aoeRadius: s.aoe_radius, baseDamage: BigInt(s.base_damage),
    applyElement: s.element, applyStacks: BigInt(s.apply_stacks),
    projectileSpeed: s.projectile_speed, chain: s.chain, slot: i, cooldownRemaining: 0,
  }))
}

const attacker = (): Attacker => ({
  attack: 400n, critPermille: 50n, critMultiplierPermille: 1500n,
  reactionMultPermille: 1000n, elementCap: 3n, reactionTier: 2n,
  elementCoefPermille: 1500n,
})

const deps = {
  myEquipped: equipped(),
  myAttacker: attacker(),
  level: level(),
  enemies: new Map(ENEMIES.map((e) => [e.id, e])),
  skills: new Map(SKILLS.map((s) => [s.id, s])),
  /**
   * 固定种子。
   *
   * ⚠️ 必须显式注入：默认 seed 依赖 Date.now() 的分钟桶，
   * 不固定的话同一份代码跨分钟跑会得到不同战局，断言变 flaky。
   * 这个坑真实踩过 —— 同一份测试两次运行结果不同。
   */
  seedOverride: 987654321n,
}

function view(over: Partial<DefenseView> = {}): DefenseView {
  return {
    id: 7,
    owner_id: 99,
    owner_name: '对手',
    name: '对手的防线',
    power: 1200,
    element_coverage: 3,
    mastery_done: 4,
    wins: 2,
    losses: 1,
    snapshot: {
      skills: [1, 2],
      equipment: [],
      mastery_nodes: [],
      works: ['slow_belt'],
      elements: ['fire', 'ice'],
      mastery: [],
      rating: { element_coverage: 2, total: 90 },
    },
    snapshot_hash: 'abc123def456',
    can_challenge: true,
    ...over,
  }
}

describe('validateSnapshot：拒绝一切不完整快照', () => {
  it('完整快照通过', () => {
    expect(validateSnapshot(view()).ok).toBe(true)
  })

  it('缺 snapshot → 拒绝', () => {
    const r = validateSnapshot(view({ snapshot: undefined }))
    expect(r.ok).toBe(false)
    if (!r.ok) expect(r.reason).toContain('快照')
  })

  it('快照里没有技能 → 拒绝', () => {
    const r = validateSnapshot(
      view({ snapshot: { ...view().snapshot!, skills: [] } }),
    )
    expect(r.ok).toBe(false)
  })

  it('缺 snapshot_hash → 拒绝（否则无法确认挑战的是同一份构筑）', () => {
    const r = validateSnapshot(view({ snapshot_hash: '' }))
    expect(r.ok).toBe(false)
    if (!r.ok) expect(r.reason).toContain('哈希')
  })

  it('护盾未过期 → 拒绝', () => {
    const future = new Date(Date.now() + 3600_000).toISOString()
    const r = validateSnapshot(view({ shielded_until: future }))
    expect(r.ok).toBe(false)
    if (!r.ok) expect(r.reason).toContain('护盾')
  })

  it('护盾已过期 → 放行', () => {
    const past = new Date(Date.now() - 3600_000).toISOString()
    expect(validateSnapshot(view({ shielded_until: past })).ok).toBe(true)
  })
})

describe('runChallenge', () => {
  it('快照不完整时明确报错且不上报（不污染对方战绩）', () => {
    const r = runChallenge(view({ snapshot: undefined }), deps)
    expect(r.error).toContain('快照')
    expect(r.report.replay_hash).toBe('0000000000000000')
    expect(r.won).toBe(false)
  })

  it('我方无技能 → 报错', () => {
    const r = runChallenge(view(), { ...deps, myEquipped: [] })
    expect(r.error).toContain('技能')
  })

  it('产出合法的上报字段（满足服务端 ChallengeInput 契约）', () => {
    const r = runChallenge(view(), deps)
    expect(r.error).toBeUndefined()
    // seed 必须是可被 int64 解析的整数字符串
    expect(r.report.seed).toMatch(/^-?\d+$/)
    expect(BigInt(r.report.seed)).toBeTypeOf('bigint')
    expect(typeof r.report.won).toBe('boolean')
    expect(r.report.duration_ms).toBeGreaterThan(0)
    expect(r.report.hp_left_pct).toBeGreaterThanOrEqual(0)
    expect(r.report.hp_left_pct).toBeLessThanOrEqual(100)
    expect(r.report.replay_hash).toMatch(/^[0-9a-f]{16}$/)
  })

  it('seed 落在 int64 范围内（超出则服务端解析失败）', () => {
    const r = runChallenge(view(), deps)
    const v = BigInt(r.report.seed)
    expect(v).toBeLessThan(1n << 63n)
    expect(v).toBeGreaterThan(-(1n << 63n))
  })

  it('hp_left_pct 与 stats.hpLeftPct 一致（自洽）', () => {
    const r = runChallenge(view(), deps)
    expect(r.report.hp_left_pct).toBe(r.stats.hpLeftPct)
  })

  it('replay_hash 与 stats 自洽（哈希是 16 位 hex）', () => {
    const r = runChallenge(view(), deps)
    expect(r.report.replay_hash).toMatch(/^[0-9a-f]{16}$/)
  })

  it('同构筑同分钟桶内结果可复现（seed 按分钟取整）', () => {
    const a = runChallenge(view(), deps)
    const b = runChallenge(view(), deps)
    expect(b.report.seed).toBe(a.report.seed)
    expect(b.report.replay_hash).toBe(a.report.replay_hash)
    expect(b.stats).toEqual(a.stats)
  })

  it('不同防线 id → 不同 seed（避免同组合结果恒定）', () => {
    // 刻意不传 seedOverride，验证的是**默认派生逻辑**。
    // 传了 override 就等于绕过这条规则，测不到东西。
    const { seedOverride: _omit, ...noOverride } = deps
    const a = runChallenge(view({ id: 7 }), noOverride)
    const b = runChallenge(view({ id: 8 }), noOverride)
    expect(b.report.seed).not.toBe(a.report.seed)
  })

  it('工程装置会改变攻方属性（slow_belt 提升元素系数）', () => {
    const withWorks = runChallenge(view(), deps)
    const without = runChallenge(
      view({ snapshot: { ...view().snapshot!, works: [] } }),
      deps,
    )
    // 装置改变攻方 → 哈希前缀改变
    expect(withWorks.report.replay_hash).not.toBe(without.report.replay_hash)
  })

  it('守恒不变量：击杀 + 漏怪恒等于该关总怪数（8 只）', () => {
    // 这条比"强弱对比"稳得多：不依赖具体战局，只依赖引擎的结算守恒。
    // 服务端 ValidateSettle 里 "kills ≤ 总怪数" 的校验依据正是同一性质。
    for (const atk of [0n, 1000n, 50000n]) {
      const r = runChallenge(view(), { ...deps, myAttacker: { ...attacker(), attack: atk } })
      expect(r.stats.kills + r.stats.leaked, `atk=${atk}`).toBe(8)
    }
  })

  it('攻击力有实际收益：攻方越强漏怪越少、耗时越短', () => {
    // 敌人 30000 血时，0‰ 攻方打不完会漏怪，50000‰ 攻方能全清。
    // 夹具选厚血是必要的：敌人太薄时刷怪节奏会掩盖攻方差异。
    const weak = runChallenge(view(), { ...deps, myAttacker: { ...attacker(), attack: 0n } })
    const strong = runChallenge(view(), { ...deps, myAttacker: { ...attacker(), attack: 50000n } })

    expect(weak.stats.leaked, '低攻方应打不完而有漏怪').toBeGreaterThan(0)
    expect(strong.stats.leaked, '高攻方应能全清').toBe(0)
    expect(strong.stats.kills).toBeGreaterThan(weak.stats.kills)
    expect(strong.stats.durationMs).toBeLessThan(weak.stats.durationMs)
    expect(strong.stats.score).toBeGreaterThan(weak.stats.score)
  })

  it('attack 是千分比：1‰ 与 100‰ 的差异本就不显著（不是缺陷）', () => {
    // 单发伤害公式 D = skill_damage × (1000 + attack) / 1000。
    // attack=1‰ → 100‰ 只把单发伤害从 120.1 抬到 132（+8%），
    // 不足以跨过"多打一发"的门槛，因此整局表现相同是**正常**的。
    //
    // 这条用例的作用是把上述认知钉死：将来若有人看到"小数值差异无效果"
    // 而误判为设计缺陷，这条断言会立刻暴露问题出在认知而非数值。
    const a1 = defaultAttacker()
    a1.attack = 1n
    const d1 = new Defender(1_000_000n, 0n, 0n)
    const r1 = resolveHit(a1, d1, { skillDamage: 120n, skillElement: 'fire', roll: 9999 })

    const a2 = defaultAttacker()
    a2.attack = 100n
    const d2 = new Defender(1_000_000n, 0n, 0n)
    const r2 = resolveHit(a2, d2, { skillDamage: 120n, skillElement: 'fire', roll: 9999 })

    // +8% 左右的差异，量级正确
    const ratio = Number(r2.totalDamage) / Number(r1.totalDamage)
    expect(ratio).toBeGreaterThan(1.05)
    expect(ratio).toBeLessThan(1.15)
  })
})

describe('辅助函数', () => {
  it('snapshotDigest 与顺序无关', () => {
    const a = snapshotDigest(['slow_belt', 'tesla_grid'], [3, 1, 2])
    const b = snapshotDigest(['tesla_grid', 'slow_belt'], [1, 2, 3])
    expect(a).toBe(b)
  })

  it('snapshotDigest 内容不同则不同', () => {
    expect(snapshotDigest(['slow_belt'], [1])).not.toBe(snapshotDigest(['tesla_grid'], [1]))
  })

  it('snapshotDigest 输出 16 位 hex', () => {
    expect(snapshotDigest([], [])).toMatch(/^[0-9a-f]{16}$/)
  })

  it('snapshotElements 过滤非法元素', () => {
    const v = view({ snapshot: { ...view().snapshot!, elements: ['fire', 'bogus', 'ice'] } })
    expect(snapshotElements(v)).toEqual(['fire', 'ice'])
  })

  it('snapshotElements 对缺失字段返回空数组', () => {
    expect(snapshotElements(view({ snapshot: undefined }))).toEqual([])
  })
})
