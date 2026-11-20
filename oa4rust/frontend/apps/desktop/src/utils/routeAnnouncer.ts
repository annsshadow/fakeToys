// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R231 SPA 路由导航无障碍（WCAG 2.4.3 Focus Order / 4.1.3 Status Messages）。
 *
 * 视图切换后入口只同步 document.title：键盘焦点仍停在点击前的导航元素
 * （侧栏为 div role=button），读屏器也不会播报页面已切换。这里在导航完成后
 * ① 把新页面标题写入 polite live region 供读屏播报；② 把焦点移到
 * #main-content（AppShell 的 <main>），使后续 Tab 从新页面开始。
 * AppShell 的 <router-view> 带 out-in 过渡，新内容可能晚于 afterEach 挂载，
 * 因此对焦点做有限次延时重试；无 <main> 的独立路由（登录/SSO/OAuth）
 * 只播报不聚焦。
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
const MAX_RETRIES = 20
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
