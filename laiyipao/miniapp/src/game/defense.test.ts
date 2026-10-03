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

/** 在默认守卫模板上覆写部分字段，快速造弱怪/强怪。 */
function mkEnemyDef(id: number, over: Partial<EnemyDef>): EnemyDef {
  return { ...ENEMIES[0], id, ...over }
}

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
  // 三项本轮新增的攻方字段（防线护甲 / 热量上限 / 机制卡强度）。
  // 刻意给 armorPermille 非零，让这个测试同时覆盖"装备护甲真的减免漏怪伤害"。
  heatCapPermille: 0n,
  armorPermille: 200n,
  mechanicPermille: 0n,
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

  it('工程装置 block_wall / tesla_grid 各自生效（不只是 slow_belt）', () => {
    // applyWorks 的 switch 有三个 case，此前只被 slow_belt 走到过 ——
    // 另两个装置是「看起来实现了」。装置折算成攻方属性后进入 replayHash
    // 前缀（block_wall→elementCap、tesla_grid→reactionMultPermille），
    // 「哈希改变」是装置生效的确定性证据。
    //
    // ⚠️ 关键：block_wall 折算的 elementCap 在 currentAttacker 里会被
    // **关卡自带的 element_cap 覆盖**（`lv.element_cap ? lv值 : base值`）——
    // 默认 level() 有 element_cap:3，block_wall 的 +1 会被丢掉、装置形同虚设。
    // 所以这里必须在 element_cap 未固定（0）的关卡上验证 block_wall，
    // 否则测的是「被覆盖后的无效果」。这条覆盖本身就是该设计约束的记录。
    const openCapLevel = { ...level(), element_cap: 0 }
    const capDeps = { ...deps, level: openCapLevel }
    const without = runChallenge(
      view({ snapshot: { ...view().snapshot!, works: [] } }),
      capDeps,
    )
    const blockWall = runChallenge(
      view({ snapshot: { ...view().snapshot!, works: ['block_wall'] } }),
      capDeps,
    )
    const teslaGrid = runChallenge(
      view({ snapshot: { ...view().snapshot!, works: ['tesla_grid'] } }),
      capDeps,
    )
    expect(blockWall.report.replay_hash).not.toBe(without.report.replay_hash)
    expect(teslaGrid.report.replay_hash).not.toBe(without.report.replay_hash)
    // 两种装置折算的是不同属性，彼此也不能等值
    expect(blockWall.report.replay_hash).not.toBe(teslaGrid.report.replay_hash)
  })

  it('block_wall 的护甲折算被关卡 element_cap 覆盖（记录该设计约束）', () => {
    // 在固定了 element_cap 的关卡（所有真实关卡都固定）上，block_wall
    // 对 elementCap 的 +1 会被 currentAttacker 丢弃，装置不改变战局。
    // 这不是断言"装置该失效"，而是把「当前实现下它确实失效」钉住，
    // 以后若接上真实护甲通道，这条会红并提示更新。
    const without = runChallenge(
      view({ snapshot: { ...view().snapshot!, works: [] } }),
      deps,
    )
    const blockWall = runChallenge(
      view({ snapshot: { ...view().snapshot!, works: ['block_wall'] } }),
      deps,
    )
    expect(blockWall.report.replay_hash).toBe(without.report.replay_hash)
  })

  it('快照缺 works 字段时按无装置处理（老快照兼容）', () => {
    // 服务端早期快照没有 works 字段。挑战不能因此失败 ——
    // 「没有装置」与「缺字段」语义等价。
    const r = runChallenge(
      view({ snapshot: { ...view().snapshot!, works: undefined as unknown as string[] } }),
      deps,
    )
    expect(r.error).toBeUndefined()
    expect(r.report.replay_hash).toMatch(/^[0-9a-f]{16}$/)
  })

  it('关卡血量为 0 的畸形数据：hp_left_pct 为 0 而不是 NaN', () => {
    // base_hp=0 时 hpMax<=0，除法会得 NaN 并污染上报。这里钉住
    // 「畸形关卡上报 0%」的兜底行为。敌人特意无攻击力 ——
    // 漏怪伤害走 breachDamage 的 baseHpMax<=0 兜底（100 点），
    // 让引擎侧与上报侧的同一条畸形数据路径都被走到。
    //
    // ⚠️ 第 65 轮：敌人血量从夹具默认值提到 40000。
    //
    // 原来只靠 `attack: 0n` 制造「打不完」，而第 65 轮修好弹射/溅射后
    // 战斗整体上移 —— 即使 0 攻方也能把默认血量的怪全清掉，
    // 于是 `won` 变成 true，这条用例测的就不再是「防线被突破」那条路径。
    //
    // 这是本项目记过多次的形态：**夹具的参数温和度本身就是一种掩盖**。
    // 这里不去改断言（hp_left_pct 必须是 0 的结论仍然正确），
    // 而是让夹具真的「打不完」—— 血量高到 0 攻方 + 4 个技能也清不掉。
    const r = runChallenge(view(), {
      ...deps,
      level: { ...level(), base_hp: 0 },
      enemies: new Map([[1, mkEnemyDef(1, { attack: 0, hp: 40_000 })]]),
      myAttacker: { ...attacker(), attack: 0n }, // 保证有怪漏进防线
    })
    expect(r.won).toBe(false)
    expect(r.report.hp_left_pct).toBe(0)
    expect(r.stats.hpLeftPct).toBe(0)
  })

  it('两波关卡：波间隙的 card_select 由无人值守循环自动跳过', () => {
    // 防线挑战没有玩家在场 —— runToEnd 必须在 card_select 阶段
    // 替玩家跳过整波，否则战斗会永远停在选牌界面（直到 25 分钟
    // 上限被强制判负）。这条用例就是那条「必须能跨过波间隙」的契约。
    const weakEnemy = mkEnemyDef(1, { hp: 30, speed: 10000 })
    const twoWaveDeps = {
      ...deps,
      enemies: new Map([[1, weakEnemy]]),
      level: {
        ...level(),
        base_hp: 100_000,
        wave_count: 2,
        waves: [
          { wave_index: 0, spawns: [{ enemy_id: 1, count: 2, interval: 60, delay: 0 }] },
          { wave_index: 1, spawns: [{ enemy_id: 1, count: 2, interval: 60, delay: 0 }] },
        ],
      },
    }
    const r = runChallenge(view(), twoWaveDeps)
    expect(r.error).toBeUndefined()
    expect(r.won).toBe(true)
    // 守恒：两波共 4 只，全部被处理
    expect(r.stats.kills + r.stats.leaked).toBe(4)
    expect(r.stats.waves).toBe(2)
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
    // 夹具必须是**厚血敌人**，否则这条测不到攻方差异 ——
    // 敌人太薄时刷怪节奏会掩盖攻方差异（README 记过这个坑）。
    //
    // ⚠️ 第 65 轮：必须**显式**给血量。
    //
    // 原来直接用 `deps.enemies`（真实内容表），注释写着「敌人 30000 血时」，
    // 但那是**愿望**不是事实 —— deps 里用的是真实敌人表，血量各不相同。
    // 第 65 轮修好弹射/溅射后战斗整体上移，0 攻方也能把默认厚度的怪清掉，
    // 于是 `weak.leaked > 0` 断言失败。
    //
    // 所以这里把血量钉到 60_000：0‰ 攻方 + 4 技能打不完（会漏怪），
    // 50000‰ 攻方能全清 —— 攻方差异因此可观测。
    const thick = new Map(ENEMIES.map((e) => [e.id, { ...e, hp: 60_000 }]))
    const weak = runChallenge(view(), {
      ...deps,
      enemies: thick,
      myAttacker: { ...attacker(), attack: 0n },
    })
    const strong = runChallenge(view(), {
      ...deps,
      enemies: thick,
      myAttacker: { ...attacker(), attack: 50000n },
    })

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

  it('snapshotDigest 对缺失 works（老快照/脏数据）按空装置计算，不抛错', () => {
    // 为什么钉：works 静态类型是 string[]，但摘要的入参可能来自缺字段的
    // 老快照（见 runChallenge 的 `?? []` 兼容）。这里直接给 undefined，
    // 触发 `[...(works ?? [])]` 的空数组回退分支——摘要必须仍是合法 16 位 hex，
    // 且与"显式传空数组"结果一致（否则同一份构筑的本地校验哈希会漂移）。
    const withUndef = snapshotDigest(undefined as unknown as string[], [1, 2])
    expect(withUndef).toMatch(/^[0-9a-f]{16}$/)
    expect(withUndef).toBe(snapshotDigest([], [1, 2]))
  })

  it('snapshotElements 过滤非法元素', () => {
    const v = view({ snapshot: { ...view().snapshot!, elements: ['fire', 'bogus', 'ice'] } })
    expect(snapshotElements(v)).toEqual(['fire', 'ice'])
  })

  it('snapshotElements 对缺失字段返回空数组', () => {
    expect(snapshotElements(view({ snapshot: undefined }))).toEqual([])
  })
})
