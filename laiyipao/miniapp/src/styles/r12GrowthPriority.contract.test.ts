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

// ===== R48：R12 成长优先级 CTA 自导航死循环修复（真实源码守卫，无内嵌参考实现） =====
describe('R48 R12 优先级 CTA 自指防护契约', () => {
  const W = { element_coverage: 1, reaction_coverage: 1, mastery_done: 1, equipment_synergy: 1, mechanic_depth: 1 }
  // 五维等权 → 未补维度 gap 最大者胜出；下面四组 rating 各自只留一个短板
  const RATINGS = {
    element: { element_coverage: 0, reaction_coverage: 7, mastery_done: 8, equipment_synergy: 18, mechanic_depth: 8 },
    mechanic: { element_coverage: 5, reaction_coverage: 7, mastery_done: 8, equipment_synergy: 18, mechanic_depth: 0 },
    equipment: { element_coverage: 5, reaction_coverage: 7, mastery_done: 8, equipment_synergy: 0, mechanic_depth: 8 },
    reaction: { element_coverage: 5, reaction_coverage: 0, mastery_done: 8, equipment_synergy: 18, mechanic_depth: 8 },
  }

  // 从真实源码取 r12Go 函数体，避免把整页误判为已修复
  const goBody = (src: string): string => {
    const i = src.indexOf('function r12Go()')
    if (i < 0) return ''
    return src.slice(i, i + 700)
  }

  // 自指分支子串：从 if (r12Cta.value.self) 花括号配对闭合处截取，
  // 仅覆盖自指 block，不含后续非自指 navigateTo —— 与整段 goBody 口径区分。
  const selfBranch = (body: string): string => {
    const marker = 'if (r12Cta.value.self)'
    const i = body.indexOf(marker)
    if (i < 0) return ''
    const open = body.indexOf('{', i)
    if (open < 0) return body.slice(i)
    let depth = 0
    for (let k = open; k < body.length; k++) {
      if (body[k] === '{') depth++
      else if (body[k] === '}') {
        depth--
        if (depth === 0) return body.slice(i, k + 1)
      }
    }
    return body.slice(i)
  }

  it('R48-A helper 行为：真实导入 resolveR12Cta，自指时 self=true 且文案改为页内指引；非自指逐字保留 hintLabel；null 空态安全', async () => {
    const mod = await import('../pages/_shared/r12GrowthPriority').catch(() => null)
    expect(mod, '真实 helper 不存在').toBeTruthy()
    const compute = mod!.computeGrowthPriority
    const resolve = mod!.resolveR12Cta
    expect(typeof compute, 'computeGrowthPriority 必须仍导出').toBe('function')
    expect(typeof resolve, 'R48 必须在 helper 导出 resolveR12Cta 消费层纯函数').toBe('function')

    // 缺口来源仍在：三条 hint 指向页面自身路由（算法不改）
    const elem = compute(RATINGS.element, W)
    expect(elem!.label).toBe('元素覆盖')
    expect(elem!.hint).toBe('/pages/bag/bag')
    const mech = compute(RATINGS.mechanic, W)
    expect(mech!.label).toBe('机制深度')
    expect(mech!.hint).toBe('/pages/bag/bag')
    const eq = compute(RATINGS.equipment, W)
    expect(eq!.label).toBe('装备契合')
    expect(eq!.hint).toBe('/pages/equipment/equipment')
    const react = compute(RATINGS.reaction, W)
    expect(react!.label).toBe('反应链')
    expect(react!.hint).toBe('/pages/reactions/reactions')

    // 自指三例：self=true，label 为页内指引口径
    const eView = resolve(elem, '/pages/bag/bag')
    expect(eView.self).toBe(true)
    expect(eView.label).toBe('在本页强化元素覆盖')
    const mView = resolve(mech, '/pages/bag/bag')
    expect(mView.self).toBe(true)
    expect(mView.label).toBe('在本页强化机制深度')
    const qView = resolve(eq, '/pages/equipment/equipment')
    expect(qView.self).toBe(true)
    expect(qView.label).toBe('在本页强化装备契合')

    // 非自指跨页：self=false，label 逐字等于原 hintLabel
    const cross = resolve(react, '/pages/bag/bag')
    expect(cross.self).toBe(false)
    expect(cross.label).toBe(react!.hintLabel)
    expect(cross.label).toBe('看反应搭配')

    // 防御性一致：目标路由等于当前路由也算自指
    const selfReact = resolve(react, '/pages/reactions/reactions')
    expect(selfReact.self).toBe(true)
    expect(selfReact.label).toBe('在本页强化反应链')

    // 空态安全
    const empty = resolve(null, '/pages/bag/bag')
    expect(empty.self).toBe(false)
    expect(empty.label).toBe('')
  })

  it('R48-B bag 页：元素覆盖/机制深度自指时不 navigateTo 自身，改走页内切页签动作；CTA 用 r12Cta.label', () => {
    const src = readSrc(PAGES.bag)
    expect(src, 'bag 未导入 resolveR12Cta').toContain('resolveR12Cta')
    expect(src, 'bag 未声明自身路由常量').toContain('/pages/bag/bag')
    expect(src, 'bag 未派生 r12Cta').toMatch(/r12Cta\s*=\s*computed/)
    expect(src, 'bag CTA 仍绑定 r12Priority.hintLabel').not.toContain('{{ r12Priority.hintLabel }}')
    expect(src, 'bag CTA 未用 r12Cta.label').toContain('r12Cta.label')
    const body = goBody(src)
    expect(body, 'bag r12Go 未识别自指').toContain('r12Cta.value.self')
    expect(body, 'bag 丢失非自指 navigateTo 跨页导航').toContain('navigateTo')
    const self = selfBranch(body)
    expect(self, 'bag 自指分支未提前返回').toContain('return')
    expect(self, 'bag 自指分支内仍 navigateTo 自身').not.toContain('navigateTo')
    expect(self, 'bag 自指未做页内切页签动作').toContain('bagTab.value =')
  })

  it('R48-C equipment 页：装备契合自指时不 navigateTo 自身，改走页内定位装备列表；CTA 用 r12Cta.label', () => {
    const src = readSrc(PAGES.equipment)
    expect(src, 'equipment 未导入 resolveR12Cta').toContain('resolveR12Cta')
    expect(src, 'equipment 未声明自身路由常量').toContain('/pages/equipment/equipment')
    expect(src, 'equipment 未派生 r12Cta').toMatch(/r12Cta\s*=\s*computed/)
    expect(src, 'equipment CTA 仍绑定 r12Priority.hintLabel').not.toContain('{{ r12Priority.hintLabel }}')
    expect(src, 'equipment CTA 未用 r12Cta.label').toContain('r12Cta.label')
    const body = goBody(src)
    expect(body, 'equipment r12Go 未识别自指').toContain('r12Cta.value.self')
    expect(body, 'equipment 丢失非自指 navigateTo 跨页导航').toContain('navigateTo')
    const self = selfBranch(body)
    expect(self, 'equipment 自指分支未提前返回').toContain('return')
    expect(self, 'equipment 自指分支内仍 navigateTo 自身').not.toContain('navigateTo')
    expect(self, 'equipment 自指未做页内定位动作').toMatch(/r3Slot\.value\s*=|r12FocusInPage|scrollIntoView/)
  })

  it('R48-D 四页自指防护齐备，且非自指仍保留 navigateTo 跨页导航', () => {
    for (const [name, path] of Object.entries(PAGES)) {
      const src = readSrc(path)
      expect(src, name + ' 未导入 resolveR12Cta').toContain('resolveR12Cta')
      expect(src, name + ' 未派生 r12Cta').toMatch(/r12Cta\s*=\s*computed/)
      const body = goBody(src)
      expect(body, name + ' r12Go 未识别自指').toContain('r12Cta.value.self')
      expect(body, name + ' 丢失非自指 navigateTo 跨页导航').toContain('navigateTo')
      expect(body, name + ' 自指分支未提前 return').toContain('return')
    }
  })

  it('R48-E 不回退：DIM_HINT 三自指路由、令牌三锚点、首页副本不动、无 r47/r48 令牌', () => {
    const h = readSrc('pages/_shared/r12GrowthPriority.ts')
    expect(h, 'helper 丢了元素覆盖自指路由').toContain('/pages/bag/bag')
    expect(h, 'helper 丢了装备契合自指路由').toContain('/pages/equipment/equipment')
    expect(h, 'helper 丢了跨页导航路由').toContain('/pages/reactions/reactions')
    expect(h, 'computeGrowthPriority 被删').toContain('export function computeGrowthPriority')
    const tokens = readSrc('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toMatch(/--r47-/ )
    expect(tokens).not.toMatch(/--r48-/ )
    const index = readSrc('pages/index/index.vue')
    expect(index, '首页副本不得改动').not.toContain('resolveR12Cta')
  })
})
