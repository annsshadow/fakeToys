// @vitest-environment node
/**
 * 「护盾剩余时间显示」功能守卫（2026-10-31 新一轮第 33 轮）。
 *
 * 玩家开启防线护盾后不知道还剩多久。本轮在 me.vue 增加：
 *   - `shieldRemainText` computed（用服务端时钟基准计算剩余时长）
 *   - 护盾开关按钮下方展示剩余时间文案
 *
 * 该功能消失时编译照过、现有测试全绿（少一行文案没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const ME_VUE = join(__dirname, '../pages/me/me.vue')

describe('护盾剩余时间显示守卫', () => {
  it('me.vue 必须计算护盾剩余时间', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    expect(src).toContain('shieldRemainText = computed')
  })

  it('me.vue 必须在护盾开关旁展示剩余时间', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    expect(src).toContain('v-if="shieldRemainText"')
    expect(src).toContain('{{ shieldRemainText }}')
  })
})