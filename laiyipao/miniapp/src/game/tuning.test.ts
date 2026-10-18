/**
 * 节奏配置的守卫（第 158 轮）。
 *
 * ## 判据分两级，与项目既有约定一致
 *
 * | 级别 | 判据 | 能证明什么 |
 * |---|---|---|
 * | 形状 | 字段齐全、取值在合理区间 | 参数表没写坏 |
 * | 行为 | 同一批关卡 DEFAULT 比 LEGACY **确实更快**，且不引入失败关 | 节奏真的变快了 |
 *
 * 只做第一级是不够的：把 `waveDelayPermille` 写成 1000（等于没提速）同样
 * 能通过所有形状断言。**「快了没有」只能由行为判定。**
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import vectors from '@vectors/formula_vectors.json'
import { BattleEngine, MAX_BATTLE_TICKS, type BattleConfig } from './engine'
import { defaultAttacker } from './damage'
import { ACTIVE_SLOTS } from './heatmap'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'
import { DEFAULT_TUNING, LEGACY_TUNING, type GameTuning } from './tuning'

const allLevels = (fixture.levels as unknown as GeneratedLevel[]).slice().sort((a, b) => a.id - b.id)
const enemies = new Map<number, EnemyDef>(
  (fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]),
)
const skills = new Map<number, SkillDef>(
  [
    ...(fixture.skills as unknown as SkillDef[]),
    ...(fixture.composite_skills as unknown as SkillDef[]),
  ].map((s) => [s.id, s]),
)

function equipped() {
  const active = [...skills.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, ACTIVE_SLOTS)
  return active.map((s, i) => ({
    skillId: s.id,
    name: s.name,
    element: s.element as Element,
    kind: s.kind,
    heatCost: BigInt(s.heat_cost),
    cooldownMs: s.cooldown_ms,
    pierce: s.pierce,
    aoeRadius: s.aoe_radius,
    baseDamage: BigInt(s.base_damage),
    applyElement: (s.apply_element ?? '') as Element | '',
    applyStacks: BigInt(s.apply_stacks),
    projectileSpeed: s.projectile_speed,
    chain: s.chain,
    slot: i,
    cooldownRemaining: 0,
  }))
}

function play(lv: GeneratedLevel, tuning: GameTuning) {
  const cfg: BattleConfig = {
    level: lv,
    enemies,
    skills,
    equipped: equipped(),
    attacker: defaultAttacker(),
    seed: 12345,
    tuning,
  }
  const e = new BattleEngine(cfg)
  e.start()
  for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
  }
  return { phase: e.phase, tick: e.tick }
}

/** 采样关卡：覆盖前中后段，全部 100 关太慢（实测 behavior 断言会到 10s+）。 */
const SAMPLES = [1, 5, 10, 20, 40, 60, 80, 90, 95, 100]

describe('节奏配置的形状', () => {
  it('DEFAULT 与 LEGACY 的字段集合完全一致（加字段时两侧都要补）', () => {
    const a = Object.keys(DEFAULT_TUNING).sort()
    const b = Object.keys(LEGACY_TUNING).sort()
    expect(a.length).toBeGreaterThan(0)
    expect(a.join(',')).toBe(b.join(','))
  })

  it('所有倍率为正、所有时长为正', () => {
    for (const t of [DEFAULT_TUNING, LEGACY_TUNING]) {
      for (const [k, v] of Object.entries(t)) {
        expect(v, `${k} 必须为正`).toBeGreaterThan(0)
      }
    }
  })

  /**
   * ⚠️ 这条是 `determinism.test.ts` 那条铁律的**镜像守卫**。
   *
   * 那边断言「13 个 tick 达成满出场」；这里断言「配置层不许把它改成
   * 能整除的数」。两边守的是同一件事，但从两个方向夹住：
   * 改常量那边会红，改配置这边也会红。
   */
  it('出场步进不得改成能整除 FULL 的值（否则存量战报哈希失配）', () => {
    const FULL = 1000
    for (const t of [DEFAULT_TUNING, LEGACY_TUNING]) {
      expect(FULL % t.spawnProgressStep, `spawnProgressStep=${t.spawnProgressStep} 会整除 FULL`).not.toBe(0)
    }
    expect(DEFAULT_TUNING.spawnProgressStep).toBe(LEGACY_TUNING.spawnProgressStep)
  })
})

