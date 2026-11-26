// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R60 role=button 键盘可达性契约（test-first RED）
 * 真实读 laiyipao/miniapp/src/pages 下所有 .vue 与 common.css，禁止内嵌实现。
 * 缺口：48 处 role=button 的 text/view 无 tabindex/@keydown，cursor=auto，
 * a11y 树暴露为 button 但键盘 Tab 不可达（WCAG 2.1.1）。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync, readdirSync, statSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join, relative } from 'node:path'

const dir = dirname(fileURLToPath(import.meta.url))
const srcRoot = join(dir, '..')
const NL = String.fromCharCode(10)

function walkVue(root: string): string[] {
  const out: string[] = []
  for (const name of readdirSync(root)) {
    const p = join(root, name)
    if (statSync(p).isDirectory()) out.push(...walkVue(p))
    else if (name.endsWith('.vue')) out.push(p)
  }
  return out
}

const vueFiles = walkVue(join(srcRoot, 'pages'))
function roleButtonLines(): { file: string; line: number; text: string }[] {
  const out: { file: string; line: number; text: string }[] = []
  for (const f of vueFiles) {
    readFileSync(f, 'utf8').split(NL).forEach((text, i) => {
      if (text.includes('role="button"') && /<(text|view)\b/.test(text))
        out.push({ file: relative(srcRoot, f), line: i + 1, text })
    })
  }
  return out
}

describe('R60 role=button 键盘可达性契约', () => {
  it('每个 role=button 元素带 tabindex="0"（WCAG 2.1.1 键盘可达）', () => {
    const rows = roleButtonLines()
    expect(rows.length, '应扫到 role=button 元素').toBeGreaterThan(0)
    const bad = rows.filter((r) => r.text.indexOf('tabindex="0"') < 0)
      .map((r) => r.file + ':' + r.line)
    expect(bad, '以下 role=button 缺 tabindex="0"，键盘 Tab 不可达: ' + bad.join(', ')).toEqual([])
  })

  it('每个 role=button 元素带 @keydown 键盘激活（Enter/Space）', () => {
    const rows = roleButtonLines()
    const bad = rows.filter((r) => r.text.indexOf('@keydown') < 0)
      .map((r) => r.file + ':' + r.line)
    expect(bad, '以下 role=button 缺 @keydown 键盘激活: ' + bad.join(', ')).toEqual([])
  })

  it('common.css 含 [role="button"] 全局 cursor:pointer（补齐鼠标可点暗示）', () => {
    const common = readFileSync(join(dir, 'common.css'), 'utf8')
    const idx = common.indexOf('[role="button"]')
    expect(idx, 'common.css 需含 [role="button"] 全局规则').toBeGreaterThanOrEqual(0)
    const end = common.indexOf('}', idx)
    const rule = common.slice(idx, end + 1)
    expect(rule, '[role="button"] 规则需含 cursor: pointer').toContain('cursor: pointer')
  })

  it('focus-visible 覆盖 [tabindex] 且未重复声明（不变量）', () => {
    const common = readFileSync(join(dir, 'common.css'), 'utf8')
    expect(common, 'focus-visible 须覆盖 [tabindex]').toContain('[tabindex]):focus-visible')
    expect(common, 'focus-visible outline 保留').toContain('outline: 3rpx solid var(--accent)')
    const decls = common.split('[tabindex]):focus-visible').length - 1
    expect(decls, 'focus-visible 规则只声明一次，勿重复造').toBe(1)
  })

  it('已收口 CTA 言行一致性锚点未被改动（不变量）', () => {
    const battlesVue = readFileSync(join(srcRoot, 'pages/battles/battles.vue'), 'utf8')
    const battleVue = readFileSync(join(srcRoot, 'pages/battle/battle.vue'), 'utf8')
    const indexVue = readFileSync(join(srcRoot, 'pages/index/index.vue'), 'utf8')
    expect(battlesVue, 'R43 r20 CTA 须保留 @click=goStage').toContain('class="r20-ai-cta" data-r20-cta role="button" @click="goStage()"')
    expect(battleVue, 'R44 r29 CTA 须保留 @click.stop').toContain('class="r29-result-timeline-cta" @click.stop="openReactionTimeline"')
    expect(indexVue, 'R43 ai-cta 须保留真实导航').toContain('@click="go(aiInsight.hint)"')
    expect(indexVue, 'R48 去战斗 CTA 须保留 goStage').toContain('@click="goStage"')
  })

  it('令牌三锚点不变且不新增 --r60-（不变量）', () => {
    const tokens = readFileSync(join(srcRoot, 'styles/tokens.css'), 'utf8')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toContain('--r60-')
  })
})