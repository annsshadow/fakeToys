// @vitest-environment node
/**
 * 「公告列表页」功能守卫（2026-11-01 新一轮第 42 轮）。
 *
 * 玩家侧公告端点最多返回 20 条，而首页只展示最新几条 ——
 * 历史公告在首页不可回看，运营发过的活动规则/补偿说明就此消失。
 * 本轮新增：
 *   - announcements.vue 页面：`fetchAnnouncements(20)` 拉取全部公告，
 *     标题/发布时间/正文全文平铺展示
 *   - pages.json 注册路由；首页公告卡片提供「全部 N 条 →」入口
 *
 * 该功能消失时编译照过、现有测试全绿（少一个区块没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const ANN_VUE = join(__dirname, '../pages/announcements/announcements.vue')
const PAGES_JSON = join(__dirname, '../pages.json')
const INDEX_VUE = join(__dirname, '../pages/index/index.vue')

describe('公告列表页守卫', () => {
  it('announcements.vue 必须拉取全量公告（上限 20）并平铺正文', () => {
    const src = readFileSync(ANN_VUE, 'utf-8')
    expect(src).toContain('api.fetchAnnouncements(20)')
    expect(src).toContain('a.title')
    expect(src).toContain('a.body')
    expect(src).toContain('a.published_at')
  })

  it('pages.json 注册路由', () => {
    const pages = readFileSync(PAGES_JSON, 'utf-8')
    expect(pages).toContain('pages/announcements/announcements')
  })

  it('首页公告卡片必须提供「全部 N 条 →」入口并跳转公告页', () => {
    const index = readFileSync(INDEX_VUE, 'utf-8')
    expect(index).toContain('全部 {{ announcements.length }} 条 →')
    expect(index).toContain("'/pages/announcements/announcements'")
  })
})