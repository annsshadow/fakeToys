// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R612b 契约：DocumentApp.vue 刷新/重试按钮必须绑定 script 真实存在的 doSearch，
 * 而非未定义 reload。真 Vue 挂载 + 点击触发，捕获 api.post 作为可观测信号。
 * 真父（死绑定 reload）下点击无文档列表请求；修复后点击发出真实请求。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createApp, nextTick } from 'vue'

const post = vi.fn(async () => ({ data: { list: [] } }))
vi.mock('@oa4rust/sdk', () => ({
  api: {
    post,
    get: vi.fn(async () => ({ data: [] })),
    put: vi.fn(async () => ({ data: [] })),
    delete: vi.fn(async () => ({ data: [] })),
  },
  ApiError: class ApiError extends Error {},
  isRequestCancelled: () => false,
}))

// <script setup> 的局部 import 优先于全局注册，须 vi.mock 组件模块替换。
vi.mock('../components/admin/', async () => {
  const { defineComponent, h: hh } = await import('vue')
  const Toolbar = defineComponent({
    emits: ['search', 'refresh', 'create', 'update:modelValue'],
    setup(_p: unknown, { emit, slots }) {
      return () =>
        hh('div', { class: 'admin-toolbar-stub' }, [
          hh('button', { class: 'toolbar-refresh', onClick: () => emit('refresh') }, '刷新'),
          slots.filters?.(),
          slots.actions?.(),
        ])
    },
  })
  const Table = defineComponent({
    emits: ['retry'],
    setup(_p: unknown, { emit }) {
      return () =>
        hh('div', { class: 'admin-table-stub' }, [
          hh('button', { class: 'table-retry', onClick: () => emit('retry') }, '重试'),
        ])
    },
  })
  const Plain = defineComponent({ setup: () => () => hh('div') })
  return { AdminToolbar: Toolbar, AdminTable: Table, AdminPageHeader: Plain, AdminFormSection: Plain }
})

const vueWarn = vi.spyOn(console, 'warn').mockImplementation(() => {})
const vueErr = vi.spyOn(console, 'error').mockImplementation(() => {})

const src = readFileSync(resolve(import.meta.dirname, 'DocumentApp.vue'), 'utf8')
const tpl = src.slice(src.indexOf('<template>'), src.lastIndexOf('</template>'))

async function mountDocument() {
  const { default: DocumentApp } = await import('./DocumentApp.vue')
  const host = document.createElement('div')
  document.body.appendChild(host)
  const app = createApp(DocumentApp)
  app.mount(host)
  await nextTick()
  return { host, app }
}

function click(host: HTMLElement, selector: string) {
  const btn = host.querySelector(selector) as HTMLButtonElement | null
  if (!btn) throw new Error(`missing button: ${selector}`)
  btn.click()
}

function listRequested() {
  return post.mock.calls.some((c: unknown[]) => String(c[0]).includes('/api/document/filter/list/1/size/50/manager'))
}

describe('R612b DocumentApp 刷新/重试按钮真绑定 doSearch（jsdom 真挂载）', () => {
  beforeEach(() => {
    post.mockClear()
    vueWarn.mockClear()
    vueErr.mockClear()
  })

  it('源码锚点：@refresh/@retry 均绑定 doSearch，不再引用 reload', () => {
    expect(tpl).toContain('@refresh="doSearch"')
    expect(tpl).toContain('@retry="doSearch"')
    expect(tpl).not.toContain('@refresh="reload"')
    expect(tpl).not.toContain('@retry="reload"')
  })

  it('点击工具栏刷新触发真实文档列表 POST 请求', async () => {
    const { host, app } = await mountDocument()
    post.mockClear() // 清掉 setup 期 doSearch() 的初始化请求
    click(host, '.toolbar-refresh')
    await nextTick()
    await Promise.resolve()
    expect(listRequested()).toBe(true)
    expect(vueWarn.mock.calls.join(' ')).not.toContain('reload')
    app.unmount()
    host.remove()
  })

  it('点击表格重试触发真实文档列表请求且无未定义 handler 警告', async () => {
    const { host, app } = await mountDocument()
    post.mockClear()
    click(host, '.table-retry')
    await nextTick()
    await Promise.resolve()
    expect(listRequested()).toBe(true)
    const warns = vueWarn.mock.calls.join(' ') + vueErr.mock.calls.join(' ')
    expect(warns).not.toContain('reload')
    app.unmount()
    host.remove()
  })
})
