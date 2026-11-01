// @vitest-environment node
/**
 * 「敌人图鉴」功能守卫（2026-10-31 新一轮第 24 轮）。
 *
 * `/config.enemies` 从第一天起就下发了完整敌人属性（血量/护甲/护盾/
 * 攻击/速度/五维抗性），但客户端零 UI 消费 —— 玩家战斗前无法了解
 * 敌人特性。本轮新增敌人图鉴页：
 *   - `pages/enemies/enemies` 路由
 *   - index.vue 首页「敌人」入口
 *   - enemies.vue 从 `store.config.enemies` 读取并展示
 *
 * 该功能消失时编译照过、现有测试全绿（少一个页面/入口没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const PAGES_JSON = join(__dirname, '../pages.json')
const INDEX_VUE = join(__dirname, '../pages/index/index.vue')
const ENEMIES_VUE = join(__dirname, '../pages/enemies/enemies.vue')

describe('敌人图鉴功能守卫', () => {
  it('pages.json 必须注册敌人图鉴页面', () => {
    const src = readFileSync(PAGES_JSON, 'utf-8')
    expect(src).toContain('pages/enemies/enemies')
  })

  it('index.vue 首页必须提供敌人图鉴入口', () => {
    const src = readFileSync(INDEX_VUE, 'utf-8')
    expect(src).toContain("path: '/pages/enemies/enemies'")
  })

  it('enemies.vue 必须从服务端配置读取敌人数据', () => {
    const src = readFileSync(ENEMIES_VUE, 'utf-8')
    expect(src).toContain('store.config?.enemies')
  })
})