// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 命令面板契约（AI_VISUAL_SYSTEM.md §5）。
 *
 * 真浏览器基线实测：11 个后台视图 hasCtrlK=false —— 命令面板完全缺失。
 * 本文件先钉住规范 §5 的可观测行为，再让实现转绿。
 *
 * 关键约束：导航源必须复用 AdminLayout 的 groups（单一路由表），
 * 面板里不得出现第二份硬编码标签。因此这里直接从侧栏 DOM 取期望值，
 * 一旦有人在面板里另抄一份标签，漂移立刻被本测试抓住。
 */
import { flushPromises } from '@vue/test-utils'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createMemoryHistory, createRouter, type RouteRecordRaw } from 'vue-router'
import { mountEl } from './helpers'

const Empty = { template: '<div data-test="page" />' }
const routes: RouteRecordRaw[] = [
  { path: '/admin/dashboard', component: Empty, meta: { title: '数据看板' } },
  { path: '/admin/levels', component: Empty, meta: { title: '关卡配置' } },
  { path: '/admin/skills', component: Empty, meta: { title: '技能与配方' } },
  { path: '/admin/equipment', component: Empty, meta: { title: '装备宝石皮肤' } },
  { path: '/admin/users', component: Empty, meta: { title: '用户管理' } },
  { path: '/admin/battles', component: Empty, meta: { title: '战斗记录与验真' } },
  { path: '/admin/recomputes', component: Empty, meta: { title: '结算重算审计' } },
  { path: '/admin/defenses', component: Empty, meta: { title: '防线值守' } },
  { path: '/admin/economy', component: Empty, meta: { title: '经济与商城' } },
  { path: '/admin/content', component: Empty, meta: { title: '公告与兑换码' } },
  { path: '/:pathMatch(.*)*', component: Empty },
]

async function mountLayout() {
  const router = createRouter({ history: createMemoryHistory(), routes })
  const { default: AdminLayout } = await import('@/layouts/AdminLayout.vue')
  const wrapper = mountEl(AdminLayout, {
    attachTo: document.body,
    global: { plugins: [router] },
  })
  await router.push('/admin/dashboard')
  await flushPromises()
  return { wrapper, router }
}

function ctrlK() {
  window.dispatchEvent(new KeyboardEvent('keydown', { key: 'k', ctrlKey: true, bubbles: true }))
}
function key(k: string, target: Element = document.activeElement || document.body) {
  target.dispatchEvent(new KeyboardEvent('keydown', { key: k, bubbles: true, cancelable: true }))
}

