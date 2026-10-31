// @vitest-environment node
/**
 * 「装备图鉴」功能守卫（2026-10-31 新一轮第 26 轮）。
 *
 * `/config.equipment` 从第一天起就下发了全部 18 件装备的护甲/加成/
 * 解锁等级，但客户端只在背包展示**已拥有**的装备。本轮新增装备图鉴页：
 *   - `pages/equipment/equipment` 路由
 *   - index.vue 首页「装备」入口
 *   - equipment.vue 从 `store.config.equipment` 读取并标注已拥有
 *
 * 该功能消失时编译照过、现有测试全绿（少一个页面/入口没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const PAGES_JSON = join(__dirname, '../pages.json')
const INDEX_VUE = join(__dirname, '../pages/index/index.vue')
const EQUIPMENT_VUE = join(__dirname, '../pages/equipment/equipment.vue')

describe('装备图鉴功能守卫', () => {
  it('pages.json 必须注册装备图鉴页面', () => {
    const src = readFileSync(PAGES_JSON, 'utf-8')
    expect(src).toContain('pages/equipment/equipment')
  })

  it('index.vue 首页必须提供装备图鉴入口', () => {
    const src = readFileSync(INDEX_VUE, 'utf-8')
    expect(src).toContain("path: '/pages/equipment/equipment'")
  })

  it('equipment.vue 必须从服务端配置读取装备数据', () => {
    const src = readFileSync(EQUIPMENT_VUE, 'utf-8')
    expect(src).toContain('store.config?.equipment')
  })
})