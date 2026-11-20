// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { resolveSafeRedirect } from './safeRedirect'

/**
 * 登录回跳必须留在同源站内。浏览器解析 URL 时把反斜杠与双斜杠等价处理，
 * '/\\evil.example' 看似站内相对路径，router.replace 后却会落到外部源。
 * 用 WHATWG URL 实际计算落地点，验证 helper 不会把用户带离本站。
 */
const loginPath = resolve(import.meta.dirname, '../../ui/src/components/LoginScreen.vue')
const base = 'https://oa.invalid/oa4rust/'
const baseOrigin = new URL(base).origin

function leavesOrigin(candidate: string): boolean {
  try {
    return new URL(candidate, base).origin !== baseOrigin
  } catch {
    return true
  }
}

describe('login redirect must stay on-origin', () => {
  it('falls back for off-origin targets even when they look like relative paths', () => {
    const offOrigin = [
      '//evil.example',
      '/\\evil.example',
      '/\\/evil.example',
      '/\t/evil.example',
      'https://evil.example/login',
      'javascript:alert(1)',
    ]
    for (const candidate of offOrigin) {
      expect(leavesOrigin(candidate), 'fixture must actually leave the origin: ' + candidate).toBe(true)
      expect(resolveSafeRedirect(candidate), 'must not navigate to: ' + candidate).toBe('/app/dashboard')
    }
  })

  it('preserves legitimate in-app deep links including query and fragment', () => {
    for (const candidate of ['/app/dashboard', '/app/bbs?board=1#top', '/']) {
      expect(resolveSafeRedirect(candidate)).toBe(candidate)
    }
  })

  it('handles vue-router repeatable query values and non-string inputs', () => {
    expect(resolveSafeRedirect(['/app/dashboard', '/\\evil.example'])).toBe('/app/dashboard')
    expect(resolveSafeRedirect(['/app/bbs'])).toBe('/app/bbs')
    expect(resolveSafeRedirect(undefined)).toBe('/app/dashboard')
    expect(resolveSafeRedirect(42)).toBe('/app/dashboard')
  })

  it('is wired into the login screen instead of the old startsWith guard', () => {
    const source = readFileSync(loginPath, 'utf8')
    expect(source).toContain('resolveSafeRedirect(route.query.redirect)')
    expect(source).not.toContain("candidate.startsWith('/') && !candidate.startsWith('//')")
  })
})
