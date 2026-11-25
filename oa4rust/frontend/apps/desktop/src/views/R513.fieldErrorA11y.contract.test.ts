// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R513 BBSForum 版块保存失败错误态无障碍语义契约（WCAG 3.3.1 / 4.1.2）：
 * sectionError 错误文本必须带 data-oa-field-error 纳入全局 formErrorA11y 增强器，
 * 由增强器补 role=alert + 稳定 id + 版块名称输入框 aria-invalid/aria-describedby，错误解除复位。
 * 真父版必 RED（核心断言失败、4 守卫通过）、修复版必 GREEN。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'BBSForum.vue'), 'utf8')

/** 取 sectionError 错误 div 的开标签 */
function errorDiv(src: string): string {
  const at = src.indexOf('v-if="sectionError"')
  if (at < 0) return ''
  const start = src.lastIndexOf('<div', at)
  const end = src.indexOf('>', at)
  return src.slice(start, end + 1)
}

describe('R513 BBSForum 版块错误态无障碍语义契约', () => {
  const err = errorDiv(source)

  it('版块错误 div 带 data-oa-field-error 纳入全局增强器（核心）', () => {
    expect(err, '必须找到 sectionError 错误 div').not.toBe('')
    expect(err).toMatch(/data-oa-field-error/)
  })

  it('业务零改守卫：错误 div 的 v-if/class/插值文本保留', () => {
    expect(err).toContain('v-if="sectionError"')
    expect(err).toContain('class="error-msg"')
    expect(source).toContain('{{ sectionError }}')
  })

  it('业务零改守卫：saveSection 的 catch 赋值与 submit 绑定保留', () => {
    expect(source).toContain('@submit.prevent="saveSection"')
    expect(source).toContain('sectionError.value = e instanceof Error ? e.message')
  })

  it('业务零改守卫：版块名称 input id/label for/data-testid 保留', () => {
    expect(source).toContain('id="bbs-section-name"')
    expect(source).toContain('for="bbs-section-name"')
    expect(source).toContain('data-testid="bbs-section-name"')
    expect(source).toContain('v-model="sectionName"')
  })

  it('业务零改守卫：modal 结构/关闭按钮/保存按钮保留', () => {
    expect(source).toContain('class="modal-overlay" @click.self="closeSectionModal"')
    expect(source).toContain('class="modal glass-card" role="dialog"')
    expect(source).toContain('aria-label="关闭版块编辑"')
    expect(source).toContain('data-testid="bbs-section-save"')
  })
})
