// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import { createPinia } from 'pinia'

import { installUniMock } from '@/test/uni-mock'

// 公告数据契约来自服务端 AnnouncementView：published_at 是 time.Time，
// json.Marshal 序列化为 RFC3339 UTC 字符串（例如 "2026-10-31T17:00:00Z"）。
// 前端 formatDate 用的是本地日历（getFullYear/...），所以分组键也必须用本地日历，
// 否则「按月归档」的组标题会与组内卡片的日期自相矛盾。
vi.mock('@/api/client', () => ({
  fetchAnnouncements: vi.fn(),
}))

import * as api from '@/api/client'
import AnnouncementsPage from './announcements.vue'

const mockFetch = vi.mocked(api.fetchAnnouncements, true)

async function mountPage() {
  const pinia = createPinia()
  const wrapper = mount(AnnouncementsPage, {
    global: {
      plugins: [pinia],
      stubs: { EnvVersionTag: true },
    },
  })
  await flushPromises()
  await flushPromises()
  return wrapper
}

describe('公告按月归档：分组标题与卡片日期必须同一日历', () => {
  const savedTZ = process.env.TZ

  beforeEach(() => {
    // 中国大陆小程序运行在 UTC+8；用运行时 TZ 复现（Node 会在后续 new Date 时使用新 TZ）。
    process.env.TZ = 'Asia/Shanghai'
    installUniMock()
    vi.clearAllMocks()
  })

  afterEach(() => {
    if (savedTZ === undefined) delete process.env.TZ
    else process.env.TZ = savedTZ
    vi.unstubAllGlobals()
    vi.clearAllMocks()
  })

  it('UTC 月末但本地月初的公告，不应被放进 UTC 月份的组标题下', async () => {
    // 2026-10-31T17:00:00Z 在北京时间是 2026-11-01 01:00，
    // 卡片显示「2026-11-01 01:00」，组标题就不能写「2026-10」。
    mockFetch.mockResolvedValue({
      items: [
        { id: 1, title: '跨月公告', body: '内容', published_at: '2026-10-31T17:00:00Z' },
      ],
    })

    const wrapper = await mountPage()
    const labels = wrapper.findAll('.month-label').map((n) => n.text())
    const date = wrapper.find('.ann-date').text()

    expect(date).toBe('2026-11-01 01:00')
    expect(labels).toContain('2026-11')
    expect(labels).not.toContain('2026-10')
  })

  it('UTC 月初但本地仍属上月末的公告，也不应被放进 UTC 月份的组标题下', async () => {
    // 负偏移时区反向边界：2026-11-01T02:00:00Z 在纽约是 2026-10-31 22:00，
    // 分组标题必须是「2026-10」而不是字符串前缀的「2026-11」。
    process.env.TZ = 'America/New_York'
    mockFetch.mockResolvedValue({
      items: [
        { id: 2, title: '跨月公告二', body: '内容', published_at: '2026-11-01T02:00:00Z' },
      ],
    })

    const wrapper = await mountPage()
    const labels = wrapper.findAll('.month-label').map((n) => n.text())
    const date = wrapper.find('.ann-date').text()

    expect(date).toBe('2026-10-31 22:00')
    expect(labels).toContain('2026-10')
    expect(labels).not.toContain('2026-11')
  })
})
