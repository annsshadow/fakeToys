// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R103 R29 完整反应时间线抽屉键盘闭环契约（test-first RED）。
 * 真实读取 battle.vue / tokens.css，禁止内嵌实现副本。
 * 8 缺口：两处 CTA 三件套、抽屉 dialog+ESC+Tab、关闭按钮三件套+ref、
 *        三个 setup ref、open 焦点移入、触发源记录与 close 焦点归还、trapTimelineTab。
 * 4 不变量：互斥、R73 图例闭环、暂停记账、令牌/文案/@click.stop。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

/** 按唯一属性串锚点定位含 needle 的开标签（从 <tag 到 >），不依赖行号。 */
function tagBefore(src: string, needle: string, tag: string): string {
  const k = src.indexOf(needle)
  if (k < 0) return ''
  const start = src.lastIndexOf('<' + tag, k)
  if (start < 0 || start > k) return ''
  const end = src.indexOf('>', k)
  return end < 0 ? src.slice(start) : src.slice(start, end + 1)
}

/** 括号配对取函数体（含大括号），不依赖行号。 */
function blockAfter(text: string, keyword: string): string {
  const k = text.indexOf(keyword)
  if (k < 0) return ''
  const open = text.indexOf('{', k)
  if (open < 0) return ''
  let depth = 0
  for (let i = open; i < text.length; i++) {
    const ch = text[i]
    if (ch === '{') depth++
    else if (ch === '}') {
      depth--
      if (depth === 0) return text.slice(open, i + 1)
    }
  }
  return ''
}

