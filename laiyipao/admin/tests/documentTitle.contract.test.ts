// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * WCAG 2.4.2 Page Titled：路由切换必须同步 document.title。
 *
 * 路由表每个页面都有 meta.title，但标签页标题此前一直停在 index.html 的静态值，
 * 运营在多标签间切换时无法从标签文字分辨当前页面（数据看板 / 用户管理 / ...）。
 */
import { flushPromises } from '@vue/test-utils'
import { beforeEach, describe, expect, it, vi } from 'vitest'

vi.mock('@/api/client', async (importOriginal) => {
  const mod = await importOriginal<typeof import('@/api/client')>()
  return { ...mod, api: { get: vi.fn(), post: vi.fn(), put: vi.fn(), del: vi.fn() } }
})

import { invalidateTokenVerification, setToken } from '@/api/client'
import { router } from '@/router'

function stubMe(status: number) {
  const body =
    status >= 200 && status < 300
      ? JSON.stringify({ admin: { id: 1, username: 'a', role: 'super' } })
      : JSON.stringify({ error: { code: 'unauthorized', message: 'no' } })
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
    ok: status >= 200 && status < 300,
    status,
    text: () => Promise.resolve(body),
  }))
}

beforeEach(async () => {
  localStorage.clear()
  invalidateTokenVerification()
  stubMe(200)
  setToken('tok')
  await router.push('/admin/login')
  await flushPromises()
  document.title = '占位标题'
})

describe('路由标题同步', () => {
  it('导航到有 meta.title 的页面后 document.title 反映该页面', async () => {
    await router.push('/admin/users')
    await flushPromises()
    const title = document.title
    // 必须体现具体页面名（用户管理），而不只是静态站点名。
    expect(title, '导航后标签标题仍停留在静态值').toContain('用户管理')
  })

  it('切换到另一页面后 document.title 随之变化', async () => {
    await router.push('/admin/levels')
    await flushPromises()
    const first = document.title
    expect(first).toContain('关卡配置')
    await router.push('/admin/battles')
    await flushPromises()
    const second = document.title
    expect(second).toContain('战斗记录与验真')
    expect(second).not.toBe(first)
  })

  it('登录页也有自己的标题，登录后跳后台不残留登录标题', async () => {
    await router.push('/admin/login')
    await flushPromises()
    expect(document.title).toContain('登录')
    await router.push('/admin/dashboard')
    await flushPromises()
    expect(document.title).toContain('数据看板')
    expect(document.title).not.toContain('登录')
  })
})
