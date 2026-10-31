// @vitest-environment node
/**
 * 「我的战报详情复制回放哈希」功能守卫（2026-11-01 新一轮第 87 轮）。
 *
 * 战报详情展示回放哈希，玩家用外部工具重放验真时需要手动选中一长串字符。
 * 本轮新增：
 *   - battles.vue 详情哈希行下「复制回放哈希」按钮
 *   - 用 uni.setClipboardData 写入剪贴板
 *
 * 该功能消失时编译照过、现有测试全绿（少一个按钮没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const BATTLES_VUE = join(__dirname, '../pages/battles/battles.vue')

describe('我的战报详情复制回放哈希守卫', () => {
  it('battles.vue 必须提供「复制回放哈希」按钮', () => {
    const src = readFileSync(BATTLES_VUE, 'utf-8')
    expect(src).toContain('复制回放哈希')
    expect(src).toContain('copyHash(detail.replay_hash)')
  })

  it('必须用 uni.setClipboardData 写入剪贴板', () => {
    const src = readFileSync(BATTLES_VUE, 'utf-8')
    expect(src).toContain('uni.setClipboardData')
    expect(src).toContain("data: hash")
  })
})