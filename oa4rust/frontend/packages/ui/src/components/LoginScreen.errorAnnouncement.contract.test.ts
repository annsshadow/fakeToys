// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R228 登录失败消息可播报契约（WCAG 2.1 AA / 4.1.3 Status Messages）。
 *
 * 缺口证据：LoginScreen.vue 的登录失败提示是
 *   <div v-if="error" class="error-msg">{{ error }}</div>
 * 认证失败（含需要验证码的 401）后由 v-if 异步插入纯视觉文案，没有 role=alert
 * 或 aria-live；读屏用户不会听到「登录失败/用户名或密码错误」，只看到页面
 * 无变化。全局 formErrorA11y 增强器只处理带 data-oa-field-error 的字段级
 * 错误（XformRuntime），并不覆盖登录页；R226 的 ListLoadError 也只覆盖列表加载
 * 失败分支。因此这是尚未被既有契约覆盖的真实 4.1.3 缺口。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const source = readFileSync(path.join(path.dirname(fileURLToPath(import.meta.url)), 'LoginScreen.vue'), 'utf8')

/** 取登录失败提示 div 的起始标签。 */
function errorTag(): string {
  expect(source, 'LoginScreen.vue 应包含登录失败提示容器').toContain('<div v-if="error" class="error-msg"')
  const idx = source.indexOf('<div v-if="error" class="error-msg"')
  const end = source.indexOf('>', idx)
  expect(end, '失败提示 div 标签未闭合').toBeGreaterThan(idx)
  return source.slice(idx, end + 1)
}

describe('登录失败消息可播报（R228 / WCAG 4.1.3）', () => {
  it('失败提示容器使用 role=alert，读屏器会立即播报', () => {
    expect(errorTag(), 'v-if 动态插入的错误消息必须声明 alert 角色').toMatch(/role="alert"/)
  })

  it('失败提示容器声明 aria-live=assertive，兼容不映射 role 的辅助技术', () => {
    expect(errorTag(), '动态插入的错误消息应有 assertive live region').toMatch(/aria-live="assertive"/)
  })

  it('失败提示仍渲染 error 文本，而不是被语义属性替换掉', () => {
    expect(source, '失败提示容器应保留 v-if="error" 且仍渲染 error 变量').toContain(
      '<div v-if="error" class="error-msg"',
    )
    expect(source).toContain('>{{ error }}</div>')
  })
})
