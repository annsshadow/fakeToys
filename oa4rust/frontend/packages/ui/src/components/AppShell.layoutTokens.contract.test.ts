// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 桌面端 L1 视觉标准化契约：共享布局（AppShell）必须消费设计令牌。
 *
 * 缺陷证据：AppShell.vue 的侧栏宽度/顶栏高度/内容边距/层级/过渡都是裸字面量
 * （240px、64px、56px/20px、100/200、0.6s），而 dark.css + tokens.css 已定义
 * --sidebar-width / --sidebar-collapsed / --topbar-height / --space-5 /
 * --z-dropdown / --z-sticky / --duration-slow。裸值与令牌并存会让双端视觉标准
 * 分裂，改一处令牌时布局不跟随，并造成 UA 样式表重复。
 *
 * 契约：布局尺寸/层级/过渡必须引用令牌；保留导航与键盘可达语义（由其它契约守护）。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'AppShell.vue'), 'utf8')

describe('AppShell 布局令牌化（L1 视觉标准）', () => {
  it('侧栏宽度/折叠宽度使用 --sidebar-width 与 --sidebar-collapsed 令牌', () => {
    expect(source).toMatch(/var\(--sidebar-width\)/g)
    expect(source).toMatch(/var\(--sidebar-collapsed\)/g)
    expect(source).not.toMatch(/(?:width|margin-left):\s*(?:240px|64px)/)
  })

  it('顶栏高度使用 --topbar-height 令牌，不写裸 56px', () => {
    expect(source).not.toMatch(/height:\s*56px/)
  })

  it('内容区内边距使用间距令牌，且不写裸 20px/12px', () => {
    expect(source).toMatch(/padding:\s*var\(--space-[a-z0-9]+\)/)
    expect(source).not.toMatch(/padding:\s*(?:20px|12px)\s*;/)
  })

  it('层级使用 --z-dropdown / --z-sticky 令牌，不写裸 100/200', () => {
    expect(source).toMatch(/z-index:\s*var\(--z-dropdown\)/)
    expect(source).toMatch(/z-index:\s*var\(--z-sticky\)/)
    expect(source).not.toMatch(/z-index:\s*\d/)
  })

  it('主按钮/卡片动效不再出现裸 0.6s 过渡（统一走 --duration-slow/--transition-slow）', () => {
    expect(source).not.toMatch(/\b0\.6s\b/)
  })

  it('无障碍语义未被令牌化改动破坏', () => {
    const navLabels = [...source.matchAll(/<nav[^>]*aria-label="([^"]+)"/g)].map((m) => m[1])
    expect(navLabels.length).toBeGreaterThanOrEqual(2)
    expect(source.match(/:aria-current="isActive\(item\.path\)\s*\?\s*'page'\s*:\s*undefined"/g)?.length).toBe(2)
    expect(source).not.toMatch(/aria-current="page"/)
    const clickableTags = [...source.matchAll(/<([a-z][a-z0-9-]*)\b[^>]*@click\b[^>]*>/gi)].map((m) => m[0])
    expect(clickableTags.length).toBeGreaterThanOrEqual(4)
    for (const tag of clickableTags) {
      expect(/^<(button|a)\b/i.test(tag) || /\btabindex="0"/.test(tag), tag).toBe(true)
      expect(
        /^<(button|a|input|select|textarea)\b/i.test(tag) || /\brole="(button|menuitem|link)"/.test(tag),
        tag,
      ).toBe(true)
      expect(
        /^<button\b/i.test(tag) || (/@keydown\.enter/.test(tag) && /@keydown\.space\.prevent/.test(tag)),
        tag,
      ).toBe(true)
    }
  })
})
