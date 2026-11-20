// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { validateNewPassword } from './password'

describe('validateNewPassword 新密码本地预校验（R291）', () => {
  it('不足 10 位的新密码被拦截，不必等后端报错', () => {
    expect(validateNewPassword('oldPass12', '123456789')).toBe('新密码至少 10 位')
  })

  it('刚好 10 位的新密码通过校验', () => {
    expect(validateNewPassword('oldPass12', 'newPass123')).toBeNull()
  })

  it('新密码与原密码相同被拦截（含去空白后相同）', () => {
    expect(validateNewPassword('samePass12', 'samePass12')).toBe('新密码不能与原密码相同')
    expect(validateNewPassword('samePass12', '  samePass12  ')).toBe('新密码不能与原密码相同')
  })

  it('原密码或新密码为空白时提示填写完整', () => {
    expect(validateNewPassword('  ', 'newPass123')).toBe('请填写完整密码')
    expect(validateNewPassword('oldPass12', '   ')).toBe('请填写完整密码')
  })

  it('可达链：mine.vue 提交前调用预校验并使用其文案，避免弱密码发到后端', () => {
    const src = readFileSync(resolve(import.meta.dirname, '../pages/mine/mine.vue'), 'utf8')
    expect(src).toContain('新密码（至少 10 位）')
    expect(src).not.toContain('6-64')
    expect(src).toContain("import { validateNewPassword } from '@/utils/password'")
    expect(src).toContain('const pwdError = validateNewPassword(pwdForm.value.oldPassword, pwdForm.value.newPassword)')
    expect(src).toContain("uni.showToast({ title: pwdError, icon: 'none' })")
    const callIndex = src.indexOf('const pwdError = validateNewPassword(')
    const requestIndex = src.indexOf('await authApi.changePassword(')
    expect(callIndex).toBeGreaterThan(-1)
    expect(requestIndex).toBeGreaterThan(callIndex)
  })
})
