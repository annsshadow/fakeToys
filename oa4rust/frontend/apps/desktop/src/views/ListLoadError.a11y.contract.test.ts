// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R226 列表加载失败状态消息契约测试（WCAG 4.1.3 Status Messages AA、3.3.1 Error Identification）。
 *
 * ListLoadError.vue 是桌面端 40+ CRUD 视图共用的加载失败组件（全仓 224 处使用），
 * 失败文案由 v-if/v-else 在异步加载失败后动态插入 DOM。原实现两个分支的根元素
 * （.list-error-empty / .list-error-banner）均无 role=alert / aria-live，
 * 读屏用户听不到「加载失败」，只能自行逐个浏览才发现——违反 WCAG 4.1.3。
 * 契约要求：两个分支都把失败文案放进 role=alert / aria-live=assertive 容器，
 * 且文案与重试按钮的可访问结构保持完整。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { afterEach, beforeEach, describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const templateSource = readFileSync(path.resolve(here, './ListLoadError.vue'), 'utf8')

/** 截取一个元素标签，断言根元素上带有指定的 alert 语义。 */
function rootTag(cls: string): string {
  const idx = templateSource.indexOf('class="' + cls + '"')
  expect(idx, '未找到 .' + cls + ' 根元素').toBeGreaterThan(-1)
  const start = templateSource.lastIndexOf('<div', idx)
  const end = templateSource.indexOf('>', idx)
  expect(start, 'class 前未找到 <div').toBeGreaterThan(-1)
  expect(end, '未找到标签闭合 >').toBeGreaterThan(idx)
  return templateSource.slice(start, end + 1)
}

/** 用真实 DOM 反映两个分支的渲染结果，验证读屏可见语义与文案。 */
function renderBranch(mode: 'empty' | 'banner', message: string): HTMLElement {
  const root = document.createElement('div')
  root.className = mode === 'empty' ? 'list-error-empty' : 'list-error-banner'
  root.setAttribute('role', rootTag(root.className).includes('role="alert"') ? 'alert' : '')
  if (rootTag(root.className).includes('aria-live="assertive"')) {
    root.setAttribute('aria-live', 'assertive')
  }
  const text = document.createElement(mode === 'empty' ? 'p' : 'span')
  text.className = 'list-error-text'
  text.textContent = message
  const retry = document.createElement('button')
  retry.className = 'list-error-retry'
  retry.textContent = '重试'
  root.append(text, retry)
  document.body.appendChild(root)
  return root
}

beforeEach(() => {
  document.body.innerHTML = ''
})
afterEach(() => {
  document.body.innerHTML = ''
})

describe('ListLoadError 加载失败状态消息（R226 / WCAG 4.1.3、3.3.1）', () => {
  it('empty 分支根元素带 role=alert 与 aria-live=assertive', () => {
    const tag = rootTag('list-error-empty')
    expect(tag).toMatch(/role="alert"/)
    expect(tag).toMatch(/aria-live="assertive"/)
  })

  it('banner 分支根元素带 role=alert 与 aria-live=assertive', () => {
    const tag = rootTag('list-error-banner')
    expect(tag).toMatch(/role="alert"/)
    expect(tag).toMatch(/aria-live="assertive"/)
  })

  it('渲染后的两个分支都是读屏可播报的错误状态且文案完整', () => {
    for (const mode of ['empty', 'banner'] as const) {
      const root = renderBranch(mode, '加载失败，请稍后重试')
      expect(root.getAttribute('role')).toBe('alert')
      expect(root.getAttribute('aria-live')).toBe('assertive')
      expect(root.querySelector('.list-error-text')?.textContent).toBe('加载失败，请稍后重试')
      expect(root.querySelector('.list-error-retry')?.textContent).toBe('重试')
    }
  })
})
