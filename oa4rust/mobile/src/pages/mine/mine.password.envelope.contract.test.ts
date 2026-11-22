// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R350 契约：H5 mine 改密后端 HTTP 200 + 业务错误信封（type:error / success:false）时
 * 必须显示后端 message，不得弹「密码修改成功」、不得清空表单；仅明确成功才进成功分支。
 * R349 审计坐实：mine.vue:78 无条件 toast 成功 + index.ts:40 discardResponse:true 吞掉响应体。
 * 源码级函数体真执行（new Function/AsyncFn），不引入新依赖。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const mineSrc = readFileSync(resolve(import.meta.dirname, './mine.vue'), 'utf8')
const idxSrc = readFileSync(resolve(import.meta.dirname, '../../services/index.ts'), 'utf8')

function bodyOf(source: string, signature: string): string {
  const start = source.indexOf(signature)
  if (start < 0) throw new Error('未找到函数: ' + signature)
  const open = source.indexOf('{', start)
  let depth = 0
  for (let i = open; i < source.length; i++) {
    if (source[i] === '{') depth++
    else if (source[i] === '}') {
      depth--
      if (depth === 0) return source.slice(open + 1, i)
    }
  }
  throw new Error('花括号未配对: ' + signature)
}

const pwdBody = bodyOf(mineSrc, 'async function changePassword()')
  .replace(/: boolean/g, '').replace(/const resp: any/g, 'const resp').replace(/catch \(e: unknown\)/g, 'catch (e)')

const AsyncFn = Object.getPrototypeOf(async function () {}).constructor

interface PwdForm { oldPassword: string; newPassword: string; confirmPassword: string }

function runChange(envelope: unknown) {
  const toasts: Array<{ title: string; icon: string }> = []
  const form: { value: PwdForm } = { value: { oldPassword: 'OLD', newPassword: 'newPass123', confirmPassword: 'newPass123' } }
  const saving: { value: boolean } = { value: false }
  const uniStub = { showToast: (o: { title: string; icon: string }): void => { toasts.push(o) } }
  const authApi = { changePassword: async (): Promise<unknown> => envelope }
  const validate = (): null => null
  const fn = new AsyncFn('uni', 'authApi', 'pwdForm', 'pwdSaving', 'validateNewPassword', pwdBody)
  return { toasts, form, saving, invoke: () => fn(uniStub, authApi, form, saving, validate) }
}

describe('R350 mine 改密业务信封判定契约', () => {
  it('index.ts changePassword 不再 discardResponse，响应体可供页面判定', () => {
    const seg = idxSrc.slice(idxSrc.indexOf('changePassword:'), idxSrc.indexOf('changePassword:') + 260)
    expect(seg).toContain('/api/person/password')
    expect(seg).not.toContain('discardResponse: true')
  })

  it('type:error 信封显示后端 message，不弹成功且不清空表单', async () => {
    const c = runChange({ type: 'error', message: 'old password mismatch', success: false })
    await c.invoke()
    expect(c.toasts.map((t) => t.title)).toContain('old password mismatch')
    expect(c.toasts.map((t) => t.title)).not.toContain('密码修改成功')
    expect(c.form.value.oldPassword).toBe('OLD')
    expect(c.form.value.newPassword).toBe('newPass123')
  })

  it('success:false 无 type 信封同样按错误处理', async () => {
    const c = runChange({ success: false, message: '修改失败：服务端拒绝' })
    await c.invoke()
    expect(c.toasts.map((t) => t.title)).toContain('修改失败：服务端拒绝')
    expect(c.toasts.map((t) => t.title)).not.toContain('密码修改成功')
    expect(c.form.value.oldPassword).toBe('OLD')
  })

  it('明确成功信封弹成功并清空表单', async () => {
    const c = runChange({ type: 'success', success: true, message: 'ok' })
    await c.invoke()
    expect(c.toasts.map((t) => t.title)).toContain('密码修改成功')
    expect(c.form.value).toEqual({ oldPassword: '', newPassword: '', confirmPassword: '' })
  })

  it('可达链保护：预校验/一致性校验/端点仍在原位置', () => {
    expect(mineSrc).toContain('const pwdError = validateNewPassword(')
    expect(mineSrc.indexOf('await authApi.changePassword(')).toBeGreaterThan(mineSrc.indexOf('const pwdError = validateNewPassword('))
    expect(mineSrc).toContain("pwdForm.value.newPassword !== pwdForm.value.confirmPassword")
    expect(idxSrc).toContain('changePassword: (data:')
  })
})