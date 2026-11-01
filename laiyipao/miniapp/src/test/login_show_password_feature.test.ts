// @vitest-environment node
/**
 * 「登录页显示密码切换」功能守卫（2026-11-01 新一轮第 83 轮）。
 *
 * 测试环境口令较长且不能预填，输入时不可见容易输错。
 * 本轮新增：
 *   - login.vue 密码框旁「显示/隐藏」切换
 *   - 用 `:password="!showPassword"` 控制明文
 *
 * 该功能消失时编译照过、现有测试全绿（少一个链接没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const LOGIN_VUE = join(__dirname, '../pages/login/login.vue')

describe('登录页显示密码切换守卫', () => {
  it('login.vue 必须提供「显示/隐藏」切换', () => {
    const src = readFileSync(LOGIN_VUE, 'utf-8')
    expect(src).toContain('showPassword = !showPassword')
    expect(src).toContain("{{ showPassword ? '隐藏' : '显示' }}")
  })

  it('密码框必须受 showPassword 控制（:password 绑定）', () => {
    const src = readFileSync(LOGIN_VUE, 'utf-8')
    expect(src).toContain(':password="!showPassword"')
    expect(src).toContain('const showPassword = ref(false)')
  })
})