// @vitest-environment node
/**
 * 「宝石图鉴」功能守卫（2026-10-31 新一轮第 27 轮）。
 *
 * `/config.gems` 从第一天起就下发了 8 种宝石的属性与说明，但客户端
 * 只在背包展示**已拥有**的宝石实例。本轮新增宝石图鉴页：
 *   - `pages/gems/gems` 路由
 *   - index.vue 首页「宝石」入口
 *   - gems.vue 从 `store.config.gems` 读取并展示拥有数量
 *
 * 该功能消失时编译照过、现有测试全绿（少一个页面/入口没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const PAGES_JSON = join(__dirname, '../pages.json')
const INDEX_VUE = join(__dirname, '../pages/index/index.vue')
const GEMS_VUE = join(__dirname, '../pages/gems/gems.vue')

describe('宝石图鉴功能守卫', () => {
  it('pages.json 必须注册宝石图鉴页面', () => {
    const src = readFileSync(PAGES_JSON, 'utf-8')
    expect(src).toContain('pages/gems/gems')
  })

  it('index.vue 首页必须提供宝石图鉴入口', () => {
    const src = readFileSync(INDEX_VUE, 'utf-8')
    expect(src).toContain("path: '/pages/gems/gems'")
  })

  it('gems.vue 必须从服务端配置读取宝石数据', () => {
    const src = readFileSync(GEMS_VUE, 'utf-8')
    expect(src).toContain('store.config?.gems')
  })
})