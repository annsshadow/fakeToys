// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * WCAG 2.4.1 Bypass Blocks：运营后台跳转到主要内容链接契约。
 *
 * 后台有完整侧栏导航，键盘用户每次进页面都要 Tab 穿过整条导航
 * 才能到主内容区。这里把「首个可聚焦元素是跳转链接、点击把焦点
 * 移到主内容、异步挂载与 dispose 都正确」钉成行为测试。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { installSkipLink } from '@/utils/skipLink'

function loadThemeCss() {
  const here = path.dirname(fileURLToPath(import.meta.url))
  const adminRoot = path.resolve(here, '..')
  const css = readFileSync(path.join(adminRoot, 'src/styles/main.css'), 'utf-8')
  const style = document.createElement('style')
  style.textContent = css
  document.head.appendChild(style)
}

async function setup() {
  document.body.innerHTML = [
    '<div id="app">',
    '<aside class="sidebar"><a href="/dashboard">工作台</a></aside>',
    '<main id="main-content" class="content"><button>主要内容按钮</button></main>',
    '</div>',
  ].join('')
  loadThemeCss()
  const stop = installSkipLink()
  await new Promise((resolve) => setTimeout(resolve, 0))
  return {
    stop,
    root: document.getElementById('app')!,
    main: document.getElementById('main-content')!,
  }
}

describe('WCAG 2.4.1 运营后台跳转到主要内容', () => {
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

  it('安装后首个可聚焦元素是链接到主内容区的跳转链接', async () => {
    const s = await setup()
    stop = s.stop
    const first = document.body.firstElementChild
    expect(first?.tagName).toBe('A')
    expect(first?.getAttribute('href')).toBe('#main-content')
    expect(first?.textContent?.trim()).toBeTruthy()
    // 链接必须在 #app 挂载点之外，才不会被 Vue 挂载/替换清掉。
    expect(s.root.contains(first!)).toBe(false)
    // 隐藏态由真实 CSS 决定：离屏但仍在可聚焦树里。
    expect(document.defaultView!.getComputedStyle(first!).position).toBe('fixed')
  })

  it('点击跳转链接把焦点移到主内容区并设 tabindex=-1', async () => {
    const s = await setup()
    stop = s.stop
    const link = document.body.firstElementChild as HTMLAnchorElement
    link.focus()
    link.click()
    expect(document.activeElement).toBe(s.main)
    expect(s.main.getAttribute('tabindex')).toBe('-1')
  })

  it('目标在异步挂载完成前不存在也能工作', async () => {
    document.body.innerHTML = '<div id="app"></div>'
    loadThemeCss()
    stop = installSkipLink()
    await new Promise((r) => setTimeout(r, 0))
    const link = document.body.firstElementChild as HTMLAnchorElement
    expect(link.getAttribute('href')).toBe('#main-content')
    document.getElementById('app')!.innerHTML = '<main id="main-content" class="content"></main>'
    await new Promise((r) => setTimeout(r, 0))
    link.click()
    expect(document.activeElement).toBe(document.getElementById('main-content'))
    expect(document.getElementById('main-content')!.getAttribute('tabindex')).toBe('-1')
  })

  it('dispose 移除链接并断开观察者', async () => {
    const s = await setup()
    s.stop()
    expect(document.querySelector('.skip-link')).toBeNull()
  })
})
