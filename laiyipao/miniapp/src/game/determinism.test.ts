/**
 * 定点整数铁律的守卫。
 *
 * ⚠️ 这里的用例有一个共同特点：**它们断言的是"数值形态"而不是"数值结果"**。
 *
 * 普通的引擎测试断言「这一关能通关」「这个击杀数对」，
 * 而把 `spawnProgress` 从整数千分比改回浮点（`+= 0.08`，上限 1.0）之后，
 * **全部 393 个测试照样绿** —— 我实际做过这个变异。
 *
 * 原因是可预期的：浮点版本在本机同一份 V8 上算出的是**同一个数**，
 * 所以行为完全一致。而 README 第 2 条铁律关心的不是"本机一致"，
 * 而是「**跨实现一致**」—— 不同 V8 版本、不同 CPU 架构的浮点中间精度
 * 可能差 1 ulp，那 1 ulp 就足以让某个边界判定落到不同一侧。
 *
 * 所以唯一的判据是**形态断言**：
 * 值必须是整数、必须是 STEP 的整数倍、必须落在闭区间内。
 * 这三条一旦被违反，说明有人重新引入了浮点累加。
 *
 * 为什么这值得单独一个文件：这类缺陷的共同点是
 * 「本机永远测不出来，只有换机器/换版本才会炸」，
 * 而那种缺陷恰恰是最该在上线前抓住的。
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import {
  BattleEngine,
  TICK_MS,
  SPAWN_PROGRESS_FULL,
  SPAWN_PROGRESS_STEP,
  type BattleConfig,
} from './engine'
import { defaultAttacker } from './damage'
import { ACTIVE_SLOTS } from './heatmap'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import type { Element } from './elements'

const level = (fixture.levels as unknown as GeneratedLevel[]).find((l) => l.id === 1)!
const enemyMap = new Map<number, EnemyDef>(
  (fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]),
)
const skillMap = new Map<number, SkillDef>(
  [
    ...(fixture.skills as unknown as SkillDef[]),
    ...(fixture.composite_skills as unknown as SkillDef[]),
  ].map((s) => [s.id, s]),
)

function mk(): BattleEngine {
  const active = [...skillMap.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, ACTIVE_SLOTS)
  const cfg: BattleConfig = {
    level,
    enemies: enemyMap,
    skills: skillMap,
    equipped: active.map((s, i) => ({
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
    })),
    attacker: defaultAttacker(),
    seed: 7,
  }
  return new BattleEngine(cfg)
}

describe('铁律：出场进度必须是整数千分比', () => {
  it('常量本身是整数（浮点常量的 typeof 仍是 number，靠值断言区分）', () => {
    expect(Number.isInteger(SPAWN_PROGRESS_FULL)).toBe(true)
    expect(Number.isInteger(SPAWN_PROGRESS_STEP)).toBe(true)
    expect(SPAWN_PROGRESS_STEP).toBeGreaterThan(0)
    expect(SPAWN_PROGRESS_FULL).toBeGreaterThan(SPAWN_PROGRESS_STEP)
  })

  it('单个敌人的推进过程中，进度始终是 STEP 的整数倍', () => {
    const e = mk()
    e.start()
    let sawSpawning = false
    for (let t = 0; t < 400; t++) {
      e.step()
      for (const en of e.enemies) {
        const p = en.spawnProgress
        // 形态断言 1：整数（浮点版本在这里就红）
        expect(Number.isInteger(p)).toBe(true)
        // 形态断言 2：要么是 STEP 的整数倍（正在按 tick 推进），
        // 要么**恰好等于** FULL（Math.min 的钳位结果）。
        //
        // ⚠️ 最初写成 `p % STEP === 0`，结果连钳位值都判不过：
        // 1000 % 80 = 40。钳位值不是 STEP 的倍数是**正常的**
        // —— FULL/STEP = 12.5，本来就不能整除。
        // 漏掉这个分支时，测试自己先红了。
        const atCap = p === SPAWN_PROGRESS_FULL
        expect(atCap || p % SPAWN_PROGRESS_STEP === 0).toBe(true)
        // 值域断言
        expect(p).toBeGreaterThanOrEqual(0)
        expect(p).toBeLessThanOrEqual(SPAWN_PROGRESS_FULL)
        if (p < SPAWN_PROGRESS_FULL) sawSpawning = true
      }
      if (sawSpawning && e.enemies.every((x) => x.spawnProgress >= SPAWN_PROGRESS_FULL)) break
    }
    // 前提：这个用例真的观察到了"正在出场"的状态，否则它在空转
    expect(sawSpawning).toBe(true)
  })

  it('推进 tick 数与满进度的关系是精确的整数关系', () => {
    // FULL / STEP = 12.5 不是整数，所以是 13 个 tick 达到满。
    // 这个"13"是从旧浮点行为逐位继承来的（0.08 × 12 = 0.96 < 1）。
    // 断言它是为了防止有人把 STEP 调成能整除的数 ——
    // 那会让所有"敌人何时可被击中"的时刻平移一位，存量战报哈希失配。
    const ticksToFull = Math.ceil(SPAWN_PROGRESS_FULL / SPAWN_PROGRESS_STEP)
    expect(ticksToFull).toBe(13)
    expect((ticksToFull - 1) * SPAWN_PROGRESS_STEP).toBeLessThan(SPAWN_PROGRESS_FULL)
    expect(ticksToFull * SPAWN_PROGRESS_STEP).toBeGreaterThanOrEqual(SPAWN_PROGRESS_FULL)
  })

  it('达到满进度后不再增长（不会溢出到 1 以上）', () => {
    const e = mk()
    e.start()
    for (let t = 0; t < 2000; t++) {
      e.step()
      for (const en of e.enemies) {
        expect(en.spawnProgress).toBeLessThanOrEqual(SPAWN_PROGRESS_FULL)
      }
      if (e.phase === 'won' || e.phase === 'lost') break
    }
  })
})

describe('铁律：PRNG 只走 BattleRng，禁用 Math.random', () => {
  /**
   * 剥掉 JS/TS 注释，保留可执行代码。
   *
   * 目的是让「源码里没有 Math.random 调用」这条判据**不会被注释本身触发** ——
   * lcg.ts 的文件头就写着「本模块禁止使用 Math.random()」，
   * 而那条警告恰恰是最该保留的。
   *
   * 实现上刻意保持简单（逐行状态机 + 行注释处理），不做完整词法分析：
   * 测试辅助代码不需要正确处理字符串字面量里的 `//`，
   * 而"简单实现"必须配一条**自检用例**（见下），否则它自己坏了也没人知道。
   */
  function stripComments(src: string): string {
    const out: string[] = []
    let inBlock = false
    for (const raw of src.split('\n')) {
      let line = raw
      if (inBlock) {
        const end = line.indexOf('*/')
        if (end < 0) continue // 整行都在块注释里
        line = line.slice(end + 2)
        inBlock = false
      }
      // 处理行内 /* */ 与行注释 //
      let guard = 0
      while (guard++ < 64) {
        const bStart = line.indexOf('/*')
        const lStart = line.indexOf('//')
        if (bStart < 0 && lStart < 0) break
        if (bStart >= 0 && (lStart < 0 || bStart < lStart)) {
          const bEnd = line.indexOf('*/', bStart + 2)
          if (bEnd < 0) {
            line = line.slice(0, bStart)
            inBlock = true
            break
          }
          line = line.slice(0, bStart) + ' ' + line.slice(bEnd + 2)
        } else {
          line = line.slice(0, lStart)
          break
        }
      }
      out.push(line)
    }
    return out.join('\n')
  }

  it('源码里没有 Math.random 调用（防回归）', async () => {
    // 这是"形态断言"的极端形态：直接查源码文本。
    // 单元测试能验证 `BattleRng` 产出确定序列，
    // 但**无法**验证「没有别处偷偷调 Math.random」——
    // 而后者恰恰是 I-6 失效最常见的方式（某个新写的辅助函数顺手用了）。
    //
    // ⚠️ 判据必须是「没有**调用**」而不是「没有**出现**」：
    // lcg.ts 的文件头注释里就写着「本模块禁止使用 Math.random()」
    // —— 那条警告本身包含这个词。最早写成 `not.toContain('Math.random')`，
    // 结果**被自己的注释判失败**。
    //
    // 所以先剥掉注释（行注释与块注释），只对可执行代码断言。
    // 逐行剥离的另一个好处：它对「字符串字面量里的 Math.random」
    // 仍然敏感 —— 那同样值得警惕（比如拼进日志文案或 seed 派生）。
    const src = await import('./lcg?raw')
    const code = stripComments(String(src.default))
    expect(code).not.toContain('Math.random')
  })

  it('engine.ts 也没有 Math.random 调用', async () => {
    const src = await import('./engine?raw')
    expect(stripComments(String(src.default))).not.toContain('Math.random')
  })

  it('剥注释的实现本身有效（用一段已知含注释的样本自检）', () => {
    // 守卫自身也要被守：如果 stripComments 写坏了（比如没处理行注释），
    // 上面两条会**假绿**。所以这里用一个最小样本验证它确实在剥。
    expect(stripComments('a // Math.random\nb')).not.toContain('Math.random')
    expect(stripComments('/** Math.random */ a')).not.toContain('Math.random')
    expect(stripComments('/*\n * Math.random\n */\na')).not.toContain('Math.random')
    // 反向自检：真正代码里的必须还在
    expect(stripComments('f(Math.random)')).toContain('Math.random')
  })

  it('确定序列可复现（同 seed 两次产生完全相同的状态链）', () => {
    const run = (): string => {
      const e = mk()
      e.start()
      const marks: string[] = []
      for (let t = 0; t < 600; t++) {
        e.step()
        if (t % 100 === 0) {
          marks.push(
            `${e.tick}:${e.shots}:${e.hits}:${e.kills}:${e.score}:${e.baseHp.toString()}`,
          )
        }
      }
      return marks.join('|')
    }
    // 同一份配置跑两次必须逐位一致。
    // 若某处混入了 Math.random 或 Date.now()，这条会红。
    expect(run()).toBe(run())
  })
})

describe('铁律：时间推进是整数 tick', () => {
  it('TICK_MS 是整数（50ms 定步长）', () => {
    expect(Number.isInteger(TICK_MS)).toBe(true)
    expect(TICK_MS).toBe(50)
  })

  it('推进 100 个 tick 后引擎时间正好是 TICK_MS × 100', () => {
    const e = mk()
    e.start()
    for (let t = 0; t < 100; t++) e.step()
    // 若 tick 与毫秒之间引入了浮点换算，这里会出现非整数
    expect(Number.isInteger(e.elapsedMs)).toBe(true)
    expect(e.elapsedMs).toBe(TICK_MS * 100)
  })
})