describe('节奏配置的跨端钉（第 159 轮）', () => {
  /**
   * ⚠️ 第 159 轮事故的直接解药。
   *
   * 事故链的另一环是 **Go 侧从未同步 158 轮的 tuning**：
   * 客户端跑 DEFAULT_TUNING，服务端重放用「整改前」的编译期常量，
   * 两端时间线整体错位（首个开火差 6 tick、逐波累积 407 tick），
   * 而存量向量是 158 轮前生成的旧产物 —— Go 与旧向量互相自洽，
   * 全体测试绿灯，线上 100% 拒发。
   *
   * 这里读 `formula_vectors.json` 的 `tuning` 段（Go 侧
   * `TestConsistencyTuning` 读同一份），断言 DEFAULT_TUNING 逐位一致。
   * 任一端单方面改数值 → 两端必有一方红。
   */
  it('DEFAULT_TUNING 与跨端契约向量逐位一致', () => {
    const t = (vectors as unknown as { tuning: Record<string, number> }).tuning
    expect(t, 'formula_vectors.json 缺 tuning 段').toBeDefined()
    const checks: Array<[string, number]> = [
      ['spawnProgressStep', t.spawn_progress_step],
      ['enemySpeedPermille', t.enemy_speed_permille],
      ['projectileSpeedPermille', t.projectile_speed_permille],
      ['skillCooldownPermille', t.skill_cooldown_permille],
      ['waveDelayPermille', t.wave_delay_permille],
      ['waveIntervalPermille', t.wave_interval_permille],
      ['enemyAttackIntervalPermille', t.enemy_attack_interval_permille],
      ['overheatDurationMs', t.overheat_duration_ms],
      ['hitFlashMs', t.hit_flash_ms],
      ['floatLifeMs', t.float_life_ms],
    ]
    for (const [field, want] of checks) {
      expect(DEFAULT_TUNING[field as keyof typeof DEFAULT_TUNING], `${field} 与契约向量不一致`).toBe(want)
    }
  })
})

describe('节奏配置的行为判据', () => {
  it('DEFAULT 相比 LEGACY 确实更快（提速 ≥ 10%），且不引入失败关', () => {
    let base = 0
    let now = 0
    const failedNow: number[] = []
    const failedBase: number[] = []
    for (const id of SAMPLES) {
      const lv = allLevels.find((l) => l.id === id)!
      const b = play(lv, LEGACY_TUNING)
      const n = play(lv, DEFAULT_TUNING)
      base += b.tick
      now += n.tick
      if (n.phase !== 'won') failedNow.push(id)
      if (b.phase !== 'won') failedBase.push(id)
    }

    const gain = (base - now) / base
    console.log(`  采样 ${SAMPLES.length} 关：LEGACY ${base} tick → DEFAULT ${now} tick（-${(gain * 100).toFixed(1)}%）`)
    console.log(`  失败关：LEGACY [${failedBase.join(',')}] → DEFAULT [${failedNow.join(',')}]`)

    // 前提守卫：LEGACY 一档必须真的跑出了 tick，否则 gain 会退化成 NaN/0，
    // 而 `0 >= 0.1` 与 `NaN >= 0.1` 都是 false —— 断言仍会红，但红得莫名其妙。
    // 更危险的是若 now 为 0（引擎停摆），gain = 1 会**恒绿**。必须排除。
    expect(base).toBeGreaterThan(0)
    expect(now).toBeGreaterThan(0)

    expect(gain).toBeGreaterThanOrEqual(0.1)

    // 提速不能以「把关卡变得打不过」为代价 —— 与 balance.probe 同一条原则，
    // 只是这里守的是「相对基线不得新增失败关」这个更窄的命题。
    const added = failedNow.filter((id) => !failedBase.includes(id))
    expect(added.join(',')).toBe('')
  }, 300_000)
})
