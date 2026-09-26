/**
 * 反通胀不变式在**有反应倍率**时仍然成立。
 *
 * ⚠️ 这个文件守护的是 I-1 的根基，而不只是"倍率能生效"。
 *
 * `reactionMultPermille` 曾是全链路空转：专精树的 reaction_mult 节点、
 * 装备契合度、防线装置「特斯拉栅格」都只写不读。
 * 把它接进伤害公式时，真正的难点不是"乘上去"，
 * 而是**乘上去之后占比不变式还成不成立**：
 *
 *   原来： A/(E + A) ≤ w  ⟺  A ≤ wE/(1-w)
 *   现在： A·M/(E + A·M) ≤ w  ⟺  A ≤ wE/((1-w)·M)     ← 上限必须收紧
 *
 * 直接把 M 乘进伤害而不动上限，等于给"堆一个专精节点"
 * 开了一扇绕过反通胀保证的后门 —— 那是整个设计的根基。
 *
 * 所以本文件**不测"倍率生效了"，只测"倍率任意取值时不变式都成立"**。
 * 前者一个 1000‰ 的用例就够了，后者要覆盖 0 / 500 / 1000 / 3000 / 极大。
 */
import { describe, it, expect } from 'vitest'
import { resolveHit, Defender, defaultAttacker, reactionAttackRatio, type Attacker } from './damage'
import { REACTIONS, REACTION_ORDER, type Element } from './elements'
import { MAX_REACTION_ATTACK_WEIGHT, PERMILLE, ELEMENT_PER_STACK_BASE } from './fixed'

/** 造一个带满元素层数、零抗性的守方。 */
function mkDef(stacks: bigint, hp = 1_000_000_000n): Defender {
  const d = new Defender(hp, 0n, 0n)
  d.applyElement('fire', stacks, 3n)
  return d
}

function att(over: Partial<Attacker> = {}): Attacker {
  return { ...defaultAttacker(), ...over }
}

/**
 * 对每条反应链、每个倍率取值，验证不变式。
 *
 * attack 取多档（0 / 1× / 100× / 100000×）是因为不变式最容易在
 * 「攻击侧被 cap 顶满」时失效 —— 那是占比的**最大值**。
 */
function assertInvariant(mult: bigint, label: string): void {
  for (const key of REACTION_ORDER) {
    const spec = REACTIONS[key]
    for (const stacks of [1n, 2n, 3n]) {
      for (const attackMul of [0n, 1n, 100n, 100_000n]) {
        const def = mkDef(stacks)
        const res = resolveHit(
          att({ attack: defaultAttacker().attack * attackMul, reactionMultPermille: mult }),
          def,
          {
            skillDamage: 100n,
            skillElement: 'fire',
            forceReaction: key,
            roll: 500,
          },
        )
        const ratio = reactionAttackRatio(res)
        if (ratio > MAX_REACTION_ATTACK_WEIGHT) {
          throw new Error(
            `${label}: 反应 ${key} stacks=${stacks} atk x${attackMul} ` +
              `占比 ${ratio}‰ 超过红线 ${MAX_REACTION_ATTACK_WEIGHT}‰ ` +
              `(mult=${mult})`,
          )
        }
      }
    }
    // 反应权重自身也不能越过红线（内容表层面的保证）
    if (BigInt(spec.attackWeightPct) > MAX_REACTION_ATTACK_WEIGHT) {
      throw new Error(`反应 ${key} 的权重 ${spec.attackWeightPct}‰ 越过红线`)
    }
  }
}

