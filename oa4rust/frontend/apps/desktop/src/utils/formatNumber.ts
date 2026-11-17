// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 人类可读的百分比（R164）。桌面端 FormDesigner / ProcessDesigner /
 * QueryStatementDesigner 三页原先各写一份：错误率用 (x*100).toFixed(2)%、成功率用
 * .toFixed(1)% 或 Math.round()%、方向分布用 .toFixed(1)%，精度与空值处理均不一致；
 * 空执行历史甚至固定返回 '100%'，把「无数据」误报成「全部成功」；count/0 产生
 * NaN% 时会把 'NaN%' 直接渲染给用户。本工具统一为下列契约：
 * - 入参为 0..1 的比例值（数字或数值字符串），digits 默认 1 位小数；
 * - 输出固定精度并带 % 字符串，digits=0 时输出整数百分比；
 * - 无效值（缺失、空串、非数字、NaN/Infinity、负数）与 0/0 的 NaN 一律返回 '—'。
 */
export function formatPercent(value: unknown, digits = 1): string {
  const n = typeof value === 'string' && value.trim() ? Number(value) : value
  if (typeof n !== 'number' || !Number.isFinite(n) || n < 0) return '—'
  return `${(n * 100).toFixed(digits)}%`
}
