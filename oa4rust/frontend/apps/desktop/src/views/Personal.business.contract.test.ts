// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

/** A 类页面业务化契约：个人中心以资料/密码/签名/授权等业务区块组织。 */
describe('Personal 业务视图契约', () => {
  const source = readFileSync(fileURLToPath(new URL('./Personal.vue', import.meta.url)), 'utf8')
  const template = source.slice(source.indexOf('<template>'), source.indexOf('</template>'))

  it('默认视图提供我的资料/密码/签名/安全/授权五个业务区块', () => {
    expect(template).toContain('个人中心')
    expect(template).toContain('我的资料')
    expect(template).toContain('修改密码')
    expect(template).toContain('个人签名')
    expect(template).toContain('账户安全与绑定')
    expect(template).toContain('授权委托')
    for (const id of [
      'personal-profile',
      'personal-password',
      'personal-signature',
      'personal-security',
      'personal-empower',
    ]) {
      expect(template, '缺少业务区块 data-testid: ' + id).toContain('data-testid="' + id + '"')
    }
  })

  it('我的资料区有头像上传与信息列表控件', () => {
    expect(template).toContain('type="file"')
    expect(template).toContain('data-testid="personal-avatar"')
    expect(template).toMatch(/<ul[^>]*class="profile-info"/)
    expect(template).toContain('info-row')
  })

  it('修改密码区是带提交按钮的表单', () => {
    const pwd = template.slice(template.indexOf('personal-password'))
    expect(pwd).toContain('<form')
    expect(pwd).toMatch(/<button[^>]*type="submit"/)
    expect(pwd).toContain('type="password"')
  })

  it('授权委托区有范围切换 tab 与记录列表', () => {
    expect(template).toContain('我发出的')
    expect(template).toContain('授权给我')
    expect(template).toMatch(/role="tablist"/)
    expect(template).toMatch(/<ul[^>]*class="[^"]*\bemp-list\b/)
  })

  it('默认视图不渲染 /api/* 端点调试按钮，副标题不暴露 API 路径', () => {
    expect(template).not.toContain('/api/')
    for (const handler of [
      'uploadSignatureImage',
      'loadSignatureManagers',
      'loadAuthMeta',
      'loadOauthConfig',
      'loadAuthOAuthServer',
      'loadAuthShortDiag',
      'loadMailMeta',
      'loadAuthScopes',
      'loadAuthDetails',
      'loadPersonalExtras',
      'loadPersonalMore',
      'loadPersonalRegistEmpower',
      'loadPersonalTwin',
      'loadPersonalTwin2',
      'acctSec',
      'empManage',
      'personMore',
      'personExtra',
    ]) {
      expect(template, '端点调试 handler 不应出现在模板: ' + handler).not.toContain(handler)
    }
  })
})

