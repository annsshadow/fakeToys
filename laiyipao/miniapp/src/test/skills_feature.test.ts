// @vitest-environment node
/**
 * 「技能图鉴」功能守卫（2026-10-31 新一轮第 25 轮）。
 *
 * `/config.skills` 从第一天起就下发了全部技能的伤害/热量/冷却/穿透/
 * 溅射/解锁等级，但客户端只在背包展示**已拥有**的技能 —— 玩家看不到
 * 未解锁技能的成长路径。本轮新增技能图鉴页：
 *   - `pages/skills/skills` 路由
 *   - index.vue 首页「技能」入口
 *   - skills.vue 从 `store.config.skills` 读取并标注已拥有
 *
 * 该功能消失时编译照过、现有测试全绿（少一个页面/入口没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const PAGES_JSON = join(__dirname, '../pages.json')
const INDEX_VUE = join(__dirname, '../pages/index/index.vue')
const SKILLS_VUE = join(__dirname, '../pages/skills/skills.vue')

describe('技能图鉴功能守卫', () => {
  it('pages.json 必须注册技能图鉴页面', () => {
    const src = readFileSync(PAGES_JSON, 'utf-8')
    expect(src).toContain('pages/skills/skills')
  })

  it('index.vue 首页必须提供技能图鉴入口', () => {
    const src = readFileSync(INDEX_VUE, 'utf-8')
    expect(src).toContain("path: '/pages/skills/skills'")
  })

  it('skills.vue 必须从服务端配置读取技能数据', () => {
    const src = readFileSync(SKILLS_VUE, 'utf-8')
    expect(src).toContain('store.config?.skills')
  })
})