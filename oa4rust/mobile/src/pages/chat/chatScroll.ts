// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 会话页滚动跟随判定（R148）。
 *
 * 旧实现 `chat.vue` 的 `<scroll-view>` 没有任何滚动定位：进入会话默认停在顶部，
 * 而消息按「旧 → 新」渲染，玩家看到的是最早的消息；自己发送或轮询收到新消息后
 * 视图也不会跟到底部。这里把「该不该自动跟随」抽成纯函数，便于用确定性数值单测。
 */

/** 距底部小于该值视为「在底部」，留给手指抖动/半行高度。 */
export const CHAT_NEAR_BOTTOM_PX = 80

export interface ScrollMetrics {
  scrollTop: number
  scrollHeight: number
  clientHeight: number
}

/** 距底部距离；内容不足一屏时 scrollHeight-clientHeight 为负，钳到 0。 */
export function distanceFromBottom(metrics: ScrollMetrics): number {
  return Math.max(0, metrics.scrollHeight - metrics.clientHeight - metrics.scrollTop)
}

export function isNearBottom(metrics: ScrollMetrics, threshold = CHAT_NEAR_BOTTOM_PX): boolean {
  return distanceFromBottom(metrics) <= threshold
}

/**
 * 追加消息后是否应自动滚到底部：从未拿到尺寸（首屏刚渲染）也跟随，
 * 否则只有玩家本来就在底部附近才跟随，避免把正在翻旧消息的人拽回去。
 */
export function shouldFollowToLatest(metrics: ScrollMetrics | null, threshold = CHAT_NEAR_BOTTOM_PX): boolean {
  return metrics === null || isNearBottom(metrics, threshold)
}

/** 未跟随时要计数的「新消息」条数：一旦跟随立即清零。 */
export function pendingAfterAppend(current: number, added: number, follow: boolean): number {
  if (follow) return 0
  if (added <= 0) return current
  return current + added
}
