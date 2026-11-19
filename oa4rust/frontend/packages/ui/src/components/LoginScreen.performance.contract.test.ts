// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * L1 性能契约：登录页粒子背景不得在低端设备上退化为逐帧 CPU 多边形绘制。
 *
 * 缺陷证据：LoginScreen.vue 每帧对 80 个粒子做 O(n²)=3160 次连线计算；画布按
 * CSS 像素设置宽高，在 DPR≥2 的屏幕上被放大 4 倍并交给 GPU 合成；Google Fonts
 * 外链在测试服离网环境会产生长时间的字体阻塞（FOUT/白屏）。
 *
 * 契约：① 粒子数显式封顶（≤80），连线设置上限；
 *       ② 画布宽高对 devicePixelRatio 做封顶并 ctx.setTransform 缩放；
 *       ③ 页面隐藏时暂停 rAF，恢复时再启动；
 *       ④ UnoCSS preflight 不得注入远程字体 URL（字体回退系统栈）。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const uiRoot = resolve(import.meta.dirname, '../../src')
const login = readFileSync(resolve(import.meta.dirname, 'LoginScreen.vue'), 'utf8')
const uno = readFileSync(resolve(uiRoot, 'unocss.config.ts'), 'utf8')

describe('登录页粒子背景与字体加载性能（L1）', () => {
  it('粒子数显式封顶（≤80），避免高分辨率屏幕粒子数膨胀', () => {
    expect(login).toMatch(/Math\.min\(\s*80\s*,/)
  })

  it('粒子连线循环存在 skip/上限，杜绝无界 O(n²)', () => {
    expect(login).toMatch(/LINE_LIMIT/)
    expect(login).toMatch(/break/)
  })

  it('画布尺寸按 devicePixelRatio 缩放并封顶，且存在 setTransform', () => {
    expect(login).toMatch(/devicePixelRatio/)
    expect(login).toMatch(/setTransform/)
  })

  it('document visibilitychange 时暂停/恢复粒子动画', () => {
    expect(login).toMatch(/visibilitychange/)
    expect(login).toMatch(/document\.hidden/)
  })

  it('UnoCSS 不注入远程 Google Fonts 外链（无新依赖，系统字体栈回退）', () => {
    expect(uno).not.toMatch(/fonts\.googleapis\.com/)
  })
})
