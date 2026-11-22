import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const repoRoot = join(__dirname, '..', '..', '..', '..')
const doc = readFileSync(join(repoRoot, 'docs', 'laiyipao', 'design', 'AI_VISUAL_SYSTEM.md'), 'utf-8')
const quick = readFileSync(join(repoRoot, 'docs', 'laiyipao', 'design', 'TOKENS_QUICK_REF.md'), 'utf-8')
const tokens = readFileSync(join(__dirname, 'tokens.css'), 'utf-8')
const start = doc.indexOf('## 1F.')
const end = doc.indexOf('## 2.', start)
const spec = start >= 0 && end > start ? doc.slice(start, end) : ''

describe('R7 H5 玩法体验规范契约', () => {
  it('§1F 五条款与 §7D 速查存在，依据锚定 R7 真浏览器审计', () => {
    expect(start).toBeGreaterThanOrEqual(0)
    for (const n of [1, 2, 3, 4, 5]) expect(spec, '缺少 1F.' + n).toContain('### 1F.' + n)
    expect(quick).toContain('## 7D.')
    for (const k of ['round7-gameplay', '非物理真机', 'r7-09-battle-result.png', 'r7-10-battle-report.png', 'r7-03-stage-select.png', 'r7-11-skills.png', 'r7-11-equipment.png']) expect(spec, k).toContain(k)
  })

  it('复盘归因讲清时长额度、星级门槛与漏怪/反应的取舍', () => {
    for (const k of ['19726', '10909', '11941', '74s', '得分超时长额度', 'clamp_note', 'settleResult.score', 'star_targets', 'leaked', 'reactions', '诚实空态', '不得重算']) expect(spec, k).toContain(k)
    for (const k of ['30%', '关系不大']) expect(spec, k).toContain(k)
  })

  it('战斗中实时差额不得写死门槛并保证可玩可视', () => {
    for (const k of ['战斗实时星级差额', '不得硬编码 11941', 'star_targets', '回合结束前', '非阻塞', '不遮挡塔位', 'prefers-reduced-motion']) expect(spec, k).toContain(k)
  })

  it('成长页推荐必须解释因果并先对比确认再落地', () => {
    for (const k of ['/me/recommendations', 'evidence[]', 'expected_gain', '当前 vs 推荐', '确认替换', '取消', '出战配置', '禁止无确认', '本地规则引擎', '重炮弹', '母核碎片']) expect(spec, k).toContain(k)
  })

  it('令牌只保留既有锚点，R7 不新增视觉变量', () => {
    expect(tokens).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/)
    expect(tokens).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/)
    expect(tokens).toMatch(/--lyp-r6-version:\s*lyp-tokens-r6/)
    expect(tokens).not.toMatch(/lyp-tokens-v4/)
    expect(tokens).not.toMatch(/--(?:lyp-)?r7-[a-z-]+:/)
    expect(spec).toContain('不新增视觉令牌')
  })
})
