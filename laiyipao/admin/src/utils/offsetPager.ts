// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 后台分页的 offset 上界，与后端 `maxQueryOffset` 对齐（R97）。
 *
 * 后端在 handler 里拒绝 offset > 25,000（400，早于 DB），因为深 offset 的
 * 扫描量由客户端决定：20 并发实测 25,000 档 p95 172ms、50,000 档 p95 330ms，
 * 预算 p95 ≤ 300ms。
 *
 * 前端不能等 400 再提示：那等于让用户先看到一次失败。做法是给
 * el-pagination 喂**封顶后的 total** —— 页码数只排到「最后一页仍合法」，
 * 「下一页」自然禁用，25,001 那种必然被拒的请求根本不会发出去。
 */
export const MAX_OFFSET = 25_000

/**
 * 把后端返回的 total 夹到「最后一页起点仍在上界内」的范围。
 *
 * 例：limit=50、真实 total=100 万 → 封顶 25,050 → 页码 501，
 * 第 501 页 offset=25,000（合法）；不存在第 502 页。
 */
export function cappedTotal(total: number, pageSize: number): number {
  // 非有限值/非正数原样返回：空表与异常值不应因为封顶而变形。
  if (!Number.isFinite(total) || total <= 0) return total
  return Math.min(total, MAX_OFFSET + pageSize)
}

/**
 * 自定义「上一页/下一页」按钮用：下一页是否会越过上界，或者本来就没有下一页。
 * 返回 false 时按钮必须禁用 —— 不是点下去再等 400。
 */
export function canGoNext(offset: number, pageSize: number, total: number): boolean {
  return offset + pageSize < total && offset + pageSize <= MAX_OFFSET
}
