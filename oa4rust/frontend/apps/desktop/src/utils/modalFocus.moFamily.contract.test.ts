// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom

import fs from 'node:fs'
import path from 'node:path'
import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { installViewModalManager } from './modalFocus'

const flush = () => new Promise<void>((resolve) => setTimeout(resolve, 0))

/**
 * R242: MeetingApp 新建会议/会议详情与 QueryManager 新建查询用缩写类名 .mo 做
 * 全屏遮罩浮层（position:fixed;inset:0），同样有 @click.self 关闭、可见 h3 标题
 * 和独立焦点上下文，与 modal-overlay / upload-overlay 等族完全同构。但全局焦点
 * 管理器的 OVERLAY_SELECTOR 没有枚举 .mo，导致这三个浮层没有 role=dialog /
 * 可访问名，Esc 关不掉，焦点不进入，背景也没被 aria-hidden
 * （WCAG 2.1.2 / 2.4.3 / 4.1.2）。本契约钉死 .mo 族必须被同一增强器覆盖。
 */
describe('真实视图中的 .mo 全屏浮层族（R242）', () => {
  const views = path.resolve(__dirname, '../views')
  const meeting = fs.readFileSync(path.join(views, 'MeetingApp.vue'), 'utf-8')
  const queryManager = fs.readFileSync(path.join(views, 'QueryManager.vue'), 'utf-8')

  it('MeetingApp/QueryManager 的 .mo 是带 @click.self 关闭的全屏遮罩', () => {
    expect(meeting).toContain('<div v-if="showCreate" class="mo" @click.self="showCreate=false">')
    expect(meeting).toContain('<div v-if="detail.open" class="mo" @click.self="detail.open=false">')
    expect(queryManager).toContain('<div v-if="showCreate" class="mo" @click.self="showCreate=false">')
    // .mo 样式确实是全屏遮罩浮层，而不是内联面板
    expect(meeting).toMatch(/\.mo\{position:fixed;inset:0;[^}]*z-index:200/)
    expect(queryManager).toMatch(/\.mo\{position:fixed;inset:0;[^}]*z-index:200/)
  })
})

function buildMoOverlay() {
  document.body.innerHTML = ''
  const appRoot = document.createElement('div')
  appRoot.id = 'o2-app-root'
  const background = document.createElement('div')
  background.id = 'app-background'
  background.textContent = '背景内容'
  document.body.appendChild(appRoot)
  appRoot.append(background)

  const opener = document.createElement('button')
  opener.textContent = '新建会议'
  appRoot.appendChild(opener)
  opener.focus()

  // 与 MeetingApp.vue:103-114 / QueryManager.vue:50-56 的真实结构同构：
  // .mo 遮罩 > .modal.glass-card 面板 > h3 标题 + 表单控件 + 取消/创建按钮
  const overlay = document.createElement('div')
  overlay.className = 'mo'
  const panel = document.createElement('div')
  panel.className = 'modal glass-card'
  const heading = document.createElement('h3')
  heading.textContent = '新建会议'
  const first = document.createElement('input')
  const cancel = document.createElement('button')
  cancel.textContent = '取消'
  const last = document.createElement('button')
  last.textContent = '创建'
  panel.append(heading, first, cancel, last)
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

describe('.mo 浮层的模态语义（R242 / WCAG 2.1.2, 2.4.3, 4.1.2）', () => {
  let dispose: () => void = () => {}

  beforeEach(() => {
    dispose = installViewModalManager()
  })
  afterEach(() => {
    dispose()
    document.body.innerHTML = ''
  })

  it('获得 role=dialog / aria-modal 与指向可见标题的可访问名', async () => {
    const { overlay, heading } = buildMoOverlay()
    await flush()

    expect(overlay.getAttribute('role')).toBe('dialog')
    expect(overlay.getAttribute('aria-modal')).toBe('true')
    const labelledBy = overlay.getAttribute('aria-labelledby')
    expect(labelledBy).toBeTruthy()
    expect(document.getElementById(labelledBy!)).toBe(heading)
  })

  it('Esc 触发 @click.self 关闭并把焦点还给 opener', async () => {
    const { opener, isClosed } = buildMoOverlay()
    await flush()

    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }))
    expect(isClosed()).toBe(true)
    await flush()
    expect(document.activeElement).toBe(opener)
  })

  it('打开时焦点进入浮层，Tab 焦点被限制在浮层内', async () => {
    const { overlay, first, last } = buildMoOverlay()
    await flush()
    expect(document.activeElement).toBe(first)

    last.focus()
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Tab', bubbles: true }))
    expect(document.activeElement).toBe(first)
    expect(overlay.contains(document.activeElement)).toBe(true)
  })

  it('打开时背景 aria-hidden，关闭后恢复', async () => {
    const { overlay, background } = buildMoOverlay()
    await flush()
    expect(background.getAttribute('aria-hidden')).toBe('true')

    overlay.dispatchEvent(new MouseEvent('click', { bubbles: true }))
    await flush()
    expect(background.hasAttribute('aria-hidden')).toBe(false)
  })
})
