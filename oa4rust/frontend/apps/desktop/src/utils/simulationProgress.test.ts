// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { computeSimulationProgress } from './simulationProgress'

describe('computeSimulationProgress：时间轴模拟进度真实可完成（R166）', () => {
  const total = 1000

  it('从 0% 起步，经过一半时间恰好到 50%', () => {
    expect(computeSimulationProgress(0, total)).toBe(0)
    expect(computeSimulationProgress(total / 2, total)).toBe(50)
  })

  it('到达总时长时进度为 100%（模拟能自行结束，不会永远卡住）', () => {
    expect(computeSimulationProgress(total, total)).toBe(100)
  })

  it('超过总时长仍封顶 100%', () => {
    expect(computeSimulationProgress(total * 1.5, total)).toBe(100)
    expect(computeSimulationProgress(total * 3, total)).toBe(100)
  })

  it('整段模拟期间读数单调不倒退（复现进度条来回滚/归零）', () => {
    const times = [0, 250, 500, 750, 900, 1000]
    const readings = times.map((t) => computeSimulationProgress(t, total))
    for (let i = 1; i < readings.length; i++) {
      expect(
        readings[i],
        `t=${times[i]} 时进度 ${readings[i]} 小于上一读数 ${readings[i - 1]}，进度倒退`,
      ).toBeGreaterThanOrEqual(readings[i - 1])
    }
    expect(readings[readings.length - 1]).toBe(100)
  })

  it('异常输入不产生 NaN%', () => {
    expect(computeSimulationProgress(0, 0)).toBe(0)
    expect(computeSimulationProgress(500, Number.NaN)).toBe(0)
    expect(computeSimulationProgress(Number.NaN, total)).toBe(0)
    expect(computeSimulationProgress(-1, total)).toBe(0)
  })
})
