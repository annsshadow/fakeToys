// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * Y 批（第二轮探索收敛）结构守卫（2026-11-17）。
 *
 * 第二轮取证发现的都是「功能在、但玩家看不见/点不动」的静默缺口：
 * 空态死文案、隐藏区块无解释、进度只有数字。这类改动删掉后编译照过、
 * 现有行为测试全绿，所以用源码扫描固化。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const read = (rel: string) => readFileSync(join(__dirname, rel), 'utf-8')

describe('Y 批结构守卫', () => {
  it('R51/R52 背包空态可执行化', () => {
    const src = read('../pages/bag/bag.vue')
    expect(src).toContain('去商城买一颗')
    expect(src).toContain('goShop')
    expect(src).toContain('focusPreset')
    expect(src).toContain('已定位到上方输入框')
  })

  it('R53/R54 关卡页空章节与星级进度条', () => {
    const src = read('../pages/stage/stage.vue')
    expect(src).toContain('本章节还没有关卡')
    expect(src).toContain('empty-block')
    expect(src).toContain('chapter-star-track')
    expect(src).toContain('chapter-star-fill')
  })

  it('R55/R56 我的页防线候选空态与结果弹入', () => {
    const src = read('../pages/me/me.vue')
    expect(src).toContain('暂时没有其他玩家的防线')
    expect(src).toContain('class="card pop-in"')
    expect(src).toContain('lastChallenge.won')
  })
})
