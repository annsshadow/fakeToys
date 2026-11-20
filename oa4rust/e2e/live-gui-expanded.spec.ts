// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { expect, type Request, test } from '@playwright/test'
import { assertNo404OrServerErrors, auditApiResponses, expectSuccessfulWrite, login, uniqueFlag } from './support/live'

/**
 * 目标四：真实 GUI 交互扩充（不只调 API）。
 * 所有断言都基于真实 DOM：点击/填表/提交/分页/键盘快捷键/重定向。
 * 写动作一律由可见控件触发，API 响应仅作为「提交是否成功」的旁证。
 * 环境约定沿用 support/live.ts + global-setup.ts + playwright.config.ts：
 *   BASE_URL / E2E_USERNAME / E2E_PASSWORD 从环境变量读取，凭证绝不入库/回显。
 */

test.describe('live GUI - 登录与认证', () => {
  test('登录页空表单时提交按钮禁用', async ({ page }) => {
    await page.goto('/login')
    const submit = page.getByRole('button', { name: '登 录' })
    await expect(submit).toBeVisible()
    // LoginScreen: :disabled="loading || !username || !password"
    await expect(submit).toBeDisabled()
    await expect(page.getByPlaceholder('请输入用户名')).toBeVisible()
    await expect(page.getByPlaceholder('请输入密码')).toBeVisible()
  })

  test('密码显隐切换按钮切换 input type', async ({ page }) => {
    await page.goto('/login')
    // 部署版本密码框无 id、眼睛按钮无 aria-label，用 placeholder + .eye-btn 定位（对真实部署鲁棒）。
    const password = page.getByPlaceholder('请输入密码')
    await expect(password).toHaveAttribute('type', 'password')
    const eye = page.locator('.eye-btn')
    await eye.click()
    await expect(password).toHaveAttribute('type', 'text')
    await eye.click()
    await expect(password).toHaveAttribute('type', 'password')
  })

  test('错误密码显示行内错误且不进入工作台', async ({ page }) => {
    await page.goto('/login')
    const username = process.env.E2E_USERNAME
    const password = process.env.E2E_PASSWORD
    if (!username || !password) throw new Error('E2E_USERNAME and E2E_PASSWORD are required')
    await page.getByPlaceholder('请输入用户名').fill(username)
    // 单次错误尝试（默认锁定阈值为 5 次，不会锁定账号）；错误值不来自也不写入仓库。
    await page.getByPlaceholder('请输入密码').fill(`${password}__wrong_once`)
    await page.getByRole('button', { name: '登 录' }).click()
    await expect(page.locator('.error-msg')).toBeVisible()
    await expect(page).toHaveURL(/\/login/)
  })
})

