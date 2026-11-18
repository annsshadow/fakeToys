// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R208 登录验证码刷新控件的键盘可达性契约（WCAG 2.1.1 Keyboard / 4.1.2 Name,Role,Value / 1.1.1）。
 *
 * 缺陷证据：LoginScreen.vue 的验证码图片是 `<img @click="refreshCaptcha">`：
 *   ① <img> 不可 Tab 聚焦、不响应 Enter/Space，键盘用户根本无法刷新验证码；
 *   ② 无 role / 可访问名，读屏用户不知道图片可点击、更不知道点击会刷新；
 *   ③ 验证码看不清时登录流程没有其它刷新入口——这是登录的硬阻断。
 * 契约要求刷新控件为原生 <button type="button"> 且带非空 aria-label：
 *   - 原生 button 天然可聚焦、可用 Enter/Space 激活、角色为 button；
 *   - 显式 type="button" 防止位于 <form> 内的刷新按钮误触登录提交；
 *   - aria-label 给出动作型可访问名（图片 alt 仅作非文本替代）。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { describe, expect, it } from 'vitest'

const here = path.dirname(new URL(import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1'))
const frontendRoot = path.resolve(here, '../../../..')
const source = readFileSync(path.join(frontendRoot, 'packages/ui/src/components/LoginScreen.vue'), 'utf8')

/** 提取模板中所有包含给定子串的起始标签（含多行标签）。 */
function tagsContaining(needle: string): string[] {
  const out: string[] = []
  for (const match of source.matchAll(/<[a-z][a-z0-9-]*\b[^>]*>/gis)) {
    if (match[0].includes(needle)) out.push(match[0])
  }
  return out
}

function refreshControls(): string[] {
  return tagsContaining('@click="refreshCaptcha"')
}

describe('登录验证码刷新控件键盘可达性（WCAG 2.1.1 / 4.1.2 / 1.1.1）', () => {
  it('存在验证码刷新控件（防止契约因模板消失而空跑）', () => {
    expect(refreshControls().length).toBeGreaterThanOrEqual(1)
  })

  it('刷新控件是原生 button（可聚焦、可由 Enter/Space 激活、具备 button 角色）', () => {
    const controls = refreshControls()
    expect(controls.length).toBeGreaterThanOrEqual(1)
    for (const tag of controls) {
      expect(tag, `刷新控件不是原生 button: ${tag}`).toMatch(/^<button\b/i)
    }
  })

  it('刷新控件显式 type="button"，点击不会误触发登录表单提交', () => {
    for (const tag of refreshControls()) {
      expect(tag, `刷新控件缺少 type="button": ${tag}`).toMatch(/\btype="button"/)
    }
  })

  it('刷新控件具有非空可访问名（aria-label）', () => {
    for (const tag of refreshControls()) {
      expect(tag, `刷新控件缺少可访问名: ${tag}`).toMatch(/\baria-label="[^"]+"/)
    }
  })

  it('不存在鼠标专属的可点击图片（img 不带 @click）', () => {
    const imgs = source.match(/<img\b[^>]*>/gis) ?? []
    expect(imgs.length, '验证码图片不应消失').toBeGreaterThanOrEqual(1)
    for (const img of imgs) {
      expect(img, `存在鼠标专属图片: ${img}`).not.toMatch(/@click/)
    }
  })

  it('验证码图片保留文本替代（alt 属性，1.1.1）', () => {
    const imgs = source.match(/<img\b[^>]*>/gis) ?? []
    expect(imgs.length).toBeGreaterThanOrEqual(1)
    for (const img of imgs) {
      expect(img, `图片缺少 alt: ${img}`).toMatch(/\balt=/)
    }
  })
})
