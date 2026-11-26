// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * §1A.5 语义命令入口：命令面板内 AI 洞察分区。
 *
 * 关键约束（§5 反漂移红线）：
 * - AI 命令的目标路由必须从 groups 单一真源派生，严禁硬编码第二份路由表；
 * - AI 入口是独立分区（.cmd-ai-option），不混入导航项 .cmd-option，
 *   因此「面板选项 ↔ 侧栏 10 项一一对应」契约保持不变；
 * - 语义词「洞察 / 智能 / 分析」可过滤；点击后面板关闭并跳真实路由。
 */
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'
import { flushPromises } from '@vue/test-utils'
import { beforeEach, describe, expect, it } from 'vitest'
import { createMemoryHistory, createRouter, type RouteRecordRaw } from 'vue-router'
import { mountEl } from './helpers'

const layoutSource = readFileSync(
  join(dirname(fileURLToPath(import.meta.url)), '..', 'src', 'layouts', 'AdminLayout.vue'),
  'utf8',
)

const Empty = { template: '<div data-test="page" />' }
const routes: RouteRecordRaw[] = [
  { path: '/admin/dashboard', component: Empty, meta: { title: '数据看板' } },
  { path: '/admin/levels', component: Empty, meta: { title: '关卡配置' } },
  { path: '/admin/skills', component: Empty, meta: { title: '技能与配方' } },
  { path: '/:pathMatch(.*)*', component: Empty },
]

async function mountLayout() {
  const router = createRouter({ history: createMemoryHistory(), routes })
  const { default: AdminLayout } = await import('@/layouts/AdminLayout.vue')
  const wrapper = mountEl(AdminLayout, { attachTo: document.body, global: { plugins: [router] } })
  await router.push('/admin/dashboard')
  await flushPromises()
  return { wrapper, router }
}
function ctrlK() {
  window.dispatchEvent(new KeyboardEvent('keydown', { key: 'k', ctrlKey: true, bubbles: true }))
}

describe('命令面板 AI 语义入口（§1A.5）', () => {
  beforeEach(() => { document.body.innerHTML = '' })

  it('面板含「AI 洞察」分区与 3 条规范命令文案', async () => {
    const { wrapper } = await mountLayout()
    ctrlK()
    await flushPromises()
    expect(wrapper.find('.cmd-ai-group-label').text()).toContain('AI 洞察')
    const titles = wrapper.findAll('.cmd-ai-option').map((o) => o.find('.cmd-ai-title').text())
    expect(titles).toEqual(['查看今日运营洞察', '复核流失最大关卡', '跳转到元素反应分布'])
    wrapper.unmount()
  })

  it('AI 命令目标路由从 groups 派生，不硬编码第二份路由表（§5 反漂移）', () => {
    // 断言源码里 AI 命令的 path 来自 groups[x].items[y].path，而非字面量 '/admin/…'
    // 纯字符串断言：AI 命令 path 必须从 groups[index].items[index].path 派生
    expect(layoutSource.includes('groups[')).toBe(true)
    expect(layoutSource.includes('].items[')).toBe(true)
    expect(layoutSource.includes('].path')).toBe(true)
    const aiBlock = layoutSource.slice(layoutSource.indexOf('aiCommands'), layoutSource.indexOf('aiCommands') + 900)
    expect(aiBlock.includes("path: '/admin/")).toBe(false)
    expect(aiBlock.includes('path: "/admin/')).toBe(false)
  })

  it('语义词「洞察 / 智能 / 分析」过滤出 AI 命令', async () => {
    const { wrapper } = await mountLayout()
    ctrlK()
    await flushPromises()
    const input = wrapper.find('.cmd-input')
    for (const word of ['洞察', '智能', '分析']) {
      await input.setValue(word)
      await flushPromises()
      expect(wrapper.findAll('.cmd-ai-option').length).toBeGreaterThan(0)
    }
    await input.setValue('不相关的词zzz')
    await flushPromises()
    expect(wrapper.findAll('.cmd-ai-option')).toHaveLength(0)
    wrapper.unmount()
  })

  it('点击「复核流失最大关卡」关闭面板并跳到 levels 真实路由', async () => {
    const { wrapper, router } = await mountLayout()
    ctrlK()
    await flushPromises()
    const funnel = wrapper.findAll('.cmd-ai-option').find((o) => o.text().includes('复核流失最大关卡'))!
    await funnel.trigger('click')
    await flushPromises()
    expect(router.currentRoute.value.path).toBe('/admin/levels')
    expect(wrapper.find('[role="dialog"]').exists()).toBe(false)
    wrapper.unmount()
  })

  it('AI 入口不破坏导航契约：面板仍有 10 个 .cmd-option 对应侧栏', async () => {
    const { wrapper } = await mountLayout()
    const sidebarTitles = wrapper.findAll('.nav-item').map((i) => i.text())
    ctrlK()
    await flushPromises()
    expect(wrapper.findAll('.cmd-option').map((o) => o.find('.cmd-option-title').text())).toEqual(sidebarTitles)
    wrapper.unmount()
  })
})