describe('AdminLayout 命令面板（§5）', () => {
  it('路由变化（浏览器前进/后退或外部跳转）时面板自动关闭，不留遮挡浮层', async () => {
    const { wrapper, router } = await mountLayout()
    ctrlK()
    await flushPromises()
    expect(document.querySelector('[role="dialog"]')).not.toBeNull()
    // 不经面板选择，直接改路由（模拟浏览器后退 / 侧栏外跳转）
    await router.push('/admin/levels')
    await flushPromises()
    expect(document.querySelector('[role="dialog"]'), '路由已变但面板残留').toBeNull()
    wrapper.unmount()
  })


  beforeEach(() => {
    document.body.innerHTML = ''
  })

  it('顶栏有可见入口并标注 Ctrl K（改造前基线 hasCtrlK=false）', async () => {
    const { wrapper } = await mountLayout()
    const trigger = wrapper.find('.command-trigger')
    expect(trigger.exists()).toBe(true)
    expect(trigger.text()).toContain('Ctrl')
    expect(trigger.text()).toContain('K')
    wrapper.unmount()
  })

  it('Ctrl+K 打开面板：role=dialog / aria-modal / 焦点入输入框；Esc 关闭并归还焦点', async () => {
    const { wrapper } = await mountLayout()
    const trigger = wrapper.find('.command-trigger')
    ;(trigger.element as HTMLElement).focus()
    expect(document.activeElement).toBe(trigger.element)

    ctrlK()
    await flushPromises()
    const panel = wrapper.find('[role="dialog"]')
    expect(panel.exists()).toBe(true)
    expect(panel.attributes('aria-modal')).toBe('true')
    const input = wrapper.find('.cmd-input')
    expect(document.activeElement).toBe(input.element)

    key('Escape', input.element)
    await flushPromises()
    expect(wrapper.find('[role="dialog"]').exists()).toBe(false)
    expect(document.activeElement).toBe(trigger.element)
    wrapper.unmount()
  })

  it('点击遮罩关闭面板', async () => {
    const { wrapper } = await mountLayout()
    ctrlK()
    await flushPromises()
    expect(wrapper.find('[role="dialog"]').exists()).toBe(true)
    await wrapper.find('.cmd-overlay').trigger('click')
    await flushPromises()
    expect(wrapper.find('[role="dialog"]').exists()).toBe(false)
    wrapper.unmount()
  })

  it('导航源复用侧栏 groups：面板选项与侧栏 10 项一一对应', async () => {
    const { wrapper } = await mountLayout()
    const sidebarTitles = wrapper.findAll('.nav-item').map((i) => i.text())
    expect(sidebarTitles).toHaveLength(10)
    ctrlK()
    await flushPromises()
    const options = wrapper.findAll('.cmd-option')
    expect(options.map((o) => o.find('.cmd-option-title').text())).toEqual(sidebarTitles)
    wrapper.unmount()
  })

  it('输入关键词过滤；无匹配显示空态与清空按钮，清空后恢复全部', async () => {
    const { wrapper } = await mountLayout()
    ctrlK()
    await flushPromises()
    const input = wrapper.find('.cmd-input')
    await input.setValue('关卡')
    await flushPromises()
    expect(wrapper.findAll('.cmd-option').map((o) => o.text())).toEqual([
      expect.stringContaining('关卡配置'),
    ])

    await input.setValue('不存在的页面xyz')
    await flushPromises()
    expect(wrapper.findAll('.cmd-option')).toHaveLength(0)
    expect(wrapper.find('.cmd-empty').text()).toContain('没有匹配的页面')
    const clear = wrapper.find('.cmd-clear')
    expect(clear.exists()).toBe(true)
    await clear.trigger('click')
    await flushPromises()
    expect(wrapper.findAll('.cmd-option')).toHaveLength(10)
    wrapper.unmount()
  })

  it('↑↓ 移动 aria-activedescendant，Enter 跳转到目标路由并关闭面板', async () => {
    const { wrapper, router } = await mountLayout()
    ctrlK()
    await flushPromises()
    const input = wrapper.find('.cmd-input')
    // 默认高亮第一项：数据看板
    expect(input.attributes('aria-activedescendant')).toContain('cmd-opt-')
    const firstId = input.attributes('aria-activedescendant')
    key('ArrowDown', input.element)
    await flushPromises()
    expect(input.attributes('aria-activedescendant')).not.toBe(firstId)
    key('ArrowUp', input.element)
    await flushPromises()
    expect(input.attributes('aria-activedescendant')).toBe(firstId)

    // 选到「关卡配置」再回车
    await input.setValue('关卡')
    await flushPromises()
    key('Enter', input.element)
    await flushPromises()
    expect(router.currentRoute.value.path).toBe('/admin/levels')
    expect(wrapper.find('[role="dialog"]').exists()).toBe(false)
    wrapper.unmount()
  })

  it('Tab 焦点陷阱：面板内循环，不逃到侧栏/顶栏', async () => {
    const { wrapper } = await mountLayout()
    ctrlK()
    await flushPromises()
    const input = wrapper.find('.cmd-input')
    const escBtn = wrapper.find('.cmd-esc')
    ;(escBtn.element as HTMLElement).focus()
    key('Tab', escBtn.element)
    await flushPromises()
    expect(document.activeElement).toBe(input.element)
    wrapper.unmount()
  })
})
