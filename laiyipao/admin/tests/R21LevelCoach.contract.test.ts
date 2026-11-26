import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const specPath = join(__dirname, '../../../docs/laiyipao/specs/round21-player-level-coach/SPEC.md')
const srcRoot = join(__dirname, '..')
const readSpec = () => readFileSync(specPath, 'utf-8')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

describe('R21 level-coach 契约守卫（admin/backend）：字段契约', () => {
  it('SPEC 钉死 me 端点、鉴权隔离与 30 天窗口', () => {
    const spec = readSpec()
    expect(spec).toContain('GET /api/v1/me/level-coach')
    for (const s of ['严格 WHERE user_id', 'window_days=30', '两个用户互不串数据']) {
      expect(spec, `缺条款 ${s}`).toContain(s)
    }
  })
  it('SPEC 钉死八字段与连续失败阈值口径', () => {
    const spec = readSpec()
    for (const k of ['level_id', 'attempts', 'wins', 'best_wave', 'avg_wave', 'last_result', 'stuck', 'coach']) {
      expect(spec, `缺字段 ${k}`).toContain(k)
    }
    expect(spec).toContain('domain.StuckThreshold')
    expect(spec).toContain('遇 win 即停')
  })
})

describe('R21 选择器与诚实红线（admin）', () => {
  it('SPEC 钉死选择器、逐字空态与诚实红线', () => {
    const spec = readSpec()
    for (const s of ['.r21-level-coach', '.r21-coach-row', '[data-r21-level]', '.r21-coach-empty', '.r21-ai-note', '近 30 天无关卡尝试记录', '辅助诊断非权威结算依据，权威看 replay_hash', '零真值']) {
      expect(spec, `缺条款 ${s}`).toContain(s)
    }
  })
  it('SPEC 钉死 AI 四判据', () => {
    const spec = readSpec()
    for (const s of ['可见汉字「AI」', '真实数据结论', '依据行', '可执行 CTA']) {
      expect(spec, `缺 AI 判据 ${s}`).toContain(s)
    }
  })
})

describe('R21 三端不回退基线（admin/backend）', () => {
  it('battle.go: StuckThreshold=2 与 Diagnose 字段保留', () => {
    const s = readSrc('../server/internal/domain/battle.go')
    expect(s).toContain('StuckThreshold = 2')
    expect(s).toContain('FailedTimes')
    expect(s).toContain('Stuck bool')
  })
  it('progression.go: 连续失败口径保留', () => {
    const s = readSrc('../server/internal/service/progression.go')
    expect(s).toContain('countConsecutiveLosses')
    expect(s).toContain('domain.StuckThreshold')
  })
  it('routes.go / migration: /stars 与 battle_records 列保留', () => {
    const r = readSrc('../server/internal/httpapi/routes.go')
    expect(r).toContain('me.Get("/stars"')
    const m = readSrc('../server/migrations/00004_battle.sql')
    for (const k of ['level_id', 'result', 'wave_reached', 'created_at', 'user_id']) {
      expect(m, `migration 缺列 ${k}`).toContain(k)
    }
  })
  it('tokens: v1/v3-r3/r6 锚点保留且无 r21 新增令牌', () => {
    const s = readSrc('src/styles/tokens.css')
    expect(s).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(s).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(s).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(s).not.toContain('--r21-')
  })
})