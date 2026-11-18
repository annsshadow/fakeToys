// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
import fs from 'node:fs'
import path from 'node:path'
import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { installLoadingA11y } from './loadingA11y'

function flush(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0))
}

function skeleton(): HTMLElement {
  const host = document.createElement('div')
  host.innerHTML = '<div class="loading-state"><div class="skel"></div><div class="skel"></div></div>'
  document.body.appendChild(host)
  return host.querySelector('.loading-state') as HTMLElement
}

let dispose: (() => void) | undefined

beforeEach(() => {
  document.body.innerHTML = ''
})

afterEach(() => {
  dispose?.()
  dispose = undefined
  document.body.innerHTML = ''
})

describe('loadingA11y 加载状态无障碍（WCAG 4.1.3 Status Messages）', () => {
  it('纯骨架加载容器补 role=status 与读屏可见的“加载中”文本', async () => {
    dispose = installLoadingA11y(document.body)
    const el = skeleton()
    await flush()
    expect(el.getAttribute('role')).toBe('status')
    expect(el.textContent).toContain('加载中')
  })

  it('已有 role=status 的容器不被重复注入文本', async () => {
    dispose = installLoadingA11y(document.body)
    const host = document.createElement('div')
    host.innerHTML = '<div class="loading-state" role="status">加载中...</div>'
    document.body.appendChild(host)
    await flush()
    const el = host.firstElementChild as HTMLElement
    expect(el.getAttribute('role')).toBe('status')
    expect(el.textContent?.trim()).toBe('加载中...')
  })

  it('尊重作者显式声明的其它 role，不覆盖也不注入', async () => {
    dispose = installLoadingA11y(document.body)
    const host = document.createElement('div')
    host.innerHTML = '<div class="loading-state" role="progressbar" aria-valuenow="40">40%</div>'
    document.body.appendChild(host)
    await flush()
    const el = host.firstElementChild as HTMLElement
    expect(el.getAttribute('role')).toBe('progressbar')
    expect(el.textContent?.trim()).toBe('40%')
  })

  it('已有加载文案但无 role 的容器补 role 且不重复文案', async () => {
    dispose = installLoadingA11y(document.body)
    const host = document.createElement('div')
    host.innerHTML = '<div class="loading-indicator">加载中...</div>'
    document.body.appendChild(host)
    await flush()
    const el = host.firstElementChild as HTMLElement
    expect(el.getAttribute('role')).toBe('status')
    expect(el.textContent?.trim()).toBe('加载中...')
  })

  it('dispose 还原注入的属性与文本，且不再处理新插入的容器', async () => {
    const d = installLoadingA11y(document.body)
    const el = skeleton()
    await flush()
    expect(el.getAttribute('role')).toBe('status')
    d()
    expect(el.hasAttribute('role')).toBe(false)
    expect(el.textContent).toBe('')
    const el2 = skeleton()
    await flush()
    expect(el2.hasAttribute('role')).toBe(false)
  })

  it('主入口在挂载应用前安装加载状态增强器', () => {
    const main = fs.readFileSync(path.resolve(__dirname, '../main.ts'), 'utf8')
    const installAt = main.indexOf('installLoadingA11y()')
    const mountAt = main.indexOf("app.mount('#o2-app-root')")
    expect(installAt).toBeGreaterThan(-1)
    expect(mountAt).toBeGreaterThan(installAt)
  })
})
