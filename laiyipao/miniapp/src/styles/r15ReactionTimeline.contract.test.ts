// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R15 战报反应时间线 · 契约（test-first）
 * ① 字段/schema_version/上限 200 + truncated
 * ② 诚实红线：展示性归因、不进 replay_hash、不改 score/loot/星级、旧战报空态不编造
 * ③ 选择器 .r15-* / [data-r15-wave]
 * ④ 不回退令牌锚点。SPEC 落盘前 RED。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const specPath = join(__dirname, '../../../../docs/laiyipao/specs/round15-reaction-timeline/SPEC.md')
const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const spec = () => readFileSync(specPath, 'utf-8')

describe('R15 §战报反应时间线 契约', () => {
  it('schema_version/可选字段/五字段/上限200与truncated明确', () => {
    const s = spec()
    for (const k of ['reaction-timeline-v1', 'reaction_timeline', 'wave', 'tick', 'reaction', 'enemy_uid', 'damage', 'truncated', '200', 'battle_collections.go', '10MB']) {
      expect(s, 'SPEC 字段契约缺少 ' + k).toContain(k)
    }
    expect(s, '单条五字段必须标注 int/int/string key/int/int').toMatch(/wave[^\n]{0,40}tick[^\n]{0,40}reaction[^\n]{0,40}enemy_uid/)
  })

  it('诚实红线：展示性归因/不进hash/不改score,loot,星级/服务端不当前提/旧战报空态不编造', () => {
    const s = spec()
    for (const k of ['展示性归因', '不参与 replay_hash', '不影响 score', 'loot', '星级', '权威结算输入', '该战报无逐次反应记录（旧版本结算）', '不得编造', '可选字段', 'AllReactionSpecs']) {
      expect(s, 'SPEC 诚实红线缺少 ' + k).toContain(k)
    }
  })

  it('选择器与展示：容器/行/空态/波次属性 + 截断提示', () => {
    const s = spec()
    for (const k of ['.r15-reaction-timeline', '.r15-timeline-row', '.r15-timeline-empty', '[data-r15-wave]', 'truncated', '该战报无逐次反应记录（旧版本结算）']) {
      expect(s, 'SPEC 选择器缺少 ' + k).toContain(k)
    }
    const engine = readSrc('game/engine.ts')
    expect(engine, '引擎必须确有 per-event reaction 记录').toContain("type: 'reaction'")
  })

  it('不回退：令牌锚点不变且不新增 r15 视觉变量', () => {
    const tokens = readSrc('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toMatch(/--r15-/)
  })
})
