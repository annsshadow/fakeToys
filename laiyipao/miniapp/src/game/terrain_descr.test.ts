import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { TERRAIN_DESCR, TERRAIN_NAME, ROTOR_RADIUS, OIL_BURN_MS } from './terrain'
import type { TerrainKind } from './types'

/**
 * `TERRAIN_DESCR` 里对玩家说的**数字**必须等于实现（第 80 轮）。
 *
 * # 缺陷：风障文案说 140，实现是 260
 *
 * `TERRAIN_DESCR.rotor_vane` 写「持续改变 **140 半径**内弹丸的飞行方向」，
 * 而 `updateRotorVane` 用的是 `ctx.within(this.x, this.y, 260, ...)`。
 *
 * 140 → 260 是一次**有意的**平衡改动（理由写在 `updateRotorVane` 的
 * 实测注释里：6 个风障的作用范围内一次弹丸都没进过，偏转计数恒为 0），
 * 但**文案没跟着改** —— 玩家读到的作用范围是实际的 **46%**。
 *
 * # 为什么这类漂移能长期存活
 *
 * 因为「文案里的数字」**在代码里根本不存在**。
 * 改实现时看到的是 `260`，不会有人想到去改一句中文；
 * 而中文那句在任何代码评审里都不会被机器检查。
 *
 * 同一个文件里的 `oil_drum`「8 秒」**恰好**与 `this.timer = 8000` 一致 ——
 * 但那个「恰好」不构成保证，只构成运气。
 *
 * # 三层防守（本文件是第三层）
 *
 *  1. **常量提取** —— 半径与时长从字面量变成 `ROTOR_RADIUS` / `OIL_BURN_MS`
 *  2. **文案插值** —— `TERRAIN_DESCR` 用模板串引用常量，编译期保证一致
 *  3. **本文件** —— 断言「文案里确实出现了那个数字」
 *
 * 第 3 层看似多余：既然是插值的，数字怎么会不对？
 *
 * ⚠️ 会 —— 有人把模板串改回硬编码字面量（为了让文案读起来更自然，
 * 或者纯粹因为拼写错误）。那时编译期不再保证，**只有这条守卫会响**。
 *
 * 这与 README 记的「守卫要区分『在名单里』与『生效』」同源：
 * 机制正确不等于有人检查机制是否还在生效。
 */

/** 五类地形：文案与常量都必须齐全（缺一个就是「新增时忘了写文案」）。 */
const KINDS: TerrainKind[] = [
  'oil_drum',
  'tidal_gate',
  'rotor_vane',
  'collapse_wall',
  'charge_tower',
]

describe('地形文案与实现的一致性（第 80 轮）', () => {
  it('五类地形都有 name 与 descr（新增地形时忘了写文案会红）', () => {
    expect(Object.keys(TERRAIN_DESCR).sort()).toEqual([...KINDS].sort())
    expect(Object.keys(TERRAIN_NAME).sort()).toEqual([...KINDS].sort())
  })

  it('风障文案里的半径 == ROTOR_RADIUS', () => {
    expect(
      TERRAIN_DESCR.rotor_vane,
      `风障文案应包含 ${ROTOR_RADIUS}（常量值），实得「${TERRAIN_DESCR.rotor_vane}」。\n` +
        '若文案被改回硬编码字面量，这条会响 —— 那正是本文件存在的意义。',
    ).toContain(String(ROTOR_RADIUS))
    // ⚠️ 明确排除旧值：文案里若还留着 140，就是漂了。
    // 只断言「包含 260」不够 —— 「140 和 260 都写了」也满足它。
    expect(TERRAIN_DESCR.rotor_vane).not.toMatch(/\b140\b/)
  })

  it('油桶文案里的秒数 == OIL_BURN_MS / 1000', () => {
    const secs = OIL_BURN_MS / 1000
    expect(OIL_BURN_MS % 1000, '时长应当是整秒 —— 否则文案里的「N 秒」不精确').toBe(0)
    expect(
      TERRAIN_DESCR.oil_drum,
      `油桶文案应包含 ${secs} 秒，实得「${TERRAIN_DESCR.oil_drum}」`,
    ).toContain(String(secs))
  })

  it('文案里的每个数字都必须有命名常量撑着（且不得被改回硬编码）', () => {
    // ⚠️ 这一条必须在**源码文本**上判，不能在求值后的字符串上判。
    //
    // 我第一版写的是 `d.includes('${')` —— 而 `d` 是**已求值**的
    // 模板串，`${OIL_BURN_MS / 1000}` 早就变成了 `8`，`${` 不复存在。
    // 于是「用了插值」这个事实被误判成「没用插值」。
    //
    // 这与第 67/68 轮记的教训同源：**在错误的层次上判**，
    // 判据看起来合理、跑起来稳定、结论全错。
    const src = readFileSync(resolve(__dirname, 'terrain.ts'), 'utf-8')

    // 取出 TERRAIN_DESCR 的源码块
    const start = src.indexOf('export const TERRAIN_DESCR')
    const end = src.indexOf('\n}', start)
    expect(start > 0 && end > start, '找不到 TERRAIN_DESCR 的源码块 —— 结构变了')
    const block = src.slice(start, end)

    for (const kind of KINDS) {
      const line = block.split('\n').find((l) => l.trim().startsWith(`${kind}:`))
      expect(line, `TERRAIN_DESCR 里没有 ${kind} 这一行`)
      const claims = /\d+\s*(半径|秒|层|点|格)/g
      const hits = [...line!.matchAll(claims)]
      for (const hit of hits) {
        const num = hit[0].match(/\d+/)![0]
        // 两种合规形态：模板串插值，或该数字本身是源码里的命名常量初值
        const interpolated = line!.includes('${')
        const asConst = new RegExp(`=\\s*${num}\\b`).test(src)
        expect(
          interpolated || asConst,
          `TERRAIN_DESCR.${kind} 的文案「${line!.trim()}」里有数字 ${num}，` +
            '但它既没有用模板串插值、也不是源码里任何常量的初值 —— ' +
            '这个数字随时会与实现漂移。',
        ).toBe(true)
      }
    }
  })

  it('含数字的那两条文案必须真的用了插值（防止有人改回硬编码）', () => {
    const src = readFileSync(resolve(__dirname, 'terrain.ts'), 'utf-8')
    const start = src.indexOf('export const TERRAIN_DESCR')
    const end = src.indexOf('\n}', start)
    const block = src.slice(start, end)

    for (const kind of ['rotor_vane', 'oil_drum'] as TerrainKind[]) {
      const line = block.split('\n').find((l) => l.trim().startsWith(`${kind}:`))!
      expect(
        line,
        `TERRAIN_DESCR.${kind} 那一行应当是**模板串**（用 \`…\${CONST}…\`），` +
          `实得：${line.trim()}\n` +
          '把它改回普通引号 + 硬编码数字，编译期就不再保证一致性了。',
      ).toContain('`')
      expect(line, `TERRAIN_DESCR.${kind} 那一行应当引用常量`).toContain('${')
    }
  })

  it('wind 文案说的是「改变方向」而不是「减速」—— 语义不许漂', () => {
    // 这条守的是**语义**而非数字。
    //
    // 实现里 rotor 只改 `p.vx / p.vy`（方向），
    // 不改速率。若有人把文案改成「减速」，玩家会按错误的预期配装。
    expect(TERRAIN_DESCR.rotor_vane).toContain('方向')
    expect(TERRAIN_DESCR.rotor_vane).not.toContain('减速')
    expect(TERRAIN_DESCR.rotor_vane).not.toContain('伤害')
  })

  it('油桶文案说的是「受到焰元素命中」—— 焰（fire）是唯一能点燃的元素', () => {
    // 实现里 `if (element === 'fire' && ...)`。
    // 文案说「焰元素」是对的；说「任何元素」或「火元素之外」就是漂。
    expect(TERRAIN_DESCR.oil_drum).toContain('焰')
    expect(TERRAIN_DESCR.oil_drum).not.toContain('任意')
    expect(TERRAIN_DESCR.oil_drum).not.toContain('任何元素')
  })
})

