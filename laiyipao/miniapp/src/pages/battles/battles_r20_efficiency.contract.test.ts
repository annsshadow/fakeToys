import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

// 测试位置：laiyipao/miniapp/src/pages/battles/；上 5 级到 worktree 根（docs 所在）
const specPath = join(__dirname, '../../../../../docs/laiyipao/specs/round20-combat-efficiency-trends/SPEC.md')
// srcRoot = laiyipao/miniapp/src
const srcRoot = join(__dirname, '../..')
const readSpec = () => readFileSync(specPath, 'utf-8')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

describe('R20 battle-efficiency 契约守卫（miniapp）：字段与选择器', () => {
  it('SPEC 钉死 me 端点与响应字段', () => {
    const spec = readSpec()
    expect(spec).toContain('GET /api/v1/me/battle-efficiency?window_days=30')
    for (const k of ['window_days', 'window_from', 'window_to', 'battles', 'shots', 'hits', 'accuracy_pct', 'avg_heat_max', 'leaked_per_battle', 'avg_duration_ms', 'buckets', 'ai_insight']) {
      expect(spec, `缺少响应字段 ${k}`).toContain(k)
    }
  })
  it('SPEC 钉死 r20 选择器与逐字空态', () => {
    const spec = readSpec()
    for (const sel of ['.r20-efficiency-trend', '.r20-efficiency-row', '.r20-efficiency-empty', '.r20-ai-note', '.r20-trend-bar[data-r20-bucket]']) {
      expect(spec, `缺选择器 ${sel}`).toContain(sel)
    }
    expect(spec).toContain('近 30 天无战斗记录')
  })
})

describe('R20 诚实红线与 AI 四判据（miniapp）', () => {
  it('SPEC 钉死辅助非权威 / 隔离 / 零真值与 AI 四要素', () => {
    const spec = readSpec()
    for (const s of ['辅助诊断非权威', '不参与或覆盖 replay_hash', '严格按 user_id 过滤', '零真值', '可见汉字「AI」', '真实数据结论', '依据行', '可执行 CTA']) {
      expect(spec, `缺条款 ${s}`).toContain(s)
    }
  })
})

describe('R20 三端不回退基线（miniapp）', () => {
  it('battles.vue: G132 efficiencyStats 本地卡保留', () => {
    const s = readSrc('pages/battles/battles.vue')
    expect(s).toContain('efficiencyStats')
    expect(s).toContain('Object.values(details.value)')
  })
  it('tokens: v1/v3-r3/r6 锚点保留且无 r20 新增令牌', () => {
    const s = readSrc('styles/tokens.css')
    expect(s).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(s).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(s).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(s).not.toContain('--r20-')
  })
})