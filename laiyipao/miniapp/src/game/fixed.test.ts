/**
 * 定点整数数学的单元测试。
 *
 * ⚠️ 此前 fixed.ts 没有专属测试文件：mulDiv / clampInt64 / applyArmor
 * 被上下游间接执行，但两对**渲染边界函数** toFloat / fromFloat
 * 从未被任何测试触碰过。它们是「定点世界」与「浮点世界」的唯一关卡，
 * 一旦舍入行为漂移（如 Math.round 换成截断），拖拽坐标、HUD 血条
 * 会与引擎状态产生系统性偏位，而战斗测试完全盲视这类偏差。
 *
 * 所以这里把「边界函数的行为」直接钉死，而不只测间接后果。
 */
import { describe, it, expect } from 'vitest'
import { mulDiv, clampInt64, minBig, maxBig, toFloat, fromFloat } from './fixed'

describe('mulDiv', () => {
  it('除数为 0 时返回 0 而不是抛 RangeError', () => {
    // bigint 除 0 会直接抛异常。mulDiv 是全项目伤害公式的底座，
    // 一条畸形数据（除数 0）不应该炸掉整场战斗。
    expect(mulDiv(1n, 1n, 0n)).toBe(0n)
    expect(mulDiv(0n, 1000n, 0n)).toBe(0n)
  })

  it('先乘后除（与 Go 的整数除法一致，向零截断）', () => {
    // 若写成 (a/c)*b，999*3/1000 会先截断成 0 —— 顺序即正确性。
    expect(mulDiv(999n, 3n, 1000n)).toBe(2n)
    expect(mulDiv(500n, 1000n, 1000n)).toBe(500n)
  })
})

describe('clampInt64', () => {
  it('低于下界时钳到下界', () => {
    expect(clampInt64(-600n, -500n, 500n)).toBe(-500n)
  })

  it('高于上界时钳到上界', () => {
    expect(clampInt64(600n, -500n, 500n)).toBe(500n)
  })

  it('区间内原样返回（不就地取整）', () => {
    expect(clampInt64(-500n, -500n, 500n)).toBe(-500n)
    expect(clampInt64(500n, -500n, 500n)).toBe(500n)
    expect(clampInt64(0n, -500n, 500n)).toBe(0n)
  })
})

describe('minBig / maxBig', () => {
  it('按 bigint 数值比较（不是引用）', () => {
    expect(minBig(3n, 7n)).toBe(3n)
    expect(maxBig(3n, 7n)).toBe(7n)
    // 相等时任取一侧，结果一致
    expect(minBig(5n, 5n)).toBe(5n)
    expect(maxBig(5n, 5n)).toBe(5n)
  })
})

describe('toFloat / fromFloat（定点 ↔ 浮点的唯一关卡）', () => {
  it('定点千分比转回浮点（渲染层专用）', () => {
    expect(toFloat(1500n)).toBe(1.5)
    expect(toFloat(0n)).toBe(0)
    expect(toFloat(-250n)).toBe(-0.25)
  })

  it('支持自定义 scale（不止千分比一种用途）', () => {
    expect(toFloat(1n, 1)).toBe(1)
    expect(toFloat(250n, 100)).toBe(2.5)
  })

  it('浮点转回定点千分比，按 Math.round 舍入', () => {
    expect(fromFloat(1.5)).toBe(1500n)
    expect(fromFloat(0)).toBe(0n)
    // 恰好落在 x.5 上：round 向远离 0 的方向取整（JS 约定）
    expect(fromFloat(0.0005)).toBe(1n)
  })

  it('定点 → 浮点 → 定点 往返无损', () => {
    // 这是不变式而非巧合：拖拽输入经 fromFloat 进定点世界，
    // HUD 显示经 toFloat 出去，往返必须回到原值。
    for (const v of [0n, 1n, 999n, 1500n, 880_000n, -250n]) {
      expect(fromFloat(toFloat(v))).toBe(v)
    }
  })
})
