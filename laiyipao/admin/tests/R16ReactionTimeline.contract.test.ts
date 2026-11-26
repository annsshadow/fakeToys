// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R16 · admin 战报详情逐次反应时间线 · 契约（test-first，规范/基线守卫）
 * ① 契约主干（schema/五字段/200/truncated）
 * ② 选择器 .r16-* / [data-r16-wave] / labelOfReaction
 * ③ 排序 / 空态 / 截断 / 诚实红线
 * ④ AI 四判据
 * ⑤ 三端不回退（后端字段 / H5 r15 选择器 / admin 令牌锚点）
 * 本轮只交付规范与契约测试，不改业务代码。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const specPath = join(__dirname, '../../../docs/laiyipao/specs/round16-admin-reaction-timeline/SPEC.md')
const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const spec = () => readFileSync(specPath, 'utf-8')

describe('R16 §admin 逐次反应时间线 契约', () => {
  it('契约主干：schema_version/可选字段/五字段/上限200与truncated明确', () => {
    const s = spec()
    for (const k of ['reaction-timeline-v1', 'reaction_timeline', 'wave', 'tick', 'reaction', 'enemy_uid', 'damage', 'truncated', '200', 'schema_version']) {
      expect(s, 'SPEC 契约主干缺少 ' + k).toContain(k)
    }
    expect(s, '五字段必须标注顺序').toMatch(/wave[^\n]{0,60}tick[^\n]{0,60}reaction[^\n]{0,60}enemy_uid[^\n]{0,60}damage/s)
  })

  it('选择器：容器/行/波次属性/空态/截断态 + 服务端取名，且区分 r15', () => {
    const s = spec()
    for (const k of ['.r16-reaction-timeline', '.r16-timeline-row', '[data-r16-wave]', '.r16-timeline-empty', '.r16-truncated-note', 'labelOfReaction']) {
      expect(s, 'SPEC 选择器缺少 ' + k).toContain(k)
    }
    expect(s, '必须明确与 H5 r15 选择器区分').toContain('.r15-reaction-timeline')
    expect(s, '必须禁止硬编码反应中文名').toContain('不得硬编码')
  })

  it('排序 / 空态 / 截断 / 诚实红线逐字', () => {
    const s = spec()
    expect(s).toContain('wave/tick 升序')
    for (const k of [
      '该战报无逐次反应记录（旧版本结算）',
      '已截断，仅显示前 200 条',
      '展示性归因', '非权威结算', '反推编造', '旧战报空态',
      'replay_hash', 'labelOfReaction', '不裸插值长浮点',
    ]) {
      expect(s, 'SPEC 诚实红线缺少 ' + k).toContain(k)
    }
  })

  it('AI 四判据：可见汉字 AI + 真实数据结论 + 依据行 + 可执行语义', () => {
    const s = spec()
    for (const k of ['AI', '真实数据结论', '依据行', '可执行']) {
      expect(s, 'SPEC AI 四判据缺少 ' + k).toContain(k)
    }
  })

  it('三端不回退：后端字段/H5 r15 选择器/admin 令牌锚点', () => {
    const types = readSrc('src/api/types.gen.ts')
    expect(types).toContain('reaction_timeline: ReactionTimeline')
    expect(types).toContain('export interface ReactionTimelineEvent')
    const evBlock = types.slice(types.indexOf('export interface ReactionTimelineEvent'))
    for (const f of ['wave', 'tick', 'reaction', 'enemy_uid', 'damage']) {
      expect(evBlock, 'ReactionTimelineEvent 五字段缺 ' + f).toContain(f)
    }
    const go = readFileSync(join(srcRoot, '../server/internal/service/adminbattledetail.go'), 'utf-8')
    expect(go).toContain('json:"reaction_timeline"')
    expect(go).toContain('非权威结算数据')
    const h5 = readFileSync(join(srcRoot, '../miniapp/src/pages/battles/battles.vue'), 'utf-8')
    expect(h5).toContain('.r15-reaction-timeline')
    expect(h5).toContain('该战报无逐次反应记录（旧版本结算）')
    const bv = readSrc('src/views/BattlesView.vue')
    expect(bv).not.toContain('.r15-reaction-timeline')
    expect(bv).toContain('labelOfReaction')
    const tokens = readSrc('src/styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toMatch(/--r16-/)
  })
})
