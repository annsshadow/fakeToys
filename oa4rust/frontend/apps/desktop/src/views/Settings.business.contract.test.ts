// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

/** A 类页面业务化契约：系统设置以分组表单呈现，而非端点触发面板。 */
describe('Settings 业务视图契约', () => {
  const source = readFileSync(fileURLToPath(new URL('./Settings.vue', import.meta.url)), 'utf8')
  const template = source.slice(source.indexOf('<template>'), source.indexOf('</template>'))

  it('默认视图提供设置分组导航与基础/安全/通知/关于四个面板', () => {
    expect(template).toContain('系统设置')
    expect(template).toContain('设置分组')
    expect(template).toContain('基础配置')
    expect(template).toContain('安全配置')
    expect(template).toContain('通知配置')
    expect(template).toContain('关于 OA4Rust')
    for (const id of ['settings-sidebar', 'settings-main']) {
      expect(template, `缺少业务区块 data-testid: ${id}`).toContain(`data-testid="${id}"`)
    }
  })

  it('分组导航由 sections 渲染为可切换 tab', () => {
    expect(template).toMatch(/v-for="s in sections"/)
    expect(template).toMatch(/role="tablist"/)
    expect(template).toContain('activeSection===s.id')
  })

  it('基础与安全面板使用真实表单控件（select/number/提交按钮）', () => {
    expect(template).toContain('<form')
    expect(template).toContain('<select')
    expect(template).toContain('type="number"')
    expect(template).toMatch(/<button[^>]*type="submit"/)
  })

  it('通知面板使用复选框开关，且各面板业务 class 齐备', () => {
    expect(template).toContain('type="checkbox"')
    for (const c of ['basic-panel', 'security-panel', 'notify-panel', 'about-panel']) {
      expect(template, `缺少业务面板 class: ${c}`).toContain(c)
    }
  })

  it('默认视图不渲染 /api/* 端点调试按钮，副标题不暴露 API 路径', () => {
    expect(template).not.toContain('/api/')
    expect(template).not.toMatch(/@click="[^"]*loadCfg/)
  })
})
