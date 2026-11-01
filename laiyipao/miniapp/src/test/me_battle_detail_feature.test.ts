// @vitest-environment node
/**
 * 「我的战报详情」功能守卫（2026-10-31 新一轮第 10 轮）。
 *
 * 战报列表只有摘要（关卡/结果/星级/分数），玩家想回顾单局的掉落、
 * 元素/反应/地形使用、命中、峰值热量等没有任何入口。本轮新增：
 *   - `GET /me/battles/:id`（服务端）
 *   - `client.ts` 的 `myBattleDetail` / `BattleDetail`
 *   - me.vue 战报行「详情」入口（点击展开单局完整详情）
 *
 * 该功能消失时编译照过、现有测试全绿（页面少一个链接没有任何报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const CLIENT_TS = join(__dirname, '../api/client.ts')
const ME_VUE = join(__dirname, '../pages/me/me.vue')

describe('战报详情功能守卫', () => {
  it('client.ts 必须导出 myBattleDetail 与 BattleDetail', () => {
    const src = readFileSync(CLIENT_TS, 'utf-8')
    expect(src).toContain('export interface BattleDetail')
    expect(src).toContain('export function myBattleDetail(')
  })

  it('me.vue 必须包含战报详情入口与展开逻辑', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    expect(src).toContain('toggleBattleDetail(b)')
    expect(src).toContain('battleDetail.value = await api.myBattleDetail(b.id)')
  })
})