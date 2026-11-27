// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R81 skills 收集进度条一致性美化契约（test-first RED）。
 * skills 图鉴页 KPI 区后须补一条 r3-progress-row 收集进度条，复用 equipment/gems/reactions/mastery
 * 同仓权威范式；r3SkillPct computed 与 r3EquipPct 同构。G66 覆盖矩阵与既有不变量不动。
 * 先 RED 再实现；令牌锚点 v1/v3-r3/r6 不变，禁新增 r81 视觉令牌。
 * 本契约只读页面源码与令牌样式，不触碰受保护域。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const linesOf = (text: string) => text.split(String.fromCharCode(10))


describe('R81 skills 收集进度条一致性美化契约', () => {
  const skills = readSrc('pages/skills/skills.vue')
  const equip = readSrc('pages/equipment/equipment.vue')
  const tokens = readSrc('styles/tokens.css')
  const lines = linesOf(skills)

  it('r3SkillPct computed 存在且与 r3EquipPct 同构', () => {
    const pctLine = lines.find(l => l.indexOf('const r3SkillPct') > -1)
    expect(pctLine, 'skills 必须定义 r3SkillPct computed').toBeTruthy()
    expect(pctLine, 'r3SkillPct 必须经 computed 派生').toContain('computed(')
    expect(pctLine, 'r3SkillPct 分母必须是 skills.value.length').toContain('skills.value.length')
    expect(pctLine, 'r3SkillPct 必须以 ownedCount.value 为分子').toContain('ownedCount.value')
    expect(pctLine, 'r3SkillPct 必须 Math.round 取整').toContain('Math.round')
    expect(pctLine, 'r3SkillPct 必须空列表回退 0').toContain(': 0')
    const equipLine = linesOf(equip).find(l => l.indexOf('const r3EquipPct') > -1)
    expect(equipLine, 'equipment 权威模板 r3EquipPct 必须存在').toBeTruthy()
    expect(equipLine, 'r3SkillPct 结构须与 r3EquipPct 同构').toContain('Math.round')
  })

  it('模板 r3-progress-row 存在且绑定 r3SkillPct、文案收集进度与已解锁', () => {
    const i = lines.findIndex(l => l.indexOf('r3-progress-row') > -1)
    expect(i, 'skills KPI 区后必须有 r3-progress-row').toBeGreaterThan(-1)
    const block = lines.slice(i, i + 5).join(String.fromCharCode(10))
    expect(block, '进度条须含 r3-progress-fill 填充元素').toContain('r3-progress-fill')
    expect(block, '进度条宽度须绑定 r3SkillPct').toContain('r3SkillPct')
    expect(block, '进度条须标注 width 百分比').toContain('width')
    expect(block, '进度条标签须为收集进度').toContain('收集进度')
    expect(block, '计数文案须为已解锁').toContain('已解锁')
    expect(block, '计数分母须为 skills.length').toContain('skills.length')
    expect(block, 'skills 进度条不得使用 is-ai 专属类').not.toContain('is-ai')
    expect(block, '进度条须复用 r3-enter 动效').toContain('r3-enter')
  })

  it('插入位置在 KPI 区之后与 r3-filter 之前', () => {
    const kpiPos = skills.indexOf('class="r3-kpi r3-enter"')
    const filterPos = skills.indexOf('class="r3-filter"')
    const progressPos = skills.indexOf('r3-progress-row')
    expect(kpiPos, 'KPI 区锚点必须存在').toBeGreaterThan(-1)
    expect(filterPos, '筛选行锚点必须存在').toBeGreaterThan(-1)
    expect(progressPos, '进度条必须位于 KPI 与筛选行之间').toBeGreaterThan(kpiPos)
    expect(progressPos, '进度条必须在筛选行之前').toBeLessThan(filterPos)
  })

  it('不变量：G66 元素×类型覆盖矩阵保留', () => {
    expect(skills, 'G66 skill-matrix 容器必须保留').toContain('skill-matrix')
    expect(skills, 'G66 covered 高亮判定必须保留').toContain('covered: cell.owned > 0')
    expect(skills, 'G66 每格 owned/total 文案必须保留').toContain('{{ cell.owned }}/{{ cell.total }}')
    expect(skills, 'G66 覆盖矩阵标题必须保留').toContain('元素 × 类型覆盖')
  })

  it('不变量：ownedCount / 解锁文案 / 筛选 / 搜索 / 结果计数原样', () => {
    const ownedLine = lines.find(l => l.indexOf('const ownedCount = computed(') > -1)
    expect(ownedLine, 'ownedCount computed 必须保留').toBeTruthy()
    const ownedIdx = ownedLine ? lines.indexOf(ownedLine) : -1
    expect(lines[ownedIdx + 1], 'ownedCount 实现必须仍按背包拥有筛选').toContain('ownedSkillIds.value.has(s.id)')
    expect(skills, '解锁状态说明文案必须保留').toContain('个已解锁（背包可查看拥有状态）')
    expect(skills, '元素筛选 chip 必须保留').toContain('class="r3-filter" aria-label="按元素筛选技能"')
    expect(skills, '搜索占位必须保留').toContain('placeholder="搜索技能名称 / 元素 / 主动被动"')
    expect(skills, '结果计数必须保留').toContain('显示 {{ r3FilteredSkills.length }} / 共 {{ skills.length }} 条')
  })

  it('令牌三锚点保留、零 r81 令牌、写域边界', () => {
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(skills + equip + tokens, '不得新增 r81 视觉令牌').not.toContain('--r81-')
    const allowed = new Set(['pages/skills/skills.vue', 'pages/equipment/equipment.vue', 'styles/tokens.css'])
    const self = readFileSync(__filename, 'utf-8')
    const calls = self.match(/readSrc\([^)]+\)/g) || []
    for (const call of calls) {
      const m = call.match(/readSrc\(['"]([^'"]+)['"]\)/)
      if (m) expect(allowed.has(m[1]), '写域越界').toBe(true)
    }
  })
})
