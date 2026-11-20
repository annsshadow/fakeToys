// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 流程执行模拟的整体进度百分比（R165）。
 *
 * 历史缺陷：ProcessDesigner 的 simulateNext 用「每个节点的局部动画进度(0..100)」
 * 直接覆盖全局 execState.progress，导致节点推进时进度从 100% 掉回 0%，
 * 进度条在多个节点间反复横跳/倒退。
 *
 * 本函数以「已完成节点数」为基线，把当前节点的动画完成度映射到整体进度区间
 * [已完成基线, 下一里程碑]，保证跨节点单调不倒退。
 *
 * @param completedBefore 当前节点开始执行前已完成的节点数
 * @param totalNodes 流程节点总数
 * @param nodeAnimationPct 当前节点动画完成度（0..100）
 * @returns 0..100 的整体进度整数百分比；异常输入返回 0
 */
export function computeExecutionProgress(
  completedBefore: number,
  totalNodes: number,
  nodeAnimationPct: number,
): number {
  if (!Number.isFinite(totalNodes) || totalNodes <= 0) return 0
  if (!Number.isFinite(completedBefore) || completedBefore < 0) return 0
  if (!Number.isFinite(nodeAnimationPct)) return 0

  // 区间宽度 = 100 / 总节点数；已完成节点给出单调基线
  const perNode = 100 / totalNodes
  const base = Math.min(Math.floor(completedBefore), totalNodes) * perNode
  const advance = Math.min(Math.max(nodeAnimationPct, 0), 100) / 100
  const next = base + perNode * advance
  // 封顶 100%，杜绝 completedBefore 越界（如流程有环导致重复完成）时 >100%
  return Math.min(100, Math.max(0, Math.round(next)))
}
