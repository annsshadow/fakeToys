// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 流程时间轴模拟的整体进度百分比（R166）。
 *
 * 历史缺陷：ProcessDesigner 的 startSimulation 用「Date.now() % totalDuration」
 * 取模，进度与已运行时间脱钩，越过 totalDuration 时从 ~100% 瞬间归零并循环，
 * 进度条来回滚且多数情况下永远走不到 >=100%（结束条件几乎不可达）。
 *
 * 本函数以「自启动起经过的毫秒数」映射到 0..100，封顶 100%，保证单调可完成。
 *
 * @param elapsedMs 自模拟启动以来经过的毫秒数
 * @param totalDurationMs 模拟总时长（毫秒）
 * @returns 0..100 的进度百分比；异常输入返回 0
 */
export function computeSimulationProgress(elapsedMs: number, totalDurationMs: number): number {
  if (!Number.isFinite(totalDurationMs) || totalDurationMs <= 0) return 0
  if (!Number.isFinite(elapsedMs) || elapsedMs < 0) return 0
  return Math.min(100, (elapsedMs / totalDurationMs) * 100)
}
