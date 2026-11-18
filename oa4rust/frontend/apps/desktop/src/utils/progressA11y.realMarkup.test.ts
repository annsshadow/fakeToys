// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment jsdom

import fs from 'node:fs'
import path from 'node:path'
import { describe, expect, it } from 'vitest'
import { installProgressA11y } from './progressA11y'

// 把真实视图中的进度条片段喂给增强器，确认用户实际看到的 DOM 获得语义（WCAG 4.1.2）。
describe('progressA11y：真实视图进度条片段', () => {
  const desktop = path.resolve(__dirname, '..')
  const fileManager = fs.readFileSync(path.join(desktop, 'views/FileManager.vue'), 'utf-8')
  const processDesigner = fs.readFileSync(path.join(desktop, 'views/ProcessDesigner.vue'), 'utf-8')

  it('FileManager.vue 确实存在无 ARIA 的纯视觉上传进度条', () => {
    expect(fileManager).toContain('<div v-if="uploadProgress > 0" class="progress-bar">')
    expect(fileManager).toContain('<div class="progress-fill" :style="{ width: uploadProgress + \'%\' }"></div>')
    expect(fileManager).not.toMatch(/class="progress-bar"[^>]*role="progressbar"/)
  })

  it('ProcessDesigner.vue 执行进度条增强后 role/数值/标签齐全', async () => {
    const snippet = [
      '<div class="exec-bar"><div class="exec-bar-fill" style="width: 80%"></div></div>',
      '<div class="tr-progress"><div class="tr-bar" style="width: 33%"></div><span>1/3</span></div>',
    ].join('')
    expect(processDesigner).toContain('class="exec-bar"')
    expect(processDesigner).toContain('class="tr-progress"')
    const host = document.createElement('div')
    document.body.appendChild(host)
    const handle = installProgressA11y(host)
    host.innerHTML = snippet
    await new Promise<void>((resolve) => setTimeout(resolve, 0))
    const exec = host.querySelector('.exec-bar')!
    const tr = host.querySelector('.tr-progress')!
    expect(exec.getAttribute('role')).toBe('progressbar')
    expect(exec.getAttribute('aria-valuenow')).toBe('80')
    expect(exec.getAttribute('aria-label')).toBe('执行进度')
    expect(tr.getAttribute('role')).toBe('progressbar')
    expect(tr.getAttribute('aria-valuenow')).toBe('33')
    handle.dispose()
    document.body.innerHTML = ''
  })
})
