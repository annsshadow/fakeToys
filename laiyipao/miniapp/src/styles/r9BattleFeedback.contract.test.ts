import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
const repoRoot = join(__dirname, '..', '..', '..', '..')
const doc = readFileSync(join(repoRoot, 'docs', 'laiyipao', 'design', 'AI_VISUAL_SYSTEM.md'), 'utf-8')
const quick = readFileSync(join(repoRoot, 'docs', 'laiyipao', 'design', 'TOKENS_QUICK_REF.md'), 'utf-8')
const tokens = readFileSync(join(__dirname, 'tokens.css'), 'utf-8')
const start = doc.indexOf('## 1H.')
const end = doc.indexOf('## 2.', start)
const spec = start >= 0 && end > start ? doc.slice(start, end) : ''
describe('R9 H5 战斗反馈与选卡因果契约', () => {
  it('§1H 五条款与 §7F 速查存在，依据锚定 R9 审计', () => {
    expect(start).toBeGreaterThanOrEqual(0)
    for (const n of [1, 2, 3, 4, 5]) expect(spec, '缺少 1H.' + n).toContain('### 1H.' + n)
    expect(quick).toContain('## 7F.')
    for (const k of ['round9-gameplay', '非物理真机', 'r9-06-reaction-feed.png']) expect(spec, k).toContain(k)
  })
  it('反应记录包含名称、触发元素与伤害，可追溯因果', () => {
    for (const k of ['recentReactions', 'REACTIONS', '触发元素', 'damage', '抗性削减', '为什么这一击痛', '不遮挡']) expect(spec, k).toContain(k)
  })
  it('选卡反馈与钳分换算写清权威来源和禁止项', () => {
    for (const k of ['card_picks', '跳过卡牌', '伤害占比', '不做数值碾压', '上限→实得→差额', '钳分中·距满分 Xs', '禁止各说各话', 'score_full_at_sec', 'star_targets']) expect(spec, k).toContain(k)
  })
  it('AI 四判据是硬下限，低透明度底纹不算', () => {
    for (const k of ['可见汉字「AI」', '真实数据结论', '依据', 'CTA', '低透明度底纹不算', '缺一不算']) expect(spec, k).toContain(k)
  })
  it('令牌锚点不变且 R9 不新增视觉变量', () => {
    expect(tokens).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/)
    expect(tokens).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/)
    expect(tokens).toMatch(/--lyp-r6-version:\s*lyp-tokens-r6/)
    expect(tokens).not.toMatch(/lyp-tokens-v4/)
    expect(tokens).not.toMatch(/--(?:lyp-)?r9-[a-z-]+:/)
    expect(spec).toContain('不新增视觉令牌')
  })
})
