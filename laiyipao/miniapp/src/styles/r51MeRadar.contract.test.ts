// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R51 契约：me 页 AI 诊断口径统一 R12 + 五维雷达可视化。
 * 真实读源码 + 真实 import 权威 helper，禁止内嵌参考实现。
 * 不碰 engine/replay/hash/score/settle/loot/tokens/后端；不新增 --r51-*。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

const ME = 'pages/me/me.vue'
const W_SERVER = { element_coverage: 12, reaction_coverage: 15, mastery_done: 10, equipment_synergy: 12, mechanic_depth: 8 }
const W_EQUAL = { element_coverage: 1, reaction_coverage: 1, mastery_done: 1, equipment_synergy: 1, mechanic_depth: 1 }
// 队长实测的两组真实存档：R4 裸百分比短板分别为 43% 反应链 / 40% 元素覆盖
const REAL_CASES = [
  { name: '案例1', rating: { element_coverage: 5, reaction_coverage: 3, mastery_done: 8, equipment_synergy: 8, mechanic_depth: 8 }, r4Weak: '反应链' },
  { name: '案例2', rating: { element_coverage: 2, reaction_coverage: 3, mastery_done: 8, equipment_synergy: 8, mechanic_depth: 8 }, r4Weak: '元素覆盖' },
]

// 只截取 r4AiMe computed 区块，避免整页误判为已修复
const r4AiMeBody = (src: string): string => {
  const start = src.indexOf('const r4AiMe = computed')
  if (start < 0) return ''
  const end = src.indexOf('function r4GoMe()', start)
  return src.slice(start, end > start ? end : start + 1200)
}

describe('R51 me 页 AI 诊断口径统一 R12（真实源码守卫）', () => {
  it('me.vue 复用权威 helper 且 r4AiMe 不再本地裸百分比排序/本地 hint 三元', () => {
    const src = readSrc(ME)
    expect(src, 'me 未 import computeGrowthPriority').toContain('computeGrowthPriority')
    expect(src, 'me 未 import resolveR12Cta').toContain('resolveR12Cta')
    expect(src, 'me 未从权威 helper 模块 import').toMatch(/from .*r12GrowthPriority/)
    const body = r4AiMeBody(src)
    expect(body, 'r4AiMe 仍用裸百分比升序排序').not.toMatch(/sort\(\(a, b\) => a\.pct - b\.pct\)/)
    expect(body, 'r4AiMe 仍枚举 R4_DIM_CAP 取短板').not.toContain('Object.entries(R4_DIM_CAP)')
    expect(body, 'r4AiMe 未调用 computeGrowthPriority').toContain('computeGrowthPriority(')
    expect(body, 'r4AiMe 未用服务端 rating_weights').toContain('rating_weights')
    expect(body, 'r4AiMe 未调用 resolveR12Cta').toContain('resolveR12Cta(')
    expect(body, 'r4AiMe 仍有本地 mastery hint 三元').not.toContain("weak.key === 'mastery_done'")
    expect(body, 'r4AiMe 仍把元素/机制短板导向商店').not.toContain('/pages/shop/shop')
  })

  it('真实 helper 在两组权重、两组真实存档下短板均为装备契合（R4 旧结论相反）', async () => {
    const mod = await import('../pages/_shared/r12GrowthPriority').catch(() => null)
    expect(mod, '真实 helper 不存在').toBeTruthy()
    const compute = mod!.computeGrowthPriority
    for (const weights of [W_SERVER, W_EQUAL]) {
      for (const c of REAL_CASES) {
        const p = compute(c.rating, weights)
        expect(p, c.name).not.toBeNull()
        expect(p!.label, c.name + ' 权威短板应为装备契合').toBe('装备契合')
        expect(p!.hint, c.name + ' 权威导航应指向装备页').toBe('/pages/equipment/equipment')
        expect(p!.gain, c.name + ' 增益必须为正').toBeGreaterThan(0)
        expect(p!.basis).toContain('权重份额 × 缺口')
      }
    }
    expect(compute(null, W_SERVER)).toBeNull()
    expect(compute(REAL_CASES[0]!.rating, {})).toBeNull()
  })

  it('me.vue 渲染 r51-radar 五维雷达：标记、五轴标签、诚实口径与空态守卫齐全', () => {
    const src = readSrc(ME)
    for (const cls of ['r51-radar', 'r51-radar-ring', 'r51-radar-axis', 'r51-radar-shape', 'r51-radar-label']) {
      expect(src, '缺雷达标记 ' + cls).toContain(cls)
    }
    for (const label of ['元素覆盖', '反应链', '专精投入', '装备契合', '机制深度']) {
      expect(src, '缺五轴标签 ' + label).toContain(label)
    }
    expect(src, '雷达 viewBox 应与 enemies.vue 同风格').toContain('viewBox="0 0 200 180"')
    expect(src, '缺诚实口径文字').toContain('非权威战斗数据；权威以 replay_hash 为准')
    expect(src, '空 rating 必须不渲染雷达').toContain('v-if="store.buildRating"')
  })

  it('令牌三锚点保留且未新增 --r51-', () => {
    const tokens = readSrc('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toMatch(/--r51-/)
    expect(readSrc(ME)).not.toMatch(/--r51-/)
  })
})
