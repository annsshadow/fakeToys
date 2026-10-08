// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'login.vue'), 'utf8')
const guard = readFileSync(resolve(import.meta.dirname, '../../utils/auth-guard.ts'), 'utf8')

describe('login 页守卫不得自 reLaunch', () => {
  it('login 页 onShow 不调用 ensureAuthenticated（未登录时它会 reLaunch 登录页自身）', () => {
    // 微信小程序上 uni.reLaunch 当前页会重触发 onShow → 无限重载循环；
    // H5 因 router replace 同路由为 no-op 而掩盖此缺陷。
    expect(source).not.toContain('ensureAuthenticated()')
    expect(source).toContain('await session.init()')
    expect(source).toContain('if (session.isAuthenticated) navigateHome()')
  })

  it('auth-guard 自身语义不变：未认证仍 reLaunch 到登录页（供受保护页使用）', () => {
    expect(guard).toContain('uni.reLaunch({ url: LOGIN_PAGE })')
    expect(guard).toContain('if (session.isAuthenticated) return true')
  })
})
