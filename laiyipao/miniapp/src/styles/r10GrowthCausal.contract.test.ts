import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
const repoRoot = join(__dirname, '..', '..', '..', '..')
const doc = readFileSync(join(repoRoot, 'docs', 'laiyipao', 'design', 'AI_VISUAL_SYSTEM.md'), 'utf-8')
const quick = readFileSync(join(repoRoot, 'docs', 'laiyipao', 'design', 'TOKENS_QUICK_REF.md'), 'utf-8')
const tokens = readFileSync(join(__dirname, 'tokens.css'), 'utf-8')
const start = doc.indexOf('## 1I.')
const end = doc.indexOf('## 2.', start)
const spec = start >= 0 && end > start ? doc.slice(start, end) : ''
describe('R10 H5 养成因果链与收益可解释契约', () => {
  it('§1I 五条款与 §7G 速查存在，依据锚定 R10 审计', () => {
    expect(start).toBeGreaterThanOrEqual(0)
    for (const n of [1, 2, 3, 4, 5]) expect(spec, '缺少 1I.' + n).toContain('### 1I.' + n)
    expect(quick).toContain('## 7G.')
    for (const k of ['round10-gameplay', '非物理真机', 'r10-07-gems-bag-cta-visible.png']) expect(spec, k).toContain(k)
  })
  it('推荐 CTA 参数可达且客户端不得伪造', () => {
    for (const k of ['candidate', 'replace', 'params', '客户端不得伪造', '禁止下发无参数 CTA']) expect(spec, k).toContain(k)
  })
  it('养成前后差值必须来自两次真实结算且宝石三段打通', () => {
    for (const k of ['养成前', '养成后', '同一关', '可信差值', '两次真实结算', '禁止编造', '去背包镶嵌', '不虚构', '属性差分']) expect(spec, k).toContain(k)
  })
  it('AI 四判据是硬下限，低透明度底纹不算', () => {
    for (const k of ['可见汉字「AI」', '真实数据结论', '依据', 'CTA', '低透明度底纹不算', '缺一不算']) expect(spec, k).toContain(k)
  })
  it('令牌锚点不变且 R10 不新增视觉变量', () => {
    expect(tokens).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/)
    expect(tokens).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/)
    expect(tokens).toMatch(/--lyp-r6-version:\s*lyp-tokens-r6/)
    expect(tokens).not.toMatch(/lyp-tokens-v4/)
    expect(tokens).not.toMatch(/--(?:lyp-)?r10-[a-z-]+:/)
    expect(spec).toContain('不新增视觉令牌')
  })
})
