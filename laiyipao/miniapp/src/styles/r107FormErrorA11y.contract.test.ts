// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R107 表单校验错误 a11y 语义契约（test-first RED）
 * 真实 fs.readFileSync 读 login/verify/signin.vue + main.ts；唯一锚点（v-model + class）定位，不依赖行号。
 * 缺口：错误态 input 无 aria-invalid / aria-describedby；错误节点无唯一 id；桥接指令未转发两属性。
 * 不变量：R61 aria-label/v-aria-native、R60 键盘三件套、R101 aria-expanded、R104 aria-busy、令牌三锚点、零 --r107-。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'

const dir = dirname(fileURLToPath(import.meta.url))
const srcRoot = join(dir, '..')

function read(rel: string): string {
  return readFileSync(join(srcRoot, rel), 'utf8')
}

// 取包含 v-model="xxx" 的 input 开标签块（跨行容错，不依赖行号）
function inputBlock(src: string, vmodel: string): string | null {
  const vm = src.indexOf('v-model="' + vmodel + '"')
  if (vm < 0) return null
  const start = src.lastIndexOf('<input', vm)
  if (start < 0 || vm - start > 400) return null
  const end = src.indexOf('>', vm)
  return end < 0 ? null : src.slice(start, end + 1)
}

// 取 needle 之前最近的 text/view 开标签块
function tagBefore(src: string, needle: string): string | null {
  const i = src.indexOf(needle)
  if (i < 0) return null
  const t = src.lastIndexOf('<text', i)
  const v = src.lastIndexOf('<view', i)
  const start = Math.max(t, v)
  if (start < 0) return null
  const gt = src.indexOf('>', start)
  return gt < 0 ? null : src.slice(start, gt + 1)
}

// 取开标签中某属性的绑定值（不依赖行号/引号风格）
function attrValue(block: string, attr: string): string | null {
  const key = attr + '="'
  const i = block.indexOf(key)
  if (i < 0) return null
  const rest = block.slice(i + key.length)
  const end = rest.indexOf('"')
  return end < 0 ? null : rest.slice(0, end)
}

const LOGIN = 'pages/login/login.vue'
const VERIFY = 'pages/verify/verify.vue'
const SIGNIN = 'pages/signin/signin.vue'
const MAIN = 'main.ts'

// 4 个错误态 input：[rel, v-model, 错误条件关键串, 目标 describedby id]
const ERROR_INPUTS: [string, string, string, string][] = [
  [LOGIN, 'username', 'errMsg', 'login-err'],
  [LOGIN, 'password', 'errMsg', 'login-err'],
  [VERIFY, 'battleId', 'errMsg', 'verify-battle-err'],
  [SIGNIN, 'code', 'redeemMsg', 'signin-redeem-err'],
]

// 3 个错误播报节点：[rel, needle, id, 既有 role/live 锚点]
const ERROR_NODES: [string, string, string, string][] = [
  [LOGIN, '{{ errMsg }}', 'login-err', 'role="alert"'],
  [VERIFY, '{{ errMsg }}', 'verify-battle-err', 'role="alert"'],
  [SIGNIN, '{{ redeemMsg }}', 'signin-redeem-err', 'aria-live="polite"'],
]

