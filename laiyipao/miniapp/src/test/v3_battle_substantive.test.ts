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

  it('A4 敌人剪影 R14飞翼/R17BOSS冠/R22精英肩甲/R19抗性角标（全部烘焙，零每帧增量）', () => {
    const src = canvas()
    expect(src).toContain('paintEnemySilhouette')
    expect(src).toContain('R14 飞行双翼')
    expect(src).toContain('R17 BOSS 王冠尖刺')
    expect(src).toContain('R22 精英肩甲')
    expect(src).toContain('R19 抗性角标')
    // 精灵与 drawImage 尺寸必须一致（装饰外扩到 2.8r 不裁切/不变形）
    expect(src).toContain('Math.ceil(rPx * 2.8 + 6)')
    expect(src).toContain('Math.ceil(r * 2.8 + 6)')
  })

  it('A5 BOSS登场缩放 R15 / 编队微扰 R24 / 死亡碎裂规模 R12', () => {
    const src = canvas()
    expect(src).toContain('R15 BOSS 登场缩放')
    expect(src).toContain('spawnScale')
    expect(src).toContain('R24 编队微扰')
    expect(src).toContain('const formation =')
    expect(src).toContain('R12 死亡碎裂规模')
  })

  it('R18 元素层发光圆点（fillRect→arc+fill，零 draw call 增量）', () => {
    const src = canvas()
    expect(src).toContain('R18 元素层数从方块改为发光圆点')
  })

  it('R10 波次横幅 + R13 护盾叠加条 + R30 暴击数字放大', () => {
    const src = canvas()
    expect(src).toContain('waveBanner')
    expect(src).toContain('drawWaveBanner')
    expect(src).toContain('R10 波次横幅')
    expect(src).toContain('R13 护盾叠加条')
    expect(src).toContain('const crit = f.color ===')
  })

  it('R11 行走摆动 + R23 受击击退（只改绘制坐标，零额外 draw call）', () => {
    const src = canvas()
    expect(src).toContain('const gait =')
    expect(src).toContain('const kb = Number(e.knockback)')
    expect(src).toContain('R11 行走摆动 + R23 受击击退')
    expect(src).toContain('ctx.drawImage(body as never, bx - size / 2, by - size / 2')
  })

  it('R16 BOSS 顶部独立血条 + R20 状态图标 + R21 潜地动画', () => {
    const src = canvas()
    expect(src).toContain('drawBossBar')
    expect(src).toContain('R16 BOSS 顶部独立血条')
    expect(src).toContain('R20 状态标记升级为可辨认符号')
    expect(src).toContain('R21 潜地动画')
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
