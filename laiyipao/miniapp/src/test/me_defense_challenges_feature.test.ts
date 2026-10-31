// @vitest-environment node
/**
 * 「我的防线被挑战记录」功能守卫（2026-10-31 新一轮第 3 轮）。
 *
 * 挑战记录此前只有挑战者能看，防守方布置防线后不知道谁攻过自己、
 * 被偷了什么。本轮新增：
 *   - `GET /me/defense/challenges`（服务端）
 *   - `client.ts` 的 `myDefenseChallenges` / `DefenseChallengeItem`
 *   - me.vue「被挑战记录」卡片（onShow 并行加载）
 *
 * 该功能消失时编译照过、现有测试全绿（页面少一个卡片没有任何报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const CLIENT_TS = join(__dirname, '../api/client.ts')
const ME_VUE = join(__dirname, '../pages/me/me.vue')

describe('我的防线被挑战记录功能守卫', () => {
  it('client.ts 必须导出 myDefenseChallenges 与 DefenseChallengeItem', () => {
    const src = readFileSync(CLIENT_TS, 'utf-8')
    expect(src).toContain('export function myDefenseChallenges()')
    expect(src).toContain('export interface DefenseChallengeItem')
  })

  it('me.vue 必须包含「被挑战记录」卡片', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    expect(src).toContain('被挑战记录')
    expect(src).toContain('loadDefenseChallengeHistory()')
  })

  it('me.vue onShow 必须并行加载被挑战记录', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    const start = src.indexOf('onShow(async')
    expect(start).toBeGreaterThan(-1)
    const blockStart = src.indexOf('{', start)
    let depth = 0
    let block = ''
    for (let i = blockStart; i < src.length; i++) {
      if (src[i] === '{') depth++
      else if (src[i] === '}') {
        depth--
        if (depth === 0) {
          block = src.slice(start, i + 1)
          break
        }
      }
    }
    expect(block).toContain('loadDefenseChallengeHistory(),')
  })
})