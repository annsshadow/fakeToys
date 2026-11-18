// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * WCAG 2.4.3 / 4.1.3：运营后台 SPA 路由切换后须播报新页面并迁移焦点。
 * 此前 afterEach 只同步 document.title，键盘焦点仍停在侧栏导航项，
 * 读屏器不会播报「已切换页面」。本测试钉住播报 live region 与焦点迁移。
 */
import { afterEach, describe, expect, it } from 'vitest'
import { createRouteAnnouncer } from '@/utils/routeAnnouncer'

function resetDom() {
  document.body.innerHTML = ''
}

afterEach(resetDom)

describe('运营后台路由导航播报与焦点迁移（第85轮）', () => {
  it('播报写入 role=status / aria-live=polite 的 sr-only 区域', () => {
    const a = createRouteAnnouncer(document)
    a.announce({ meta: { title: '用户管理' } } as never)
    const region = document.getElementById('route-announcer')!
    expect(region).toBeTruthy()
    expect(region.getAttribute('role')).toBe('status')
    expect(region.getAttribute('aria-live')).toBe('polite')
    expect(region.textContent).toContain('用户管理')
    a.dispose()
  })

  it('焦点迁移到 #main-content 并补 tabindex=-1', () => {
    const main = document.createElement('main')
    main.id = 'main-content'
    document.body.appendChild(main)
    const a = createRouteAnnouncer(document)
    a.announce({ meta: { title: '关卡配置' } } as never)
    expect(main.getAttribute('tabindex')).toBe('-1')
    expect(document.activeElement).toBe(main)
    a.dispose()
  })

  it('无 meta.title 时播报兜底文案，不写空串', () => {
    const a = createRouteAnnouncer(document)
    a.announce({} as never)
    expect(document.getElementById('route-announcer')!.textContent).toBeTruthy()
    a.dispose()
  })

  it('dispose 移除播报区域并停止后续播报', () => {
    const a = createRouteAnnouncer(document)
    a.announce({ meta: { title: 'x' } } as never)
    a.dispose()
    expect(document.getElementById('route-announcer')).toBeNull()
  })
})