/** R328 D1/D2 修复契约：从源码提取真实代码块后 new Function 真执行业务语义。 */
describe('Personal R328 D1/D2 修复契约', () => {
  const source = readFileSync(fileURLToPath(new URL('./Personal.vue', import.meta.url)), 'utf8')

  function stripTs328(code: string): string {
    return code
      .replace(/\s+as\s+any\b/g, '')
      .replace(/\s+as\s+Array<\{[^}]*\}>/g, '')
      .replace(/\(data:\s*any\)/g, '(data)')
  }

  function extractOnSuccessBody(block: string): string {
    const k = block.indexOf('onSuccess:')
    expect(k, '未找到 onSuccess').toBeGreaterThan(-1)
    const open = block.indexOf('{', k)
    expect(open).toBeGreaterThan(k)
    let depth = 0
    for (let j = open; j < block.length; j++) {
      if (block[j] === '{') depth++
      else if (block[j] === '}') {
        depth--
        if (depth === 0) return block.slice(open, j + 1)
      }
    }
    throw new Error('onSuccess 花括号未闭合')
  }

  function execSuccess(body: string, env: Record<string, unknown>, resp: unknown): void {
    const names = Object.keys(env)
    const fn = new Function(...names, 'resp', 'return (function(resp)' + body + ')(resp);') as
      (...args: unknown[]) => void
    fn(...names.map((n) => env[n]), resp)
  }

  function mutationBlocks(): { mut: string; sav: string } {
    const mStart = source.indexOf('const pwdMutation = useMutation({')
    const sStart = source.indexOf('function savePassword()')
    const sEnd = source.indexOf('function handleAvatarUpload')
    expect(mStart).toBeGreaterThan(-1)
    expect(sStart).toBeGreaterThan(mStart)
    expect(sEnd).toBeGreaterThan(sStart)
    return { mut: source.slice(mStart, sStart), sav: source.slice(sStart, sEnd) }
  }

  it('D1 源码两处 onSuccess 均检查 type:error 业务错误信封', () => {
    const { mut, sav } = mutationBlocks()
    const re = /type\s*===\s*['"]error['"]/g
    expect((mut.match(re) ?? []).length, 'mutation 级 onSuccess 缺少 error 检查').toBeGreaterThan(0)
    expect((sav.match(re) ?? []).length, 'mutate options onSuccess 缺少 error 检查').toBeGreaterThan(0)
  })

  it('D1 错误旧密码信封不弹成功提示、显示后端 message', () => {
    const { mut, sav } = mutationBlocks()
    const calls = { success: [] as string[], error: [] as string[] }
    const form = { value: { oldPassword: 'old', newPassword: 'new', confirmPassword: 'new' } }
    const err = { value: '' }
    const env = {
      pwdForm: form,
      pwdError: err,
      pwdSaving: { value: false },
      toast: {
        success: (m: string) => { calls.success.push(m) },
        error: (m: string) => { calls.error.push(m) },
      },
    }
    const resp = { type: 'error', message: 'old password mismatch' }
    execSuccess(stripTs328(extractOnSuccessBody(mut)), env, resp)
    execSuccess(stripTs328(extractOnSuccessBody(sav)), env, resp)
    expect(calls.success, '错误信封绝不能弹密码修改成功').toEqual([])
    expect(err.value).toBe('old password mismatch')
    expect(calls.error.join('|')).toContain('old password mismatch')
    expect(form.value.oldPassword, '错误信封不应清空表单').toBe('old')
  })

  it('D1 正确改密成功路径仍弹成功提示并清空表单', () => {
    const { mut, sav } = mutationBlocks()
    const calls = { success: [] as string[], error: [] as string[] }
    const form = { value: { oldPassword: 'old', newPassword: 'new', confirmPassword: 'new' } }
    const err = { value: '' }
    const env = {
      pwdForm: form,
      pwdError: err,
      pwdSaving: { value: false },
      toast: {
        success: (m: string) => { calls.success.push(m) },
        error: (m: string) => { calls.error.push(m) },
      },
    }
    execSuccess(stripTs328(extractOnSuccessBody(mut)), env, { type: 'success', message: '' })
    execSuccess(stripTs328(extractOnSuccessBody(sav)), env, { type: 'success', message: '' })
    expect(calls.success).toContain('密码修改成功')
    expect(calls.error).toEqual([])
    expect(form.value.oldPassword).toBe('')
    expect(err.value).toBe('')
  })

  function runSigQuery(resp: unknown): Promise<string> {
    const anchor = 'queryFn: async () => {'
    const start = source.indexOf(anchor)
    expect(start, '未找到签名 queryFn').toBeGreaterThan(-1)
    const open = source.indexOf('{', start + 'queryFn: async ()'.length)
    expect(open).toBeGreaterThan(start)
    let depth = 0
    let close = open
    for (let j = open; j < source.length; j++) {
      if (source[j] === '{') depth++
      else if (source[j] === '}') {
        depth--
        if (depth === 0) { close = j; break }
      }
    }
    const body = stripTs328(source.slice(open, close + 1))
    const fn = new Function('api', 'return (async () => ' + body + ')();') as
      (a: unknown) => Promise<string>
    return fn({ get: async () => resp })
  }

  it('D2 双层信封 signatures[0].value 真解包', async () => {
    const out = await runSigQuery({
      type: 'success',
      data: { signatures: [{ id: 'x', name: 'SIGNATURE_x', value: 'R328_SAVED_VALUE' }] },
    })
    expect(out).toBe('R328_SAVED_VALUE')
  })

  it('D2 空签名列表回空串且不抛错', async () => {
    expect(await runSigQuery({ type: 'success', data: { signatures: [] } })).toBe('')
  })

  it('D2 签名保存与列表端点写读链未回退', () => {
    expect(source).toContain('/api/person/signature/list')
    expect(source).toContain('/api/person/signature/save')
  })

  it('D2 签名数据返回后响应式同步到 textarea（禁止 setup 期一次性快照）', () => {
    expect(source, '签名 query 结果必须经 watch 响应式同步到 textarea，不能只在 setup 期取静态快照').toMatch(/watch\(\s*sigData\s*,/)
    expect(source, '不得保留 setup 期一次性赋值 signature.value = sigData.value').not.toMatch(/^signature\.value\s*=\s*sigData\.value/m)
  })
})
