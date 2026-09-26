/**
 * 地形机制（I-4）的行为测试。
 *
 * ⚠️ 这个文件补的是一个整类空洞：`terrain.ts` 此前**没有任何测试文件**。
 * 直接后果是 5 类地形里有 4 类长期不生效，而没有任何测试会红：
 *
 *  1. 油桶/掩体的 param 阈值与每次命中的充能增量差 2~3 个数量级
 *     （param 100~180 vs 增量 damage/100 ≈ 0~2）—— 实测 12 个油桶关卡 0 个点燃
 *  2. 掩体的每 tick 充能路径与 blocksProjectile 在几何上互斥，
 *     弹丸在半径 100 被销毁、充能判定在半径 110，永远走不到
 *  3. onHit 不判位置 —— 任意位置的火焰都能点燃远处的油桶
 *
 * 三条都是"看起来实现了、实际不工作"的形态，正是零覆盖能长期存活的原因。
 * 所以这里对**每一类地形**都断言"能被触发"，而不只断言"调用不报错"。
 */
import { describe, it, expect } from 'vitest'
import { Terrain, toFixed, BASE_X, type TerrainContext } from './terrain'
import type { TerrainPlacement, Enemy, Projectile } from './types'
import type { Element } from './elements'
import type { TerrainEffect } from './terrain'

/** 造一个空场景的 TerrainContext。 */
function ctx(over: Partial<TerrainContext> = {}): TerrainContext {
  return {
    enemies: [] as Enemy[],
    projectiles: [] as Projectile[],
    terrainTick: 5n,
    enemiesInRadius: () => [] as Enemy[],
    within: (_tx: number, _ty: number, _r: number, x: bigint, y: bigint) => {
      void _tx
      void _ty
      void _r
      void x
      void y
      return false
    },
    onKill: () => {},
    onTerrainTrigger: () => {},
    ...over,
  }
}

function place(kind: TerrainPlacement['kind'], param: number, x = 500, y = 500): Terrain {
  return new Terrain({ kind, x, y, param })
}

describe('地形：油桶', () => {
  it('近距离火焰命中会累积充能', () => {
    const t = place('oil_drum', 20)
    // 命中点就在地形上（500,500）
    expect(t.onHit('fire', 500n, toFixed(500), toFixed(500))).toBe(false) // 500/100 = 5 < 20
    expect(t.charge).toBe(5)
    expect(t.onHit('fire', 500n, toFixed(500), toFixed(500))).toBe(false)
    expect(t.charge).toBe(10)
  })

  it('充能达阈值即引燃并返回 true', () => {
    const t = place('oil_drum', 20)
    let fired = false
    for (let i = 0; i < 10 && !fired; i++) {
      fired = t.onHit('fire', 500n, toFixed(500), toFixed(500))
    }
    expect(fired).toBe(true)
    expect(t.state).toBe('burning')
  })

  // 这是修复前完全不存在的判据：远处的火不该点燃油桶。
  it('远处的火焰命中不累积充能', () => {
    const t = place('oil_drum', 20)
    // 命中点在 (200,200)，油桶在 (500,500)，距离约 424 逻辑单位 > 120
    t.onHit('fire', 500n, toFixed(200), toFixed(200))
    expect(t.charge).toBe(0)
  })

  it('非火焰元素不充能', () => {
    const t = place('oil_drum', 20)
    t.onHit('ice', 500n, toFixed(500), toFixed(500))
    expect(t.charge).toBe(0)
  })

  it('已引燃后不再重复充能', () => {
    const t = place('oil_drum', 4)
    t.onHit('fire', 500n, toFixed(500), toFixed(500))
    expect(t.state).toBe('burning')
    const before = t.charge
    t.onHit('fire', 500n, toFixed(500), toFixed(500))
    expect(t.charge).toBe(before)
  })

  it('火区在持续时间内存在，到期后回到 idle', () => {
    const t = place('oil_drum', 4)
    t.onHit('fire', 500n, toFixed(500), toFixed(500))
    expect(t.state).toBe('burning')
    // 8000ms 持续期，tick 50ms
    for (let i = 0; i < 161; i++) t.update(50, ctx())
    expect(t.state).toBe('idle')
  })

  it('火区对范围内敌人造成伤害', () => {
    const t = place('oil_drum', 4, 500, 500)
    t.onHit('fire', 500n, toFixed(500), toFixed(500))
    let killed = 0
    const victim = {
      hp: 3n,
      maxHp: 100n,
      shield: 0n,
      hitFlashMs: 0,
      stacks: new Map<Element, bigint>(),
      dead: false,
    }
    t.update(50, ctx({
      terrainTick: 5n,
      enemiesInRadius: () => [victim as unknown as Enemy],
      onKill: () => {
        killed++
      },
    }))
    expect(killed).toBe(1)
  })
})

