import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const specPath = join(__dirname, '../../../docs/laiyipao/specs/round20-combat-efficiency-trends/SPEC.md')
const srcRoot = join(__dirname, '..')
const readSpec = () => readFileSync(specPath, 'utf-8')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

describe('R20 battle-efficiency 契约守卫：字段契约', () => {
  it('SPEC 钉死双端点与窗口字段', () => {
    const spec = readSpec()
    expect(spec).toContain('GET /api/v1/me/battle-efficiency?window_days=30')
    expect(spec).toContain('GET /api/v1/admin/battle-efficiency')
    for (const k of ['window_days', 'window_from', 'window_to', 'battles', 'shots', 'hits', 'accuracy_pct', 'avg_heat_max', 'leaked_per_battle', 'avg_duration_ms', 'buckets', 'ai_insight']) {
      expect(spec, `缺少响应字段 ${k}`).toContain(k)
    }
    expect(spec).toContain('window_days=30')
  })
  it('SPEC 钉死 buckets 单行与权威 count / 归零口径', () => {
    const spec = readSpec()
    expect(spec).toContain('{bucket, battles, shots, hits, accuracy_pct}')
    expect(spec).toContain('battles 权威就是 battle_records 行数')
    expect(spec).toContain('accuracy = hits/shots')
    expect(spec).toContain('分母为 0 时归零')
    expect(spec).toContain('buckets=[]')
    expect(spec).toContain('两个用户互不串数据')
  })
})

describe('R20 选择器与逐字文案', () => {
  it('SPEC 钉死全部 r20 选择器且不含 data-r20-reaction', () => {
    const spec = readSpec()
    for (const sel of ['.r20-efficiency-trend', '.r20-efficiency-row', '.r20-efficiency-empty', '.r20-ai-note', '.r20-trend-bar[data-r20-bucket]']) {
      expect(spec, `缺选择器 ${sel}`).toContain(sel)
    }
    expect(spec).toContain('[data-r20-reaction] 不需要')
  })
  it('SPEC 钉死逐字空态/依据行/窗口文案', () => {
    const spec = readSpec()
    expect(spec).toContain('近 30 天无战斗记录')
    expect(spec).toContain('依据：近 30 天 battle_records 的 shots/hits/heat_max 聚合；场次以 battle_records 行数为准')
    expect(spec).toContain('window_from / window_to')
  })
})

describe('R20 诚实红线', () => {
  it('SPEC 钉死辅助非权威/空态诚实/隔离/零真值', () => {
    const spec = readSpec()
    for (const s of ['辅助诊断非权威', '不参与或覆盖 replay_hash', '不依赖 details 缓存、不依赖 reaction_timeline', '严格按 user_id 过滤', '零真值']) {
      expect(spec, `缺红线 ${s}`).toContain(s)
    }
  })
})

describe('R20 AI 四判据', () => {
  it('SPEC 钉死可见 AI / 真实数据结论 / 依据行 / 可执行 CTA', () => {
    const spec = readSpec()
    for (const s of ['可见汉字「AI」', '真实数据结论', '依据行', '可执行 CTA', 'AI · 战斗效率诊断']) {
      expect(spec, `缺 AI 判据 ${s}`).toContain(s)
    }
  })
})

describe('R20 三端不回退基线（admin）', () => {
  it('admin: R18 伤害贡献端点 / Dashboard 次数图基线保留', () => {
    const s = readSrc('src/api/index.ts')
    expect(s).toContain("api.get<ReactionDamageImpactResp>('/admin/reaction-damage-impact')")
    expect(s).toContain("api.get<Dashboard>('/admin/dashboard')")
  })
  it('tokens: v1/v3-r3/r6 锚点保留且无 r20 新增令牌', () => {
    const s = readSrc('src/styles/tokens.css')
    expect(s).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(s).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(s).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(s).not.toContain('--r20-')
  })
})