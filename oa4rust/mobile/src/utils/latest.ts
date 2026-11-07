// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 列表请求的「只认最新一次」令牌（R151）。
 *
 * 移动端 `uni.request` 封装没有暴露 RequestTask，页面无法真正 abort 在途请求；
 * 页签/列表快速切换时，旧请求晚到会把新页签的行、页码、hasMore 覆盖回去。
 * 这里用递增令牌判定「这次响应是否仍是最新一次请求」，过期响应直接丢弃。
 */
export interface LatestGate {
  /** 开始一次新请求，返回它的令牌；同时让此前所有令牌失效。 */
  begin(): number
  /** 该令牌是否仍是最新一次请求。 */
  isLatest(token: number): boolean
}

export function createLatestGate(): LatestGate {
  let current = 0
  return {
    begin: () => {
      current += 1
      return current
    },
    isLatest: (token) => token === current,
  }
}
