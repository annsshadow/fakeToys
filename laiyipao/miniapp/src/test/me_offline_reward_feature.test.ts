// @vitest-environment node
/**
 * 「离线（挂机）收益」功能守卫（2026-10-31 新一轮第 7 轮）。
 *
 * `offline_rewards` 表与 `OFFLINE_CAP_HOURS` 配置存在已久，但一直没有
 * 玩家入口（运维文档曾标注「已解析但当前无离线收益端点消费」）。本轮新增：
 *   - `GET /me/offline-reward` 与 `POST /me/offline-reward/claim`（服务端）
 *   - `client.ts` 的 `offlineRewardStatus` / `claimOfflineReward` / `OfflineRewardStatus`
 *   - me.vue「离线收益」卡片（onShow 并行加载、领取后刷新钱包）
 *
 * 该功能消失时编译照过、现有测试全绿（页面少一个卡片没有任何报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const CLIENT_TS = join(__dirname, '../api/client.ts')
const ME_VUE = join(__dirname, '../pages/me/me.vue')

describe('离线收益功能守卫', () => {
  it('client.ts 必须导出离线收益相关方法与类型', () => {
    const src = readFileSync(CLIENT_TS, 'utf-8')
    expect(src).toContain('export interface OfflineRewardStatus')
    expect(src).toContain('export function offlineRewardStatus()')
    expect(src).toContain('export function claimOfflineReward()')
  })

  it('me.vue 必须包含「离线收益」卡片与领取逻辑', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    expect(src).toContain('离线收益')
    expect(src).toContain('claimOffline()')
  })

  it('me.vue onShow 必须并行加载离线收益', () => {
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
    expect(block).toContain('loadOfflineReward(),')
  })
})