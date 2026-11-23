// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * 「我的页面战斗音效开关」功能守卫（2026-11-01 新一轮第 79 轮）。
 *
 * 音效系统支持静音并持久化（lyp_sfx_muted），但此前没有任何 UI 入口 ——
 * 玩家只能被战斗声音打扰或手动清缓存。本轮新增：
 *   - me.vue「设置」卡：战斗音效 switch
 *   - 切换时调用 setMuted，初始状态读取 isMuted
 *
 * 该功能消失时编译照过、现有测试全绿（少一个开关没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const ME_VUE = join(__dirname, '../pages/me/me.vue')

describe('我的页面战斗音效开关守卫', () => {
  it('me.vue 必须提供「战斗音效」开关', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    expect(src).toContain('战斗音效')
    expect(src).toContain('<switch')
  })

  it('R13 开关语义：checked 绑定"音效开启"而非静音，切换时取反传给 setMuted', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    expect(src).toContain(':checked="sfxEnabled"')
    expect(src).not.toContain(':checked="sfxMuted"')
    expect(src).toContain('const sfxEnabled = ref(!isMuted())')
    expect(src).toContain('setMuted(!next)')
    expect(src).toContain('sfxEnabled.value = next')
  })
})