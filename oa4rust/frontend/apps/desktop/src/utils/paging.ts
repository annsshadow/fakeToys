// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 分页页码夹取（R170）。
 *
 * AttendanceApp 的 page 只在初始化时设为 1，切换月份或删除记录后若结果集缩小，
 * 页码仍停留在旧值，列表为空却显示「第N/1页」；BBSForum / QueryStatementDesigner
 * 都显式把页码归 1，唯独此处遗漏。本函数把「页码不得越过总页数、空结果回第 1 页」
 * 收敛为纯逻辑，便于直接构造「结果集缩小」场景做行为断言。
 *
 * 幂等：clampPage(clampPage(p, t), t) === clampPage(p, t)，
 * 因此可在 queryFn 内自纠而不触发无限重取。
 *
 * @param page 当前页码（任意数值，非整数向下取整）
 * @param totalPages 总页数（<=0 表示无结果）
 * @returns 夹取后的合法页码，最小为 1
 */
export function clampPage(page: number, totalPages: number): number {
  if (!Number.isFinite(totalPages) || totalPages < 1) return 1
  const p = Math.floor(page)
  if (!Number.isFinite(p) || p < 1) return 1
  return Math.min(p, Math.floor(totalPages))
}
