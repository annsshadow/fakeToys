// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R489 Personal 修改密码错误态无障碍语义契约：
 * .error-msg 错误文本必须带 data-oa-field-error 纳入全局 formErrorA11y 增强器，
 * 由增强器补 role=alert + 稳定 id + 三密码框 aria-invalid/aria-describedby，错误解除复位。
 * 真父版必 RED（核心断言失败、4 守卫通过）、修复版必 GREEN。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'Personal.vue'), 'utf8')

/** 取 pwdError 错误 div 的开标签 */
function errorDiv(src: string): string {
  const at = src.indexOf('v-if="pwdError"')
  if (at < 0) return ''
  const start = src.lastIndexOf('<div', at)
  const end = src.indexOf('>', at)
  return src.slice(start, end + 1)
}

describe('R489 Personal 修改密码错误态无障碍语义契约', () => {
  const err = errorDiv(source)

  it('密码错误 div 带 data-oa-field-error 纳入全局增强器（核心）', () => {
    expect(err, '必须找到 pwdError 错误 div').not.toBe('')
    expect(err).toMatch(/data-oa-field-error/)
  })

  it('业务零改守卫：错误 div 的 v-if/class/插值文本保留', () => {
    expect(err).toContain('v-if="pwdError"')
    expect(err).toContain('class="error-msg"')
    expect(source).toContain('{{ pwdError }}')
  })

  it('业务零改守卫：savePassword 校验文案与 submit 绑定保留', () => {
    expect(source).toContain('@submit.prevent="savePassword"')
    expect(source).toContain("pwdError.value = '请填写完整密码'")
    expect(source).toContain("pwdError.value = '两次密码不一致'")
  })

  it('业务零改守卫：三密码框 autocomplete/v-model/data-testid 保留', () => {
    expect(source).toContain('autocomplete="current-password"')
    expect((source.match(/autocomplete="new-password"/g) || []).length).toBe(2)
    expect(source).toContain('v-model="pwdForm.oldPassword"')
    expect(source).toContain('v-model="pwdForm.newPassword"')
    expect(source).toContain('v-model="pwdForm.confirmPassword"')
    expect(source).toContain('data-testid="personal-password"')
  })

  it('业务零改守卫：保存按钮与 error-msg 样式保留', () => {
    expect(source).toContain('class="save-btn modern-btn primary"')
    expect(source).toContain(':disabled="pwdSaving"')
    expect(source).toContain('.error-msg{color:var(--color-error);font-size:13px;padding:8px 0}')
  })
})