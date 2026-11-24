import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { buildReactionTimelineView, R29_EMPTY_TEXT, R29_TRUNCATED_TEXT, R29_BASIS, R29_HONESTY, R29_UNKNOWN_SUFFIX } from './reactionTimelineView'

const specPath = join(__dirname, '../../../../../docs/laiyipao/specs/round29-live-reaction-timeline/SPEC.md')
const srcRoot = join(__dirname, '../../..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const spec = readFileSync(specPath, 'utf-8')
const battleSrc = readSrc('src/pages/battle/battle.vue')
const engineSrc = readSrc('src/game/engine.ts')
const tokensSrc = readSrc('src/styles/tokens.css')
const CTA = '查看完整反应时间线 →'

// 平衡花括号提取完整函数体，避免 slice 到 EOF 误伤后续函数/CSS
function extractFunctionBody(src: string, signature: string): string {
  const start = src.indexOf(signature)
  if (start < 0) return ''
  const braceStart = src.indexOf('{', start)
  if (braceStart < 0) return ''
  let depth = 0
  for (let i = braceStart; i < src.length; i++) {
    const ch = src[i]
    if (ch === '{') depth++
    else if (ch === '}') {
      depth--
      if (depth === 0) return src.slice(start, i + 1)
    }
  }
  return src.slice(start)
}

type TL = { wave: number; tick: number; reaction: string; enemy_uid: number; damage: number }
const tl = (wave: number, tick: number, reaction: string, enemy_uid: number, damage: number): TL => ({ wave, tick, reaction, enemy_uid, damage })
const rx = [{ key: 'steam_burst', name: '蒸汽爆发' }, { key: 'overheat', name: '过热' }]

describe('R29 时间线展示纯函数（真实模块 reactionTimelineView）', () => {
  it('按 wave/tick 正序且过滤非法记录（wave<1/tick<0/空reaction/uid<1/非有限damage）', () => {
    const events = [tl(2, 5, 'overheat', 9, 50), tl(1, 9, 'steam_burst', 5, 100), tl(1, 3, 'steam_burst', 7, 80)]
    const bad = [tl(0, 1, 'overheat', 9, 50), tl(1, -1, 'overheat', 9, 50), tl(1, 1, '', 9, 50), tl(1, 1, 'overheat', 0, 50), tl(1, 1, 'overheat', 9, NaN)]
    const view = buildReactionTimelineView([...events, ...bad], false, rx)
    expect(view.rows.map(r => [r.wave, r.tick])).toEqual([[1, 3], [1, 9], [2, 5]])
    expect(view.rows.map(r => r.index)).toEqual([0, 1, 2])
    expect(view.rows.every(r => r.wave >= 1 && r.tick >= 0 && r.enemy_uid >= 1 && Number.isFinite(r.damage))).toBe(true)
  })

  it('反应名以 config.reactions 为准，未知 key 原样并明确标注未知', () => {
    const view = buildReactionTimelineView([tl(1, 1, 'steam_burst', 5, 100), tl(1, 2, 'mystery_key', 6, 20)], false, rx)
    expect(view.rows[0].reactionName).toBe('蒸汽爆发')
    expect(view.rows[1].reactionName).toBe('mystery_key' + R29_UNKNOWN_SUFFIX)
    expect(view.rows[1].reaction).toBe('mystery_key')
  })

  it('空/全非法 → empty=true、rows=[]，截断不制造伪数据', () => {
    for (const input of [null, undefined, [], [tl(0, 1, 'overheat', 9, 50)]]) {
      const view = buildReactionTimelineView(input as any, true, rx)
      expect(view.rows).toEqual([])
      expect(view.empty).toBe(true)
    }
  })

  it('truncated 透传且有数据时为 true；不修改传入数组', () => {
    const events = [tl(1, 1, 'overheat', 9, 50)]
    const snapshot = JSON.stringify(events)
    const view = buildReactionTimelineView(events, true, rx)
    expect(view.truncated).toBe(true)
    expect(JSON.stringify(events)).toBe(snapshot)
    expect(buildReactionTimelineView(events, false, rx).truncated).toBe(false)
  })

  it('同 wave+tick 保持原输入稳定次序', () => {
    const view = buildReactionTimelineView([tl(1, 1, 'overheat', 9, 50), tl(1, 1, 'steam_burst', 5, 100)], false, rx)
    expect(view.rows.map(r => r.reaction)).toEqual(['overheat', 'steam_burst'])
  })
})

describe('R29 逐字文案与 SPEC 契约', () => {
  it('常量与 SPEC 逐字一致', () => {
    expect(R29_EMPTY_TEXT).toBe('本局暂无逐次反应记录')
    expect(R29_TRUNCATED_TEXT).toBe('已截断，仅显示前 200 条')
    expect(R29_BASIS).toBe('依据：引擎 reactionTimeline() 只读旁路（前 200 条，不进 replay_hash）；反应名以服务端 config.reactions 为准')
    expect(R29_HONESTY).toBe('辅助展示非权威判据，权威以 replay_hash 为准')
    expect(R29_UNKNOWN_SUFFIX).toBe('（未知反应）')
  })

  it('SPEC 钉死选择器、CTA、暂停恢复、AI 四判据与 RED 门禁', () => {
    for (const k of ['.r29-timeline-overlay', '.r29-timeline-box', '.r29-timeline-scroll', '.r29-timeline-row', '[data-r29-index]', '[data-r29-wave]', '[data-r29-enemy]', '.r29-timeline-empty', '.r29-timeline-truncated', 'openReactionTimeline', 'timelinePausedByDrawer', 'reactionTimelineVisible', 'helpPausedByPage', CTA, 'AI', R29_BASIS, R29_HONESTY, 'ERR_MODULE_NOT_FOUND', '禁止内嵌参考实现', 'z-index: 40', 'closeHelp()', 'closeReactionTimeline()', '本局显示 {{ rows.length }} 条逐次反应记录', '本局共']) expect(spec).toContain(k)
  })
})

describe('R29 battle.vue 真实源码守卫', () => {
  it('R28 CTA 已改指 openReactionTimeline，文案不变；图例 CTA 仍 openHelp', () => {
    const ctaLine = battleSrc.split('\n').find(l => l.includes('r28-feed-cta')) || ''
    expect(ctaLine).toContain(CTA)
    expect(ctaLine).toContain('@click.stop="openReactionTimeline"')
    expect(ctaLine).not.toContain('@click.stop="openHelp"')
    expect(battleSrc).toContain('@click="openHelp"')
    expect(battleSrc).toContain('legend-overlay')
  })

  it('结算结果 sheet 内含真实可见可点的时间线入口（结算态用户可达）', () => {
    // t227：结算 sheet 遮挡战斗态 R28 CTA，必须在结果页内提供独立入口
    const sheetStart = battleSrc.indexOf('v-if="settleResult" class="sheet"')
    expect(sheetStart).toBeGreaterThanOrEqual(0)
    // 取结算 sheet 到下一个 startError sheet 之间的片段，确保入口在结果 sheet 内
    const resultSheet = battleSrc.slice(sheetStart, battleSrc.indexOf('v-if="startError" class="sheet"'))
    const ctaLine = resultSheet.split('\n').find(l => l.includes('r29-result-timeline-cta')) || ''
    expect(ctaLine).toContain('r29-result-timeline-cta')
    expect(ctaLine).toContain(CTA)
    expect(ctaLine).toContain('@click.stop="openReactionTimeline"')
    expect(ctaLine).not.toContain('@click.stop="openHelp"')
    // 战斗态 R28 CTA 仍保留（不回退）
    expect(battleSrc).toContain('r28-feed-cta')
  })

  it('抽屉 UI 选择器与逐字文案真实存在', () => {
    for (const s of ['r29-timeline-overlay', 'r29-timeline-box', 'r29-timeline-scroll', 'r29-timeline-row', 'data-r29-index', 'data-r29-wave', 'data-r29-enemy', 'r29-timeline-empty', 'r29-timeline-truncated', 'AI · 完整反应时间线', R29_EMPTY_TEXT, R29_TRUNCATED_TEXT, R29_BASIS, R29_HONESTY]) expect(battleSrc).toContain(s)
    // AI 结论按 Vue 插值逐字断言（双花括号），空态独立 v-if 分支，非空才显示结论
    const VUE_AI = '本局显示 {{ rows.length }} 条逐次反应记录'
    expect(battleSrc).toContain(VUE_AI)
    expect(battleSrc).not.toContain('本局显示 {rows.length} 条逐次反应记录')
    expect(battleSrc).not.toContain('本局共')
    const aiLine = battleSrc.split('\n').find(l => l.includes(VUE_AI)) || ''
    expect(aiLine).toContain('v-if')
    const emptyLine = battleSrc.split('\n').find(l => l.includes(R29_EMPTY_TEXT)) || ''
    expect(emptyLine).toContain('v-if')
  })

  it('真实导入纯函数并调用；暂停/恢复独立记账', () => {
    expect(battleSrc).toContain("from './reactionTimelineView'")
    for (const fn of ['buildReactionTimelineView', 'openReactionTimeline', 'closeReactionTimeline']) {
      expect(battleSrc).toMatch(new RegExp('\\b' + fn + '\\s*\\('))
    }
    const openBlock = extractFunctionBody(battleSrc, 'function openReactionTimeline')
    const closeBlock = extractFunctionBody(battleSrc, 'function closeReactionTimeline')
    expect(openBlock).not.toBe('')
    expect(closeBlock).not.toBe('')
    expect(openBlock).toContain('reactionTimelineVisible.value = true')
    expect(openBlock).toContain('timelinePausedByDrawer = true')
    expect(openBlock).toContain('doPause()')
    expect(closeBlock).toContain('reactionTimelineVisible.value = false')
    expect(closeBlock).toContain('timelinePausedByDrawer')
    expect(closeBlock).toContain('doResume()')
    expect(openBlock + closeBlock).not.toContain('helpPausedByPage')
    expect(openBlock + closeBlock).not.toContain('helpVisible')
    // 结算态/已暂停不得记账；倍速不得被抽屉开关改写
    expect(openBlock).toContain('settled.value')
    expect(openBlock).toContain('paused.value')
    expect(openBlock + closeBlock).not.toContain('setSpeed(')
    expect(openBlock + closeBlock).not.toContain('speed.value =')
    // 互斥：openReactionTimeline 先无条件 closeHelp()；openHelp 先无条件 closeReactionTimeline()
    expect(openBlock).toMatch(/\bcloseHelp\s*\(\s*\)/)
    const helpOpenBlock = extractFunctionBody(battleSrc, 'function openHelp')
    expect(helpOpenBlock).not.toBe('')
    expect(helpOpenBlock).toMatch(/\bcloseReactionTimeline\s*\(\s*\)/)
  })

  it('从引擎只读 API 取时间线与截断标记，反应名取 store.config.reactions', () => {
    expect(battleSrc).toContain('reactionTimeline()')
    expect(battleSrc).toContain('reactionTimelineTruncated()')
    expect(battleSrc).toContain('store.config.reactions')
    expect(battleSrc).not.toContain('reactionTimelineEvents')
  })

  it('CSS：抽屉可滚动、内容可点击、长列表不溢出文档', () => {
    const rule = (sel: string) => {
      const m = battleSrc.match(new RegExp(sel.replace(/\./g, '\\.') + '\\s*\\{([^}]*)\\}'))
      return m ? m[1] : ''
    }
    const box = rule('.r29-timeline-box')
    expect(box).toMatch(/pointer-events\s*:\s*auto/)
    const scroll = rule('.r29-timeline-scroll')
    expect(scroll).toMatch(/max-height\s*:/)
    expect(scroll).toMatch(/overflow-y\s*:\s*auto/)
    // z-index:40：高于 pause/legend 的 30 与结算 sheet(auto)，标题与关闭键不被遮挡
    const overlay = rule('.r29-timeline-overlay')
    expect(overlay.replace(/\s/g, '')).toContain('z-index:40')
  })
})

describe('R29 三端不回退与零真值', () => {
  it('engine 只读 API 存在且未反向写入；R15/R24/R26/R27/R28 结构保留', () => {
    for (const k of ['reactionTimeline()', 'reactionTimelineTruncated()', 'REACTION_TIMELINE_LIMIT = 200']) expect(engineSrc).toContain(k)
    expect(engineSrc).not.toContain('buildReactionTimelineView')
    for (const k of ['r15-reaction-timeline', 'r24-element-hud', 'r26-wave-brief', 'r27-wave-progress', 'r28-feed-cta']) expect(battleSrc + readSrc('src/pages/battles/battles.vue')).toContain(k)
  })

  it('令牌三锚点保留且无 --r29-', () => {
    for (const k of ['lyp-tokens-v1', 'lyp-tokens-v3-r3', 'lyp-tokens-r6']) expect(tokensSrc).toContain(k)
    expect(tokensSrc).not.toContain('--r29-')
  })

  it('SPEC 与源码不含域名/IP/JWT/口令/个人绝对路径', () => {
    for (const text of [spec, battleSrc, engineSrc, tokensSrc]) {
      expect(text).not.toMatch(/eyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}/)
      expect(text).not.toMatch(/\b(?:\d{1,3}\.){3}\d{1,3}\b/)
      expect(text).not.toMatch(/postgres:\/\/[^\s]+:[^\s]+@/)
      expect(text).not.toMatch(/C:\\Users\\[^\s]+/)
    }
  })
})