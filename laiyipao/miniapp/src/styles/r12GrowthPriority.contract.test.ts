// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R12 玩法实测契约：跨系统成长目标感。
 * 真浏览器实测发现：首页 aiInsight（marginalGain）算出「优先补装备契合」，
 * 但 skills/equipment/gems/bag 四页各自用本地规则给出互相矛盾的「推荐」，
 * 跨系统优先级口径缺失。本契约钉住统一口径 helper + 四页一致的优先级卡。
 * 锚点 v1 / v3-r3 / r6 不变，禁 --r12-*；不回退 R6/R7/R10 与幂等加载。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

const PAGES = {
  skills: 'pages/skills/skills.vue',
  equipment: 'pages/equipment/equipment.vue',
  gems: 'pages/gems/gems.vue',
  bag: 'pages/bag/bag.vue',
} as const

describe('R12 跨系统成长优先级契约', () => {
  it('锚点与令牌不回退，未新增 r12 视觉令牌', () => {
    const tokens = readSrc('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toMatch(/--r12-/)
    const common = readSrc('styles/common.css')
    expect(common).toContain('LYP-R12-GROWTH-PRIORITY-V1')
    for (const cls of ['r12-priority', 'r12-priority-cta', 'r12-priority-basis']) {
      expect(common, '缺少全局样式 ' + cls).toContain('.' + cls)
    }
    expect(common).toMatch(/prefers-reduced-motion[\s\S]{0,220}animation: none/)
  })

  it('共享 helper 用首页同口径（五维 caps + 权重份额 × 缺口）输出唯一优先级，纯函数不碰 store/game/api 写操作', () => {
    const h = readSrc('pages/_shared/r12GrowthPriority.ts')
    expect(h).toContain('export function computeGrowthPriority')
    for (const cap of ['元素覆盖', '反应链', '专精投入', '装备契合', '机制深度']) {
      expect(h, '缺维度 ' + cap).toContain(cap)
    }
    // 与首页 marginalGain 同口径：权重份额 × 缺口
    expect(h).toMatch(/weightPct[\s\S]{0,120}gap|gap[\s\S]{0,120}weightPct/)
    for (const route of ['/pages/equipment/equipment', '/pages/bag/bag', '/pages/reactions/reactions', '/pages/mastery/mastery']) {
      expect(h, '缺 hint 路由 ' + route).toContain(route)
    }
    expect(h).not.toMatch(/from '@\/game\/(damage|combat|stage|enemy)/)
    expect(h).not.toContain('persistLoadout')
    expect(h).not.toContain('saveEquippedEquipment')
  })

  it('helper 行为：给定 buildRating 与权重返回增益最大的短板维度与可执行 hint；无数据返回 null', async () => {
    const mod = await import('../pages/_shared/r12GrowthPriority').catch(() => null)
    expect(mod, 'helper 不存在').toBeTruthy()
    const compute = mod!.computeGrowthPriority
    const weights = { element_coverage: 1, reaction_coverage: 1, mastery_done: 1, equipment_synergy: 1, mechanic_depth: 1 }
    const top = compute({ element_coverage: 3, reaction_coverage: 7, mastery_done: 0, equipment_synergy: 0, mechanic_depth: 0 }, weights)
    expect(top).not.toBeNull()
    expect(top!.label).toBe('装备契合')
    expect(top!.value).toBe('0/18')
    expect(top!.hint).toBe('/pages/equipment/equipment')
    expect(top!.gain).toBeGreaterThan(0)
    expect(compute(null, weights)).toBeNull()
    expect(compute({}, undefined)).toBeNull()
  })

  for (const [name, path] of Object.entries(PAGES)) {
    it(`${name} 页消费统一优先级 helper 并渲染 r12-priority 卡（AI 判据四要素齐全）`, () => {
      const src = readSrc(path)
      expect(src, name + ' 未 import helper').toContain('computeGrowthPriority')
      expect(src, name + ' 未定义 r12Priority').toMatch(/r12Priority\s*=\s*computed|const r12Priority/)
      expect(src, name + ' 无 r12-priority 卡').toContain('r12-priority')
      expect(src, name + ' 无 AI 徽章').toContain('r12-priority-badge')
      expect(src, name + ' 无结论').toContain('r12-priority-text')
      expect(src, name + ' 无依据').toContain('r12-priority-basis')
      expect(src, name + ' 无 CTA').toContain('r12-priority-cta')
      // 四判据：真实数据结论（pct + weightPct）+ 依据标注
      expect(src, name + ' 未展示短板百分比').toContain('r12Priority.pct')
      expect(src, name + ' 未展示权重').toContain('r12Priority.weightPct')
    })
  }

  it('四页口径一致：都指向同一个短板维度（消费同一个 helper，不再各自为政）', () => {
    for (const [name, path] of Object.entries(PAGES)) {
      const src = readSrc(path)
      expect(src, name + ' 未复用统一口径').toMatch(/r12Priority\.value|r12Priority\.label|r12Priority!.value|r12Priority!.label/)
    }
  })

  it('不回退 R6/R7/R10 标记与幂等加载', () => {
    // R6 推荐卡在 §1E 只覆盖 skills / equipment；gems 与 bag 有各自的 AI 卡，
    // 不能错误要求它们也算 R6 页（那会是契约自身的假断言）。
    const skills = readSrc(PAGES.skills)
    const equip = readSrc(PAGES.equipment)
    expect(skills, 'skills 丢 R6 卡').toMatch(/r6-rec|r6-skeleton/)
    expect(equip, 'equipment 丢 R6 卡').toMatch(/r6-rec|r6-skeleton/)
    expect(skills).toContain('.apply-done')
    expect(equip).toContain('.apply-done')
    const gems = readSrc(PAGES.gems)
    expect(gems).toContain('去背包镶嵌')
    expect(gems).toMatch(/function loadPage|function doLoadPage/)
    const bag = readSrc(PAGES.bag)
    expect(bag).toContain('doUpgrade')
  })
})
