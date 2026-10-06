import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { REACTIONS, type ReactionKey } from './elements'
import type { ReactionSpec } from './elements'

/**
 * `ReactionSpec.descr` **只允许承诺已实现的效果**（第 83 轮）。
 *
 * # 缺陷：7 条反应里有 4 条对外文案承诺了不存在的机制
 *
 * 逐字段核过消费面（`resolveHit` + `BattleEngine.hitEnemy`）：
 *
 * | 字段            | 消费者                                   | 状态 |
 * |-----------------|------------------------------------------|------|
 * | baseCoef        | `damage.ts` 反应伤害                     | ✓    |
 * | attackWeightPct | `damage.ts` 攻方贡献比例                 | ✓    |
 * | statusDurationMs| `damage.ts` → 引擎写 `frozenMs`/`stunnedMs` | ✓ |
 * | dispelShield    | `damage.ts`                             | ✓    |
 * | amplifyPct      | `engine.ts`                              | ✓    |
 * | aoeRadius       | **只有 `render/canvas.ts` 的屏幕震动**     | ✗    |
 *
 * 而 `descr` 是**被消费的**（Go 侧 `ReactionSpec.Descr` 的注释写着
 * 「运营后台的玩法文档站直接展示它」）——
 * 所以这不是「内部注释不准」，是**对外文案在承诺不存在的机制**。
 *
 * | 反应            | 原来的文案                  | 实际 |
 * |-----------------|-----------------------------|------|
 * | `steam_burst`   | 范围伤害并驱散护盾          | 只有驱散 |
 * | `overheat`      | 爆炸并眩晕                  | 只有眩晕 |
 * | `burn_cloud`    | 生成持续火区                | **什么都没有** |
 * | `corrosion_spread` | 把元素层数传播给周围敌人  | **什么都没有** |
 * | `armor_break`   | 击退并削减护甲              | **什么都没有** |
 *
 * # 为什么改文案而不是「补实现」
 *
 * 「要不要给它们实现溅射/火区/传播」是**产品决策**：
 * 伤害量？是否计分？溅射到的目标要不要再触发反应？
 * 是否需要服务端一并重算以免 I-6 失配？—— 这些猜不得。
 *
 * 但**文案承诺一个不存在的机制**是确定的缺陷，无论将来是否实现。
 * 所以这轮把文案改成只描述已实现的效果，
 * 并把「要不要实现」记进 README 已知边界，附上确切的清单。
 *
 * 这与第 80 轮（风障文案 140 / 实现 260）是同一个形状：
 * **代码里的「承诺」与「实现」必须由机器核对。**
 */

const KEYS = Object.keys(REACTIONS) as ReactionKey[]

/**
 * `PROMISE_WORDS` 是「文案里出现它，就意味着承诺了一个效果」的词表。
 *
 * ⚠️ 词表是**人工**的，所以它不是完备判据 ——
 * 它只覆盖已知的那几类承诺。新增一种承诺方式（比如「召唤」「减速」）
 * 时需要往这里加词。
 *
 * 但它仍然是有效的：**它把「已知的那 4 条谎话」钉成事实**，
 * 而那正是本轮要消灭的东西。
 *
 * 每条都注明它对应哪个未被消费的字段 ——
 * 这样「为什么这个词不该出现」在失败信息里就一目了然。
 */
const PROMISE_WORDS: Array<{ word: string; unbackedBy: string }> = [
  { word: '范围', unbackedBy: 'aoeRadius' },
  { word: '爆炸', unbackedBy: 'aoeRadius' },
  { word: '溅射', unbackedBy: 'aoeRadius' },
  { word: '火区', unbackedBy: 'aoeRadius + statusDurationMs' },
  { word: '传播', unbackedBy: 'aoeRadius' },
  { word: '击退', unbackedBy: '（无字段，代码里也没有 knockback 写入）' },
  { word: '削减护甲', unbackedBy: '（无字段，applyArmor 只读 def.armorPermille）' },
  { word: '破甲', unbackedBy: '（无字段）' },
  { word: '减速', unbackedBy: '（无字段）' },
  { word: '召唤', unbackedBy: '（无字段）' },
]

