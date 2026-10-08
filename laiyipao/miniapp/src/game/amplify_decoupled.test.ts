import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { stripTsComments } from '../testkit/stripComments'

/**
 * `amplifyPct` 的生效**不得被 `statusDurationMs` 把门**（第 88 轮）。
 *
 * # 缺陷：潜伏耦合
 *
 * 原写法：
 *
 * ```ts
 * if (res.statusDurationMs > 0) {
 *   const spec = REACTIONS[res.reaction as ReactionKey]
 *   if (spec.amplifyPct > 0) {
 *     e.amplifyPermille = BigInt(spec.amplifyPct)   // ← 被关在里面
 *   }
 *   ...
 * }
 * ```
 *
 * 于是：**新增一条「只有受击增伤、没有附加状态」的反应时，
 * 它的增伤永远不生效，而代码看起来完全正常。**
 *
 * 为什么这个洞能活到今天 —— 而且**测不出来**：
 * 当前 7 条反应里没有任何一条落在「有增伤、无状态」这个组合：
 *
 * | 反应 | amplifyPct | statusDurationMs |
 * |---|---|---|
 * | `superconduct` | 600 | 4000 |
 * | `flash_freeze` | 300 | 2000 |
 * | 其余 5 条 | 0 | 0 / 1500 |
 *
 * 所以判据「增伤生效」与「增伤不生效」在今天**观察不到差别** ——
 * 它是一个**只有在改动之后才会显形**的洞。
 *
 * 这与第 80 轮（风障半径）、第 85 轮（`EnergyOnWin(5) < cap(15)`）
 * 记的**同一个陷阱**：
 * **只要输入落不到分界线上，关于分界线的断言都是空的。**
 *
 * # 为什么不构造一条假的反应来做行为断言
 *
 * 那需要让 `lookupReaction` 返回一个表里没有的 key，
 * 而反应表是 7×7 的元素组合查表 —— 造出来要么改类型联合
 * （动生产类型），要么改 `REACTION_TABLE`（动生产数据）。
 *
 * **为一个「当前不存在」的输入去改生产结构**是本末倒置。
 *
 * 所以判据落在**能直接观测的那一层**：源码里那条赋值的**嵌套深度**。
 * 嵌套是可数的、确定的、无需夹具的。
 *
 * 与第 80 轮（扫源码核对半径常量）、第 82 轮（扫 Tx 闭包）、
 * 第 87 轮（扫 wiki 模板列）同一条原则。
 */

/**
 * 拆开把门之后，**查表本身**必须重新有自己的守卫（第 88 轮补）。
 *
 * # 我第一版把查表提到条件之外，当场引入了一个崩溃
 *
 * ```ts
 * const reactSpec = REACTIONS[res.reaction as ReactionKey]  // ← 无条件执行
 * if (reactSpec.amplifyPct > 0) { ... }
 * ```
 *
 * 而**无反应时 `res.reaction` 是空串** —— `REACTIONS['']` 是 `undefined`，
 * 紧接着读 `.amplifyPct` 就抛：
 *
 * ```
 * TypeError: Cannot read properties of undefined (reading 'amplifyPct')
 * ```
 *
 * 被 `replay_discard.test.ts` / `elem_coef_shape.test.ts` /
 * `balance.probe.test.ts` / `loadout_attacker.test.ts`
 * **四个文件 35 个用例当场抓到**。
 *
 * # 为什么既有守卫没提前警告
 *
 * 因为原来的 `if (res.statusDurationMs > 0)` 除了把门 amplify，
 * **顺带**挡住了「无反应时不要查表」。那是一个**副作用式的守卫** ——
 * 它在做两件事，而只有一件被写在代码里。
 *
 * 拆耦合时必须把它显式补回来：
 * **拿一个偶发崩溃换另一个潜伏洞，是净亏。**
 *
 * 这与第 87 轮那条同源：`aoeRadius` 的「被震动消费」也是副作用，
 * 而 `descr` 的「被文档站展示」只是一条注释。
 */
