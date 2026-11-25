// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom

import { afterEach, describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { installViewModalManager } from './modalFocus'

const here = path.dirname(fileURLToPath(import.meta.url))
const repoRoot = path.resolve(here, '../../../../..')
const readView = (relative: string) => readFileSync(path.join(repoRoot, relative), 'utf8')
const flush = () => new Promise<void>((resolve) => setTimeout(resolve, 0))

/**
 * R531 / R530 §522: ProgramCenterApp 的 .detail-drawer 是 role=dialog 抽屉，
 * R530 双主题 CDP 复核 Tab 后方操作按钮 100% 被 z-index=120 抽屉覆盖，
 * 违反 WCAG 2.2 SC 2.4.11 Focus Not Obscured (Minimum)。
 * 本契约钉死把该抽屉纳入全局 modalFocus 家族：aria-modal、焦点移入、
 * Tab 陷阱、背景 aria-hidden、关闭焦点回归；并守卫家族语义与业务零改。
 */

type Fixture = {
  opener: HTMLButtonElement
  background: HTMLDivElement
  drawer: HTMLDivElement
  close: HTMLButtonElement
  last: HTMLButtonElement
  isClosed: () => boolean
}

function buildDrawer(): Fixture {
  document.body.innerHTML = ''
  const shell = document.createElement('div')
  const appRoot = document.createElement('div')
  appRoot.id = 'o2-app-root'
  const background = document.createElement('div')
  background.id = 'app-background'
  const opener = document.createElement('button')
  opener.textContent = '详情链接'
  const behind = document.createElement('button')
  behind.textContent = '删除'
  background.append(opener, behind)
  appRoot.appendChild(background)

  const drawer = document.createElement('div')
  drawer.className = 'detail-drawer'
  drawer.setAttribute('role', 'dialog')
  drawer.setAttribute('aria-label', 'Agent · 智能体甲 详情')
  const close = document.createElement('button')
  close.textContent = '关闭'
  const last = document.createElement('button')
  last.textContent = '复制 JSON'
  drawer.append(close, last)
  appRoot.appendChild(drawer)

  shell.appendChild(appRoot)
  document.body.appendChild(shell)
  opener.focus()
  let closed = false
  close.addEventListener('click', () => { closed = true; drawer.remove() })
  return { opener, background, drawer, close, last, isClosed: () => closed }
}

let dispose: () => void = () => {}

describe('R531 抽屉焦点遮挡修复契约（WCAG 2.2 SC 2.4.11）', () => {
  afterEach(() => { dispose(); document.body.innerHTML = '' })

  it('核心: .detail-drawer 纳入 OVERLAY_SELECTOR 并获 aria-modal=true, 原生 aria-label 保留', async () => {
    const { drawer } = buildDrawer()
    dispose = installViewModalManager()
    await flush()
    expect(drawer.hasAttribute('data-oa4-modal-managed')).toBe(true)
    expect(drawer.getAttribute('role')).toBe('dialog')
    expect(drawer.getAttribute('aria-modal')).toBe('true')
    expect(drawer.getAttribute('aria-label')).toBe('Agent · 智能体甲 详情')
    expect(drawer.getAttribute('aria-labelledby')).toBe(null)
  })

  it('核心: 打开抽屉焦点移入内部, 背景 aria-hidden, Tab 与 focusin 不落到后方按钮', async () => {
    const { background, drawer, close, last } = buildDrawer()
    dispose = installViewModalManager()
    await flush()
    expect(document.activeElement).toBe(close)
    expect(drawer.contains(document.activeElement)).toBe(true)
    expect(background.getAttribute('aria-hidden')).toBe('true')
    ;(background.querySelector('button') as HTMLButtonElement).focus()
    await flush()
    expect(document.activeElement).toBe(close)
    expect(drawer.contains(document.activeElement)).toBe(true)
    last.focus()
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Tab', bubbles: true }))
    expect(document.activeElement).toBe(close)
  })

  it('核心: 源码 OVERLAY_SELECTOR 末尾追加 .detail-drawer 且仍为唯一一行', () => {
    const src = readView('frontend/apps/desktop/src/utils/modalFocus.ts')
    expect(src).toContain("const OVERLAY_SELECTOR = '.modal-overlay,.upload-overlay,.paging-overlay,.modal-mask,.preview-overlay,.mo,.detail-drawer'")
    expect(src.split('\n').filter((line) => line.includes('const OVERLAY_SELECTOR')).length).toBe(1)
    expect(src).toContain("const ATTRIBUTE_MARKER = 'data-oa4-modal-managed'")
    expect(src).toContain('export function installViewModalManager(')
  })

  it('守卫: ProgramCenterApp 抽屉模板与 openDetail 业务零改', () => {
    const src = readView('frontend/apps/desktop/src/views/ProgramCenterApp.vue')
    for (const token of ['v-if="selectedDetail"', 'class="detail-drawer"', 'role="dialog"', ':aria-label=', 'selectedDetail.type || ', 'selectedDetail.name || ', ' 详情', '<button type="button" @click="selectedDetail = null">关闭</button>', 'function openDetail(', 'selectedDetail.value = { type, name: String(name ?? ', 'raw: JSON.stringify(row ?? {}, null, 2)']) {
      expect(src).toContain(token)
    }
    expect(src).not.toContain('o2AaFocusStep')
  })

  it('守卫: modalFocus 既有家族语义未被削弱（modal-overlay 仍有 dialog/aria-modal/标题命名）', async () => {
    document.body.innerHTML = ''
    const shell = document.createElement('div')
    const appRoot = document.createElement('div')
    appRoot.id = 'o2-app-root'
    shell.appendChild(appRoot)
    document.body.appendChild(shell)
    const overlay = document.createElement('div')
    overlay.className = 'modal-overlay'
    const heading = document.createElement('h2')
    heading.textContent = '既有弹窗'
    const first = document.createElement('button')
    first.textContent = '取消'
    overlay.append(heading, first)
    appRoot.appendChild(overlay)
    dispose = installViewModalManager()
    await flush()
    expect(overlay.getAttribute('role')).toBe('dialog')
    expect(overlay.getAttribute('aria-modal')).toBe('true')
    const labelledBy = overlay.getAttribute('aria-labelledby')
    expect(labelledBy).toBeTruthy()
    expect(document.getElementById(labelledBy!)).toBe(heading)
  })

  it('守卫: 关闭抽屉后焦点回归触发链接且背景 aria-hidden 恢复（行为不回归）', async () => {
    const { opener, background, close } = buildDrawer()
    dispose = installViewModalManager()
    await flush()
    close.click()
    await flush()
    expect(document.activeElement).toBe(opener)
    expect(background.hasAttribute('aria-hidden')).toBe(false)
  })
})
