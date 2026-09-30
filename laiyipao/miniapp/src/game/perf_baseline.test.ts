/**
 * 引擎性能的**回归绊线**（不是「打印数字的测试」）。
 *
 * ## 为什么存在
 *
 * 第一版这个文件只有 `console.log`，**一条断言都没有** ——
 * 它永远不会红，却每次要花 ~4 秒跑完 100 关全扫。
 * 那是「看起来像测试的东西」：占了反馈回路的时间，却不提供任何保护。
 *
 * 现在它守两件事：一件能判、一件只能看。
 *
 * ---
 *
 * ## ① 能判的：全量扫描的总耗时上限（粗回归绊线）
 *
 * 实测基线（2026-09-28，Node 24，本机）：
 *
 *   100 关全扫 2273 ms / 561,516 tick / 平均 4.0 µs per tick
 *   （同一份扫描在另一次冷启动下测到 3679 ms / 6.6 µs；
 *     tick 数两次都是 561,516，说明扫描本身是确定的，差异来自 JIT 预热）
 *
 * 这里取 **60 秒**，也就是相对**较慢那次**仍有 **16 倍余量**。
 * 宁可按最差情况留余量：绊线的价值取决于它**从不误报**。
 *
 * ### 为什么不取更紧的阈值
 *
 * 墙钟断言天然易抖动：CI 机器、后台负载、其它并行任务都会影响它。
 * 本项目吃过「2 关差 1~2 点就红」的亏 —— 那条守卫后来被当成 flaky 忽略掉，
 * 比没有守卫更糟。**一条经常误报的守卫等于没有守卫。**
 *
 * 16 倍余量意味着：正常波动（哪怕慢 5 倍）不会触发；
 * 而「有人在热循环里加了一层嵌套遍历 / 把 bigint 转成字符串再转回来」
 * 这类改动会轻松超出 10 倍，**一定会被抓到**。
 *
 * 也就是说这条断言的定位很明确：**不是性能基准，是 gross regression 报警器。**
 *
 * ---
 *
 * ## ② 只能看的：每 tick 成本与敌人数的关系
 *
 * 实测（6 个关卡，敌人数 39 ~ 84）：
 *
 *   L1(39 敌) 3.2 µs   L20(51) 3.3   L40(52) 3.9
 *   L60(64)   3.4 µs   L80(55) 3.3   L100(84) 3.1
 *
 * 基本持平 → 每 tick 的成本**不随同屏敌人数超线性增长**，
 * 与「没有嵌套遍历」的判断一致。
 *
 * ### ⚠️ 这个观察的**局限**，必须一起记下来
 *
 * **2 个数据点无法区分 O(n) 与 O(n²)**，6 个点也只是弱证据：
 *
 *  1. 敌人数 39 → 84 只差 2.15 倍，O(n²) 的信号是 4.6 倍。
 *     要把两者分开，采样跨度至少要一个数量级，而夹具里最大的关卡也只有 84 敌。
 *  2. 更关键：**这 6 关的差异不止敌人数**。波数、反应次数、卡牌机制、
 *     地形都不同。每 tick 成本是所有因素的总和，不是敌人数的函数。
 *     所以「持平」并不严格等于「与敌人数无关」。
 *
 * 结论只能是「**没有发现**超线性」，不是「证明了线性」。
 *
 * ### 正确的做法（还没做，诚实记下）
 *
 * 要真正判定复杂度，得**固定其它变量、只扫敌人数**：
 * 同一关卡、同一 seed、同一构筑，把同屏上限从 8 扫到 128，
 * 看每 tick 成本随上限的增长阶。那需要能生成任意敌人数的关卡，
 * 属于另一个量级的工作。
 *
 * ---
 *
 * ## 结论：引擎运行时**不是**性能问题
 *
 * 20Hz × 6.6 µs/tick = 每秒占 **0.013% CPU**。真实对局毫无压力。
 *
 * 真正影响开发节奏的是**测试套件耗时**（详见本轮提交说明：
 * 消除了 4 个文件里 16 次重复的全量扫描，累计测试时间 -40%）。
 * 这也是为什么本文件只留一条宽松的绊线，而不去优化引擎热循环 ——
 * 引擎的定点 bigint 运算正是喂给 `replayHash()` 的最敏感代码，
 * 改它收益 ~20~40%、代价是**可能静默毁掉全部历史战报的可重放性**。
 */
import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleEngine, MAX_BATTLE_TICKS, TICK_MS } from './engine'
import { equippedFromSnapshot, type BuildSnapshot } from './replay'
import { ACTIVE_SLOTS } from './heatmap'
import { defaultAttacker } from './damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'

