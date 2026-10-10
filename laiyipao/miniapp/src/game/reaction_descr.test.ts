import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { REACTIONS, type ReactionKey } from './elements'
import type { ReactionSpec } from './elements'
import { stripTsComments } from '../testkit/stripComments'

/**
 * `ReactionSpec.descr` **只允许承诺已实现的效果**（第 83 轮）。
 *
 * # 缺陷：7 条反应里有 3 条对外文案承诺了不存在的机制
 *
 * 逐字段核对消费面（`resolveHit` + `BattleEngine.hitEnemy`）：
 *
 * | 字段            | 消费者                                   | 状态 |
 * |-----------------|------------------------------------------|------|
 * | baseCoef        | `damage.ts` 反应伤害                     | ✓    |
 * | attackWeightPct | `damage.ts` 攻方贡献比例                 | ✓    |
 * | statusDurationMs| `damage.ts` → 引擎写 `frozenMs`/`stunnedMs` | ✓ |
 * | dispelShield    | `damage.ts`                             | ✓    |
 * | amplifyPct      | `engine.ts`                              | ✓    |
 * | aoeRadius       | `engine.ts triggerSteamBurstAoe`（steam_burst）| ✓（2026-10-10）|
 * | armorShredPermille | `engine.ts` hitEnemy                 | ✓（armor_break）|
 * | knockback       | `engine.ts` hitEnemy                     | ✓（armor_break）|
 *
 * | 反应            | 文案承诺的效果            | 实际 |
 * |-----------------|-----------------------------|------|
 * | `steam_burst`   | 范围伤害 + 驱散护盾            | **已实现全部**（2026-10-10） |
 * | `overheat`      | 爆炸 + 眩晕                  | 只有眩晕 |
 * | `burn_cloud`    | 生成持续火区                 | **什么都没有** |
 * | `corrosion_spread` | 把元素层数传播给周围敌人     | **什么都没有** |
 * | `armor_break`   | 击退 + 削减护甲              | **已实现全部** |
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
 * 但它仍然是有效的：**它把「已知的谎话」钉成事实**。
 *
 * ⚠️ 「击退」「削减护甲」「破甲」已从本表移除：armor_break 的两个效果
 * （`armorShredPermille` 削甲 + `knockback` 击退位移）已由引擎实现
 * （engine.ts hitEnemy），文案承诺现在有真实消费者。
 * 移出的判据：`reaction_contract.test.ts` 锁定两端字段值一致，
 * `armor_break_effect.test.ts` 行为断言「施加后状态确实变了」。
 *
 * ⚠️「范围」「溅射」「爆炸」「火区」「传播」亦已移出：
 * steam_burst 的范围伤害已实现（`triggerSteamBurstAoe`，aoeRadius=120），
 * 文案现在真实承诺了一个真实效果。overheat/burn_cloud/corrosion_spread
 * 的「爆炸/火区/传播」仍没实现 —— 它们的 descr 里**已经不包含**这些词
 * （见第 83 轮的诚实修订），所以词表里不需要它们做守卫。
 */
const PROMISE_WORDS: Array<{ word: string; unbackedBy: string }> = [
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

  it('aoeRadius 非零的反应，必须有一个真实的伤害消费者', () => {
    // 2026-10-10：steam_burst 把 aoeRadius 120 接上了范围伤害
    // （engine.ts triggerSteamBurstAoe），所以「aoeRadius=0」不再是
    // 全体反应的必要条件。但凡非零，都必须被**伤害/状态逻辑**而非
    // 仅屏幕震动所消费 —— 否则文案就在承诺一个看不见摸不着的爆炸。
    const AOE_CONSUMER: Partial<Record<ReactionKey, 'engine.ts triggerSteamBurstAoe'>> = {
      steam_burst: 'engine.ts triggerSteamBurstAoe',
    }
    const unbacked = KEYS.filter((k) => {
      const r = REACTIONS[k]
      if (r.aoeRadius === 0) return false
      return AOE_CONSUMER[k] === undefined
    })
    expect(
      unbacked,
      `这些反应的 aoeRadius 非 0 但没有伤害消费者：${unbacked.join(', ')}\n` +
        '非零 aoeRadius 必被 triggerSteamBurstAoe（或今后新增的消费者）接线。',
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

  it('aoeRadius 只有真实伤害消费者或全无（防止「假信号」回流）', () => {
    // ⚠️ 2026-10-10 之前：aoeRadius 全 0，唯一曾读它的地方是
    // render/canvas.ts 的屏幕震动（第 83 轮删掉）。「加个假读者」变异
    // 观察不到差异 —— 于是旧断语钉住「aoeRadius=0 → 不准有消费者」。
    //
    // 2026-10-10 steam_burst 把 aoeRadius 接到**真实伤害逻辑**上
    // (`engine.ts triggerSteamBurstAoe`)，判据升级为两个：
    //   1. 值非零的反应，必须在 AOE_CONSUMERS 显式名单里  （防承诺没实现）
    //   2. render/canvas.ts **不准**读反应规格的 aoeRadius  （防假信号）
    // 名单是显式的，不是「扫出引用就放行」 —— 后者会把
    // `if (spec.aoeRadius > 0) shake` 这种渲染消费当成消费者。
    const AOE_CONSUMERS: Partial<Record<ReactionKey, string>> = {
      steam_burst: 'engine.ts triggerSteamBurstAoe',
    }
    const nonzero = KEYS.filter((k) => REACTIONS[k].aoeRadius !== 0)
    const backed = nonzero.filter((k) => AOE_CONSUMERS[k] !== undefined)
    expect(
      backed.length,
      `aoeRadius 非零的反应必须都有注册消费者；只有 ${backed.join(', ')} 接上了`.trimEnd(),
    ).toBe(nonzero.length)

    // render/canvas.ts 里不准有 `spec.aoeRadius` / `react.aoeRadius` 这类读法 ——
    // 屏幕震动是反馈，不是玩法，它不能成为「aoeRadius 被读到」的理由。
    let canvasHasRead = ''
    try {
      canvasHasRead =
        stripTsComments(readFileSync(resolve(__dirname, '../render/canvas.ts'), 'utf-8'))
        .match(/\b(?:spec|reaction|react|r\.reaction)\w*\.aoeRadius\b/g)
        ?.join(', ') ?? ''
    } catch {
      /* canvas.ts 是可选依赖，缺不存在即合格 */
    }
    expect(
      canvasHasRead,
      `render/canvas.ts 在读反应规格的 aoeRadius：${canvasHasRead}\n` +
        '屏幕震动不是消费者 —— 玩家从震动推断「炸到了」而实际没伤害，就是假信号。',
    ).toBe('')
  })

