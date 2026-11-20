// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment jsdom

import fs from 'node:fs'
import path from 'node:path'
import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { installProgressA11y, type ProgressA11yHandle } from './progressA11y'

// 无障碍判据（WCAG 4.1.2 Name, Role, Value）：所有进度条是纯视觉 <div>，存在百分比文本
// 却无任何 ARIA 语义，屏幕阅读器用户无法获知上传/模拟/执行进度与数值。
describe('progressA11y：进度条 ARIA 语义（R216 契约）', () => {
  let host: HTMLDivElement
  let handle: ProgressA11yHandle

  beforeEach(() => {
    document.body.innerHTML = ''
    host = document.createElement('div')
    document.body.appendChild(host)
    handle = installProgressA11y()
  })

  afterEach(() => {
    handle.dispose()
    document.body.innerHTML = ''
  })

  const nextTick = () => new Promise<void>((resolve) => setTimeout(resolve, 0))

  it('为 FileManager 上传进度条补 role=progressbar 与 min/max/now/label', async () => {
    host.innerHTML =
      '<div class="progress-bar"><div class="progress-fill" style="width: 42%"></div><span>42%</span></div>'
    await nextTick()
    const bar = host.querySelector('.progress-bar')!
    expect(bar.getAttribute('role')).toBe('progressbar')
    expect(bar.getAttribute('aria-valuemin')).toBe('0')
    expect(bar.getAttribute('aria-valuemax')).toBe('100')
    expect(bar.getAttribute('aria-valuenow')).toBe('42')
    expect(bar.getAttribute('aria-label')).toBe('上传进度')
  })

  it('为表单模拟进度条补完整 progressbar 语义', async () => {
    host.innerHTML =
      '<div class="sim-progress"><div class="sim-bar-wrap"><div class="sim-bar" style="width:70%"></div></div><span>70%</span></div>'
    await nextTick()
    const bar = host.querySelector('.sim-progress')!
    expect(bar.getAttribute('role')).toBe('progressbar')
    expect(bar.getAttribute('aria-valuenow')).toBe('70')
    expect(bar.getAttribute('aria-valuemin')).toBe('0')
    expect(bar.getAttribute('aria-valuemax')).toBe('100')
    expect(bar.getAttribute('aria-label')).toBe('模拟进度')
  })

  it('为 ProcessDesigner 步骤模拟 / 执行 / 回放进度条补语义', async () => {
    host.innerHTML = [
      '<div class="st-progress"><div class="st-bar" style="width:55%"></div><span class="st-pct">55%</span></div>',
      '<div class="exec-bar"><div class="exec-bar-fill" style="width: 80%"></div></div>',
      '<div class="tr-progress"><div class="tr-bar" style="width:33%"></div><span>1/3</span></div>',
    ].join('')
    await nextTick()
    const st = host.querySelector('.st-progress')!
    const exec = host.querySelector('.exec-bar')!
    const tr = host.querySelector('.tr-progress')!
    expect(st.getAttribute('role')).toBe('progressbar')
    expect(st.getAttribute('aria-valuenow')).toBe('55')
    expect(st.getAttribute('aria-label')).toBe('模拟进度')
    expect(exec.getAttribute('role')).toBe('progressbar')
    expect(exec.getAttribute('aria-valuenow')).toBe('80')
    expect(exec.getAttribute('aria-label')).toBe('执行进度')
    expect(tr.getAttribute('role')).toBe('progressbar')
    expect(tr.getAttribute('aria-valuenow')).toBe('33')
    expect(tr.getAttribute('aria-label')).toBe('回放进度')
  })

  it('进度值随 DOM 变化实时更新', async () => {
    host.innerHTML =
      '<div class="progress-bar"><div class="progress-fill" style="width: 10%"></div><span>10%</span></div>'
    await nextTick()
    const bar = host.querySelector('.progress-bar') as HTMLElement
    expect(bar.getAttribute('aria-valuenow')).toBe('10')
    bar.querySelector('.progress-fill')!.setAttribute('style', 'width: 90%')
    bar.querySelector('span')!.textContent = '90%'
    await nextTick()
    expect(bar.getAttribute('aria-valuenow')).toBe('90')
  })

  it('尊重作者已有的 role 与 aria-label，不覆盖', async () => {
    host.innerHTML =
      '<div class="progress-bar" role="progressbar" aria-label="自定义进度"><div class="progress-fill" style="width: 50%"></div><span>50%</span></div>'
    await nextTick()
    const bar = host.querySelector('.progress-bar')!
    expect(bar.getAttribute('role')).toBe('progressbar')
    expect(bar.getAttribute('aria-label')).toBe('自定义进度')
    expect(bar.getAttribute('aria-valuenow')).toBe('50')
  })

  it('dispose 后不再增强新增节点，且 observer 断开', async () => {
    handle.dispose()
    host.innerHTML =
      '<div class="progress-bar"><div class="progress-fill" style="width: 42%"></div><span>42%</span></div>'
    await nextTick()
    expect(host.querySelector('.progress-bar')!.getAttribute('role')).toBeNull()
  })

  it('main.ts 在挂载前装配进度条增强器', () => {
    const mainSource = fs.readFileSync(path.resolve(__dirname, '../main.ts'), 'utf-8')
    expect(mainSource).toContain("import { installProgressA11y } from './utils/progressA11y'")
    expect(mainSource).toContain('installProgressA11y()')
    expect(mainSource.indexOf('installProgressA11y()')).toBeLessThan(mainSource.indexOf("app.mount('#o2-app-root')"))
  })
})
