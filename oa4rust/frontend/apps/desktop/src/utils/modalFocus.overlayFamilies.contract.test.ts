// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom

import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { installViewModalManager } from './modalFocus'

const flush = () => new Promise<void>((resolve) => setTimeout(resolve, 0))

/**
 * R241: IMChat 消息分页、PortalDesigner 新建门户、FormDesigner 表单预览分别用
 * paging-overlay / modal-mask / preview-overlay 做全屏遮罩浮层：都有 @click.self
 * 关闭、可见标题和独立焦点上下文，与 modal-overlay / upload-overlay 完全同构。
 * 但全局焦点管理器只枚举了后两类，导致这三个浮层没有 role=dialog / 可访问名，
 * Esc 关不掉，焦点不进入，背景也没被 aria-hidden（WCAG 2.1.2 / 2.4.3 / 4.1.2）。
 * 本契约钉死这些同类浮层必须被同一增强器覆盖。
 */
const families = [
  { className: 'paging-overlay', titleTag: 'h3' },
  { className: 'modal-mask', titleTag: 'h2' },
  { className: 'preview-overlay', titleTag: 'h2' },
] as const

function buildOverlay(className: string, titleTag: 'h2' | 'h3') {
  document.body.innerHTML = ''
  const appRoot = document.createElement('div')
  appRoot.id = 'o2-app-root'
  const background = document.createElement('div')
  background.id = 'app-background'
  background.textContent = '背景内容'
  document.body.appendChild(appRoot)
  appRoot.append(background)

  const opener = document.createElement('button')
  opener.textContent = '打开浮层'
  appRoot.appendChild(opener)
  opener.focus()

  const overlay = document.createElement('div')
  overlay.className = className
  const panel = document.createElement('div')
  const heading = document.createElement(titleTag)
  heading.textContent = '消息分页'
  const first = document.createElement('button')
  first.textContent = '关闭'
  const last = document.createElement('button')
  last.textContent = '下一页'
  panel.append(heading, first, last)
  overlay.appendChild(panel)
  appRoot.appendChild(overlay)

  let closed = false
  overlay.addEventListener('click', (event) => {
    if (event.target === overlay) {
      closed = true
      overlay.remove()
    }
  })
  return { opener, background, overlay, heading, first, last, isClosed: () => closed }
}

describe.each(families)('$className 浮层的模态语义（R241 / WCAG 2.1.2, 2.4.3, 4.1.2）', ({ className, titleTag }) => {
  let dispose: () => void = () => {}

  beforeEach(() => {
    dispose = installViewModalManager()
  })
  afterEach(() => {
    dispose()
    document.body.innerHTML = ''
  })

  it('获得 role=dialog / aria-modal 与指向可见标题的可访问名', async () => {
    const { overlay, heading } = buildOverlay(className, titleTag)
    await flush()

    expect(overlay.getAttribute('role')).toBe('dialog')
    expect(overlay.getAttribute('aria-modal')).toBe('true')
    const labelledBy = overlay.getAttribute('aria-labelledby')
    expect(labelledBy).toBeTruthy()
    expect(document.getElementById(labelledBy!)).toBe(heading)
  })

  it('Esc 触发 @click.self 关闭并把焦点还给 opener', async () => {
    const { opener, isClosed } = buildOverlay(className, titleTag)
    await flush()

    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }))
    expect(isClosed()).toBe(true)
    await flush()
    expect(document.activeElement).toBe(opener)
  })

  it('Tab 焦点被限制在浮层内', async () => {
    const { overlay, first, last } = buildOverlay(className, titleTag)
    await flush()
    expect(document.activeElement).toBe(first)

    last.focus()
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Tab', bubbles: true }))
    expect(document.activeElement).toBe(first)
    expect(overlay.contains(document.activeElement)).toBe(true)
  })

  it('打开时背景 aria-hidden，关闭后恢复', async () => {
    const { overlay, background } = buildOverlay(className, titleTag)
    await flush()
    expect(background.getAttribute('aria-hidden')).toBe('true')

    overlay.dispatchEvent(new MouseEvent('click', { bubbles: true }))
    await flush()
    expect(background.hasAttribute('aria-hidden')).toBe(false)
  })
})
