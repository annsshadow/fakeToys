// @vitest-environment node
/**
 * 「我的页面复制用户 ID」功能守卫（2026-11-01 新一轮第 90 轮）。
 *
 * 玩家在社区/反馈中报问题时需要提供用户 ID，此前只能手动记住/截图。
 * 本轮新增：
 *   - me.vue 账号卡 ID 行「复制」链接
 *   - 用 uni.setClipboardData 写入并 toast 提示
 *
 * 该功能消失时编译照过、现有测试全绿（少一个链接没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const ME_VUE = join(__dirname, '../pages/me/me.vue')

describe('我的页面复制用户 ID 守卫', () => {
  it('me.vue 必须提供「复制」用户 ID 链接', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    expect(src).toContain('ID {{ store.userId }}')
    expect(src).toContain('@click="copyUserId"')
  })

  it('必须用 uni.setClipboardData 写入用户 ID', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    expect(src).toContain('uni.setClipboardData')
    expect(src).toContain('data: String(store.userId)')
  })
})