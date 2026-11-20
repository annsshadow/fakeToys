// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * WCAG 2.4.3 / 4.1.3：运营后台 SPA 路由导航播报与焦点迁移（R235）。
 *
 * afterEach 此前只同步 document.title：键盘焦点仍停在点击前的侧栏导航项，
 * 读屏器也不会播报页面已切换。这里在导航完成后
 * ① 把新页面标题写入 polite live region 供读屏播报；
 * ② 把焦点移到 AdminLayout 的 #main-content，使后续 Tab 从新页面开始。
 * 登录页 / wiki 等布局外的独立路由没有 #main-content，只播报不聚焦。
 *
 * keep-alive 下主内容区常驻，焦点目标通常同步可迁移；但对异步挂载
 * 仍做有限次重试，避免 afterEach 早于 DOM 更新的边界。
 */
import type { RouteLocationNormalized } from 'vue-router'

export interface RouteAnnouncer {
  announce(to: RouteLocationNormalized): void
  dispose(): void
}

const REGION_ID = 'route-announcer'
const MAIN_ID = 'main-content'
const FALLBACK_TEXT = '页面已切换'
const RETRY_INTERVAL = 50
const MAX_RETRIES = 10
const SR_ONLY_STYLE =
  'position:absolute;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border-width:0'

export function createRouteAnnouncer(doc: Document = document): RouteAnnouncer {
  let region: HTMLElement | null = doc.getElementById(REGION_ID)
  if (!region) {
    region = doc.createElement('div')
    region.id = REGION_ID
    region.setAttribute('role', 'status')
    region.setAttribute('aria-live', 'polite')
    region.setAttribute('aria-atomic', 'true')
    region.setAttribute('style', SR_ONLY_STYLE)
    ;(doc.body ?? doc.documentElement).appendChild(region)
  }
  const live = region

  let retryTimer: ReturnType<typeof setTimeout> | undefined
  let disposed = false

  const clearRetry = (): void => {
    if (retryTimer !== undefined) {
      clearTimeout(retryTimer)
      retryTimer = undefined
    }
  }

  const tryFocus = (remaining: number): void => {
    if (disposed || remaining < 0) return
    const main = doc.getElementById(MAIN_ID)
    if (!main) {
      retryTimer = setTimeout(() => {
        retryTimer = undefined
        tryFocus(remaining - 1)
      }, RETRY_INTERVAL)
      return
    }
    main.setAttribute('tabindex', '-1')
    main.focus()
  }

  return {
    announce(to) {
      if (disposed) return
      const title = to.meta?.title as string | undefined
      live.textContent = title ? `已切换到：${title}` : FALLBACK_TEXT
      clearRetry()
      tryFocus(MAX_RETRIES)
    },
    dispose() {
      if (disposed) return
      disposed = true
      clearRetry()
      live.remove()
    },
  }
}
