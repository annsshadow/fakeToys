// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R69 昵称提交在途反馈与防连点契约（test-first RED）。
 * 真实读 laiyipao/miniapp/src/pages/me/me.vue 源码，禁止内嵌实现副本。
 * 缺口：submitNickname 无 busy ref / 无入口守卫 / 无 finally 复位；保存按钮与输入框无禁用态、无「提交中…」文案。
 * 范式：同页 submitPassword 的 passwordSubmitting 守卫与模板 :class/:disabled/文案。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

function blockAfter(text: string, keyword: string): string | null {
  const k = text.indexOf(keyword)
  if (k < 0) return null
  const open = text.indexOf('{', k)
  if (open < 0) return null
  let depth = 0
  for (let i = open; i < text.length; i++) {
    const ch = text[i]
    if (ch === '{') depth++
    else if (ch === '}') {
      depth--
      if (depth === 0) return text.slice(open, i + 1)
    }
  }
  return null
}

describe('R69 昵称提交在途反馈与防连点契约', () => {
  const me = readSrc('pages/me/me.vue')
  const body = blockAfter(me, 'async function submitNickname()')

  it('submitNickname 须有 busy ref 声明、入口 if 守卫与 finally 复位', () => {
    expect(body, '必须能定位 submitNickname 函数块').not.toBeNull()
    const b = body!
    expect(me, '须声明 nicknameSubmitting = ref(false)').toMatch(/const\s+nicknameSubmitting\s*=\s*ref\(false\)/)
    const ai = b.indexOf('await ')
    const head = ai < 0 ? b : b.slice(0, ai)
    expect(/if\s*\(\s*nicknameSubmitting\.value\s*\)\s*return/.test(head),
      '首个 await 前须有 if (nicknameSubmitting.value) return 入口守卫（原始缺陷：连点可发两次 PUT）').toBe(true)
    const fi = b.indexOf('finally')
    expect(fi, '函数块须含 finally 复位分支').toBeGreaterThan(-1)
    expect(/finally\s*\{[^}]*nicknameSubmitting\.value\s*=\s*false/.test(b.slice(fi)),
      'finally 中须 nicknameSubmitting.value = false，保证成功/失败都复位').toBe(true)
    expect(/nicknameSubmitting\.value\s*=\s*true/.test(b), '校验通过后须置 nicknameSubmitting.value = true').toBe(true)
  })

  it('保存按钮须有禁用态 class 与「提交中…」条件文案', () => {
    const btn = me.match(/<view[^>]*@click="submitNickname"[^>]*>[\s\S]*?<\/view>/)
    expect(btn, '须能定位保存按钮（@click="submitNickname"）').not.toBeNull()
    expect(btn![0], '按钮须绑定 :class 且 btn-disabled 键（可带引号，JS/Vue 合法写法）值为 nicknameSubmitting').toMatch(/:class="\{\s*['"]?\s*btn-disabled\s*['"]?\s*:\s*nicknameSubmitting\s*\}"/)
    expect(btn![0], '按钮文案须含 nicknameSubmitting ? 提交中… : 保存').toMatch(/nicknameSubmitting\s*\?\s*'提交中…'\s*:\s*'保存'/)
  })

  it('昵称输入框须在在途期间 :disabled', () => {
    const inp = me.match(/<input[\s\S]*?v-model="nicknameDraft"[\s\S]*?\/>/)
    expect(inp, '须能定位 v-model="nicknameDraft" 输入框').not.toBeNull()
    expect(inp![0], '输入框须 :disabled="nicknameSubmitting" 防键盘 confirm 连发').toContain(':disabled="nicknameSubmitting"')
  })

  it('不变量：校验、权威调用、R61 语义与令牌三锚点保持', () => {
    expect(body!, 'trim 校验保持不变').toContain('nicknameDraft.value.trim()')
    expect(body!, '码点 64 校验保持不变').toContain('[...name].length > 64')
    expect(body!, '空值文案不变').toContain('昵称不能为空')
    expect(body!, '超长文案不变').toContain('昵称最长 64 个字符')
    expect(body!, 'api.updateNickname 权威调用不变').toContain('await api.updateNickname(name)')
    expect(body!, 'refreshProfile 权威调用不变').toContain('await store.refreshProfile()')
    expect(body!, 'catch 错误文案不变').toContain('修改失败，请稍后再试')
    expect(me, 'R61 v-aria-native 标签保持').toContain('v-aria-native=')
    expect(me, 'R61 v-aria-native 文案保持').toContain('新昵称（≤64 字）')
    expect(me, 'R61 aria-label 保持').toContain('aria-label="新昵称（≤64 字）"')
    const pw = blockAfter(me, 'async function submitPassword()')
    expect(pw, 'submitPassword 正面对照须保留').not.toBeNull()
    expect(pw!, 'passwordSubmitting 守卫保留').toMatch(/if\s*\(\s*passwordSubmitting\.value\s*\)\s*return/)
    const tokens = readSrc('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(me + tokens, '不得新增 r69 视觉令牌').not.toMatch(/--r69-/)
  })

  it('@confirm 与 @click 两条入口均指向 submitNickname', () => {
    expect(me, '输入框 confirm 入口保持').toMatch(/@confirm="submitNickname"/)
    expect(me, '保存按钮 click 入口保持').toMatch(/@click="submitNickname"/)
  })
})