describe('R107 表单校验错误 a11y 语义契约', () => {
  const cache = new Map<string, string>()
  const src = (rel: string) => {
    let s = cache.get(rel)
    if (s === undefined) { s = read(rel); cache.set(rel, s) }
    return s
  }

  for (const [rel, vm, cond, id] of ERROR_INPUTS) {
    it('缺口：' + rel + ' ' + vm + ' 错误态语义（:aria-invalid 绑定失败态 + describedby 指向 ' + id + '）', () => {
      const block = inputBlock(src(rel), vm)
      expect(block, rel + ' v-model=' + vm + ' 未找到 input 开标签').not.toBeNull()
      const invalid = attrValue(block!, ':aria-invalid')
      expect(invalid, rel + ' ' + vm + ' 缺 :aria-invalid 绑定').not.toBeNull()
      expect(invalid!, rel + ' ' + vm + ' 的 :aria-invalid 未绑定 ' + cond + ' 失败态').toContain(cond)
      expect(invalid!, rel + ' ' + vm + ' 不应静态写死 true').not.toBe('true')
      const desc = attrValue(block!, ':aria-describedby')
      expect(desc, rel + ' ' + vm + ' 缺 :aria-describedby 绑定').not.toBeNull()
      expect(desc!, rel + ' ' + vm + ' describedby 未指向 ' + id).toContain(id)
    })
  }

  for (const [rel, needle, id, roleAnchor] of ERROR_NODES) {
    it('缺口：' + rel + ' 错误播报节点补唯一 id="' + id + '" 且保留 ' + roleAnchor, () => {
      const tag = tagBefore(src(rel), needle)
      expect(tag, rel + ' 未找到 ' + needle + ' 的开标签').not.toBeNull()
      expect(tag!, rel + ' 错误节点缺 id="' + id + '"').toContain('id="' + id + '"')
      expect(tag!, rel + ' 错误节点丢失既有 ' + roleAnchor).toContain(roleAnchor)
    })
  }

  it('缺口：main.ts applyAriaNative 桥接 aria-invalid 到原生 input', () => {
    const s = src(MAIN)
    expect(s, 'main.ts 缺 applyAriaNative').toContain('function applyAriaNative')
    const bridged = s.split('\n').some((l) => l.includes('aria-invalid') && l.includes('setAttribute'))
    expect(bridged, '桥接未把 aria-invalid setAttribute 到原生 input').toBe(true)
  })

  it('缺口：main.ts applyAriaNative 桥接 aria-describedby 到原生 input', () => {
    const s = src(MAIN)
    const bridged = s.split('\n').some((l) => l.includes('aria-describedby') && l.includes('setAttribute'))
    expect(bridged, '桥接未把 aria-describedby setAttribute 到原生 input').toBe(true)
  })

  // ---------- 不变量（RED 期即应通过） ----------
  it('不变量：R61 原生 input aria-label 桥接保留且 4 个目标 input 仍有 aria-label + v-aria-native', () => {
    const labelBridge = src(MAIN).split('\n').some((l) => l.includes('setAttribute') && l.includes('aria-label'))
    expect(labelBridge, 'R61 setAttribute(aria-label) 被回退').toBe(true)
    for (const [rel, vm] of ERROR_INPUTS) {
      const block = inputBlock(src(rel), vm)
      expect(block, rel + ' ' + vm + ' 未找到').not.toBeNull()
      expect(block!, rel + ' ' + vm + ' 丢 aria-label').toContain('aria-label="')
      expect(block!, rel + ' ' + vm + ' 丢 v-aria-native').toContain('v-aria-native')
    }
  })

  it('不变量：R60 键盘三件套（me back-btn）未被误改', () => {
    const s = src('pages/me/me.vue')
    const i = s.indexOf('class="back-btn tap"')
    expect(i, 'me back-btn 锚点丢失').toBeGreaterThan(-1)
    const gt = s.indexOf('>', i)
    const tag = s.slice(s.lastIndexOf('<text', i), gt + 1)
    expect(tag).toContain('role="button"')
    expect(tag).toContain('tabindex="0"')
    expect(tag).toContain('@keydown.enter')
  })

  it('不变量：R101 aria-expanded 与 R104 aria-busy 未被误改', () => {
    expect(src('pages/index/index.vue')).toContain(':aria-expanded="String(expanded.has(a.id))"')
    const eq = src('pages/equipment/equipment.vue')
    expect(eq).toContain('aria-busy="true"')
    expect(eq).toContain('role="status"')
  })

  it('不变量：令牌三锚点不动且 3 页 + main.ts 零 --r107-', () => {
    const tokens = read('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    for (const rel of [LOGIN, VERIFY, SIGNIN, MAIN]) {
      expect(src(rel), rel + ' 出现 --r107-').not.toContain('--r107-')
    }
  })
})
