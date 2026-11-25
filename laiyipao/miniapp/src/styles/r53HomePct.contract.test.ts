// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R53 契约：首页 dims.pct 取整消除 AI 结论长浮点。
 * 真实读源码 + 真实 import 权威 helper，禁止内嵌参考实现。
 * 不碰 engine/replay/hash/score/settle/loot/tokens/后端；不新增 --r53-*。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

const INDEX = 'pages/index/index.vue'
// 队长坐实的真实存档：装备契合 8/18，权威短板=装备契合、整数 pct=44
const CASE = { element_coverage: 5, reaction_coverage: 3, mastery_done: 8, equipment_synergy: 8, mechanic_depth: 8 }
const W_SERVER = { element_coverage: 12, reaction_coverage: 15, mastery_done: 10, equipment_synergy: 12, mechanic_depth: 8 }

// 只截取 dims computed 区块，避免 ratingGrade 等已取整段落干扰
const dimsBody = (src: string): string => {
  const start = src.indexOf('const dims = computed')
  if (start < 0) return ''
  const end = src.indexOf('const weaknesses', start)
  return src.slice(start, end > start ? end : start + 1600)
}
const pctLines = (body: string): string[] =>
  body.split(String.fromCharCode(10)).map((l) => l.trim()).filter((l) => l.startsWith('pct:'))

describe('R53 首页 dims.pct 取整（真实源码守卫）', () => {
  it('index.vue 五维 pct 全部 Math.round 包裹且无裸浮点模式', () => {
    const src = readSrc(INDEX)
    const body = dimsBody(src)
    expect(body, '未找到 dims computed 区块').not.toBe('')
    const lines = pctLines(body)
    expect(lines.length, 'dims 五维 pct 行数应为 5').toBe(5)
    for (const line of lines) {
      expect(line, 'pct 裸浮点未取整: ' + line).toContain('Math.round(')
    }
    expect(body, '仍有裸 pct: (( 浮点模式').not.toContain('pct: ((')
    expect(body, '装备契合仍无 Math.round 包裹上界').not.toContain('pct: Math.min(100, ((')
  })

  it('AI 结论模板仍走 dims.pct（不变量：模板与 aiInsight 来源不变）', () => {
    const src = readSrc(INDEX)
    expect(src, 'AI 结论模板必须保留 dims.pct 直出').toContain('{{ aiInsight.weak.pct }}%')
    const start = src.indexOf('const aiInsight = computed')
    const end = src.indexOf('const attackerRows', start)
    const body = src.slice(start, end > start ? end : start + 500)
    expect(body, 'aiInsight 必须从 dims 取 weak').toContain('dims.value.find')
    expect(body, 'aiInsight 不得自行重算 pct').not.toContain('pct:')
  })

  it('真实 helper 对 8/18 案例给出整数 44%，首页装备契合行必须与之对齐', async () => {
    const mod = await import('../pages/_shared/r12GrowthPriority').catch(() => null)
    expect(mod, '权威 helper 不存在').toBeTruthy()
    const p = mod!.computeGrowthPriority(CASE, W_SERVER)
    expect(p, '权威 helper 应给出短板').not.toBeNull()
    expect(p!.label, '权威短板应为装备契合').toBe('装备契合')
    expect(p!.value, '权威 value 应为 8/18').toBe('8/18')
    expect(p!.pct, '权威 pct 应为整数 44').toBe(44)
    expect(Number.isInteger(p!.pct), '权威 pct 必须是整数').toBe(true)
    const lines = pctLines(dimsBody(readSrc(INDEX)))
    const eq = lines.find((l) => l.includes('equipment_synergy'))
    expect(eq, '首页缺装备契合 pct 行').toBeTruthy()
    expect(eq, '首页装备契合 pct 必须 Math.round 取整，与 helper 44% 一致').toContain('Math.round(')
  })

  it('不变量：marginalGain 排序 / 雷达几何 / 进度条 / 空态 / 令牌均保留', () => {
    const src = readSrc(INDEX)
    const mg = src.slice(src.indexOf('const marginalGain = computed'), src.indexOf('const aiInsight = computed'))
    expect(mg, 'marginalGain 仍应用 weightPct×gap').toContain('d.weightPct * gap')
    expect(mg, 'marginalGain 排序不应依赖 pct').toContain('sort((a, b) => b.gain - a.gain)')
    expect(src, '进度条宽度仍消费 d.pct').toContain("width: d.pct + '%'")
    expect(src, '雷达几何仍消费 dims.pct').toContain('dims.value[i]?.pct ?? 0')
    expect(src, '空维入口仍用 pct===0 判定').toContain('d.pct === 0 && d.hint')
    const tokens = readSrc('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toContain('--r53-')
    expect(src).not.toContain('--r53-')
  })
})