describe('无反应时不得查反应表（第 88 轮）', () => {
  it('查表被 res.reaction 非空的守卫包住', () => {
    const body = hitEnemyBody()
    const idx = body.search(/const reactSpec = /)
    expect(idx, '找不到 reactSpec 的声明').toBeGreaterThan(0)

    // reactKey 必须先由 res.reaction 导出
    expect(
      /const reactKey = res\.reaction as ReactionKey/.test(body),
      'reactKey 应当由 res.reaction 导出',
    ).toBe(true)
    expect(
      /const reactSpec = reactKey \? REACTIONS\[reactKey\] : undefined/.test(body),
      '查表必须写成 `reactKey ? REACTIONS[reactKey] : undefined` ——\n' +
        '无反应时 res.reaction 是空串，REACTIONS[空串] 是 undefined，\n' +
        '无条件查表 + 直接读属性 = TypeError。',
    ).toBe(true)
  })

  it('读 amplifyPct 之前有 reactSpec 的存在性判断', () => {
    const body = hitEnemyBody()
    expect(
      /if \(reactSpec && reactSpec\.amplifyPct > 0\)/.test(body),
      '读 reactSpec.amplifyPct 之前必须先判 reactSpec 存在',
    ).toBe(true)
  })

  it('REACTIONS 查表在 hitEnemy 里被 res.reaction 守卫着（源码自查）', () => {
    const body = hitEnemyBody()
    const lookups = [...body.matchAll(/REACTIONS\[/g)]
    expect(lookups.length, 'hitEnemy 里应当只有两处查表（主查 + 兜底），实际 ' + lookups.length)
    // 兜底那处是 `reactSpec ?? REACTIONS[...]`，它只在 statusDurationMs > 0 内，
    // 而那时 res.reaction 必然非空 —— 所以它不需要额外守卫。
    expect(body).toMatch(/reactSpec \?\? REACTIONS\[react as ReactionKey\]/)
  })
})


/** engine.ts 里 `hitEnemy` 方法的源码区间（**已剥注释**）。 */
function hitEnemyBody(): string {
  const src = stripTsComments(readFileSync(resolve(__dirname, 'engine.ts'), 'utf-8'))
  const start = src.indexOf('private hitEnemy(')
  expect(start, '找不到 hitEnemy —— 结构变了').toBeGreaterThan(0)
  // 方法体结束：从 start 起找第一个行首两个空格的 `}`
  const end = src.indexOf('\n  }', start)
  expect(end, 'hitEnemy 没有正常闭合').toBeGreaterThan(start)
  return src.slice(start, end)
}

/**
 * `amplifyPermille =` 的那一行**是否被包在**「条件里提到 statusDurationMs」的块里。
 *
 * 判据是**逐层回溯花括号配对**：
 * 从赋值行往上走，每经过一个 `{` 就看它所属的 `if (...)` 条件。
 * 只要有一层提到 `statusDurationMs`，就判定为「被把门」。
 */
function amplifyIsGatedByStatus(body: string): { gated: boolean; gateLine: string } {
  const lines = body.split('\n')
  const assignIdx = lines.findIndex((l) => /e\.amplifyPermille\s*=/.test(l))
  if (assignIdx < 0) return { gated: false, gateLine: '' }

  let depth = 0 // 从赋值行往上累积未闭合的 `{`
  for (let i = assignIdx - 1; i >= 0; i--) {
    const line = lines[i]
    depth += countUnclosed(line)
    if (depth <= 0) {
      // 这一行的 `}` 闭合了我们所在的那一层 —— 检查它所属的 if
      if (/}\s*else\s+if\s*\(/.test(line)) {
        const cond = line.slice(line.indexOf('if'))
        if (/statusDurationMs/.test(cond)) return { gated: true, gateLine: cond.trim() }
      }
      // 继续往上一层：depth 归零意味着再往上就是别的块了
      depth = 0
      continue
    }
    if (/^\s*if\s*\(/.test(line) || /^\s*}\s*else\s+if\s*\(/.test(line)) {
      if (/statusDurationMs/.test(line)) {
        return { gated: true, gateLine: line.trim() }
      }
    }
  }
  return { gated: false, gateLine: '' }
}

/** 一行里未闭合的 `{` 个数（`}` 不计）。 */
function countUnclosed(line: string): number {
  return (line.match(/\{/g) ?? []).length
}

describe('反应增伤与状态时长不得互相把门（第 88 轮）', () => {
  it('amplifyPermille 的赋值不在 statusDurationMs 条件里', () => {
    const body = hitEnemyBody()
    const { gated, gateLine } = amplifyIsGatedByStatus(body)
    expect(
      gated,
      gated
        ? `amplifyPermille 的赋值被包在「${gateLine}」里。\n\n` +
            `后果：新增一条 amplify_pct > 0 且 status_duration_ms = 0 的反应时，` +
            `它的受击增伤**永远不会生效**，而代码看起来完全正常。\n\n` +
            `这是潜伏耦合 —— 今天 7 条反应都不落在那个组合上，` +
            `所以没有任何行为断言能测出来。\n` +
            `请把 amplifyPct 的赋值挪到 statusDurationMs 判断之外。`
        : '',
    ).toBe(false)
  })

  it('判据命中的那一行不是注释', () => {
    // ⚠️ 这条是被变异测试逼出来的。
    //
    // 变异「去掉注释剥离」在**当前**代码上是**无害**的 ——
    // 因为第 88 轮第一次修复时我在上方写的「修复前写法」示例注释，
    // 已经在第二轮（fix 无条件查表那版）里被换掉了，不再含
    // `e.amplifyPermille =` 这行诱饵。
    //
    // 所以那条变异全绿，**但它守的洞是真实存在的**：
    // 只要有人再写一段带代码示例的说明注释，判据就会在注释里找到目标，
    // 于是「修复有效」这个结论纯属侥幸。
    //
    // 与第 83 轮撞的是同一面墙（那次是我自己绊倒的，
    // `canvas.ts` 的说明注释里两次提到 `spec.aoeRadius`）。
    //
    // 判据：命中的那一行不得以 `//` 开头。
    // 它**不关心**剥离实现有没有被删掉 ——
    // 只要结果仍然落在真代码上，两种实现都算通过；
    // 一旦诱饵回来，只有一条会失败。
    const body = hitEnemyBody()
    const lines = body.split('\n')
    const idx = lines.findIndex((l) => /e\.amplifyPermille\s*=/.test(l))
    expect(idx, '找不到 amplifyPermille 的赋值').toBeGreaterThan(0)
    expect(
      lines[idx].trim().startsWith('//'),
      `判据命中的第一处赋值在**注释**里：\n${lines[idx]}\n\n` +
        '这意味着结论来自注释而不是代码 —— 「看起来在检查」与「真的在检查」' +
        '在这里长得一模一样。\n' +
        '（成因：注释剥离被去掉 + 上方有带代码示例的说明注释。）',
    ).toBe(false)
  })

  it('assign 与 if 分离后，两个字段各自有判据（结构自查）', () => {
    // 形状自查：赋值存在，且它**前面最近的一层 if** 不是 statusDurationMs。
    const body = hitEnemyBody()
    expect(body, 'hitEnemy 里找不到 amplifyPermille 的赋值').toMatch(
      /e\.amplifyPermille\s*=\s*BigInt\(reactSpec\.amplifyPct\)/,
    )
    expect(body, 'hitEnemy 里找不到 statusDurationMs 的状态写入').toMatch(
      /(frozenMs|stunnedMs)\s*=/,
    )
  })
})

/**
 * 判据自身的守卫：把「嵌套」与「并列」两种形状喂给它。
 *
 * ⚠️ 与第 78/82 轮同一条理由：**不能**用「扫不到就 Fatal」来保证判据非盲 ——
 * 修复完成后真实代码里一处违规都不剩，「扫到 0 处」既可能是「真的合规」
 * 也可能是「判据坏了」，两者在输出里一模一样。而**一条永远红的守卫会被人删掉**。
 */
describe('嵌套判定器的自测', () => {
  const synth = (body: string) => ({ gated: amplifyIsGatedByStatus(body).gated })

  it('赋值在 statusDurationMs 的 if 里 → 判定为被把门', () => {
    const r = synth(
      [
        '    if (res.statusDurationMs > 0) {',
        '      const spec = REACTIONS[x]',
        '      if (spec.amplifyPct > 0) {',
        '        e.amplifyPermille = BigInt(spec.amplifyPct)',
        '      }',
        '    }',
      ].join('\n'),
    )
    expect(r.gated, '这是本轮修复前的真实形状，判据必须抓得到').toBe(true)
  })

  it('赋值与状态写入是并列的两个 if → 判定为未被把门', () => {
    const r = synth(
      [
        '    if (reactSpec.amplifyPct > 0) {',
        '      e.amplifyPermille = BigInt(reactSpec.amplifyPct)',
        '    }',
        '    if (res.statusDurationMs > 0) {',
        "      if (react === 'overheat') e.stunnedMs = reactSpec.statusDurationMs",
        '    }',
      ].join('\n'),
    )
    expect(r.gated).toBe(false)
  })

  it('赋值被包在与 statusDurationMs 无关的 if 里 → 判定为未被把门', () => {
    const r = synth(
      [
        '    if (e.hp > 0n) {',
        '      if (reactSpec.amplifyPct > 0) {',
        '        e.amplifyPermille = BigInt(reactSpec.amplifyPct)',
        '      }',
        '    }',
      ].join('\n'),
    )
    expect(r.gated).toBe(false)
  })

  it('赋值在 if/else 链的 else-if 里 → 也要被抓到', () => {
    const r = synth(
      [
        '    if (x) {',
        '      doThing()',
        '    } else if (res.statusDurationMs > 0) {',
        '      e.amplifyPermille = BigInt(k)',
        '    }',
      ].join('\n'),
    )
    expect(r.gated, 'else-if 链是「看起来不像嵌套」的形态').toBe(true)
  })

  it('注释里的示例代码不得被当成真实代码', () => {
    // ⚠️ 上面那条变异就是被这一条挡住的，所以它必须**自己**被测到。
    //
    // 没有这一条时，判据「恰好」在注释里找到了目标而返回 false，
    // 看起来像是「修复有效」，实际上它**根本没看代码**。
    const body = [
      '    // ⚠️ 下面是修复前的写法，仅作示例：',
      '    //    if (res.statusDurationMs > 0) {',
      '    //      e.amplifyPermille = BigInt(spec.amplifyPct)',
      '    //    }',
      '    if (reactSpec.amplifyPct > 0) {',
      '      e.amplifyPermille = BigInt(reactSpec.amplifyPct)',
      '    }',
    ].join('\n')
    expect(
      amplifyIsGatedByStatus(stripTsComments(body)).gated,
      '剥掉注释后，真正的代码不在 statusDurationMs 里 —— 判定必须为「未被把门」',
    ).toBe(false)
  })
})
