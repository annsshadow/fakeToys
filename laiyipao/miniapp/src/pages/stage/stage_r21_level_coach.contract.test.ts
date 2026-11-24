import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

// 测试位置：laiyipao/miniapp/src/pages/stage/；上 5 级到 worktree 根（docs 所在）
const specPath = join(__dirname, '../../../../../docs/laiyipao/specs/round21-player-level-coach/SPEC.md')
const srcRoot = join(__dirname, '../..')
const readSpec = () => readFileSync(specPath, 'utf-8')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

describe('R21 level-coach 契约守卫（miniapp）：字段与选择器', () => {
  it('SPEC 钉死 me 端点与响应八字段', () => {
    const spec = readSpec()
    expect(spec).toContain('GET /api/v1/me/level-coach')
    for (const k of ['level_id', 'attempts', 'wins', 'best_wave', 'avg_wave', 'last_result', 'stuck', 'coach', 'items', 'window_days']) {
      expect(spec, `缺少响应字段 ${k}`).toContain(k)
    }
    expect(spec).toContain('{level_id,attempts,wins,best_wave,avg_wave,last_result,stuck,coach}')
  })
  it('SPEC 钉死 r21 选择器与逐字空态', () => {
    const spec = readSpec()
    for (const sel of ['.r21-level-coach', '.r21-coach-row', '[data-r21-level]', '.r21-coach-empty', '.r21-ai-note']) {
      expect(spec, `缺选择器 ${sel}`).toContain(sel)
    }
    expect(spec).toContain('近 30 天无关卡尝试记录')
  })
})

describe('R21 诚实红线与 AI 四判据（miniapp）', () => {
  it('SPEC 钉死辅助非权威 / 隔离 / 分母归零 / 零真值与 AI 四要素', () => {
    const spec = readSpec()
    for (const s of ['辅助诊断非权威结算依据，权威看 replay_hash', '严格 WHERE user_id', '分母为 0 时归零，不产生 NaN', '零真值', '可见汉字「AI」', '真实数据结论', '依据行', '可执行 CTA']) {
      expect(spec, `缺条款 ${s}`).toContain(s)
    }
  })
})

describe('R21 三端不回退基线（miniapp）', () => {
  it('stage.vue: 星级与单局诊断视图保留', () => {
    const s = readSrc('pages/stage/stage.vue')
    for (const k of ['fetchMyStars', 'diagnoseInfo', 'diagnose', 'd.stuck']) {
      expect(s, `stage.vue 基线丢失 ${k}`).toContain(k)
    }
  })
  it('client.ts: fetchMyStars / diagnose 保留', () => {
    const s = readSrc('api/client.ts')
    expect(s).toContain('fetchMyStars')
    expect(s).toContain('diagnose')
  })
  it('tokens: v1/v3-r3/r6 锚点保留且无 r21 新增令牌', () => {
    const s = readSrc('styles/tokens.css')
    expect(s).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(s).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(s).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(s).not.toContain('--r21-')
  })
})