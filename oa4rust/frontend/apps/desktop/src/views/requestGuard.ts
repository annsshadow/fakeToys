// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 页面级请求取消（R143）。
 *
 * 代际校验只能「丢弃旧结果」，旧请求仍会占着连接直到超时。这里维护当前在途请求的
 * `AbortController`：每次 `begin()` 先 abort 上一次，再返回新 signal 交给 SDK。
 * 放在独立模块而不是塞进 SFC，是为了在 node 环境用假 transport 直接验证
 * 「切换后旧请求真的断了」，而不是只做源码字符串匹配。
 */
export interface RequestGuard {
  /** 取消上一次请求并返回本次请求的 signal。 */
  begin(): AbortSignal
  /** 取消当前请求（离开页面/删除当前会话时调用）。 */
  abort(): void
}

export function createRequestGuard(): RequestGuard {
  let controller: AbortController | null = null
  return {
    begin() {
      controller?.abort()
      controller = new AbortController()
      return controller.signal
    },
    abort() {
      controller?.abort()
      controller = null
    },
  }
}