const levels = (fixture.levels as unknown as GeneratedLevel[]).slice().sort((a, b) => a.id - b.id)
const enemies = new Map<number, EnemyDef>((fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]))
const skills = new Map<number, SkillDef>(
  [...(fixture.skills as unknown as SkillDef[]), ...(fixture.composite_skills as unknown as SkillDef[])].map(
    (s) => [s.id, s],
  ),
)

function buildSnapshot(): BuildSnapshot {
  const active = [...skills.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, ACTIVE_SLOTS)
  const map: Record<string, unknown> = {}
  active.forEach((s, i) => {
    map[String(s.id)] = {
      id: s.id, name: s.name, family: 't', element: s.element, kind: s.kind, slot: i, level: 1,
    }
  })
  return { skills: map, attacker: { attack: 0 }, active_slots: ACTIVE_SLOTS } as unknown as BuildSnapshot
}

function mk(lv: GeneratedLevel): BattleEngine {
  return new BattleEngine({
    level: lv,
    enemies,
    skills,
    equipped: equippedFromSnapshot(buildSnapshot(), skills),
    attacker: defaultAttacker(),
    seed: 12345,
    activeSlots: ACTIVE_SLOTS,
  })
}

/** 跑完一关，返回耗时与 tick 数。 */
function play(lv: GeneratedLevel) {
  const e = mk(lv)
  const t0 = process.hrtime.bigint()
  e.start()
  for (let t = 0; t < MAX_BATTLE_TICKS; t++) {
    if (e.phase === 'won' || e.phase === 'lost') break
    if (e.phase === 'card_select') e.skipCards()
    e.step()
  }
  const ms = Number(process.hrtime.bigint() - t0) / 1e6
  return { ms, ticks: e.tick, won: e.phase === 'won', phase: e.phase }
}

describe('引擎性能绊线', () => {
  it('100 关全扫不超过 60 秒（gross regression 报警器，实测 2.3~3.7 秒）', () => {
    const rows = levels.map(play)
    const totalMs = rows.reduce((a, b) => a + b.ms, 0)
    const totalTicks = rows.reduce((a, b) => a + b.ticks, 0)

    console.log(`  关卡数            ${rows.length}`)
    console.log(`  模拟总耗时         ${totalMs.toFixed(0)} ms（真实游戏时长 ${((totalTicks * TICK_MS) / 1000).toFixed(0)} s）`)
    console.log(`  总 tick 数         ${totalTicks}`)
    console.log(`  平均每 tick        ${((totalMs * 1000) / totalTicks).toFixed(1)} µs`)

    // 前提守卫：总 tick 数为 0 会让上面的「平均每 tick」变成 Infinity，
    // 而 Infinity 在这条断言里反而会通过 —— 必须在断言前排除。
    expect(totalTicks).toBeGreaterThan(0)
    expect(totalMs).toBeLessThan(60_000)
  }, 900_000)

  it('每 tick 成本不随同屏敌人数爆炸（诊断性观察，局限见文件头注释）', () => {
    const rows: string[] = []
    const samples: Array<{ enemies: number; usPerTick: number }> = []
    for (const id of [1, 20, 40, 60, 80, 100]) {
      const lv = levels.find((l) => l.id === id)!
      const total = lv.waves.reduce((a, w) => a + w.spawns.reduce((b, s) => b + s.count, 0), 0)
      const r = play(lv)
      const us = r.ticks > 0 ? (r.ms * 1000) / r.ticks : 0
      samples.push({ enemies: total, usPerTick: us })
      rows.push(`  L${id}: 总怪 ${String(total).padStart(3)}  ${String(r.ticks).padStart(5)} tick  ${us.toFixed(1)} µs/tick`)
    }
    console.log(rows.join('\n'))

    expect(samples.length).toBeGreaterThan(1)
    const min = Math.min(...samples.map((s) => s.usPerTick))
    const max = Math.max(...samples.map((s) => s.usPerTick))

    // 前提守卫：`usPerTick` 全为 0（tick 数为 0 / 计时器坏）会让下面那条
    // 变成 `0 <= 0`，**恒真**。而「断言恒真」正是本项目反复栽的形状：
    // 它看上去在保护什么，实际什么都不保护。
    expect(min).toBeGreaterThan(0)

    // 极宽松：只抓「某个关卡的每 tick 成本是最低者的 5 倍以上」这种明显异常。
    //
    // ⚠️ 阈值故意**不**用来判定 O(n²)：
    // 敌人数只差 2.15 倍，O(n²) 的信号是 4.6 倍，而这两者之间没有足够
    // 区分度 —— 5 倍的阈值会同时放过 O(n) 和 O(n²)。
    // 真正判定复杂度需要「固定其它变量只扫敌人数」，那是另一份工作。
    //
    // 所以这条断言的**唯一**作用是：某个关卡的单 tick 成本出现数量级异常时报警。
    expect(max).toBeLessThanOrEqual(min * 5)
  }, 900_000)
})
