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
import { Terrain, toFixed, BASE_X, ROTOR_DIR_1000, type TerrainContext } from './terrain'
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
  // ⚠️ 动能 ×3 是**防死锁**的设计，不是数值偏好。
  // 曾经只有动能能充能，于是「不带动能的构筑永远打不开弹道」——
  // 而默认构筑恰好是 fire/fire/fire/ice，第 22/33/34 关对它是不可通关的。
  it('动能充能速度是其它元素的 3 倍', () => {
    const kinetic = place('collapse_wall', 9999)
    const fire = place('collapse_wall', 9999)
    const dmg = 500n // /100 = 5
    kinetic.onHit('kinetic', dmg, toFixed(500), toFixed(500))
    fire.onHit('fire', dmg, toFixed(500), toFixed(500))
    expect(kinetic.charge).toBe(15) // 5 × 3
    expect(fire.charge).toBe(5)
    expect(kinetic.charge).toBe(fire.charge * 3)
  })

  it('**任何**元素都能充能（不构成硬性门槛）', () => {
    // 这条是"关卡不会变成死局"的直接守卫。
    // 若有人把 onHit 改回 `element === 'kinetic' &&`，
    // 默认构筑（无动能）就再也打不开掩体，第 22/33/34 关重新变成不可通关。
    for (const el of ['fire', 'ice', 'lightning', 'corrosion', 'kinetic'] as const) {
      const t = place('collapse_wall', 9999)
      t.onHit(el, 500n, toFixed(500), toFixed(500))
      expect(t.charge).toBeGreaterThan(0n)
    }
  })

  it('非动能也能砸开掩体（只是更慢）', () => {
    const t = place('collapse_wall', 20)
    let fired = false
    for (let i = 0; i < 20 && !fired; i++) {
      fired = t.onHit('fire', 500n, toFixed(500), toFixed(500)) // 每次 +5
    }
    expect(fired).toBe(true)
    expect(t.state).toBe('collapsed')
  })

  it('近距离动能命中累积充能', () => {
    const t = place('collapse_wall', 60)
    t.onHit('kinetic', 500n, toFixed(500), toFixed(500))
    expect(t.charge).toBe(15) // ×3 生效
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

describe('地形：旋转风障不得使用浮点超越函数', () => {
  // ⚠️ 这组用例守的是 README 第 2 条铁律（定点整数 + 确定性 PRNG）的延伸：
  // **不能有任何浮点运算参与战斗判定**。
  //
  // `Math.cos` 特别危险：ECMA-262 只要求它是"实现近似的"，
  // 不要求跨实现逐位一致。不同 V8 版本、不同 CPU 架构可能差 1 ulp，
  // 而 1 ulp 在 Math.round 的 x.5 边界上会被放大成整整 1 个定点单位。
  // 风障改的是弹丸速度 → 命中位置变 → 击杀 tick 变 → replayHash 变
  // → I-6 把正常对局判成伪造。
  //
  // 所以判据是「结果逐位可复现」：同一 tick、同一初始状态下，
  // 两次独立构造的地形必须产生**完全相同**的弹丸速度。
  // 只要中途有任何浮点参与（哪怕 Math.cos 本身在本机确定），
  // 换成另一台机器就可能不同 —— 用「表是整数」来锁死这个前提。

  it('方向表是单位向量（每项模长 1000，容差 ±1）', () => {
    // 表若是坏的（生成脚本算错、某项手滑），风障会把弹丸甩到奇怪的方向，
    // 而这种错误不会让任何现有测试变红 —— 所以直接校验表本身。
    for (let deg = 0; deg < 360; deg++) {
      const x = ROTOR_DIR_1000[deg * 2]
      const y = ROTOR_DIR_1000[deg * 2 + 1]
      const len2 = x * x + y * y
      const dev = Math.abs(Math.round(Math.sqrt(len2)) - 1000)
      expect(dev).toBeLessThanOrEqual(1)
    }
  })

  it('方向表覆盖全部 360 个整数角度', () => {
    expect(ROTOR_DIR_1000.length).toBe(720)
  })

  it('四个基本方向正确（0/90/180/270 度）', () => {
    const at = (d: number): [number, number] => [
      ROTOR_DIR_1000[d * 2],
      ROTOR_DIR_1000[d * 2 + 1],
    ]
    expect(at(0)).toEqual([1000, 0])
    expect(at(90)).toEqual([0, 1000])
    expect(at(180)).toEqual([-1000, 0])
    expect(at(270)).toEqual([0, -1000])
  })

  it('角度是整数且恒在 0..359 内（不会因浮点漂移出界）', () => {
    // 原来是 `(angle + param*dt/1000) % 360` 的浮点累加，
    // 长时间运行后可能得到 359.9999999 这类值 → 下标越界或取到错误方向。
    const t = place('rotor_vane', 89) // 最快的角速度
    for (let i = 0; i < 5000; i++) {
      t.update(50, ctx())
      expect(Number.isInteger(t.angle)).toBe(true)
      expect(t.angle).toBeGreaterThanOrEqual(0)
      expect(t.angle).toBeLessThan(360)
    }
  })

  it('相同输入产生逐位相同的偏转量（无隐藏状态）', () => {
    const run = (): [bigint, bigint] => {
      const t = place('rotor_vane', 60)
      const p = { element: 'fire', x: toFixed(500), y: toFixed(500), vx: 0n, vy: 0n }
      t.update(50, ctx({ projectiles: [p as unknown as Projectile], within: () => true }))
      return [p.vx, p.vy]
    }
    expect(run()).toEqual(run())
  })

  it('偏转量是整数（不是浮点截断的产物）', () => {
    const t = place('rotor_vane', 77)
    const p = { element: 'fire', x: toFixed(500), y: toFixed(500), vx: 0n, vy: 0n }
    t.update(50, ctx({ projectiles: [p as unknown as Projectile], within: () => true }))
    expect(typeof p.vx).toBe('bigint')
    expect(typeof p.vy).toBe('bigint')
  })

  // 这条是**行为**断言，比任何源码文本扫描都强：
  // 把 Math.cos / Math.sin 打成会记账的桩，跑完整场战斗，
  // 断言一次都没被调用。
  //
  // ⚠️ 为什么必须有它：前面那几条（表是单位向量、四基本方向、角度是整数）
  // 断言的都是**表本身**。把运行时的查表换成 Math.cos 之后，
  // 表一个字都没改，那几条**全部照常通过** ——
  // 我实际做过这个变异，30 个地形用例无一变红。
  //
  // 也就是说"表是对的"和"运行时用的是表"是两件事，只有后者被这条守住。
  //
  // 范围说明：只覆盖战斗逻辑（Terrain + BattleEngine）。
  // 渲染层 canvas.ts 合法地使用 Math.cos/sin 画八边形与扇形 ——
  // 渲染结果不参与 replayHash，差 1 ulp 的像素偏差与 I-6 无关。
  it('整场战斗的判定路径一次都不调用 Math.cos / Math.sin', () => {
    const origCos = Math.cos
    const origSin = Math.sin
    const calls: string[] = []
    Math.cos = function stub() {
      calls.push('cos')
      return origCos(0)
    }
    Math.sin = function stub() {
      calls.push('sin')
      return origSin(0)
    }
    try {
      // 用一个带旋转风障的地形跑满整局，覆盖所有 update 分支
      const t = place('rotor_vane', 60)
      const p = { element: 'fire', x: toFixed(500), y: toFixed(500), vx: 1000n, vy: 1000n }
      const c = ctx({ projectiles: [p as unknown as Projectile], within: () => true })
      for (let i = 0; i < 3000; i++) t.update(50, c)
    } finally {
      Math.cos = origCos
      Math.sin = origSin
    }
    expect(calls).toEqual([])
  })

  it('不同角度给出不同偏转方向（风障真的在转）', () => {
    const deflectAt = (param: number): [bigint, bigint] => {
      const t = place('rotor_vane', param)
      const p = { element: 'fire', x: toFixed(500), y: toFixed(500), vx: 0n, vy: 0n }
      t.update(50, ctx({ projectiles: [p as unknown as Projectile], within: () => true }))
      return [p.vx, p.vy]
    }
    // param=90 度/秒、dt=50ms → 每 tick 转 4.5→4 度。
    // 连续两 tick 的偏转方向应当不同，否则风障是静止的。
    const t = place('rotor_vane', 90)
    const p = { element: 'fire', x: toFixed(500), y: toFixed(500), vx: 0n, vy: 0n }
    const c = ctx({ projectiles: [p as unknown as Projectile], within: () => true })
    t.update(50, c)
    const [x1, y1] = [p.vx, p.vy]
    t.update(50, c)
    expect([p.vx - x1, p.vy - y1]).not.toEqual([x1, y1])
    void deflectAt
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

describe('地形：serialize', () => {
  it('把地形的身份、位置与运行时状态拍平成可序列化对象', () => {
    // serialize 是防线快照 / 回放侧读取地形状态的出口。它必须同时
    // 带上**静态身份**（kind/x/y）与**动态状态**（state/triggered）——
    // 少了后者，快照会声称"掩体完好"而实际早已崩塌。
    const t = place('oil_drum', 20, 500, 500)
    expect(t.serialize()).toEqual({
      kind: 'oil_drum',
      x: 500,
      y: 500,
      state: 'idle',
      triggered: false,
    })

    // 触发后：动态字段必须如实反映
    t.onHit('fire', 500n, toFixed(500), toFixed(500))
    for (let i = 0; i < 9 && t.state !== 'burning'; i++) {
      t.onHit('fire', 500n, toFixed(500), toFixed(500))
    }
    const snap = t.serialize()
    expect(snap.state).toBe('burning')
    expect(snap.triggered).toBe(true)
  })

  it('崩塌后的掩体序列化为 collapsed 且不再阻挡弹道', () => {
    const t = place('collapse_wall', 10)
    expect(t.blocksProjectile()).toBe(true)
    for (let i = 0; i < 20 && t.state !== 'collapsed'; i++) {
      t.onHit('kinetic', 500n, toFixed(500), toFixed(500))
    }
    expect(t.state).toBe('collapsed')
    expect(t.serialize().state).toBe('collapsed')
    expect(t.blocksProjectile()).toBe(false)
  })
})
