// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R17 · 验真页逐次反应序列对照 · 契约（test-first，规范/基线守卫）
 * ① 契约主干（schema/五字段/上限/truncated）
 * ② 选择器 .r17-* / [data-r17-index] / 逐字空态与说明
 * ③ diff 算法口径（五字段全等/长度差/0 基序号）
 * ④ 诚实红线（不参与也不覆盖 replay_hash / 辅助线索非权威 / 绝不冒充）
 * ⑤ AI 四判据与三端不回退（verify/replay/engine/tokens）
 * 本轮只交付规范与契约测试，不改业务代码。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const specPath = join(__dirname, '../../../../docs/laiyipao/specs/round17-verify-reaction-diff/SPEC.md')
const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const spec = () => readFileSync(specPath, 'utf-8')

describe('R17 §验真页逐次反应序列对照 契约', () => {
  it('契约主干：schema_version/可选字段/五字段/上限200与truncated明确', () => {
    const s = spec()
    for (const k of ['reaction-timeline-v1', 'reaction_timeline', 'wave', 'tick', 'reaction', 'enemy_uid', 'damage', 'truncated', '200', 'schema_version']) {
      expect(s, 'SPEC 契约主干缺少 ' + k).toContain(k)
    }
    expect(s, '五字段必须标注顺序').toMatch(/wave[^\n]{0,60}tick[^\n]{0,60}reaction[^\n]{0,60}enemy_uid[^\n]{0,60}damage/s)
  })

  it('选择器与文案：容器/摘要/行/首差/空态/说明 + 逐字空态，且与 r15 区分', () => {
    const s = spec()
    for (const k of ['.r17-reaction-diff', '.r17-diff-summary', '.r17-diff-row', '[data-r17-index]', '.r17-first-diff', '.r17-diff-empty', '.r17-diff-note']) {
      expect(s, 'SPEC 选择器缺少 ' + k).toContain(k)
    }
    expect(s).toContain('该战报无逐次反应记录，暂无法对照（旧版本结算）')
    expect(s).toContain('辅助对照线索，非验真判据；权威裁定仍以 replay_hash 为准')
    expect(s, '必须标注与 H5 r15 选择器区分').toContain('.r15-')
    expect(s, '必须禁止硬编码反应中文名').toContain('不硬编码中文名 map')
  })

  it('diff 算法口径：五字段全等/长度差/0 基序号/截断提示', () => {
    const s = spec()
    for (const k of ['abs(lenA-lenB)', 'min(本地长, 服务端长)', 'i+1', '0 基', 'wave/tick/reaction/enemy_uid/damage', '已截断，仅显示前 200 条', 'toFixed(1)']) {
      expect(s, 'SPEC diff 算法缺少 ' + k).toContain(k)
    }
  })

  it('诚实红线：不参与也不覆盖 replay_hash / 辅助线索非权威 / 绝不冒充 / 反推编造', () => {
    const s = spec()
    for (const k of [
      '不参与也不覆盖 replay_hash', '辅助对照线索', '非权威验真判据',
      '权威裁定仍是', 'replay_hash matched', '绝不冒充验真通过/失败',
      '反推编造', '旧记录诚实空态', '零真值',
    ]) {
      expect(s, 'SPEC 诚实红线缺少 ' + k).toContain(k)
    }
  })

  it('AI 四判据与三端不回退：verify/replay/engine/tokens 基线不破坏', () => {
    const s = spec()
    for (const k of ['AI', '真实数据结论', '依据行', '可执行']) {
      expect(s, 'SPEC AI 四判据缺少 ' + k).toContain(k)
    }
    const verify = readSrc('pages/verify/verify.vue')
    expect(verify).toContain('outcome.matched')
    expect(verify).toContain('result.matched')
    expect(verify).toContain('验真通过')
    expect(verify).toContain('验真失败')
    const replay = readSrc('game/replay.ts')
    expect(replay).toContain('engine.replayHash()')
    expect(replay).toContain('matched: computed === info.expected_hash')
    const engine = readSrc('game/engine.ts')
    expect(engine).toContain('reactionTimeline()')
    expect(engine).toContain('enemy_uid')
    const tokens = readSrc('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toMatch(/--r17-/)
  })
})
