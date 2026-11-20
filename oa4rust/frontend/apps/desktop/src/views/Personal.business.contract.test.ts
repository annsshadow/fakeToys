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
      expect(template, `缺少业务区块 data-testid: ${id}`).toContain(`data-testid="${id}"`)
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
    expect(template).toMatch(/<ul[^>]*class="emp-list"/)
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
      expect(template, `端点调试 handler 不应出现在模板: ${handler}`).not.toContain(handler)
    }
  })
})