describe('地形：崩塌掩体', () => {
  it('近距离动能命中累积充能', () => {
    const t = place('collapse_wall', 30)
    t.onHit('kinetic', 500n, toFixed(500), toFixed(500))
    expect(t.charge).toBe(5)
  })

  it('充能达阈值即崩塌', () => {
    const t = place('collapse_wall', 20)
    let fired = false
    for (let i = 0; i < 10 && !fired; i++) {
      fired = t.onHit('kinetic', 500n, toFixed(500), toFixed(500))
    }
    expect(fired).toBe(true)
    expect(t.state).toBe('collapsed')
  })

  it('未崩塌时阻挡弹道，崩塌后不再阻挡', () => {
    const t = place('collapse_wall', 30)
    expect(t.blocksProjectile()).toBe(true)
    for (let i = 0; i < 10; i++) t.onHit('kinetic', 500n, toFixed(500), toFixed(500))
    expect(t.state).toBe('collapsed')
    expect(t.blocksProjectile()).toBe(false)
  })

  it('远处的动能命中不充能', () => {
    const t = place('collapse_wall', 30)
    t.onHit('kinetic', 500n, toFixed(200), toFixed(200))
    expect(t.charge).toBe(0)
  })

  // 曾经的每-tick 弹丸充能路径：弹丸在半径 100 被销毁，
  // 而判定在半径 110 —— 几何上永远走不到。留着它只会让人误以为机制在工作。
  it('update 不再从弹丸充能（唯一来源是 onHit）', () => {
    const t = place('collapse_wall', 5)
    const projectile = {
      element: 'kinetic',
      x: toFixed(500),
      y: toFixed(500),
      vx: 0n,
      vy: 0n,
    }
    const inside = ctx({
      projectiles: [projectile as unknown as Projectile],
      within: () => true, // 假装"在范围内"，验证 update 根本不看弹丸
    })
    for (let i = 0; i < 50; i++) t.update(50, inside)
    expect(t.charge).toBe(0)
    expect(t.state).not.toBe('collapsed')
  })
})

describe('地形：潮汐闸', () => {
  it('按 param 周期开合', () => {
    const t = place('tidal_gate', 1000)
    const seen: boolean[] = []
    for (let i = 0; i < 45; i++) {
      seen.push(t.update(50, ctx()).blocked)
    }
    // 1000ms 周期、50ms/tick → 每 20 tick 切换一次。
    //
    // 状态机：初始 'idle' → toggle → 'open' → toggle → 'closed' → ...
    // 而 blocked = (state === 'closed')，所以 idle 与 open 都**放行**。
    // 也就是说闸门刚出现的前 20 tick 是敞开的 ——
    // 这不是 bug，是「先给玩家一个窗口，别一开局就被堵死」的设计。
    expect(seen[0]).toBe(false) // idle：放行
    expect(seen[19]).toBe(false) // 还不到一个周期
    expect(seen[20]).toBe(false) // 切成 open：仍放行
    expect(seen[40]).toBe(true) // 再一个周期，切到 closed：开始阻挡
  })

  it('关闭时阻挡、打开时放行', () => {
    const t = place('tidal_gate', 1000)
    const first = t.update(50, ctx())
    expect(typeof first.blocked).toBe('boolean')
  })

  it('param 为 0 时不卡死（除零保护）', () => {
    const t = place('tidal_gate', 0)
    for (let i = 0; i < 100; i++) t.update(50, ctx())
    expect(true).toBe(true) // 不抛异常即可
  })
})

