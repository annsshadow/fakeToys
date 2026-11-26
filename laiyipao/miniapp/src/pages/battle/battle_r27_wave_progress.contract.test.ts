import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { buildWaveProgress } from './waveProgressHud'

const specPath = join(__dirname, '../../../../../docs/laiyipao/specs/round27-wave-progress-hud/SPEC.md')
const srcRoot = join(__dirname, '../../..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const spec = readFileSync(specPath, 'utf-8')
const battleSrc = readSrc('src/pages/battle/battle.vue')
const engineSrc = readSrc('src/game/engine.ts')
const tokensSrc = readSrc('src/styles/tokens.css')

type Raw = { waveIndex: number; waveTotal: number; onField: number; pending: number }
const raw = (waveIndex: number, waveTotal: number, onField: number, pending: number): Raw => ({ waveIndex, waveTotal, onField, pending })

describe('R27 波次进度聚合口径（真实模块 buildWaveProgress）', () => {
  it('resolved = waveTotal - onField - pending，clearedPct 按已处理比例给出', () => {
    const p = buildWaveProgress(raw(1, 10, 3, 2))
    expect(p.waveIndex).toBe(1)
    expect(p.waveTotal).toBe(10)
    expect(p.onField).toBe(3)
    expect(p.pending).toBe(2)
    expect(p.resolved).toBe(5)
    expect(p.clearedPct).toBe(50)
    expect(p.empty).toBe(false)
  })

  it('无限循环小数进度只保留一位小数，不产 NaN', () => {
    const p = buildWaveProgress(raw(0, 3, 1, 0))
    expect(p.resolved).toBe(2)
    expect(p.clearedPct).toBe(66.7)
    expect(Number.isFinite(p.clearedPct)).toBe(true)
    expect(p.ai).toContain('清理进度 66.7%')
  })

  it('超额与负数按 0..total clamp，resolved 亦收敛到 0..total', () => {
    const over = buildWaveProgress(raw(0, 5, 9, 9))
    expect(over.onField).toBe(5)
    expect(over.pending).toBe(5)
    expect(over.resolved).toBe(0)
    expect(over.clearedPct).toBe(0)
    const negative = buildWaveProgress(raw(0, 5, -2, -3))
    expect(negative.onField).toBe(0)
    expect(negative.pending).toBe(0)
    expect(negative.resolved).toBe(5)
    expect(negative.clearedPct).toBe(100)
  })

  it('非法数据（空/NaN/Infinity/负波次下标）走诚实空态，不抛异常不编造', () => {
    const cases: unknown[] = [null, undefined, raw(-1, 5, 1, 1), raw(0, NaN, 1, 1), raw(0, 5, Infinity, 1), raw(0, 5, 1, -Infinity)]
    for (const c of cases) {
      const p = buildWaveProgress(c as Raw)
      expect(p.empty).toBe(true)
      expect(p.resolved).toBe(0)
      expect(p.clearedPct).toBe(0)
      expect(p.ai).toBe('')
    }
  })

  it('本波无敌人（waveTotal=0）走空态', () => {
    const p = buildWaveProgress(raw(0, 0, 0, 0))
    expect(p.empty).toBe(true)
    expect(p.clearedPct).toBe(0)
    expect(p.ai).toBe('')
  })

  it('漏怪语义：resolved 含漏怪但不拆出 kills/leaked，也不反推击杀', () => {
    const p = buildWaveProgress(raw(0, 8, 1, 1))
    expect(p.resolved).toBe(6)
    expect(p).not.toHaveProperty('kills')
    expect(p).not.toHaveProperty('leaked')
  })

  it('Boss 按 1 只计入，不加权、不额外产出威胁字段', () => {
    const p = buildWaveProgress(raw(0, 1, 1, 0))
    expect(p.resolved).toBe(0)
    expect(p.clearedPct).toBe(0)
    expect(buildWaveProgress({ ...raw(0, 1, 1, 0), is_boss: true } as unknown as Raw).resolved).toBe(0)
    expect(p).not.toHaveProperty('threat')
  })

  it('AI 结论真实来自 clamp 后快照，含全部真实数字', () => {
    const p = buildWaveProgress(raw(0, 10, 3, 2))
    expect(p.ai).toContain('本波共 10 只')
    expect(p.ai).toContain('已处理 5 只')
    expect(p.ai).toContain('场上 3 只')
    expect(p.ai).toContain('待出场 2 只')
    expect(p.ai).toContain('清理进度 50.0%')
  })

  it('AI 结论百分比固定一位小数：0.0% 与 100.0% 边界且不产 NaN/undefined', () => {
    const zero = buildWaveProgress(raw(0, 5, 5, 0))
    expect(zero.clearedPct).toBe(0)
    expect(zero.ai).toContain('清理进度 0.0%')
    const full = buildWaveProgress(raw(0, 5, 0, 0))
    expect(full.clearedPct).toBe(100)
    expect(full.ai).toContain('清理进度 100.0%')
    for (const s of [zero.ai, full.ai]) expect(s).not.toMatch(/undefined|NaN/)
  })
})

describe('R27 真实源码守卫（防止只测 SPEC 文本）', () => {
  it('engine.ts 含只读 waveProgressHud()，方法体不写任何权威字段', () => {
    expect(engineSrc).toContain('waveProgressHud()')
    const m = engineSrc.match(/waveProgressHud\(\)[\s\S]*?\n  \}/)
    expect(m).toBeTruthy()
    for (const bad of ['this.record(', 'this.step(', 'this.beginWave(', 'this.emit(', 'this.finish(', 'this.score =', 'this.kills =', 'this.leaked =']) expect(m![0]).not.toContain(bad)
  })

  it('battle.vue 模板落地 R27 选择器与逐字空态/依据/红线文案', () => {
    for (const s of ['r27-wave-progress', 'data-r27-progress', 'r27-progress-track', 'r27-progress-fill', 'r27-progress-text', 'r27-progress-empty', 'r27-ai-note']) expect(battleSrc).toContain(s)
    expect(battleSrc).toContain('本波暂无敌人进度数据')
    expect(battleSrc).toContain('依据：引擎 waveProgressHud() 只读快照（当前波 spawns 总数 − 场上存活 − 待出场）；只读派生不进 replay_hash')
    expect(battleSrc).toContain('辅助进度提示非权威战斗判定，已处理含击败与漏怪（漏怪已扣防线）；权威以 replay_hash 为准')
  })

  it('battle.vue 进度文本固定一位小数（toFixed(1)）', () => {
    expect(battleSrc).toContain('r27Progress.clearedPct.toFixed(1)')
    expect(battleSrc).not.toMatch(/\{\{ r27Progress\.clearedPct \}\}/)
  })

  it('battle.vue CSS：r27 容器自身单行紧凑，hud-row 保留 flex-wrap: wrap', () => {
    expect(battleSrc).toMatch(/\.r27-wave-progress[\s\S]{0,400}?max-height\s*:/)
    expect(battleSrc).toContain('overflow: hidden')
    expect(battleSrc).toMatch(/\.r27-wave-progress[\s\S]{0,400}?flex-shrink\s*:\s*0/)
    // R9 390×844 修复：hud-row 必须保留换行，绝不改成 nowrap / 单行 flex
    expect(battleSrc).toMatch(/\.hud-row\s*\{[\s\S]{0,300}?flex-wrap\s*:\s*wrap/)
    expect(battleSrc).not.toMatch(/\.hud-row\s*\{[\s\S]{0,300}?flex-wrap\s*:\s*nowrap/)
    for (const k of ['overlay-top', 'hud-row', 'hud-pill', 'wave-text', 'kill-text', 'star-gap-text', 'time-budget']) expect(battleSrc).toContain(k)
  })

  it('battle.vue 从 waveProgressHud 导入并调用真实纯函数', () => {
    expect(battleSrc).toContain('buildWaveProgress')
    expect(battleSrc).toContain('waveProgressHud()')
    expect(battleSrc).toContain('./waveProgressHud')
  })
})

describe('R27 SPEC 选择器与逐字文案契约', () => {
  it('SPEC 钉死七类选择器', () => {
    for (const s of ['.r27-wave-progress', '[data-r27-progress]', '.r27-progress-track', '.r27-progress-fill', '.r27-progress-text', '.r27-progress-empty', '.r27-ai-note']) expect(spec).toContain(s)
  })

  it('SPEC 钉死空态、依据行与逐字红线', () => {
    expect(spec).toContain('本波暂无敌人进度数据')
    expect(spec).toContain('依据：引擎 waveProgressHud() 只读快照（当前波 spawns 总数 − 场上存活 − 待出场）；只读派生不进 replay_hash')
    expect(spec).toContain('辅助进度提示非权威战斗判定，已处理含击败与漏怪（漏怪已扣防线）；权威以 replay_hash 为准')
  })

  it('SPEC 钉死 clamp、Boss/漏怪语义与 RED 门禁', () => {
    for (const k of ['clamp', 'Boss', '漏怪', 'RED 门禁', 'ERR_MODULE_NOT_FOUND', '禁止内嵌参考实现']) expect(spec).toContain(k)
  })
})

describe('R27 诚实红线与 AI 四判据', () => {
  it('SPEC 含 AI 四判据关键词', () => {
    for (const k of ['可见汉字', '真实数据结论', '依据行', 'CTA']) expect(spec).toContain(k)
  })

  it('SPEC 钉死只读派生边界', () => {
    for (const k of ['不得调用', 'step()', 'replay_hash', 'score', 'loot']) expect(spec).toContain(k)
  })
})

describe('R27 既有数据源依据与三端不回退', () => {
  it('engine.ts 含波次数据源与清波判定依据', () => {
    for (const k of ['waveIndex', 'spawnQueue', 'totalEnemies', 'checkWaveEnd', 'compactInPlace']) expect(engineSrc).toContain(k)
  })

  it('battle.vue 保留 R24/R26 既有 HUD 锚点', () => {
    for (const k of ['r24-element-hud', 'r26-wave-brief', 'showCards', 'r22-synergy-card']) expect(battleSrc).toContain(k)
  })

  it('令牌三锚点保留且无 --r27- 新令牌', () => {
    for (const k of ['lyp-tokens-v1', 'lyp-tokens-v3-r3', 'lyp-tokens-r6']) expect(tokensSrc).toContain(k)
    expect(tokensSrc).not.toContain('--r27-')
  })
})

// R43 CTA 文案-行为一致性守卫（R28 范式：真实源码文本断言，禁止内嵌参考实现）
describe('R43 战斗 HUD AI 解读区 CTA 文案-行为一致性（真实 battle.vue）', () => {
  const ctaLine = (cls: string): string => {
    const line = battleSrc.split('\n').find(l => l.includes('class="' + cls + '"'))
    expect(line, '未找到 class="' + cls + '" 的模板行').toBeTruthy()
    return line ?? ''
  }
  const cssRule = (sel: string): string => {
    const start = battleSrc.indexOf(sel + ' {')
    if (start < 0) return ''
    const end = battleSrc.indexOf('}', start)
    return end < 0 ? '' : battleSrc.slice(start, end)
  }

  it('R27 r27-ai-cta 必须是只读提示：逐字文案保留，但不得绑定 openHelp/@click', () => {
    const line = ctaLine('r27-ai-cta')
    expect(line).toContain('继续清理场上敌人 →')
    expect(line).not.toContain('openHelp')
    expect(line).not.toContain('@click')
  })

  it('R27 .r27-ai-cta CSS 不得 pointer-events:auto（只读提示不拦截点击）', () => {
    const rule = cssRule('.r27-ai-cta')
    expect(rule).not.toContain('pointer-events: auto')
    expect(rule).not.toContain('pointer-events:auto')
  })

  it('R26 r26-ai-cta 保持只读锚点（无 @click），作为修复对齐目标', () => {
    expect(ctaLine('r26-ai-cta')).not.toContain('@click')
  })
})
describe('R27 零真值', () => {
  it('SPEC 与源码不含域名/IP/JWT/口令/个人绝对路径', () => {
    for (const text of [spec, battleSrc, tokensSrc]) {
      expect(text).not.toMatch(/eyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}/)
      expect(text).not.toMatch(/\b(?:\d{1,3}\.){3}\d{1,3}\b/)
      expect(text).not.toMatch(/postgres:\/\/[^\s]+:[^\s]+@/)
      expect(text).not.toMatch(/C:\\Users\\[^\s]+/)
    }
  })
})