describe('R103 时间线抽屉键盘闭环契约', () => {
  const battle = readSrc('pages/battle/battle.vue')
  const tokens = readSrc('styles/tokens.css')
  const cta1 = () => tagBefore(battle, 'class="r28-feed-cta"', 'text')
  const cta2 = () => tagBefore(battle, 'class="r29-result-timeline-cta"', 'view')
  const overlay = () => tagBefore(battle, 'class="r29-timeline-overlay"', 'view')
  const closeBtn = () => tagBefore(battle, 'class="r29-timeline-close"', 'view')
  const openBody = () => blockAfter(battle, 'function openReactionTimeline')
  const closeBody = () => blockAfter(battle, 'function closeReactionTimeline')

  it('it1 R28 feed CTA 补键盘三件套且保留 @click.stop', () => {
    const tag = cta1()
    expect(tag, '须定位 r28-feed-cta 开标签').not.toBe('')
    expect(tag, 'CTA1 须有 role="button"').toContain('role="button"')
    expect(tag, 'CTA1 须有 tabindex="0"').toContain('tabindex="0"')
    expect(tag, 'CTA1 须有 @keydown.enter 复用 openReactionTimeline').toContain('@keydown.enter="openReactionTimeline"')
    expect(tag, 'CTA1 原 @click.stop 须保留').toContain('@click.stop="openReactionTimeline"')
  })

  it('it2 结算页 timeline CTA 补键盘三件套且保留 @click.stop', () => {
    const tag = cta2()
    expect(tag, '须定位 r29-result-timeline-cta 开标签').not.toBe('')
    expect(tag, 'CTA2 须有 role="button"').toContain('role="button"')
    expect(tag, 'CTA2 须有 tabindex="0"').toContain('tabindex="0"')
    expect(tag, 'CTA2 须有 @keydown.enter 复用 openReactionTimeline').toContain('@keydown.enter="openReactionTimeline"')
    expect(tag, 'CTA2 原 @click.stop 须保留').toContain('@click.stop="openReactionTimeline"')
  })

  it('it3 时间线抽屉补 dialog 语义 + ESC/Tab 守卫且保留 v-if/@click', () => {
    const tag = overlay()
    expect(tag, '须定位 r29-timeline-overlay 开标签').not.toBe('')
    expect(tag, '抽屉须有 role="dialog"').toContain('role="dialog"')
    expect(tag, '抽屉须有 aria-modal="true"').toContain('aria-modal="true"')
    expect(tag, '抽屉须有 aria-label="完整反应时间线"').toContain('aria-label="完整反应时间线"')
    expect(tag, '抽屉须有 ref="timelineOverlayRef"').toContain('ref="timelineOverlayRef"')
    expect(tag, '抽屉须有 @keydown.esc="closeReactionTimeline"').toContain('@keydown.esc="closeReactionTimeline"')
    expect(tag, '抽屉须有 @keydown.tab="trapTimelineTab"').toContain('@keydown.tab="trapTimelineTab"')
    expect(tag, '抽屉原 v-if 须保留').toContain('v-if="reactionTimelineVisible"')
    expect(tag, '抽屉原 @click 须保留').toContain('@click="closeReactionTimeline"')
  })

  it('it4 关闭按钮补 ref + 键盘三件套且保留 @click', () => {
    const tag = closeBtn()
    expect(tag, '须定位 r29-timeline-close 开标签').not.toBe('')
    expect(tag, '关闭按钮须有 ref="timelineCloseRef"').toContain('ref="timelineCloseRef"')
    expect(tag, '关闭按钮须有 role="button"').toContain('role="button"')
    expect(tag, '关闭按钮须有 tabindex="0"').toContain('tabindex="0"')
    expect(tag, '关闭按钮须有 @keydown.enter 复用 closeReactionTimeline').toContain('@keydown.enter="closeReactionTimeline"')
    expect(tag, '关闭按钮原 @click 须保留').toContain('@click="closeReactionTimeline"')
  })

  it('it5 setup 区新增三个 timeline ref', () => {
    for (const name of ['timelineOverlayRef', 'timelineCloseRef', 'timelineTriggerRef']) {
      expect(battle, name + ' 须以 ref(null) 声明').toMatch(new RegExp(name + '\\s*=\\s*ref(?:<[^>]*>)?\\s*\\(\\s*null\\s*\\)'))
    }
  })

  it('it6 openReactionTimeline 末尾 nextTick 移入焦点到关闭按钮', () => {
    const body = openBody()
    expect(body, 'openReactionTimeline 必须存在').not.toBe('')
    expect(body, 'openReactionTimeline 必须 nextTick 后移入焦点').toContain('nextTick')
    expect(body, '焦点目标须为 timelineCloseRef').toContain('timelineCloseRef.value')
    expect(body, '必须调用 focus').toContain('focus(')
  })

  it('it7 触发源记录（currentTarget）与 close 焦点归还', () => {
    expect(battle, 'openReactionTimeline 须接收事件参数 e').toContain('function openReactionTimeline(e')
    const open = openBody()
    expect(open, 'open 须把触发源记入 timelineTriggerRef').toContain('timelineTriggerRef.value')
    expect(open, '触发源须取 e.currentTarget').toContain('currentTarget')
    const close = closeBody()
    expect(close, 'close 须存在').not.toBe('')
    expect(close, 'close 须从 timelineTriggerRef 取回触发源').toContain('timelineTriggerRef.value')
    expect(close, 'close 须 focus 归还').toContain('focus(')
  })

  it('it8 新增 trapTimelineTab 复用 R73 模式并绑定 timelineOverlayRef', () => {
    const body = blockAfter(battle, 'function trapTimelineTab')
    expect(body, 'trapTimelineTab 必须存在').not.toBe('')
    expect(body, '须以 timelineOverlayRef 取根节点').toContain('timelineOverlayRef.value')
    expect(body, '须查询可聚焦节点').toContain('querySelectorAll')
    expect(body, '须含 tabindex 选择器').toContain('[tabindex="0"]')
    expect(body, '须含 role=button 选择器').toContain('[role="button"]')
  })

  it('it9 不变量：openHelp/openReactionTimeline 双向互斥保留', () => {
    const help = blockAfter(battle, 'function openHelp')
    expect(help, 'openHelp 必须存在').not.toBe('')
    expect(help, 'openHelp 须无条件先关时间线抽屉').toContain('closeReactionTimeline()')
    expect(openBody(), 'openReactionTimeline 须先关图例').toContain('closeHelp()')
  })

  it('it10 不变量：R73 图例闭环不回退', () => {
    const legend = tagBefore(battle, 'class="legend-overlay"', 'view')
    expect(legend, '图例遮罩须保留').not.toBe('')
    expect(legend).toContain('role="dialog"')
    expect(legend).toContain('aria-modal="true"')
    expect(legend).toContain('@keydown.tab="trapLegendTab"')
    const close = tagBefore(battle, 'class="legend-close"', 'view')
    expect(close, '图例关闭按钮三件套保留').toContain('role="button"')
    expect(close).toContain('tabindex="0"')
    expect(close).toContain('@keydown.enter="closeHelp"')
    expect(blockAfter(battle, 'function trapLegendTab'), 'trapLegendTab 须保留').not.toBe('')
  })

  it('it11 不变量：timelinePausedByDrawer 暂停记账与 doPause/doResume 逐字不动', () => {
    const open = openBody()
    expect(open, '暂停守卫须保留').toContain('if (!paused.value && !settled.value)')
    expect(open, 'doPause 须保留').toContain('doPause()')
    expect(open, '本抽屉暂停记账须保留').toContain('timelinePausedByDrawer = true')
    const close = closeBody()
    expect(close, 'doResume 须保留').toContain('doResume()')
    expect(close, '关闭时须清本抽屉记账').toContain('timelinePausedByDrawer = false')
  })

  it('it12 不变量：令牌三锚点、零 --r103-、@click.stop 与文案逐字不动', () => {
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(battle + tokens, '业务源码不得新增 --r103- 令牌').not.toContain('--r103-')
    expect(battle.split('@click.stop="openReactionTimeline"').length - 1, '两处 @click.stop 须保留').toBe(2)
    expect(battle.split('查看完整反应时间线 →').length - 1, '两处文案须保留').toBe(2)
  })
})
