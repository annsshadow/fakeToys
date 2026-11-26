// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R14 AI 建议一键执行闭环 · 三端可测试契约（test-first）
 * ① 服务端 owned 校验：新端点只下发 owned 候选 id/replace，无候选给 code 空态
 * ② 客户端不伪造：query 不含 id/replace，前端不硬编码元素→id 映射
 * ③ 端到端闭环可达：看建议→一键→两步确认→写入→战力变化 + AI 四判据
 * 本测试钉住 SPEC.md 契约文本与既有防作弊红线，SPEC 落盘前 RED。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const specPath = join(__dirname, '../../../../docs/laiyipao/specs/round14-one-tap-apply/SPEC.md')
const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const spec = () => readFileSync(specPath, 'utf-8')

describe('R14 §建议一键执行闭环 三端契约', () => {
  it('服务端：新端点路径/查询白名单/六字段/owned 校验/诚实空态 code 明确', () => {
    const s = spec()
    for (const k of ['/api/v1/me/advice/action', 'advice-action-v1', 'candidate_id', 'replaced_id', 'expected_reaction', 'based_on', 'NO_OWNED_CANDIDATE', 'recOwnedIDs', 'recImpactCTA']) {
      expect(s, 'SPEC 服务端契约缺少 ' + k).toContain(k)
    }
    expect(s, '新端点禁止接收 id/replace 入参').toMatch(/query[^\n]{0,120}(不含|禁止|不接受)[^\n]{0,40}(id|replace)/)
    expect(s, '服务端必须给了 GO owned RED 用例').toContain('TestE2EAdviceActionSkipsUnownedCandidate')
  })

  it('客户端不伪造：不硬编码元素到 id 映射，沿用不伪造替换 id 红线', () => {
    const s = spec()
    for (const k of ['不硬编码', '白名单', '只回传 element', '不得伪造', 'NO_OWNED_CANDIDATE']) {
      expect(s, 'SPEC 防作弊缺少 ' + k).toContain(k)
    }
    const r10 = readSrc('styles/r10GrowthGaps.contract.test.ts')
    expect(r10).toContain('不伪造替换 id')
    const apply = readSrc('pages/_shared/r7ImpactApply.ts')
    expect(apply).toContain('/me/recommendations/impact?kind=')
  })

  it('端到端闭环可达：五步 + E2E 选择器 + R7 复用 + AI 四判据', () => {
    const s = spec()
    for (const k of ['看建议', '一键替换', '两步确认', '写入', '战力变化', '#r14-advice-action-btn', '.r14-advice-cta', '.r14-advice-empty', '[data-r14-source', '.r7-compare-title', '.apply-done', '可见汉字「AI」', '真实数据结论', '依据', 'CTA', 'AI 暂时没有足够数据给出建议']) {
      expect(s, 'SPEC 闭环契约缺少 ' + k).toContain(k)
    }
  })

  it('不回退：令牌锚点不变且不新增 r14 视觉变量', () => {
    const tokens = readSrc('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toMatch(/--r14-/)
  })
})