/**
 * `updateRotorVane` 必须**引用常量**，不得内联裸字面量（第 80 轮补）。
 *
 * # 为什么前三条守卫抓不到这个
 *
 * `风障文案里的半径 == ROTOR_RADIUS` 守的是「文案 ↔ 常量」，
 * 而「常量 ↔ 实现」这一环没人守。
 *
 * 实测：把调用点改成 `ctx.within(this.x, this.y, 999, ...)`
 * （绕过常量直接写数字），前面 7 条守卫**全部通过** ——
 * 因为文案、量、测试三者都还在，只有实现漂了。
 *
 * 那正是本轮要消灭的那类漂移，只是从「文案漂」换成了「实现漂」。
 *
 * # 判据：扫 `updateRotorVane` 的源码文本
 *
 * 要求它出现 `ROTOR_RADIUS`，且**不出现** `within(<数字>` 这种形态。
 *
 * ⚠️ 扫源码而不是「跑一遍量半径」：后者需要构造一场弹道恰好穿过
 * 风障边界的战斗，而边界是闭区间（`<=`）—— 测「恰好在边界上」
 * 与「恰好不在」会落进量化的 x.5 陷阱（README 记的
 * 「round 在 x.5 边界上依赖浮点误差」的同型问题）。
 * **判据要落在能直接观测的那一层。**
 */
describe('风障半径的实现侧（第 80 轮补）', () => {
  const src = readFileSync(resolve(__dirname, 'terrain.ts'), 'utf-8')

  function rotorBody(): string {
    const start = src.indexOf('private updateRotorVane(')
    expect(start, '找不到 updateRotorVane —— 结构变了').toBeGreaterThan(0)
    const end = src.indexOf('\n  }', start)
    return src.slice(start, end > 0 ? end : start + 2000)
  }

  it('updateRotorVane 引用了 ROTOR_RADIUS 常量', () => {
    expect(
      rotorBody(),
      'updateRotorVane 没有引用 ROTOR_RADIUS —— ' +
        '有人把半径改成了裸字面量，文案与实现就此分家。',
    ).toContain('ROTOR_RADIUS')
  })

  it('updateRotorVane 的 within() 不含裸数字半径', () => {
    const body = rotorBody()
    const hits = [...body.matchAll(/within\([^)]*?,\s*(\d+)\s*,/g)]
    for (const h of hits) {
      expect.fail(
        `updateRotorVane 里出现 within(..., ${h[1]}, ...) —— ` +
          '半径被内联成了字面量。请用 ROTOR_RADIUS。',
      )
    }
  })

  it('油桶的持续时长同样不得内联', () => {
    const start = src.indexOf("case 'oil_drum': {")
    expect(start, "找不到 onHit 里的 oil_drum 分支").toBeGreaterThan(0)
    const body = src.slice(start, start + 700)
    expect(body).toContain('OIL_BURN_MS')
    expect(
      [...body.matchAll(/this\.timer\s*=\s*(\d+)/g)].map((m) => m[1]),
      'onHit 的 oil_drum 分支里 this.timer 被赋了裸数字 —— 请用 OIL_BURN_MS',
    ).toEqual([])
  })
})
