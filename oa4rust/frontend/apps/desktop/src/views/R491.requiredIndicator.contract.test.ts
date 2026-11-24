// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R491 ThreeMemberApp 姓名必填语义契约（WCAG 3.3.2）：
 * 姓名 label 必须带可见星号 span.required aria-hidden=true，姓名 input 必须带 required；
 * 真父版必 RED（核心断言失败、业务守卫通过）、修复版必 GREEN。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'ThreeMemberApp.vue'), 'utf8')

function nameGroup(src: string): string {
  const at = src.indexOf('<label>姓名')
  if (at < 0) return ''
  const start = src.lastIndexOf('<div', at)
  const end = src.indexOf('/>', at) + 2
  return src.slice(start, end)
}

describe('R491 ThreeMemberApp 姓名必填语义契约', () => {
  const name = nameGroup(source)

  it('姓名 label 带可见星号 span.required aria-hidden=true 且 input 带 required（核心）', () => {
    expect(name, '必须找到姓名表单组').not.toBe('')
    expect(name).toMatch(/<span class="required" aria-hidden="true">\*<\/span>/)
    expect(name).toMatch(/<input[^>]*\brequired\b/)
  })

  it('业务零改守卫：保存按钮 disabled 校验保留', () => {
    expect(source).toContain(':disabled="!form.name?.trim()||saving"')
    expect(source).toContain('@click="saveItem"')
  })

  it('业务零改守卫：其它四字段 label 与 v-model 保留且不加 required', () => {
    expect(source).toContain('<label>手机</label><input v-model="form.mobile"')
    expect(source).toContain('<label>邮箱</label><input v-model="form.email"')
    expect(source).toContain('<label>所属单位</label><input v-model="form.unitId"')
    expect(source).toContain('<label>状态</label><select v-model="form.status"')
    expect(source).not.toMatch(/<label>(手机|邮箱|所属单位|状态)<span class="required"/)
  })

  it('业务零改守卫：姓名 input 的 v-model/placeholder/class 与 data-testid 保留', () => {
    expect(name).toContain('v-model="form.name"')
    expect(name).toContain('placeholder="姓名"')
    expect(name).toContain('class="form-input"')
    expect(source).toContain('data-testid="three-member-page"')
    expect(source).toContain('data-testid="three-unit-input"')
    expect(source).toContain('data-testid="three-status-select"')
  })

  it('可见星号颜色样式 .required 存在', () => {
    expect(source).toMatch(/\.required\{[^}]*color:\s*var\(--color-error\)/)
  })
})
