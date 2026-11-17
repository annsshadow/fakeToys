// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * 「离线（挂机）收益」功能守卫。
 *
 * `offline_rewards` 表与 `OFFLINE_CAP_HOURS` 配置存在已久，但一直没有
 * 玩家入口（运维文档曾标注「已解析但当前无离线收益端点消费」）。新增：
 *   - `GET /me/offline-reward` 与 `POST /me/offline-reward/claim`（服务端）
 *   - `client.ts` 的 `offlineRewardStatus` / `claimOfflineReward` / `OfflineRewardStatus`
 *   - 玩家入口：最初在 me.vue「我的」页；第六批 R1 迁到 signin.vue「签到」页，
 *     与「每日签到」这个同为每日领取的动作放在一起。
 *
 * 该功能消失时编译照过、现有测试全绿（页面少一个卡片没有任何报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const CLIENT_TS = join(__dirname, '../api/client.ts')
const SIGNIN_VUE = join(__dirname, '../pages/signin/signin.vue')
const ME_VUE = join(__dirname, '../pages/me/me.vue')

describe('离线收益功能守卫', () => {
  it('client.ts 必须导出离线收益相关方法与类型', () => {
    const src = readFileSync(CLIENT_TS, 'utf-8')
    expect(src).toContain('export interface OfflineRewardStatus')
    expect(src).toContain('export function offlineRewardStatus()')
    expect(src).toContain('export function claimOfflineReward()')
  })

  it('signin.vue 必须包含「离线收益」卡片与领取逻辑', () => {
    const src = readFileSync(SIGNIN_VUE, 'utf-8')
    expect(src).toContain('离线收益')
    expect(src).toContain('claimOffline')
    expect(src).toContain('offlineRewardStatus')
    expect(src).toContain('claimOfflineReward')
  })

  it('signin.vue onMounted 必须并行加载离线收益', () => {
    const src = readFileSync(SIGNIN_VUE, 'utf-8')
    const start = src.indexOf('await Promise.all([loadCalendar()')
    expect(start).toBeGreaterThan(-1)
    const lineEnd = src.indexOf('\n', start)
    const line = src.slice(start, lineEnd > -1 ? lineEnd : src.length)
    expect(line).toContain('loadOfflineReward()')
  })

  it('me.vue 不再重复渲染离线收益入口（单一入口在签到页）', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    expect(src).not.toContain('claimOffline()')
    expect(src).not.toContain('claimOfflineReward')
    expect(src).not.toContain('loadOfflineReward(),')
  })
})