describe('反应文案与实现的一致性（第 83 轮）', () => {
  it('七条反应都在', () => {
    expect(KEYS.length).toBe(7)
  })

  it('文案不承诺任何未被消费的机制', () => {
    const bad: string[] = []
    for (const k of KEYS) {
      const d = REACTIONS[k].descr
      for (const p of PROMISE_WORDS) {
        if (d.includes(p.word)) {
          bad.push(
            `${k} 的文案「${d}」里的「${p.word}」承诺了一个**没有实现**的机制` +
              `（对应字段：${p.unbackedBy}）`,
          )
        }
      }
    }
    expect(
      bad,
      `以下文案承诺了不存在的效果：\n  ${bad.join('\n  ')}\n\n` +
        '两条路可选：(a) 实现它；(b) 文案只写已实现的效果。\n' +
        '**若选 (a)**，请同时改 miniapp 与 server 两端的 ReactionSpec ' +
        '并重新生成 testdata/reaction_specs.json。',
    ).toEqual([])
  })

  it('aoeRadius 全部为 0（它从未被任何伤害逻辑消费）', () => {
    // 字段保留是因为它已在 `/config` 的公开 JSON 契约里，删字段是破坏性变更。
    // 但值必须是 0 —— 非 0 就是「承诺一个不存在的溅射」。
    const nonZero = KEYS.filter((k) => REACTIONS[k].aoeRadius !== 0)
    expect(
      nonZero,
      `这些反应的 aoeRadius 非 0：${nonZero.join(', ')}\n` +
        '而 `resolveHit` 从不读它 —— 唯一的消费者是屏幕震动（已在第 83 轮删掉）。',
    ).toEqual([])
  })

  it('没有任何反应是「完全空转」（必须有反应伤害）', () => {
    // 上面几条把承诺都去掉了，但反应本身不能变成「什么都不发生」——
    // 那会让这条元素组合彻底失去价值。
    //
    // 判据用**伤害**而不是「有没有特殊效果」：
    // `baseCoef > 0` 是反应伤害存在的充要条件。
    const dead = KEYS.filter((k) => REACTIONS[k].baseCoef <= 0)
    expect(dead, `这些反应没有任何伤害：${dead.join(', ')}`).toEqual([])
  })

  it('文案非空且与 name 不重复（重复说明是批量改的）', () => {
    for (const k of KEYS) {
      expect(REACTIONS[k].descr.length, `${k} 的文案是空的`).toBeGreaterThan(0)
      expect(REACTIONS[k].descr, `${k} 的文案与 name 相同`).not.toBe(REACTIONS[k].name)
    }
  })
})

/**
 * `ReactionSpec` 的每个字段都必须有**玩法**消费者。
 *
 * 这是本轮最结构化的一条：它不检查具体值，而是检查
 * 「这个字段承诺的东西，真的有人用吗」。
 *
 * ⚠️ 用**源码扫描**而不是运行观测：
 * 「aoeRadius 是否影响伤害」需要构造一场多敌人在场的战斗，
 * 而反应链的触发依赖元素层数组合 —— 判据会依赖夹具的巧合性质。
 * 扫源码能直接回答「有没有人读它」。
 *
 * ⚠️ 扫描排除 `render/`（表现层）：
 * 屏幕震动是**反馈**，不是玩法。把 `aoeRadius` 接到震动上
 * 让它在「有人读」的意义上活着，正是本轮要消灭的那种假信号。
 */
/**
 * stripTsComments 剥掉 TS 的行注释与块注释。
 *
 * ⚠️ 这是**文本级**的剥离，不是 AST —— 它对字符串字面量里的
 * `//` 会误伤。
 *
 * 那会不会出问题？只有当某个字符串字面量里恰好含有 `//spec.aoeRadius`
 * 这类文本时才会误判，而那种写法本身就可疑。
 * 真正的解析器（`ts.createSourceFile`）更准，但引入编译器依赖
 * 只为一条守卫不值 —— **误伤的代价是「有人要来解释这条为什么红了」**。
 *
 * 这个取舍记在这里，而不是留给后来的人猜。
 */