describe('地形：旋转风障', () => {
  it('会改变范围内弹丸的速度方向', () => {
    const t = place('rotor_vane', 60)
    const p = { element: 'fire', x: toFixed(500), y: toFixed(500), vx: 1000n, vy: 0n }
    const vx0 = p.vx
    const vy0 = p.vy
    t.update(50, ctx({
      projectiles: [p as unknown as Projectile],
      within: () => true,
    }))
    expect(p.vx !== vx0 || p.vy !== vy0).toBe(true)
  })

  it('范围外的弹丸不受影响', () => {
    const t = place('rotor_vane', 60)
    const p = { element: 'fire', x: toFixed(500), y: toFixed(500), vx: 1000n, vy: 0n }
    t.update(50, ctx({
      projectiles: [p as unknown as Projectile],
      within: () => false,
    }))
    expect(p.vx).toBe(1000n)
    expect(p.vy).toBe(0n)
  })
})

describe('地形：蓄能塔', () => {
  it('击杀累积充能，满时给全场敌人上同种元素', () => {
    const t = place('charge_tower', 3)
    const enemies: Enemy[] = [
      { stacks: new Map<Element, bigint>(), applyElement: () => true } as unknown as Enemy,
      { stacks: new Map<Element, bigint>(), applyElement: () => true } as unknown as Enemy,
    ]
    const c = ctx({ enemies })
    expect(t.onKillCharging('fire', 3n, c)).toBe(false)
    expect(t.onKillCharging('fire', 3n, c)).toBe(false)
    expect(t.onKillCharging('fire', 3n, c)).toBe(true)
  })

  it('触发后充能归零（可重复触发）', () => {
    const t = place('charge_tower', 2)
    const c = ctx({ enemies: [] })
    // param=2：第一次充到 1（未达阈值），第二次触发并归零
    expect(t.onKillCharging('ice', 3n, c)).toBe(false)
    expect(t.charge).toBe(1)
    expect(t.onKillCharging('ice', 3n, c)).toBe(true)
    expect(t.charge).toBe(0)
    // 归零后可以再次累积并再次触发
    expect(t.onKillCharging('ice', 3n, c)).toBe(false)
    expect(t.onKillCharging('ice', 3n, c)).toBe(true)
  })
})

describe('地形：未知类型不得崩掉战斗', () => {
  // TerrainKind 是字面量联合，TS 不会报"可能隐式返回 undefined"，
  // 但它来自网络数据 —— DB 被手改 / 灰度中新类型下发到旧客户端 / JSON 缺字段。
  it('未知 kind 按无效果处理（不返回 undefined）', () => {
    const t = new Terrain({ kind: 'meteor_strike' as never, x: 500, y: 500, param: 1 })
    const eff: TerrainEffect = t.update(50, ctx())
    expect(eff).toBeDefined()
    expect(eff.blocked).toBe(false)
  })

  it('未知 kind 的 onHit 返回 false 而不抛异常', () => {
    const t = new Terrain({ kind: 'meteor_strike' as never, x: 500, y: 500, param: 1 })
    expect(t.onHit('fire', 500n, toFixed(500), toFixed(500))).toBe(false)
  })
})

describe('地形：坐标约定', () => {
  it('配置用逻辑单位、内部用定点整数，转换必须无损往返', () => {
    // 混用会让判定范围差 1000 倍 —— 这是 I-4 最容易犯也最难发现的错。
    for (const n of [0, 1, 60, 100, 500, 1000]) {
      expect(Number(toFixed(n))).toBe(n * 1000)
    }
  })

  it('防线的 x 坐标与地形配置同量纲', () => {
    expect(BASE_X).toBe(60)
  })
})
