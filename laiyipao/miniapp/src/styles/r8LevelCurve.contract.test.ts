import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
const repoRoot = join(__dirname, '..', '..', '..', '..')
const doc = readFileSync(join(repoRoot, 'docs', 'laiyipao', 'design', 'AI_VISUAL_SYSTEM.md'), 'utf-8')
const quick = readFileSync(join(repoRoot, 'docs', 'laiyipao', 'design', 'TOKENS_QUICK_REF.md'), 'utf-8')
const tokens = readFileSync(join(__dirname, 'tokens.css'), 'utf-8')
const start = doc.indexOf('## 1G.')
const end = doc.indexOf('## 2.', start)
const spec = start >= 0 && end > start ? doc.slice(start, end) : ''
describe('R8 H5 关卡规范契约', () => {
  it('§1G 五条款与 §7E 速查存在，依据锚定 R8 真浏览器审计', () => {
    expect(start).toBeGreaterThanOrEqual(0)
    for (const n of [1, 2, 3, 4, 5]) expect(spec, '缺少 1G.' + n).toContain('### 1G.' + n)
    expect(quick).toContain('## 7E.')
    for (const k of ['round8-gameplay', '非物理真机', 'r8-01-terrain-index-deduped.png', 'r8-02-enemy-threat-tags.png', 'r8-03-level1-result.png']) expect(spec, k).toContain(k)
  })
  it('星级门槛用本关满分解释且禁止写死具体关卡数字', () => {
    for (const k of ['1★门槛=本关满分约60%', '降 47 次', '降 44 次', '非单调', '高关更容易', '按本关满分百分比给星', '不得硬编码 11941']) expect(spec, k).toContain(k)
  })
  it('难度曲线标注机制拐点，拒绝并列跳变假洞察', () => {
    for (const k of ['56/60/91/95/96/100', 'r5AiStage', '+33‰', '本章最大跳变', '新敌种首现', '地形变化', '飞行', '潜地', '远程']) expect(spec, k).toContain(k)
  })
  it('战斗时间预算必须来自配置且临近截止即时提示', () => {
    for (const k of ['剩余保分窗口', 'score_full_at_sec', '不得硬编码', '不遮挡', '临近截止', '即时提示', '钳分']) expect(spec, k).toContain(k)
  })
  it('令牌只保留既有锚点，R8 不新增视觉变量', () => {
    expect(tokens).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/)
    expect(tokens).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/)
    expect(tokens).toMatch(/--lyp-r6-version:\s*lyp-tokens-r6/)
    expect(tokens).not.toMatch(/lyp-tokens-v4/)
    expect(tokens).not.toMatch(/--(?:lyp-)?r8-[a-z-]+:/)
    expect(spec).toContain('不新增视觉令牌')
  })
})
