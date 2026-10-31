// @vitest-environment node
/**
 * 「皮肤图鉴」功能守卫（2026-10-31 新一轮第 35 轮）。
 *
 * `/config.skins` 从第一天起就下发了 6 款皮肤的稀有度/被动/说明，但
 * 客户端零 UI 消费。本轮新增皮肤图鉴页：
 *   - `pages/skins/skins` 路由
 *   - index.vue 首页「皮肤」入口
 *   - skins.vue 从 `store.config.skins` 读取并展示
 *
 * 该功能消失时编译照过、现有测试全绿（少一个页面/入口没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const PAGES_JSON = join(__dirname, '../pages.json')
const INDEX_VUE = join(__dirname, '../pages/index/index.vue')
const SKINS_VUE = join(__dirname, '../pages/skins/skins.vue')

describe('皮肤图鉴功能守卫', () => {
  it('pages.json 必须注册皮肤图鉴页面', () => {
    const src = readFileSync(PAGES_JSON, 'utf-8')
    expect(src).toContain('pages/skins/skins')
  })

  it('index.vue 首页必须提供皮肤图鉴入口', () => {
    const src = readFileSync(INDEX_VUE, 'utf-8')
    expect(src).toContain("path: '/pages/skins/skins'")
  })

  it('skins.vue 必须从服务端配置读取皮肤数据', () => {
    const src = readFileSync(SKINS_VUE, 'utf-8')
    expect(src).toContain('store.config?.skins')
  })
})