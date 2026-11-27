// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R98 表单 Enter 提交键盘入口契约（test-first RED）。
 * 真实读三页 vue 源码，禁止内嵌实现副本。
 * 缺口：signin 兑换码 / verify battleId / bag 预设名三处 input 无 @confirm 键盘入口。
 * 范式：r69 双入口、login:56 @confirm=submit。
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

// 取包含 v-model="xxx" 的 input 开标签块（r61 同款稳健定位，不依赖行号）
function inputBlock(src: string, vmodel: string): string | null {
  const vm = src.indexOf('v-model="' + vmodel + '"')
  if (vm < 0) return null
  const start = src.lastIndexOf('<input', vm)
  if (start < 0 || vm - start > 400) return null
  const end = src.indexOf('>', vm)
  return end < 0 ? null : src.slice(start, end + 1)
}

describe('R98 表单 Enter 提交键盘入口契约', () => {
  const signin = read('pages/signin/signin.vue')
  const verify = read('pages/verify/verify.vue')
  const bag = read('pages/bag/bag.vue')
  const me = read('pages/me/me.vue')
  const login = read('pages/login/login.vue')
  const tokens = read('styles/tokens.css')

  const CASES = [
    { name: 'signin 兑换码', src: signin, vmodel: 'code', handler: 'doRedeem', click: '@click="doRedeem"' },
    { name: 'verify battleId', src: verify, vmodel: 'battleId', handler: 'fetchReplay', click: '@click="fetchReplay"' },
    { name: 'bag 预设名', src: bag, vmodel: 'presetName', handler: 'doSavePreset', click: '@click="doSavePreset"' },
  ]

  // 缺口断言：三处 input 须补 @confirm 键盘入口（RED 时这 3 项失败）
  for (const c of CASES) {
    it(c.name + ' input 须有 @confirm="' + c.handler + '" 键盘入口', () => {
      const block = inputBlock(c.src, c.vmodel)
      expect(block, '必须能定位 ' + c.vmodel + ' input 块').not.toBeNull()
      expect(block!, c.name + ' 须补 @confirm="' + c.handler + '"').toContain('@confirm="' + c.handler + '"')
    })
  }

  // 不变量：原 @click 提交入口仍在
  for (const c of CASES) {
    it(c.name + ' 原 @click 提交入口保留', () => {
      expect(c.src, c.name + ' 原 ' + c.click).toContain(c.click)
    })
  }

  it('r69 双入口范式仍成立（me 三处 + login 一处 @confirm）', () => {
    expect(me).toContain('@confirm="submitNickname"')
    expect(me).toContain('@click="submitNickname"')
    expect(me).toContain('@confirm="submitPassword"')
    expect(me).toContain('@confirm="submitUpgrade"')
    expect(login).toContain('@confirm="submit"')
  })

  it('令牌三锚点不动且无 --r98-', () => {
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toMatch(/--r98-/)
  })

  it('业务源码零 --r98- 令牌', () => {
    for (const c of CASES) expect(c.src, c.name).not.toMatch(/--r98-/)
  })
})
