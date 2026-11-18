// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R235 集成契约：路由导航完成后 afterEach 必须触发播报。
 * 单元测试只验证 createRouteAnnouncer 本身，这里钉住 router 的接线，
 * 防止「工具写了但忘了在 afterEach 调用」这种静默失效。
 */
import { flushPromises } from '@vue/test-utils'
import { beforeEach, describe, expect, it, vi } from 'vitest'

vi.mock('@/api/client', async (importOriginal) => {
  const mod = await importOriginal<typeof import('@/api/client')>()
  return { ...mod, api: { get: vi.fn(), post: vi.fn(), put: vi.fn(), del: vi.fn() } }
})

import { invalidateTokenVerification, setToken } from '@/api/client'
import { router } from '@/router'

beforeEach(async () => {
  localStorage.clear()
  invalidateTokenVerification()
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
    ok: true, status: 200,
    text: () => Promise.resolve(JSON.stringify({ admin: { id: 1, username: 'a', role: 'super' } }))
  }))
  setToken('tok')
  await router.push('/admin/login')
  await flushPromises()
})

describe('路由导航后播报接线（第85轮）', () => {
  it('导航到带 meta.title 的页面后 live region 反映新页面', async () => {
    await router.push('/admin/users')
    await flushPromises()
    const region = document.getElementById('route-announcer')
    expect(region, 'router.afterEach 未安装播报区域').toBeTruthy()
    expect(region!.textContent).toContain('用户管理')
  })
})
