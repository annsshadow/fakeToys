// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * X 批 R31/R32 微交互守卫（2026-11-17）。
 *
 * 按压反馈与骨架屏都是「看不见也能编译通过」的纯样式改动 ——
 * 用源码扫描把它变成结构约束：删除后测试立刻失败。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const read = (rel: string) => readFileSync(join(__dirname, rel), 'utf-8')
const CSS = () => read('../styles/common.css')

describe('微交互与加载反馈守卫', () => {
  it('R31 按钮/链接按压反馈 + 减少动效无障碍分支', () => {
    const css = CSS()
    expect(css).toContain('.btn:active:not(.btn-disabled)')
    expect(css).toContain('.link:active')
    expect(css).toContain('@media (prefers-reduced-motion: reduce)')
  })

  it('R32 共享骨架屏样式落地', () => {
    const css = CSS()
    expect(css).toContain('.skeleton-row')
    expect(css).toContain('.skeleton-bar')
    expect(css).toContain('@keyframes skeleton-shimmer')
  })

  it('R32 四个列表页加载态使用骨架占位且保留加载文案', () => {
    for (const p of ['rank', 'battles', 'shop', 'announcements']) {
      const src = read(`../pages/${p}/${p}.vue`)
      expect(src, p).toContain('skeleton-row')
      expect(src, p).toContain('加载中…')
    }
  })

  it('R33 验真裁定结果带弹入动效', () => {
    const src = read('../pages/verify/verify.vue')
    expect(src).toContain('verdict-box pop-in')
    expect(src).toContain('card pop-in')
    expect(CSS()).toContain('@keyframes pop-in')
  })

  it('R34 签到/领奖成功高亮动效', () => {
    const src = read('../pages/signin/signin.vue')
    expect(src).toContain('claim-pop')
    expect(src).toContain('flashFeedback')
    expect(CSS()).toContain('@keyframes claim-pop')
  })

  it('R35 商城购买成功行高亮', () => {
    const src = read('../pages/shop/shop.vue')
    expect(src).toContain('bought-flash')
    expect(src).toContain('justBoughtId')
  })

  it('R36 六个图鉴页列表错峰入场', () => {
    expect(CSS()).toContain('@keyframes stagger-in')
    for (const p of ['enemies', 'equipment', 'skills', 'skins', 'gems', 'reactions']) {
      const src = read(`../pages/${p}/${p}.vue`)
      expect(src, p).toContain('stagger-in')
    }
  })

  it('R37 错误消息抖动反馈（key 强制重播）', () => {
    expect(CSS()).toContain('@keyframes shake')
    for (const p of ['login', 'verify', 'signin']) {
      const src = read(`../pages/${p}/${p}.vue`)
      expect(src, p).toContain('shake')
      expect(src, p).toContain(':key=')
    }
  })

  it('R38 进度完成态脉冲', () => {
    expect(CSS()).toContain('@keyframes ok-pulse')
    expect(read('../pages/shop/shop.vue')).toContain("'ok-pulse': soldOut(it)")
    expect(read('../pages/signin/signin.vue')).toContain("'ok-pulse': t.done")
  })

  it('R39 加载态转圈 spinner', () => {
    expect(CSS()).toContain('@keyframes spin')
    for (const p of ['login', 'shop', 'verify']) {
      expect(read(`../pages/${p}/${p}.vue`), p).toContain('spinner')
    }
  })

  it('R40 签到连续进度条', () => {
    const src = read('../pages/signin/signin.vue')
    expect(src).toContain('streak-track')
    expect(src).toContain('streak-fill')
    expect(src).toContain('streakPct')
  })

  it('R41 当前关播放键提示', () => {
    const src = read('../pages/stage/stage.vue')
    expect(src).toContain('play-glyph')
    expect(src).toContain('@keyframes play-pulse')
  })

  it('R42 首页资源变动高亮', () => {
    const src = read('../pages/index/index.vue')
    expect(src).toContain('res-bump')
    expect(src).toContain('bump.coin')
    expect(CSS()).toContain('@keyframes res-bump')
  })

  it('R43 专精可选节点加号脉冲', () => {
    const src = read('../pages/mastery/mastery.vue')
    expect(src).toContain('node-plus')
    expect(src).toContain('@keyframes node-plus-pulse')
  })

  it('R44 页面进入淡入', () => {
    expect(CSS()).toContain('animation: page-in')
    expect(CSS()).toContain('@keyframes page-in')
  })
})
