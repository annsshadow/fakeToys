// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R209 登录页密码可见性按钮无障碍契约（WCAG 4.1.2 Name, Role, Value / Level A）。
 *
 * 缺陷证据：LoginScreen.vue 的 password-wrapper 内，切换明文/密文的
 * <button class="eye-btn"> 唯一内容是 emoji（👁 / 🙈），既没有 aria-label /
 * title 提供可访问名称，也没有 aria-pressed 暴露当前状态。读屏用户在密码框
 * 后会遇到一个「无名按钮」，且无法知道密码当前是隐藏还是可见——这是登录这一
 * 关键路径上的真实可用性缺陷。同文件里的验证码刷新按钮有 aria-label，
 * OAuth 按钮有 :title，唯独此按钮是孤岛。
 *
 * labelA11y 契约只校验 <input>，inputPurposeA11y 只校验 autocomplete，
 * 均未覆盖该按钮；本测试补上这个盲区。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const loginScreen = readFileSync(path.join(here, 'LoginScreen.vue'), 'utf8')

/** 取出眼睛按钮完整标签块（开标签 + 内容 + 闭标签）。 */
function eyeButton(): string {
  const match = loginScreen.match(/<button\b[^>]*class="eye-btn"[^>]*>[\s\S]*?<\/button>/)
  expect(match, '未找到 class="eye-btn" 的按钮').toBeTruthy()
  return match![0]
}

/** 取出按钮开标签，属性都写在开标签上。 */
function eyeButtonOpenTag(): string {
  const btn = eyeButton()
  return btn.slice(0, btn.indexOf('>'))
}

describe('LoginScreen 密码可见性按钮（WCAG 4.1.2 名称/角色/值）', () => {
  it('眼睛按钮必须有可访问名称，不能只有 emoji 文本', () => {
    const openTag = eyeButtonOpenTag()
    const label = openTag.match(/aria-label\s*=\s*"([^"]+)"/)?.[1]
    expect(label, '眼睛按钮缺少 aria-label：读屏用户只会遇到一个无名按钮，无法得知它用于显示/隐藏密码').toBeTruthy()
    expect(label!.trim(), 'aria-label 不能为空').not.toBe('')
    // 名称必须是人类语言，而不是随状态变化的 emoji。
    expect(label, 'aria-label 不能是 emoji，必须是可读文字').toMatch(/[\u4e00-\u9fffA-Za-z]/)
  })

  it('眼睛按钮必须用 aria-pressed 暴露当前切换状态', () => {
    const openTag = eyeButtonOpenTag()
    expect(openTag, '眼睛按钮缺少 :aria-pressed：读屏用户无法知道密码当前是隐藏还是可见').toMatch(
      /:aria-pressed\s*=\s*"showPassword"/,
    )
  })
})
