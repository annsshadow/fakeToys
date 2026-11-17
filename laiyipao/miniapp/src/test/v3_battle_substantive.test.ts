// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * 第七批「实质优化 100 轮」结构守卫。
 * 本批只认实质优化：一个完整可视系统 / 让无效状态可见 / 真实缺陷修复 / 结构性提效。
 * 每个关键实现标记被删即红。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const canvas = () => readFileSync(join(__dirname, '../render/canvas.ts'), 'utf-8')

describe('第七批 战斗画面实质优化守卫', () => {
  it('R1 远景山脊视差层', () => {
    const src = canvas()
    expect(src).toContain('farLayer')
    expect(src).toContain('drawFarParallax')
    expect(src).toContain('ensureFarLayer')
    expect(src).toContain('R1 远景山脊视差层')
  })

  it('R2 地面战损纹理烘焙进静态层', () => {
    const src = canvas()
    expect(src).toContain('paintBattleScars')
    expect(src).toContain('R2 地面战损纹理')
  })

  it('R3 边界能量栅栏', () => {
    const src = canvas()
    expect(src).toContain('R3 边界能量栅栏')
    expect(src).toContain('fencePulse')
  })

  it('R5 波次地面脉冲 + R9 击杀地面残留', () => {
    const src = canvas()
    expect(src).toContain('groundPulseMs')
    expect(src).toContain('drawGroundPulse')
    expect(src).toContain('killScars')
    expect(src).toContain('drawKillScars')
    expect(src).toContain('R9 击杀地面残留')
  })

  it('R6 出生门双环旋转 + R7 防线警戒区', () => {
    const src = canvas()
    expect(src).toContain('R6 出生门双环')
    expect(src).toContain('paintDefenseZone')
    expect(src).toContain('R7 防线警戒')
  })

  it('R4 氛围粒子（单 path，不污染 arc 判据、不破 perf 预算）', () => {
    const src = canvas()
    expect(src).toContain('drawAmbientParticles')
    expect(src).toContain('R4 氛围粒子')
    // 粒子必须在单条 path 上用 stroke 提交（否则 perf 与封顶用例会红）
    const start = src.indexOf('private drawAmbientParticles')
    const end = src.indexOf('private drawFarParallax', start)
    const block = src.slice(start, end)
    expect(block).toContain('ctx.stroke()')
    const codeOnly = block.replace(/\/\*[\s\S]*?\*\//g, '').replace(/\/\/.*$/gm, '')
    expect(codeOnly).not.toContain('fillRect')
    expect(codeOnly).not.toContain('ctx.arc')
  })
})
