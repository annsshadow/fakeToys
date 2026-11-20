// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { expect, test } from '@playwright/test'

/**
 * R296 P2 样板：会议管理页真机回归。
 * 验证会议页可打开，且三个任务导向业务区块（会议室预订 / 我的会议 / 日历视图）
 * 对员工可见；默认视图不再出现 /api/* 端点调试按钮。
 *
 * 注意：测试服把 OA4RUST 挂在子路径 /oa4rust/ 下，而 Playwright 的 baseURL
 * 相对导航会把 "/login" 解析到域名根（new URL('/login', baseURL)），
 * 因此这里自行按子路径拼接绝对 URL 完成登录，不复用 support/live.ts 的 login()。
 * 不修改 live-gui-expanded.spec.ts（其它子代理负责）。
 */
function appBaseUrl(): string {
  const raw = process.env.BASE_URL ?? 'http://127.0.0.1:3000'
  const url = new URL(raw)
  const path = url.pathname.replace(/\/+$/, '')
  const isLocal = url.hostname === 'localhost' || url.hostname === '127.0.0.1'
  const basePath = isLocal ? path : path || '/oa4rust'
  return url.origin + basePath
}

test('会议管理页可打开，三个业务区块可见且无端点调试按钮', async ({ page }) => {
  const username = process.env.E2E_USERNAME
  const password = process.env.E2E_PASSWORD
  if (!username || !password) throw new Error('E2E_USERNAME 和 E2E_PASSWORD 必填')

  const base = appBaseUrl()
  await page.goto(`${base}/login`)
  await page.getByPlaceholder('请输入用户名').fill(username)
  await page.getByPlaceholder('请输入密码').fill(password)
  await page.getByRole('button', { name: '登 录' }).click()
  await expect(page).toHaveURL(/\/app\//)

  await page.goto(`${base}/app/meeting`)

  await expect(page.getByRole('heading', { name: '会议管理' })).toBeVisible()
  await expect(page.getByRole('heading', { name: '会议室预订' })).toBeVisible()
  await expect(page.getByRole('heading', { name: '我的会议' })).toBeVisible()
  await expect(page.getByRole('heading', { name: '日历视图' })).toBeVisible()

  // 真实业务控件：预订表单与日历网格
  await expect(page.locator('form.booking-form')).toBeVisible()
  await expect(page.locator('.calendar-grid')).toBeVisible()

  // 默认视图不渲染端点调试按钮（旧面板把 /api/meeting/... 平铺成按钮）
  const debugButtons = page.locator('button', { hasText: '/api/' })
  await expect(debugButtons).toHaveCount(0)
  await expect(page.locator('.filter-bar .sb')).toHaveCount(0)
})
