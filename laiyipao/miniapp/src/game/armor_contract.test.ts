import { describe, it, expect } from 'vitest'
import vectors from '@vectors/formula_vectors.json'
import { applyArmor } from './fixed'

/**
 * applyArmor 的**跨端绝对值契约**（第 71 轮）。
 *
 * 抄的是 `server/internal/domain/armor_contract_test.go` 的分工：
 * 那个文件盯 Go 侧，本文件盯 TS 侧，两端读**同一个** JSON。
 *
 * # 缺陷：负伤害的语义两端不一致
 *
 * Go 侧有：
 *
 *     if dmg <= 0 { return dmg }   // 负伤害表示「反伤/异常」，不该被护甲改写
 *
 * TS 侧此前没有这一行，于是 `applyArmor(-1000n, 500n)` 得 -500 而 Go 得 -1000。
 *
 * # 为什么当前不可达也要修
 *
 * 三个调用点都传非负值（`damage.ts` 上游有 `if (d < 0n) d = 0n` 夹紧）。
 * 但 `fixed.ts` 的文件头写着「本文件的 applyArmor 必须与 damage.go 逐行等价」——
 * 声明与实现不符时，下一个读声明的人会按声明推理，而声明是错的。
 *
 * 且 Go 侧注释说明负伤害**将来会有语义**（反伤/异常）。
 * 那天两端分叉，而 `formula_vectors.json` 的 damage.cases 全是合法路径，
 * **没有任何测试会响**。
 *
 * 这与 README 第 9.3 节记的 `reactionElement` 空串分歧同源：
 * 契约向量只覆盖「两端传了同一个合法值」时的比对，
 * 分歧恰好在**非法输入路径**上。
 */

const cases = vectors.apply_armor.cases

describe('applyArmor 跨端契约', () => {
  it('向量非空（否则本文件会静默变成空转）', () => {
    expect(cases.length, 'formula_vectors.json 里 apply_armor.cases 是空的').toBeGreaterThan(0)
  })

  for (const c of cases) {
    it(c.name, () => {
      expect(
        applyArmor(BigInt(c.dmg), BigInt(c.armor)),
        `applyArmor(${c.dmg}, ${c.armor}) 与跨端契约不符（期望 ${c.expected}）。\n` +
          '两端只要有一端漂了，合法玩家的对局就会算出不同的伤害 → ' +
          'replayHash 不同 → I-6 判伪造。',
      ).toBe(BigInt(c.expected))
    })
  }

  it('负伤害必须原样返回（两端曾经在这里分叉）', () => {
    // 这条不依赖向量文件 —— 判据直接写在这里，
    // 防止有人「清理非法输入用例」时把向量里的负值删掉而这里还留着对照。
    for (const armor of [0n, 100n, 500n, 750n, 1000n, 999999n]) {
      expect(applyArmor(-1000n, armor), `applyArmor(-1000, ${armor}) 应原样返回 -1000`).toBe(-1000n)
      expect(applyArmor(-1n, armor), `applyArmor(-1, ${armor}) 应原样返回 -1`).toBe(-1n)
    }
    expect(applyArmor(0n, 500n), '零伤害恒为 0').toBe(0n)
  })

  it('正伤害仍然按护甲递减（确认负值守卫没有误伤正常路径）', () => {
    expect(applyArmor(1000n, 0n)).toBe(1000n)
    expect(applyArmor(1000n, 250n)).toBe(750n)
    expect(applyArmor(1000n, 750n)).toBe(250n)
    // 超过封顶被夹到 750‰ —— 减伤封顶 75% 是设计意图
    expect(applyArmor(1000n, 1000n)).toBe(250n)
    expect(applyArmor(1000n, 999999n)).toBe(250n)
  })

  it('负护甲不变成增伤（两端一致的既有语义）', () => {
    expect(applyArmor(1000n, -500n)).toBe(1000n)
    expect(applyArmor(1000n, -1n)).toBe(1000n)
  })
})