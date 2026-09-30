/**
 * 护甲公式的跨端绝对值测试。
 *
 * ⚠️ 这个文件补的是一个**整包级的空洞**：`applyArmor` 在两端都从未被真正执行过。
 *
 * 怎么发现的：契约向量 `formula_vectors.json` 里 armor_permille 三个取值全是 0，
 * 而 applyArmor 第一行就是 `if (armorPermille <= 0) return dmg` ——
 * 也就是说这个函数在 13 个跨端向量里**从未执行到第二行**。
 * 实测把两端都改成 `return dmg`（护甲 100% 失效），
 * TS 侧 0 个测试失败，Go 侧全量 domain 测试全绿。
 * MAX_ARMOR 从 750 偷改成 100 同样没人发现。
 *
 * 现有的平衡测试（damage.test.ts / damage_test.go）也测不到：
 * 它们只断言**比值**（低护甲伤害 > 高护甲伤害），而去掉护甲会让两者
 * 同比例缩小，比值不变 —— 对「护甲整体失效」是盲的。
 *
 * 所以这里全部用**绝对值**：给定护甲 → 期望的确切伤害。
 */
import { describe, it, expect } from 'vitest'
import { applyArmor, MAX_ARMOR, PERMILLE } from './fixed'

describe('护甲：绝对值（跨端空洞的补齐）', () => {
  it('零护甲时伤害不变', () => {
    expect(applyArmor(1000n, 0n)).toBe(1000n)
  })

  it('1000‰ 护甲被 MAX_ARMOR 截到 750‰，即最多减伤 75%', () => {
    // 不是打到 0。MAX_ARMOR 的设计意图就是「减伤不超过 75%」——
    // 完全免疫会让护甲类构筑变成纯粹的数值比拼。
    expect(applyArmor(1000n, 1000n)).toBe(250n)
  })

  // ⚠️ 这条是核心：以前所有向量都是 armor_permille=0，
  // 意味着「护甲真的按比例减伤」这件事**从未被跨端验证过**。
  it('250‰ 护甲：1000 → 750', () => {
    expect(applyArmor(1000n, 250n)).toBe(750n)
  })

  it('100‰ 护甲：1000 → 900', () => {
    expect(applyArmor(1000n, 100n)).toBe(900n)
  })

  it('750‰ 护甲：2000 → 500', () => {
    expect(applyArmor(2000n, 750n)).toBe(500n)
  })

  it('扣除量随护甲线性（逐档 50‰，直到 MAX_ARMOR）', () => {
    // 用一整组断言把「线性」钉死，而不是只测一个点。
    // 少了这一步，有人把 250‰ 写成 300‰ 也可能躲过单点断言。
    //
    // ⚠️ 循环只能到 MAX_ARMOR：超出部分会被封顶，
    // 那里不是线性的（由下面「上限封顶」那组单独验证）。
    for (let permille = 0; permille <= Number(MAX_ARMOR); permille += 50) {
      const want = (2000n * BigInt(1000 - permille)) / 1000n
      expect(applyArmor(2000n, BigInt(permille))).toBe(want)
    }
  })
})

describe('护甲：上限封顶 MAX_ARMOR', () => {
  it('超过 750‰ 的护甲按 750‰ 计算（等同减伤 75%）', () => {
    expect(MAX_ARMOR).toBe(750n)
    expect(applyArmor(2000n, 900n)).toBe(500n)
    expect(applyArmor(2000n, 1000n)).toBe(500n)
    // 超大护甲也不能把伤害打到 0 以外，更不能溢出
    expect(applyArmor(2000n, 100000n)).toBe(500n)
  })

  // 这条是「MAX_ARMOR 确实是常数」的守卫。
  // 之前把 750 改成 100 不会有任何测试变红。
  it('MAX_ARMOR 的值本身被钉住', () => {
    expect(MAX_ARMOR).toBe(750n)
    expect(MAX_ARMOR).toBeLessThan(PERMILLE)
  })
})

describe('护甲：边界与异常输入', () => {
  it('负护甲原样返回（不变成增伤）', () => {
    // 这是有意的设计：MAX_ARMOR 保护的是"减伤不超过 75%"，
    // 但负护甲（增伤）不该被这套封顶逻辑顺手处理掉。
    expect(applyArmor(1000n, -500n)).toBe(1000n)
    expect(applyArmor(1000n, -1n)).toBe(1000n)
  })

  it('零伤害恒为 0', () => {
    expect(applyArmor(0n, 0n)).toBe(0n)
    expect(applyArmor(0n, 500n)).toBe(0n)
    expect(applyArmor(0n, 9999n)).toBe(0n)
  })

  it('巨大伤害值不溢出', () => {
    // bigint 不会溢出，但减法顺序写错（dmg - dmg*armor/1000 而非先乘后除）
    // 会在某些值上差 1。这条用极端值守住取整顺序。
    const huge = 10n ** 30n
    expect(applyArmor(huge, 250n)).toBe(huge * 750n / 1000n)
  })

  it('取整方向与 Go 的整数除法一致（向零截断）', () => {
    // 999 * 750 / 1000 = 749.25 -> 749（向零截断，不是四舍五入 749 同、750 不同）
    expect(applyArmor(999n, 250n)).toBe(749n)
    // 1001 * 750 / 1000 = 750.75 -> 750（向零截断，不是 751）
    expect(applyArmor(1001n, 250n)).toBe(750n)
  })
})