describe('跨端语义：reactionElement 的空值处理两端必须一致', () => {
  // ⚠️ 这条守的是 README 约束 4「同名不同义」那一类隐患。
  //
  // `reactionElement` 这个字段两端同名，但曾经对**同一个空值**给出不同抗性：
  //
  //   TS:  input.reactionElement ??  def.dominantElement()
  //        `??` 只对 null/undefined 回退 → 传 '' 时 resistOf('') = 0（无抗性）
  //   Go:  if resElement == "" { resElement = dominantElement(def) }
  //        空串**会**回退到守方主元素 → 用真实抗性
  //
  // 后果是反应伤害不同 → replayHash 不同 → I-6 把正常对局判成伪造。
  //
  // 为什么契约向量抓不到：向量里 `reaction_element` **从未出现过**
  // （实测确认）。一致性测试只在"两端传了同一个合法值"时才比对结果，
  // 而分歧发生在"传了非法值"的时候 —— 也就是说，
  // **两端的分歧恰好落在向量覆盖不到的地方**。
  //
  // 所以这里两端各写一条**语义相同**的用例：
  // Go 侧见 damage_test.go 的 TestReactionElementEmptyFallsBackToDominant。
  // 任一端回退改动，变异那一端的用例立刻红。

  function resistApplied(reactionElement: Element | undefined | ''): bigint {
    const def = new Defender(1_000_000n, 0n, 0n)
    def.applyElement('ice', 2n, 3n)
    def.resist.set('ice', 500n) // 守方对冰有 +500‰ 抗性，回退后必然体现
    const res = resolveHit(defaultAttacker(), def, {
      skillDamage: 1000n,
      skillElement: 'fire',
      forceReaction: 'flash_freeze',
      // 故意绕过类型：模拟"来自网络/DB 的类型上不可能的值"
      reactionElement: reactionElement as Element | undefined,
      roll: 500,
    })
    return res.resistAppliedPermille
  }

  it('空串与 undefined 必须产生完全相同的抗性', () => {
    // 这是本用例的全部要点：**两个空值等价**。
    // 若某一端把 '' 当成"无抗性"，500‰ 抗性就不会被应用。
    expect(resistApplied('')).toBe(resistApplied(undefined))
  })

  it('空串会回退到守方主元素（而不是变成无抗性）', () => {
    // 守方对冰 +500‰ 抗性，回退正确时应体现为 500‰
    expect(resistApplied('')).toBe(500n)
    expect(resistApplied(undefined)).toBe(500n)
  })

  it('显式传入元素时用该元素的抗性（不回落）', () => {
    // 守方对冰 +500‰、对焰 0‰。显式传 'fire' 应得到 1000‰（无抗性）。
    const def = new Defender(1_000_000n, 0n, 0n)
    def.applyElement('ice', 2n, 3n)
    def.resist.set('ice', 500n)
    def.resist.set('fire', 0n)
    const res = resolveHit(defaultAttacker(), def, {
      skillDamage: 1000n,
      skillElement: 'fire',
      forceReaction: 'flash_freeze',
      reactionElement: 'fire',
      roll: 500,
    })
    expect(res.resistAppliedPermille).toBe(1000n)
  })
})

describe('反通胀不变式：任意反应倍率下都成立', () => {
  const cases: Array<[bigint, string]> = [
    [0n, '倍率 0'],
    [1n, '倍率 1‰（极端反向）'],
    [250n, '倍率 250‰'],
    [500n, '倍率 500‰'],
    [1000n, '倍率 1000‰（无加成，基线）'],
    [2000n, '倍率 2000‰（2 倍）'],
    [3000n, '倍率 3000‰（3 倍）'],
    [10_000n, '倍率 10000‰（10 倍）'],
    [1_000_000n, '倍率 100 万‰（荒谬值）'],
  ]
  for (const [mult, label] of cases) {
    it(label, () => {
      assertInvariant(mult, label)
    })
  }
})

