// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 搜索/筛选类页面的「只落地最后一次请求」执行器（R145）。
 *
 * 背景：R143 已有 `createRequestGuard`（`begin()` 先 abort 上一次请求），但只在 AIChatApp 使用。
 * 桌面搜索页仍是「每触发一次就无条件写列表」——慢网下快速连续搜索时，先前请求的响应
 * 后到会把最新关键词的结果覆盖掉；`OrgViewer` 更是在 `@input` 上每个字符发一次请求。
 *
 * 本模块把三件事收口为一处：
 * 1. 触发即取消上一次（真 abort，复用 `createRequestGuard`）；
 * 2. 可选防抖（对 `@input` 驱动的搜索，连续输入只发最后一次）；
 * 3. 取消错误静默（`RequestCancelledError` 不冒泡到页面，不弹「请求失败」）。
 *
 * 独立成模块而不是塞进 SFC，是为了在 node 环境用假 transport 验证「旧响应真的不会落地」。
 */
import { isRequestCancelled } from '@oa4rust/sdk'
import { createRequestGuard } from './requestGuard'

export interface RunHooks {
  /** 仅当本次仍是最新一次触发时才调用（用于复位 loading，避免旧请求提前解锁 UI）。 */
  onSettled?: () => void
}

export interface SearchRunner {
  /**
   * 执行一次搜索请求。被后续触发取代时返回 `undefined`（且不抛错、不调用 `onSettled`）；
   * 真实失败（网络/权限/服务端）才抛给调用方。
   */
  run<T>(task: (signal: AbortSignal) => Promise<T>, hooks?: RunHooks): Promise<T | undefined>
  /** 离开页面或清空关键词时调用：打断防抖等待并 abort 在途请求。 */
  cancel(): void
}

export function createSearchRunner(options: { debounceMs?: number } = {}): SearchRunner {
  const delay = Math.max(0, Math.floor(options.debounceMs ?? 0))
  const guard = createRequestGuard()
  let seq = 0
  let timer: ReturnType<typeof setTimeout> | null = null

  function clearTimer(): void {
    if (!timer) return
    clearTimeout(timer)
    timer = null
  }

  return {
    async run<T>(task: (signal: AbortSignal) => Promise<T>, hooks?: RunHooks): Promise<T | undefined> {
      const token = ++seq
      // 触发新一次搜索即取消上一次：已经在途的直接断开，还在防抖等待的直接作废。
      guard.abort()
      clearTimer()
      if (delay > 0) {
        await new Promise<void>((resolve) => {
          timer = setTimeout(() => {
            timer = null
            resolve()
          }, delay)
        })
        if (token !== seq) return undefined
      }

      const signal = guard.begin()
      try {
        return await task(signal)
      } catch (error) {
        // 取消是用户主动切走了，不是失败：静默；被更新触发取代的旧错误同样不落地。
        if (isRequestCancelled(error) || token !== seq) return undefined
        throw error
      } finally {
        if (token === seq) hooks?.onSettled?.()
      }
    },
    cancel() {
      seq += 1
      clearTimer()
      guard.abort()
    },
  }
}
