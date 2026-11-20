// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * L2 视觉节奏契约：工作台作为桌面端首屏，卡片间距/面板留白必须消费统一令牌，
 * 且不得用硬编码色值回退色（回退色会在 dark/light 主题切换时留下错误颜色）。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, 'Dashboard.vue'), 'utf8')

describe('工作台视觉节奏标准化（L2）', () => {
  it('主区块纵向间距使用 --space-section-gap（24px）', () => {
    expect(src).toMatch(/\.dashboard\s*\{[^}]*gap:\s*var\(--space-section-gap\)/)
  })

  it('欢迎卡与统计卡内边距使用间距令牌', () => {
    expect(src).toMatch(/\.welcome-bar\s*\{[^}]*padding:\s*var\(--space-[a-z0-9]+\)/)
    expect(src).toMatch(/\.stat-card\s*\{[^}]*padding:\s*var\(--space-[a-z0-9]+\)/)
  })

  it('主网格间距使用令牌且不再出现裸 gap: 16px/12px', () => {
    expect(src).toMatch(/\.main-grid\s*\{[^}]*gap:\s*var\(--space-[a-z0-9]+\)/)
    expect(src).not.toMatch(/gap:\s*(?:16|12)px/)
  })

  it('错误态颜色不写十六进制回退值（统一由 dark.css 提供）', () => {
    expect(src).not.toMatch(/var\(--color-error\s*,/)
  })
})