function stripTsComments(src: string): string {
  return src
    .replace(/\/\*[\s\S]*?\*\//g, ' ')
    .replace(/^\s*\/\/.*$/gm, ' ')
}

describe('ReactionSpec 没有「只有表现层消费者」的字段', () => {
  const src = readFileSync(resolve(__dirname, 'elements.ts'), 'utf-8')

  it('每个字段在 game/ 下都有玩法消费者', () => {
    const gameplay = readFileSync(resolve(__dirname, 'damage.ts'), 'utf-8') +
      readFileSync(resolve(__dirname, 'engine.ts'), 'utf-8')

    const fields = Object.keys(REACTIONS.steam_burst) as (keyof ReactionSpec)[]
    // key / name 是标识与显示，不参与计算
    const calcFields = fields.filter((f) => f !== 'key' && f !== 'name' && f !== 'descr')

    const orphans: string[] = []
    for (const f of calcFields) {
      const used = new RegExp(`\\b${f}\\b`).test(gameplay)
      if (!used) orphans.push(`${f}（game/ 下无人读）`)
    }
    expect(
      orphans,
      `这些字段在伤害/引擎里没有任何消费者：\n  ${orphans.join('\n  ')}\n\n` +
        '它们只是**承诺**。要么接上实现，要么把值置零并在注释里说明。',
    ).toEqual([])
  })

  it('descr 必须非空（它是文档站的展示源）', () => {
    // 单列一条，因为「字段有没有被读」这个判据会把 descr 排除掉
    // （它被 UI 而不是 game/ 读），而它恰恰是最需要守卫的那个。
    for (const k of KEYS) {
      expect(REACTIONS[k].descr, `${k}.descr 是空的 —— 文档站会显示空白`).not.toBe('')
    }
    // 反面：descr 在 UI 侧确实被读吗？
    // Go 侧由 /config 下发给文档站；客户端则用 name 弹字。
    void src
  })
})

  it('aoeRadius 不得在 elements.ts 之外被引用（防止「假信号」回流）', () => {
    // 变异实测：只把 `render/canvas.ts` 加进「玩法消费者」集合 → **全绿通过**。
    //
    // 原因是第 83 轮已经把 canvas.ts 里那行震动删了，
    // 于是 `aoeRadius` 在任何地方都没有读者 —— 「加一个假读者」这个变异
    // 观察不到任何差异。
    //
    // ⚠️ 但真缺口在这里：**把震动加回来**（`if (spec.aoeRadius > 0) shake`）
    // 不会被任何现有守卫抓到。而那正是本轮消灭的那个形态 ——
    // 一个让玩家误以为「炸到了」的假反馈。
    //
    // 所以这里直接断言「除 elements.ts 外无人引用」，
    // 而不是在消费者集合里做排除 —— 排除法依赖「谁没被列进去」，
    // 那是**白名单**，会漂；「谁被列进去了」是黑名单，也会漂。
    // 唯一不会漂的是「这个字段只允许出现在这张表里」。
    const files = [
      'damage.ts',
      'engine.ts',
      'replay.ts',
      'heatmap.ts',
      '../render/canvas.ts',
      'skill.ts',
      'defense.ts',
      'score.ts',
      'terrain.ts',
      'types.ts',
    ]
    const offenders: string[] = []
    for (const f of files) {
      let text: string
      try {
        text = readFileSync(resolve(__dirname, f), 'utf-8')
      } catch {
        continue // 文件不存在（可选依赖）
      }
      // ⚠️ 必须**先剥掉注释**。
      //
      // 我第一版直接扫原文，结果自己被自己绊倒：第 83 轮在 canvas.ts 里
      // 写的说明注释里两次提到 `spec.aoeRadius`，于是本守卫报「canvas.ts 在读它」。
      //
      // 这与 README 第 67/68/78 轮记的教训**完全同源**：
      // **正则/字符串扫描分不清「声明」与「使用」**，也分不清注释里提到的东西。
      text = stripTsComments(text)
      // 排除「技能的 aoe_radius」—— 那是另一个东西，共用字段名。
      // 只在出现 `spec.aoeRadius` / `reaction.aoeRadius` 这类
      // 「反应规格的字段」形态时才算。
      const hits = text.match(/\b(?:spec|reaction|react|r\.reaction)\w*\.aoeRadius\b/g)
      if (hits && hits.length > 0) {
        offenders.push(`${f}: ${hits.join(', ')}`)
      }
    }
    expect(
      offenders,
      `这些文件在读「反应规格的 aoeRadius」：\n  ${offenders.join('\n  ')}\n\n` +
        '它从未影响任何伤害。唯一的合理用途是「接到伤害逻辑上」，' +
        '接到表现层（屏幕震动/特效）会变成**假信号** —— ' +
        '玩家会从震动推断「炸到了」，而实际上一个敌人都没被打到。',
    ).toEqual([])
  })

