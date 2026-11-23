// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R14 H5 消费层流程契约（test-first）：
 * ① 只回传 element/reaction/source，绝不伪造 id；
 * ② 成功/空态归一与防作弊；
 * ③ 两页选择器 + R7 两步确认 + AI 四判据；
 * ④ 不回退 R7/R10 与令牌锚点。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const read = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

describe('R14 H5 建议一键执行闭环流程', () => {
  it('helper 只请求白名单 query，不拼 id/replace，不硬编码元素到 id 映射', () => {
    const h = read('pages/_shared/r14AdviceAction.ts')
    expect(h).toContain('/me/advice/action?element=')
    expect(h).toContain("'&source='")
    expect(h).not.toMatch(/[?&](id|replace|replace_id|candidate_id)=/)
    expect(h).not.toMatch(/fire:\s*\d+|ice:\s*\d+|lightning:\s*\d+/)
    for (const k of ['advice-action-v1', 'NO_OWNED_CANDIDATE', 'based_on', 'expected_reaction', 'applyAdviceAction']) {
      expect(h, 'helper 缺少 ' + k).toContain(k)
    }
  })

  it('normalize 成功态：六字段齐全 + 服务端 impact + 可执行按钮参数', async () => {
    const m = await import('../pages/_shared/r14AdviceAction')
    const a = m.normalizeAdviceAction({
      schema_version: 'advice-action-v1', badge: 'AI', element: 'fire', expected_reaction: 'overheat',
      based_on: ['battle_records.elements_used', '/config(v=7)'], kind: 'skill',
      candidate_id: 12, replaced_id: 4, candidate_name: '焰弹', replaced_name: '通用弹',
      params: { kind: 'skill', id: 12, replace: 4 }, empty_reason: null,
      evidence: [{ metric: '触发率', value: 0.31, unit: '', source: 'battle_records.reactions_used' }],
      impact: { kind: 'skill', candidate_id: 12, replaced_id: 4, candidate_name: '焰弹', replaced_name: '通用弹',
        baseline: { attack_permille: 100, armor_permille: 100, estimated_damage_per_wave: 50, power: 10, rating_total: 60 },
        candidate: { attack_permille: 120, armor_permille: 100, estimated_damage_per_wave: 60, power: 12, rating_total: 70 },
        changes: { attack_permille_delta: 20, armor_permille_delta: 0, damage_per_wave_delta: 10, damage_per_wave_pct: 20,
          power_delta: 2, rating_total_delta: 10, star_rate_observed: 0.3, star_probability_estimate: null },
        method: '真实战报模拟', star_rate_note: '只给历史三星率' }
    })
    expect(a?.status).toBe('ready')
    expect(a?.candidateId).toBe(12)
    expect(a?.replacedId).toBe(4)
    expect(a?.kind).toBe('skill')
    expect(a?.impact?.candidateName).toBe('焰弹')
    expect(a?.basedOn.join(' ')).toContain('battle_records.elements_used')
  })

  it('normalize 空态：id=0 且无按钮参数，给诚实 headline/message/CTA', async () => {
    const m = await import('../pages/_shared/r14AdviceAction')
    const payload: Record<string, unknown> = {
      schema_version: 'advice-action-v1', element: 'fire', expected_reaction: 'overheat',
      based_on: ['battle_records.elements_used'], kind: null, candidate_id: 0, replaced_id: 0, params: null,
      empty_reason: { code: 'NO_OWNED_CANDIDATE', headline: 'AI 暂时没有足够数据给出建议',
        message: '已拥有技能/装备里没有同槽且数值更优的候选', cta: '去打一场战斗 →', route: '/pages/stage/stage' }
    }
    const a = m.normalizeAdviceAction(payload)
    expect(a?.status).toBe('empty')
    expect(a?.empty?.code).toBe('NO_OWNED_CANDIDATE')
    expect(a?.empty?.cta).toContain('去打一场战斗')
    expect(a?.params).toBeNull()
  })

  it('图鉴页 reaction 参数可选：expected_reaction 空串仍归一出 ready/empty，不吞掉权威候选', async () => {
    const m = await import('../pages/_shared/r14AdviceAction')
    const base: Record<string, unknown> = {
      schema_version: 'advice-action-v1', element: 'fire', expected_reaction: '',
      based_on: ['battle_records.elements_used'], kind: 'skill',
      candidate_id: 21, replaced_id: 11, candidate_name: '重炮弹', replaced_name: '燃烧弹',
      params: { kind: 'skill', id: 21, replace: 11 }, empty_reason: null,
    }
    const ready = m.normalizeAdviceAction(structuredClone(base), 'reactions')
    expect(ready?.status).toBe('ready')
    expect(ready?.expectedReaction).toBe('')
    expect(ready?.candidateId).toBe(21)
    const emptyPayload = structuredClone(base)
    emptyPayload.kind = null
    emptyPayload.candidate_id = 0
    emptyPayload.replaced_id = 0
    emptyPayload.params = null
    emptyPayload.empty_reason = { code: 'NO_OWNED_CANDIDATE', headline: '暂无建议',
      message: '没有同元素候选', cta: '去打一场战斗 →', route: '/pages/stage/stage' }
    expect(m.normalizeAdviceAction(emptyPayload, 'reactions')?.status).toBe('empty')
    const v = read('pages/reactions/reactions.vue')
    expect(v).toContain('可提升当前构筑（服务端未给出目标反应）')
  })

  it('防伪造：空态带非零 id / params 与候选不一致 / 版本错误 一律拒绝', async () => {
    const m = await import('../pages/_shared/r14AdviceAction')
    const base: Record<string, unknown> = {
      schema_version: 'advice-action-v1', element: 'fire', expected_reaction: 'overheat',
      based_on: ['battle_records.elements_used'], kind: 'skill',
      candidate_id: 12, replaced_id: 4, params: { kind: 'skill', id: 12, replace: 4 }
    }
    const forgedEmpty = structuredClone(base)
    forgedEmpty.empty_reason = { code: 'NO_OWNED_CANDIDATE', headline: '空', message: '空', cta: '去打一场战斗 →', route: '/pages/stage/stage' }
    expect(m.normalizeAdviceAction(forgedEmpty)).toBeNull()
    const mismatched = structuredClone(base)
    mismatched.params = { kind: 'skill', id: 999, replace: 4 }
    expect(m.normalizeAdviceAction(mismatched)).toBeNull()
    const wrongVersion = structuredClone(base)
    wrongVersion.schema_version = 'rec-v1'
    expect(m.normalizeAdviceAction(wrongVersion)).toBeNull()
  })

  it('battles.vue 保留原建议文案，新增 CTA/空态与 R7 两步浮层', () => {
    const v = read('pages/battles/battles.vue')
    for (const k of ['AI · 反应搭配建议', '{{ reactionLearning }}', '#r14-advice-action-btn', '.r14-advice-cta',
      '.r14-advice-empty', 'data-r14-source="battle"', 'r7-compare-mask', 'r7-compare-title', 'r7-vs-label',
      'r7-btn', 'apply-done', 'r14ConfirmApply()', 'r14OpenCompare()']) {
      expect(v, 'battles.vue 缺少 ' + k).toContain(k)
    }
    expect(v).toContain('function r14OpenCompare()')
    const open = v.slice(v.indexOf('function r14OpenCompare()'), v.indexOf('function r14OpenCompare()') + 320)
    expect(open).not.toMatch(/persistLoadout|saveEquippedEquipment|applyAdviceAction/)
  })

  it('reactions.vue 每条目按权威元素渲染 CTA/空态，不硬编码元素到 id', () => {
    const v = read('pages/reactions/reactions.vue')
    for (const k of ['#r14-advice-action-btn', '.r14-advice-cta', '.r14-advice-empty',
      'data-r14-source="reactions"', 'reactionPairs(r.key)', 'r14AdviceOf(', 'r7-compare-mask',
      'r7-compare-title', 'apply-done', 'r14ConfirmApply()']) {
      expect(v, 'reactions.vue 缺少 ' + k).toContain(k)
    }
    expect(v).not.toMatch(/candidate_id\s*:\s*\d|replaced_id\s*:\s*\d/)
  })

  it('common.css 新增 R14 全局样式层且 reduced-motion 可关', () => {
    const c = read('styles/common.css')
    expect(c).toContain('LYP-R14-ADVICE-ACTION-V1')
    for (const k of ['.r14-advice {', '.r14-advice-badge', '.r14-advice-cta', '.r14-advice-empty',
      '.r14-advice-text', '.r14-advice-basis', '.r14-enter']) {
      expect(c, 'common.css 缺少 ' + k).toContain(k)
    }
    expect(c).toMatch(/@media[\s\S]{0,120}prefers-reduced-motion[\s\S]{0,200}r14-enter[\s\S]{0,80}animation:\s*none/)
  })

  it('不回退令牌锚点、R7 impact helper 与 R10 防伪造红线', () => {
    const t = read('styles/tokens.css')
    for (const k of ['--lyp-token-version: lyp-tokens-v1', '--lyp-r3-version: lyp-tokens-v3-r3', '--lyp-r6-version: lyp-tokens-r6']) {
      expect(t).toContain(k)
    }
    expect(t).not.toMatch(/--r14-/)
    const r7 = read('pages/_shared/r7ImpactApply.ts')
    expect(r7).toContain('/me/recommendations/impact?kind=')
    const r10 = read('styles/r10GrowthGaps.contract.test.ts')
    expect(r10).toContain('不伪造替换 id')
  })
})