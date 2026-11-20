// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom

import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { installSkipLink } from './skipLink'

/** Read the real theme CSS and feed it to jsdom so layouts are exercised. */
function loadThemeCss() {
  const here = path.dirname(fileURLToPath(import.meta.url))
  const frontendRoot = path.resolve(here, '../../../..')
  const css = readFileSync(path.join(frontendRoot, 'packages/ui/src/theme/dark.css'), 'utf-8')
  const style = document.createElement('style')
  style.textContent = css
  document.head.appendChild(style)
}

/** End state after install + asynchronous app mount. */
async function setup() {
  document.body.innerHTML = `
    <div id="o2-app-root">
      <aside class="sidebar"><a href="/app/dashboard">工作台</a></aside>
      <main id="main-content" class="content"><button>主要内容按钮</button></main>
    </div>
  `
  loadThemeCss()
  const stop = installSkipLink()
  await new Promise((resolve) => setTimeout(resolve, 0))
  return {
    stop,
    root: document.getElementById('o2-app-root')!,
    main: document.getElementById('main-content')!,
  }
}

describe('WCAG 2.4.1 跳转到主要内容（全局跳转链接）', () => {
  let stop: () => void = () => {}

  beforeEach(() => {
    document.head.innerHTML = ''
    document.body.innerHTML = ''
  })

  afterEach(() => {
    stop()
    document.head.innerHTML = ''
    document.body.innerHTML = ''
  })

  it('安装后首个可聚焦元素是链接到主内容区的可见跳转链接', async () => {
    const setupResult = await setup()
    stop = setupResult.stop

    const first = document.body.firstElementChild
    expect(first?.tagName).toBe('A')
    expect(first?.getAttribute('href')).toBe('#main-content')
    expect(first?.textContent?.trim()).toBeTruthy()
    // 链接必须在应用挂载点之外，才不会被 Vue 的挂载/替换清掉。
    expect(setupResult.root.contains(first!)).toBe(false)
    // 隐藏态由真实主题 CSS 决定：离屏但仍在可聚焦树里。
    const position = document.defaultView!.getComputedStyle(first!).position
    expect(position).toBe('fixed')
  })

  it('点击跳转链接把焦点移到主内容区，并让键盘焦点环可见', async () => {
    const { stop: dispose, main } = await setup()
    stop = dispose

    const link = document.body.firstElementChild as HTMLAnchorElement
    link.focus()
    link.click()

    expect(document.activeElement).toBe(main)
    expect(main.getAttribute('tabindex')).toBe('-1')
    // 聚焦目标不能留下永久轮廓；只在 :focus-visible 时显示。
    expect(document.head.querySelector('style')?.textContent).toContain('main.content:focus:not(:focus-visible)')
  })

  it('目标在主内容区挂载完成前不存在的异步挂载场景也能工作', async () => {
    document.body.innerHTML = '<div id="o2-app-root"></div>'
    loadThemeCss()
    stop = installSkipLink()
    await new Promise((resolve) => setTimeout(resolve, 0))

    const link = document.body.firstElementChild as HTMLAnchorElement
    expect(link.getAttribute('href')).toBe('#main-content')

    const root = document.getElementById('o2-app-root')!
    const main = document.createElement('main')
    main.id = 'main-content'
    root.appendChild(main)
    await new Promise((resolve) => setTimeout(resolve, 0))

    link.focus()
    link.click()
    expect(document.activeElement).toBe(main)
  })

  it('卸载时移除链接并停止监听，不留全局副作用', async () => {
    const { stop: dispose } = await setup()
    dispose()
    stop = () => {}

    expect(document.querySelector('a[href="#main-content"]')).toBeNull()
    const root = document.getElementById('o2-app-root')!
    const main = document.createElement('main')
    main.id = 'main-content'
    root.appendChild(main)
    await new Promise((resolve) => setTimeout(resolve, 0))
    expect(main.getAttribute('tabindex')).toBeNull()
  })
})
