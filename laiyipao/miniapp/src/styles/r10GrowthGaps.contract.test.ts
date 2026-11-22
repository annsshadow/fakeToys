// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R10 §1I 因果链缺口契约：
 *  ① 战报/结算页养成前后同一关可信差值（只认两次真实结算 + hash）
 *  ② 宝石佩戴收益来自服务端属性差分，未上报写诚实空态
 *  ③ 推荐收益无候选时消费 insights[].basis 解释「为什么收益不可用」
 * 先 RED 再实现；锚点 v1 / v3-r3 / r6 不变，禁 --r10-*。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const tpl = (s: string) => s.slice(0, s.lastIndexOf('</template>'))

const PAGES = {
  skills: 'pages/skills/skills.vue',
  equipment: 'pages/equipment/equipment.vue',
  gems: 'pages/gems/gems.vue',
  bag: 'pages/bag/bag.vue',
  battles: 'pages/battles/battles.vue',
  battle: 'pages/battle/battle.vue',
} as const

describe('R10 §1I 缺口实现契约', () => {
  it('锚点与令牌不回退，且未新增 r10 视觉令牌', () => {
    const tokens = readSrc('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toMatch(/--r10-/)
    const common = readSrc('styles/common.css')
    expect(common).toContain('LYP-R10-CAUSAL-V1')
    for (const cls of ['r10-growth', 'r10-gem-gain', 'r10-basis-note']) {
      expect(common, '缺少全局样式 ' + cls).toContain('.' + cls)
    }
    expect(common).toMatch(/prefers-reduced-motion[\s\S]{0,200}animation: none/)
  })

  it('共享 helper 从真实战报字段构造养成前后视图，缺一次结算返回诚实空态', async () => {
    const helper = readSrc('pages/_shared/r10GrowthCompare.ts')
    expect(helper).toContain("from '@/api/client'")
    expect(helper).toMatch(/myBattleDetail|BattleDetail/)
    expect(helper).toContain('replay_hash')
    expect(helper).not.toMatch(/19726|10909|11941/)
    const mod = await import('@/pages/_shared/r10GrowthCompare')
    const emptyRow = mod.normalizeGrowthCompare([])
    expect(emptyRow.available).toBe(false)
    expect(emptyRow.text).toContain('还缺养成前那局战报，先打一局同关')
    const before = { id: 1, level_id: 9, result: 'won', stars: 2, score: 10614, kills: 40, leaked: 5, reactions: 92, hp_left: 10, wave_reached: 8, duration_ms: 1000, shots: 50, hits: 30, heat_max: 80, elements_used: {}, reactions_used: {}, terrain_used: [], loot: {}, replay_hash: '501a22c7', created_at: '2026-11-22T08:00:00Z' }
    const after = { ...before, id: 2, score: 12088, stars: 3, kills: 45, leaked: 3, reactions: 93, replay_hash: '1c36fcc2', created_at: '2026-11-22T09:00:00Z' }
    const ok = mod.normalizeGrowthCompare([after, before])
    expect(ok.available).toBe(true)
    expect(ok.levelId).toBe(9)
    expect(ok.rows.some((r: any) => r.label.includes('分数') && r.before === 10614 && r.after === 12088)).toBe(true)
    expect(ok.rows.some((r: any) => r.label.includes('反应') && r.delta === 1)).toBe(true)
    expect(ok.hashes).toContain('501a22c7')
    expect(ok.hashes).toContain('1c36fcc2')
    expect(ok.text).toContain('同一关')
  })

  it('战报详情展开区渲染可信差值卡与两次 hash，缺数据时为诚实空态', () => {
    const s = tpl(readSrc(PAGES.battles))
    for (const m of ['r10-growth', 'r10-growth-title', 'r10-growth-row', 'growthCompare', 'growthHashes', '还缺养成前那局战报，先打一局同关', '构筑变更点未上报']) {
      expect(s, '战报模板缺少 ' + m).toContain(m)
    }
    const full = readSrc(PAGES.battles)
    expect(full, '战报脚本缺少 loadGrowthCompare').toContain('loadGrowthCompare')
    expect(full).toMatch(/myBattleHistory|fetchGrowthCompareForDetail/)
  })

  it('战斗结算面板渲染同一关可信差值，未入账结算不伪造养成结论', () => {
    const s = tpl(readSrc(PAGES.battle))
    for (const m of ['r10-growth', 'growthCompare', 'settleFailed', '还缺养成前那局战报，先打一局同关']) {
      expect(s, '结算模板缺少 ' + m).toContain(m)
    }
    expect(readSrc(PAGES.battle), '结算脚本缺少 loadGrowthCompare').toContain('loadGrowthCompare')
  })

  it('宝石佩戴收益 helper 只做权威属性差分，未上报用诚实文案', async () => {
    const helper = readSrc('pages/_shared/r10GemGain.ts')
    for (const m of ['fetchMe', 'attack', 'crit_permille', 'reaction_mult_permille', 'element_coef_permille', 'heat_cap_permille']) {
      expect(helper, '差分 helper 缺少 ' + m).toContain(m)
    }
    const mod = await import('@/pages/_shared/r10GemGain')
    const none = mod.normalizeGemGain(null)
    expect(none.available).toBe(false)
    expect(none.text).toContain('佩戴收益未上报，先看词条描述')
    const diff = mod.normalizeGemGain({
      before: { user_id: 3, build: { attacker: { attack: 1000, crit_permille: 100, reaction_mult_permille: 1000, element_coef_permille: 1000, heat_cap_permille: 1000 } } },
      after: { user_id: 3, build: { attacker: { attack: 1050, crit_permille: 110, reaction_mult_permille: 1000, element_coef_permille: 1000, heat_cap_permille: 1000 } } },
    })
    expect(diff.available).toBe(true)
    expect(diff.rows.some((r: any) => r.key === 'attack' && r.delta === 50)).toBe(true)
    expect(diff.text).toContain('服务端属性差分')
  })

  it('背包佩戴/卸下真实写后采集两次 /me 权威属性差分并缓存，不写真值', () => {
    const s = readSrc(PAGES.bag)
    expect(s).toContain('captureGemGain')
    expect((s.match(/captureGemGain\(/g) ?? []).length).toBeGreaterThanOrEqual(2)
    expect(s).toContain('r10GemGainCacheKey')
    expect(s).toContain('localStorage')
    expect(s).toContain('user_id')
    expect(s).toContain('store.refreshProfile()')
    expect(s).toMatch(/await api\.equipGem\(g\.id\)/)
    expect(s).toMatch(/await api\.unequipGem\(g\.id\)/)
  })

  it('宝石图鉴展示佩戴收益卡或诚实空态，保留去背包镶嵌双 CTA', () => {
    const s = tpl(readSrc(PAGES.gems))
    for (const m of ['r10-gem-gain', 'gemGain', '佩戴收益未上报，先看词条描述', '去背包镶嵌 →', '去商城找宝石 →', 'r10GemGainCacheKey', '服务端属性差分']) {
      expect(s, '宝石页缺少 ' + m).toContain(m)
    }
  })

  it('技能/装备无候选时错误位展示 insights[].basis 真实依据，不编造参数', () => {
    for (const page of ['skills', 'equipment'] as const) {
      const s = readSrc(PAGES[page])
      expect(s, page + ' 未消费服务端 why').toContain('r7WhyUnavailable')
      expect(s, page + ' 未用 basis 解释').toContain('rec.basis')
      expect(s, page + ' 未展示依据').toContain('收益数据不可用：{{ r7WhyUnavailable')
      expect(s, page + ' 不应保留干瘪默认').not.toContain("r7Error || '接口未提供真实预估'")
      expect(s, page + ' 不伪造候选 id').not.toMatch(/candidateId\s*=\s*\d+/)
      expect(s, page + ' 不伪造替换 id').not.toMatch(/replacedId\s*=\s*\d+/)
    }
  })

  it('不回退 R6/R7/R5/R4 既有契约锚点', () => {
    for (const page of ['skills', 'equipment'] as const) {
      const s = readSrc(PAGES[page])
      for (const marker of ['r6-rec', 'r6-badge', 'r6-headline', 'r6-evidence', 'r6-cta', 'r7-impact', 'r7-compare', 'r7-btn', 'r7-causal', '.apply-done']) {
        expect(s, page + ' 缺少 ' + marker).toContain(marker)
      }
    }
    const gems = readSrc(PAGES.gems)
    for (const marker of ['r3-ai', 'r7-causal', 'r3GoShop', 'r3GoBag']) {
      expect(gems, '宝石页缺少 ' + marker).toContain(marker)
    }
  })
})
