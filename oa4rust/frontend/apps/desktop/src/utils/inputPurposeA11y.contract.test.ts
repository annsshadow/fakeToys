// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R194 输入目的标识契约测试（WCAG 2.1 AA / 1.3.5 Identify Input Purpose）。
 *
 * 登录与「修改密码」表单收集用户名和密码，属于 WCAG 1.3.5 Appendix A 列出的
 * 输入目的（username / current-password / new-password），必须用 HTML autocomplete
 * token 标明。缺失时：
 *   - 密码管理器/浏览器无法自动填充，登录与改密体验显著退化；
 *   - 认知/运动障碍用户失去自动填充这一减轻操作负担的关键机制；
 *   - 辅助技术无法从 token 推断字段用途。
 *
 * 全仓扫描确认此前无任何 HTML autocomplete 属性（命中的 autocompletion 均为
 * CodeMirror 代码补完，与输入目的无关）。本测试读 .vue 模板源码，定位各
 * v-model 对应的 <input> 起始标签并断言 autocomplete token，防止回归。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const frontendRoot = path.resolve(here, '../../../..')

function vue(rel: string): string {
  return readFileSync(path.join(frontendRoot, rel), 'utf8')
}

/** 取绑定指定 v-model 的 <input> 起始标签全文。 */
function inputTag(source: string, vmodel: string): string {
  const at = source.indexOf(`v-model="${vmodel}"`)
  expect(at, `未找到 v-model="${vmodel}" 的输入框`).toBeGreaterThan(-1)
  const start = source.lastIndexOf('<input', at)
  const end = source.indexOf('>', at)
  expect(start, `v-model="${vmodel}" 不在 <input> 标签内`).toBeGreaterThan(-1)
  return source.slice(start, end + 1)
}

describe('登录与改密表单的输入目的标识（WCAG 2.1 AA / 1.3.5）', () => {
  const login = vue('packages/ui/src/components/LoginScreen.vue')
  const personal = vue('apps/desktop/src/views/Personal.vue')

  it('登录用户名输入声明 autocomplete="username"', () => {
    expect(inputTag(login, 'username')).toMatch(/autocomplete="username"/)
  })

  it('登录密码输入声明 autocomplete="current-password"', () => {
    expect(inputTag(login, 'password')).toMatch(/autocomplete="current-password"/)
  })

  it('改密「当前密码」输入声明 autocomplete="current-password"', () => {
    expect(inputTag(personal, 'pwdForm.oldPassword')).toMatch(/autocomplete="current-password"/)
  })

  it('改密「新密码」输入声明 autocomplete="new-password"', () => {
    expect(inputTag(personal, 'pwdForm.newPassword')).toMatch(/autocomplete="new-password"/)
  })

  it('改密「确认密码」输入声明 autocomplete="new-password"', () => {
    expect(inputTag(personal, 'pwdForm.confirmPassword')).toMatch(/autocomplete="new-password"/)
  })
})
