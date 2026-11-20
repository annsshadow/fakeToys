// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
import fs from 'node:fs'
import path from 'node:path'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import type { RouteLocationNormalized } from 'vue-router'
import { createRouteAnnouncer } from './routeAnnouncer'

function route(title?: string): RouteLocationNormalized {
  return { meta: title === undefined ? {} : { title } } as RouteLocationNormalized
}

function announcer() {
  return createRouteAnnouncer(document)
}

let dispose: (() => void) | undefined

beforeEach(() => {
  document.body.innerHTML = ''
  vi.useRealTimers()
})

afterEach(() => {
  dispose?.()
  dispose = undefined
  document.body.innerHTML = ''
  vi.useRealTimers()
})

function region(): HTMLElement | null {
  return document.getElementById('route-announcer')
}

describe('routeAnnouncer 路由导航无障碍（WCAG 2.4.3 / 4.1.3）', () => {
  it('导航后把「已切换到：<title>」写入 polite live region', () => {
    const a = announcer()
    dispose = a.dispose
    a.announce(route('工作台'))
    const live = region()
    expect(live).not.toBeNull()
    expect(live!.getAttribute('role')).toBe('status')
    expect(live!.getAttribute('aria-live')).toBe('polite')
    expect(live!.getAttribute('aria-atomic')).toBe('true')
    expect(live!.textContent).toBe('已切换到：工作台')
  })

  it('焦点移到 #main-content 并补 tabindex=-1', () => {
    const main = document.createElement('main')
    main.id = 'main-content'
    document.body.appendChild(main)
    const a = announcer()
    dispose = a.dispose
    a.announce(route('组织架构'))
    expect(main.getAttribute('tabindex')).toBe('-1')
    expect(document.activeElement).toBe(main)
  })

  it('后续导航更新同一 live region 的文案；无 title 使用兜底文案', () => {
    const a = announcer()
    dispose = a.dispose
    a.announce(route('工作台'))
    a.announce(route('文件管理'))
    expect(region()!.textContent).toBe('已切换到：文件管理')
    a.announce(route())
    expect(region()!.textContent).toBe('页面已切换')
  })

  it('无 #main-content（登录/SSO 路由）时仅播报、不抛错', () => {
    const a = announcer()
    dispose = a.dispose
    expect(() => a.announce(route('登录'))).not.toThrow()
    expect(region()!.textContent).toBe('已切换到：登录')
    expect(document.activeElement).toBe(document.body)
  })

  it('#main-content 延迟挂载时重试聚焦', () => {
    vi.useFakeTimers()
    const a = announcer()
    dispose = a.dispose
    a.announce(route('工作台')) // 此刻无 main
    const main = document.createElement('main')
    main.id = 'main-content'
    document.body.appendChild(main)
    vi.advanceTimersByTime(100)
    expect(document.activeElement).toBe(main)
  })

  it('dispose 移除 live region，且取消待执行的重试聚焦', () => {
    vi.useFakeTimers()
    const a = announcer()
    a.announce(route('工作台')) // 无 main → 排队重试
    a.dispose()
    expect(region()).toBeNull()
    const main = document.createElement('main')
    main.id = 'main-content'
    document.body.appendChild(main)
    vi.advanceTimersByTime(1000)
    expect(document.activeElement).not.toBe(main)
  })

  it('main.ts 在 router.afterEach 内调用 announcer.announce(to)', () => {
    const main = fs.readFileSync(path.resolve(__dirname, '../main.ts'), 'utf8')
    expect(main).toContain("from './utils/routeAnnouncer'")
    expect(main).toContain('createRouteAnnouncer()')
    const afterEachAt = main.indexOf('router.afterEach')
    const announceAt = main.indexOf('routeAnnouncer.announce(to)')
    const titleAt = main.indexOf('document.title = title ?')
    expect(afterEachAt).toBeGreaterThan(-1)
    expect(titleAt).toBeGreaterThan(afterEachAt)
    expect(announceAt).toBeGreaterThan(titleAt)
  })
})
