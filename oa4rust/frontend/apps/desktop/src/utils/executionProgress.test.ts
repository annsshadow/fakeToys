// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { computeExecutionProgress } from './executionProgress'

describe('computeExecutionProgress：执行模拟进度单调不倒退（R165）', () => {
  const total = 3

  it('跨节点推进时进度不回退：上一节点终点不超过下一节点起点', () => {
    const endOfFirst = computeExecutionProgress(1, total, 100)
    const startOfSecond = computeExecutionProgress(2, total, 0)
    expect(startOfSecond).toBeGreaterThanOrEqual(endOfFirst)
  })

  it('节点动画从已完成基线起步，而非从 0% 重新爬升', () => {
    expect(computeExecutionProgress(1, total, 5)).toBeGreaterThan(30)
    expect(computeExecutionProgress(2, total, 0)).toBeGreaterThan(60)
  })

  it('整个流程的进度读数单调不倒退（复现用户可见的进度条横跳）', () => {
    const readings = [
      computeExecutionProgress(1, total, 0),
      computeExecutionProgress(1, total, 50),
      computeExecutionProgress(1, total, 100),
      computeExecutionProgress(2, total, 0),
      computeExecutionProgress(2, total, 50),
      computeExecutionProgress(2, total, 100),
      computeExecutionProgress(3, total, 0),
      computeExecutionProgress(3, total, 100),
    ]
    for (let i = 1; i < readings.length; i++) {
      expect(
        readings[i],
        `progress[${i}]=${readings[i]} 小于 progress[${i - 1}]=${readings[i - 1]}，进度倒退`,
      ).toBeGreaterThanOrEqual(readings[i - 1])
    }
    expect(readings[0]).toBe(33)
    expect(readings[readings.length - 1]).toBe(100)
  })

  it('异常输入不产生 NaN%/>100%', () => {
    expect(computeExecutionProgress(0, 0, 50)).toBe(0)
    expect(computeExecutionProgress(1, total, Number.NaN)).toBeGreaterThanOrEqual(0)
    expect(computeExecutionProgress(99, total, 100)).toBe(100)
  })
})