describe('倍率确实生效（而不只是不破坏不变式）', () => {
  /**
   * ⚠️ 关键：倍率**只在攻击侧未被 cap 顶满时才生效**。
   *
   * 攻击侧 = min(d·k/t, wE/((1-w)·M)) × M
   *   - d·k/t < cap 时 → d·k·M/t，**随 M 线性增长** ← 倍率生效
   *   - d·k/t ≥ cap 时 → wE/(1-w)，**与 M 无关**（cap 收紧恰好抵消）
   *
   * 这不是缺陷而是设计：倍率是「元素反应强度」加成，
   * 而 I-1 的红线决定了攻击侧一旦顶到上限就固定在 wE/(1-w)，
   * 再高的倍率也不能突破 —— 否则就把倍率变成了绕过红线的后门。
   *
   * 所以验证方式必须用**低攻击力**（攻击侧远未触顶）。
   * 用 attack×100000 测会得到"倍率无影响"的错误结论
   * （我第一次就是这么写的，205 → 204 让人以为没生效）。
   */
  const lowAtkDmg = (mult: bigint) =>
    resolveHit(
      att({ attack: 1n, reactionMultPermille: mult }),
      mkDef(2n),
      { skillDamage: 100n, skillElement: 'fire', forceReaction: 'steam_burst', roll: 500 },
    ).reactionDamage

  it('攻击侧未触顶时，倍率越高反应伤害越高', () => {
    expect(lowAtkDmg(1000n)).toBeLessThan(lowAtkDmg(2000n))
    expect(lowAtkDmg(2000n)).toBeLessThan(lowAtkDmg(4000n))
  })

  it('攻击侧触顶时，倍率不再改变结果（cap 恰好抵消）', () => {
    // 这是不变式的直接推论，也是必须写下来的行为：
    // 有人在数值面板上看到"倍率提升 8 倍但伤害没变"时，
    // 那是攻击侧已经顶到 I-1 的红线，不是 bug。
    const capped = (mult: bigint) =>
      resolveHit(
        att({ attack: defaultAttacker().attack * 100_000n, reactionMultPermille: mult }),
        mkDef(2n),
        { skillDamage: 100n, skillElement: 'fire', forceReaction: 'steam_burst', roll: 500 },
      ).reactionDamage
    const a = capped(1000n)
    const b = capped(8000n)
    // 不逐位相等：cap = floor(wE/((1-w)·M)) 与随后的 ×M 是**两次**截断，
    // 所以 floor(floor(x/M)·M/PERMILLE) 与 floor(x/PERMILLE) 会有 1~2 点差异。
    // 方向是保守的 —— 倍率越大结果略小，即更不会突破红线。
    expect(Number(b)).toBeLessThanOrEqual(Number(a))
    expect(Math.abs(Number(a) - Number(b)) / Number(a)).toBeLessThan(0.05)
  })

  it('倍率 1000‰ 与旧公式逐位一致（默认配置不受影响）', () => {
    // 这是"零风险接入"的关键：默认攻方是 1000‰，
    // 所以既有契约向量与平衡数据全部保持有效。
    const res = resolveHit(att({ reactionMultPermille: 1000n }), mkDef(2n), {
      skillDamage: 100n,
      skillElement: 'fire',
      forceReaction: 'steam_burst',
      roll: 500,
    })
    // 按 damage.ts 的实际顺序推导（常量从 fixed 导入，不硬编码）：
    //   d        = skillDamage × (1000 + attack) / 1000
    //   elemSide = 1200 × stacks × k/1000 / t × elementCoef/1000
    //   cap      = w × elemSide / (1000 - w)      (M=1000 时与旧式相同)
    //   attack   = min(d×k/1000/t, cap) × 1000/1000
    const k = 60n // steam_burst.baseCoef
    const w = 300n // steam_burst.attackWeightPct
    const t = defaultAttacker().reactionTier
    const d = (100n * (PERMILLE + defaultAttacker().attack)) / PERMILLE
    const elemSide = (ELEMENT_PER_STACK_BASE * 2n * k) / PERMILLE / t
    const elemSide2 = (elemSide * defaultAttacker().elementCoefPermille) / PERMILLE
    const cap = (w * elemSide2) / (PERMILLE - w)
    const raw = ((d * k) / PERMILLE / t)
    const expectAttack = (raw > cap ? cap : raw) * PERMILLE / PERMILLE
    expect(res.reactionElemPortion).toBe(elemSide2)
    expect(res.reactionAttackPortion).toBe(expectAttack)
  })

  it('倍率 0 时攻击侧归零，但元素侧不受影响', () => {
    const res = resolveHit(att({ reactionMultPermille: 0n }), mkDef(2n), {
      skillDamage: 100n,
      skillElement: 'fire',
      forceReaction: 'steam_burst',
      roll: 500,
    })
    expect(res.reactionAttackPortion).toBe(0n)
    expect(res.reactionElemPortion).toBeGreaterThan(0n)
  })
})
