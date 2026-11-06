// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 代际校验：只接受最后一次发起请求的结果（R141）。
 *
 * 典型场景：快速点选 A→B，A 的响应后到。没有校验时旧响应会把 B 的内容覆盖成 A，
 * 用户看到「点了 B 却显示 A 的消息」。发起请求前 `next()` 拿 token，落地前用
 * `isCurrent(token)` 判断，过期结果直接丢弃。
 */
export interface Generation {
  /** 开始一次新请求，返回本次 token；此前所有 token 立即过期。 */
  next(): number
  /** 该 token 是否仍是最后一次发起。 */
  isCurrent(token: number): boolean
}

export function createGeneration(): Generation {
  let current = 0
  return {
    next: () => {
      current += 1
      return current
    },
    isCurrent: (token) => token === current,
  }
}
