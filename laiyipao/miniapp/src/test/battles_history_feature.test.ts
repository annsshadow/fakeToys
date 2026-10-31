// @vitest-environment node
/**
 * 「我的战报历史页」功能守卫（2026-11-01 新一轮第 41 轮）。
 *
 * `/me/battles` 与 `/me/battles/:id` 端点早已存在，但客户端此前没有任何
 * 战报历史入口 —— 玩家打完一局就再也找不到那一局的记录。本轮新增：
 *   - battles.vue 页面：调用 `myBattleHistory` 拉取最近 20 场，
 *     点开一行按需调用 `myBattleDetail` 回顾掉落/元素/反应/地形
 *   - pages.json 注册路由；首页菜单加入口
 *
 * 该功能消失时编译照过、现有测试全绿（少一个区块没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const BATTLES_VUE = join(__dirname, '../pages/battles/battles.vue')
const PAGES_JSON = join(__dirname, '../pages.json')
const INDEX_VUE = join(__dirname, '../pages/index/index.vue')

describe('我的战报历史页守卫', () => {
  it('battles.vue 必须拉取战报列表并展示胜负/星级/分数/击杀', () => {
    const src = readFileSync(BATTLES_VUE, 'utf-8')
    expect(src).toContain('api.myBattleHistory()')
    expect(src).toContain('it.result === \'win\'')
    expect(src).toContain('it.kills')
    expect(src).toContain('it.score')
  })

  it('点击战报行必须按需拉取详情并展示掉落/元素/反应/地形', () => {
    const src = readFileSync(BATTLES_VUE, 'utf-8')
    expect(src).toContain('api.myBattleDetail(it.id)')
    expect(src).toContain('details.value[it.id]')
    expect(src).toContain('lootText(detail.loot)')
    expect(src).toContain('d.elements_used')
    expect(src).toContain('d.reactions_used')
    expect(src).toContain('d.terrain_used')
  })

  it('pages.json 注册路由且首页菜单有「战报」入口', () => {
    const pages = readFileSync(PAGES_JSON, 'utf-8')
    expect(pages).toContain('pages/battles/battles')
    const index = readFileSync(INDEX_VUE, 'utf-8')
    expect(index).toContain("'/pages/battles/battles'")
    expect(index).toContain("label: '战报'")
  })
})