test.describe('live GUI - 导航与应用外壳', () => {
  test.beforeEach(async ({ page }) => login(page))

  test('侧边栏导航切换路由并同步面包屑标题', async ({ page }) => {
    await expect(page).toHaveURL(/\/app\//)
    await page.locator('.sidebar-nav .nav-item', { hasText: '通讯录' }).click()
    await expect(page).toHaveURL(/\/app\/directory/)
    await expect(page.locator('.breadcrumb')).toHaveText('组织通讯录')
    await expect(page.getByRole('heading', { name: '组织通讯录' })).toBeVisible()
  })

  test('管理导航指向已注册路由而非跳登录 (R296)', async ({ page }) => {
    await expect(page).toHaveURL(/\/app\//)
    // R298: 系统管理已收敛为侧栏单一入口，点击直达管理区，不再有折叠子项
    await page.getByRole('button', { name: '⚙ 系统管理', exact: true }).click()
    await expect(page).toHaveURL(/\/app\/admin\/control-panel/)
    await expect(page).not.toHaveURL(/\/login/)
    await expect(page.getByRole('navigation', { name: '管理区导航' })).toBeVisible()
  })

  test('R297 分组侧栏切换与管理员折叠区均不跳登录', async ({ page }) => {
    await expect(page).toHaveURL(/\/app\//)
    // 员工四组齐全，disabled「工具」占位已移除
    for (const group of ['工作台', '沟通', '协作', '个人']) {
      await expect(page.locator('.nav-section-label', { hasText: group })).toBeVisible()
    }
    await expect(page.locator('.sidebar-nav').getByText('工具', { exact: true })).toHaveCount(0)

    // 员工分组项切换走真实路由，不命中兜底 → /login
    await page.getByRole('button', { name: '📒 通讯录', exact: true }).click()
    await expect(page).toHaveURL(/\/app\/directory/)
    await expect(page).not.toHaveURL(/\/login/)

    // R298: 管理区入口收敛为侧栏单一「⚙ 系统管理」，点击直达 AdminLayout 六组导航，不跳登录
    await page.getByRole('button', { name: '⚙ 系统管理', exact: true }).click()
    await expect(page).toHaveURL(/\/app\/admin\/control-panel/)
    await expect(page).not.toHaveURL(/\/login/)
    await expect(page.getByRole('navigation', { name: '管理区导航' })).toBeVisible()
  })

  test('用户菜单可打开并跳转到个人中心', async ({ page }) => {
    await page.locator('.user-menu').click()
    const menu = page.locator('.user-dropdown')
    await expect(menu).toBeVisible()
    await expect(menu.getByText('个人中心')).toBeVisible()
    await expect(menu.getByText('系统设置')).toBeVisible()
    await expect(menu.getByText('退出登录')).toBeVisible()
    await menu.getByText('个人中心').click()
    await expect(page).toHaveURL(/\/app\/personal/)
    await expect(page.getByRole('heading', { name: '个人中心' })).toBeVisible()
  })

  test('全局搜索快捷键聚焦搜索框', async ({ page }) => {
    await page.keyboard.press('Control+k')
    await expect(page.locator('input[data-oa-global-search]')).toBeFocused()
  })

  test('侧边栏折叠与展开切换外壳样式', async ({ page }) => {
    const shell = page.locator('.app-shell')
    await expect(shell).not.toHaveClass(/collapsed/)
    await page.getByRole('button', { name: '展开或收起侧边栏' }).click()
    await expect(shell).toHaveClass(/collapsed/)
    await page.getByRole('button', { name: '展开或收起侧边栏' }).click()
    await expect(shell).not.toHaveClass(/collapsed/)
    await expect(page.locator('#main-content')).toBeVisible()
  })
})

test.describe('live GUI - 表单填写与提交', () => {
  test('个人签名填写并提交后更新成功', async ({ page }) => {
    const audit = auditApiResponses(page)
    await login(page)
    await page.goto('/app/personal')

    const signature = uniqueFlag('e2e-sign')
    await page.getByPlaceholder('设置您的个性签名...').fill(signature)
    const saveResponse = audit.waitForWrite((response) =>
      /\/person\/signature\/save$/.test(new URL(response.url()).pathname),
    )
    await page.getByRole('button', { name: '保存签名' }).click()
    await expectSuccessfulWrite(await saveResponse)
    await expect(page.locator('.oa4-toast', { hasText: '签名已保存' })).toBeVisible()
    await assertNo404OrServerErrors(audit)
  })

  test('系统设置分栏切换暴露安全表单字段并操作下拉', async ({ page }) => {
    await login(page)
    await page.goto('/app/settings')
    await expect(page.getByRole('heading', { name: '系统设置' })).toBeVisible()
    await expect(page.locator('.fg', { hasText: '系统名称' }).locator('input')).toBeVisible()
    await expect(page.locator('.fg', { hasText: '时区' }).locator('select')).toBeVisible()
    await page.locator('.s-item', { hasText: '安全设置' }).click()
    const panel = page.locator('.s-panel', { hasText: '安全配置' })
    await expect(panel).toBeVisible()
    const minLen = panel.locator('.fg', { hasText: '密码最小长度' }).locator('input')
    await expect(minLen).toBeVisible()
    await minLen.fill('12')
    await expect(minLen).toHaveValue('12')
    const register = panel.locator('.fg', { hasText: '允许注册' }).locator('select')
    await register.selectOption('false')
    await expect(register).toHaveValue('false')
  })
})

test.describe('live GUI - 表格查询与分页操作', () => {
  test('通讯录搜索人员渲染名册列表与统计', async ({ page }) => {
    const audit = auditApiResponses(page)
    await login(page)
    await page.goto('/app/directory')
    await expect(page.getByRole('heading', { name: '组织通讯录' })).toBeVisible()
    // 业务化视图无自动加载：点「搜索人员」触发 person/list/filter，渲染人员列表。
    await page.getByTestId('org-search-button').click()
    const list = page.getByTestId('org-person-list')
    await expect(list).toBeVisible()
    await expect(list.locator('li.person-item').first()).toBeVisible()
    await expect(page.getByTestId('org-person-meta')).toContainText('共')
    await expect(page.getByTestId('org-person-meta')).toContainText('人')
    await assertNo404OrServerErrors(audit)
  })

  test('通讯录人员分页按钮推进页码', async ({ page }) => {
    const audit = auditApiResponses(page)
    await login(page)
    await page.goto('/app/directory')
    // 加载名册后 meta 显示「共 N 人 · 第 X / Y 页」；21 人 / 每页 20 → 2 页。
    await page.getByTestId('org-person-reload').click()
    const meta = page.getByTestId('org-person-meta')
    await expect(meta, '服务端分页信息必须渲染').toBeVisible()
    await expect(meta).toContainText('第 1 /')
    await page.getByRole('button', { name: '下一页' }).click()
    await expect(meta).toContainText('第 2 /')
    await page.getByRole('button', { name: '上一页' }).click()
    await expect(meta).toContainText('第 1 /')
    await assertNo404OrServerErrors(audit)
  })
})

test.describe('live GUI - 错误处理', () => {
  test.beforeEach(async ({ page }) => login(page))

  test('两次密码不一致时前端校验报错且不发写请求', async ({ page }) => {
    await page.goto('/app/personal')
    const writes: string[] = []
    const onRequest = (request: Request): void => {
      if (/\/person\/password$/.test(new URL(request.url()).pathname)) writes.push(request.url())
    }
    page.on('request', onRequest)
    await page.getByPlaceholder('请输入当前密码').fill('current-pass')
    await page.getByPlaceholder('请输入新密码').fill('new-pass-a')
    await page.getByPlaceholder('请再次输入').fill('new-pass-b')
    await page.getByRole('button', { name: '保存修改' }).click()
    await expect(page.locator('.error-msg', { hasText: '两次密码不一致' })).toBeVisible()
    expect(writes, '前端校验失败时不应发起密码写请求').toEqual([])
    page.off('request', onRequest)
  })

  test('密码表单填写不完整时校验报错且不发写请求', async ({ page }) => {
    await page.goto('/app/personal')
    const writes: string[] = []
    const onRequest = (request: Request): void => {
      if (/\/person\/password$/.test(new URL(request.url()).pathname)) writes.push(request.url())
    }
    page.on('request', onRequest)
    await page.getByPlaceholder('请输入当前密码').fill('current-pass')
    await page.getByRole('button', { name: '保存修改' }).click()
    await expect(page.locator('.error-msg', { hasText: '请填写完整密码' })).toBeVisible()
    expect(writes, '表单不完整时不应发起密码写请求').toEqual([])
    page.off('request', onRequest)
  })
})

test.describe('live GUI - 权限路径', () => {
  test('未登录访问受保护路由重定向到登录页并保留 redirect', async ({ page }) => {
    await page.goto('/app/dashboard')
    await expect(page).toHaveURL(/\/login/)
    expect(page.url(), '登录 URL 必须携带原目标 redirect').toContain('redirect=')
    await expect(page.getByRole('button', { name: '登 录' })).toBeVisible()
  })

  test('退出登录后受保护路由再次重定向到登录页', async ({ page }) => {
    await login(page)
    await page.locator('.user-menu').click()
    await page.locator('.user-dropdown').getByText('退出登录').click()
    await expect(page).toHaveURL(/\/login/)
    await page.goto('/app/personal')
    await expect(page).toHaveURL(/\/login/)
  })
})
