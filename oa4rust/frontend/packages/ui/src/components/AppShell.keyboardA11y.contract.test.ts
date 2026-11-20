// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R206 交互控件键盘可达性契约（WCAG 2.1.1 Keyboard / 4.1.2 Name,Role,Value）。
 *
 * 缺陷证据：AppShell.vue 中 logo、侧栏导航项、用户菜单、移动端 tab 项均为裸
 * <div @click>，既不可 Tab 聚焦，也不响应 Enter/Space，键盘用户与读屏用户
 * 无法激活这些入口；collapse-btn <button> 的内容仅为装饰字符 "☰"，无可访问名。
 *
 * 契约：模板中任何带 @click 的交互元素必须同时具备
 *   ① 键盘可聚焦（原生 <button>/<a> 或 tabindex="0"）；
 *   ② 可访问角色（原生语义标签或显式 role）；
 *   ③ 键盘激活（原生 <button> 或 @keydown.enter + @keydown.space.prevent）。
 * 同时禁止出现纯装饰 <button>（无内容文本、无 aria-label/title）。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'AppShell.vue'), 'utf8')

/** 提取模板中每一个带 @click 的起始标签（含多行标签）。 */
function interactiveTags(): string[] {
  const tags: string[] = []
  for (const match of source.matchAll(/<([a-z][a-z0-9-]*)\b[^>]*@click\b[^>]*>/gis)) {
    tags.push(match[0])
  }
  return tags
}

/** 去掉 class/:class/v-if/v-for 等属性，仅保留与语义相关的属性。 */
function semanticAttrs(tag: string): string {
  return tag.replace(/\s*:?class="[^"]*"/g, '').replace(/\sv-(?:if|for|show)="[^"]*"/g, '')
}

describe('AppShell 交互控件键盘可达性契约', () => {
  it('模板中存在预期的交互控件（防止契约因模板消失而空跑）', () => {
    const tags = interactiveTags()
    expect(tags.length).toBeGreaterThanOrEqual(4)
    expect(tags.some((t) => /class="logo"/.test(t))).toBe(true)
    expect(tags.some((t) => /class="nav-item"/.test(t))).toBe(true)
    expect(tags.some((t) => /class="user-menu"/.test(t))).toBe(true)
    expect(tags.some((t) => /class="tab-item"/.test(t))).toBe(true)
  })

  it('每个 @click 控件都必须键盘可聚焦', () => {
    for (const tag of interactiveTags()) {
      const focusable = /^<(button|a)\b/i.test(tag) || /\btabindex="0"/.test(tag)
      expect(focusable, `交互控件不可聚焦: ${semanticAttrs(tag)}`).toBe(true)
    }
  })

  it('每个 @click 控件都必须具备可访问角色', () => {
    for (const tag of interactiveTags()) {
      const role = /^<(button|a|input|select|textarea)\b/i.test(tag) || /\brole="(button|menuitem|link)"/.test(tag)
      expect(role, `交互控件缺少可访问角色: ${semanticAttrs(tag)}`).toBe(true)
    }
  })

  it('每个 @click 控件都必须可由键盘激活', () => {
    for (const tag of interactiveTags()) {
      const activatable =
        /^<button\b/i.test(tag) || (/@keydown\.enter/.test(tag) && /@keydown\.space\.prevent/.test(tag))
      expect(activatable, `交互控件缺少键盘激活: ${semanticAttrs(tag)}`).toBe(true)
    }
  })

  it('装饰图标按钮（collapse-btn）必须提供可访问名', () => {
    const collapse = interactiveTags().find((t) => /class="collapse-btn"/.test(t))
    expect(collapse).toBeDefined()
    expect(/aria-label="[^"]+"/.test(collapse!) || /title="[^"]+"/.test(collapse!)).toBe(true)
  })
})
